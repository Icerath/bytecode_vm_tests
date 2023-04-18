use std::{fmt, ops::Deref};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    NOP = 0,

    Dup,

    BinOp,

    LoadStr,
    LoadInt,
    LoadFloat,

    Jump,
    PopJumpIfFalse,

    LEN,
}

impl OpCode {
    pub const JUMP_SIZE: usize = usize::BITS as usize / 8;
    #[must_use]
    pub fn size(self) -> Option<u8> {
        Some(match self {
            Self::LoadStr => return None,
            Self::NOP | Self::LEN => 0,
            Self::BinOp | Self::Dup => 1,
            #[allow(clippy::cast_possible_truncation)]
            Self::Jump | Self::PopJumpIfFalse => Self::JUMP_SIZE as u8,
            Self::LoadInt | Self::LoadFloat => 8,
        })
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add = 0,
    Sub,
    Mul,
    Div,
    Mod,

    LE,
    LT,
    GE,
    GT,
    Eq,
    Ne,
}

#[derive(Debug, Default)]
pub struct Pool {
    items: Vec<u8>,
}

impl Pool {
    #[inline]
    pub fn push_dup(&mut self) {
        self.items.push(OpCode::Dup as u8);
    }
    #[inline]
    pub fn emit_jump(&mut self) -> usize {
        self.items.push(OpCode::Jump as u8);
        self.items.extend_from_slice(&0usize.to_le_bytes());
        self.items.len() - OpCode::JUMP_SIZE
    }
    #[inline]
    pub fn emit_pop_jump_if_false(&mut self) -> usize {
        self.items.push(OpCode::PopJumpIfFalse as u8);
        self.items.extend_from_slice(&0usize.to_le_bytes());
        self.items.len() - OpCode::JUMP_SIZE
    }
    #[inline]
    pub fn patch_jump(&mut self, pos: usize) {
        let here = self.len();
        let slice = &mut self.items[pos..pos + OpCode::JUMP_SIZE];
        debug_assert_eq!(slice, &[0; OpCode::JUMP_SIZE]);
        slice.copy_from_slice(&here.to_le_bytes());
    }
    #[inline]
    pub fn emit_flag(&mut self) -> usize {
        self.len()
    }
    #[inline]
    pub fn jump_flag(&mut self, pos: usize) {
        self.items.push(OpCode::Jump as u8);
        self.items.extend_from_slice(&pos.to_le_bytes());
    }
    #[inline]
    pub fn pop_jump_flag_if_false(&mut self, pos: usize) {
        self.items.push(OpCode::PopJumpIfFalse as u8);
        self.items.extend_from_slice(&pos.to_le_bytes());
    }
    #[inline]
    pub fn push_int(&mut self, int: i64) {
        self.items.push(OpCode::LoadInt as u8);
        self.items.extend_from_slice(&int.to_le_bytes());
    }
    #[inline]
    pub fn push_float(&mut self, float: f64) {
        self.items.push(OpCode::LoadFloat as u8);
        self.items.extend_from_slice(&float.to_le_bytes());
    }
    /// Pushes a null terminated string
    #[inline]
    pub fn push_str(&mut self, str: &str) {
        self.items.push(OpCode::LoadStr as u8);
        self.items.extend_from_slice(str.as_bytes());
        self.items.push(0);
    }
    #[inline]
    pub fn push_binop(&mut self, binop: BinOp) {
        self.items.push(OpCode::BinOp as u8);
        self.items.push(binop as u8);
    }
    #[inline]
    pub fn push_if(&mut self, subpool: &Pool) {
        let jump = self.emit_pop_jump_if_false();
        self.items.extend_from_slice(&subpool.items);
        self.patch_jump(jump);
    }
    #[inline]
    pub fn push_if_or_else(&mut self, subpool: &Pool, or_else: &Pool) {
        let jump_if = self.emit_pop_jump_if_false();
        self.items.extend_from_slice(&subpool.items);
        let jump_else = self.emit_jump();
        self.patch_jump(jump_if);
        self.items.extend_from_slice(&or_else.items);
        self.patch_jump(jump_else);
    }
    #[inline]
    pub fn push_loop(&mut self, body: &Pool) {
        let flag = self.emit_flag();
        self.items.extend_from_slice(body);
        self.jump_flag(flag);
    }
    #[inline]
    pub fn push_while_loop(&mut self, condition: &Pool, body: &Pool) {
        let flag = self.emit_flag();
        self.items.extend_from_slice(condition);
        let jump = self.emit_pop_jump_if_false();
        self.items.extend_from_slice(body);
        self.jump_flag(flag);
        self.patch_jump(jump);
    }
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl Deref for Pool {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl fmt::Display for Pool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut head = 0;
        while head < self.len() {
            let op_byte = self[head];
            assert!(op_byte < OpCode::LEN as u8);
            let op: OpCode = unsafe { std::mem::transmute(op_byte) };

            write!(f, "{head} ")?;
            head += 1;

            match op {
                OpCode::LEN => unreachable!(),
                OpCode::Dup => writeln!(f, "Dup")?,
                OpCode::NOP => writeln!(f, "Nop")?,
                OpCode::BinOp => {
                    let binop_byte = self[head];
                    let binop: BinOp = unsafe { std::mem::transmute(binop_byte) };
                    writeln!(f, "BinOp ({binop:?})")?;
                    head += 1;
                }
                OpCode::LoadFloat => {
                    let float = f64::from_le_bytes(read(self, head));
                    writeln!(f, "LoadFloat ({float})")?;
                    head += 8;
                }
                OpCode::LoadInt => {
                    let int = i64::from_le_bytes(read(self, head));
                    writeln!(f, "LoadInt ({int})")?;
                    head += 8;
                }
                OpCode::LoadStr => {
                    let num_bytes = self[head..].iter().take_while(|&b| *b != 0).count();
                    let str_bytes = &self[head..head + num_bytes];
                    let str = std::str::from_utf8(str_bytes).unwrap();
                    writeln!(f, "LoadStr ({str})")?;
                    head += num_bytes + 1;
                }
                OpCode::Jump => {
                    let jump = usize::from_le_bytes(read(self, head));
                    writeln!(f, "Jump ({jump})")?;
                    head += OpCode::JUMP_SIZE;
                }
                OpCode::PopJumpIfFalse => {
                    let jump = usize::from_le_bytes(read(self, head));
                    writeln!(f, "PopJumpIfFalse ({jump})")?;
                    head += OpCode::JUMP_SIZE;
                }
            }
        }
        Ok(())
    }
}

#[inline]
#[must_use]
pub fn read<const LEN: usize>(slice: &[u8], head: usize) -> [u8; LEN] {
    slice[head..head + LEN].try_into().unwrap()
}

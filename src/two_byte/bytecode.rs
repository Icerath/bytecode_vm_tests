use crate::{BinOp, Value};
use std::{fmt, ops::Deref};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    NOP = 0,

    Dup,

    BinOp,

    LoadConst,

    Jump,
    PopJumpIfFalse,

    LEN,
}

#[derive(Debug, Default)]
pub struct Pool<'a> {
    pub bytes: Vec<u8>,
    pub constants: Vec<Value<'a>>,
}

impl<'a> Pool<'a> {
    pub fn push(&mut self, opcode: OpCode, bytes: [u8; 2]) {
        self.bytes.push(opcode as u8);
        self.bytes.push(bytes[0]);
        self.bytes.push(bytes[1]);
    }
    pub fn push_u16(&mut self, opcode: OpCode, value: u16) {
        self.push(opcode, value.to_le_bytes());
    }
    pub fn push_const(&mut self, val: Value<'a>) {
        let index = self.insert_const(val);
        self.push_u16(OpCode::LoadConst, index);
    }
    #[inline]
    pub fn push_literal<V: Into<Value<'a>>>(&mut self, val: V) {
        self.push_const(val.into());
    }
    pub fn push_binop(&mut self, binop: BinOp) {
        self.push(OpCode::BinOp, [binop as u8, 0]);
    }
    pub fn push_zeroed(&mut self, op_code: OpCode) {
        self.push(op_code, [0, 0]);
    }
    pub fn insert_const(&mut self, val: Value<'a>) -> u16 {
        if let Some(index) = self.find_const(&val) {
            return index;
        }
        self.constants.push(val);
        self.constants.len() as u16 - 1
    }
    #[must_use]
    pub fn find_const(&self, target: &Value) -> Option<u16> {
        for (index, value) in self.constants.iter().enumerate() {
            if value == target {
                return Some(index as u16);
            }
        }
        None
    }
    #[must_use]
    pub fn get_const(&self, index: u16) -> Option<&Value<'a>> {
        self.constants.get(index as usize)
    }
    pub fn push_jump(&mut self, pos: u16) -> usize {
        self.push_u16(OpCode::Jump, pos);
        self.len() - 3
    }
    pub fn push_pop_jump_if_false(&mut self, pos: u16) -> usize {
        self.push_u16(OpCode::PopJumpIfFalse, pos);
        self.len() - 3
    }
    pub fn patch_jump(&mut self, pos: usize) {
        let new_pos = u16::try_from(self.len()).unwrap();
        let new_pos_bytes = new_pos.to_le_bytes();
        self.bytes[pos + 1] = new_pos_bytes[0];
        self.bytes[pos + 2] = new_pos_bytes[1];
    }
    #[must_use]
    pub fn len_u16(&self) -> u16 {
        u16::try_from(self.len()).unwrap()
    }
}

impl<'a> Deref for Pool<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl<'a> fmt::Display for Pool<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn read(bytes: &[u8], head: usize) -> [u8; 2] {
            bytes[head..head + 2].try_into().unwrap()
        }
        fn read_u16(bytes: &[u8], head: usize) -> u16 {
            u16::from_le_bytes(read(bytes, head))
        }
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
                }
                OpCode::LoadConst => {
                    let index = read_u16(self, head);
                    let constant = &self.constants[index as usize];
                    writeln!(f, "LoadConst ({constant:?})")?;
                }
                OpCode::Jump => {
                    let jump = read_u16(self, head);
                    writeln!(f, "Jump ({jump})")?;
                }
                OpCode::PopJumpIfFalse => {
                    let jump = read_u16(self, head);
                    writeln!(f, "PopJumpIfFalse ({jump})")?;
                }
            }
            head += 2;
        }
        Ok(())
    }
}

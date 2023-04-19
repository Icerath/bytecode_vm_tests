use std::{fmt, ops::Deref};

use crate::{BinOp, Value};

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

impl OpCode {
    pub const JUMP_SIZE: usize = usize::BITS as usize / 8;
    #[must_use]
    pub fn size(self) -> Option<u8> {
        Some(match self {
            Self::LoadConst => 4,
            Self::NOP | Self::LEN => 0,
            Self::BinOp | Self::Dup => 1,
            #[allow(clippy::cast_possible_truncation)]
            Self::Jump | Self::PopJumpIfFalse => Self::JUMP_SIZE as u8,
        })
    }
}

#[derive(Debug, Default)]
pub struct Pool<'a> {
    items: Vec<u8>,
    pub constants: Vec<Value<'a>>,
}

impl<'a> Pool<'a> {
    #[inline]
    pub fn push_dup(&mut self) {
        self.items.push(OpCode::Dup as u8);
    }
    #[inline]
    pub fn push_jump(&mut self, pos: usize) -> usize {
        self.items.push(OpCode::Jump as u8);
        self.items.extend_from_slice(&pos.to_le_bytes());
        self.items.len() - OpCode::JUMP_SIZE
    }
    #[inline]
    pub fn push_pop_jump_if_false(&mut self, pos: usize) -> usize {
        self.items.push(OpCode::PopJumpIfFalse as u8);
        self.items.extend_from_slice(&pos.to_le_bytes());
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
    pub fn push_const(&mut self, value: Value<'a>) -> usize {
        self.items.push(OpCode::LoadConst as u8);

        let index = if let Some(index) = self.constants.iter().position(|val| val == &value) {
            index
        } else {
            self.constants.push(value);
            self.constants.len() - 1
        };
        let index_u32 = u32::try_from(index).unwrap();
        self.items.extend_from_slice(&index_u32.to_le_bytes());

        index
    }
    #[inline]
    pub fn push_literal<V: Into<Value<'a>>>(&mut self, value: V) -> usize {
        self.push_const(value.into())
    }
    #[inline]
    pub fn push_binop(&mut self, binop: BinOp) {
        self.items.push(OpCode::BinOp as u8);
        self.items.push(binop as u8);
    }
    #[inline]
    pub fn push_if<F>(&mut self, body: F)
    where
        F: FnOnce(&mut Self),
    {
        let jump = self.push_pop_jump_if_false(0);
        body(self);
        self.patch_jump(jump);
    }
    #[inline]
    pub fn push_if_or_else<F1, F2>(&mut self, body: F1, or_else: F2)
    where
        F1: FnOnce(&mut Self),
        F2: FnOnce(&mut Self),
    {
        let jump_if = self.push_pop_jump_if_false(0);
        body(self);
        let jump_else = self.push_jump(0);
        self.patch_jump(jump_if);
        or_else(self);
        self.patch_jump(jump_else);
    }
    #[inline]
    pub fn push_loop(&mut self, body: &Pool) {
        let start = self.len();
        self.items.extend_from_slice(body);
        self.push_jump(start);
    }
    #[inline]
    pub fn push_while_loop(&mut self, condition: &Pool, body: &Pool) {
        let start = self.len();
        self.items.extend_from_slice(condition);
        let jump = self.push_pop_jump_if_false(0);
        self.items.extend_from_slice(body);
        self.push_jump(start);
        self.patch_jump(jump);
    }
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl<'a> Deref for Pool<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<'a> fmt::Display for Pool<'a> {
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
                OpCode::LoadConst => {
                    let index = u32::from_le_bytes(read(self, head)) as usize;
                    let value = &self.constants[index];
                    writeln!(f, "LoadConst ({index}) ({value:?})")?;
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

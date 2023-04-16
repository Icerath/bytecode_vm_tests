use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    NOOP = 0,

    Dup,

    BinOp,

    LoadStr,
    LoadInt,
    LoadFloat,

    Jump,
    PopJumpIfFalse,

    LEN,
}

impl Instruction {
    #[must_use]
    pub fn size(self) -> Option<u8> {
        Some(match self {
            Self::LoadStr => return None,
            Self::NOOP | Self::LEN => 0,
            Self::BinOp | Self::Dup => 1,
            Self::Jump | Self::PopJumpIfFalse => 4,
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
        self.items.push(Instruction::Dup as u8);
    }
    #[inline]
    pub fn push_jump_raw(&mut self, distance: i32) {
        self.items.push(Instruction::Jump as u8);
        self.items.extend_from_slice(&distance.to_le_bytes());
    }
    #[inline]
    pub fn push_int(&mut self, int: i64) {
        self.items.push(Instruction::LoadInt as u8);
        self.items.extend_from_slice(&int.to_le_bytes());
    }
    #[inline]
    pub fn push_float(&mut self, float: f64) {
        self.items.push(Instruction::LoadFloat as u8);
        self.items.extend_from_slice(&float.to_le_bytes());
    }
    /// Pushes a null terminated string
    #[inline]
    pub fn push_str(&mut self, str: &str) {
        self.items.push(Instruction::LoadStr as u8);
        self.items.extend_from_slice(str.as_bytes());
        self.items.push(0);
    }
    #[inline]
    pub fn push_binop(&mut self, binop: BinOp) {
        self.items.push(Instruction::BinOp as u8);
        self.items.push(binop as u8);
    }
    #[inline]
    pub fn push_pop_jump_if_false_raw(&mut self, distance: i32) {
        self.items.push(Instruction::PopJumpIfFalse as u8);
        self.items.extend_from_slice(&distance.to_le_bytes());
    }
    #[inline]
    pub fn push_if(&mut self, subpool: &Pool) {
        self.push_pop_jump_if_false_raw(subpool.len_i32());
        self.items.extend_from_slice(&subpool.items);
    }
    #[inline]
    pub fn push_if_or_else(&mut self, subpool: &Pool, or_else: &Pool) {
        self.push_pop_jump_if_false_raw(subpool.len_i32() + 5);
        self.items.extend_from_slice(&subpool.items);
        self.push_jump_raw(or_else.len_i32());
        self.items.extend_from_slice(&or_else.items);
    }
    #[inline]
    pub fn push_loop(&mut self, body: &Pool) {
        self.items.extend_from_slice(body);
        self.push_jump_raw(-body.len_i32() - 5);
    }
    #[inline]
    pub fn push_while_loop(&mut self, condition: &Pool, body: &Pool) {
        self.items.extend_from_slice(condition);
        self.push_pop_jump_if_false_raw(body.len_i32() + 5);
        self.items.extend_from_slice(body);
        self.push_jump_raw(-(condition.len_i32() + 5 + body.len_i32() + 5));
    }
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self
    }
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    #[must_use]
    pub fn len_i32(&self) -> i32 {
        self.len() as i32
    }
}

impl Deref for Pool {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

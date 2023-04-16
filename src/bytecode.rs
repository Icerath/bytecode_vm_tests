use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    NOOP = 0,
    Jump,

    BinOp,

    LoadStr,
    LoadInt,
    LoadFloat,

    PopJumpIfFalse,

    LEN,
}

impl Instruction {
    #[must_use]
    pub fn size(self) -> Option<u8> {
        Some(match self {
            Self::LoadStr => return None,
            Self::NOOP | Self::LEN => 0,
            Self::BinOp => 1,
            Self::LoadInt | Self::LoadFloat => 8,
            Self::PopJumpIfFalse | Self::Jump => 2,
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
    num_instructions: u16,
}

impl Pool {
    #[inline]
    pub fn push_jump(&mut self, distance: u16) {
        self.items.push(Instruction::Jump as u8);
        self.items.extend_from_slice(&distance.to_le_bytes());
        self.num_instructions += 1;
    }
    #[inline]
    pub fn push_int(&mut self, int: i64) {
        self.items.push(Instruction::LoadInt as u8);
        self.items.extend_from_slice(&int.to_le_bytes());
        self.num_instructions += 1;
    }
    #[inline]
    pub fn push_float(&mut self, float: f64) {
        self.items.push(Instruction::LoadFloat as u8);
        self.items.extend_from_slice(&float.to_le_bytes());
        self.num_instructions += 1;
    }
    /// Pushes a null terminated string
    #[inline]
    pub fn push_str(&mut self, str: &str) {
        self.items.push(Instruction::LoadStr as u8);
        self.items.extend_from_slice(str.as_bytes());
        self.items.push(0);
        self.num_instructions += 1;
    }
    #[inline]
    pub fn push_binop(&mut self, binop: BinOp) {
        self.items.push(Instruction::BinOp as u8);
        self.items.push(binop as u8);
        self.num_instructions += 1;
    }
    #[inline]
    pub fn push_pop_jump_if_false(&mut self, distance: u16) {
        self.items.push(Instruction::PopJumpIfFalse as u8);
        self.items.extend_from_slice(&distance.to_le_bytes());
        self.num_instructions += 1;
    }
    #[inline]
    pub fn push_if(&mut self, subpool: &Pool) {
        self.push_pop_jump_if_false(subpool.num_instructions);
        self.items.extend_from_slice(&subpool.items);
    }
    #[inline]
    pub fn push_if_or_else(&mut self, subpool: &Pool, or_else: &Pool) {
        self.push_pop_jump_if_false(subpool.num_instructions + or_else.num_instructions + 1);
        self.items.extend_from_slice(&subpool.items);
        self.push_jump(or_else.num_instructions);
        self.items.extend_from_slice(&or_else.items);
    }
    /// # Safety
    /// The vm can assume instructions are followed by their proper arguments
    #[inline]
    pub unsafe fn push(&mut self, instruction: Instruction) {
        self.items.push(instruction as u8);
    }
    /// # Safety
    /// The vm can assume instructions are followed by their proper arguments
    #[inline]
    pub unsafe fn push_byte(&mut self, byte: u8) {
        self.items.push(byte);
    }
    /// # Safety
    /// The vm can assume instructions are followed by their proper arguments
    #[inline]
    pub unsafe fn extend_from_slice(&mut self, slice: &[u8]) {
        self.items.extend_from_slice(slice);
    }
    #[inline]
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

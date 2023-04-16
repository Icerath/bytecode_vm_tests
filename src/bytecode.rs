use std::ops::Deref;

#[derive(Debug)]
#[repr(u8)]
pub enum Instruction {
    NOOP = 0,

    BinOp,

    LoadStr,
    LoadInt,
    LoadFloat,

    LEN,
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

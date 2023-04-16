use std::borrow::Cow;

use crate::bytecode::BinOp;

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Int(i64),
    Float(f64),
    Str(Cow<'a, str>),
}

impl<'a> Value<'a> {
    /// ## Panics
    pub fn run_binop(lhs: Self, rhs: Self, op: BinOp) -> Self {
        match op {
            BinOp::Add => Self::add(lhs, rhs),
            _ => todo!("{op:?}"),
        }
    }
    /// ## Panics
    fn add(lhs: Self, rhs: Self) -> Self {
        match (lhs, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(lhs + rhs),
            (lhs, rhs) => todo!("{lhs:?} - {rhs:?}"),
        }
    }
}

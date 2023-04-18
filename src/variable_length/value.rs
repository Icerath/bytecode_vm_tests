use std::borrow::Cow;

use super::bytecode::BinOp;

#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
    Int(i64),
    Float(f64),
    Str(Cow<'a, str>),
}

impl<'a> Value<'a> {
    #[must_use]
    pub fn run_binop(lhs: Self, rhs: Self, op: BinOp) -> Self {
        match op {
            BinOp::Add => Self::add(lhs, rhs),
            BinOp::Sub => Self::sub(lhs, rhs),
            BinOp::Mul => Self::mul(lhs, rhs),

            _ => todo!("{op:?}"),
        }
    }
    #[allow(clippy::cast_precision_loss)]
    fn add(lhs: Self, rhs: Self) -> Self {
        match (lhs, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(lhs + rhs),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs + rhs),
            (Self::Float(lhs), Self::Int(rhs)) => Self::Float(lhs + rhs as f64),
            (Self::Int(lhs), Self::Float(rhs)) => Self::Float(lhs as f64 + rhs),
            (Self::Str(lhs), Self::Str(rhs)) => {
                Self::Str(Cow::Owned(lhs.into_owned() + rhs.as_ref()))
            }
            (lhs, rhs) => todo!("{lhs:?} - {rhs:?}"),
        }
    }
    #[allow(clippy::cast_precision_loss)]
    fn sub(lhs: Self, rhs: Self) -> Self {
        match (lhs, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(lhs - rhs),
            (Self::Int(lhs), Self::Float(rhs)) => Self::Float(lhs as f64 - rhs),
            (Self::Float(lhs), Self::Int(rhs)) => Self::Float(lhs - rhs as f64),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs - rhs),
            (lhs, rhs) => todo!("{lhs:?} - {rhs:?}"),
        }
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn mul(lhs: Self, rhs: Self) -> Self {
        match (lhs, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(lhs * rhs),
            (Self::Int(lhs), Self::Float(rhs)) => Self::Float(lhs as f64 * rhs),
            (Self::Float(lhs), Self::Int(rhs)) => Self::Float(lhs * rhs as f64),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs * rhs),
            (Self::Str(str), Self::Int(int)) | (Self::Int(int), Self::Str(str)) => {
                Self::Str(Cow::Owned(str.repeat(int as usize)))
            }

            (lhs, rhs) => todo!("{lhs:?} - {rhs:?}"),
        }
    }
}

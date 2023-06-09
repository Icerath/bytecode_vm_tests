use crate::BinOp;
use std::borrow::Cow;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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

impl<'a> From<&Value<'a>> for bool {
    fn from(value: &Value<'a>) -> Self {
        match value {
            Value::Int(int) => *int != 0,
            Value::Str(str) => str.is_empty(),
            Value::Float(float) => *float != 0.0,
        }
    }
}

impl<'a> From<i64> for Value<'a> {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl<'a> From<f64> for Value<'a> {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(Cow::Borrowed(value))
    }
}

impl<'a> From<String> for Value<'a> {
    fn from(value: String) -> Self {
        Self::Str(Cow::Owned(value))
    }
}

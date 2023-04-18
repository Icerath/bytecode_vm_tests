#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

pub mod two_byte;
pub mod variable_length;

pub mod binop;
pub mod value;

pub use binop::BinOp;
pub use value::Value;

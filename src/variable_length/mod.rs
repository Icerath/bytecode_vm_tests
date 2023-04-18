#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

pub mod bytecode;
pub mod value;
pub mod vm;

#[cfg(test)]
mod tests;

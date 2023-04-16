#![warn(clippy::pedantic)]

pub mod bytecode;
pub mod value;
pub mod vm;

#[cfg(test)]
mod tests;

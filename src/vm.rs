use std::borrow::Cow;

use crate::{bytecode::Instruction, value::Value};

#[derive(Debug)]
pub struct Vm<'a> {
    pub bytes: &'a [u8],
    pub head: usize,
    pub stack: Vec<Value<'a>>,
}

impl<'a> Vm<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            head: 0,
            stack: vec![],
        }
    }
    pub fn run(&mut self) {
        while self.head < self.bytes.len() {
            self.run_next();
        }
    }
    pub fn run_next(&mut self) {
        let instruction_byte = self.bytes[self.head];
        self.head += 1;

        assert!(
            instruction_byte < Instruction::LEN as u8,
            "{instruction_byte:?}"
        );
        let instruction: Instruction = unsafe { std::mem::transmute(instruction_byte) };

        match instruction {
            Instruction::LoadInt => {
                let int_bytes = self.bytes[self.head..self.head + 8].try_into().unwrap();
                let int = i64::from_le_bytes(int_bytes);
                self.stack.push(Value::Int(int));
                self.head += 8;
            }
            Instruction::LoadFloat => {
                let float_bytes = self.bytes[self.head..self.head + 8].try_into().unwrap();
                let float = f64::from_le_bytes(float_bytes);
                self.stack.push(Value::Float(float));
                self.head += 8;
            }
            Instruction::LoadStr => {
                let string_bytes = slice_take_while_ne(&self.bytes[self.head..], &0);
                let string = std::str::from_utf8(string_bytes).unwrap();
                self.stack.push(Value::Str(Cow::Borrowed(string)));
                self.head += string_bytes.len();
            }
            Instruction::NOOP => (),
            _ => todo!("{instruction:?}"),
        }
    }
}

#[inline]
fn slice_take_while_ne<'a, T: Eq>(slice: &'a [T], target: &T) -> &'a [T] {
    for (index, item) in slice.iter().enumerate() {
        if item == target {
            return &slice[..index];
        }
    }
    slice
}

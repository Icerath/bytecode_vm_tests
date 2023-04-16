use std::borrow::Cow;

use crate::{
    bytecode::{BinOp, Instruction},
    value::Value,
};

#[must_use]
pub fn create_and_run(bytes: &[u8]) -> Vec<Value> {
    let mut vm = Vm::new(bytes);
    vm.run();
    vm.stack
}

#[derive(Debug)]
pub struct Vm<'a> {
    pub bytes: &'a [u8],
    pub head: usize,
    pub stack: Vec<Value<'a>>,
}

impl<'a> Vm<'a> {
    #[must_use]
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
            Instruction::Jump => {
                let jump_distance_bytes = self.bytes[self.head..self.head + 2].try_into().unwrap();
                let jump_distance = u16::from_le_bytes(jump_distance_bytes);
                self.head += 2;
                self.skip_instructions(jump_distance);
            }
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
            Instruction::BinOp => {
                let op_byte = self.bytes[self.head];
                let op: BinOp = unsafe { std::mem::transmute(op_byte) };

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();

                let new_value = Value::run_binop(lhs, rhs, op);
                self.stack.push(new_value);

                self.head += 1;
            }
            Instruction::PopJumpIfFalse => {
                let jump_distance_bytes = self.bytes[self.head..self.head + 2].try_into().unwrap();
                let jump_distance = u16::from_le_bytes(jump_distance_bytes);
                self.head += 2;

                let value = self.stack.pop().unwrap();
                if !bool::from(&value) {
                    self.skip_instructions(jump_distance);
                }
            }

            Instruction::NOOP => (),
            Instruction::LEN => todo!("{instruction:?}"),
        }
    }
    pub fn skip_instructions(&mut self, num: u16) {
        for _ in 0..num {
            let instruction = self.read_instruction().unwrap();
            self.head += 1;

            let size = match instruction.size() {
                Some(size) => size as usize,
                None => slice_take_while_ne(&self.bytes[self.head..], &0).len(),
            };

            self.head += size;
        }
    }
    #[inline]
    #[must_use]
    pub fn read_instruction(&self) -> Option<Instruction> {
        let byte = self.bytes[self.head];
        assert!(byte < Instruction::LEN as u8);
        unsafe { std::mem::transmute(byte) }
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

#[inline]
fn slice_take_while_ne<'a, T: Eq>(slice: &'a [T], target: &T) -> &'a [T] {
    for (index, item) in slice.iter().enumerate() {
        if item == target {
            return &slice[..index];
        }
    }
    slice
}

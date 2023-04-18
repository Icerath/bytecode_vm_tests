use super::bytecode::OpCode;
use crate::{BinOp, Value};
use std::borrow::Cow;

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
        let op_code_byte = self.bytes[self.head];
        self.head += 1;

        assert!(op_code_byte < OpCode::LEN as u8, "{op_code_byte:?}");
        let op_code: OpCode = unsafe { std::mem::transmute(op_code_byte) };
        match op_code {
            OpCode::Dup => {
                let top = self.stack.last().unwrap();
                self.stack.push(top.clone());
            }
            OpCode::LoadInt => {
                let int = i64::from_le_bytes(self.read());
                self.stack.push(Value::Int(int));

                self.head += 8;
            }
            OpCode::LoadFloat => {
                let float = f64::from_le_bytes(self.read());
                self.stack.push(Value::Float(float));

                self.head += 8;
            }
            OpCode::LoadStr => {
                let string_bytes = slice_take_while_ne(&self.bytes[self.head..], &0);
                let string = std::str::from_utf8(string_bytes).unwrap();
                self.stack.push(Value::Str(Cow::Borrowed(string)));

                self.head += string_bytes.len();
            }
            OpCode::BinOp => {
                let op_byte = self.bytes[self.head];
                let op: BinOp = unsafe { std::mem::transmute(op_byte) };

                self.head += 1;

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();

                let new_value = Value::run_binop(lhs, rhs, op);
                self.stack.push(new_value);
            }
            OpCode::Jump => {
                let jump_pos = usize::from_le_bytes(self.read());
                self.head = jump_pos;
            }
            OpCode::PopJumpIfFalse => {
                let jump_pos = usize::from_le_bytes(self.read());
                self.head += OpCode::JUMP_SIZE;
                let value = self.stack.pop().unwrap();
                if !bool::from(&value) {
                    self.head = jump_pos;
                }
            }

            OpCode::NOP => (),
            OpCode::LEN => todo!("{op_code:?}"),
        }
    }
    #[inline]
    #[must_use]
    pub fn read_op_code(&self) -> Option<OpCode> {
        let byte = self.bytes[self.head];
        assert!(byte < OpCode::LEN as u8);
        unsafe { std::mem::transmute(byte) }
    }
    #[inline]
    #[must_use]
    pub fn read<const LEN: usize>(&self) -> [u8; LEN] {
        self.bytes[self.head..self.head + LEN].try_into().unwrap()
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

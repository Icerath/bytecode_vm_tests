use super::bytecode::{OpCode, Pool};
use crate::{BinOp, Value};

#[must_use]
pub fn create_and_run<'a>(pool: &'a Pool<'a>) -> Vec<Value<'a>> {
    let mut vm = Vm::new(pool.as_bytes(), &pool.constants);
    vm.run();
    vm.stack
}

#[derive(Debug)]
pub struct Vm<'a> {
    pub bytes: &'a [u8],
    pub constants: &'a [Value<'a>],
    pub head: usize,
    pub stack: Vec<Value<'a>>,
}

impl<'a> Vm<'a> {
    #[must_use]
    pub fn new(bytes: &'a [u8], constants: &'a [Value<'a>]) -> Self {
        Self {
            bytes,
            constants,
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
            OpCode::LoadConst => {
                let index = u32::from_le_bytes(self.read()) as usize;
                let constant = &self.constants[index];
                self.stack.push(constant.clone());
                self.head += 4;
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

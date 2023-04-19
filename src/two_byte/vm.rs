use super::bytecode::{OpCode, Pool};
use crate::Value;

#[must_use]
pub fn create_and_run<'a>(pool: &'a Pool<'a>) -> Vec<Value<'a>> {
    let mut vm = Vm::new(pool);
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
    pub fn new(pool: &'a Pool<'a>) -> Self {
        Self {
            bytes: &pool.bytes,
            constants: &pool.constants,
            head: 0,
            stack: vec![],
        }
    }
    pub fn run(&mut self) {
        eprintln!("{:?}", self.constants);
        while self.head < self.bytes.len() {
            self.run_next();
        }
    }
    pub fn run_next(&mut self) {
        let op_code_byte = self.bytes[self.head];
        self.head += 1;

        assert!(op_code_byte < OpCode::LEN as u8);
        let op_code = unsafe { std::mem::transmute(op_code_byte) };

        match op_code {
            OpCode::NOP => (),
            OpCode::Dup => {
                if let Some(last) = self.stack.last() {
                    self.stack.push(last.clone());
                }
            }
            OpCode::LoadConst => {
                let index = self.read_u16();
                let constant = &self.constants[index as usize];
                self.stack.push(constant.clone());
            }
            OpCode::BinOp => {
                let binop_byte = self.bytes[self.head];
                let binop = unsafe { std::mem::transmute(binop_byte) };

                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();

                let new_val = Value::run_binop(lhs, rhs, binop);
                self.stack.push(new_val);
            }
            OpCode::Jump => {
                let location = self.read_u16();
                self.head = location as usize;
                return;
            }
            OpCode::PopJumpIfFalse => {
                let location = self.read_u16();

                let top = self.stack.pop().unwrap();

                if !bool::from(&top) {
                    self.head = location as usize;
                    return;
                }
            }
            OpCode::LEN => unreachable!(),
        }

        self.head += 2;
    }
    pub fn read_u16(&mut self) -> u16 {
        u16::from_le_bytes(self.read_bytes())
    }
    pub fn read_bytes(&mut self) -> [u8; 2] {
        [self.bytes[self.head], self.bytes[self.head + 1]]
    }
}

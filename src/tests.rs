use std::borrow::Cow;

use crate::{
    bytecode::{BinOp, Pool},
    value::Value,
    vm,
};

mod load_literals {
    use super::*;

    #[test]
    pub fn int() {
        let ints = [10, 15, 20, 25];

        let mut pool = Pool::default();
        for int in ints {
            pool.push_int(int);
        }

        let stack = vm::create_and_run(&pool);
        let expected_stack: Vec<Value> = ints.into_iter().map(Value::Int).collect();
        assert_eq!(stack, expected_stack);
    }

    #[test]
    pub fn float() {
        let floats = [1.1, 0.2, 12.2, f64::MAX, f64::INFINITY, f64::NEG_INFINITY];

        let mut pool = Pool::default();
        for float in floats {
            pool.push_float(float);
        }

        let stack = vm::create_and_run(&pool);
        let expected_stack: Vec<Value> = floats.into_iter().map(Value::Float).collect();
        assert_eq!(stack, expected_stack);
    }

    #[test]
    pub fn str() {
        let strings = ["", "Hello, World!", &"repeated ".repeat(100)];

        let mut pool = Pool::default();
        for str in strings {
            pool.push_str(str);
        }

        let stack = vm::create_and_run(&pool);

        let expected_stack: Vec<Value> = strings
            .into_iter()
            .map(|str| Value::Str(Cow::Borrowed(str)))
            .collect();

        assert_eq!(stack, expected_stack);
    }
}
mod binop_add {
    use super::*;
    const OP: BinOp = BinOp::Add;

    #[test]
    pub fn int() {
        let mut pool = Pool::default();
        pool.push_int(1);
        pool.push_int(2);
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(3)]);
    }

    #[test]
    pub fn float() {
        let mut pool = Pool::default();
        pool.push_float(1.23);
        pool.push_float(4.56);

        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Float(1.23 + 4.56)]);
    }

    #[test]
    pub fn int_float() {
        let mut pool = Pool::default();
        pool.push_int(1);
        pool.push_float(0.5);
        pool.push_binop(OP);

        pool.push_float(12.5);
        pool.push_int(2);
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(
            stack,
            vec![Value::Float(1.0 + 0.5), Value::Float(12.5 + 2.0)]
        );
    }

    #[test]
    pub fn str() {
        let mut pool = Pool::default();
        pool.push_str("Hello, ");
        pool.push_str("World!");
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Str(Cow::Owned("Hello, World!".into()))]);
    }
}

mod binop_sub {
    const OP: BinOp = BinOp::Sub;

    use super::*;
    #[test]
    pub fn int() {
        let mut pool = Pool::default();
        pool.push_int(1);
        pool.push_int(2);
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(-1)]);
    }

    #[test]
    pub fn float() {
        let mut pool = Pool::default();
        pool.push_float(1.23);
        pool.push_float(4.56);

        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Float(1.23 - 4.56)]);
    }

    #[test]
    pub fn int_float() {
        let mut pool = Pool::default();
        pool.push_int(1);
        pool.push_float(0.5);
        pool.push_binop(OP);

        pool.push_float(12.5);
        pool.push_int(2);
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(
            stack,
            vec![Value::Float(1.0 - 0.5), Value::Float(12.5 - 2.0)]
        );
    }
}

mod binop_mul {
    use super::*;
    const OP: BinOp = BinOp::Mul;

    #[test]
    pub fn int() {
        let mut pool = Pool::default();
        pool.push_int(1);
        pool.push_int(2);
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(2)]);
    }

    #[test]
    pub fn float() {
        let mut pool = Pool::default();
        pool.push_float(1.23);
        pool.push_float(4.56);

        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Float(1.23 * 4.56)]);
    }

    #[test]
    pub fn int_float() {
        let mut pool = Pool::default();
        pool.push_int(1);
        pool.push_float(0.5);
        pool.push_binop(OP);

        pool.push_float(12.5);
        pool.push_int(2);
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(
            stack,
            vec![Value::Float(1.0 * 0.5), Value::Float(12.5 * 2.0)]
        );
    }

    #[test]
    pub fn str_int() {
        let mut pool = Pool::default();

        pool.push_str("repeat ");
        pool.push_int(5);
        pool.push_binop(OP);

        pool.push_int(3);
        pool.push_str("hello ");
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(
            stack,
            vec![
                Value::Str(Cow::Owned("repeat ".repeat(5))),
                Value::Str(Cow::Owned("hello ".repeat(3)))
            ]
        );
    }
}

mod test_pop_jump_if_false {
    use super::*;

    #[test]
    fn jump_false() {
        let mut pool = Pool::default();
        pool.push_int(5);
        pool.push_int(0);
        pool.push_pop_jump_if_false(2);
        pool.push_int(2);
        pool.push_float(3.3);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(5)]);
    }

    #[test]
    fn jump_true() {
        let mut pool = Pool::default();
        pool.push_int(5);
        pool.push_int(1);
        pool.push_pop_jump_if_false(2);
        pool.push_int(2);
        pool.push_float(3.3);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(5), Value::Int(2), Value::Float(3.3)]);
    }

    #[test]
    fn if_true() {
        let mut pool = Pool::default();

        let mut if_body = Pool::default();
        if_body.push_str("Hello");

        pool.push_int(1);
        pool.push_if(&if_body);
        pool.push_str(", World!");

        let stack = vm::create_and_run(&pool);
        assert_eq!(
            stack,
            vec![
                Value::Str(Cow::Borrowed("Hello")),
                Value::Str(Cow::Borrowed(", World!"))
            ]
        );
    }

    #[test]
    fn if_false() {
        let mut pool = Pool::default();

        let mut if_body = Pool::default();
        if_body.push_str("Hello");

        pool.push_int(0);
        pool.push_if(&if_body);
        pool.push_str(", World!");

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Str(Cow::Borrowed(", World!"))]);
    }
}

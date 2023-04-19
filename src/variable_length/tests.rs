use super::{bytecode::Pool, vm};
use crate::{BinOp, Value};
use std::borrow::Cow;

mod load_literals {
    use super::*;

    #[test]
    pub fn int() {
        let ints = [10, 15, 20, 25];

        let mut pool = Pool::default();
        for int in ints {
            pool.push_literal(int);
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
            pool.push_literal(float);
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
            pool.push_literal(str);
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
        pool.push_literal(1);
        pool.push_literal(2);
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(3)]);
    }

    #[test]
    pub fn float() {
        let mut pool = Pool::default();
        pool.push_literal(1.23);
        pool.push_literal(4.56);

        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Float(1.23 + 4.56)]);
    }

    #[test]
    pub fn int_float() {
        let mut pool = Pool::default();
        pool.push_literal(1);
        pool.push_literal(0.5);
        pool.push_binop(OP);

        pool.push_literal(12.5);
        pool.push_literal(2);
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
        pool.push_literal("Hello, ");
        pool.push_literal("World!");
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
        pool.push_literal(1);
        pool.push_literal(2);
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(-1)]);
    }

    #[test]
    pub fn float() {
        let mut pool = Pool::default();
        pool.push_literal(1.23);
        pool.push_literal(4.56);

        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Float(1.23 - 4.56)]);
    }

    #[test]
    pub fn int_float() {
        let mut pool = Pool::default();
        pool.push_literal(1);
        pool.push_literal(0.5);
        pool.push_binop(OP);

        pool.push_literal(12.5);
        pool.push_literal(2);
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
        pool.push_literal(1);
        pool.push_literal(2);
        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(2)]);
    }

    #[test]
    pub fn float() {
        let mut pool = Pool::default();
        pool.push_literal(1.23);
        pool.push_literal(4.56);

        pool.push_binop(OP);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Float(1.23 * 4.56)]);
    }

    #[test]
    pub fn int_float() {
        let mut pool = Pool::default();
        pool.push_literal(1);
        pool.push_literal(0.5);
        pool.push_binop(OP);

        pool.push_literal(12.5);
        pool.push_literal(2);
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

        pool.push_literal("repeat ");
        pool.push_literal(5);
        pool.push_binop(OP);

        pool.push_literal(3);
        pool.push_literal("hello ");
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

mod test_jump {
    use super::*;

    #[test]
    fn patch_jump() {
        let mut pool = Pool::default();

        pool.push_literal(1);
        let jump = pool.push_jump(0);
        pool.push_literal(2);
        pool.push_literal(3);
        pool.patch_jump(jump);
        pool.push_literal(4);

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(1), Value::Int(4)]);
    }

    #[test]
    fn jump_flag() {
        let mut pool = Pool::default();

        pool.push_literal(1);
        let flag = pool.len();
        pool.push_literal(1);
        pool.push_binop(BinOp::Sub);
        pool.push_dup();
        pool.push_pop_jump_if_false(flag);

        eprintln!("{pool}");
        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(-1)]);
    }
}

mod test_pop_jump_if_false {
    use super::*;
    #[test]
    fn if_true() {
        let mut pool = Pool::default();

        pool.push_literal(1);
        pool.push_if(|if_body| {
            if_body.push_literal("Hello");
        });
        pool.push_literal(", World!");

        eprintln!("{pool}");
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

        pool.push_literal(0);
        pool.push_if(|if_body| {
            if_body.push_literal("Hello");
        });
        pool.push_literal(", World!");

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Str(Cow::Borrowed(", World!"))]);
    }

    #[test]
    fn if_else_true() {
        let mut pool = Pool::default();

        let mut if_body = Pool::default();
        if_body.push_literal("if");

        let mut else_body = Pool::default();
        else_body.push_literal("else");

        pool.push_literal(1);
        pool.push_if_or_else(
            |if_body| {
                if_body.push_literal("if");
            },
            |else_body| {
                else_body.push_literal("else");
            },
        );

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Str(Cow::Borrowed("if"))]);
    }

    #[test]
    fn if_else_false() {
        let mut pool = Pool::default();

        let mut if_body = Pool::default();
        if_body.push_literal("if");

        let mut else_body = Pool::default();
        else_body.push_literal("else");

        pool.push_literal(0);
        pool.push_if_or_else(
            |if_body| {
                if_body.push_literal("if");
            },
            |else_body| {
                else_body.push_literal("else");
            },
        );

        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Str(Cow::Borrowed("else"))]);
    }
}

mod loops {
    use super::*;

    #[test]
    fn test_while_loop() {
        let mut pool = Pool::default();
        pool.push_literal(10);

        let mut condition = Pool::default();
        condition.push_literal(1);
        condition.push_binop(BinOp::Sub);
        condition.push_dup();

        let loop_body = Pool::default();
        pool.push_while_loop(&condition, &loop_body);

        eprintln!("{pool}");
        let stack = vm::create_and_run(&pool);
        assert_eq!(stack, vec![Value::Int(0)]);
    }
}

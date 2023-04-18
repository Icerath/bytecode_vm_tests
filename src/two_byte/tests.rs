use super::{
    bytecode::{OpCode, Pool},
    vm,
};
use crate::{BinOp, Value};
use std::borrow::Cow;

#[test]
fn test_consts() {
    let mut pool = Pool::default();
    pool.push_literal(1);
    pool.push_literal(1.5);
    pool.push_literal("Hello, World!");

    eprintln!("{}", &pool);
    let stack = vm::create_and_run(&pool);
    assert_eq!(
        stack,
        vec![
            Value::Int(1),
            Value::Float(1.5),
            Value::Str(Cow::Borrowed("Hello, World!"))
        ]
    );
}
#[test]
fn test_dup() {
    let mut pool = Pool::default();
    pool.push_literal(1);
    pool.push_literal(2.0);
    pool.push_zeroed(OpCode::Dup);

    eprintln!("{pool}");
    let stack = vm::create_and_run(&pool);
    assert_eq!(
        stack,
        vec![Value::Int(1), Value::Float(2.0), Value::Float(2.0)]
    );
}
#[test]
fn test_binops() {
    let mut pool = Pool::default();
    pool.push_literal(2);
    pool.push_literal(3);
    pool.push_binop(BinOp::Add);
    pool.push_literal(2);
    pool.push_binop(BinOp::Sub);
    pool.push_literal("Hello, ");
    pool.push_binop(BinOp::Mul);

    let stack = vm::create_and_run(&pool);
    assert_eq!(
        stack,
        vec![Value::Str(Cow::Borrowed("Hello, Hello, Hello, "))]
    );
}

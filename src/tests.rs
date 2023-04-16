use std::{borrow::Cow, fmt::LowerExp};

use crate::{bytecode::Pool, value::Value, vm::Vm};

#[test]
pub fn test_ints() {
    let ints = [10, 15, 20, 25];

    let mut pool = Pool::default();
    for int in ints {
        pool.push_int(int);
    }

    let mut vm = Vm::new(pool.as_bytes());
    vm.run();

    let expected_stack: Vec<Value> = ints.into_iter().map(Value::Int).collect();
    assert_eq!(vm.stack, expected_stack);
}

#[test]
pub fn test_floats() {
    let floats = [1.1, 0.2, 12.2, f64::MAX, f64::INFINITY, f64::NEG_INFINITY];

    let mut pool = Pool::default();
    for float in floats {
        pool.push_float(float);
    }

    let mut vm = Vm::new(pool.as_bytes());
    vm.run();

    let expected_stack: Vec<Value> = floats.into_iter().map(Value::Float).collect();
    assert_eq!(vm.stack, expected_stack);
}

#[test]
pub fn test_strings() {
    let strings = ["", "Hello, World!", &"repeated ".repeat(100)];

    let mut pool = Pool::default();
    for str in strings {
        pool.push_str(str);
    }

    let mut vm = Vm::new(pool.as_bytes());
    vm.run();

    let expected_stack: Vec<Value> = strings
        .into_iter()
        .map(|str| Value::Str(Cow::Borrowed(str)))
        .collect();
    assert_eq!(vm.stack, expected_stack);
}

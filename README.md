# petty_vm_test

## About
Experimenting with a basic bytecode virtual machine for fun.

## Usage
This crate isn't meant to be used, however it is a library that has a public api.

## General Api
Creating bytecode can be done using `bytecode::Pool`
```rust
let mut pool = Pool::default();
pool.push_int(10);
pool.push_int(20);
pool.push_op(BinOp::Add);
```
Running bytecode can be done using the vm or it's helper method
```rust
let mut vm = Vm::new(&pool);
vm.run();
let stack: Vec<Value> = vm.stack;
```
or 
```rust
let stack = vm::create_and_run(&pool);
```
The stack is simply a `Vec<Value>` where `Value` is an enum of some value (like a str or int).

## Example Usage

```rust
let mut pool = Pool::default();

pool.push_int(5);
pool.push_int(7);
pool.push_binop(BinOp::Mul);

pool.push_float(3.3);
pool.push_binop(BinOp::Mul);

let stack = vm::create_and_run(&pool);
println!("{stack:?}");
```

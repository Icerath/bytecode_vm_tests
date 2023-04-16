use petty_static::{bytecode::Pool, vm::Vm};

fn main() {
    let mut pool = Pool::default();
    pool.push_int(13);
    pool.push_int(14);
    pool.push_float(1.10);
    pool.push_str("Hello, World!");

    let mut vm = Vm::new(pool.as_bytes());
    vm.run();
    println!("{:?}", vm.stack);
}

use std::{thread, time};

fn main() {
    const  ONE_GIB : usize = 1 << 30;
    let ptr = allocator_suite::simple_use::simple_alloicate_memory_address(ONE_GIB, true, Some(1)).unwrap();
    let time = time::Duration::from_secs(20);
    println!("{:?}", ptr);
    thread::sleep(time);
    println!("the END");
}

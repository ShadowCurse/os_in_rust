#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

use bootloader::{entry_point, BootInfo};

use core::panic::PanicInfo;

use os_in_rust::alloc::{boxed::Box, rc::Rc, vec};
use os_in_rust::{
    basic_initialization, panic_handler, println,
    task::{executor::Executor, keyboard::print_keypress, Task},
};

entry_point!(main);
fn main(boot_info: &'static mut BootInfo) -> ! {
    #[cfg(test)]
    test_main();

    basic_initialization(boot_info);

    let _x = Box::new(69);

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(print_keypress()));
    executor.run();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[panic_handler]
fn ph(info: &PanicInfo) -> ! {
    panic_handler(info)
}

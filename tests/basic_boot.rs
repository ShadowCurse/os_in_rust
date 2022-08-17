#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
use os_in_rust::{interrupts::init_idt, println};

entry_point!(main);
fn main(boot_info: &'static mut BootInfo) -> ! {
    init_idt();
    test_main();
    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

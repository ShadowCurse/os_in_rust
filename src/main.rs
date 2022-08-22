#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os_in_rust::{init, panic_handler, println, text_display::init_text_display};

entry_point!(main);
fn main(boot_info: &'static mut BootInfo) -> ! {
    #[cfg(test)]
    test_main();

    init_text_display(boot_info);
    init();

    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    println!("Hello world");
    loop {}
}

#[panic_handler]
fn ph(info: &PanicInfo) -> ! {
    panic_handler(info)
}

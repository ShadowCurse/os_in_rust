#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use os_in_rust::{basic_initialization, println};

entry_point!(main);
fn main(boot_info: &'static mut BootInfo) -> ! {
    basic_initialization(boot_info);
    test_main();
    panic!("Qemu not exited");
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[panic_handler]
fn ph(info: &PanicInfo) -> ! {
    use os_in_rust::{exit_qemu, QemuExitCode};

    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

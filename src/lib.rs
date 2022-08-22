#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod print;
pub mod serial;
pub mod tests;
pub mod text_display;

use gdt::init_gdt;
use interrupts::init_idt;

pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("PANIC: INFO:{:#?}", info);
    loop {}
}

pub fn panic_handler_test(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub fn test_runner(tests: &[&dyn tests::Testable]) {
    use crate::serial::init_serial_port;

    init_serial_port();
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn init() {
    init_idt();
    init_gdt();
}

#[cfg(test)]
mod test {
    use super::*;
    use bootloader::{entry_point, BootInfo};

    entry_point!(main);

    fn main(_boot_info: &'static mut BootInfo) -> ! {
        test_main();
        loop {}
    }

    #[panic_handler]
    fn ph(info: &PanicInfo) -> ! {
        panic_handler_test(info)
    }

    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

pub extern crate alloc;

use core::panic::PanicInfo;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod print;
pub mod serial;
pub mod task;
pub mod tests;
pub mod text_display;

use bootloader::BootInfo;
use x86_64::VirtAddr;

use allocator::init_heap;
use gdt::init_gdt;
use interrupts::{init_idt, init_pics};
use memory::{memory_mapper, BootInfoFrameAllocator};
use text_display::init_text_display;

pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("PANIC: INFO:{:#?}", info);
    hlt_loop();
}

pub fn panic_handler_test(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
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

pub fn basic_initialization(boot_info: &'static mut BootInfo) {
    init_text_display(boot_info.framebuffer.as_mut().unwrap());

    init_gdt();
    init_idt();
    init_pics();

    // Enabling interrupts
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(*boot_info.physical_memory_offset.as_ref().unwrap());
    let mut mapper = unsafe { memory_mapper(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&boot_info.memory_regions) };
    init_heap(&mut mapper, &mut frame_allocator).expect("Heap init failed");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
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

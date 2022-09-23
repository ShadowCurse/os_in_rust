#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os_in_rust::{
    hlt_loop, init, memory::active_level_4_table, panic_handler, println,
    text_display::init_text_display,
};
use x86_64::VirtAddr;

entry_point!(main);
fn main(boot_info: &'static mut BootInfo) -> ! {
    #[cfg(test)]
    test_main();

    init_text_display(boot_info.framebuffer.as_mut().unwrap());
    init();

    let phys_mem_offset = VirtAddr::new(*boot_info.physical_memory_offset.as_ref().unwrap());
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    println!("Hello world");
    hlt_loop();
}

#[panic_handler]
fn ph(info: &PanicInfo) -> ! {
    panic_handler(info)
}

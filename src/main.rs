#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os_in_rust::{hlt_loop, init, panic_handler, println, text_display::init_text_display};
use x86_64::{structures::paging::Translate, VirtAddr};

entry_point!(main);
fn main(boot_info: &'static mut BootInfo) -> ! {
    #[cfg(test)]
    test_main();

    init_text_display(boot_info.framebuffer.as_mut().unwrap());
    init();

    let phys_mem_offset = VirtAddr::new(*boot_info.physical_memory_offset.as_ref().unwrap());

    let mapper = unsafe { os_in_rust::memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        *boot_info.physical_memory_offset.as_ref().unwrap(),
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
    println!("Hello world");
    hlt_loop();
}

#[panic_handler]
fn ph(info: &PanicInfo) -> ! {
    panic_handler(info)
}

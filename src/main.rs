#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

use core::panic::PanicInfo;

use os_in_rust::alloc::{boxed::Box, rc::Rc, vec};
use os_in_rust::{
    allocator::init_heap, hlt_loop, init, memory::BootInfoFrameAllocator, panic_handler, println,
    text_display::init_text_display,
};

entry_point!(main);
fn main(boot_info: &'static mut BootInfo) -> ! {
    #[cfg(test)]
    test_main();

    init_text_display(boot_info.framebuffer.as_mut().unwrap());
    init();

    let phys_mem_offset = VirtAddr::new(*boot_info.physical_memory_offset.as_ref().unwrap());

    let mut mapper = unsafe { os_in_rust::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(&boot_info.memory_regions) };

    init_heap(&mut mapper, &mut frame_allocator).expect("Heap init failed");

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

    println!("Hello world");
    hlt_loop();
}

#[panic_handler]
fn ph(info: &PanicInfo) -> ! {
    panic_handler(info)
}

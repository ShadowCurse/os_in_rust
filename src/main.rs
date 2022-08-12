#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

mod text_display;

use text_display::init_text_display;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    init_text_display(boot_info);
    println!("Hello world");
    loop {}
}

#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::{fmt::Write, panic::PanicInfo};

mod text_display;

use text_display::TextDisplay;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(main);

fn main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let mut display = TextDisplay::new(framebuffer);
        write!(display, "Hello!");
    }
    loop {}
}

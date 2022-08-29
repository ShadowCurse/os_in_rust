#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os_in_rust::{exit_qemu, init, println, serial::init_serial_port, QemuExitCode};

entry_point!(main);
fn main(_boot_info: &'static mut BootInfo) -> ! {
    init();
    init_serial_port();

    should_fail();
    println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);

    panic!("Qemu not exited");
}

fn should_fail() {
    println!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn ph(_info: &PanicInfo) -> ! {
    println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

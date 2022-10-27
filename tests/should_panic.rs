#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use os_in_rust::{
    basic_initialization, exit_qemu, println, QemuExitCode,
};

entry_point!(main);
fn main(boot_info: &'static mut BootInfo) -> ! {
    basic_initialization(boot_info);

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

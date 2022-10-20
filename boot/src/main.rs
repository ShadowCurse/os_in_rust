use std::{
    path::{Path, PathBuf},
    process::Command,
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    kernel_path: PathBuf,
    #[arg(long)]
    no_run: bool,
    #[arg(long)]
    no_display: bool,
}

fn main() {
    let args = Args::parse();
    let kernel_binary_path = args.kernel_path.canonicalize().unwrap();

    let bios = create_disk_images(&kernel_binary_path);

    if args.no_run {
        println!("Created disk image at `{}`", bios.display());
        return;
    }

    let mut run_cmd = Command::new("qemu-system-x86_64");
    run_cmd
        .arg("-drive")
        .arg(format!("format=raw,file={}", bios.display()))
        .arg("-device")
        .arg("isa-debug-exit,iobase=0xf4,iosize=0x04")
        .arg("-serial")
        .arg("stdio")
        .arg("-cpu")
        .arg("Skylake-Client-v3")
        .arg("--no-reboot")
        .arg("-s");

    if args.no_display {
        run_cmd.arg("-display").arg("none");
    }

    let exit_status = run_cmd.status().unwrap();
    if !exit_status.success() {
        if let Some(code) = exit_status.code() {
            // 33 means successuful shutdown
            if code == 33 {
                std::process::exit(0);
            } else {
                std::process::exit(exit_status.code().unwrap_or(1));
            }
        }
    }
}

pub fn create_disk_images(kernel_binary_path: &Path) -> PathBuf {
    let bootloader_manifest_path = bootloader_locator::locate_bootloader("bootloader").unwrap();
    let kernel_manifest_path = locate_cargo_manifest::locate_manifest().unwrap();

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd
        .arg("--kernel-manifest")
        .arg(&kernel_manifest_path);
    build_cmd.arg("--kernel-binary").arg(kernel_binary_path);
    build_cmd
        .arg("--target-dir")
        .arg(kernel_manifest_path.parent().unwrap().join("target"));
    build_cmd
        .arg("--out-dir")
        .arg(kernel_binary_path.parent().unwrap());
    build_cmd.arg("--quiet");

    if !build_cmd.status().unwrap().success() {
        panic!("build failed");
    }

    let kernel_binary_name = kernel_binary_path.file_name().unwrap().to_str().unwrap();
    let disk_image = kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-bios-{}.img", kernel_binary_name));
    if !disk_image.exists() {
        panic!(
            "Disk image does not exist at {} after bootloader build",
            disk_image.display()
        );
    }
    disk_image
}

#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

global_asm!(include_str!("assembly/gdt.s"));
global_asm!(include_str!("assembly/boot.s"));

#[no_mangle]
extern "C" fn boot_main() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
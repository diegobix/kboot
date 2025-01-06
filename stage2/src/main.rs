#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let message = b"Hello Rust! Si lees esto es que funciono!";

    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x07; // Light grey on black
        }
    }

    #[allow(clippy::empty_loop)]
    loop {
        unsafe {asm!("hlt");}
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".text.boot"]
pub extern "C" fn kernel_main() -> ! {
    write_debug(10,"Hello from kernel!".as_bytes());

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[used]
#[link_section = ".text.padding"]
static PAD: [u8; 8192] = [0x90; 8192];

pub fn write_debug(row: usize, message: &[u8]) {
    let vga_buffer = 0xB8000 as *mut u8;
    unsafe {
        for (i, &byte) in message.iter().enumerate() {
            *vga_buffer.add(row * 160 + i * 2) = byte;
            *vga_buffer.add(row * 160 + i * 2 + 1) = 0x0F; // Blanco sobre negro
        }
    }
}
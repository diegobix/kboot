#![no_std]
#![no_main]

use core::{arch::asm, fmt, panic::PanicInfo};

use stage2::{vga_log, vga_logln, vga_video_buffer::VgaBuffer};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    vga_logln!("Hola macro!");
    vga_logln!("Bye!");

    for i in 0..10 {
        vga_logln!("X");
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

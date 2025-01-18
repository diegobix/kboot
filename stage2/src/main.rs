#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

use stage2::{vga_print, vga_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    vga_print!("Hola desde VGA!");
    vga_println!("Soy Diego!");

    #[allow(clippy::empty_loop)]
    loop {
        unsafe {asm!("hlt");}
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
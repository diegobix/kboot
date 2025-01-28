#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

use stage2::{vga_log, vga_logln, vga_video_buffer::VgaBuffer};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use stage2::vga_video_buffer::Colour;

    VgaBuffer::change_colour(VgaBuffer::instance(), Colour::Red, Colour::Black);
    vga_logln!("Hola!");
    VgaBuffer::change_colour(VgaBuffer::instance(), Colour::LightGreen, Colour::DarkGray);
    vga_logln!("La segunda etapa / kernel esta cargada!");
    vga_logln!("\n\n\n\n\n");

    VgaBuffer::instance().change_colour(Colour::White, Colour::Black);

    vga_logln!("Kboot");
    vga_logln!("by Diego Arenas");

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

#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

use stage2::fat::{find_file, load_file};

pub fn write_debug(row: usize, message: &[u8]) {
    let vga_buffer = 0xB8000 as *mut u8;
    unsafe {
        for (i, &byte) in message.iter().enumerate() {
            *vga_buffer.add(row * 160 + i * 2) = byte;
            *vga_buffer.add(row * 160 + i * 2 + 1) = 0x0F; // Blanco sobre negro
        }
    }
}

#[used]
#[link_section = ".rodata"]
static FILE_NAME: &str = "KERNEL.BIN";

#[used]
#[link_section = ".rodata"]
static KERNEL_ADDR: usize = 0x20000;

#[no_mangle]
#[link_section = ".text.boot"]
pub extern "C" fn _start() -> ! {
    write_debug(1, "Fase 2 iniciada!".as_bytes());

    unsafe {
        // Buscamos el primer cluster en la Root Dir
        let filename = FILE_NAME;
        let first_cluster = find_file(filename.as_bytes());

        // Cargamos cada cluster siguiendo la FAT
        load_file(first_cluster, KERNEL_ADDR);

        write_debug(2, "Saltando al kernel".as_bytes());

        // Limpiamos registros para el kernel
        asm!("xor eax, eax");
        asm!("xor ebx, ebx");
        asm!("xor ecx, ecx");
        asm!("xor edx, edx");
        asm!("xor esi, esi");

        // Saltamos al kernel
        let entry_point: extern "C" fn() = core::mem::transmute(KERNEL_ADDR as *const u8);
        entry_point();
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

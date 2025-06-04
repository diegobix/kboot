#![no_std]
#![no_main]

use core::{arch::asm, hint::black_box, panic::PanicInfo};

use stage2::{ata, fat::{find_file, BootSector}};

unsafe fn write_debug(row: usize, message: &[u8]) {
    let vga_buffer = 0xB8000 as *mut u8;
    for (i, &byte) in message.iter().enumerate() {
        *vga_buffer.add(row * 160 + i * 2) = byte;
        *vga_buffer.add(row * 160 + i * 2 + 1) = 0x0F; // Blanco sobre negro
    }
}

#[no_mangle]
#[link_section = ".text.boot"]
pub extern "C" fn _start() -> ! {
    unsafe {write_debug(0, b"Hello World!");}
    // inicializar bss
    let test_value: u8 = black_box(0x42);
    let mut stack_test = black_box(0u8);
    stack_test = black_box(0x55);

    black_box(&test_value);
    black_box(&stack_test);

    let mut simple_array = [0u8; 4];
    simple_array[0] = 0x4B;
    simple_array[1] = 0x45;

    unsafe { 
        let bpb = &*(0x7c00 as *const BootSector);

    
        let filename_bytes = *b"KERNEL.BIN";

        let cluster = find_file(&filename_bytes);
        black_box(&cluster);
        let kernel_dest = 0x0010_0000 as *mut u16;
        ata::read_sectors_lba(88, 1, kernel_dest).unwrap();

        asm!("xor eax, eax");
        asm!("xor ebx, ebx");
        asm!("xor ecx, ecx");
        asm!("xor edx, edx");
        asm!("xor esi, esi");

        let entry_point: extern "C" fn() = core::mem::transmute(kernel_dest as *const u8);
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

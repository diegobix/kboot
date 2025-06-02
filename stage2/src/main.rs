#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

use stage2::{ata, fat::BootSector};

#[no_mangle]
#[link_section = ".text.boot"]
pub extern "C" fn _start() -> ! {
    // vga_logln!("HolA");

    unsafe { 
        let bpb = &*(0x7c00 as *const BootSector);
    
        let kernel_dest = 0x0010_0000 as *mut u16;
        ata::read_sectors_lba(88, 1, kernel_dest).unwrap();

        asm!("mov esp, {}", in(reg) 0x0009_0000);
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

use core::arch::asm;

// Puertos estándar para canal primario ATA
const ATA_DATA: u16         = 0x1F0;
const ATA_SECTOR_COUNT: u16 = 0x1F2;
const ATA_LBA_LOW: u16      = 0x1F3;
const ATA_LBA_MID: u16      = 0x1F4;
const ATA_LBA_HIGH: u16     = 0x1F5;
const ATA_DRIVE_SELECT: u16 = 0x1F6;
const ATA_COMMAND: u16      = 0x1F7;
const ATA_STATUS: u16       = 0x1F7;
const ATA_STATUS_ERR: u8    = 0x01; 

const ATA_DEVICE_CONTROL: u16 = 0x3F6;
const ATA_CMD_READ_SECTORS: u8 = 0x20;

#[derive(Debug)]
pub enum AtaError {
    InvalidLba,
    InvalidCount,
    DeviceFault
}
/// Función que usa ATA PIO para leer count sectores en buffer desde lba
/// # Safety
/// La lectura puede fallar
pub unsafe fn read_sectors_lba(lba: u32, count: u8, buffer: *mut u16) -> Result<(), AtaError> {
    if lba >= 0x10000000 {
        return Err(AtaError::InvalidLba);
    }

    if count == 0 {
        return Err(AtaError::InvalidCount);
    }

    // Reset del controlador
    outb(ATA_DEVICE_CONTROL, 0x04);
    wait_bsy();
    outb(ATA_DEVICE_CONTROL, 0x00);
    wait_bsy();

    // Limpiar el registro de estado
    inb(ATA_STATUS);

    wait_bsy();

    // Comandar lectura
    outb(ATA_DRIVE_SELECT, 0xE0 | ((lba >> 24) & 0x0f) as u8);
    outb(ATA_SECTOR_COUNT, count);
    outb(ATA_LBA_LOW, lba as u8);
    outb(ATA_LBA_MID, (lba >> 8) as u8);
    outb(ATA_LBA_HIGH, (lba >> 16) as u8);
    outb(ATA_COMMAND, ATA_CMD_READ_SECTORS);

    // Recibir datos leidos
    for sector in 0..count {
        wait_drq();

        asm!(
            "rep insw",
            in("dx") ATA_DATA,
            in("ecx") 256,
            inout("edi") buffer.add(sector as usize * 256) => _,
            options(nostack, preserves_flags)
        );
    }

    // Comprobar error
    if inb(ATA_STATUS) & ATA_STATUS_ERR != 0 {
        return Err(AtaError::DeviceFault)
    }

    Ok(())
}

fn wait_bsy() {
    while (inb(ATA_STATUS) & 0x80) != 0 {}
}

fn wait_drq() {
    while (inb(ATA_STATUS) & 0x08) == 0 {}
}

fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {asm!("in al, dx", out("al") value, in("dx") port)};
    value
}

fn outb(port: u16, value: u8) {
    unsafe { asm!("out dx, al", in("dx") port, in("al") value)};
}

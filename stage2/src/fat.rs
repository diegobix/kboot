#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
/// Representa el sector de arranque (Boot Sector) de un sistema de archivos FAT.
///
/// Contiene la información necesaria para identificar y montar el sistema de archivos,
/// así como parámetros esenciales para la gestión de archivos y directorios.
///
/// Campos:
/// - `jump`: Instrucción de salto para el código de arranque.
/// - `oem_id`: Identificador OEM (8 bytes).
/// - `bytes_per_sector`: Cantidad de bytes por sector.
/// - `sectors_per_cluster`: Sectores por clúster.
/// - `reserved_sectors`: Sectores reservados al inicio del volumen.
/// - `num_fats`: Número de tablas FAT.
/// - `root_entry_count`: Cantidad de entradas en el directorio raíz.
/// - `total_sectors_16`: Total de sectores (si es menor a 65536).
/// - `media`: Descriptor de tipo de medio.
/// - `fat_size_sectors`: Tamaño de cada FAT en sectores.
/// - `sectors_per_track`: Sectores por pista (para BIOS).
/// - `num_heads`: Número de cabezas (para BIOS).
/// - `hidden_sectors`: Sectores ocultos antes de la partición.
/// - `total_sectors_32`: Total de sectores (si es mayor a 65535).
/// - `drive_number`: Número de unidad lógica.
/// - `reserved1`: Reservado.
/// - `boot_signature`: Firma de arranque (debe ser 0x29 para FAT12/16).
/// - `volume_id`: Identificador único del volumen.
/// - `volume_label`: Etiqueta del volumen (11 bytes).
/// - `fs_type`: Tipo de sistema de archivos (8 bytes, por ejemplo "FAT16   ").
/// - `boot_code`: Código de arranque ejecutable.
/// - `signature`: Firma final del sector de arranque (debe ser 0xAA55).
pub struct BootSector {
    // BPB Bios Parameter Block
    pub jump: [u8; 3],
    pub oem_id: [u8; 8],
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub num_fats: u8,
    pub root_entry_count: u16,
    pub total_sectors_16: u16,
    pub media: u8,
    pub fat_size_sectors: u16,
    pub sectors_per_track: u16,
    pub num_heads: u16,
    pub hidden_sectors: u32,
    pub total_sectors_32: u32,
    // A partir de aquí es el Extended Boot Record
    pub drive_number: u8,
    pub reserved1: u8,
    pub boot_signature: u8,
    pub volume_id: u32,
    pub volume_label: [u8; 11],
    pub fs_type: [u8; 8],
    pub boot_code: [u8; 448],
    pub signature: u16, // Debe ser 0xAA55
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
/// Representa una entrada de directorio en un sistema de archivos FAT.
///
/// Los campos de esta estructura corresponden a los atributos estándar de una entrada FAT:
/// - `filename`: Nombre del archivo (8 bytes, relleno con espacios si es necesario).
/// - `extension`: Extensión del archivo (3 bytes).
/// - `attributes`: Atributos del archivo (por ejemplo, solo lectura, oculto, sistema, etc.).
/// - `reserved`: Espacio reservado para uso futuro (10 bytes).
/// - `time`: Hora de la última modificación.
/// - `date`: Fecha de la última modificación.
/// - `starting_cluster`: Número del primer clúster del archivo.
/// - `file_size`: Tamaño del archivo en bytes.
pub struct DirectoryEntry {
    pub filename: [u8; 8],
    pub extension: [u8; 3],
    pub attributes: u8,
    pub reserved: [u8; 10],
    pub time: u16,
    pub date: u16,
    pub starting_cluster: u16,
    pub file_size: u32,
}

// Función para buscar KERNEL.BIN en el directorio raíz y obtener sus sectores
pub fn find_kernel_sectors(
    boot_sector: &BootSector,
    read_sector_fn: fn(u32) -> [u8; 512],
) -> Option<(u16, u32)> {
    // Retorna (starting_cluster, file_size)

    // Calcular dónde empieza el directorio raíz
    let fat_start_sector = boot_sector.reserved_sectors as u32;
    let root_dir_sectors = ((boot_sector.root_entry_count as u32 * 32) + 511) / 512;
    let root_dir_start_sector =
        fat_start_sector + (boot_sector.num_fats as u32 * boot_sector.fat_size_sectors as u32);

    // Nombre que buscamos en formato FAT: "KERNEL  BIN"
    let target_name = b"KERNEL  ";
    let target_ext = b"BIN";

    // Buscar en cada sector del directorio raíz
    for sector in 0..root_dir_sectors {
        let sector_data = read_sector_fn(root_dir_start_sector + sector);

        // Cada sector tiene 16 entradas de directorio (512 bytes / 32 bytes por entrada)
        for entry_idx in 0..16 {
            let offset = entry_idx * 32;

            // Verificar si llegamos al final del directorio
            if sector_data[offset] == 0x00 {
                return None; // No hay más entradas
            }

            // Saltear entradas eliminadas
            if sector_data[offset] == 0xE5 {
                continue;
            }

            // Comparar nombre (8 bytes)
            let mut name_matches = true;
            for i in 0..8 {
                if sector_data[offset + i] != target_name[i] {
                    name_matches = false;
                    break;
                }
            }

            // Comparar extensión (3 bytes)
            if name_matches {
                for i in 0..3 {
                    if sector_data[offset + 8 + i] != target_ext[i] {
                        name_matches = false;
                        break;
                    }
                }
            }

            if name_matches {
                // Encontrado! Extraer starting_cluster y file_size
                let starting_cluster =
                    (sector_data[offset + 26] as u16) | ((sector_data[offset + 27] as u16) << 8);
                let file_size = (sector_data[offset + 28] as u32)
                    | ((sector_data[offset + 29] as u32) << 8)
                    | ((sector_data[offset + 30] as u32) << 16)
                    | ((sector_data[offset + 31] as u32) << 24);

                return Some((starting_cluster, file_size));
            }
        }
    }

    None // No encontrado
}

// Función para cargar KERNEL.BIN en memoria siguiendo la cadena de clusters
pub unsafe fn load_kernel_to_memory(
    boot_sector: &BootSector,
    starting_cluster: u16,
    file_size: u32,
    load_address: *mut u8,
    read_sector_fn: fn(u32) -> [u8; 512],
) {
    // Calcular posiciones importantes
    let fat_start_sector = boot_sector.reserved_sectors as u32;
    let root_dir_sectors = ((boot_sector.root_entry_count as u32 * 32) + 511) / 512;
    let root_dir_start_sector =
        fat_start_sector + (boot_sector.num_fats as u32 * boot_sector.fat_size_sectors as u32);
    let data_start_sector = root_dir_start_sector + root_dir_sectors;

    let mut current_cluster = starting_cluster;
    let mut bytes_loaded = 0u32;
    let sectors_per_cluster = boot_sector.sectors_per_cluster as u32;

    while current_cluster != 0 && current_cluster < 0xFFF8 && bytes_loaded < file_size {
        // Convertir cluster a sector
        let first_sector = data_start_sector + ((current_cluster as u32 - 2) * sectors_per_cluster);

        // Leer todos los sectores del cluster
        for sector_offset in 0..sectors_per_cluster {
            if bytes_loaded >= file_size {
                break;
            }

            let sector_data = read_sector_fn(first_sector + sector_offset);
            let bytes_to_copy = core::cmp::min(512, file_size - bytes_loaded);

            // Copiar datos a memoria
            unsafe {
                let dest = load_address.add(bytes_loaded as usize);
                for i in 0..(bytes_to_copy as usize) {
                    *dest.add(i) = sector_data[i];
                }
            }

            bytes_loaded += bytes_to_copy;
        }

        // Obtener siguiente cluster de la FAT
        current_cluster = get_next_cluster_fat16(current_cluster, fat_start_sector, read_sector_fn);
    }
}

// Función auxiliar para obtener el siguiente cluster de la FAT
fn get_next_cluster_fat16(
    cluster: u16,
    fat_start_sector: u32,
    read_sector_fn: fn(u32) -> [u8; 512],
) -> u16 {
    // En FAT16, cada entrada son 2 bytes
    let fat_offset = cluster as u32 * 2;
    let fat_sector = fat_start_sector + (fat_offset / 512);
    let sector_offset = (fat_offset % 512) as usize;

    let sector_data = read_sector_fn(fat_sector);

    // Leer entrada FAT (little endian)
    (sector_data[sector_offset] as u16) | ((sector_data[sector_offset + 1] as u16) << 8)
}

// Función completa que hace todo el proceso
pub unsafe fn load_kernel_bin(
    boot_sector: &BootSector,
    load_address: *mut u8,
    read_sector_fn: fn(u32) -> [u8; 512],
) -> Result<u32, &'static str> {
    // Buscar KERNEL.BIN
    if let Some((starting_cluster, file_size)) = find_kernel_sectors(boot_sector, read_sector_fn) {
        // Cargar el archivo
        unsafe {
            load_kernel_to_memory(
                boot_sector,
                starting_cluster,
                file_size,
                load_address,
                read_sector_fn,
            );
        }
        Ok(file_size)
    } else {
        Err("KERNEL.BIN not found")
    }
}

// Ejemplo de uso:
/*
// En tu bootloader, después de leer el boot sector:
let boot_sector_data = read_disk_sector(0); // Tu función ATA PIO
let boot_sector = unsafe { *(boot_sector_data.as_ptr() as *const BootSector) };

// Cargar KERNEL.BIN en la dirección 0x100000 (1MB)
let kernel_address = 0x100000 as *mut u8;

match load_kernel_bin(&boot_sector, kernel_address, read_disk_sector) {
    Ok(size) => {
        // KERNEL.BIN cargado exitosamente, size bytes cargados
        // Ahora puedes saltar a kernel_address para ejecutar el kernel
    }
    Err(msg) => {
        // Error: no se encontró KERNEL.BIN
    }
}
*/



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


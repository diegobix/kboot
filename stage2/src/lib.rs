#![no_std]

pub mod vga_video_buffer;

/// Representa la cabecera ELF.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ElfHeader {
    pub e_ident: [u8; 16], // Identificador ELF
    pub e_type: u16,       // Tipo de archivo
    pub e_machine: u16,    // Arquitectura
    pub e_version: u32,    // Versión ELF
    pub e_entry: u32,      // Dirección de entrada
    pub e_phoff: u32,      // Offset a cabecera de programa
    pub e_shoff: u32,      // Offset a cabecera de sección
    pub e_flags: u32,      // Flags
    pub e_ehsize: u16,     // Tamaño de esta cabecera
    pub e_phentsize: u16,  // Tamaño de una entrada en la cabecera del programa
    pub e_phnum: u16,      // Número de entradas en la cabecera del programa
    pub e_shentsize: u16,  // Tamaño de una entrada en la cabecera de sección
    pub e_shnum: u16,      // Número de entradas en la cabecera de sección
    pub e_shstrndx: u16,   // Índice de la tabla de nombres de secciones
}

/// Representa una entrada de programa en ELF.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProgramHeader {
    pub p_type: u32,   // Tipo de segmento
    pub p_offset: u32, // Offset del segmento
    pub p_vaddr: u32,  // Dirección virtual del segmento
    pub p_paddr: u32,  // Dirección física del segmento
    pub p_filesz: u32, // Tamaño en el archivo
    pub p_memsz: u32,  // Tamaño en memoria
    pub p_flags: u32,  // Flags
    pub p_align: u32,  // Alineación
}
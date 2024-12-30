#GDT
.align 4
gdt_start:
gdt_null: # el descriptor nulo obligatorio
    .long 0x0 #
    .long 0x0

gdt_code: # el descriptor del segmento de código
    # base=0x0, límite=0xfffff,
    # primeros flags: (presente)1 (privilegio)00 (tipo de descriptor)1 -> 1001b
    # flags de tipo: (código)1 (conforme)0 (legible)1 (accedido)0 -> 1010b
    # segundos flags: (granularidad)1 (32 bits)1 (segmento de 64 bits)0 (AVL)0 -> 1100b
    .word 0xffff     # Límite (bits 0-15)
    .word 0x0        # Base (bits 0-15)
    .byte 0x0        # Base (bits 16-23)
    .byte 0b10011010  # primeros flags, flags de tipo
    .byte 0b11001111  # segundos flags, Límite (bits 16-19)
    .byte 0x0        # Base (bits 24-31)

gdt_data: # el descriptor del segmento de datos
    # Igual que el segmento de código excepto por los flags de tipo:
    # flags de tipo: (código)0 (expandir hacia abajo)0 (escribible)1 (accedido)0 -> 0010b
    .word 0xffff     # Límite (bits 0-15)
    .word 0x0        # Base (bits 0-15)
    .byte 0x0        # Base (bits 16-23)
    .byte 0b10010010  # primeros flags, flags de tipo
    .byte 0b11001111  # segundos flags, Límite (bits 16-19)
    .byte 0x0        # Base (bits 24-31)

gdt_end: # La razón para poner una etiqueta al final de la
# GDT es para que podamos hacer que el ensamblador calcule
# el tamaño de la GDT para el descriptor de GDT

# Descriptor de GDT
gdt_descriptor:
    .word gdt_end - gdt_start - 1 # Tamaño de la GDT
    .long gdt_start # Dirección de inicio de la GDT

# (0x0 -> NULO# 0x08 -> CODIGO# 0x10 -> DATOS)
.equ CODE_SEG, gdt_code - gdt_start
.equ DATA_SEG, gdt_data - gdt_start
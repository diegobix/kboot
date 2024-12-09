;GDT
align 4
gdt_start:
gdt_null: ; el descriptor nulo obligatorio
    dd 0x0 ;
    dd 0x0

gdt_code: ; el descriptor del segmento de código
    ; base=0x0, límite=0xfffff,
    ; primeros flags: (presente)1 (privilegio)00 (tipo de descriptor)1 -> 1001b
    ; flags de tipo: (código)1 (conforme)0 (legible)1 (accedido)0 -> 1010b
    ; segundos flags: (granularidad)1 (32 bits)1 (segmento de 64 bits)0 (AVL)0 -> 1100b
    dw 0xffff     ; Límite (bits 0-15)
    dw 0x0        ; Base (bits 0-15)
    db 0x0        ; Base (bits 16-23)
    db 10011010b  ; primeros flags, flags de tipo
    db 11001111b  ; segundos flags, Límite (bits 16-19)
    db 0x0        ; Base (bits 24-31)

gdt_data: ; el descriptor del segmento de datos
    ; Igual que el segmento de código excepto por los flags de tipo:
    ; flags de tipo: (código)0 (expandir hacia abajo)0 (escribible)1 (accedido)0 -> 0010b
    dw 0xffff     ; Límite (bits 0-15)
    dw 0x0        ; Base (bits 0-15)
    db 0x0        ; Base (bits 16-23)
    db 10010010b  ; primeros flags, flags de tipo
    db 11001111b  ; segundos flags, Límite (bits 16-19)
    db 0x0        ; Base (bits 24-31)

gdt_end: ; La razón para poner una etiqueta al final de la
; GDT es para que podamos hacer que el ensamblador calcule
; el tamaño de la GDT para el descriptor de GDT

; Descriptor de GDT
gdt_descriptor:
    dw gdt_end - gdt_start - 1 ; Tamaño de la GDT
    dd gdt_start ; Dirección de inicio de la GDT

; (0x0 -> NULO; 0x08 -> CODIGO; 0x10 -> DATOS)
CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start
# [bits 16]   # Modo read -> 16 bits
.code16
# [org 0x7c00]  # Ya no es necesario, manejado en el script de link

# .include "src/assembly/gdt.s"

.section .text.stage1   # seccion ejecutable

.global init # Hacemos visible init al link script
init:
stack_conf:
    cli             # Desactivamos interrupciones mientras configuramos
    mov ax, 0
    mov ss, ax      # Stack segment a 0
    mov ds, ax      # Ponemos el data y extra segment a 0 para desactivar la segmentacion
    mov es, ax
    mov sp, 0x7b00  # El stack crece hacia abajo, por lo que lo ponemos antes de nuestro programa
    sti             # Reactivamos interrupciones

video_mode:        # Activamos el modo video
    mov ah, 0
    mov al, 0x3
    int 0x10

program:
    mov si, message     # Cargamos en el reg si el string
    call print_string   # llamamos al procedimiento print_string

    mov si, name
    call print_string
    jmp initial_config

# Imprime el string null terminado cargado en el registro si
print_string:
    push ax         # Guardamos en el stack
    mov al, [si]    # Cargamos el primer char
print_continue:
    mov ah, 0xe    # Colocamos en ah el valor requerido para imprimir
    call print_char # Imprimimos el caracter
    inc si          # Incrementamos el puntero al string
    mov al, [si]    # Cargamos el byte a imprimir en al
    cmp al, 0       # Si es el caracter nulo terminamos
    jnz print_continue# Si no seguimos

    pop ax          # Recuperamos del stack
    ret

print_char:
    mov al, [si]   
    int 0x10
    ret

initial_config:
    cli
# Activamos la línea A20 del procesador, que está desactivada por defecto.
# Esto nos permite tener 21 bits para direcciones (activando el bit 20).
# Para lograrlo, utilizamos el puerto del teclado??
enable_a20_1:
    # Leer desde el puerto 0x64
    in al, 0x64
    # Comprobar el bit 1 de AL
    test al, 0x02
    # Si el bit 1 está activado, saltar a enable_a20_1
    jnz enable_a20_1

    # Establecer AL en 0xd1
    mov al, 0xd1
    # Escribir AL en el puerto 0x64
    out 0x64, al

enable_a20_2:
    # Leer desde el puerto 0x64
    in al, 0x64
    # Comprobar el bit 1 de AL
    test al, 0x02
    # Si el bit 1 está activado, saltar a enable_a20_2
    jnz enable_a20_2

    # Establecer AL en 0xdf
    mov al, 0xdf
    # Escribir AL en el puerto 0x60
    out 0x60, al

load_stage2:
    mov ah, 0x2             # Leer sector a mem
    mov al, stage2_sectors  # Numero de sectores
    mov ch, 0               # Numero de cilindro
    mov cl, 2               # Primer sector a leer
    mov dh, 0               # Cabeza del disco
    mov dl, 0x80            # 0x80 es el primer disco duro, 0x00 el primer disquete
    mov bx, 0x7e00          # Direccion a cargar
    int 0x13

load_gdt:
    # Cargamos la GDT en el procesador
    lgdt [gdt_descriptor]
enter_protected_mode:
    # Pasamos a modo protegido (32 bits)
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    # Hacemos un salto largo para finalizar la entrada al modo protegido
    # jmp CODE_SEG:stage2

    .byte 0xEA
    .long stage2
    .word CODE_SEG

.section .data.stage1

.extern stage2_sectors

message: .ascii "Iniciando bootloader...\r\n\0"
name: .ascii "By Diego Arenas\r\n\0"

# times 510-($-$$) db 0 # Ya no es necesario, se maneja en el linker script
# db 0x55, 0xaa

.code32 # Estamos trabajando ya con 32 bits
.section .text.stage2
stage2:
load_data_segment:
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov ss, ax

    mov ax, 0
    mov fs, ax
    mov gs, ax

jump_to_rust:
.extern boot_main
    call boot_main
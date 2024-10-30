[bits 16]   ; Modo read -> 16 bits
; [org 0x7c00]  ; Ya no es necesario, manejado en el script de link

section .text   ; seccion ejecutable

global init ; Hacemos visible init al link script
init:
stack_conf:
    cli             ; Desactivamos interrupciones mientras configuramos
    mov ax, 0
    mov ss, ax      ; Stack segment a 0
    mov sp, 0x7b00  ; El stack crece hacia abajo, por lo que lo ponemos antes de nuestro programa
    sti             ; Reactivamos interrupciones

video_mode:        ; Activamos el modo video
    mov ah, 0
    mov al, 0x03
    int 0x10

program:
    mov si, message     ; Cargamos en el reg si el string
    call print_string   ; llamamos al procedimiento print_string

    mov si, name
    call print_string
    jmp loop

; Imprime el string null terminado cargado en el registro si
print_string:
    push ax         ; Guardamos en el stack
    mov al, [si]    ; Cargamos el primer char
print_continue:
    mov ah, 0x0e    ; Colocamos en ah el valor requerido para imprimir
    call print_char ; Imprimimos el caracter
    inc si          ; Incrementamos el puntero al string
    mov al, [si]    ; Cargamos el byte a imprimir en al
    cmp al, 0       ; Si es el caracter nulo terminamos
    jnz print_continue; Si no seguimos

    pop ax          ; Recuperamos del stack
    ret

print_char:
    mov al, [si]   
    int 0x10
    ret

; para que qemu no salga
loop:
    hlt
    jmp $

section .data
message db "Iniciando bootloader...", 0x0d, 0x0a, 0
name db "By Diego Arenas", 0

; times 510-($-$$) db 0
; db 0x55, 0xaa
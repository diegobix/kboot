[bits 16]
[org 0x7c00]
global _init

_init:
  cli

  xor ax, ax
  mov ds, ax
  mov ss, ax
  mov es, ax

  mov bp, 0x7c00
  mov sp, 0x7c00

; Activamos la línea A20 del procesador, que está desactivada por defecto.
; Esto nos permite tener 21 bits para direcciones (activando el bit 20).
; Para lograrlo, utilizamos el puerto del teclado.
enable_a20_1:
  ; Leer desde el puerto 0x64
  in al, 0x64
  ; Comprobar el bit 1 de AL
  test al, 0x02
  ; Si el bit 1 está activado, saltar a enable_a20_1
  jnz enable_a20_1

  ; Establecer AL en 0xd1
  mov al, 0xd1
  ; Escribir AL en el puerto 0x64
  out 0x64, al

enable_a20_2:
  ; Leer desde el puerto 0x64
  in al, 0x64
  ; Comprobar el bit 1 de AL
  test al, 0x02
  ; Si el bit 1 está activado, saltar a enable_a20_2
  jnz enable_a20_2

  ; Establecer AL en 0xdf
  mov al, 0xdf
  ; Escribir AL en el puerto 0x60
  out 0x60, al

  sti
video_mode:
  mov ah, 0
  mov al, 0x03
  int 0x10

read_stage2:
  lea si, read_msg
  call print_string

  mov bx, STAGE2_DIR
  mov ah, 0x02
  mov al, 15
  mov ch, 0
  mov dh, 0
  mov dl, 0
  mov cl, 0x02
  int 0x13

  jc read_stage2_error

  mov dh, 15
  cmp dh, al
  jne read_stage2_error2
  jmp change_to_protected

read_stage2_error:
  lea si, read_error
  call print_string
  jmp $

read_stage2_error2:
  lea si, read_error2
  call print_string
  jmp $

change_to_protected:
cli
  lgdt [gdt_descriptor]
  mov eax, cr0
  or eax, 1
  mov cr0, eax

  jmp CODE_SEG:start_32


; Procedures 
print_string:
  mov ah, 0x0e
print_loop:
  lodsb
  cmp al, 0
  je print_done
  int 0x10
  jmp print_loop
print_done:
  ret

[bits 32]
start_32:
  ; En modo real los segmentos eran numeros literales. En protegido, hacen referencia a la entrada de la gdt correspondiente
  ; al segmento que corresponda. En este caso, todos apuntan al segmento de datos.
  mov ax, DATA_SEG
  mov ds, ax
  mov es, ax
  mov ss, ax
  mov fs, ax
  mov gs, ax

  mov ebp, 0x7c00
  mov esp, ebp

  call STAGE2_DIR ; 

spin:
  hlt
  jmp spin

%include "gdt.s"

read_msg db "Leyendo del disco...", 0
read_error db "Error al leer del disco (1)", 0
read_error2 db "Error al leer del disco (2)", 0

STAGE2_DIR equ 0x7e00

times 510 - ($-$$) db 0
dw 0xaa55   ; Magic number
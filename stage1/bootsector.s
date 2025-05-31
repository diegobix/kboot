[bits 16]
[org 0x7c3e]  ; FAT16 salta a esta instrucción al iniciar el bootsector

global _init
_init:
  cli ; Desactivamos las interrupciones

  xor ax, ax  ; Registros de segmento a 0
  mov ds, ax
  mov ss, ax
  mov es, ax

  mov bp, 0x7c00  ; El stack crece hacia abajo -> hacia la dirección contraria del bootlaoder
  ; mov sp, 0x7c00

; Activamos video mode 3
; Activamos la línea A20 del procesador, que está desactivada por defecto.
; Esto nos permite tener 21 bits para direcciones (activando el bit 20).
; Para lograrlo, utilizamos el puerto del teclado.
enable_a20_1:
enable_a20_1_wait:
  ; En este bucle esperamos hasta que no este ocupado el controlador del puerto
  ; Leer desde el puerto 0x64
  in al, 0x64
  ; Comprobar el bit 1 de AL
  test al, 2
  ; Si el bit 1 está activado, saltar a enable_a20_1
  jnz enable_a20_1_wait

  ; Establecer AL en 0xd1
  mov al, 0xd1
  ; Escribir AL en el puerto 0x64
  out 0x64, al  ; El comando 0xd1 indica que queremos enviar un comando

enable_a20_2:
enable_a20_2_wait:
  ; Volvemos a esperar a que esté listo el controlador
  ; Leer desde el puerto 0x64
  in al, 0x64
  ; Comprobar el bit 1 de AL
  test al, 2
  ; Si el bit 1 está activado, saltar a enable_a20_2
  jnz enable_a20_2_wait

  ; Establecer AL en 0xdf
  mov al, 0xdf
  ; Escribir AL en el puerto 0x60
  out 0x60, al

  sti ; Reactivamos interrupciones
video_mode:
  mov ah, 0 ; Activamos video mode 3
  mov al, 0x03
  int 0x10

; Cargamos en la memoria la segunda fase del bootloader
read_stage2:
  lea si, read_msg
  call print_string

  mov si, dap_packet
  mov dl, 0x80
  mov ah, 0x42
  int 0x13
  mov si, after_int
  call print_string
  jc read_stage2_error 
  jmp change_to_protected
  ; -----------------------------------------------------------------------------
  ; Utiliza la interrupción 0x13 para leer 9 sectores (AL=9) del disco duro (DL=0x80)
  ; comenzando desde el cilindro 0 (CH=0), sector 36 (CL=36), cabeza 0 (DH=0).
  ; El buffer de destino está en la dirección apuntada por BX (STAGE2_DIR).
  ; AH=0x02 indica la función de "leer sectores desde disco".
  ; ES:BX debe apuntar al buffer de destino (ES está comentado, se asume 0).
  ; -----------------------------------------------------------------------------
  ; mov ah, 0x02
  ; mov al, 8
  ; mov ch, 1
  ; mov cl, 13
  ; mov dh, 0
  ; mov dl, 0x80
  ; mov bx, STAGE2_DIR
  ; mov es, 0
  ; int 0x13

  jc read_stage2_error

  ; mov dh, 15
  ; cmp dh, al
  ; jne read_stage2_error2
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
  lea si, read_ok
  call print_string
  cli ; Desactivamos interrupciones
  
  lgdt [gdt_descriptor] ; Cargamos GDT

  mov eax, cr0  ; Activamos bit de modo protegido
  or eax, 1
  mov cr0, eax

  jmp CODE_SEG:start_32 ; Salto largo al segmento de código de la GDT


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

align 16
dap_packet:
  db 0x10
  db 0
  dw 8
  dw STAGE2_DIR
  dw STAGE2_DIR >> 4
  dq 76

read_msg db "Leyendo del disco...", 0
read_error db "Error al leer del disco (1)", 0
read_error2 db "Error al leer del disco (2)", 0
read_ok db "Disco leido! Cambiando a modo protegido...", 0
after_int db "INT13 returned!", 0


cli

STAGE2_DIR equ 0x8000

; times 510 - ($-$$) db 0
; dw 0xaa55   ; Magic number
ENTRY(_start)

SECTIONS {
  . = 0x8000;

  .text : {
    KEEP(*(.text.boot))
    *(.text .text.*)

    . = ALIGN(4);
    *(.rodata .rodata.*)
    *(.data .data.*)
    *(.got .got.*)
    *(.got.pls .got.plt.*)
  }

  /* .rodata : {
    *(.rodata .rodata.*)
  } */


  .bss : ALIGN(4) {
    *(.bss .bss.*)
    *(.sbss .sbss.*)
    *(COMMON)
  }
}
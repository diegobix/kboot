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

  .bss : ALIGN(4) {
    __bss_start = .;
    *(.bss .bss.*)
    *(.sbss .sbss.*)
    *(COMMON)
    __bss_end = .;
  }
}

PROVIDE(__bss_start = __bss_start);
PROVIDE(__bss_end = __bss_end);
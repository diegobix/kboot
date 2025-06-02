ENTRY(_start)

SECTIONS {
  . = 0x8000;

  .text.boot : {
    KEEP(*(.text.boot))
  }

  .text : {
    *(.text .text.*)
    *(.data .data.*)
    *(.sbss* .sbss.*)
    *(.rodata .rodata.*)
    *(.bss .bss.*)
  }

  /* .rodata : {
  }

  .data : {
  }

  .bss : {
  } */
}
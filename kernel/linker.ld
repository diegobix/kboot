ENTRY(kernel_main)

SECTIONS {
  . = 0x20000;

  .text.boot : {
    KEEP(*(.text.boot))
  }

  .text : {
    *(.text .text.*)
    *(.rodata .rodata.*)
    *(.data .data.*)
    *(.got .got.*)
    *(.got.pls .got.plt.*)
    *(.bss .bss.*)
    *(.sbss .sbss.*)
    *(.text.padding)
  }

  /DISCARD/ : {
    *(.comment)
  }
}
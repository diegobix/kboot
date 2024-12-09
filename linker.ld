ENTRY(init)

SECTIONS {
    . = 0x7c00;

    .text.stage1 : {
        *(.text.stage1)
    }

    .data.stage1 : {
        *(.data.stage1)
    }

    .magic 0x7dfe : {
        BYTE(0x55);
        BYTE(0xAA)
    }

    .text : {
        *(.text.stage2)
        *(.text.*)
    }
}
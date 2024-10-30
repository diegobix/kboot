ENTRY(init)

SECTIONS {
    . = 0x7c00;

    .text : {
        *(.text)
    }

    .data : {
        *(.data)
    }

    .magic : {
        *(.magic)
    }

    .magic 0x7dfe : {
        BYTE(0x55);
        BYTE(0xAA)
    }
}
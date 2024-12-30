OUTPUT_FORMAT(binary)
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

    _stage2_start = .;

    .text : {
        *(.text.stage2)
        *(.text.*)
    }

    .data : {
        *(.data.stage2)
        *(.data.*)
    }

    _stage2_end = .;

    PROVIDE(stage2_sectors = (_stage2_end - _stage2_start)/512 + 1);
}
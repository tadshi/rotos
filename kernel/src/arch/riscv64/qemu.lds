ENTRY(rotos_entry);
SECTIONS
{
    . = 0x80200000;
    .entry : {
        *(.entry)
    }
    .text : {
        *(.entry)
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata*)
    }
    .data : {
        *(.data*)
    }
    bss_start = .;
    .bss : {
        *(.bss*)
    }
    bss_end = .;
    kernel_end = .;
    .eh_frame : {
        *(.eh_frame)
    }
}


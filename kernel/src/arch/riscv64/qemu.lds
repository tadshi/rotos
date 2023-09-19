ENTRY(rotos_entry);
SECTIONS
{
    kernel_start = 0xffffffff80000000;
    . = 0xffffffff80200000;
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
    .eh_frame : {
        *(.eh_frame)
    }
    kernel_end = .;
}


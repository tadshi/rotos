ENTRY(rotos_entry);
SECTIONS
{
    kernel_start = 0xffffffff80000000;
    . = 0xffffffff80200000;
    .entry : {
        *(.entry)
    }
    .trap : {
        *(.trap)
    }
    .text : {
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata*)
    }
    .data : {
        *(.data* .sdata*)
    }
    bss_start = .;
    .bss : {
        *(.bss* .sbss*)
    }
    bss_end = .;
    .eh_frame : {
        *(.eh_frame)
    }
    kernel_end = .;
}

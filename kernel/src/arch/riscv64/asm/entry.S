.section .entry, "ax"
.globl rotos_entry
rotos_entry:
    csrw sscratch, zero

	li	t0, 1
	slli t0, t0, 18
	csrs sstatus, t0 //set SUM but not MXR

    la sp, kernel_stack

    j main
    c.nop

.data
.skip 4096
kernel_stack:

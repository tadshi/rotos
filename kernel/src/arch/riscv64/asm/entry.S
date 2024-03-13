#include "../../device.h"
.section .entry, "ax"
.globl rotos_entry
rotos_entry:
    csrw sscratch, zero
    csrw sie, zero

	li	t0, 1
	slli t0, t0, 18
	csrs sstatus, t0 //set SUM but not MXR

    addi t0, zero, -1
    slli t0, t0, 32
    la sp, kernel_stack
    or sp, sp, t0
    slli t1, a0, 12
    sub sp, sp, t1

    not t0, t0
    la t1, l1_page_table
    and t1, t0, t1
    srli t1, t1, 12

    li t2, 0x8
    slli t2, t2, 60
    or t1, t2, t1
    csrw satp, t1

    la t1, exp_handler
    addi t1, t1, 1
    slli t0, t0, 32
    or t1, t0, t1
    csrw stvec, t1

    la t1, main
    or t1, t0, t1
    jr t1
    c.nop

.data
.subsection 0
.p2align 12
l1_page_table:
.quad 0x000000000000002f
.quad 0x000000001000002f
.quad 0x000000002000002f
.quad 0x000000003000002f
.skip 504 * 8
.quad 0x000000000000002f
.quad 0x000000001000002f
.quad 0x000000002000002f
.quad 0x000000003000002f

.subsection 1
.p2align 3
.skip 4096 * HART_COUNT
.globl kernel_stack
kernel_stack:

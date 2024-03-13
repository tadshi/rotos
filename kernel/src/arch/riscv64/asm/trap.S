.macro SAVE_FULL_1
    csrrw sp, sscratch, sp
    c.sdsp x1, 0(sp)
    c.sdsp x3, 16(sp)
.endm
.macro SAVE_FULL_2
    c.sdsp x4, 24(sp)
    c.sdsp x5, 32(sp)
    c.sdsp x6, 40(sp)
    c.sdsp x7, 48(sp)
    c.sdsp x8, 56(sp)
    c.sdsp x9, 64(sp)
    c.sdsp x10, 72(sp)
    c.sdsp x11, 80(sp)
    c.sdsp x12, 88(sp)
    c.sdsp x13, 96(sp)
    c.sdsp x14, 104(sp)
    c.sdsp x15, 112(sp)
    c.sdsp x16, 120(sp)
    c.sdsp x17, 128(sp)
    c.sdsp x18, 136(sp)
    c.sdsp x19, 144(sp)
    c.sdsp x20, 152(sp)
    c.sdsp x21, 160(sp)
    c.sdsp x22, 168(sp)
    c.sdsp x23, 176(sp)
    c.sdsp x24, 184(sp)
    c.sdsp x25, 192(sp)
    c.sdsp x26, 200(sp)
    c.sdsp x27, 208(sp)
    c.sdsp x28, 216(sp)
    c.sdsp x29, 224(sp)
    c.sdsp x30, 232(sp)
    c.sdsp x31, 240(sp)
    csrr t0, sstatus
    csrr t1, sepc
    csrr t2, scause
    csrr t3, stval
    c.sdsp t0, 248(sp)
    c.sdsp t1, 256(sp)
    c.sdsp t2, 264(sp)
    c.sdsp t3, 272(sp)
    addi a0, sp, 280
    csrrw t0, sscratch, a0
    sd t0, 8(sp)
.endm
.macro SAVE_FULL
    SAVE_FULL_1
    SAVE_FULL_2
.endm
.macro CONVERT_STACK
    move t1, sp
    // Another dark alignment magic
    la t0, HART_CONTEXTS
    sub t0, sp, t0
    la sp, kernel_stack
    sub sp, sp, t0
    c.sdsp t1, 0(sp)
    c.addi sp, -8
.endm

.section .trap, "ax"
.p2align 3
.option norvc
.globl exp_handler
exp_handler:
j fault_handler
nop // Supervisor soft interrupt
.skip 12
j timer_handler
.skip 12
nop // Supervisor externel interrupt
.skip 4*54
.option rvc

timer_handler:
    SAVE_FULL

    li t0, 0x0020
    csrc sip, t0
    csrc sie, t0
    csrsi sstatus, 0x2

    CONVERT_STACK
    call schedule

    ld sp, 8(sp)
    csrci sstatus, 0x2
    li t0, 0x0020
    csrs sie, t0
    j pop_full_and_ret

fault_handler:
    SAVE_FULL
    li t0, 0x0020
    csrc sie, t2
    csrsi sstatus, 0x2

    csrr t0, scause
    andi a0, t0, 0x3f
    li  t0, 0x8
    beq a0, t0, syscall_handler

    CONVERT_STACK
    j unknown_fault_handler

    fault_fin:
    ld sp, 8(sp)
    csrci sstatus, 0x2
    li t0, 0x0020
    csrs sie, t0
    j pop_full_and_ret

syscall_handler:

    la t0, SYSCALL_HANDLERS
    ld t1, 72(sp)
    slli t1, t1, 3
    add t2, t0, t1
    
    c.ldsp a0, 80(sp)
    c.ldsp a1, 88(sp)
    c.ldsp a2, 96(sp)
    c.ldsp a3, 104(sp)
    CONVERT_STACK
    jalr t2
    j fault_fin
    // c.ldsp x15, 112(sp)
    // c.ldsp x16, 120(sp)
    // c.ldsp x17, 128(sp)

unknown_fault_handler:
    li t0, 0x6e6b6e55
    li t1, 0x206e776f
    li t2, 0x6c756166
    li t3, 0x78302074
    lui t4, 0x636f0
    srli t6, a0, 8
    addi t5, t6, 48
    slli t6, t6, 8
    sub t6, a0, t6
    addi t6, t6, 48
    add t5, t5, t6
    add t4, t4, t5
    li t5, 0x65727563
    li t6, 0x002e64
    c.swsp t0, 0(sp)
    c.swsp t1, 4(sp)
    c.swsp t2, 8(sp)
    c.swsp t3, 12(sp)
    c.swsp t4, 16(sp)
    c.swsp t5, 20(sp)
    c.swsp t6, 24(sp)
    move a0, sp
    call kcputs
    addi sp, sp, 24
    j fault_fin

pop_full_and_ret:
    csrr sp, sscratch
    addi sp, sp, -280
    c.ldsp t0, 248(sp)
    c.ldsp t1, 256(sp)
    c.ldsp t2, 264(sp)
    c.ldsp t3, 272(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw scause, t2
    csrw stval, t3
    c.ldsp x1, 0(sp)
    c.ldsp x3, 16(sp)
    c.ldsp x4, 24(sp)
    c.ldsp x5, 32(sp)
    c.ldsp x6, 40(sp)
    c.ldsp x7, 48(sp)
    c.ldsp x8, 56(sp)
    c.ldsp x9, 64(sp)
    c.ldsp x10, 72(sp)
    c.ldsp x11, 80(sp)
    c.ldsp x12, 88(sp)
    c.ldsp x13, 96(sp)
    c.ldsp x14, 104(sp)
    c.ldsp x15, 112(sp)
    c.ldsp x16, 120(sp)
    c.ldsp x17, 128(sp)
    c.ldsp x18, 136(sp)
    c.ldsp x19, 144(sp)
    c.ldsp x20, 152(sp)
    c.ldsp x21, 160(sp)
    c.ldsp x22, 168(sp)
    c.ldsp x23, 176(sp)
    c.ldsp x24, 184(sp)
    c.ldsp x25, 192(sp)
    c.ldsp x26, 200(sp)
    c.ldsp x27, 208(sp)
    c.ldsp x28, 216(sp)
    c.ldsp x29, 224(sp)
    c.ldsp x30, 232(sp)
    c.ldsp x31, 240(sp)
    ld sp, 8(sp)
    csrw sscratch, sp
    sret

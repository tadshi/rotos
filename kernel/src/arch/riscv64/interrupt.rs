use core::{arch::asm, ptr::addr_of, usize};
use crate::{arch::sbi::sbi_set_timer, utils::Bss};
use super::device::HART_COUNT;
use crate::config::limit::INTERRUPT_STACK_COUNT;

#[repr(C)]
pub struct RegisterEnv {
    xregs: [usize; 31],
    status: usize,
    epc: usize,
    cause: usize,
    tval: usize
}

impl Bss for RegisterEnv {
    const ZERO: Self = RegisterEnv {
        xregs: [0; 31],
        status: 0,
        epc: 0,
        cause: 0,
        tval: 0
    };
}

pub const HART_CONTEXT_ALIGN:usize = 4096;
#[repr(C, align(4096))]
pub struct HartContext {
    hart_id: usize,
    env_stack: [RegisterEnv; INTERRUPT_STACK_COUNT]
}

impl Bss for HartContext {
    const ZERO: Self = HartContext {
        hart_id: 0,
        env_stack: [RegisterEnv::ZERO; INTERRUPT_STACK_COUNT]
    };
}

#[no_mangle]
pub static mut HART_CONTEXTS: [HartContext; HART_COUNT] = [HartContext::ZERO; HART_COUNT];

pub fn init_interrupt(hart_id: usize) {
    unsafe {
        HART_CONTEXTS[hart_id].hart_id = hart_id;
        asm!(
            "csrw sscratch, {treg}",
            "li {treg}, 0x222",
            "csrs sie, {treg}",
            "csrsi sstatus, 0x2",
            treg = inout(reg) addr_of!(HART_CONTEXTS[hart_id].env_stack) => _,
        );
        sbi_set_timer(2000).expect("Failed to start timer.");
    }
}

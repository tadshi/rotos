pub mod atomic;
#[cfg_attr(target_arch = "riscv64", path = "riscv64/kconsole.rs")]
pub mod kconsole;
#[cfg_attr(target_arch = "riscv64", path = "riscv64/mem.rs")]
pub mod mem;

pub mod rtype {
    pub const SIZE_SHIFT: u32 = usize::BITS.ilog2();
}


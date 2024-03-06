pub mod atomic;
pub mod device;
#[cfg_attr(target_arch = "riscv64", path = "riscv64/kconsole.rs")]
pub mod kconsole;
#[cfg_attr(target_arch = "riscv64", path = "riscv64/mem.rs")]
pub mod mem;
#[cfg_attr(target_arch = "riscv64", path = "riscv64/interrupt.rs")]
pub mod interrupt;
#[cfg_attr(target_arch = "riscv64", path = "riscv64/page/mod.rs")]
pub mod page;
#[cfg_attr(target_arch = "riscv64", path = "riscv64/power.rs")]
pub mod power;
#[cfg(target_arch = "riscv64")]
#[path ="riscv64/sbi.rs"]
pub mod sbi;

pub mod rtype {
    pub const SIZE_SHIFT: u32 = usize::BITS.ilog2() - 3;
}

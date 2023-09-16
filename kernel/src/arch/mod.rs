pub mod spinlock;
#[cfg_attr(target_arch = "riscv64", path = "riscv64/kconsole.rs")]
pub mod kconsole;

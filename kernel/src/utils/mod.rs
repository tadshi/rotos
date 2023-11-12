pub mod kprint;
pub mod klist;
pub mod kerror;

pub trait Bss {
    const ZERO: Self;
}

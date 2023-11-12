use crate::utils::Bss;

pub struct RegisterEnv {
    xregs: [usize; 32]
}

impl Bss for RegisterEnv {
    const ZERO: Self = RegisterEnv {
        xregs: [0; 32]
    };
}
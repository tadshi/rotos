use std::{env, process::Command};
use core::panic;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Fail to read arch");
    println!("cargo:rustc-link-search=native=kernel/src/arch/{}/asm/", &arch);
    println!("cargo:rustc-link-lib=entry");
    if arch == "riscv64" {
        println!("cargo:rerun-if-changed=src/arch/riscv64/asm/entry.S");
    }
    let status = Command::new("make").arg(&arch)
                                        .status().expect("Fail to make kernel.");
    if !status.success() {
        panic!("Makefile failed.")
    }
    // print_cargo_env()
}

// fn print_cargo_env() {
//     for (key, value) in env::vars() {
//         if key.starts_with("CARGO_CFG_") {
//             println!("{}: {:?}", key, value);
//         }
//     }
//     panic!("!!")
// }

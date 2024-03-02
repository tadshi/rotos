use std::{env, fmt::write, fs::{self, File}, io::Write, process::Command, string};
use core::panic;
use toml::Table;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Fail to read arch");
    println!("cargo:rustc-link-search=native=kernel/lib/{}", &arch);
    println!("cargo:rustc-link-lib=entry");
    if arch == "riscv64" {
        println!("cargo:rerun-if-changed=src/arch/riscv64/asm/");
    }
    println!("cargo:rerun-if-changed=src/config/toml/");
    let status = Command::new("make").arg(&arch)
                                        .status().expect("Fail to make kernel.");
    if !status.success() {
        panic!("Makefile failed.")
    }
    make_config();
    // print_cargo_env()
}

fn make_config() {
    let mut config_rs = File::create("src/config/mod.rs").expect("Cannot create config rust file.");
    let config_dir = fs::read_dir("src/config/toml").expect("Cannot read config dir.");
    for file_res in config_dir {
        if file_res.is_err() {
            continue;
        }
        let config_file = file_res.unwrap();
        let filename = config_file.file_name().into_string().expect("Cannot parse config file name");
        write!(config_rs, "pub mod {} {{\n", &filename[..filename.len() - 5]).expect("Fail to write config file.");
        let config = fs::read_to_string(config_file.path()).expect("Cannot read config file.")
                                .parse::<Table>().expect("Fail to parse config file.");
        for (key, value) in config {
            match value {
                toml::Value::String(string) => write!(config_rs, "    pub const {}:str = \"{}\";\n", key, string),
                toml::Value::Integer(int64) => write!(config_rs, "    pub const {}:usize = {};\n", key, int64 as usize),
                toml::Value::Boolean(bool) => write!(config_rs, "    pub const {}:bool = {};\n", key, bool),
                _ => panic!("Type {:?} not supported for config file.", value)
            }.expect("Fail to write config file.");
        }
        config_rs.write(b"}\n").expect("Fail to write config file.");
    }
}

// fn print_cargo_env() {
//     for (key, value) in env::vars() {
//         if key.starts_with("CARGO_CFG_") {
//             println!("{}: {:?}", key, value);
//         }
//     }
//     panic!("!!")
// }

use std::{env, fs::{self, File}, io::Write, path::Path, process::Command};
use core::panic;
use toml::Table;

fn main() {
    let arch: String = env::var("CARGO_CFG_TARGET_ARCH").expect("Fail to read arch");
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
    let flags = env::var("CARGO_ENCODED_RUSTFLAGS")
                        .expect("Fail to read Cargo-set flags");
    for flag in flags.split('\x1f') {
        if flag.starts_with("device=") {
            make_device_info(&arch, &flag[8..flag.len() - 1])
        }
    }
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
        toml_to_rust(&config_file.path(), &mut config_rs, 4);
        config_rs.write(b"}\n").expect("Fail to write config file.");
    }
}

fn make_device_info(arch: &str, device_name: &str) {
    println!("{}:{}", arch, device_name);
    let device_desp_path = format!("src/arch/{}/device/{}.toml", arch, device_name);
    let mut rust_output = File::create("src/arch/device.rs").expect("Fail to creat device rust file");
    toml_to_rust(&device_desp_path, &mut rust_output, 0);
    let mut header_output = File::create("src/arch/device.h").expect("Fail to create device header file");
    toml_to_header(&device_desp_path, &mut header_output);
}

fn toml_to_rust(toml_path: &dyn AsRef<Path>, output: &mut dyn Write, ident:usize) {
    let config = fs::read_to_string(toml_path).expect("Cannot read toml file.")
            .parse::<Table>().expect("Fail to parse toml file.");
    for (key, value) in config {
        match value {
        toml::Value::String(string) => write!(output, "{}pub const {}:str = \"{}\";\n", " ".repeat(ident), key, string),
        toml::Value::Integer(int64) => write!(output, "{}pub const {}:usize = {};\n", " ".repeat(ident), key, int64 as usize),
        toml::Value::Boolean(bool) => write!(output, "{}pub const {}:bool = {};\n", " ".repeat(ident), key, bool),
        _ => panic!("Type {:?} not supported for toml to rust.", value)
        }.expect("Fail to write output rust file.");
    }
}

fn toml_to_header(toml_path: &dyn AsRef<Path>, output: &mut dyn Write) {
    let config = fs::read_to_string(toml_path).expect("Cannot read toml file.")
            .parse::<Table>().expect("Fail to parse toml file.");
    for (key, value) in config {
        match value {
        toml::Value::String(string) => write!(output, "#define {} {}\n", key, string),
        toml::Value::Integer(int64) => write!(output, "#define {} {}\n", key, int64),
        toml::Value::Boolean(bool) => write!(output, "#define {} {}\n", key, bool),
        _ => panic!("Type {:?} not supported for toml to header.", value)
        }.expect("Fail to write output header file.");
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

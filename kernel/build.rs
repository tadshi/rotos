use std::{env, fs::{self, File}, io::Write, path::Path, process::Command};
use core::panic;
use toml::Table;

fn main() {
    let arch: String = env::var("CARGO_CFG_TARGET_ARCH").expect("Fail to read arch");
    println!("cargo:rustc-link-search=native=kernel/lib/");
    println!("cargo:rustc-link-lib=asm");
    println!("cargo:rerun-if-changed=src/arch/{}/asm/", &arch);
    println!("cargo:rerun-if-changed=src/config/toml/");
    println!("cargo:rerun-if-env-changed=DEVICE");
    make_config();
    let device = env::var("DEVICE").expect("Fail to get target device.");
    parse_device_info(&arch, &device);
    println!("cargo:rustc-link-arg-bins=-Tkernel/src/arch/{}/{}.lds", &arch, &device);
    let status = Command::new("make").arg(format!("ARCH={}", &arch))
                                        .arg(format!("DEVICE={}", &device))
                                        .arg(format!("OUT_DIR={}", env::var_os("OUT_DIR").unwrap()
                                            .into_string().expect("Invalid target dir.")))
                                        .status().expect("Fail to invoke makefile.");
    if !status.success() {
        panic!("Fail to compile assembly files.")
    }
}

fn make_config() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let config_out_path =  Path::new(&out_dir).join("config.rs");
    let mut config_rs = File::create(config_out_path).expect("Cannot create config rust file.");
    let config_input_dir = fs::read_dir("config/").expect("Cannot read config dir.");
    for file_res in config_input_dir {
        if file_res.is_err() {
            continue;
        }
        let config_file = file_res.unwrap();
        let filename = config_file.file_name().into_string().expect("Cannot parse config file name");
        if !filename.ends_with(".toml") {
            continue;
        }
        write!(config_rs, "pub mod {} {{\n", &filename[..filename.len() - 5]).expect("Fail to write config file.");
        let config_table = fs::read_to_string(&config_file.path()).expect("Cannot read toml file.")
                .parse::<Table>().expect("Fail to parse toml file.");
        toml_to_rust(&config_table, &mut config_rs, 4);
        config_rs.write(b"}\n").expect("Fail to write config file.");
    }
}

fn parse_device_info(arch: &str, device_name: &str) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let device_out_path =  Path::new(&out_dir).join("device.rs");
    let device_desp_path = format!("device/{}/{}.toml", arch, device_name);
    let mut rust_output = File::create(device_out_path).expect("Fail to creat device rust file");
    let device_table = fs::read_to_string(device_desp_path).expect("Cannot read toml file.")
            .parse::<Table>().expect("Fail to parse toml file.");
    toml_to_rust(device_table["parameters"].as_table().unwrap(), &mut rust_output, 0);
    for (key, value) in device_table["features"].as_table().unwrap() {
        match value {
            toml::Value::String(string) => println!("cargo:rustc-cfg={}=\"{}\"", key, string),
            toml::Value::Boolean(bool) => if *bool { println!("cargo:rustc-cfg={}", key) },
            _ => panic!("Type {:?} not supported for features.", value)
        }
    }
}

fn toml_to_rust(table: &toml::Table, output: &mut dyn Write, ident:usize) {
    for (key, value) in table {
        match value {
        toml::Value::String(string) => write!(output, "{}pub const {}:str = \"{}\";\n", " ".repeat(ident), key, string),
        toml::Value::Integer(int64) => write!(output, "{}pub const {}:usize = {};\n", " ".repeat(ident), key, *int64 as usize),
        toml::Value::Boolean(bool) => write!(output, "{}pub const {}:bool = {};\n", " ".repeat(ident), key, bool),
        _ => panic!("Type {:?} not supported for toml to rust.", value)
        }.expect("Fail to write output rust file.");
    }
}

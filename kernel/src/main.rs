#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(format_args_nl)]

mod arch;
mod kprint;

#[no_mangle]
pub extern fn main() {
    kprintln!("Hello, {}!", "RotoS");
    panic!("Work in process...");
}

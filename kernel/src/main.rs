#![no_std]
#![no_main]
#![feature(panic_info_message, format_args_nl, strict_provenance)]

mod arch;
mod kprint;
mod page;

#[no_mangle]
pub extern fn main() {
    kprintln!("Hello, {}!", "RotoS");
    panic!("Work in process...");
}

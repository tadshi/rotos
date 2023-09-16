#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod arch;
mod panic;

#[no_mangle]
pub extern fn main() {
    panic!("Work in process...");
}

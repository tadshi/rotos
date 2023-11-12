#![no_std]
#![no_main]
#![feature(panic_info_message, format_args_nl, strict_provenance, never_type)]
#![feature(ptr_sub_ptr, maybe_uninit_uninit_array, const_mut_refs)]
#![feature(const_maybe_uninit_uninit_array)]

use core::ptr::{addr_of, addr_of_mut};

mod arch;
mod config;
mod kserver;
mod utils;

fn init_bss() {
    extern {
        static mut bss_start: u8;
        static bss_end: u8;
    }
    unsafe {
        core::ptr::write_bytes(addr_of_mut!(bss_start), 0,
         addr_of!(bss_end).sub_ptr(addr_of!(bss_start)))
    }
}

#[no_mangle]
pub extern fn main() {
    kprintln!("Hello, {}!", "RotoS");
    init_bss();
    kserver::KServerManager::init();
    panic!("Work in process...");
}

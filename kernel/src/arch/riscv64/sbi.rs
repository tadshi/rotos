use core::arch::asm;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum SbiError {
    SBI_SUCCESS,
    SBI_ERR_FAILED,
    SBI_ERR_NOT_SUPPORTED,
    SBI_ERR_INVALID_PARAM,
    SBI_ERR_DENIED,
    SBI_ERR_INVALID_ADDRESS,
    SBI_ERR_ALREADY_AVAILABLE,
    SBI_ERR_ALREADY_STARTED,
    SBI_ERR_ALREADY_STOPPED,
    SBI_ERR_NO_SHMEM,
    OTO_PARSE_FAILED(isize)
}

fn parse_sbi_error(error: isize) -> SbiError {
    use self::SbiError::*;
    static ERRORS: [SbiError;10] = [SBI_SUCCESS, SBI_ERR_FAILED, SBI_ERR_NOT_SUPPORTED, SBI_ERR_INVALID_PARAM,
                                    SBI_ERR_DENIED, SBI_ERR_INVALID_ADDRESS, SBI_ERR_ALREADY_AVAILABLE,
                                    SBI_ERR_ALREADY_STARTED, SBI_ERR_ALREADY_STOPPED, SBI_ERR_NO_SHMEM];
    match error {
        -9..=0 => ERRORS[-error as usize],
        _ => OTO_PARSE_FAILED(error)
    }
}

pub fn sbi_debug_console_write(num_bytes: usize,base_addr_lo: usize, base_addr_hi: usize) -> Result<usize, SbiError> {
    let errno: isize;
    let ret: isize;
    unsafe {
        asm!(
            "ecall",
            inout("a0") num_bytes => errno,
            inout("a1") base_addr_lo => ret,
            in("a2") base_addr_hi,
            in("a7") 0x4442434e,
            in("a6") 0
        );
    }
    match parse_sbi_error(errno) {
        SbiError::SBI_SUCCESS => Ok(ret as usize),
        other => Err(other)
    }
}

#[allow(dead_code)]
pub fn sbi_debug_console_read(num_bytes: usize,base_addr_lo: usize, base_addr_hi: usize) -> Result<usize, SbiError> {
    let errno: isize;
    let ret: isize;
    unsafe {
        asm!(
            "ecall",
            inout("a0") num_bytes => errno,
            inout("a1") base_addr_lo => ret,
            in("a2") base_addr_hi,
            in("a7") 0x4442434e,
            in("a6") 1
        );
    }
    match parse_sbi_error(errno) {
        SbiError::SBI_SUCCESS => Ok(ret as usize),
        other => Err(other)
    }
}

#[allow(dead_code)]
pub fn sbi_system_suspend(sleep_type: u32,resume_addr: usize, opaque: usize) -> Result<usize, SbiError> {
    let errno: isize;
    let ret: isize;
    unsafe{
        asm!(
            "ecall",
            inout("a0") sleep_type as isize => errno,
            inout("a1") resume_addr => ret,
            in("a2") opaque,
            in("a7") 0x53555350,
            in("a6") 1
        );
    }
    match parse_sbi_error(errno) {
        SbiError::SBI_SUCCESS => Ok(ret as usize),
        other => Err(other)
    }
}

pub fn sbi_hart_start(hart_id: usize, start_addr: usize, opaque: usize) -> Result<usize, SbiError> {
    let errno: isize;
    let ret: isize;
    unsafe{
        asm!(
            "ecall",
            inout("a0") hart_id => errno,
            inout("a1") start_addr => ret,
            in("a2") opaque,
            in("a7") 0x48534d,
            in("a6") 0
        );
    }
    match parse_sbi_error(errno) {
        SbiError::SBI_SUCCESS => Ok(ret as usize),
        other => Err(other)
    }
}

pub fn sbi_set_timer(stimer_value: u64) -> Result<usize, SbiError> {
    let errno: isize;
    let ret: isize;
    unsafe{
        asm!(
            "ecall",
            inout("a0") stimer_value => errno,
            lateout("a1") ret,
            in("a7") 0x54494d45,
            in("a6") 0
        );
    }
    match parse_sbi_error(errno) {
        SbiError::SBI_SUCCESS => Ok(ret as usize),
        other => Err(other)
    }
}

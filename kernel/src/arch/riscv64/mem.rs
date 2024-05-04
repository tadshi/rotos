use core::arch::asm;

pub const MEM_START:usize = 0x80000000;
pub const MEM_END:usize = 0xc0000000;
#[cfg(target_pointer_width = "64")]
const PADDR_MASK: usize = 0x00000000ffffffff;
const KADDR_PRED: usize = !PADDR_MASK;

pub fn paddr<T>(kvaddr: *const T) -> *const T {
    let ret: *const T;
    unsafe{
        asm!(
            "and {ret}, {kvaddr}, {mask}",
            ret = lateout(reg) ret,
            kvaddr = in(reg) kvaddr.addr(),
            mask = in(reg) PADDR_MASK
        )
    }
   ret
}

pub fn kaddr<T>(paddr: *const T) -> *const T {
    let ret: *const T;
    unsafe{
        asm!(
            "or {ret}, {paddr}, {mask}",
            ret = lateout(reg) ret,
            paddr = in(reg) paddr.addr(),
            mask = in(reg) KADDR_PRED
        )
    }
   ret
}

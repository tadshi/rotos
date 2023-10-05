use core::ptr;

pub const MEM_START:usize = 0x80000000;
pub const MEM_END:usize = 0xc0000000;
#[cfg(target_pointer_width = "64")]
const PADDR_MASK: usize = 0x00000000ffffffff;
const KADDR_PRED: usize = !PADDR_MASK;

pub fn paddr<T>(kvaddr: *const T) -> *const T {
    ptr::from_exposed_addr(kvaddr.expose_addr() & PADDR_MASK)
}

pub fn kaddr<T>(paddr: *const T) -> *const T {
    ptr::from_exposed_addr(paddr.expose_addr() | KADDR_PRED)
}

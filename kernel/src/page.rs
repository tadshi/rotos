use core::ptr;

#[cfg(target_pointer_width = "64")]
const PADDR_MASK: usize = 0x00000000ffffffff;
const KADDR_PRED: usize = !PADDR_MASK;

pub fn paddr<T>(kvaddr: *const T) -> *const T {
    ptr::from_exposed_addr(kvaddr.expose_addr() & PADDR_MASK)
}

pub fn kaddr<T>(paddr: *const T) -> *const T {
    ptr::from_exposed_addr(paddr.expose_addr() | KADDR_PRED)
}

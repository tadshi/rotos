pub const MEM_START:usize = 0x80000000;
pub const MEM_END:usize = 0xc0000000;
#[cfg(target_pointer_width = "64")]
const PADDR_MASK: usize = 0x00000000ffffffff;
const KADDR_PRED: usize = !PADDR_MASK;

// They are not even in a same address space!
// Maybe inline asm is the right way to handle this, this shouldn't be safe anyway
pub fn paddr<T>(kvaddr: *const T) -> *const T {
   kvaddr.with_addr(kvaddr.addr() & PADDR_MASK)
}

pub fn kaddr<T>(paddr: *const T) -> *const T {
    paddr.with_addr(paddr.addr() | KADDR_PRED)
}

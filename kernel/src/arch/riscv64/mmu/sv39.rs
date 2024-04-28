use crate::{arch::rtype::SIZE_SHIFT, kserver::page::PageSystem, utils::kerror::KError};

const VALID_BIT:  u64 = 0x1;
const READ_BIT: u64 = 0x2;
const WRITE_BIT: u64 = 0x4;
const EXEC_BIT: u64 = 0x8;
const USER_BIT: u64 = 0x10;
const GLOBAL_BIT: u64 = 0x20;
const ACCESSED_BIT: u64 = 0x40;
const DIRTY_BIT: u64 = 0x80;
const XWR_BITS: u64 = READ_BIT | WRITE_BIT | EXEC_BIT;

const PAGE_SHIFT: usize = 12;
const VPN_0_SHIFT: usize = PAGE_SHIFT;
const VPN_1_SHIFT: usize = PAGE_SHIFT + 9;
const VPN_2_SHIFT: usize = PAGE_SHIFT + 18;
const VPN_0_MASK: u64 = 0x1ff << VPN_0_SHIFT;
const VPN_1_MASK: u64 = 0x1ff << VPN_1_SHIFT;
const VPN_2_MASK: u64 = 0x1ff << VPN_2_SHIFT;
const PPN_MASK: u64 = 0x003ffffffffffc00;
const PPN_SHIFT: usize = 10;
const PAGE_MASK: u64 = PPN_MASK << (PAGE_SHIFT - PPN_SHIFT);

pub struct Sv39 {}

struct Sv39Entry {
    addr: *mut u64
}

impl Sv39Entry {

    fn from_ptr(addr: *mut u64) -> Self {
        Sv39Entry { addr }
    }

    fn set_page(&mut self, page_addr: usize, flags: u64) {
        assert_eq_size!(usize, u64);
        let ppn = (page_addr as u64 >> PAGE_SHIFT << PPN_SHIFT) & PPN_MASK;
        unsafe { self.addr.write(ppn | flags);}
    }

    fn child_page(&self, vpn: u64) -> Sv39Entry {
        unsafe {
            Sv39Entry {
                addr: self.addr.with_addr(
                    (((self.addr.read() << (PAGE_SHIFT - PPN_SHIFT)) as u64 & PAGE_MASK) | (vpn << SIZE_SHIFT)) as usize
                )
            }
        }
    }

    fn check_flag(&self, flags: u64) -> bool {
        // I guess maybe rust will optimize for this?
        unsafe {
            self.addr.read() & flags != 0
        }
    }
}

impl PageSystem for Sv39 {
    const PAGE_SHIFT: usize = 12;
    
    fn walk_pagedir_alloc(&mut self, pde: *mut usize, vaddr: usize, 
        mut page_alloc: impl FnMut() -> Result<usize, KError>) -> Result<*mut usize, KError> {
        let vaddr = vaddr as u64;
        let mut pde = Sv39Entry::from_ptr(pde.wrapping_offset(((vaddr & VPN_0_MASK) >> VPN_0_SHIFT) as isize) as *mut u64);
        if !pde.check_flag(VALID_BIT) {
            pde.set_page(page_alloc()?, VALID_BIT)
        }
        if pde.check_flag(XWR_BITS) {
            return Ok(pde.addr as *mut usize);
        }
        let mut psde = pde.child_page((vaddr & VPN_1_MASK) >> VPN_1_SHIFT);
        if !psde.check_flag(VALID_BIT) {
            psde.set_page(page_alloc()?, VALID_BIT)
        }
        if psde.check_flag(XWR_BITS) {
            return Ok(psde.addr as *mut usize);
        }
        let mut pte = psde.child_page((vaddr & VPN_2_MASK) >> VPN_2_SHIFT);
        if !pte.check_flag(VALID_BIT) {
            pte.set_page(page_alloc()?, VALID_BIT)
        }
        Ok(pte.addr as *mut usize)
    }

}

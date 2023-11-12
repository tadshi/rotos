use core::ptr::{addr_of, from_exposed_addr, from_exposed_addr_mut, read};

use crate::utils::kerror::KError;
use crate::{arch::mem::*, kprintln, prepare_k_list, utils::Bss, k_list_eforeach, from_prepared};
use crate::utils::klist::KLinkedList;
use crate::arch::page::helper::*;
use crate::arch::rtype::SIZE_SHIFT;

const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const PPAGE_NUM:usize = (MEM_END - MEM_START) >> PAGE_SHIFT;

struct PPageInfo {
    paddr: usize,
    used_count: u8
}

impl Bss for PPageInfo {
    const ZERO: Self = PPageInfo { paddr: 0, used_count: 0 };
}

prepare_k_list!(PPAGES_LIST [PPageInfo; PPAGE_NUM]);
extern {
    static kernel_end: u8;
}

pub struct PageManager {
    free: KLinkedList<PPageInfo>,
    used: KLinkedList<PPageInfo>
}

impl PageManager {
    pub(super) fn init() -> Result<PageManager, &'static str> {
        unsafe{
            let kernel_end_addr = addr_of!(kernel_end).addr();
            let mut first_user_ppn = kernel_end_addr >> PAGE_SHIFT;
            if first_user_ppn << PAGE_SHIFT != kernel_end_addr {
                first_user_ppn += 1;
            }
            k_list_eforeach!(PPAGES_LIST, |(index, pinfo)| {
                pinfo.paddr = MEM_START + (index << PAGE_SHIFT);
            });
            let ret = PageManager {
                free: from_prepared!(PPAGES_LIST, first_user_ppn, PPAGE_NUM),
                used: KLinkedList::new()
            };
            kprintln!("PageManager init successfully.");
            Ok(ret)
        }
    }

    pub fn alloc_ppage(&mut self) ->Result<usize, KError> {
        let page = self.free.pop_front().ok_or(KError::NotEnoughPage)?;
        page.get().used_count = 1;
        let ret = page.get().paddr;
        self.used.push_front(page);
        Ok(ret)
    }

    // Note: These impl should be moved to "mmu" module and guarded by cfg_attr someday    
    // Every page system should implement pub fns and use usize as input type
    // It is possible to make a trait for this but I guess that can be overhead.

    // V is set defaultly; If no perm is provided then it is a valid page dir
    #[cfg(page = "Sv39")]
    fn make_pte(&mut self, addr: *mut u64, perm: u64) -> Result<u64, KError> {
        let ppage = self.alloc_ppage()? as u64;
        let ppn = (ppage >> PAGE_SHIFT << PPN_SHIFT) & PPN_MASK;
        unsafe { addr.write(ppn | perm | VALID_BIT);}
        Ok(ppage)
    }

    #[cfg(page = "Sv39")]
    pub fn walk_pagedir_alloc(&mut self, pde_addr: usize, vaddr: usize) -> Result<usize, KError> {
        unsafe {
            let vaddr = vaddr as u64;
            let pde_addr: *mut u64 = address_pte(pde_addr as u64, (vaddr & VPN_0_MASK) >> VPN_0_SHIFT);
            let mut pde = read(pde_addr);
            if (pde & VALID_BIT) == 0 {
                pde = self.make_pte(pde_addr, 0)?;
            }
            if (pde & XWR_MASK) != 0 {
                return Ok(pde as usize);
            }
            let psde_addr = address_pte(pde << (PAGE_SHIFT - PPN_SHIFT), (vaddr & VPN_1_MASK) >> VPN_1_SHIFT);
            let mut psde = read(psde_addr);
            if (psde & VALID_BIT) == 0 {
                psde = self.make_pte(psde_addr, 0)?;
            }
            if (psde & XWR_MASK) != 0 {
                return Ok(pde as usize);
            }
            let pte_addr = address_pte(psde << (PAGE_SHIFT - PPN_SHIFT), (vaddr & VPN_2_MASK) >> VPN_2_SHIFT);
            let mut pte = read(pte_addr);
            if (pte & VALID_BIT) == 0 {
                pte = self.make_pte(pte_addr, 0)?
            }
            Ok(pte as usize)
        }
    }
}

#[cfg(page = "Sv39")]
#[inline]
fn address_pte(pte_addr_unmasked: u64, vpn: u64) -> *mut u64 {
    return from_exposed_addr_mut(((pte_addr_unmasked & PAGE_MASK) | (vpn << SIZE_SHIFT)) as usize);
}

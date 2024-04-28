use core::ptr::addr_of;

use crate::utils::kerror::KError;
use crate::{arch::mem::*, kprintln, prepare_k_list, utils::Bss, k_list_eforeach, from_prepared};
use crate::utils::klist::KLinkedList;
use crate::arch::mmu::PageType;

use super::Memory;

pub trait PageSystem {
    const PAGE_SHIFT: usize;
    fn walk_pagedir_alloc(&mut self, pde: *mut usize, vaddr: usize, 
        page_alloc: impl FnMut() -> Result<usize, KError>) -> Result<*mut usize, KError>;
}

assert_impl_all!(PageType: PageSystem);
const PAGE_SIZE: usize = 1 << PageType::PAGE_SHIFT;
const PPAGE_NUM:usize = (MEM_END - MEM_START) >> PageType::PAGE_SHIFT;

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
            let mut first_user_ppn = kernel_end_addr >> PageType::PAGE_SHIFT;
            if first_user_ppn << PageType::PAGE_SHIFT != kernel_end_addr {
                first_user_ppn += 1;
            }
            k_list_eforeach!(PPAGES_LIST, |(index, pinfo)| {
                pinfo.paddr = MEM_START + (index << PageType::PAGE_SHIFT);
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
}

impl Memory for PageManager {
    
}

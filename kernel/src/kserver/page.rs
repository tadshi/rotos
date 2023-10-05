use core::ptr::addr_of;

use crate::{arch::mem::*, kprintln, prepare_k_list, utils::{Bss, KLinkedList}, k_list_eforeach, from_prepared};
use super::KServerWrapper;

const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const PPAGE_NUM:usize = (MEM_END - MEM_START) >> PAGE_SHIFT;

struct PPageInfo {
    paddr: usize
}
unsafe impl Sync for PPageInfo {}
impl Bss for PPageInfo {
    const ZERO: Self = PPageInfo { paddr: 0 };
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
    pub(super) fn init_page() -> Result<PageManager, &'static str> {
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
}

impl<'a> TryFrom<KServerWrapper<'a>> for &'a PageManager {
    type Error = &'static str;

    fn try_from(value: KServerWrapper<'a>) -> Result<Self, Self::Error> {
        match value {
            KServerWrapper::Page(server) => Ok(server),
            _ => Err("Illegal unwrapping for kserver")
        }
    }
}

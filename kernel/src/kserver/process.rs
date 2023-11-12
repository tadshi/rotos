use crate::{from_prepared, kprintln, k_list_eforeach};
use crate::utils::{klist::KLinkedList, kerror::KError};
use crate::{arch::interrupt::RegisterEnv, utils::Bss, prepare_k_list};
use crate::config::MAX_PROCESS;

use super::{KServerWrapper, KServerManager};
pub struct ProcessInfo {
    pid: u32,
    pgid: u32,
    ppid: u32,
    pde_paddr: usize,
    regs: RegisterEnv
}

impl Bss for ProcessInfo {
    const ZERO: Self = ProcessInfo {
        pid: 0,
        pgid: 0,
        ppid: 0,
        pde_paddr: 0,
        regs: RegisterEnv::ZERO
    };
}

prepare_k_list!(PROCESS_LIST [ProcessInfo; MAX_PROCESS]);

pub struct ProcessManager {
    free: KLinkedList<ProcessInfo>,
    used: KLinkedList<ProcessInfo>
}

impl ProcessManager {
    pub fn init() -> Result<ProcessManager, &'static str> {
        unsafe {
            let ret = from_prepared!(PROCESS_LIST, 0, MAX_PROCESS);
            k_list_eforeach!(PROCESS_LIST, |(idx, pinfo)| {pinfo.pid = idx as u32});
            kprintln!("ProcessManager init successfully.");
            Ok(ProcessManager { free: ret, used: KLinkedList::new() })
        }
    }

    pub fn new_process(&mut self) -> Result<&mut ProcessInfo, KError> {
        let pde = KServerManager::with_page_mut(|page_server| {
            page_server.alloc_ppage()
        })?;
        let pinfo: &mut ProcessInfo = self.free.pop_front().ok_or(KError::NotEnoughProcess)?.get();
        pinfo.pde_paddr = pde;
        Ok(pinfo)
    }
}


use core::mem::MaybeUninit;

use crate::{arch::atomic::RWLock, utils::kerror::KError};
use page::PageManager;

use self::process::ProcessManager;

pub mod page;
pub mod process;

type Memory = PageManager;
type Process = ProcessManager;
pub type KServerManager = _KServerManager<Memory, Process>;

pub trait MemoryTrait {
    fn init() -> Result<Self, &'static str> where Self: Sized;
}

pub trait ProcessTrait {
    type PCB: Sized;
    fn init() -> Result<Self, &'static str> where Self: Sized;
    fn new_process(&mut self) -> Result<&mut Self::PCB, KError>;
}

// Be careful of deadlock!
// No cyclic use of server should be allowed. Take with care!
pub struct _KServerManager<M: MemoryTrait, P: ProcessTrait> {
    memory: RWLock<M>,
    process: RWLock<P>
}

static mut SERVER_MANAGER: MaybeUninit<KServerManager> = MaybeUninit::<KServerManager>::uninit();

impl KServerManager {
    fn manager() -> &'static mut Self{
        unsafe {
            SERVER_MANAGER.assume_init_mut()
        }
    }

    pub fn init() {
        unsafe {
            SERVER_MANAGER.write(
                _KServerManager {
                    memory: RWLock::new(Memory::init().unwrap()),
                    process: RWLock::new(Process::init().unwrap())
            });
        }
    }

    pub fn with_page<F, T>(operation: F) -> T where F: Fn(&Memory) -> T {
        let server = &KServerManager::manager().memory;
        let ret = operation(server.start_read_context());
        server.end_read_context();
        ret
    }

    pub fn with_page_mut<F, T>(operation: F) -> T where F: Fn(&mut Memory) -> T {
        let server = &mut KServerManager::manager().memory;
        let ret = operation(server.start_write_context());
        server.end_write_context();
        ret
    }

    pub fn with_process_mut<F, T>(operation: F) -> T where F: Fn(&mut Process) -> T {
        let server = &mut KServerManager::manager().process;
        let ret = operation(server.start_write_context());
        server.end_write_context();
        ret
    }

}

use core::mem::MaybeUninit;

use crate::arch::atomic::RWLock;
use page::PageManager;

use self::process::ProcessManager;

pub mod page;
pub mod process;

trait Memory {
    
}

trait Process {
    
}

// Be careful of deadlock!
// No cyclic use of server should be allowed. Take with care!
pub struct KServerManager {
    memory: RWLock<PageManager>,
    process: RWLock<ProcessManager>
}

static mut SERVER_MANAGER: MaybeUninit<KServerManager> = MaybeUninit::<KServerManager>::uninit();

impl KServerManager {
    pub fn init() {
        unsafe {
            SERVER_MANAGER.write(
                KServerManager {
                    memory: RWLock::new(PageManager::init().unwrap()),
                    process: RWLock::new(ProcessManager::init().unwrap())
            });
        }
    }

    pub fn get_manager() -> &'static mut Self{
        unsafe {
            SERVER_MANAGER.assume_init_mut()
        }
    }

    pub fn with_page<F, T>(operation: F) -> T where F: Fn(&PageManager) -> T {
        let server = &KServerManager::get_manager().memory;
        let ret = operation(server.start_read_context());
        server.end_read_context();
        ret
    }

    pub fn with_page_mut<F, T>(operation: F) -> T where F: Fn(&mut PageManager) -> T {
        let server = &mut KServerManager::get_manager().memory;
        let ret = operation(server.start_write_context());
        server.end_write_context();
        ret
    }

}

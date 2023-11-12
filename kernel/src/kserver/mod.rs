use core::mem::MaybeUninit;

use crate::arch::atomic::RWLock;
use page::PageManager;

use self::process::ProcessManager;

mod page;
mod process;
pub enum KServerType {
    PageServer,
    ProcessServer
}


pub enum KServerWrapper<'a> {
    Page(&'a RWLock<PageManager>),
    Process(&'a RWLock<ProcessManager>)
}

// Be careful of deadlock!
// No cyclic use of server should be allowed. Take with care!
// Can we do this?
pub struct KServerManager {
    page_server: RWLock<PageManager>,
    process_server: RWLock<ProcessManager>
}

static mut SERVER_MANAGER: MaybeUninit<KServerManager> = MaybeUninit::<KServerManager>::uninit();

impl<'a> KServerManager {
    pub fn init() {
        unsafe {
            SERVER_MANAGER.write(
                KServerManager {
                    page_server: RWLock::new(PageManager::init().unwrap()),
                    process_server: RWLock::new(ProcessManager::init().unwrap())
            });
        }
    }

    pub fn get_manager() -> &'static mut Self{
        unsafe {
            SERVER_MANAGER.assume_init_mut()
        }
    }

    #[deprecated]
    pub fn get_server(&self, req: KServerType) -> KServerWrapper {
        match req {
            KServerType::PageServer => KServerWrapper::Page(&self.page_server),
            KServerType::ProcessServer => KServerWrapper::Process(&self.process_server)
        }
    }

    pub fn with_page<F, T>(operation: F) -> T where F: Fn(&PageManager) -> T {
        let server = &KServerManager::get_manager().page_server;
        let ret = operation(server.start_read_context());
        server.end_read_context();
        ret
    }

    pub fn with_page_mut<F, T>(operation: F) -> T where F: Fn(&mut PageManager) -> T {
        let server = &mut KServerManager::get_manager().page_server;
        let ret = operation(server.start_write_context());
        server.end_write_context();
        ret
    }

}

// It may seems promising to define kernel servers as traits
// so as to have replacable kernel servers but remember we
// have no heap now and OOP with v-table could be impossible

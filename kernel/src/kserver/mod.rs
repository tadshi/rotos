use core::mem::MaybeUninit;

use crate::arch::atomic::RWLock;
use page::PageManager;

mod page;
pub enum KServerType {
    PageServer
}


pub enum KServerWrapper<'a> {
    Page(&'a PageManager)
}

pub struct KServerManager {
    page_server: PageManager
}

static mut SERVER_MANAGER: MaybeUninit<KServerManager> = MaybeUninit::<KServerManager>::uninit();

impl<'a> KServerManager {
    pub fn init() {
        unsafe {
            SERVER_MANAGER.write(
                KServerManager {
                    page_server: PageManager::init_page().unwrap()
            });
        }
    }

    pub fn get_manager() -> &'static mut Self{
        unsafe {
            SERVER_MANAGER.assume_init_mut()
        }
    }

    pub fn get_server(&'a self, req: KServerType) -> RWLock<KServerWrapper<'a>> {
        match req {
            KServerType::PageServer => RWLock::new(KServerWrapper::<'a>::Page(&self.page_server))
        }
    }
}

// It may seems promising to define kernel servers as traits
// so as to have replacable kernel servers but remember we
// have no heap now and OOP with v-table could be impossible

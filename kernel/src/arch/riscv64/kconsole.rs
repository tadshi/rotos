mod sbi;
use core::fmt::Write;
use core::fmt::Error;
use crate::arch::atomic::SpinLock;
use crate::arch::mem::paddr;

static mut KERNEL_CONSOLE: KConsole = KConsole { lock: SpinLock::new()};

pub struct KConsole {
    lock: SpinLock
}

impl KConsole {
    pub fn get_console() -> &'static mut KConsole {
        unsafe {
            &mut KERNEL_CONSOLE
        }
    }
}

impl Write for KConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.lock.lock();
        let ret = match sbi::sbi_debug_console_write(s.len(), paddr(s.as_ptr()) as usize, 0) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error)
        };
        self.lock.unlock();
        ret
    }
}

use core::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock {
    lock: AtomicBool
}

impl SpinLock {
    pub const fn new() -> SpinLock {
        SpinLock {
            lock: AtomicBool::new(false)
        }
    }

    pub fn try_lock(&self) -> bool {
        self.lock.compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed).is_ok()
    }

    pub fn lock(&self) {
        while self.lock.compare_exchange_weak(false, true, Ordering::AcqRel, Ordering::Relaxed).is_err() {}
    }

    pub fn unlock(&self) {
        while self.lock.compare_exchange_weak(true, false, Ordering::AcqRel, Ordering::Relaxed).is_err() {}
    }
}

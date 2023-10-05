use core::sync::atomic::{AtomicBool, Ordering, AtomicU32};

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

// A Reader-preferring lock.
pub struct MRSWLock {
    lock: SpinLock,
    count_lock: SpinLock,
    reader_count: AtomicU32
}

impl MRSWLock {
    fn new() -> Self {
        MRSWLock { lock: SpinLock::new(), count_lock: SpinLock::new(), reader_count: AtomicU32::new(0) }
    }
    fn reader_lock(&self) {
        self.count_lock.lock();
        let pv_reader_count = self.reader_count.fetch_add(1, Ordering::AcqRel);
        if pv_reader_count == 0 {
            self.lock.lock()
        }
        self.count_lock.unlock();
    }

    fn reader_unlock(&self) {
        self.count_lock.lock();
        let pv_reader_count = self.reader_count.fetch_sub(1, Ordering::AcqRel);
        if pv_reader_count == 1 {
            self.lock.unlock()
        }
        self.count_lock.unlock();
    }

    fn writer_lock(&self) {
        self.lock.lock()
    }

    fn writer_unlock(&self) {
        self.lock.lock()
    }
}

pub struct RWLock<T> {
    lock: MRSWLock,
    content: T
}

impl<T> RWLock<T> {
    pub fn new(content: T) -> Self {
        RWLock {
            lock: MRSWLock::new(),
            content
        }
    }

    pub fn start_read_context(&self) -> &T {
        self.lock.reader_lock();
        &self.content
    }

    pub fn end_read_context(&self) {
        self.lock.reader_unlock()
    }

    pub fn start_write_context(&mut self) -> &mut T{
        self.lock.writer_lock();
        &mut self.content
    }

    pub fn end_write_context(&self) {
        self.lock.writer_unlock()
    }
}

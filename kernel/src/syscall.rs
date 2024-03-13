use crate::config::limit::MAX_SYSCALL;

#[no_mangle]
static mut SYSCALL_HANDLERS: [usize; MAX_SYSCALL] = [0; MAX_SYSCALL];

pub fn register_handler(number: usize, handler: *const ()) {
    unsafe {
        SYSCALL_HANDLERS[number] = handler.addr()
    }
}

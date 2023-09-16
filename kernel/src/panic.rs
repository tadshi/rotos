use core::panic::PanicInfo;

use crate::arch::kconsole::KConsole;
use core::fmt::write;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    let _ = write(KConsole::get_console(), *_panic.message().unwrap());
    loop{
    }
}

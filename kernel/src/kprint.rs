use core::fmt::write;
use core::panic::PanicInfo;
use crate::arch::kconsole::KConsole;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    let _ = write(KConsole::get_console(), *_panic.message().unwrap());
    loop{
    }
}

// Note: This function is for kernel debug only.
#[macro_export]
macro_rules! kprint {
    ($fmt:expr $(, $args:expr)*) => {
        core::fmt::write($crate::arch::kconsole::KConsole::get_console(),
            core::format_args!($fmt, $($args),*)).expect("Kprint failed.");
    };
}

// Note: This function is for kernel debug only.
#[macro_export]
macro_rules! kprintln {
    ($fmt:expr $(,$args:expr)*) => {
        core::fmt::write($crate::arch::kconsole::KConsole::get_console(),
            core::format_args_nl!($fmt, $($args),*)).expect("Kprintln failed.");
    };
}

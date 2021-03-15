use core::panic::PanicInfo;
use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        log::error!("[kernel] Panic: {} at {}:{}", location.file(), location.line(), info.message().unwrap());
    } else {
        log::error!("[kernel] Panic: {}", info.message().unwrap());
    }
    shutdown()
}

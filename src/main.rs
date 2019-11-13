#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Entry Point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// Compiler Panic Handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
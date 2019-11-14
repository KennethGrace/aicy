#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// Entry Point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    write!(vga_buffer::WRITER.lock(), "Hello World!").unwrap();

    loop {}
}

// Compiler Panic Handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
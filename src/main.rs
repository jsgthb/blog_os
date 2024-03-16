#![no_std] // Disable standard library as it depends on underlying operating system
#![no_main] // Remove main function as it needs an underlying runtime

use core::panic::PanicInfo;

// Function is called on panic
// Required as panic handler is typically defined in standard libary
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

// Operating system entry point
#[no_mangle] // Disable name mangling so start function is not renamed on compilation
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("This is a test").unwrap();
    write!(vga_buffer::WRITER.lock(), "The magic number is {}", 42).unwrap();
    
    loop {}
}
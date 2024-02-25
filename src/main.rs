#![no_std] // Disable standard library as it depends on underlying operating system

use core::panic::PanicInfo;

// Function is called on panic
// Required as panic handler is typically defined in standard libary
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}

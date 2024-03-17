#![no_std] // Disable standard library as it depends on underlying operating system
#![no_main] // Remove main function as it needs an underlying runtime
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

// Function is called on panic
// Required as panic handler is typically defined in standard libary
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

mod vga_buffer;

// Operating system entry point
#[no_mangle] // Disable name mangling so start function is not renamed on compilation
pub extern "C" fn _start() -> ! {
    println!("This is test number {}", 42);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
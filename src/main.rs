#![no_std] // Disable standard library as it depends on underlying operating system
#![no_main] // Remove main function as it needs an underlying runtime
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;

// Function is called on panic
// Required as panic handler is typically defined in standard libary
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Operating system entry point
#[no_mangle] // Disable name mangling so start function is not renamed on compilation
pub extern "C" fn _start() -> ! {
    println!("Custom kernel has started");

    #[cfg(test)] // Only call test_main function on test compilation
    test_main();

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("Trivial assertion...");
    assert_eq!(1, 1);
    println!("[ok]")
}
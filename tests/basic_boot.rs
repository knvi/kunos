#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kunos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kunos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kunos::test_panic_handler(info)
}
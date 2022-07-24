#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kunos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kunos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    kunos::init();

    #[cfg(test)]
    test_main();

    println!("No crash!");

    kunos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    
    kunos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kunos::test_panic_handler(info)
}
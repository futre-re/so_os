#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(so_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use so_os::println;
//extern crate compiler_builtins;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    so_os::init();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    so_os::test_panic_handler(info)
}

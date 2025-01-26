#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(theta::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    theta::test_panic_handler(info);
}
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

use theta::println;
#[test_case]
fn print() {
    for i in 1..=100 {
        println!("Hello, {}!", i);
    }
}

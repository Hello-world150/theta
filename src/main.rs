#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(theta::test_runner)]
#![reexport_test_harness_main = "test_main"]

use theta::println;
mod panic_handler;

//入口函数
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello Theta!");

    theta::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("Did not panic!");
    loop {}
}

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(theta::test_runner)]
#![reexport_test_harness_main = "test_main"]

use theta::{hlt_loop, println};
mod panic_handler;

//入口函数
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello Theta!");

    theta::init();

    let ptr = 0xdeadbeaf as *mut u8;
    unsafe {
        *ptr = 42;
    }

    #[cfg(test)]
    test_main();

    hlt_loop();
}

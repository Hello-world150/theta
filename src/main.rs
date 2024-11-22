#![no_std] //禁用Rust标准库
#![no_main] //禁用Rust入口函数

use core::panic::PanicInfo;

mod vga_buffer;

//入口函数
#[no_mangle] //不重整
pub extern "C" fn _start() -> ! {
    println!("To you{}", ",Yan");
    loop {}
}

//panic处理函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

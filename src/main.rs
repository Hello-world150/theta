#![no_std] //不链接 Rust 标准库
#![no_main] //禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;

mod vga_buffer;

//入口函数
#[no_mangle] //不重整函数名
pub extern "C" fn _start() -> ! {
    println!("To you{}", ",Yan");
    //永不返回
    loop {}
}

//panic函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

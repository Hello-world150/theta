#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 入口点函数 
    loop {}
}

/// 入口点
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

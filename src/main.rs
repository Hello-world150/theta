#![no_std] //不链接 Rust 标准库
#![no_main] //禁用所有 Rust 层级的入口点

use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"Hello Theta!"; //声明字节形式的字符串`Hello Theta!`

//入口函数
#[no_mangle] //不重整函数名
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; //将0xb8000转化为裸指针

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; //偏移指针并写入`0xb`(青色)
        }
    }

    //永不返回 
    loop {}
}

//panic函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

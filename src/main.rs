#![no_std] //禁用Rust标准库
#![no_main] //禁用Rust入口函数
#![feature(custom_test_frameworks)] //启用自定义测试框架
#![test_runner(crate::test_runner)] //指定测试运行器
#![reexport_test_harness_main = "test_main"] //指定测试入口函数

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;


pub trait Testable {
    fn run(&self) -> ();
}
impl <T> Testable for T
    where T: Fn() {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
//测试运行器，接受一个测试函数的切片
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

//panic处理函数
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", _info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


//退出QEMU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


//入口函数
#[no_mangle] //不重整
pub extern "C" fn _start() -> ! {
    println!("Hello Theta!");

    #[cfg(test)]
    test_main();
    loop {}
}

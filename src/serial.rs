use ::spin::Mutex;
use lazy_static::lazy_static;
use uart_16550::SerialPort;

//使用时调用
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe {SerialPort::new(0x3F8)}; // 0x3F8是COM的标准起始端口号
        // 自动计算所有串口号，初始化串口
        serial_port.init();
        Mutex::new(serial_port)
    };
}

//内部调用，不对外暴露
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

//通过串口打印
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

//通过串口打印并换行
#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial_print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::serial_print!("{}\n", format_args!($($arg)*));
    };
}

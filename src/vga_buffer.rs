//实现VGA字符输出
use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[allow(dead_code)] //忽略无效代码
#[derive(Debug, Clone, Copy, PartialEq, Eq)] //自动实现`trait`
#[repr(u8)] //以`u8`形式存储枚举类型

pub enum Colors {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] //自动实现`trait`
#[repr(transparent)] //使用相同内存布局
struct Color(u8); //文本及背景颜色

impl Color {
    fn new(foreground: Colors, background: Colors) -> Color {
        Color((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] //自动实现`trait`
#[repr(C)] //使用C语言内存布局
struct Char {
    //单个字符
    ascii_character: u8,
    color: Color,
}

//缓冲区

pub const VGA_START_POINT: u32 = 0xb8000; //VGA映射起点
const BUFFER_HEIGHT: usize = 25; //缓冲区最大高度
const BUFFER_WIDTH: usize = 80; //缓冲区最大宽度

#[repr(transparent)] //使用相同内存布局
struct Buffer {
    chars: [[Volatile<Char>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize, //光标位置
    color: Color,
    buffer: &'static mut Buffer, //全局有效VGA缓冲区可变借用
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        //打印字节
        match byte {
            b'\n' => self.new_line(), //立即换行
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line(); //光标超出缓冲区宽度换行
                }

                let row = BUFFER_HEIGHT - 1;
                let column = self.column_position; //更新光标位置

                let color = self.color;
                self.buffer.chars[row][column].write(Char {
                    ascii_character: byte,
                    color,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        //打印字符串
        for byte in s.bytes() {
            match byte {
                //可打印字符或`\n`
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //非可打印字符
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            ascii_character: b' ',
            color: self.color,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

//延迟初始化`Writer`实例
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color: Color::new(Colors::Yellow, Colors::Black),
        buffer: unsafe { &mut *(VGA_START_POINT as *mut Buffer) },
    });
}

//`print`宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}

#[doc(hidden)] //隐藏实现细节
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

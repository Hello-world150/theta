//实现VGA字符输出

use volatile::Volatile;
use core::fmt;

#[allow(dead_code)] //忽略无效代码
#[derive(Debug, Clone, Copy, PartialEq, Eq)] //自动实现trait
#[repr(u8)] //以u8形式存储枚举类型
pub enum Color {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)] //自动实现trait
#[repr(transparent)]
struct ColorCode(u8); //封装单个字符颜色

impl ColorCode {
    //实现new方法
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] //自动实现trait
#[repr(C)] //使用C语言内存布局
struct ScreenChar { //封装单个字符
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25; //设置缓冲区最大高度
const BUFFER_WIDTH: usize = 80; //设置缓冲区最大宽度

#[repr(transparent)]
struct Buffer { //抽象VGA缓冲区
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//用于打印字符的结构体
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer, //全局有效的VGA缓冲区可变借用
}

//实现打印方法
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();

            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                //可打印字符或\n
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //非可打印字符
                _ => self.write_byte(0xfe),
            }
        }
    }
}

//实现格式化宏
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

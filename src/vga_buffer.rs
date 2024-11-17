//实现VGA字符输出
use core::fmt;
use volatile::Volatile;

#[allow(dead_code)] //忽略无效代码
#[derive(Debug, Clone, Copy, PartialEq, Eq)] //自动实现trait
#[repr(u8)] //以u8形式存储枚举类型

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)] //自动实现trait
#[repr(transparent)] //使用u8相同内存布局
struct Color(u8); //单个字符颜色

#[allow(dead_code)]
impl Color {
    fn new(foreground: Colors, background: Colors) -> Color {
        //实现new方法
        Color((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] //自动实现trait
#[repr(C)] //使用C语言内存布局
struct Char {
    //封装单个字符
    ascii_character: u8,
    color: Color,
}

//缓冲区

pub const VGA_START_POINT: u32 = 0xb8000; //VGA内存起点
const BUFFER_HEIGHT: usize = 25; //设置缓冲区最大高度
const BUFFER_WIDTH: usize = 80; //设置缓冲区最大宽度

#[repr(transparent)] //使用相同内存布局
struct Buffer {
    //VGA缓冲区
    chars: [[Volatile<Char>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize, //光标位置
    color: Color,
    buffer: &'static mut Buffer, //全局有效的VGA缓冲区可变借用
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        //按字节打印
        match byte {
            b'\n' => self.new_line(), //立即换行
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line(); //当光标超出缓冲区宽度换行
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
        //按字符串打印
        for byte in s.bytes() {
            match byte {
                //可打印字符或\n
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //非可打印字符
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let _character = self.buffer.chars[row][col].read();
            }
        }
    }
}

//覆写内置Write宏
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color: Color::new(Colors::Yellow, Colors::Black),
        buffer: unsafe { &mut *(VGA_START_POINT as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello! ");
    write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
}

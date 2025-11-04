
use core::prelude::rust_2024::derive;
use core::cmp::Eq;
use core::fmt::Debug;
use core::clone::Clone;
use core::cmp::PartialEq;
use core::marker::Copy;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground : Color, background : Color) -> ColorCode {
        ColorCode(((background as u8) << 4) | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_char : u8,
    color_code : ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Buffer {
    chars : [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub cur_row: usize,
    pub cur_col: usize,
    pub color_code: ColorCode,
    pub buffer : &'static mut Buffer,
}

impl Writer {

    fn write_byte(&mut self, byte : u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.cur_col == BUFFER_WIDTH {
                    self.new_line()
                }
                let color = self.color_code;
                self.buffer.chars[self.cur_row][self.cur_col] = ScreenChar{
                    ascii_char: byte,
                    color_code: color,
                };
                self.cur_col += 1;
            }
        }
    }

    pub fn write_string(&mut self, string : &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }

    fn new_line(&mut self) {
        self.cur_col = 0;
        self.cur_row += 1;
        if self.cur_row == BUFFER_HEIGHT {
            for i in 1..BUFFER_HEIGHT {
                self.buffer.chars[i-1] = self.buffer.chars[i];
            }
            self.cur_row = BUFFER_HEIGHT - 1;
            self.buffer.chars[BUFFER_HEIGHT-1] = [ScreenChar{
                ascii_char : b' ',
                color_code : ColorCode::new(Color::Yellow, Color::Black),
            }; BUFFER_WIDTH]
        }
    }
}


// pub fn print_something() {
//     let mut TESTBUFF :  [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[ScreenChar{
//         ascii_char : b' ',
//         color_code : ColorCode::new(Color::Yellow, Color::Black),
//     }; BUFFER_WIDTH]; BUFFER_HEIGHT];

//     let mut writer = Writer{
//         cur_row : BUFFER_HEIGHT-1,
//         cur_col : 0,
//         color_code : ColorCode::new(Color::Yellow, Color::Black),
//         buffer : unsafe { &mut *( core::ptr::addr_of_mut!(TESTBUFF) as *mut Buffer) },
//     };
//     writer.write_byte(b'H');
//     writer.write_string("HHHHHHHHHHHHHHHEEEEEEEEEEEEELLLLLLLLLLLLLLLLLOOOOOOOOOOOOOOOOOOOOO");
//     writer.write_string("HHHHHHHHHHHHHHHEEEEEEEEEEEEELLLLLLLLLLLLLLLLLOOOOOOOOOOOOOOOOOOOOO");
//     println!("{}", 'H' as u8);
//     // println!("{}", TESTBUFF[0][0] as u8);

//     for row in TESTBUFF.iter() {
//         for ch in row.iter() {
//             print!("{}", ch.ascii_char);
//         }
//         println!();
//     }
 
// }
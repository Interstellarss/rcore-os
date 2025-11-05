use core::prelude::rust_2024::derive;
use core::cmp::Eq;
use core::fmt::Debug;
use core::clone::Clone;
use core::cmp::PartialEq;
use core::marker::Copy;

use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer{
        cur_row: 0,
        cur_col: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    });
}

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

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Buffer {
    chars : [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
                self.buffer.chars[self.cur_row][self.cur_col].write(ScreenChar {
                    ascii_char: byte,
                    color_code: color,
                });
                self.cur_col += 1;
            }
        }
    }

    pub fn write_string(&mut self, string : &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }

    // fn new_line(&mut self) {
    //     self.cur_col = 0;
    //     self.cur_row += 1;
    //     if self.cur_row == BUFFER_HEIGHT {
    //         for i in 1..BUFFER_HEIGHT {
    //             self.buffer.chars[i-1] = self.buffer.chars[i];
    //         }
    //         self.cur_row = BUFFER_HEIGHT - 1;
    //         self.buffer.chars[BUFFER_HEIGHT-1].write([ScreenChar{
    //             ascii_char : b' ',
    //             color_code : ColorCode::new(Color::Yellow, Color::Black),
    //         }; BUFFER_WIDTH])
    //     }
    // }
    fn new_line(&mut self) {
        self.cur_col = 0;
        if self.cur_row < BUFFER_HEIGHT - 1 {
            self.cur_row += 1;
            return;
        }
  
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let ch = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(ch);
            }
        }
  
        self.clear_row(BUFFER_HEIGHT - 1);
    }

    fn clear_row(&mut self, row: usize){
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank);
        }

    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s : &str) -> fmt::Result{
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print{
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments){
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });

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



#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::Writer;
use vga_buffer::Color;
use vga_buffer::ColorCode;
use crate::vga_buffer::Buffer;

// static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    let mut writer = Writer{
        cur_row : 0,
        cur_col : 0,
        color_code : ColorCode::new(Color::Yellow, Color::Black),
        buffer : unsafe { &mut *( 0xb8000 as *mut Buffer) },
    };

    writer.write_string("Hello this is the first line!\n");
    writer.write_string("Hello this is the second line!\n");

    loop {}
}

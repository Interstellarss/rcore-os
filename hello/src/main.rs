#![no_std]
#![no_main]


// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
//mod vga_buffer;
use core::panic::PanicInfo;
use rosh::println;
// use vga_buffer::Writer;
// use vga_buffer::Color;
// use vga_buffer::ColorCode;
//use crate::vga_buffer::Buffer;

// static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    rosh::hlt_loop();
}



#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // let mut writer = Writer{
    //     cur_row : 0,
    //     cur_col : 0,
    //     color_code : ColorCode::new(Color::Yellow, Color::Black),
    //     buffer : unsafe { &mut *( 0xb8000 as *mut Buffer) },
    // };
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_string("Hello again");

    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337);
    println!("Hello World{}", "!");

    rosh::init();

    //x86_64::instructions::interrupts::int3();

    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // }

    println!("It did not crash!");

    rosh::hlt_loop();
}


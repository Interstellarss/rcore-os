#![no_std]
#![no_main]


use core::panic::PanicInfo;
use rosh::println;
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::Page;


entry_point!(kernel_main);

//#[unsafe(no_mangle)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {

    use rosh::memory::{self, BootInfoFrameAllocator};
    use x86_64::{structures::paging::Translate, VirtAddr};
    //use rosh::memory::translate_addr;
    
    println!("Hello World{}", "!");

    rosh::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    // init a mapper
    let mut mapper = unsafe {memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {BootInfoFrameAllocator::init(&boot_info.memory_map)};

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };


    println!("It did not crash!");

    rosh::hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    rosh::hlt_loop();
}


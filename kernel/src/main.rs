#![no_std]
#![no_main]
#![feature(naked_functions)]

mod display;
mod interrupts;

use bootloader_api::{entry_point, info::FrameBuffer, BootInfo};
use core::panic::PanicInfo;
use display::char_writer::CHAR_WRITER;
use display::framebuffer_writter::FRAMEBUFFER_WRITER;
use interrupts::idt::IDT;

/// Function acting as panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn load_idt() {
    println!("Loading idt...");
    unsafe {
        IDT.init();
        IDT.load();
    }
    println!("IDT loaded.");
}

fn init_framebuffer_writter(boot_info: &'static mut BootInfo) {
    let framebuffer: &mut FrameBuffer = boot_info.framebuffer.as_mut().unwrap();
    let width = framebuffer.info().width;
    let height = framebuffer.info().height;
    let bytes_per_pixel = framebuffer.info().bytes_per_pixel;
    let stride = framebuffer.info().stride;
    FRAMEBUFFER_WRITER
        .lock()
        .set(display::framebuffer_writter::FrameBufferWriter::init(
            framebuffer.buffer_mut(),
            width,
            height,
            bytes_per_pixel,
            stride,
        ))
        .unwrap();
    println!("Try clearing screen...");
    CHAR_WRITER.lock().clear();
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init_framebuffer_writter(boot_info);
    load_idt();
    loop {}
}

entry_point!(kernel_main);

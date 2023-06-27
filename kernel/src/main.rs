#![no_std]
#![no_main]
#![feature(naked_functions)]

mod display;
mod interrupts;

use bootloader_api::{entry_point, info::FrameBuffer, BootInfo};
use core::panic::PanicInfo;
use interrupts::idt::IDT;

/// Function acting as panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn load_idt() {
    unsafe {
        IDT.init();
        IDT.load();
    }
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer: &mut FrameBuffer = boot_info.framebuffer.as_mut().unwrap();
    let width = framebuffer.info().width;
    let height = framebuffer.info().height;
    let bytes_per_pixel = framebuffer.info().bytes_per_pixel;
    let stride = framebuffer.info().stride;
    let framebufferwritter = display::framebuffer_writter::FrameBufferWriter::new(
        framebuffer.buffer_mut(),
        width,
        height,
        bytes_per_pixel,
        stride,
    );
    let mut char_writter = display::char_writter::CharWritter::new(framebufferwritter);
    load_idt();
    loop {}
}

entry_point!(kernel_main);

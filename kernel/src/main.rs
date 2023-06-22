#![no_std]
#![no_main]

use bootloader_api::{entry_point, info::FrameBuffer, BootInfo};
mod display;
mod interrupts;

/// Specify main function not existing
use core::panic::PanicInfo;

/// Function acting as panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
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
    char_writter.clear();
    for i in 0..50 {
        if i % 2 == 0 {
            char_writter.write_string("HUGO");
        }
        if i % 2 == 1 {
            char_writter.write_string("LUCAS");
        }
        char_writter.newline();
    }

    loop {}
}

entry_point!(kernel_main);

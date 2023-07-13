/// Structure for writing pixels in the framebuffer.
/// It should be the only object writing in the framebuffer.
/// References: https://github.com/rust-osdev/bootloader/blob/main/common/src/framebuffer.rs
use lazy_static::lazy_static;
use once_cell::unsync::OnceCell;
use spin::Mutex;

lazy_static! {
    /// The global frame buffer writter instance.
    pub static ref FRAMEBUFFER_WRITER: Mutex<OnceCell<FrameBufferWriter>> =
        Mutex::new(OnceCell::new());
}

#[derive(Debug)]
pub struct FrameBufferWriter {
    /// framebuffer physical address
    framebuffer: &'static mut [u8],
    /// screen width
    width: usize,
    /// screen height
    height: usize,
    /// bytes per pixel
    bytes_per_pixel: usize,
    /// vertical space between pixels
    pitch: usize,
}

impl FrameBufferWriter {
    /// Returns a new FrameBufferWritter.
    ///
    /// ## Arguments
    /// * `framebuffer`: framebuffer physical address
    /// * `width`: display width
    /// * `height`: display height
    /// * `bytes_per_pixel`: number of bytes per pixel
    /// * `pitch`: vertical space between pixels
    pub fn init(
        framebuffer: &'static mut [u8],
        width: usize,
        height: usize,
        bytes_per_pixel: usize,
        pitch: usize,
    ) -> Self {
        Self {
            framebuffer,
            width,
            height,
            bytes_per_pixel,
            pitch,
        }
    }

    /// Returns display width.
    pub fn width(&self) -> usize {
        self.width
    }
    /// Returns display height
    pub fn height(&self) -> usize {
        self.height
    }

    /// Write the color in the pixel.
    ///
    /// ## Arguments
    /// * `x_width`: the position x in the width axis
    /// * `y_height`: the position y in the height axis
    /// * `rgb`: RGB value
    pub fn write_pixel(&mut self, x_width: usize, y_height: usize, rgb: [u8; 3]) {
        // Real color on screen
        // Color is RGBA with endianess 0xAARRGGBB [blue, green, red, alpha]
        let color = [rgb[2], rgb[1], rgb[0], 0];
        // Position of the pixel with pitch
        let position = (y_height * self.pitch) + x_width;
        // Add byte offset
        let byte_offset: usize = position * self.bytes_per_pixel;
        // Copy the color into framebuffer
        self.framebuffer[byte_offset..(byte_offset + self.bytes_per_pixel)]
            .copy_from_slice(&color[..self.bytes_per_pixel]);
    }
}

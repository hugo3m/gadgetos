use crate::display::framebuffer_writter::FRAMEBUFFER_WRITER;
use lazy_static::lazy_static;
use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};
use spin::Mutex;

lazy_static! {
    /// The global frame buffer writter instance.
    pub static ref CHAR_WRITER: Mutex<CharWriter> = Mutex::new(CharWriter {
        x_width: BORDER_PADDING,
        y_height: BORDER_PADDING,
    });
}

// Vertical space between lines.
const LINE_SPACING: usize = 2;
// Horizontal space between characters.
const LETTER_SPACING: usize = 0;
// Padding from the border.
const BORDER_PADDING: usize = 1;
// Height of each char raster. The font size is ~0.84% of this. Thus, this is the line height that
// enables multiple characters to be side-by-side and appear optically in one line in a natural way.
const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
// The width of each single symbol of the mono space font.
const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
// Backup character if a desired symbol is not available by the font.
const BACKUP_CHAR: char = '�';
// Characters font weight
const FONT_WEIGHT: FontWeight = FontWeight::Regular;

/// Returns the raster of the given char or the raster of BACKUP_CHAR.
///
/// ## Arguments
/// * `c`: char
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).unwrap())
}
/// Structure used to write char on the display.
pub struct CharWriter {
    // X position on width
    x_width: usize,
    // Y position on height
    y_height: usize,
}

impl CharWriter {
    /// Clear the display by setting all pixels to black.
    pub fn clear(&mut self) {
        let width = FRAMEBUFFER_WRITER.lock().get().unwrap().width();
        let height = FRAMEBUFFER_WRITER.lock().get().unwrap().height();
        // Set pixels to black
        for i in 0..width {
            for j in 0..height {
                FRAMEBUFFER_WRITER
                    .lock()
                    .get_mut()
                    .unwrap()
                    .write_pixel(i, j, [0, 0, 0]);
            }
        }
        // Set position to top left
        self.x_width = BORDER_PADDING;
        self.y_height = BORDER_PADDING;
    }
    /// Process a carriage return.
    fn carriage_return(&mut self) {
        self.x_width = BORDER_PADDING;
    }
    /// Jump to a new line.
    /// When jumping above the height of the screen, clear the screen.
    pub fn newline(&mut self) {
        let height = FRAMEBUFFER_WRITER.lock().get().unwrap().height();
        // Increment y position
        self.y_height += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        // If position is above the height limit
        if (self.y_height + CHAR_RASTER_HEIGHT.val() + BORDER_PADDING) >= height {
            // Clear the screen
            self.clear()
        }
        // Process carriage return
        self.carriage_return()
    }
    /// Write a char.
    fn write_char(&mut self, c: char) {
        let width = FRAMEBUFFER_WRITER.lock().get().unwrap().width();
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                // Calculate new X position
                let next_x_width = self.x_width + CHAR_RASTER_WIDTH;
                // If above the screen width
                if next_x_width >= width {
                    // Process new line
                    self.newline()
                }
                // Write the char on screen
                self.write_char_render(get_char_raster(c))
            }
        }
    }
    /// Write the char render on screen.
    fn write_char_render(&mut self, char_render: RasterizedChar) {
        // For each pixels row of the char
        for (y, row) in char_render.raster().iter().enumerate() {
            // For each pixel of the char
            for (x, byte) in row.iter().enumerate() {
                // Write the pixel
                FRAMEBUFFER_WRITER.lock().get_mut().unwrap().write_pixel(
                    self.x_width + x,
                    self.y_height + y,
                    [*byte, *byte, *byte],
                );
            }
        }
        // Increment X position
        self.x_width += char_render.width() + LETTER_SPACING;
    }
    /// Write a string on the screen
    ///
    /// ## Arguments
    /// * `value`: value to write on the string
    pub fn write_string(&mut self, value: &str) {
        for c in value.chars() {
            self.write_char(c);
        }
    }
}

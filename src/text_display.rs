use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use noto_sans_mono_bitmap::{get_bitmap, get_bitmap_width, BitmapChar, BitmapHeight, FontWeight};

/// Additional vertical space between lines
const LINE_SPACING: usize = 0;
/// Font size
const FONT_SIZE: usize = 14;
const BITMAP_LETTER_WIDTH: usize = get_bitmap_width(FontWeight::Regular, BitmapHeight::Size14);

/// Global object for printing to the framebuffer
pub static mut TEXTDISPLAY: Option<TextDisplay<'static>> = None;

pub fn init_text_display(framebuffer: &'static mut FrameBuffer) {
    unsafe { TEXTDISPLAY = Some(TextDisplay::new(framebuffer)) };
}

pub struct TextDisplay<'a> {
    info: FrameBufferInfo,
    framebuffer: &'a mut [u8],
    x_pos: usize,
    y_pos: usize,
}

impl<'a> TextDisplay<'a> {
    /// Creates a new logger that uses the given framebuffer.
    pub fn new(framebuffer: &'a mut FrameBuffer) -> Self {
        let info = framebuffer.info();
        let framebuffer = framebuffer.buffer_mut();
        let mut logger = Self {
            info,
            framebuffer,
            x_pos: 0,
            y_pos: 0,
        };
        logger.clear();
        logger
    }

    fn newline(&mut self) {
        self.y_pos += FONT_SIZE + LINE_SPACING;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x_pos = 0;
    }

    /// Erases all text on the screen.
    pub fn clear(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.horizontal_resolution
    }

    fn height(&self) -> usize {
        self.info.vertical_resolution
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                if self.width() <= self.x_pos {
                    self.newline();
                }
                if (self.height() - BITMAP_LETTER_WIDTH) <= self.y_pos {
                    self.clear();
                }
                let bitmap_char = get_bitmap(c, FontWeight::Regular, BitmapHeight::Size14)
                    .unwrap_or_else(|| {
                        get_bitmap(' ', FontWeight::Regular, BitmapHeight::Size14).unwrap()
                    });
                self.write_rendered_char(bitmap_char);
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: BitmapChar) {
        for (y, row) in rendered_char.bitmap().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width();
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::RGB => [intensity, intensity, intensity / 2, 0],
            PixelFormat::BGR => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            _ => unreachable!(),
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
    }
}

impl<'a> core::fmt::Write for TextDisplay<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

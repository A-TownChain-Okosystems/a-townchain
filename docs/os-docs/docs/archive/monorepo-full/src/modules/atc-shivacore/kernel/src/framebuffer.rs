// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — Framebuffer-Textausgabe.
// bootloader 0.11 startet den Kernel im linearen Pixel-Framebuffer-Modus
// (kein klassisches VGA-Text @0xb8000 mehr). Dieses Modul schreibt Text
// direkt als gerasterte Glyphen in den vom Bootloader übergebenen Buffer.

use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use core::fmt;
use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};
use spin::Mutex;

const CHAR_HEIGHT: RasterHeight = RasterHeight::Size16;
const CHAR_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_HEIGHT);
const LINE_SPACING: usize = 2;

pub struct FbWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x: usize,
    y: usize,
}

unsafe impl Send for FbWriter {}

impl FbWriter {
    pub fn new(buffer: &'static mut FrameBuffer) -> Self {
        let info = buffer.info();
        Self {
            framebuffer: buffer.buffer_mut(),
            info,
            x: 0,
            y: 0,
        }
    }

    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for py in 0..self.info.height {
            for px in 0..self.info.width {
                self.set_pixel(px, py, r, g, b);
            }
        }
        self.x = 0;
        self.y = 0;
    }

    fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x >= self.info.width || y >= self.info.height {
            return;
        }
        let bpp = self.info.bytes_per_pixel;
        let offset = y * self.info.stride * bpp + x * bpp;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [r, g, b, 0],
            PixelFormat::Bgr => [b, g, r, 0],
            PixelFormat::U8 => [((r as u16 + g as u16 + b as u16) / 3) as u8, 0, 0, 0],
            _ => [r, g, b, 0],
        };
        if offset + bpp <= self.framebuffer.len() {
            self.framebuffer[offset..offset + bpp].copy_from_slice(&color[..bpp]);
        }
    }

    fn newline(&mut self) {
        self.x = 0;
        self.y += CHAR_HEIGHT.val() + LINE_SPACING;
        if self.y + CHAR_HEIGHT.val() >= self.info.height {
            self.y = 0;
        }
    }

    fn write_char(&mut self, c: char) {
        if c == '\n' {
            self.newline();
            return;
        }
        if self.x + CHAR_WIDTH >= self.info.width {
            self.newline();
        }
        if let Some(raster) = get_raster(c, FontWeight::Regular, CHAR_HEIGHT) {
            for (row, line) in raster.raster().iter().enumerate() {
                for (col, intensity) in line.iter().enumerate() {
                    let v = *intensity;
                    self.set_pixel(self.x + col, self.y + row, v, v, v);
                }
            }
            self.x += CHAR_WIDTH;
        }
    }
}

impl fmt::Write for FbWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

pub static WRITER: Mutex<Option<FbWriter>> = Mutex::new(None);

pub fn init(buffer: &'static mut FrameBuffer) {
    let mut writer = FbWriter::new(buffer);
    writer.clear(0, 20, 0); // dunkelgrün = "Kernel laeuft"
    *WRITER.lock() = Some(writer);
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        if let Some(writer) = WRITER.lock().as_mut() {
            let _ = writer.write_fmt(args);
        }
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::framebuffer::_print(format_args!("\n")));
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!("{}\n", format_args!($($arg)*))));
}

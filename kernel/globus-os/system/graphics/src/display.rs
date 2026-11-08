//! Safe software framebuffer and desktop-shell renderer.
//!
//! This module operates on a caller-provided framebuffer slice. It does not
//! discover hardware or map physical memory; the platform boot/display layer
//! must validate and provide that memory before constructing a framebuffer.

use crate::{
    shell::DesktopShell,
    wm::{Window, WindowState},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgbx8888,
    Bgrx8888,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayError {
    ZeroDimensions,
    InvalidStride,
    BufferTooSmall,
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_hz: u32,
}

impl DisplayMode {
    pub const fn new(width: u32, height: u32, refresh_hz: u32) -> Option<Self> {
        if width == 0 || height == 0 || refresh_hz == 0 {
            None
        } else {
            Some(Self {
                width,
                height,
                refresh_hz,
            })
        }
    }
}

/// A checked view over a caller-owned 32-bit pixel framebuffer.
pub struct Framebuffer<'a> {
    bytes: &'a mut [u8],
    width: u32,
    height: u32,
    stride: usize,
    format: PixelFormat,
}

impl<'a> Framebuffer<'a> {
    pub fn new(
        bytes: &'a mut [u8],
        width: u32,
        height: u32,
        stride: usize,
        format: PixelFormat,
    ) -> Result<Self, DisplayError> {
        if width == 0 || height == 0 {
            return Err(DisplayError::ZeroDimensions);
        }
        let row_bytes = usize::try_from(width)
            .ok()
            .and_then(|w| w.checked_mul(4))
            .ok_or(DisplayError::ArithmeticOverflow)?;
        if stride < row_bytes {
            return Err(DisplayError::InvalidStride);
        }
        let required = usize::try_from(height - 1)
            .ok()
            .and_then(|h| h.checked_mul(stride))
            .and_then(|offset| offset.checked_add(row_bytes))
            .ok_or(DisplayError::ArithmeticOverflow)?;
        if bytes.len() < required {
            return Err(DisplayError::BufferTooSmall);
        }
        Ok(Self {
            bytes,
            width,
            height,
            stride,
            format,
        })
    }

    pub const fn width(&self) -> u32 {
        self.width
    }
    pub const fn height(&self) -> u32 {
        self.height
    }
    pub const fn stride(&self) -> usize {
        self.stride
    }
    pub const fn pixel_format(&self) -> PixelFormat {
        self.format
    }

    pub fn clear(&mut self, color: u32) {
        self.fill_rect(0, 0, self.width, self.height, color);
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: u32) {
        if width == 0 || height == 0 {
            return;
        }
        let left = i64::from(x).max(0).min(i64::from(self.width));
        let top = i64::from(y).max(0).min(i64::from(self.height));
        let right = i64::from(x)
            .saturating_add(i64::from(width))
            .max(0)
            .min(i64::from(self.width));
        let bottom = i64::from(y)
            .saturating_add(i64::from(height))
            .max(0)
            .min(i64::from(self.height));
        if left >= right || top >= bottom {
            return;
        }
        let (r, g, b) = ((color >> 16) as u8, (color >> 8) as u8, color as u8);
        for py in top as usize..bottom as usize {
            let row = py * self.stride;
            for px in left as usize..right as usize {
                let offset = row + px * 4;
                match self.format {
                    PixelFormat::Rgbx8888 => {
                        self.bytes[offset..offset + 4].copy_from_slice(&[r, g, b, 0xff])
                    }
                    PixelFormat::Bgrx8888 => {
                        self.bytes[offset..offset + 4].copy_from_slice(&[b, g, r, 0xff])
                    }
                }
            }
        }
    }

    /// Draws the desktop background, visible window frames, launcher and taskbar.
    /// Text shaping and app process launch remain platform-shell responsibilities.
    pub fn render_shell(&mut self, windows: &[Window], shell: &DesktopShell) {
        self.clear(0x172235);
        for window in windows {
            if matches!(window.state, WindowState::Closed | WindowState::Minimized) {
                continue;
            }
            let title_height = 28.min(window.bounds.height);
            let frame = if window.focused { 0x4d8fe8 } else { 0x65748b };
            self.fill_rect(
                window.bounds.x,
                window.bounds.y,
                window.bounds.width,
                window.bounds.height,
                0xe5eaf1,
            );
            self.fill_rect(
                window.bounds.x,
                window.bounds.y,
                window.bounds.width,
                title_height,
                frame,
            );
            let button_y = window.bounds.y.saturating_add(6);
            let right = i64::from(window.bounds.x) + i64::from(window.bounds.width);
            if right <= i64::from(i32::MAX) {
                let close_x = (right - 20).max(i64::from(i32::MIN)) as i32;
                self.fill_rect(close_x, button_y, 12, 12, 0xe65c66);
                let minimize_x = (right - 40).max(i64::from(i32::MIN)) as i32;
                self.fill_rect(minimize_x, button_y, 12, 12, 0xf2c15d);
            }
        }
        let taskbar_height = 48.min(self.height);
        let taskbar_y = self.height.saturating_sub(taskbar_height) as i32;
        self.fill_rect(0, taskbar_y, self.width, taskbar_height, 0x101722);
        self.fill_rect(12, taskbar_y.saturating_add(9), 30, 30, 0x4d8fe8);
        let mut x = 54i32;
        for task in shell.tasks() {
            if x.saturating_add(112) > self.width.min(i32::MAX as u32) as i32 {
                break;
            }
            let color = if task.active {
                0x4d8fe8
            } else if task.minimized {
                0x394557
            } else {
                0x28364a
            };
            self.fill_rect(x, taskbar_y.saturating_add(8), 104, 32, color);
            x = x.saturating_add(112);
        }
        // A visible launcher panel marker; app names require a text renderer.
        if !shell.apps().is_empty() {
            self.fill_rect(12, taskbar_y.saturating_sub(180), 220, 168, 0x202d40);
            let mut y = taskbar_y.saturating_sub(164);
            for app in shell.apps().iter().take(6) {
                let marker = if app.pinned { 0x6ea8fe } else { 0x93a4ba };
                self.fill_rect(24, y, 184, 12, marker);
                y = y.saturating_add(24);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_stride_and_short_buffers() {
        let mut bytes = [0u8; 16];
        assert_eq!(
            Framebuffer::new(&mut bytes, 2, 2, 4, PixelFormat::Rgbx8888).err(),
            Some(DisplayError::InvalidStride)
        );
        assert_eq!(
            Framebuffer::new(&mut bytes, 3, 2, 12, PixelFormat::Rgbx8888).err(),
            Some(DisplayError::BufferTooSmall)
        );
    }

    #[test]
    fn clips_draws_and_obeys_pixel_format() {
        let mut bytes = [0u8; 32];
        let mut fb = Framebuffer::new(&mut bytes, 2, 2, 16, PixelFormat::Rgbx8888).unwrap();
        fb.fill_rect(-1, 0, 2, 1, 0x123456);
        assert_eq!(&fb.bytes[0..4], &[0x12, 0x34, 0x56, 0xff]);
        assert_eq!(&fb.bytes[4..8], &[0, 0, 0, 0]);
    }
}

//! Display, compositor, desktop shell and GPU policy boundary.

pub mod compositor;
pub mod desktop;
pub mod display;
pub mod gpu;
pub mod input;
pub mod media_surface;
pub mod shell;
pub mod wm;

pub use media_surface::{MediaSurface, MediaSurfaceError, MediaSurfaceRegistry};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsBackend {
    Framebuffer,
    Vulkan,
    WebGpu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceId(pub u64);

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_invalid_display_mode() {
        assert!(DisplayMode::new(0, 1080, 60).is_none());
    }
}

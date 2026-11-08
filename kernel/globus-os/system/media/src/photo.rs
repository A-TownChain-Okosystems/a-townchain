//! Photo decoding, metadata and viewer state. Codec implementations plug in through `PhotoDecoder`.

use crate::{MediaError, PhotoTransform, PixelFormat};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    Srgb,
    DisplayP3,
    Rec2020,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhotoCodec {
    Jpeg,
    Png,
    Webp,
    Avif,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExifOrientation {
    Normal,
    FlipHorizontal,
    Rotate180,
    FlipVertical,
    Transpose,
    Rotate90,
    Transverse,
    Rotate270,
}

impl ExifOrientation {
    pub const fn transform(self) -> PhotoTransform {
        match self {
            Self::Normal => PhotoTransform::None,
            Self::Rotate90 => PhotoTransform::Rotate90,
            Self::Rotate180 => PhotoTransform::Rotate180,
            Self::Rotate270 => PhotoTransform::Rotate270,
            Self::FlipHorizontal | Self::FlipVertical | Self::Transpose | Self::Transverse => {
                PhotoTransform::None
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhotoInfo {
    pub width: u32,
    pub height: u32,
    pub color_space: ColorSpace,
    pub transform: PhotoTransform,
    pub codec: PhotoCodec,
    pub orientation: ExifOrientation,
}

impl PhotoInfo {
    pub fn validate(self) -> Result<(), MediaError> {
        if self.width == 0 || self.height == 0 {
            Err(MediaError::InvalidPhotoSpec)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhotoFrame {
    pub info: PhotoInfo,
    pub format: PixelFormat,
    pub data: Vec<u8>,
}

pub trait PhotoDecoder {
    fn info(&self) -> PhotoInfo;
    fn decode(&mut self) -> Result<PhotoFrame, MediaError>;
}

pub trait ProgressivePhotoDecoder: PhotoDecoder {
    fn decode_next(&mut self) -> Result<Option<PhotoFrame>, MediaError>;
    fn is_complete(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhotoViewport {
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub rotation: u16,
}

impl Default for PhotoViewport {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            rotation: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct PhotoViewer {
    frame: Option<PhotoFrame>,
    viewport: PhotoViewport,
}

impl PhotoViewer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn frame(&self) -> Option<&PhotoFrame> {
        self.frame.as_ref()
    }
    pub fn viewport(&self) -> PhotoViewport {
        self.viewport
    }
    pub fn load<D: PhotoDecoder>(&mut self, decoder: &mut D) -> Result<(), MediaError> {
        let frame = decoder.decode()?;
        frame.info.validate()?;
        self.frame = Some(frame);
        self.reset();
        Ok(())
    }
    pub fn reset(&mut self) {
        self.viewport = PhotoViewport::default();
    }
    pub fn set_zoom(&mut self, zoom: f32) -> Result<(), MediaError> {
        if !zoom.is_finite() || !(0.1..=16.0).contains(&zoom) {
            return Err(MediaError::InvalidPosition);
        }
        self.viewport.zoom = zoom;
        Ok(())
    }
    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.viewport.pan_x += dx;
        self.viewport.pan_y += dy;
    }
    pub fn rotate_quarter_turn(&mut self, clockwise: bool) {
        self.viewport.rotation = if clockwise {
            (self.viewport.rotation + 90) % 360
        } else {
            (self.viewport.rotation + 270) % 360
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct D;
    impl PhotoDecoder for D {
        fn info(&self) -> PhotoInfo {
            PhotoInfo {
                width: 2,
                height: 2,
                color_space: ColorSpace::Srgb,
                transform: PhotoTransform::None,
                codec: PhotoCodec::Png,
                orientation: ExifOrientation::Normal,
            }
        }
        fn decode(&mut self) -> Result<PhotoFrame, MediaError> {
            Ok(PhotoFrame {
                info: self.info(),
                format: PixelFormat::Rgba8,
                data: vec![0; 16],
            })
        }
    }
    #[test]
    fn viewer_controls_are_bounded() {
        let mut v = PhotoViewer::new();
        let mut d = D;
        v.load(&mut d).unwrap();
        assert!(v.set_zoom(16.1).is_err());
        v.rotate_quarter_turn(true);
        assert_eq!(v.viewport().rotation, 90);
    }
}

//! Unified media model for audio, video, and photos.

pub mod av_sync;
pub mod codec;
pub mod demux;
pub mod desktop;
pub mod gpu;
pub mod library;
pub mod photo;
pub mod playback;
pub mod queue;
pub mod scanner;
pub mod surface;
pub mod thumbnail;
pub mod thumbnail_cache;
pub mod thumbnail_store;
pub mod video;

pub use av_sync::{AvSyncPolicy, SyncAction, SyncDecision};
pub use codec::{
    DecoderCapabilities, DecoderKind, PhotoCodecDecoder, TimedVideoFrame, VideoCodec,
    VideoCodecDecoder, VideoScaling,
};
pub use demux::{ContainerFormat, DemuxPacket, Demuxer, PacketDemuxer, Timestamp, TrackKind};
pub use desktop::{
    AudioControl, MediaInput, MediaWindow, MediaWindowMode, PlaybackState, WindowLifecycle,
};
pub use gpu::{
    BufferFormat, ColorConversion, DisplayMode, DisplayOutput, DmaBufDescriptor, FramePacer,
    FrameTiming, HardwareOverlay, MediaBuffer, MediaGpuBackend, SharedBufferHandle,
};
pub use library::{MediaCatalog, MediaMetadata, MimeType, SortOrder, mime_from_extension};
pub use photo::{
    ColorSpace, ExifOrientation, PhotoCodec, PhotoDecoder, PhotoFrame, PhotoInfo, PhotoViewer,
    PhotoViewport, ProgressivePhotoDecoder,
};
pub use playback::{CodecVideoSession, PlaybackStats};
pub use queue::FrameQueue;
pub use scanner::{
    FileRecord, MediaFileSource, MediaScanner, MediaWatchSource, ScanChange, ScanEvent,
};
pub use surface::{MediaSurface, PresentationState, SurfaceKind};
pub use thumbnail::{Thumbnail, ThumbnailCache};
pub use thumbnail_cache::{ThumbnailCache as BoundedThumbnailCache, ThumbnailKey, ThumbnailSize};
pub use thumbnail_store::{
    MemoryThumbnailStore, StoredThumbnail, ThumbnailDimensions, ThumbnailStore,
};
pub use video::{ClockMaster, SyncClock, VideoDecoder, VideoPacket, VideoPipeline};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    Audio,
    Video,
    Photo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFormat {
    Mp3,
    Flac,
    Opus,
    Aac,
    Mp4,
    Webm,
    Mkv,
    Mov,
    Jpeg,
    Png,
    Webp,
    Avif,
    Unknown,
}

impl MediaFormat {
    pub fn from_extension(extension: &str) -> Self {
        match extension
            .rsplit('.')
            .next()
            .unwrap_or(extension)
            .trim_start_matches('.')
            .to_ascii_lowercase()
            .as_str()
        {
            "mp3" => Self::Mp3,
            "flac" => Self::Flac,
            "opus" => Self::Opus,
            "aac" => Self::Aac,
            "mp4" => Self::Mp4,
            "webm" => Self::Webm,
            "mkv" => Self::Mkv,
            "mov" => Self::Mov,
            "jpg" | "jpeg" => Self::Jpeg,
            "png" => Self::Png,
            "webp" => Self::Webp,
            "avif" => Self::Avif,
            _ => Self::Unknown,
        }
    }
    pub fn kind(self) -> Option<MediaKind> {
        match self {
            Self::Mp3 | Self::Flac | Self::Opus | Self::Aac => Some(MediaKind::Audio),
            Self::Mp4 | Self::Webm | Self::Mkv | Self::Mov => Some(MediaKind::Video),
            Self::Jpeg | Self::Png | Self::Webp | Self::Avif => Some(MediaKind::Photo),
            Self::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaItem {
    pub id: u64,
    pub uri: String,
    pub title: String,
    pub format: MediaFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoFrameSpec {
    pub width: u32,
    pub height: u32,
    pub fps_num: u32,
    pub fps_den: u32,
}
impl VideoFrameSpec {
    pub fn validate(self) -> Result<(), MediaError> {
        if self.width == 0 || self.height == 0 || self.fps_num == 0 || self.fps_den == 0 {
            Err(MediaError::InvalidVideoSpec)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgba8,
    Bgra8,
    Yuv420p,
    Nv12,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoFrame {
    pub spec: VideoFrameSpec,
    pub format: PixelFormat,
    pub data: Vec<u8>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhotoSpec {
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
}
impl PhotoSpec {
    pub fn validate(self) -> Result<(), MediaError> {
        if self.width == 0 || self.height == 0 {
            Err(MediaError::InvalidPhotoSpec)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhotoTransform {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontal,
    FlipVertical,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaError {
    InvalidVideoSpec,
    InvalidPhotoSpec,
    InvalidPosition,
    UnsupportedFormat,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaState {
    Stopped,
    Playing,
    Paused,
    Seeking,
    Ended,
    Error,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timeline {
    position_ms: u64,
    duration_ms: Option<u64>,
}
impl Timeline {
    pub fn new(d: Option<u64>) -> Self {
        Self {
            position_ms: 0,
            duration_ms: d,
        }
    }
    pub fn position_ms(&self) -> u64 {
        self.position_ms
    }
    pub fn duration_ms(&self) -> Option<u64> {
        self.duration_ms
    }
    pub fn seek(&mut self, p: u64) -> Result<(), MediaError> {
        if let Some(d) = self.duration_ms
            && p > d
        {
            return Err(MediaError::InvalidPosition);
        }
        self.position_ms = p;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct MediaLibrary {
    items: Vec<MediaItem>,
}
impl MediaLibrary {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn items(&self) -> &[MediaItem] {
        &self.items
    }
    pub fn add(&mut self, item: MediaItem) {
        self.items.retain(|x| x.id != item.id);
        self.items.push(item);
        self.items.sort_by_key(|x| x.id)
    }
    pub fn remove(&mut self, id: u64) -> bool {
        let n = self.items.len();
        self.items.retain(|x| x.id != id);
        n != self.items.len()
    }
    pub fn find(&self, id: u64) -> Option<&MediaItem> {
        self.items.iter().find(|x| x.id == id)
    }
}

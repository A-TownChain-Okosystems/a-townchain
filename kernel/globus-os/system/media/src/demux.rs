//! Container demuxing contracts and deterministic timestamp handling.

use crate::MediaError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContainerFormat {
    Mp4,
    Mkv,
    Webm,
    #[default]
    Unknown,
}

impl ContainerFormat {
    pub fn from_uri(uri: &str) -> Self {
        match uri
            .rsplit('.')
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str()
        {
            "mp4" => Self::Mp4,
            "mkv" => Self::Mkv,
            "webm" => Self::Webm,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackKind {
    Audio,
    Video,
    Subtitle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp {
    pub pts_us: u64,
    pub dts_us: u64,
    pub duration_us: u64,
    pub keyframe: bool,
}

impl Timestamp {
    pub fn new(pts_us: u64, dts_us: u64, duration_us: u64, keyframe: bool) -> Self {
        Self {
            pts_us,
            dts_us,
            duration_us,
            keyframe,
        }
    }
    pub fn presentation_delay_us(self) -> i64 {
        self.pts_us as i64 - self.dts_us as i64
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemuxPacket {
    pub track: TrackKind,
    pub timestamp: Timestamp,
    pub data: Vec<u8>,
}

pub trait Demuxer {
    fn format(&self) -> ContainerFormat;
    fn next_packet(&mut self) -> Result<Option<DemuxPacket>, MediaError>;
    fn seek_us(&mut self, position_us: u64) -> Result<(), MediaError>;
}

#[derive(Debug, Default)]
pub struct PacketDemuxer {
    format: ContainerFormat,
    packets: Vec<DemuxPacket>,
    cursor: usize,
}

impl PacketDemuxer {
    pub fn new(format: ContainerFormat, packets: Vec<DemuxPacket>) -> Self {
        Self {
            format,
            packets,
            cursor: 0,
        }
    }
    pub fn remaining(&self) -> usize {
        self.packets.len().saturating_sub(self.cursor)
    }
}

impl Demuxer for PacketDemuxer {
    fn format(&self) -> ContainerFormat {
        self.format
    }
    fn next_packet(&mut self) -> Result<Option<DemuxPacket>, MediaError> {
        let packet = self.packets.get(self.cursor).cloned();
        if packet.is_some() {
            self.cursor += 1;
        }
        Ok(packet)
    }
    fn seek_us(&mut self, position_us: u64) -> Result<(), MediaError> {
        self.cursor = self
            .packets
            .iter()
            .position(|p| p.timestamp.pts_us >= position_us)
            .unwrap_or(self.packets.len());
        Ok(())
    }
}

pub fn reorder_timestamp(pts_us: u64, dts_us: u64) -> Timestamp {
    Timestamp {
        pts_us,
        dts_us,
        duration_us: 0,
        keyframe: false,
    }
}

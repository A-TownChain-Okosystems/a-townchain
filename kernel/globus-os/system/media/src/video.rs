//! Codec-independent video playback pipeline.

use crate::{MediaError, MediaState, Timeline, VideoFrame, VideoFrameSpec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoPacket {
    pub pts_us: u64,
    pub duration_us: u64,
    pub keyframe: bool,
    pub data_len: usize,
}

pub trait VideoDecoder {
    fn spec(&self) -> VideoFrameSpec;
    fn decode_next(&mut self) -> Result<Option<VideoFrame>, MediaError>;
    fn seek_us(&mut self, position_us: u64) -> Result<(), MediaError>;
    fn finished(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockMaster {
    Audio,
    Video,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyncClock {
    pub position_us: u64,
    pub master: ClockMaster,
}

impl SyncClock {
    pub const fn new(master: ClockMaster) -> Self {
        Self {
            position_us: 0,
            master,
        }
    }
    pub fn advance(&mut self, delta_us: u64) {
        self.position_us = self.position_us.saturating_add(delta_us);
    }
    pub fn delta_us(self, pts_us: u64) -> i64 {
        pts_us as i64 - self.position_us as i64
    }
}

#[derive(Debug)]
pub struct VideoPipeline<D> {
    decoder: D,
    state: MediaState,
    timeline: Timeline,
    clock: SyncClock,
    last_frame: Option<VideoFrame>,
}

impl<D: VideoDecoder> VideoPipeline<D> {
    pub fn new(decoder: D, duration_ms: Option<u64>) -> Result<Self, MediaError> {
        decoder.spec().validate()?;
        Ok(Self {
            decoder,
            state: MediaState::Stopped,
            timeline: Timeline::new(duration_ms),
            clock: SyncClock::new(ClockMaster::Video),
            last_frame: None,
        })
    }
    pub fn state(&self) -> MediaState {
        self.state
    }
    pub fn clock(&self) -> SyncClock {
        self.clock
    }
    pub fn current_frame(&self) -> Option<&VideoFrame> {
        self.last_frame.as_ref()
    }
    pub fn play(&mut self) {
        self.state = MediaState::Playing;
    }
    pub fn pause(&mut self) {
        if self.state == MediaState::Playing {
            self.state = MediaState::Paused;
        }
    }
    pub fn stop(&mut self) {
        self.state = MediaState::Stopped;
        self.clock.position_us = 0;
        let _ = self.timeline.seek(0);
    }
    pub fn seek_us(&mut self, position_us: u64) -> Result<(), MediaError> {
        self.state = MediaState::Seeking;
        self.decoder.seek_us(position_us)?;
        self.clock.position_us = position_us;
        self.timeline.seek(position_us / 1_000)?;
        self.last_frame = None;
        self.state = MediaState::Paused;
        Ok(())
    }
    pub fn pump(&mut self) -> Result<Option<&VideoFrame>, MediaError> {
        if self.state != MediaState::Playing {
            return Ok(self.last_frame.as_ref());
        }
        match self.decoder.decode_next()? {
            Some(frame) => {
                self.last_frame = Some(frame);
                Ok(self.last_frame.as_ref())
            }
            None => {
                self.state = MediaState::Ended;
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PixelFormat;
    struct Decoder {
        spec: VideoFrameSpec,
        done: bool,
    }
    impl VideoDecoder for Decoder {
        fn spec(&self) -> VideoFrameSpec {
            self.spec
        }
        fn decode_next(&mut self) -> Result<Option<VideoFrame>, MediaError> {
            if self.done {
                return Ok(None);
            }
            self.done = true;
            Ok(Some(VideoFrame {
                spec: self.spec,
                format: PixelFormat::Rgba8,
                data: vec![0; 4],
            }))
        }
        fn seek_us(&mut self, _: u64) -> Result<(), MediaError> {
            self.done = false;
            Ok(())
        }
        fn finished(&self) -> bool {
            self.done
        }
    }
    #[test]
    fn pipeline_reaches_end() {
        let spec = VideoFrameSpec {
            width: 1,
            height: 1,
            fps_num: 30,
            fps_den: 1,
        };
        let mut p = VideoPipeline::new(Decoder { spec, done: false }, None).unwrap();
        p.play();
        assert!(p.pump().unwrap().is_some());
        assert!(p.pump().unwrap().is_none());
        assert_eq!(p.state(), MediaState::Ended);
    }
}

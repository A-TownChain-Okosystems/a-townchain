//! Audio output device abstraction and deterministic ring-buffer backend.

use crate::codec::{AudioSpec, PcmFrame};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputError {
    NotOpen,
    AlreadyOpen,
    SpecMismatch,
    BufferFull,
    InvalidCapacity,
}

pub trait AudioOutput {
    fn open(&mut self, spec: AudioSpec) -> Result<(), OutputError>;
    fn start(&mut self) -> Result<(), OutputError>;
    fn stop(&mut self) -> Result<(), OutputError>;
    fn write(&mut self, frame: &PcmFrame) -> Result<usize, OutputError>;
    fn queued_frames(&self) -> usize;
    fn latency_frames(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct RingAudioOutput {
    capacity_frames: usize,
    spec: Option<AudioSpec>,
    running: bool,
    queued: usize,
}

impl RingAudioOutput {
    pub fn new(capacity_frames: usize) -> Result<Self, OutputError> {
        if capacity_frames == 0 {
            return Err(OutputError::InvalidCapacity);
        }
        Ok(Self {
            capacity_frames,
            spec: None,
            running: false,
            queued: 0,
        })
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl AudioOutput for RingAudioOutput {
    fn open(&mut self, spec: AudioSpec) -> Result<(), OutputError> {
        spec.validate().map_err(|_| OutputError::SpecMismatch)?;
        if self.spec.is_some() {
            return Err(OutputError::AlreadyOpen);
        }
        self.spec = Some(spec);
        self.queued = 0;
        Ok(())
    }

    fn start(&mut self) -> Result<(), OutputError> {
        if self.spec.is_none() {
            return Err(OutputError::NotOpen);
        }
        self.running = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), OutputError> {
        if self.spec.is_none() {
            return Err(OutputError::NotOpen);
        }
        self.running = false;
        Ok(())
    }

    fn write(&mut self, frame: &PcmFrame) -> Result<usize, OutputError> {
        let spec = self.spec.ok_or(OutputError::NotOpen)?;
        if frame.spec != spec {
            return Err(OutputError::SpecMismatch);
        }
        let available = self.capacity_frames.saturating_sub(self.queued);
        if frame.frames() > available {
            return Err(OutputError::BufferFull);
        }
        self.queued += frame.frames();
        Ok(frame.frames())
    }

    fn queued_frames(&self) -> usize {
        self.queued
    }
    fn latency_frames(&self) -> usize {
        self.queued
    }
}

impl Default for RingAudioOutput {
    fn default() -> Self {
        Self::new(48_000).expect("non-zero default capacity")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::AudioFormat;

    #[test]
    fn ring_output_accepts_matching_pcm() {
        let spec = AudioSpec {
            sample_rate: 48_000,
            channels: 2,
            format: AudioFormat::PcmF32Le,
        };
        let frame = PcmFrame::new(spec, vec![0.0; 8]).unwrap();
        let mut output = RingAudioOutput::new(8).unwrap();
        output.open(spec).unwrap();
        output.start().unwrap();
        assert_eq!(output.write(&frame).unwrap(), 4);
        assert_eq!(output.queued_frames(), 4);
    }
}

//! Deterministic audio format and decoder boundary.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    PcmS16Le,
    PcmF32Le,
    Mp3,
    Flac,
    Opus,
    Aac,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioSpec {
    pub sample_rate: u32,
    pub channels: u16,
    pub format: AudioFormat,
}

impl AudioSpec {
    pub fn validate(self) -> Result<(), CodecError> {
        if self.sample_rate == 0 || self.sample_rate > 384_000 {
            return Err(CodecError::InvalidSampleRate);
        }
        if self.channels == 0 || self.channels > 32 {
            return Err(CodecError::InvalidChannels);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PcmFrame {
    pub spec: AudioSpec,
    pub samples: Vec<f32>,
}

impl PcmFrame {
    pub fn new(spec: AudioSpec, samples: Vec<f32>) -> Result<Self, CodecError> {
        spec.validate()?;
        if !samples.len().is_multiple_of(spec.channels as usize) {
            return Err(CodecError::InvalidSampleCount);
        }
        Ok(Self { spec, samples })
    }

    pub fn frames(&self) -> usize {
        self.samples.len() / self.spec.channels as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecError {
    InvalidSampleRate,
    InvalidChannels,
    InvalidSampleCount,
    InvalidPosition,
    UnsupportedFormat,
}

pub trait AudioDecoder {
    fn spec(&self) -> AudioSpec;
    fn decode(&mut self, max_frames: usize) -> Result<PcmFrame, CodecError>;
    fn seek_ms(&mut self, position_ms: u64) -> Result<(), CodecError>;
    fn finished(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_pcm_frame() {
        let spec = AudioSpec {
            sample_rate: 48_000,
            channels: 2,
            format: AudioFormat::PcmF32Le,
        };
        let frame = PcmFrame::new(spec, vec![0.0; 8]).expect("valid frame");
        assert_eq!(frame.frames(), 4);
    }

    #[test]
    fn rejects_partial_channel_frame() {
        let spec = AudioSpec {
            sample_rate: 48_000,
            channels: 2,
            format: AudioFormat::PcmF32Le,
        };
        assert_eq!(
            PcmFrame::new(spec, vec![0.0; 3]),
            Err(CodecError::InvalidSampleCount)
        );
    }
}

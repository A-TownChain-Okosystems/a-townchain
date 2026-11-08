//! Deterministic PCM mixer with bounded gain and clipping.

use crate::codec::{AudioSpec, CodecError, PcmFrame};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixerError {
    SpecMismatch,
    InvalidGain,
    Codec(CodecError),
}

impl From<CodecError> for MixerError {
    fn from(value: CodecError) -> Self {
        Self::Codec(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PcmMixer {
    gain: f32,
    muted: bool,
}

impl Default for PcmMixer {
    fn default() -> Self {
        Self {
            gain: 1.0,
            muted: false,
        }
    }
}

impl PcmMixer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_gain(&mut self, gain: f32) -> Result<(), MixerError> {
        if !gain.is_finite() || !(0.0..=4.0).contains(&gain) {
            return Err(MixerError::InvalidGain);
        }
        self.gain = gain;
        Ok(())
    }

    pub fn gain(&self) -> f32 {
        self.gain
    }
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }
    pub fn muted(&self) -> bool {
        self.muted
    }

    pub fn mix(&self, spec: AudioSpec, inputs: &[PcmFrame]) -> Result<PcmFrame, MixerError> {
        spec.validate()?;
        if inputs.iter().any(|frame| frame.spec != spec) {
            return Err(MixerError::SpecMismatch);
        }
        let sample_count = inputs.iter().map(|f| f.samples.len()).max().unwrap_or(0);
        let mut output = vec![0.0; sample_count];
        if !self.muted {
            for frame in inputs {
                for (dst, src) in output.iter_mut().zip(&frame.samples) {
                    *dst += *src * self.gain;
                }
            }
            for sample in &mut output {
                *sample = sample.clamp(-1.0, 1.0);
            }
        }
        Ok(PcmFrame::new(spec, output)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::AudioFormat;

    fn spec() -> AudioSpec {
        AudioSpec {
            sample_rate: 48_000,
            channels: 2,
            format: AudioFormat::PcmF32Le,
        }
    }

    #[test]
    fn mixes_and_clips() {
        let frame = PcmFrame::new(spec(), vec![0.8, -0.8]).unwrap();
        let mixed = PcmMixer::new()
            .mix(spec(), &[frame.clone(), frame])
            .unwrap();
        assert_eq!(mixed.samples, vec![1.0, -1.0]);
    }

    #[test]
    fn mute_outputs_silence() {
        let mut mixer = PcmMixer::new();
        mixer.set_muted(true);
        let frame = PcmFrame::new(spec(), vec![0.5, -0.5]).unwrap();
        assert_eq!(mixer.mix(spec(), &[frame]).unwrap().samples, vec![0.0, 0.0]);
    }
}

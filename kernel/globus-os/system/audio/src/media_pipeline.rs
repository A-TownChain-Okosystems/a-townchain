//! Playback pipeline coordinating the player, decoder, mixer, and output.

use crate::{
    AudioDecoder, AudioOutput, CodecError, MediaPlayer, MixerError, OutputError, PcmMixer,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineError {
    NoTrack,
    Decoder(CodecError),
    Mixer(MixerError),
    Output(OutputError),
}

impl From<CodecError> for PipelineError {
    fn from(e: CodecError) -> Self {
        Self::Decoder(e)
    }
}
impl From<MixerError> for PipelineError {
    fn from(e: MixerError) -> Self {
        Self::Mixer(e)
    }
}
impl From<OutputError> for PipelineError {
    fn from(e: OutputError) -> Self {
        Self::Output(e)
    }
}

pub struct MediaPipeline<D, O> {
    pub player: MediaPlayer,
    decoder: D,
    mixer: PcmMixer,
    output: O,
}

impl<D: AudioDecoder, O: AudioOutput> MediaPipeline<D, O> {
    pub fn new(player: MediaPlayer, decoder: D, output: O) -> Self {
        Self {
            player,
            decoder,
            mixer: PcmMixer::new(),
            output,
        }
    }

    pub fn mixer(&self) -> &PcmMixer {
        &self.mixer
    }
    pub fn mixer_mut(&mut self) -> &mut PcmMixer {
        &mut self.mixer
    }
    pub fn output(&self) -> &O {
        &self.output
    }
    pub fn output_mut(&mut self) -> &mut O {
        &mut self.output
    }

    pub fn open(&mut self) -> Result<(), PipelineError> {
        self.output.open(self.decoder.spec())?;
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), PipelineError> {
        self.output.start()?;
        self.player.play().map_err(|_| PipelineError::NoTrack)
    }

    pub fn pump(&mut self, max_frames: usize) -> Result<usize, PipelineError> {
        if self.player.queue().current().is_none() {
            return Err(PipelineError::NoTrack);
        }
        let frame = self.decoder.decode(max_frames)?;
        let frames = frame.frames();
        if frames == 0 {
            return Ok(0);
        }
        let mixed = self.mixer.mix(frame.spec, &[frame])?;
        self.output.write(&mixed)?;
        Ok(frames)
    }

    pub fn stop(&mut self) -> Result<(), PipelineError> {
        self.output.stop()?;
        self.player.stop();
        Ok(())
    }
}

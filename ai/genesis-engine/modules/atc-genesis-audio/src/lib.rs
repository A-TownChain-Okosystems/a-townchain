use atc_genesis_platform::AudioRuntime;

mod mixer;
pub use mixer::{AudioMixer, AudioVoice, AudioVoiceId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialAudio {
    pub position: [f32; 3],
    pub gain: f32,
    pub min_distance: f32,
    pub max_distance: f32,
}
impl SpatialAudio {
    pub fn attenuation(&self, listener: [f32; 3]) -> f32 {
        let d = ((0..3)
            .map(|i| (self.position[i] - listener[i]).powi(2))
            .sum::<f32>())
        .sqrt();
        if d <= self.min_distance {
            self.gain
        } else if d >= self.max_distance {
            0.0
        } else {
            let t =
                (d - self.min_distance) / (self.max_distance - self.min_distance).max(f32::EPSILON);
            self.gain * (1.0 - t)
        }
    }
}

pub trait AudioBackend {
    fn start(&mut self) -> Result<(), String>;
    fn stop(&mut self);
    fn render(&mut self, samples: &mut [f32]);
}
#[derive(Default)]
pub struct NullAudioBackend {
    running: bool,
}
impl AudioBackend for NullAudioBackend {
    fn start(&mut self) -> Result<(), String> {
        self.running = true;
        Ok(())
    }
    fn stop(&mut self) {
        self.running = false;
    }
    fn render(&mut self, samples: &mut [f32]) {
        if self.running {
            samples.fill(0.0);
        }
    }
}

#[derive(Default)]
pub struct RuntimeAudio<B = NullAudioBackend> {
    pub mixer: AudioMixer,
    pub backend: B,
}
impl<B: AudioBackend> RuntimeAudio<B> {
    pub fn start(&mut self) -> Result<(), String> {
        self.backend.start()
    }
    pub fn stop(&mut self) {
        self.backend.stop()
    }
}
impl<B: AudioBackend> AudioRuntime for RuntimeAudio<B> {
    fn update(&mut self, _dt_seconds: f32) {}
    fn set_master_gain(&mut self, gain: f32) {
        self.mixer.set_master_gain(gain)
    }
}

#[derive(Default)]
pub struct NullAudioRuntime {
    master_gain: f32,
}
impl NullAudioRuntime {
    pub fn master_gain(&self) -> f32 {
        self.master_gain
    }
}
impl AudioRuntime for NullAudioRuntime {
    fn update(&mut self, _dt_seconds: f32) {}
    fn set_master_gain(&mut self, gain: f32) {
        self.master_gain = gain.clamp(0.0, 4.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spatial_fades() {
        let s = SpatialAudio {
            position: [10.0, 0.0, 0.0],
            gain: 1.0,
            min_distance: 1.0,
            max_distance: 11.0,
        };
        assert!((s.attenuation([0.0, 0.0, 0.0]) - 0.1).abs() < 1e-6);
    }
}

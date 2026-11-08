#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AudioVoiceId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AudioBusId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioVoice {
    pub gain: f32,
    pub pitch: f32,
    pub looping: bool,
    pub playing: bool,
    pub bus: AudioBusId,
    pub position: [f32; 3],
    pub max_distance: f32,
}
impl Default for AudioVoice {
    fn default() -> Self {
        Self {
            gain: 1.0,
            pitch: 1.0,
            looping: false,
            playing: false,
            bus: AudioBusId(0),
            position: [0.0; 3],
            max_distance: 100.0,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioBus {
    pub gain: f32,
    pub muted: bool,
}
impl Default for AudioBus {
    fn default() -> Self {
        Self {
            gain: 1.0,
            muted: false,
        }
    }
}
#[derive(Default)]
pub struct AudioMixer {
    voices: Vec<AudioVoice>,
    buses: Vec<AudioBus>,
    master_gain: f32,
}
impl AudioMixer {
    pub fn new() -> Self {
        Self {
            voices: Vec::new(),
            buses: vec![AudioBus::default()],
            master_gain: 1.0,
        }
    }
    pub fn play(&mut self, voice: AudioVoice) -> AudioVoiceId {
        let id = AudioVoiceId(self.voices.len() as u32);
        self.voices.push(AudioVoice {
            playing: true,
            ..voice
        });
        id
    }
    pub fn stop(&mut self, id: AudioVoiceId) {
        if let Some(v) = self.voices.get_mut(id.0 as usize) {
            v.playing = false;
        }
    }
    pub fn voice(&self, id: AudioVoiceId) -> Option<&AudioVoice> {
        self.voices.get(id.0 as usize)
    }
    pub fn create_bus(&mut self, bus: AudioBus) -> AudioBusId {
        let id = AudioBusId(self.buses.len() as u32);
        self.buses.push(bus);
        id
    }
    pub fn set_bus_gain(&mut self, id: AudioBusId, gain: f32) {
        if let Some(b) = self.buses.get_mut(id.0 as usize) {
            b.gain = gain.clamp(0.0, 4.0);
        }
    }
    pub fn set_bus_muted(&mut self, id: AudioBusId, muted: bool) {
        if let Some(b) = self.buses.get_mut(id.0 as usize) {
            b.muted = muted;
        }
    }
    pub fn set_master_gain(&mut self, gain: f32) {
        self.master_gain = gain.clamp(0.0, 4.0);
    }
    pub fn master_gain(&self) -> f32 {
        self.master_gain
    }
    pub fn effective_gain(&self, id: AudioVoiceId, listener: [f32; 3]) -> Option<f32> {
        let v = self.voice(id)?;
        if !v.playing {
            return Some(0.0);
        }
        let bus = self.buses.get(v.bus.0 as usize)?;
        if bus.muted {
            return Some(0.0);
        }
        let dx = v.position[0] - listener[0];
        let dy = v.position[1] - listener[1];
        let dz = v.position[2] - listener[2];
        let d = (dx * dx + dy * dy + dz * dz).sqrt();
        let attenuation = if v.max_distance <= 0.0 {
            1.0
        } else {
            (1.0 - d / v.max_distance).clamp(0.0, 1.0)
        };
        Some(v.gain.max(0.0) * bus.gain * self.master_gain * attenuation)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spatial_gain_falls_off() {
        let mut m = AudioMixer::new();
        let id = m.play(AudioVoice {
            position: [10.0, 0.0, 0.0],
            max_distance: 20.0,
            ..Default::default()
        });
        assert!((m.effective_gain(id, [0.0; 3]).unwrap() - 0.5).abs() < f32::EPSILON);
    }
}

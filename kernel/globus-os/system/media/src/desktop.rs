//! Desktop media-window state independent of the compositor implementation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaWindowMode {
    Windowed,
    Fullscreen,
    PictureInPicture,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Seeking,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowLifecycle {
    Created,
    Attached,
    Suspended,
    Detached,
    Closed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaInput {
    PlayPause,
    Stop,
    SeekForward,
    SeekBackward,
    ToggleFullscreen,
    TogglePictureInPicture,
    VolumeUp,
    VolumeDown,
    Mute,
}

pub trait AudioControl {
    fn set_volume(&mut self, percent: u8);
    fn set_muted(&mut self, muted: bool);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaWindow {
    pub id: u64,
    pub surface_id: u64,
    pub mode: MediaWindowMode,
    pub playback: PlaybackState,
    pub lifecycle: WindowLifecycle,
    pub position_us: u64,
    pub duration_us: Option<u64>,
    pub seek_step_us: u64,
    pub volume_percent: u8,
    pub muted: bool,
}

impl MediaWindow {
    pub fn new(id: u64, surface_id: u64) -> Self {
        Self {
            id,
            surface_id,
            mode: MediaWindowMode::Windowed,
            playback: PlaybackState::Stopped,
            lifecycle: WindowLifecycle::Created,
            position_us: 0,
            duration_us: None,
            seek_step_us: 10_000_000,
            volume_percent: 100,
            muted: false,
        }
    }
    pub fn attach(&mut self) {
        if self.lifecycle != WindowLifecycle::Closed {
            self.lifecycle = WindowLifecycle::Attached;
        }
    }
    pub fn suspend(&mut self) {
        if self.lifecycle == WindowLifecycle::Attached {
            self.lifecycle = WindowLifecycle::Suspended;
        }
    }
    pub fn resume(&mut self) {
        if self.lifecycle == WindowLifecycle::Suspended {
            self.lifecycle = WindowLifecycle::Attached;
        }
    }
    pub fn detach(&mut self) {
        if self.lifecycle != WindowLifecycle::Closed {
            self.lifecycle = WindowLifecycle::Detached;
        }
    }
    pub fn close(&mut self) {
        self.lifecycle = WindowLifecycle::Closed;
        self.playback = PlaybackState::Stopped;
    }
    pub fn set_duration(&mut self, duration_us: Option<u64>) {
        self.duration_us = duration_us;
        self.position_us = self.position_us.min(duration_us.unwrap_or(u64::MAX));
    }
    pub fn set_position(&mut self, position_us: u64) {
        self.position_us = position_us.min(self.duration_us.unwrap_or(u64::MAX));
    }
    pub fn play(&mut self) {
        if self.lifecycle != WindowLifecycle::Closed {
            self.playback = PlaybackState::Playing;
        }
    }
    pub fn pause(&mut self) {
        if self.playback == PlaybackState::Playing {
            self.playback = PlaybackState::Paused;
        }
    }
    pub fn stop(&mut self) {
        self.playback = PlaybackState::Stopped;
        self.position_us = 0;
    }
    pub fn seek_by(&mut self, delta_us: i64) {
        self.playback = PlaybackState::Seeking;
        if delta_us.is_negative() {
            self.position_us = self.position_us.saturating_sub(delta_us.unsigned_abs());
        } else {
            self.position_us = self
                .position_us
                .saturating_add(delta_us as u64)
                .min(self.duration_us.unwrap_or(u64::MAX));
        }
        self.playback = PlaybackState::Paused;
    }
    pub fn apply(&mut self, input: MediaInput) {
        match input {
            MediaInput::PlayPause => {
                if self.playback == PlaybackState::Playing {
                    self.pause()
                } else {
                    self.play()
                }
            }
            MediaInput::Stop => self.stop(),
            MediaInput::SeekForward => self.seek_by(self.seek_step_us as i64),
            MediaInput::SeekBackward => self.seek_by(-(self.seek_step_us as i64)),
            MediaInput::ToggleFullscreen => {
                self.mode = if self.mode == MediaWindowMode::Fullscreen {
                    MediaWindowMode::Windowed
                } else {
                    MediaWindowMode::Fullscreen
                }
            }
            MediaInput::TogglePictureInPicture => {
                self.mode = if self.mode == MediaWindowMode::PictureInPicture {
                    MediaWindowMode::Windowed
                } else {
                    MediaWindowMode::PictureInPicture
                }
            }
            MediaInput::VolumeUp => {
                self.volume_percent = self.volume_percent.saturating_add(5).min(100)
            }
            MediaInput::VolumeDown => self.volume_percent = self.volume_percent.saturating_sub(5),
            MediaInput::Mute => self.muted = !self.muted,
        }
    }
    pub fn sync_audio<A: AudioControl>(&self, audio: &mut A) {
        audio.set_volume(self.volume_percent);
        audio.set_muted(self.muted);
    }
}

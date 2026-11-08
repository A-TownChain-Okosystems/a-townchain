//! Desktop/compositor bridge for media surfaces.

use crate::{SurfaceId, compositor::Rect};
use globus_media::{
    MediaInput, MediaKind, MediaState, MediaWindow, MediaWindowMode, PresentationState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaSurface {
    pub surface: SurfaceId,
    pub kind: MediaKind,
    pub bounds: Rect,
    pub state: MediaState,
    pub fullscreen: bool,
    pub picture_in_picture: bool,
    pub presentation: PresentationState,
    pub last_pts_us: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaSurfaceError {
    InvalidBounds,
    Missing,
}

impl MediaSurface {
    pub fn new(
        surface: SurfaceId,
        kind: MediaKind,
        bounds: Rect,
    ) -> Result<Self, MediaSurfaceError> {
        if bounds.width == 0 || bounds.height == 0 {
            return Err(MediaSurfaceError::InvalidBounds);
        }
        Ok(Self {
            surface,
            kind,
            bounds,
            state: MediaState::Stopped,
            fullscreen: false,
            picture_in_picture: false,
            presentation: PresentationState::Empty,
            last_pts_us: None,
        })
    }
    pub fn set_state(&mut self, state: MediaState) {
        self.state = state;
    }
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }
    pub fn set_picture_in_picture(&mut self, pip: bool) {
        self.picture_in_picture = pip;
    }
    pub fn resize(&mut self, bounds: Rect) -> Result<(), MediaSurfaceError> {
        if bounds.width == 0 || bounds.height == 0 {
            return Err(MediaSurfaceError::InvalidBounds);
        }
        self.bounds = bounds;
        Ok(())
    }
    pub fn mark_ready(&mut self) {
        self.presentation = PresentationState::Ready;
    }
    pub fn present(&mut self, pts_us: u64) -> bool {
        if self.presentation == PresentationState::Suspended {
            return false;
        }
        self.presentation = PresentationState::Presented;
        self.last_pts_us = Some(pts_us);
        true
    }
    pub fn suspend(&mut self) {
        self.presentation = PresentationState::Suspended;
    }
    pub fn resume(&mut self) {
        self.presentation = PresentationState::Ready;
    }
}

#[derive(Debug)]
pub struct MediaSurfaceBinding {
    pub surface: MediaSurface,
    pub window: MediaWindow,
}

#[derive(Debug, Default)]
pub struct MediaSurfaceRegistry {
    surfaces: Vec<MediaSurfaceBinding>,
}

impl MediaSurfaceRegistry {
    pub fn register(&mut self, surface: MediaSurface) {
        let id = surface.surface;
        self.surfaces.retain(|x| x.surface.surface != id);
        self.surfaces.push(MediaSurfaceBinding {
            surface,
            window: MediaWindow::new(id.0, id.0),
        });
    }
    pub fn remove(&mut self, surface: SurfaceId) {
        self.surfaces.retain(|x| x.surface.surface != surface);
    }
    pub fn get(&self, surface: SurfaceId) -> Option<&MediaSurface> {
        self.surfaces
            .iter()
            .find(|x| x.surface.surface == surface)
            .map(|x| &x.surface)
    }
    pub fn get_mut(&mut self, surface: SurfaceId) -> Option<&mut MediaSurface> {
        self.surfaces
            .iter_mut()
            .find(|x| x.surface.surface == surface)
            .map(|x| &mut x.surface)
    }
    pub fn window_mut(&mut self, surface: SurfaceId) -> Option<&mut MediaWindow> {
        self.surfaces
            .iter_mut()
            .find(|x| x.surface.surface == surface)
            .map(|x| &mut x.window)
    }
    pub fn input(
        &mut self,
        surface: SurfaceId,
        input: MediaInput,
    ) -> Result<(), MediaSurfaceError> {
        let binding = self
            .surfaces
            .iter_mut()
            .find(|x| x.surface.surface == surface)
            .ok_or(MediaSurfaceError::Missing)?;
        binding.window.apply(input);
        binding
            .surface
            .set_fullscreen(binding.window.mode == MediaWindowMode::Fullscreen);
        binding
            .surface
            .set_picture_in_picture(binding.window.mode == MediaWindowMode::PictureInPicture);
        binding.surface.set_state(match binding.window.playback {
            globus_media::PlaybackState::Stopped => MediaState::Stopped,
            globus_media::PlaybackState::Playing => MediaState::Playing,
            globus_media::PlaybackState::Paused => MediaState::Paused,
            globus_media::PlaybackState::Seeking => MediaState::Seeking,
        });
        Ok(())
    }
    pub fn surfaces(&self) -> Vec<MediaSurface> {
        self.surfaces.iter().map(|x| x.surface).collect()
    }
}

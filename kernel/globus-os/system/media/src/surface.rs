//! Graphics-facing media surface contract.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceKind {
    Video,
    Photo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationState {
    Empty,
    Ready,
    Presented,
    Suspended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaSurface {
    pub id: u64,
    pub kind: SurfaceKind,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
    pub fullscreen: bool,
    pub picture_in_picture: bool,
    pub presentation: PresentationState,
    pub frame_sequence: u64,
    pub last_pts_us: Option<u64>,
}

impl MediaSurface {
    pub fn new(id: u64, kind: SurfaceKind, width: u32, height: u32) -> Self {
        Self {
            id,
            kind,
            width,
            height,
            visible: true,
            fullscreen: false,
            picture_in_picture: false,
            presentation: PresentationState::Empty,
            frame_sequence: 0,
            last_pts_us: None,
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
    pub fn set_fullscreen(&mut self, value: bool) {
        self.fullscreen = value;
    }
    pub fn set_picture_in_picture(&mut self, value: bool) {
        self.picture_in_picture = value;
    }
    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
        if !value {
            self.presentation = PresentationState::Suspended;
        }
    }
    pub fn mark_ready(&mut self) {
        if self.visible {
            self.presentation = PresentationState::Ready;
        }
    }
    pub fn present(&mut self, pts_us: u64) -> bool {
        if !self.visible || self.presentation == PresentationState::Suspended {
            return false;
        }
        self.frame_sequence = self.frame_sequence.saturating_add(1);
        self.last_pts_us = Some(pts_us);
        self.presentation = PresentationState::Presented;
        true
    }
    pub fn suspend(&mut self) {
        self.presentation = PresentationState::Suspended;
    }
    pub fn resume(&mut self) {
        self.presentation = PresentationState::Ready;
    }
}

//! Capability-neutral window manager state machine.
//!
//! This module owns logical window lifecycle and geometry only. It does not
//! grant application capabilities or perform privileged device operations.

use crate::SurfaceId;
use crate::compositor::{Rect, SurfacePlacement};

const MIN_WINDOW_WIDTH: u32 = 64;
const MIN_WINDOW_HEIGHT: u32 = 48;
const REACHABLE_TITLEBAR_HEIGHT: i32 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub surface: SurfaceId,
    pub title: String,
    pub bounds: Rect,
    pub state: WindowState,
    pub focused: bool,
    pub resizable: bool,
    restore_bounds: Option<Rect>,
    state_before_minimize: WindowState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowError {
    Exists,
    Missing,
    Closed,
    InvalidSize,
    NotResizable,
}

#[derive(Debug, Default)]
pub struct WindowManager {
    windows: Vec<Window>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(
        &mut self,
        surface: SurfaceId,
        title: impl Into<String>,
        bounds: Rect,
    ) -> Result<(), WindowError> {
        if bounds.width < MIN_WINDOW_WIDTH || bounds.height < MIN_WINDOW_HEIGHT {
            return Err(WindowError::InvalidSize);
        }
        if self
            .windows
            .iter()
            .any(|w| w.surface == surface && w.state != WindowState::Closed)
        {
            return Err(WindowError::Exists);
        }
        self.windows.push(Window {
            surface,
            title: title.into(),
            bounds,
            state: WindowState::Normal,
            focused: false,
            resizable: true,
            restore_bounds: None,
            state_before_minimize: WindowState::Normal,
        });
        self.focus(surface)
    }

    pub fn focus(&mut self, surface: SurfaceId) -> Result<(), WindowError> {
        let index = self
            .windows
            .iter()
            .position(|w| w.surface == surface && w.state != WindowState::Closed)
            .ok_or(WindowError::Missing)?;
        if self.windows[index].state == WindowState::Minimized {
            self.windows[index].state = self.windows[index].state_before_minimize;
        }
        for window in &mut self.windows {
            window.focused = window.surface == surface && window.state != WindowState::Closed;
        }
        // Vector order is the authoritative logical z-order: the focused
        // window is last and therefore receives the greatest placement z.
        let focused = self.windows.remove(index);
        self.windows.push(focused);
        Ok(())
    }

    pub fn move_window(
        &mut self,
        surface: SurfaceId,
        x: i32,
        y: i32,
        viewport: Rect,
    ) -> Result<(), WindowError> {
        let window = self.active_window_mut(surface)?;
        if window.state != WindowState::Normal {
            return Err(WindowError::Closed);
        }
        let max_x = viewport
            .x
            .saturating_add(viewport.width.min(i32::MAX as u32) as i32)
            .saturating_sub(REACHABLE_TITLEBAR_HEIGHT);
        let min_x = viewport
            .x
            .saturating_sub(window.bounds.width.min(i32::MAX as u32) as i32)
            .saturating_add(REACHABLE_TITLEBAR_HEIGHT);
        let min_y = viewport.y;
        let max_y = viewport
            .y
            .saturating_add(viewport.height.min(i32::MAX as u32) as i32)
            .saturating_sub(REACHABLE_TITLEBAR_HEIGHT);
        window.bounds.x = x.clamp(min_x.min(max_x), max_x.max(min_x));
        window.bounds.y = y.clamp(min_y, max_y.max(min_y));
        Ok(())
    }

    pub fn resize_window(
        &mut self,
        surface: SurfaceId,
        width: u32,
        height: u32,
        viewport: Rect,
    ) -> Result<(), WindowError> {
        if width < MIN_WINDOW_WIDTH || height < MIN_WINDOW_HEIGHT {
            return Err(WindowError::InvalidSize);
        }
        let window = self.active_window_mut(surface)?;
        if !window.resizable {
            return Err(WindowError::NotResizable);
        }
        if window.state != WindowState::Normal {
            return Err(WindowError::Closed);
        }
        let max_width = viewport.width.max(MIN_WINDOW_WIDTH);
        let max_height = viewport.height.max(MIN_WINDOW_HEIGHT);
        window.bounds.width = width.min(max_width);
        window.bounds.height = height.min(max_height);
        // Keep the top-left inside the current work area after display changes.
        let max_x = viewport
            .x
            .saturating_add(viewport.width.min(i32::MAX as u32) as i32)
            .saturating_sub(REACHABLE_TITLEBAR_HEIGHT);
        let max_y = viewport
            .y
            .saturating_add(viewport.height.min(i32::MAX as u32) as i32)
            .saturating_sub(REACHABLE_TITLEBAR_HEIGHT);
        window.bounds.x = window.bounds.x.clamp(
            viewport
                .x
                .saturating_sub(window.bounds.width as i32 - REACHABLE_TITLEBAR_HEIGHT),
            max_x.max(viewport.x),
        );
        window.bounds.y = window.bounds.y.clamp(viewport.y, max_y.max(viewport.y));
        Ok(())
    }

    pub fn maximize(&mut self, surface: SurfaceId, viewport: Rect) -> Result<(), WindowError> {
        if viewport.width == 0 || viewport.height == 0 {
            return Err(WindowError::InvalidSize);
        }
        let window = self.active_window_mut(surface)?;
        if window.state == WindowState::Minimized {
            return Err(WindowError::Closed);
        }
        if window.state == WindowState::Closed {
            return Err(WindowError::Closed);
        }
        if window.state != WindowState::Maximized {
            window.restore_bounds = Some(window.bounds);
        }
        window.bounds = viewport;
        window.state = WindowState::Maximized;
        Ok(())
    }

    pub fn minimize(&mut self, surface: SurfaceId) -> Result<(), WindowError> {
        let window = self.active_window_mut(surface)?;
        if window.state == WindowState::Minimized {
            return Ok(());
        }
        window.state_before_minimize = window.state;
        window.state = WindowState::Minimized;
        window.focused = false;
        Ok(())
    }

    pub fn restore(&mut self, surface: SurfaceId) -> Result<(), WindowError> {
        let window = self.active_window_mut(surface)?;
        if window.state == WindowState::Minimized {
            window.state = window.state_before_minimize;
        } else if window.state == WindowState::Maximized || window.state == WindowState::Fullscreen
        {
            if let Some(bounds) = window.restore_bounds.take() {
                window.bounds = bounds;
            }
            window.state = WindowState::Normal;
        }
        Ok(())
    }

    pub fn set_state(&mut self, surface: SurfaceId, state: WindowState) -> Result<(), WindowError> {
        let window = self
            .windows
            .iter_mut()
            .find(|w| w.surface == surface)
            .ok_or(WindowError::Missing)?;
        if window.state == WindowState::Closed {
            return Err(WindowError::Closed);
        }
        if state == WindowState::Minimized {
            window.state_before_minimize = window.state;
        }
        if state == WindowState::Closed {
            window.focused = false;
        }
        window.state = state;
        Ok(())
    }

    pub fn close(&mut self, surface: SurfaceId) -> Result<(), WindowError> {
        self.set_state(surface, WindowState::Closed)
    }

    fn active_window_mut(&mut self, surface: SurfaceId) -> Result<&mut Window, WindowError> {
        let window = self
            .windows
            .iter_mut()
            .find(|w| w.surface == surface)
            .ok_or(WindowError::Missing)?;
        if window.state == WindowState::Closed {
            return Err(WindowError::Closed);
        }
        Ok(window)
    }

    pub fn focused(&self) -> Option<&Window> {
        self.windows.iter().find(|w| w.focused)
    }

    pub fn windows(&self) -> &[Window] {
        &self.windows
    }

    pub fn placements(&self) -> impl Iterator<Item = SurfacePlacement> + '_ {
        self.windows.iter().enumerate().filter_map(|(i, w)| {
            if w.state == WindowState::Closed || w.state == WindowState::Minimized {
                return None;
            }
            Some(SurfacePlacement {
                surface: w.surface,
                bounds: w.bounds,
                z: i as u32 + 1,
                visible: true,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn viewport() -> Rect {
        Rect::new(0, 0, 1920, 1080)
    }

    #[test]
    fn focus_is_exclusive_and_updates_z_order() {
        let mut wm = WindowManager::new();
        wm.open(SurfaceId(1), "one", Rect::new(0, 0, 100, 100))
            .unwrap();
        wm.open(SurfaceId(2), "two", Rect::new(10, 10, 100, 100))
            .unwrap();
        assert_eq!(wm.focused().unwrap().surface, SurfaceId(2));
        wm.focus(SurfaceId(1)).unwrap();
        assert_eq!(wm.focused().unwrap().surface, SurfaceId(1));
        assert_eq!(wm.placements().last().unwrap().surface, SurfaceId(1));
        assert_eq!(wm.windows().iter().filter(|w| w.focused).count(), 1);
    }

    #[test]
    fn move_clamps_titlebar_to_reachable_viewport() {
        let mut wm = WindowManager::new();
        wm.open(SurfaceId(1), "one", Rect::new(10, 10, 400, 300))
            .unwrap();
        wm.move_window(SurfaceId(1), i32::MAX, i32::MAX, viewport())
            .unwrap();
        let bounds = wm.windows()[0].bounds;
        assert!(bounds.x <= 1920 - REACHABLE_TITLEBAR_HEIGHT);
        assert!(bounds.y <= 1080 - REACHABLE_TITLEBAR_HEIGHT);
    }

    #[test]
    fn maximize_restore_preserves_normal_geometry() {
        let mut wm = WindowManager::new();
        let original = Rect::new(20, 30, 640, 480);
        wm.open(SurfaceId(1), "one", original).unwrap();
        wm.maximize(SurfaceId(1), viewport()).unwrap();
        assert_eq!(wm.windows()[0].bounds, viewport());
        wm.restore(SurfaceId(1)).unwrap();
        assert_eq!(wm.windows()[0].bounds, original);
        assert_eq!(wm.windows()[0].state, WindowState::Normal);
    }

    #[test]
    fn minimize_hides_from_placements_and_restore_reveals() {
        let mut wm = WindowManager::new();
        wm.open(SurfaceId(1), "one", Rect::new(20, 30, 640, 480))
            .unwrap();
        wm.minimize(SurfaceId(1)).unwrap();
        assert!(wm.placements().next().is_none());
        wm.restore(SurfaceId(1)).unwrap();
        assert_eq!(wm.placements().count(), 1);
    }

    #[test]
    fn rejects_too_small_window_geometry() {
        let mut wm = WindowManager::new();
        assert_eq!(
            wm.open(SurfaceId(1), "tiny", Rect::new(0, 0, 1, 1)),
            Err(WindowError::InvalidSize)
        );
        wm.open(SurfaceId(1), "one", Rect::new(0, 0, 100, 100))
            .unwrap();
        assert_eq!(
            wm.resize_window(SurfaceId(1), 1, 100, viewport()),
            Err(WindowError::InvalidSize)
        );
    }
}

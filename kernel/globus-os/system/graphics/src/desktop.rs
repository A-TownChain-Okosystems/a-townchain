//! GlobusOS desktop session orchestration.

use crate::{
    SurfaceId,
    compositor::{Compositor, Rect, SurfacePlacement},
    input::{InputEvent, InputRouter, RoutedEvent},
    wm::{WindowError, WindowManager, WindowState},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesktopViewport {
    pub width: u32,
    pub height: u32,
}

impl DesktopViewport {
    fn rect(self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopMode {
    Locked,
    Active,
    Suspended,
    Shutdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopError {
    Locked,
    Shutdown,
    Window(WindowError),
}

impl From<WindowError> for DesktopError {
    fn from(value: WindowError) -> Self {
        Self::Window(value)
    }
}

#[derive(Debug)]
pub struct DesktopSession {
    mode: DesktopMode,
    viewport: DesktopViewport,
    compositor: Compositor,
    windows: WindowManager,
    input: InputRouter,
    next_surface: u64,
}

impl DesktopSession {
    pub fn new(viewport: DesktopViewport) -> Self {
        Self {
            mode: DesktopMode::Locked,
            viewport,
            compositor: Compositor::new(),
            windows: WindowManager::new(),
            input: InputRouter::new(),
            next_surface: 1,
        }
    }

    pub fn unlock(&mut self) {
        if self.mode == DesktopMode::Locked {
            self.mode = DesktopMode::Active;
        }
    }

    pub fn lock(&mut self) {
        if self.mode != DesktopMode::Shutdown {
            self.mode = DesktopMode::Locked;
            self.input.set_keyboard_focus(None);
            self.input.set_pointer_focus(None);
        }
    }

    pub fn suspend(&mut self) {
        if self.mode == DesktopMode::Active {
            self.mode = DesktopMode::Suspended;
            self.input.set_keyboard_focus(None);
            self.input.set_pointer_focus(None);
        }
    }

    pub fn resume(&mut self) {
        if self.mode == DesktopMode::Suspended {
            self.mode = DesktopMode::Active;
        }
    }

    pub fn shutdown(&mut self) {
        self.mode = DesktopMode::Shutdown;
        self.input.set_keyboard_focus(None);
        self.input.set_pointer_focus(None);
    }

    pub fn mode(&self) -> DesktopMode {
        self.mode
    }

    pub fn viewport(&self) -> DesktopViewport {
        self.viewport
    }

    /// Updates the display work area and clamps windows so their title bars
    /// remain reachable after a resolution change.
    pub fn set_viewport(&mut self, viewport: DesktopViewport) -> Result<(), DesktopError> {
        if viewport.width == 0 || viewport.height == 0 {
            return Err(DesktopError::Window(WindowError::InvalidSize));
        }
        self.viewport = viewport;
        for window in self.windows.windows().to_vec() {
            if window.state == WindowState::Normal {
                self.windows.move_window(
                    window.surface,
                    window.bounds.x,
                    window.bounds.y,
                    viewport.rect(),
                )?;
                self.windows.resize_window(
                    window.surface,
                    window.bounds.width.min(viewport.width),
                    window.bounds.height.min(viewport.height),
                    viewport.rect(),
                )?;
            } else if window.state == WindowState::Maximized {
                self.windows.maximize(window.surface, viewport.rect())?;
            }
        }
        self.sync_compositor()?;
        Ok(())
    }

    pub fn open_window(
        &mut self,
        title: impl Into<String>,
        bounds: Rect,
    ) -> Result<SurfaceId, DesktopError> {
        if self.mode != DesktopMode::Active {
            return Err(DesktopError::Locked);
        }
        let id = SurfaceId(self.next_surface);
        self.next_surface = self
            .next_surface
            .checked_add(1)
            .ok_or(DesktopError::Shutdown)?;
        self.compositor
            .create(id, bounds)
            .map_err(|_| DesktopError::Window(WindowError::InvalidSize))?;
        if let Err(error) = self.windows.open(id, title, bounds) {
            let _ = self.compositor.remove(id);
            return Err(error.into());
        }
        self.windows.focus(id)?;
        self.compositor
            .focus(id)
            .map_err(|_| DesktopError::Window(WindowError::Missing))?;
        self.input.set_pointer_focus(Some(id));
        self.input.set_keyboard_focus(Some(id));
        self.sync_compositor()?;
        Ok(id)
    }

    pub fn focus_window(&mut self, surface: SurfaceId) -> Result<(), DesktopError> {
        if self.mode != DesktopMode::Active {
            return Err(DesktopError::Locked);
        }
        self.windows.focus(surface)?;
        self.input.set_pointer_focus(Some(surface));
        self.input.set_keyboard_focus(Some(surface));
        self.sync_compositor()
    }

    pub fn move_window(&mut self, surface: SurfaceId, x: i32, y: i32) -> Result<(), DesktopError> {
        self.require_active()?;
        self.windows
            .move_window(surface, x, y, self.viewport.rect())?;
        self.sync_compositor()
    }

    pub fn resize_window(
        &mut self,
        surface: SurfaceId,
        width: u32,
        height: u32,
    ) -> Result<(), DesktopError> {
        self.require_active()?;
        self.windows
            .resize_window(surface, width, height, self.viewport.rect())?;
        self.sync_compositor()
    }

    pub fn maximize_window(&mut self, surface: SurfaceId) -> Result<(), DesktopError> {
        self.require_active()?;
        self.windows.maximize(surface, self.viewport.rect())?;
        self.sync_compositor()
    }

    pub fn minimize_window(&mut self, surface: SurfaceId) -> Result<(), DesktopError> {
        self.require_active()?;
        self.windows.minimize(surface)?;
        if self.input.keyboard_focus() == Some(surface) {
            self.input.set_keyboard_focus(None);
        }
        if self.input.pointer_focus() == Some(surface) {
            self.input.set_pointer_focus(None);
        }
        self.sync_compositor()
    }

    pub fn restore_window(&mut self, surface: SurfaceId) -> Result<(), DesktopError> {
        self.require_active()?;
        self.windows.restore(surface)?;
        self.windows.focus(surface)?;
        self.input.set_pointer_focus(Some(surface));
        self.input.set_keyboard_focus(Some(surface));
        self.sync_compositor()
    }

    fn require_active(&self) -> Result<(), DesktopError> {
        if self.mode == DesktopMode::Active {
            Ok(())
        } else if self.mode == DesktopMode::Shutdown {
            Err(DesktopError::Shutdown)
        } else {
            Err(DesktopError::Locked)
        }
    }

    fn sync_compositor(&mut self) -> Result<(), DesktopError> {
        for (index, window) in self.windows.windows().iter().enumerate() {
            if window.state == WindowState::Closed {
                continue;
            }
            let placement = SurfacePlacement {
                surface: window.surface,
                bounds: window.bounds,
                z: index as u32 + 1,
                visible: window.state != WindowState::Minimized,
            };
            self.compositor
                .set_placement(placement)
                .map_err(|_| DesktopError::Window(WindowError::Missing))?;
        }
        Ok(())
    }

    pub fn close_window(&mut self, surface: SurfaceId) -> Result<(), DesktopError> {
        if self.mode == DesktopMode::Shutdown {
            return Err(DesktopError::Shutdown);
        }
        self.windows.close(surface)?;
        self.compositor
            .remove(surface)
            .map_err(|_| DesktopError::Window(WindowError::Missing))?;
        if self.input.keyboard_focus() == Some(surface) {
            self.input.set_keyboard_focus(None);
        }
        if self.input.pointer_focus() == Some(surface) {
            self.input.set_pointer_focus(None);
        }
        Ok(())
    }

    pub fn set_window_state(
        &mut self,
        surface: SurfaceId,
        state: WindowState,
    ) -> Result<(), DesktopError> {
        if state == WindowState::Closed {
            return self.close_window(surface);
        }
        self.require_active()?;
        match state {
            WindowState::Minimized => self.windows.minimize(surface)?,
            WindowState::Maximized => self.windows.maximize(surface, self.viewport.rect())?,
            WindowState::Normal | WindowState::Fullscreen => {
                self.windows.set_state(surface, state)?
            }
            WindowState::Closed => unreachable!("closed state handled above"),
        }
        self.sync_compositor()
    }

    pub fn route_input(&mut self, event: InputEvent) -> Result<RoutedEvent, DesktopError> {
        if self.mode != DesktopMode::Active {
            return Err(if self.mode == DesktopMode::Shutdown {
                DesktopError::Shutdown
            } else {
                DesktopError::Locked
            });
        }
        match event {
            InputEvent::PointerMove { x, y } => {
                self.input.set_pointer_focus(self.hit_test(x, y));
            }
            InputEvent::PointerButton {
                pressed: true,
                x,
                y,
                ..
            } => {
                if let Some(surface) = self.hit_test(x, y) {
                    self.focus_window(surface)?;
                } else {
                    self.input.set_pointer_focus(None);
                    self.input.set_keyboard_focus(None);
                }
            }
            _ => {}
        }
        Ok(self.input.route(event))
    }

    pub fn hit_test(&self, x: i32, y: i32) -> Option<SurfaceId> {
        if self.mode != DesktopMode::Active {
            return None;
        }
        self.compositor.hit_test(x, y)
    }

    pub fn windows(&self) -> &[crate::wm::Window] {
        self.windows.windows()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn desktop() -> DesktopSession {
        let mut desktop = DesktopSession::new(DesktopViewport {
            width: 1920,
            height: 1080,
        });
        desktop.unlock();
        desktop
    }

    #[test]
    fn locked_desktop_cannot_open_windows() {
        let mut desktop = DesktopSession::new(DesktopViewport {
            width: 1920,
            height: 1080,
        });
        assert!(matches!(
            desktop.open_window("Shell", Rect::new(0, 0, 100, 100)),
            Err(DesktopError::Locked)
        ));
    }

    #[test]
    fn active_session_routes_input_to_open_window() {
        let mut desktop = desktop();
        let surface = desktop
            .open_window("Shell", Rect::new(0, 0, 500, 500))
            .unwrap();
        let routed = desktop
            .route_input(InputEvent::PointerMove { x: 10, y: 10 })
            .unwrap();
        assert_eq!(routed.target, Some(surface));
    }

    #[test]
    fn move_resize_minimize_restore_and_close_update_compositor() {
        let mut desktop = desktop();
        let surface = desktop
            .open_window("Editor", Rect::new(20, 30, 640, 480))
            .unwrap();
        desktop.move_window(surface, 100, 120).unwrap();
        desktop.resize_window(surface, 800, 600).unwrap();
        assert_eq!(desktop.hit_test(110, 130), Some(surface));
        desktop.minimize_window(surface).unwrap();
        assert_eq!(desktop.hit_test(110, 130), None);
        desktop.restore_window(surface).unwrap();
        assert_eq!(desktop.hit_test(110, 130), Some(surface));
        desktop.close_window(surface).unwrap();
        assert_eq!(desktop.hit_test(110, 130), None);
    }

    #[test]
    fn maximize_restore_and_display_change_preserve_reachable_geometry() {
        let mut desktop = desktop();
        let surface = desktop
            .open_window("Editor", Rect::new(20, 30, 640, 480))
            .unwrap();
        desktop.maximize_window(surface).unwrap();
        assert_eq!(desktop.windows()[0].bounds, Rect::new(0, 0, 1920, 1080));
        desktop.restore_window(surface).unwrap();
        assert_eq!(desktop.windows()[0].bounds, Rect::new(20, 30, 640, 480));
        desktop
            .set_viewport(DesktopViewport {
                width: 800,
                height: 600,
            })
            .unwrap();
        assert!(desktop.windows()[0].bounds.x <= 800 - 24);
        assert!(desktop.windows()[0].bounds.y <= 600 - 24);
    }

    #[test]
    fn non_active_desktop_does_not_hit_test_or_mutate_windows() {
        let mut desktop = desktop();
        let surface = desktop
            .open_window("Editor", Rect::new(20, 30, 640, 480))
            .unwrap();
        desktop.suspend();
        assert_eq!(desktop.hit_test(30, 40), None);
        assert!(matches!(
            desktop.move_window(surface, 0, 0),
            Err(DesktopError::Locked)
        ));
    }
}

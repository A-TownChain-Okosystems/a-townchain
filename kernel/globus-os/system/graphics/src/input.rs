//! Desktop input routing contracts.

use crate::SurfaceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Escape,
    Enter,
    Tab,
    Backspace,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Character(u32),
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    PointerMove {
        x: i32,
        y: i32,
    },
    PointerButton {
        button: Button,
        pressed: bool,
        x: i32,
        y: i32,
    },
    Key {
        code: KeyCode,
        pressed: bool,
        modifiers: Modifiers,
    },
    Scroll {
        delta_x: i32,
        delta_y: i32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoutedEvent {
    pub target: Option<SurfaceId>,
    pub event: InputEvent,
}

#[derive(Debug, Default)]
pub struct InputRouter {
    pointer_focus: Option<SurfaceId>,
    keyboard_focus: Option<SurfaceId>,
}

impl InputRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_pointer_focus(&mut self, target: Option<SurfaceId>) {
        self.pointer_focus = target;
    }

    pub fn set_keyboard_focus(&mut self, target: Option<SurfaceId>) {
        self.keyboard_focus = target;
    }

    pub fn route(&self, event: InputEvent) -> RoutedEvent {
        let target = match event {
            InputEvent::PointerMove { .. }
            | InputEvent::PointerButton { .. }
            | InputEvent::Scroll { .. } => self.pointer_focus,
            InputEvent::Key { .. } => self.keyboard_focus,
        };
        RoutedEvent { target, event }
    }

    pub fn pointer_focus(&self) -> Option<SurfaceId> {
        self.pointer_focus
    }

    pub fn keyboard_focus(&self) -> Option<SurfaceId> {
        self.keyboard_focus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_and_pointer_have_independent_targets() {
        let mut r = InputRouter::new();
        r.set_pointer_focus(Some(SurfaceId(1)));
        r.set_keyboard_focus(Some(SurfaceId(2)));
        let pointer = r.route(InputEvent::PointerMove { x: 1, y: 2 });
        let key = r.route(InputEvent::Key {
            code: KeyCode::Enter,
            pressed: true,
            modifiers: Modifiers {
                shift: false,
                ctrl: false,
                alt: false,
                meta: false,
            },
        });
        assert_eq!(pointer.target, Some(SurfaceId(1)));
        assert_eq!(key.target, Some(SurfaceId(2)));
        assert_eq!(r.pointer_focus(), Some(SurfaceId(1)));
    }
}

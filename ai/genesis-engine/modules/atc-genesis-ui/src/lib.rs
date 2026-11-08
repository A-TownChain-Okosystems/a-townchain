#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x <= self.x + self.width && y <= self.y + self.height
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiPointerButton {
    Primary,
    Secondary,
    Middle,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UiEvent {
    PointerMove {
        x: f32,
        y: f32,
    },
    PointerDown {
        x: f32,
        y: f32,
        button: UiPointerButton,
    },
    PointerUp {
        x: f32,
        y: f32,
        button: UiPointerButton,
    },
    KeyDown {
        key: u32,
    },
    KeyUp {
        key: u32,
    },
}
pub trait UiWidget {
    fn bounds(&self) -> Rect;
    fn handle_event(&mut self, event: UiEvent) -> bool;
}
#[derive(Default)]
pub struct UiFocus {
    focused: Option<u32>,
}
impl UiFocus {
    pub fn set(&mut self, id: Option<u32>) {
        self.focused = id
    }
    pub fn get(&self) -> Option<u32> {
        self.focused
    }
}
pub struct UiButton {
    pub id: u32,
    pub rect: Rect,
    pub pressed: bool,
}
impl UiWidget for UiButton {
    fn bounds(&self) -> Rect {
        self.rect
    }
    fn handle_event(&mut self, event: UiEvent) -> bool {
        match event {
            UiEvent::PointerDown {
                x,
                y,
                button: UiPointerButton::Primary,
            } if self.rect.contains(x, y) => {
                self.pressed = true;
                true
            }
            UiEvent::PointerUp {
                x,
                y,
                button: UiPointerButton::Primary,
            } => {
                let was = self.pressed;
                self.pressed = false;
                was && self.rect.contains(x, y)
            }
            _ => false,
        }
    }
}
#[derive(Default)]
pub struct UiTree {
    widgets: Vec<UiButton>,
    focus: UiFocus,
}
impl UiTree {
    pub fn add_button(&mut self, button: UiButton) {
        self.widgets.push(button)
    }
    pub fn dispatch(&mut self, event: UiEvent) -> Option<u32> {
        for w in self.widgets.iter_mut().rev() {
            if w.handle_event(event) {
                if matches!(event, UiEvent::PointerDown { .. }) {
                    self.focus.set(Some(w.id))
                }
                return Some(w.id);
            }
        }
        None
    }
    pub fn focus(&self) -> Option<u32> {
        self.focus.get()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn button_dispatches() {
        let mut ui = UiTree::default();
        ui.add_button(UiButton {
            id: 7,
            rect: Rect {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
            },
            pressed: false,
        });
        assert_eq!(
            ui.dispatch(UiEvent::PointerDown {
                x: 2.0,
                y: 2.0,
                button: UiPointerButton::Primary
            }),
            Some(7)
        );
        assert_eq!(ui.focus(), Some(7));
        assert_eq!(
            ui.dispatch(UiEvent::PointerUp {
                x: 2.0,
                y: 2.0,
                button: UiPointerButton::Primary
            }),
            Some(7)
        );
    }
}

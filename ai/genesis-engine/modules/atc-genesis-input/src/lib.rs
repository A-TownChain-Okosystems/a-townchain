#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Unknown(u32),
    Escape,
    Enter,
    Space,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputEvent {
    KeyPressed(Key),
    KeyReleased(Key),
    MouseMoved { x: f32, y: f32 },
    MouseButton { button: u8, pressed: bool },
    Axis { id: u8, value: f32 },
}

#[derive(Default)]
pub struct InputState {
    pressed: std::collections::HashSet<Key>,
    axes: std::collections::HashMap<u8, f32>,
}

impl InputState {
    pub fn apply(&mut self, event: InputEvent) {
        match event {
            InputEvent::KeyPressed(k) => {
                self.pressed.insert(k);
            }
            InputEvent::KeyReleased(k) => {
                self.pressed.remove(&k);
            }
            InputEvent::Axis { id, value } => {
                self.axes.insert(id, value.clamp(-1.0, 1.0));
            }
            _ => {}
        }
    }
    pub fn pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }
    pub fn axis(&self, id: u8) -> f32 {
        self.axes.get(&id).copied().unwrap_or(0.0)
    }
}

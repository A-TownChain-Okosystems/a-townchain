// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Entity component system
pub struct Entity { pub id: u64, pub x: i32, pub y: i32, pub active: bool }
impl Entity {
    pub fn new(id: u64) -> Self { Self { id, x: 0, y: 0, active: true } }
    pub fn move_to(&mut self, x: i32, y: i32) { self.x = x; self.y = y; }
    pub fn destroy(&mut self) { self.active = false; }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_entity() {
        let mut e = Entity::new(1);
        e.move_to(10, 20);
        assert_eq!((e.x, e.y), (10, 20));
        e.destroy();
        assert!(!e.active);
    }
}

//! Deterministic character/NPC needs and settlement utility foundation.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Needs {
    pub hunger: u16,
    pub thirst: u16,
    pub sleep: u16,
    pub comfort: u16,
    pub health: u16,
}

impl Default for Needs {
    fn default() -> Self { Self { hunger: 0, thirst: 0, sleep: 0, comfort: 1000, health: 1000 } }
}

impl Needs {
    pub fn tick(&mut self, ticks: u32) {
        let t = ticks.min(u16::MAX as u32) as u16;
        self.hunger = self.hunger.saturating_add(t);
        self.thirst = self.thirst.saturating_add(t.saturating_mul(2));
        self.sleep = self.sleep.saturating_add(t);
        self.comfort = self.comfort.saturating_sub(t);
        if self.hunger > 900 || self.thirst > 900 { self.health = self.health.saturating_sub(t); }
    }

    pub fn feed(&mut self, amount: u16) { self.hunger = self.hunger.saturating_sub(amount); }
    pub fn drink(&mut self, amount: u16) { self.thirst = self.thirst.saturating_sub(amount); }
    pub fn rest(&mut self, amount: u16) { self.sleep = self.sleep.saturating_sub(amount); }
    pub fn furnish(&mut self, comfort: u16) { self.comfort = self.comfort.saturating_add(comfort).min(1000); }

    pub fn critical(&self) -> bool { self.hunger >= 900 || self.thirst >= 900 || self.health == 0 }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeedAction { Eat, Drink, Sleep, ImproveComfort, Idle }

pub fn next_need_action(needs: Needs) -> NeedAction {
    if needs.thirst >= needs.hunger && needs.thirst >= needs.sleep && needs.thirst >= 700 { return NeedAction::Drink; }
    if needs.hunger >= needs.sleep && needs.hunger >= 700 { return NeedAction::Eat; }
    if needs.sleep >= 700 { return NeedAction::Sleep; }
    if needs.comfort <= 300 { return NeedAction::ImproveComfort; }
    NeedAction::Idle
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn needs_decay_and_health_pressure_are_deterministic() { let mut n=Needs::default(); n.tick(100); assert_eq!(n.hunger,100); assert_eq!(n.thirst,200); assert_eq!(n.comfort,900); n.tick(800); assert!(n.critical()); }
    #[test] fn actions_prioritize_thirst() { let mut n=Needs::default(); n.hunger=800; n.thirst=900; assert_eq!(next_need_action(n),NeedAction::Drink); }
    #[test] fn recovery_is_bounded() { let mut n=Needs::default(); n.feed(500); n.drink(500); n.rest(500); n.furnish(500); assert_eq!(n,Needs::default()); }
}

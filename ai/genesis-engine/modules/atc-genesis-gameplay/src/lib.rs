use atc_genesis_ecs::World;
use atc_genesis_input::{InputState, Key};
use atc_genesis_platform::EntityId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementConfig {
    pub speed: f32,
    pub axis_x: Option<u8>,
    pub axis_z: Option<u8>,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            speed: 5.0,
            axis_x: None,
            axis_z: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameplayCommand {
    pub entity: EntityId,
    pub movement: [f32; 3],
}

#[derive(Default)]
pub struct GameplayRuntime {
    pub commands: Vec<GameplayCommand>,
}

impl GameplayRuntime {
    pub fn sample_movement(
        &mut self,
        input: &InputState,
        entity: EntityId,
        config: MovementConfig,
    ) -> GameplayCommand {
        let mut x = input.axis(config.axis_x.unwrap_or(255));
        let mut z = input.axis(config.axis_z.unwrap_or(254));
        if x == 0.0 {
            x = if input.pressed(Key::Right) {
                1.0
            } else if input.pressed(Key::Left) {
                -1.0
            } else {
                0.0
            };
        }
        if z == 0.0 {
            z = if input.pressed(Key::Down) {
                1.0
            } else if input.pressed(Key::Up) {
                -1.0
            } else {
                0.0
            };
        }

        x = x.clamp(-1.0, 1.0);
        z = z.clamp(-1.0, 1.0);
        let magnitude = (x * x + z * z).sqrt();
        if magnitude > 1.0 {
            x /= magnitude;
            z /= magnitude;
        }

        let command = GameplayCommand {
            entity,
            movement: [x * config.speed, 0.0, z * config.speed],
        };
        self.commands.push(command);
        command
    }

    pub fn apply(&mut self, world: &mut World, dt_seconds: f32) -> usize {
        let dt = dt_seconds.max(0.0);
        let commands = std::mem::take(&mut self.commands);
        let mut applied = 0;
        for command in commands {
            if let Some(mut t) = world.transform(command.entity).copied() {
                t.translation[0] += command.movement[0] * dt;
                t.translation[1] += command.movement[1] * dt;
                t.translation[2] += command.movement[2] * dt;
                if world.set_transform(command.entity, t) {
                    applied += 1;
                }
            }
        }
        applied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atc_genesis_input::{InputEvent, Key};
    use atc_genesis_platform::Transform;

    #[test]
    fn keyboard_moves_entity_deterministically() {
        let mut input = InputState::default();
        input.apply(InputEvent::KeyPressed(Key::Right));
        let mut w = World::new();
        let e = w.spawn(Transform::default());
        let mut g = GameplayRuntime::default();
        g.sample_movement(
            &input,
            e,
            MovementConfig {
                speed: 2.0,
                ..Default::default()
            },
        );
        assert_eq!(g.apply(&mut w, 0.5), 1);
        assert_eq!(w.transform(e).unwrap().translation[0], 1.0);
    }

    #[test]
    fn axis_is_clamped() {
        let mut input = InputState::default();
        input.apply(InputEvent::Axis { id: 1, value: 3.0 });
        let e = EntityId(1);
        let mut g = GameplayRuntime::default();
        let c = g.sample_movement(
            &input,
            e,
            MovementConfig {
                speed: 2.0,
                axis_x: Some(1),
                ..Default::default()
            },
        );
        assert_eq!(c.movement[0], 2.0);
    }

    #[test]
    fn diagonal_movement_preserves_max_speed() {
        let mut input = InputState::default();
        input.apply(InputEvent::KeyPressed(Key::Right));
        input.apply(InputEvent::KeyPressed(Key::Down));
        let mut g = GameplayRuntime::default();
        let c = g.sample_movement(
            &input,
            EntityId(1),
            MovementConfig {
                speed: 6.0,
                ..Default::default()
            },
        );
        let magnitude = (c.movement[0] * c.movement[0] + c.movement[2] * c.movement[2]).sqrt();
        assert!((magnitude - 6.0).abs() < 0.0001);
        assert!((c.movement[0] - c.movement[2]).abs() < 0.0001);
    }
}

use atc_genesis_platform::EntityId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl Aabb {
    pub fn contains(&self, p: [f32; 3]) -> bool {
        (0..3).all(|i| p[i] >= self.min[i] && p[i] <= self.max[i])
    }

    pub fn intersects(&self, other: &Self) -> bool {
        (0..3).all(|i| self.min[i] <= other.max[i] && self.max[i] >= other.min[i])
    }

    pub fn ray_intersection(
        &self,
        origin: [f32; 3],
        direction: [f32; 3],
        max_distance: f32,
    ) -> Option<f32> {
        if max_distance < 0.0 {
            return None;
        }
        let mut near = 0.0f32;
        let mut far = max_distance;
        for i in 0..3 {
            if direction[i].abs() < f32::EPSILON {
                if origin[i] < self.min[i] || origin[i] > self.max[i] {
                    return None;
                }
                continue;
            }
            let inv = 1.0 / direction[i];
            let mut t1 = (self.min[i] - origin[i]) * inv;
            let mut t2 = (self.max[i] - origin[i]) * inv;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            near = near.max(t1);
            far = far.min(t2);
            if near > far {
                return None;
            }
        }
        Some(near)
    }

    pub fn point_resolution(&self, point: [f32; 3]) -> Option<([f32; 3], f32)> {
        if !self.contains(point) {
            return None;
        }
        let distances = [
            point[0] - self.min[0],
            self.max[0] - point[0],
            point[1] - self.min[1],
            self.max[1] - point[1],
            point[2] - self.min[2],
            self.max[2] - point[2],
        ];
        let mut best = 0usize;
        for i in 1..distances.len() {
            if distances[i] < distances[best] {
                best = i;
            }
        }
        let normal = match best {
            0 => [-1.0, 0.0, 0.0],
            1 => [1.0, 0.0, 0.0],
            2 => [0.0, -1.0, 0.0],
            3 => [0.0, 1.0, 0.0],
            4 => [0.0, 0.0, -1.0],
            _ => [0.0, 0.0, 1.0],
        };
        Some((normal, distances[best]))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Collider {
    pub entity: EntityId,
    pub bounds: Aabb,
}

#[derive(Default)]
pub struct CollisionWorld {
    colliders: Vec<Collider>,
}

impl CollisionWorld {
    pub fn add(&mut self, collider: Collider) {
        self.colliders.push(collider);
        self.colliders.sort_by_key(|c| c.entity.0);
    }

    pub fn remove(&mut self, entity: EntityId) -> bool {
        if let Some(i) = self.colliders.iter().position(|c| c.entity == entity) {
            self.colliders.remove(i);
            true
        } else {
            false
        }
    }

    pub fn query_aabb(&self, bounds: Aabb) -> Vec<EntityId> {
        self.colliders
            .iter()
            .filter(|c| c.bounds.intersects(&bounds))
            .map(|c| c.entity)
            .collect()
    }

    pub fn resolve_point(&self, point: [f32; 3]) -> Option<(EntityId, [f32; 3], f32)> {
        self.colliders
            .iter()
            .filter_map(|c| {
                c.bounds
                    .point_resolution(point)
                    .map(|(n, d)| (c.entity, n, d))
            })
            .min_by(|a, b| {
                a.2.partial_cmp(&b.2)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.0 .0.cmp(&b.0 .0))
            })
    }

    pub fn raycast(
        &self,
        origin: [f32; 3],
        direction: [f32; 3],
        max_distance: f32,
    ) -> Option<EntityId> {
        self.colliders
            .iter()
            .filter_map(|c| {
                c.bounds
                    .ray_intersection(origin, direction, max_distance)
                    .map(|t| (t, c.entity))
            })
            .min_by(|a, b| {
                a.0.partial_cmp(&b.0)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.1 .0.cmp(&b.1 .0))
            })
            .map(|(_, id)| id)
    }

    pub fn clear(&mut self) {
        self.colliders.clear();
    }
    pub fn len(&self) -> usize {
        self.colliders.len()
    }

    /// Gegenstueck zu `len` (Clippy: `len_without_is_empty`).
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raycast_returns_nearest() {
        let mut w = CollisionWorld::default();
        w.add(Collider {
            entity: EntityId(2),
            bounds: Aabb {
                min: [5.0, -1.0, -1.0],
                max: [6.0, 1.0, 1.0],
            },
        });
        w.add(Collider {
            entity: EntityId(1),
            bounds: Aabb {
                min: [2.0, -1.0, -1.0],
                max: [3.0, 1.0, 1.0],
            },
        });
        assert_eq!(
            w.raycast([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 10.0),
            Some(EntityId(1))
        );
    }

    #[test]
    fn point_resolution_uses_nearest_face() {
        let aabb = Aabb {
            min: [-1.0; 3],
            max: [1.0; 3],
        };
        let (normal, dist) = aabb
            .point_resolution([0.9, 0.0, 0.0])
            .expect("Face erwartet");
        for (ist, soll) in normal.iter().zip([1.0f32, 0.0, 0.0]) {
            assert!(
                (ist - soll).abs() < 1e-5,
                "Normal weicht ab: {ist} != {soll}"
            );
        }
        assert!(
            (dist - 0.1).abs() < 1e-5,
            "Distanz weicht ab: {dist} != 0.1"
        );
    }

    #[test]
    fn resolution_is_entity_deterministic() {
        let mut w = CollisionWorld::default();
        let bounds = Aabb {
            min: [-1.0; 3],
            max: [1.0; 3],
        };
        w.add(Collider {
            entity: EntityId(2),
            bounds,
        });
        w.add(Collider {
            entity: EntityId(1),
            bounds,
        });
        assert_eq!(w.resolve_point([0.0; 3]).unwrap().0, EntityId(1));
    }
}

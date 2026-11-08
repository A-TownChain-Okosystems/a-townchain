#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BehaviorState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Flee,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiAgent {
    pub state: BehaviorState,
    pub detection_radius: f32,
    pub attack_radius: f32,
    pub health_fraction: f32,
    pub speed: f32,
}

impl Default for AiAgent {
    fn default() -> Self {
        Self {
            state: BehaviorState::Idle,
            detection_radius: 20.0,
            attack_radius: 2.0,
            health_fraction: 1.0,
            speed: 3.0,
        }
    }
}

impl AiAgent {
    pub fn decide(&mut self, distance: Option<f32>) -> BehaviorState {
        self.state = if self.health_fraction < 0.2 {
            BehaviorState::Flee
        } else if let Some(d) = distance {
            if d <= self.attack_radius {
                BehaviorState::Attack
            } else if d <= self.detection_radius {
                BehaviorState::Chase
            } else {
                BehaviorState::Patrol
            }
        } else {
            BehaviorState::Patrol
        };
        self.state
    }

    pub fn desired_velocity(&self, direction: [f32; 3]) -> [f32; 3] {
        match self.state {
            BehaviorState::Chase | BehaviorState::Flee => {
                let n = (direction[0] * direction[0]
                    + direction[1] * direction[1]
                    + direction[2] * direction[2])
                    .sqrt();
                if n > f32::EPSILON {
                    let s = if self.state == BehaviorState::Flee {
                        -self.speed
                    } else {
                        self.speed
                    };
                    [
                        direction[0] / n * s,
                        direction[1] / n * s,
                        direction[2] / n * s,
                    ]
                } else {
                    [0.0; 3]
                }
            }
            _ => [0.0; 3],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavCell {
    Walkable,
    Blocked,
}

#[derive(Clone, Debug)]
pub struct GridNav {
    pub width: u32,
    pub height: u32,
    cells: Vec<NavCell>,
}

impl GridNav {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            cells: vec![NavCell::Walkable; (width * height) as usize],
        }
    }

    pub fn set_blocked(&mut self, x: u32, y: u32, blocked: bool) {
        if let Some(c) = self.cells.get_mut((y * self.width + x) as usize) {
            *c = if blocked {
                NavCell::Blocked
            } else {
                NavCell::Walkable
            };
        }
    }

    pub fn walkable(&self, x: u32, y: u32) -> bool {
        x < self.width
            && y < self.height
            && matches!(self.cells[(y * self.width + x) as usize], NavCell::Walkable)
    }

    pub fn neighbors(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let mut n = Vec::with_capacity(4);
        if x > 0 && self.walkable(x - 1, y) {
            n.push((x - 1, y));
        }
        if x + 1 < self.width && self.walkable(x + 1, y) {
            n.push((x + 1, y));
        }
        if y > 0 && self.walkable(x, y - 1) {
            n.push((x, y - 1));
        }
        if y + 1 < self.height && self.walkable(x, y + 1) {
            n.push((x, y + 1));
        }
        n
    }

    pub fn shortest_path(&self, start: (u32, u32), goal: (u32, u32)) -> Option<Vec<(u32, u32)>> {
        if !self.walkable(start.0, start.1) || !self.walkable(goal.0, goal.1) {
            return None;
        }
        if start == goal {
            return Some(vec![start]);
        }

        let mut queue = std::collections::VecDeque::new();
        let mut visited = vec![false; (self.width * self.height) as usize];
        let mut parent: Vec<Option<(u32, u32)>> = vec![None; visited.len()];
        let index = |p: (u32, u32)| (p.1 * self.width + p.0) as usize;

        queue.push_back(start);
        visited[index(start)] = true;

        while let Some(current) = queue.pop_front() {
            for next in self.neighbors(current.0, current.1) {
                let next_index = index(next);
                if visited[next_index] {
                    continue;
                }
                visited[next_index] = true;
                parent[next_index] = Some(current);
                if next == goal {
                    let mut path = vec![goal];
                    let mut cursor = goal;
                    while cursor != start {
                        cursor = parent[index(cursor)]?;
                        path.push(cursor);
                    }
                    path.reverse();
                    return Some(path);
                }
                queue.push_back(next);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_decision() {
        let mut a = AiAgent::default();
        assert_eq!(a.decide(Some(1.0)), BehaviorState::Attack);
        assert_eq!(a.decide(Some(10.0)), BehaviorState::Chase);
    }

    #[test]
    fn navigation_respects_blockers() {
        let mut n = GridNav::new(3, 3);
        n.set_blocked(1, 1, true);
        assert!(!n.walkable(1, 1));
        assert!(!n.neighbors(1, 0).contains(&(1, 1)));
    }

    #[test]
    fn pathfinding_is_deterministic_and_avoids_blockers() {
        let mut n = GridNav::new(4, 3);
        n.set_blocked(1, 0, true);
        n.set_blocked(1, 1, true);
        let path = n.shortest_path((0, 0), (3, 0)).expect("path should exist");
        assert_eq!(path.first(), Some(&(0, 0)));
        assert_eq!(path.last(), Some(&(3, 0)));
        assert!(!path.contains(&(1, 0)));
        assert!(!path.contains(&(1, 1)));
        assert_eq!(path, n.shortest_path((0, 0), (3, 0)).unwrap());
    }

    #[test]
    fn unreachable_goal_returns_none() {
        let mut n = GridNav::new(3, 3);
        n.set_blocked(1, 0, true);
        n.set_blocked(0, 1, true);
        assert_eq!(n.shortest_path((0, 0), (2, 2)), None);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AnimationClipId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AnimationStateId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoneId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PoseTransform {
    pub translation: [f32; 3],
    pub rotation_xyzw: [f32; 4],
    pub scale: [f32; 3],
}
impl Default for PoseTransform {
    fn default() -> Self {
        Self {
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Bone {
    pub id: BoneId,
    pub parent: Option<BoneId>,
    pub bind: PoseTransform,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Skeleton {
    pub bones: Vec<Bone>,
}
impl Skeleton {
    pub fn add_bone(&mut self, bone: Bone) -> bool {
        if self.bones.iter().any(|b| b.id == bone.id) {
            false
        } else {
            self.bones.push(bone);
            true
        }
    }
    pub fn bone(&self, id: BoneId) -> Option<&Bone> {
        self.bones.iter().find(|b| b.id == id)
    }
    pub fn validate(&self) -> bool {
        self.bones
            .iter()
            .all(|b| b.parent.is_none_or(|p| p != b.id && self.bone(p).is_some()))
    }
    pub fn roots(&self) -> Vec<BoneId> {
        self.bones
            .iter()
            .filter_map(|b| b.parent.is_none().then_some(b.id))
            .collect()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Keyframe {
    pub time_seconds: f32,
    pub value: PoseTransform,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationTrack {
    pub bone: BoneId,
    pub keys: Vec<Keyframe>,
}
impl AnimationTrack {
    pub fn sample(&self, time: f32) -> PoseTransform {
        if self.keys.is_empty() {
            return PoseTransform::default();
        }
        if time <= self.keys[0].time_seconds {
            return self.keys[0].value;
        }
        for pair in self.keys.windows(2) {
            let a = &pair[0];
            let b = &pair[1];
            if time <= b.time_seconds {
                let span = (b.time_seconds - a.time_seconds).max(f32::EPSILON);
                return lerp_pose(
                    a.value,
                    b.value,
                    ((time - a.time_seconds) / span).clamp(0.0, 1.0),
                );
            }
        }
        self.keys.last().unwrap().value
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationClip {
    pub id: AnimationClipId,
    pub duration_seconds: f32,
    pub tracks: Vec<AnimationTrack>,
}
impl AnimationClip {
    pub fn sample(&self, time: f32) -> Vec<(BoneId, PoseTransform)> {
        self.tracks
            .iter()
            .map(|t| {
                (
                    t.bone,
                    t.sample(time.clamp(0.0, self.duration_seconds.max(0.0))),
                )
            })
            .collect()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationState {
    pub state: AnimationStateId,
    pub clip: AnimationClipId,
    pub speed: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimationPlayer {
    pub clip: Option<AnimationClipId>,
    pub time_seconds: f32,
    pub speed: f32,
    pub looping: bool,
}
impl Default for AnimationPlayer {
    fn default() -> Self {
        Self {
            clip: None,
            time_seconds: 0.0,
            speed: 1.0,
            looping: true,
        }
    }
}
impl AnimationPlayer {
    pub fn update(&mut self, dt: f32, duration: f32) {
        self.time_seconds += dt.max(0.0) * self.speed;
        if duration > 0.0 {
            if self.looping {
                self.time_seconds = self.time_seconds.rem_euclid(duration)
            } else {
                self.time_seconds = self.time_seconds.min(duration)
            }
        }
    }
    pub fn sample(&self, clip: &AnimationClip) -> Vec<(BoneId, PoseTransform)> {
        clip.sample(self.time_seconds)
    }
}
fn lerp_pose(a: PoseTransform, b: PoseTransform, t: f32) -> PoseTransform {
    let mut out = PoseTransform::default();
    for (i, value) in out.translation.iter_mut().enumerate() {
        *value = a.translation[i] + (b.translation[i] - a.translation[i]) * t;
    }
    for (i, value) in out.scale.iter_mut().enumerate() {
        *value = a.scale[i] + (b.scale[i] - a.scale[i]) * t;
    }
    let mut q = [0.0; 4];
    for (i, value) in q.iter_mut().enumerate() {
        *value = a.rotation_xyzw[i] + (b.rotation_xyzw[i] - a.rotation_xyzw[i]) * t;
    }
    let n = (q.iter().map(|v| v * v).sum::<f32>()).sqrt();
    out.rotation_xyzw = if n > f32::EPSILON {
        q.map(|v| v / n)
    } else {
        [0.0, 0.0, 0.0, 1.0]
    };
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn track_interpolates() {
        let t = AnimationTrack {
            bone: BoneId(0),
            keys: vec![
                Keyframe {
                    time_seconds: 0.0,
                    value: PoseTransform::default(),
                },
                Keyframe {
                    time_seconds: 1.0,
                    value: PoseTransform {
                        translation: [10.0, 0.0, 0.0],
                        ..Default::default()
                    },
                },
            ],
        };
        assert_eq!(t.sample(0.5).translation[0], 5.0);
    }
    #[test]
    fn skeleton_validates() {
        let mut s = Skeleton::default();
        assert!(s.add_bone(Bone {
            id: BoneId(0),
            parent: None,
            bind: Default::default()
        }));
        assert!(s.add_bone(Bone {
            id: BoneId(1),
            parent: Some(BoneId(0)),
            bind: Default::default()
        }));
        assert!(s.validate());
    }
}

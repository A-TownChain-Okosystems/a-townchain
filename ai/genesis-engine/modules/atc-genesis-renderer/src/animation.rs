use atc_genesis_animation::{AnimationClip, AnimationPlayer, BoneId, PoseTransform, Skeleton};
use atc_genesis_platform::Transform;

#[derive(Clone, Debug, PartialEq)]
pub struct SkinnedPose {
    pub bones: Vec<(BoneId, Transform)>,
}

impl SkinnedPose {
    pub fn from_animation(
        skeleton: &Skeleton,
        clip: &AnimationClip,
        player: &AnimationPlayer,
    ) -> Self {
        let sampled = clip.sample(player.time_seconds);
        let bones = skeleton
            .bones
            .iter()
            .map(|bone| {
                let pose = sampled
                    .iter()
                    .find(|(id, _)| *id == bone.id)
                    .map(|(_, p)| *p)
                    .unwrap_or(bone.bind);
                (bone.id, pose_to_transform(pose))
            })
            .collect();
        Self { bones }
    }

    pub fn validate_against(&self, skeleton: &Skeleton) -> bool {
        self.bones.len() == skeleton.bones.len()
            && skeleton
                .bones
                .iter()
                .all(|b| self.bones.iter().any(|(id, _)| *id == b.id))
    }
}

fn pose_to_transform(p: PoseTransform) -> Transform {
    Transform {
        translation: p.translation,
        rotation_xyzw: p.rotation_xyzw,
        scale: p.scale,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atc_genesis_animation::{AnimationClipId, AnimationTrack, Bone, Keyframe};
    #[test]
    fn pose_uses_bind_for_unanimated_bones() {
        let mut skeleton = Skeleton::default();
        skeleton.add_bone(Bone {
            id: BoneId(0),
            parent: None,
            bind: PoseTransform {
                translation: [2.0, 0.0, 0.0],
                ..Default::default()
            },
        });
        let clip = AnimationClip {
            id: AnimationClipId(1),
            duration_seconds: 1.0,
            tracks: vec![],
        };
        let pose = SkinnedPose::from_animation(&skeleton, &clip, &AnimationPlayer::default());
        assert_eq!(pose.bones[0].1.translation[0], 2.0);
        assert!(pose.validate_against(&skeleton));
    }
    #[test]
    fn pose_samples_animation() {
        let mut skeleton = Skeleton::default();
        skeleton.add_bone(Bone {
            id: BoneId(0),
            parent: None,
            bind: Default::default(),
        });
        let clip = AnimationClip {
            id: AnimationClipId(1),
            duration_seconds: 1.0,
            tracks: vec![AnimationTrack {
                bone: BoneId(0),
                keys: vec![
                    Keyframe {
                        time_seconds: 0.0,
                        value: Default::default(),
                    },
                    Keyframe {
                        time_seconds: 1.0,
                        value: PoseTransform {
                            translation: [4.0, 0.0, 0.0],
                            ..Default::default()
                        },
                    },
                ],
            }],
        };
        let player = AnimationPlayer {
            clip: Some(AnimationClipId(1)),
            time_seconds: 0.5,
            speed: 1.0,
            looping: true,
        };
        let pose = SkinnedPose::from_animation(&skeleton, &clip, &player);
        assert_eq!(pose.bones[0].1.translation[0], 2.0);
    }
}

//! Audio/video synchronization policy with bounded resynchronization.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncAction {
    Present,
    Hold,
    Drop,
    Resync,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvSyncPolicy {
    pub hold_threshold_us: u64,
    pub drop_threshold_us: u64,
    pub resync_threshold_us: u64,
}

impl Default for AvSyncPolicy {
    fn default() -> Self {
        Self {
            hold_threshold_us: 20_000,
            drop_threshold_us: 80_000,
            resync_threshold_us: 250_000,
        }
    }
}

impl AvSyncPolicy {
    pub fn action(&self, audio_pts_us: u64, video_pts_us: u64) -> SyncAction {
        let delta = video_pts_us as i128 - audio_pts_us as i128;
        let magnitude = delta.unsigned_abs() as u64;
        if magnitude >= self.resync_threshold_us {
            SyncAction::Resync
        } else if delta > self.hold_threshold_us as i128 {
            SyncAction::Hold
        } else if delta < -(self.drop_threshold_us as i128) {
            SyncAction::Drop
        } else {
            SyncAction::Present
        }
    }

    pub fn is_late(&self, audio_pts_us: u64, video_pts_us: u64) -> bool {
        self.action(audio_pts_us, video_pts_us) == SyncAction::Drop
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyncDecision {
    pub audio_pts_us: u64,
    pub video_pts_us: u64,
    pub action: SyncAction,
}

impl AvSyncPolicy {
    pub fn decide(&self, audio_pts_us: u64, video_pts_us: u64) -> SyncDecision {
        SyncDecision {
            audio_pts_us,
            video_pts_us,
            action: self.action(audio_pts_us, video_pts_us),
        }
    }
}

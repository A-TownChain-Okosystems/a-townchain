//! Audio/video synchronization policy.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncAction { Present, Hold, Drop, Resync }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyncPolicy { pub hold_us: u64, pub drop_us: u64, pub resync_us: u64 }

impl Default for SyncPolicy { fn default() -> Self { Self { hold_us: 20_000, drop_us: 80_000, resync_us: 250_000 } } }

impl SyncPolicy {
    pub fn action(&self, media_pts_us: u64, master_us: u64) -> SyncAction {
        let delta = media_pts_us as i128 - master_us as i128;
        let abs = delta.unsigned_abs() as u64;
        if abs >= self.resync_us { SyncAction::Resync }
        else if delta > self.hold_us as i128 { SyncAction::Hold }
        else if delta < -(self.drop_us as i128) { SyncAction::Drop }
        else { SyncAction::Present }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameTiming { pub pts_us: u64, pub presented_us: u64, pub action: SyncAction }

//! Timed codec playback session: packet timestamps -> frame queue -> sync decision.

use crate::{
    AvSyncPolicy, FrameQueue, MediaError, SyncAction, TimedVideoFrame, VideoCodecDecoder,
    VideoFrame,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PlaybackStats {
    pub submitted: u64,
    pub decoded: u64,
    pub presented: u64,
    pub dropped: u64,
    pub held: u64,
    pub resyncs: u64,
}

#[derive(Debug)]
pub struct CodecVideoSession<D> {
    decoder: D,
    queue: FrameQueue,
    sync: AvSyncPolicy,
    stats: PlaybackStats,
    next_audio_pts_us: u64,
}

impl<D: VideoCodecDecoder> CodecVideoSession<D> {
    pub fn new(decoder: D, queue_capacity: usize, sync: AvSyncPolicy) -> Self {
        Self {
            decoder,
            queue: FrameQueue::new(queue_capacity),
            sync,
            stats: PlaybackStats::default(),
            next_audio_pts_us: 0,
        }
    }

    pub fn stats(&self) -> PlaybackStats {
        self.stats
    }
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }
    pub fn set_audio_clock(&mut self, pts_us: u64) {
        self.next_audio_pts_us = pts_us;
    }

    pub fn submit_packet(
        &mut self,
        packet: &[u8],
        pts_us: u64,
        dts_us: u64,
        keyframe: bool,
    ) -> Result<(), MediaError> {
        self.decoder.submit(packet, pts_us, dts_us, keyframe)?;
        self.stats.submitted = self.stats.submitted.saturating_add(1);
        Ok(())
    }

    pub fn drain_decoder(&mut self) -> Result<usize, MediaError> {
        let mut count = 0;
        while let Some(TimedVideoFrame { frame, pts_us, .. }) = self.decoder.receive()? {
            self.queue.push(pts_us, frame);
            self.stats.decoded = self.stats.decoded.saturating_add(1);
            count += 1;
        }
        Ok(count)
    }

    pub fn next_for_presentation(&mut self) -> Option<(u64, VideoFrame)> {
        let pts = self.queue.peek_pts()?;
        match self.sync.action(self.next_audio_pts_us, pts) {
            SyncAction::Present => {
                let frame = self.queue.pop()?.1;
                self.stats.presented = self.stats.presented.saturating_add(1);
                Some((pts, frame))
            }
            SyncAction::Hold => {
                self.stats.held = self.stats.held.saturating_add(1);
                None
            }
            SyncAction::Drop => {
                let _ = self.queue.pop();
                self.stats.dropped = self.stats.dropped.saturating_add(1);
                None
            }
            SyncAction::Resync => {
                let _ = self.queue.pop();
                self.stats.resyncs = self.stats.resyncs.saturating_add(1);
                None
            }
        }
    }

    pub fn drop_late(&mut self, threshold_us: u64) -> usize {
        let dropped = self.queue.drop_late(self.next_audio_pts_us, threshold_us);
        self.stats.dropped = self.stats.dropped.saturating_add(dropped as u64);
        dropped
    }
}

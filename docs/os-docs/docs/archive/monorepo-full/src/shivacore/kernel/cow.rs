// ShivaCore — K-Sprint 45: Copy-on-Write Fork Engine
// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//
// Dedizierter Copy-on-Write Fork Engine — baut auf K44 (vmm.rs) auf.
// Kompletter CoW-Lifecycle: Page-Sharing-Map, KSM-Dedup, Container-CoW,
// Performance-Tracking, Lazy TLB Flush, Batch-Operations.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use spin::Mutex;

// ════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ════════════════════════════════════════════════════════════════════════════

pub const PAGE_SIZE: u64 = 4096;
pub const MAX_SHARERS: usize = 256;
pub const MAX_COW_REGIONS: usize = 1024;
pub const KSM_SCAN_INTERVAL: u64 = 100;
pub const KSM_MAX_PAGES_PER_SCAN: usize = 64;
pub const TLB_FLUSH_BATCH_SIZE: usize = 64;
pub const COW_BREAK_BATCH_SIZE: usize = 32;

// ════════════════════════════════════════════════════════════════════════════
// PAGE STATE
// ════════════════════════════════════════════════════════════════════════════

pub type FrameNumber = u64;
pub type VirtPageNumber = u64;
pub type Pid = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PagePerms {
    pub read: bool,
    pub write: bool,
    pub exec: bool,
}

impl Default for PagePerms {
    fn default() -> Self { Self { read: true, write: true, exec: false } }
}

impl PagePerms {
    pub fn ro() -> Self { Self { read: true, write: false, exec: false } }
    pub fn rw() -> Self { Self { read: true, write: true, exec: false } }
    pub fn rx() -> Self { Self { read: true, write: false, exec: true } }
    pub fn rwx() -> Self { Self { read: true, write: true, exec: true } }
}

// ════════════════════════════════════════════════════════════════════════════
// COW PAGE — per physical frame sharing state
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct CowPage {
    pub frame: FrameNumber,
    pub ref_count: u32,
    pub sharers: Vec<(Pid, VirtPageNumber)>,
    pub origin_pid: Pid,
    pub perms: PagePerms,
    pub generation: u64,
    pub ksm_candidate: bool,
    pub content_hash: u64,
}

impl CowPage {
    pub fn new(frame: FrameNumber, pid: Pid, vpage: VirtPageNumber, perms: PagePerms) -> Self {
        Self { frame, ref_count: 1, sharers: vec![(pid, vpage)], origin_pid: pid, perms,
               generation: 0, ksm_candidate: false, content_hash: 0 }
    }

    pub fn add_sharer(&mut self, pid: Pid, vpage: VirtPageNumber) -> Result<(), CowError> {
        if self.ref_count >= MAX_SHARERS as u32 { return Err(CowError::MaxSharersExceeded); }
        if self.sharers.iter().any(|(p, v)| *p == pid && *v == vpage) { return Err(CowError::AlreadyShared); }
        self.sharers.push((pid, vpage));
        self.ref_count += 1;
        Ok(())
    }

    pub fn remove_sharer(&mut self, pid: Pid, vpage: VirtPageNumber) -> bool {
        if let Some(idx) = self.sharers.iter().position(|(p, v)| *p == pid && *v == vpage) {
            self.sharers.swap_remove(idx);
            self.ref_count = self.ref_count.saturating_sub(1);
            true
        } else { false }
    }

    pub fn is_shared(&self) -> bool { self.ref_count > 1 }
    pub fn is_unique(&self) -> bool { self.ref_count == 1 }
    pub fn sharer_pids(&self) -> Vec<Pid> {
        let mut pids: Vec<Pid> = self.sharers.iter().map(|(p, _)| *p).collect();
        pids.sort_unstable(); pids.dedup(); pids
    }
    pub fn bump_generation(&mut self) { self.generation += 1; }
}

// ════════════════════════════════════════════════════════════════════════════
// COW REGION — contiguous range of CoW pages
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct CowRegion {
    pub region_id: u32,
    pub parent_pid: Pid,
    pub child_pid: Pid,
    pub start_vpage: VirtPageNumber,
    pub page_count: u64,
    pub pages_copied: u64,
    pub pages_shared: u64,
    pub created_tick: u64,
    pub fully_broken: bool,
}

impl CowRegion {
    pub fn new(id: u32, parent: Pid, child: Pid, start: VirtPageNumber, count: u64, tick: u64) -> Self {
        Self { region_id: id, parent_pid: parent, child_pid: child, start_vpage: start,
               page_count: count, pages_copied: 0, pages_shared: count,
               created_tick: tick, fully_broken: false }
    }

    pub fn break_page(&mut self) {
        self.pages_copied += 1;
        self.pages_shared = self.pages_shared.saturating_sub(1);
        if self.pages_shared == 0 { self.fully_broken = true; }
    }

    pub fn progress(&self) -> u8 {
        if self.page_count == 0 { return 100; }
        ((self.pages_copied * 100) / self.page_count) as u8
    }
}

// ════════════════════════════════════════════════════════════════════════════
// PAGE MAPPING & PROCESS PAGE TABLE
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct PageMapping {
    pub vpage: VirtPageNumber,
    pub frame: FrameNumber,
    pub perms: PagePerms,
    pub is_cow: bool,
    pub cow_region: Option<u32>,
    pub dirty: bool,
    pub accessed: bool,
}

impl PageMapping {
    pub fn new(vpage: VirtPageNumber, frame: FrameNumber, perms: PagePerms) -> Self {
        Self { vpage, frame, perms, is_cow: false, cow_region: None, dirty: false, accessed: false }
    }
    pub fn make_cow(&mut self, region: u32) { self.is_cow = true; self.cow_region = Some(region); self.perms.write = false; }
    pub fn break_cow(&mut self, new_frame: FrameNumber) {
        self.is_cow = false; self.cow_region = None; self.frame = new_frame;
        self.perms.write = true; self.dirty = true;
    }
}

#[derive(Clone, Debug)]
pub struct ProcessPageTable {
    pub pid: Pid,
    pub mappings: BTreeMap<VirtPageNumber, PageMapping>,
    pub parent_pid: Option<Pid>,
    pub children: Vec<Pid>,
    pub fork_tick: u64,
}

impl ProcessPageTable {
    pub fn new(pid: Pid) -> Self {
        Self { pid, mappings: BTreeMap::new(), parent_pid: None, children: Vec::new(), fork_tick: 0 }
    }
    pub fn map_page(&mut self, vpage: VirtPageNumber, frame: FrameNumber, perms: PagePerms) {
        self.mappings.insert(vpage, PageMapping::new(vpage, frame, perms));
    }
    pub fn unmap_page(&mut self, vpage: VirtPageNumber) -> Option<PageMapping> { self.mappings.remove(&vpage) }
    pub fn get_mapping(&self, vpage: VirtPageNumber) -> Option<&PageMapping> { self.mappings.get(&vpage) }
    pub fn get_mapping_mut(&mut self, vpage: VirtPageNumber) -> Option<&mut PageMapping> { self.mappings.get_mut(&vpage) }
    pub fn page_count(&self) -> usize { self.mappings.len() }
    pub fn cow_pages(&self) -> usize { self.mappings.values().filter(|m| m.is_cow).count() }
    pub fn writable_pages(&self) -> usize { self.mappings.values().filter(|m| m.perms.write && !m.is_cow).count() }
    pub fn dirty_pages(&self) -> usize { self.mappings.values().filter(|m| m.dirty).count() }
}

// ════════════════════════════════════════════════════════════════════════════
// TLB FLUSH QUEUE
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Default)]
pub struct TlbFlushQueue {
    pub pending: Vec<(Pid, VirtPageNumber)>,
    pub flushed: u64,
    pub batched: u64,
}

impl TlbFlushQueue {
    pub fn new() -> Self { Self::default() }
    pub fn schedule(&mut self, pid: Pid, vpage: VirtPageNumber) {
        if self.pending.len() < TLB_FLUSH_BATCH_SIZE { self.pending.push((pid, vpage)); }
    }
    pub fn flush_batch(&mut self) -> usize {
        let count = self.pending.len();
        self.pending.clear();
        self.flushed += count as u64;
        if count > 1 { self.batched += 1; }
        count
    }
    pub fn pending_count(&self) -> usize { self.pending.len() }
}

// ════════════════════════════════════════════════════════════════════════════
// KSM — Kernel Same-Page Merging
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct KsmScan {
    pub scan_id: u64,
    pub pages_scanned: u64,
    pub pages_merged: u64,
    pub pages_deduped: u64,
    pub hash_collisions: u64,
    pub last_scan_tick: u64,
    pub enabled: bool,
}

impl KsmScan {
    pub fn new() -> Self {
        Self { scan_id: 0, pages_scanned: 0, pages_merged: 0, pages_deduped: 0,
               hash_collisions: 0, last_scan_tick: 0, enabled: false }
    }
    pub fn start_scan(&mut self, tick: u64) {
        self.scan_id += 1; self.pages_scanned = 0; self.pages_merged = 0; self.last_scan_tick = tick;
    }
    pub fn merge_page(&mut self) { self.pages_merged += 1; self.pages_deduped += 1; }
    pub fn collision(&mut self) { self.hash_collisions += 1; }
}

// ════════════════════════════════════════════════════════════════════════════
// STATS & AUDIT
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Default)]
pub struct CowStats {
    pub total_forks: u64,
    pub total_cow_pages: u64,
    pub total_cow_faults: u64,
    pub total_cow_breaks: u64,
    pub total_pages_copied: u64,
    pub total_pages_shared: u64,
    pub total_ksm_merges: u64,
    pub total_tlb_flushes: u64,
    pub total_tlb_batched: u64,
    pub total_regions_created: u64,
    pub total_regions_broken: u64,
    pub avg_pages_per_fork: f64,
    pub avg_faults_per_fork: f64,
    pub avg_cow_break_time_us: u64,
    pub max_sharers_seen: u32,
}

impl CowStats {
    pub fn new() -> Self { Self::default() }
    pub fn record_fork(&mut self, pages: u64) {
        self.total_forks += 1; self.total_cow_pages += pages;
        if self.total_forks > 0 { self.avg_pages_per_fork = self.total_cow_pages as f64 / self.total_forks as f64; }
    }
    pub fn record_fault(&mut self) {
        self.total_cow_faults += 1;
        if self.total_forks > 0 { self.avg_faults_per_fork = self.total_cow_faults as f64 / self.total_forks as f64; }
    }
    pub fn record_break(&mut self) { self.total_cow_breaks += 1; self.total_pages_copied += 1; }
    pub fn record_ksm_merge(&mut self) { self.total_ksm_merges += 1; }
    pub fn record_tlb_flush(&mut self, batched: bool) {
        self.total_tlb_flushes += 1; if batched { self.total_tlb_batched += 1; }
    }
}

#[derive(Clone, Debug)]
pub struct CowAuditEntry {
    pub tick: u64,
    pub event: CowAuditEvent,
    pub pid: Pid,
    pub vpage: Option<VirtPageNumber>,
    pub frame: Option<FrameNumber>,
    pub detail: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CowAuditEvent {
    ForkInitiated, ForkCompleted, CowFault, CowBreak,
    PageShared, PageUnshared, KsmMerged, KsmScanStart, KsmScanEnd,
    TlbFlush, RegionCreated, RegionBroken, ProcessExit,
}

impl CowAuditEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            CowAuditEvent::ForkInitiated => "fork_start", CowAuditEvent::ForkCompleted => "fork_done",
            CowAuditEvent::CowFault => "cow_fault", CowAuditEvent::CowBreak => "cow_break",
            CowAuditEvent::PageShared => "page_shared", CowAuditEvent::PageUnshared => "page_unshared",
            CowAuditEvent::KsmMerged => "ksm_merged", CowAuditEvent::KsmScanStart => "ksm_scan_start",
            CowAuditEvent::KsmScanEnd => "ksm_scan_end", CowAuditEvent::TlbFlush => "tlb_flush",
            CowAuditEvent::RegionCreated => "region_created", CowAuditEvent::RegionBroken => "region_broken",
            CowAuditEvent::ProcessExit => "process_exit",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// ERROR & RESULT TYPES
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CowError {
    ProcessNotFound, PageNotMapped, PageNotCow, AlreadyShared,
    MaxSharersExceeded, MaxRegionsExceeded, FrameAllocFailed,
    InvalidRegion, KsmDisabled, NotChildProcess, RegionNotBroken,
    InvalidPerms, ContainerNotFound,
}

impl CowError {
    pub fn as_str(&self) -> &'static str {
        match self {
            CowError::ProcessNotFound => "process_not_found", CowError::PageNotMapped => "page_not_mapped",
            CowError::PageNotCow => "page_not_cow", CowError::AlreadyShared => "already_shared",
            CowError::MaxSharersExceeded => "max_sharers_exceeded", CowError::MaxRegionsExceeded => "max_regions_exceeded",
            CowError::FrameAllocFailed => "frame_alloc_failed", CowError::InvalidRegion => "invalid_region",
            CowError::KsmDisabled => "ksm_disabled", CowError::NotChildProcess => "not_child_process",
            CowError::RegionNotBroken => "region_not_broken", CowError::InvalidPerms => "invalid_perms",
            CowError::ContainerNotFound => "container_not_found",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ForkResult {
    pub child_pid: Pid,
    pub region_id: u32,
    pub total_pages: u64,
    pub cow_pages: u64,
    pub shared_pages: u64,
    pub copied_pages: u64,
    pub fork_time_us: u64,
    pub memory_saved: u64,
}

impl ForkResult {
    pub fn success(&self) -> bool { self.total_pages > 0 }
    pub fn efficiency(&self) -> f64 {
        if self.total_pages == 0 { 1.0 } else { self.cow_pages as f64 / self.total_pages as f64 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FaultResolution { CowBroken, NotCow, PageNotPresent, Oom, Success }

#[derive(Clone, Debug)]
pub struct ContainerCowState {
    pub container_id: u32,
    pub base_pid: Pid,
    pub forked_pids: Vec<Pid>,
    pub total_shared: u64,
    pub total_copied: u64,
}

impl ContainerCowState {
    pub fn new(container_id: u32, base_pid: Pid) -> Self {
        Self { container_id, base_pid, forked_pids: Vec::new(), total_shared: 0, total_copied: 0 }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// PAGE INFO & SNAPSHOT
// ════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct PageInfo {
    pub vpage: VirtPageNumber,
    pub frame: FrameNumber,
    pub perms: PagePerms,
    pub is_cow: bool,
    pub ref_count: u32,
    pub sharer_count: usize,
    pub cow_region: Option<u32>,
    pub dirty: bool,
    pub accessed: bool,
}

#[derive(Clone, Debug)]
pub struct CowSnapshot {
    pub tick: u64, pub processes: usize, pub total_pages: usize, pub cow_pages: usize,
    pub shared_frames: usize, pub active_regions: usize, pub broken_regions: usize,
    pub ksm_enabled: bool, pub ksm_merges: u64, pub tlb_pending: usize,
    pub tlb_flushed: u64, pub containers: usize, pub audit_entries: usize,
    pub forks: u64, pub cow_faults: u64, pub cow_breaks: u64, pub ksm_total: u64,
    pub memory_saved: u64,
}

// ════════════════════════════════════════════════════════════════════════════
// COW MANAGER — the engine
// ════════════════════════════════════════════════════════════════════════════

pub struct CowManager {
    processes: BTreeMap<Pid, ProcessPageTable>,
    shared_pages: BTreeMap<FrameNumber, CowPage>,
    regions: BTreeMap<u32, CowRegion>,
    next_region_id: u32,
    tlb_queue: TlbFlushQueue,
    ksm: KsmScan,
    stats: CowStats,
    audit: Vec<CowAuditEntry>,
    containers: BTreeMap<u32, ContainerCowState>,
    tick_count: u64,
    next_frame: FrameNumber,
}

impl CowManager {
    pub fn new() -> Self {
        Self { processes: BTreeMap::new(), shared_pages: BTreeMap::new(), regions: BTreeMap::new(),
               next_region_id: 1, tlb_queue: TlbFlushQueue::new(), ksm: KsmScan::new(),
               stats: CowStats::new(), audit: Vec::new(), containers: BTreeMap::new(),
               tick_count: 0, next_frame: 0x100000 }
    }

    fn tick(&mut self) { self.tick_count += 1; }
    fn alloc_frame(&mut self) -> Option<FrameNumber> { let f = self.next_frame; self.next_frame += 1; Some(f) }

    fn audit_log(&mut self, event: CowAuditEvent, pid: Pid, vpage: Option<VirtPageNumber>,
                 frame: Option<FrameNumber>, detail: &str) {
        if self.audit.len() < 4096 {
            self.audit.push(CowAuditEntry { tick: self.tick_count, event, pid, vpage, frame,
                                           detail: detail.to_string() });
        }
    }

    // ── Process Management ────────────────────────────────────

    pub fn register_process(&mut self, pid: Pid) { self.processes.insert(pid, ProcessPageTable::new(pid)); }

    pub fn unregister_process(&mut self, pid: Pid) {
        let frames_to_clean: Vec<FrameNumber> = self.shared_pages.iter()
            .filter(|(_, cp)| cp.sharers.iter().any(|(p, _)| *p == pid)).map(|(f, _)| *f).collect();
        for frame in frames_to_clean {
            if let Some(cp) = self.shared_pages.get_mut(&frame) {
                let vpages: Vec<VirtPageNumber> = cp.sharers.iter()
                    .filter(|(p, _)| *p == pid).map(|(_, v)| *v).collect();
                for vp in vpages { cp.remove_sharer(pid, vp); }
                if cp.is_unique() { self.shared_pages.remove(&frame); }
            }
        }
        self.processes.remove(&pid);
        self.audit_log(CowAuditEvent::ProcessExit, pid, None, None, "process exit cleanup");
    }

    pub fn process_exists(&self, pid: Pid) -> bool { self.processes.contains_key(&pid) }
    pub fn process_page_count(&self, pid: Pid) -> usize { self.processes.get(&pid).map(|p| p.page_count()).unwrap_or(0) }
    pub fn process_cow_pages(&self, pid: Pid) -> usize { self.processes.get(&pid).map(|p| p.cow_pages()).unwrap_or(0) }
    pub fn process_dirty_pages(&self, pid: Pid) -> usize { self.processes.get(&pid).map(|p| p.dirty_pages()).unwrap_or(0) }

    // ── Page Mapping ──────────────────────────────────────────

    pub fn map_page(&mut self, pid: Pid, vpage: VirtPageNumber, perms: PagePerms) -> Result<FrameNumber, CowError> {
        let pt = self.processes.get_mut(&pid).ok_or(CowError::ProcessNotFound)?;
        let frame = self.alloc_frame().ok_or(CowError::FrameAllocFailed)?;
        pt.map_page(vpage, frame, perms);
        self.shared_pages.insert(frame, CowPage::new(frame, pid, vpage, perms));
        self.tick();
        Ok(frame)
    }

    pub fn unmap_page(&mut self, pid: Pid, vpage: VirtPageNumber) -> Result<(), CowError> {
        let pt = self.processes.get_mut(&pid).ok_or(CowError::ProcessNotFound)?;
        let mapping = pt.unmap_page(vpage).ok_or(CowError::PageNotMapped)?;
        if let Some(cp) = self.shared_pages.get_mut(&mapping.frame) {
            cp.remove_sharer(pid, vpage);
            if cp.is_unique() { self.shared_pages.remove(&mapping.frame); }
        }
        self.tlb_queue.schedule(pid, vpage);
        self.audit_log(CowAuditEvent::PageUnshared, pid, Some(vpage), Some(mapping.frame), "unmap");
        self.tick();
        Ok(())
    }

    pub fn map_range(&mut self, pid: Pid, start_vpage: VirtPageNumber, count: u64,
                     perms: PagePerms) -> Result<Vec<FrameNumber>, CowError> {
        let mut frames = Vec::with_capacity(count as usize);
        for i in 0..count { frames.push(self.map_page(pid, start_vpage + i, perms)?); }
        Ok(frames)
    }

    // ── Copy-on-Write Fork ────────────────────────────────────

    pub fn fork(&mut self, parent_pid: Pid, child_pid: Pid) -> Result<ForkResult, CowError> {
        self.tick();
        let parent = self.processes.get(&parent_pid).ok_or(CowError::ProcessNotFound)?;
        if self.processes.contains_key(&child_pid) { return Err(CowError::AlreadyShared); }

        let total_pages = parent.page_count() as u64;
        let writable_pages = parent.writable_pages() as u64;
        let read_only_pages = total_pages - writable_pages;

        self.audit_log(CowAuditEvent::ForkInitiated, parent_pid, None, None,
                       &format!("forking to child {} ({} pages, {} writable)", child_pid, total_pages, writable_pages));

        let mut child_pt = ProcessPageTable::new(child_pid);
        child_pt.parent_pid = Some(parent_pid);
        child_pt.fork_tick = self.tick_count;

        let region_id = if writable_pages > 0 {
            let rid = self.next_region_id; self.next_region_id += 1;
            if self.regions.len() >= MAX_COW_REGIONS { return Err(CowError::MaxRegionsExceeded); }
            self.regions.insert(rid, CowRegion::new(rid, parent_pid, child_pid, 0, writable_pages, self.tick_count));
            self.stats.total_regions_created += 1;
            self.audit_log(CowAuditEvent::RegionCreated, parent_pid, None, None, &format!("region {} for child {}", rid, child_pid));
            Some(rid)
        } else { None };

        let mut cow_page_count = 0u64;
        let mut shared_ro_count = 0u64;
        let parent_mappings: Vec<(VirtPageNumber, PageMapping)> = parent.mappings.iter().map(|(k, v)| (*k, v.clone())).collect();
        let mut parent_cow_updates: Vec<(VirtPageNumber, u32)> = Vec::new();

        for (vpage, mapping) in &parent_mappings {
            let frame = mapping.frame;
            if mapping.perms.write && !mapping.is_cow {
                let mut child_mapping = mapping.clone();
                child_mapping.make_cow(region_id.unwrap());
                child_mapping.dirty = false; child_mapping.accessed = false;
                child_pt.mappings.insert(*vpage, child_mapping);
                parent_cow_updates.push((*vpage, region_id.unwrap()));
                if let Some(cp) = self.shared_pages.get_mut(&frame) {
                    cp.add_sharer(child_pid, *vpage).ok(); cp.bump_generation();
                }
                cow_page_count += 1;
                self.tlb_queue.schedule(parent_pid, *vpage);
                self.tlb_queue.schedule(child_pid, *vpage);
                self.audit_log(CowAuditEvent::PageShared, parent_pid, Some(*vpage), Some(frame), &format!("cow shared with child {}", child_pid));
            } else {
                let mut child_mapping = mapping.clone();
                child_mapping.accessed = false;
                child_pt.mappings.insert(*vpage, child_mapping);
                if let Some(cp) = self.shared_pages.get_mut(&frame) { cp.add_sharer(child_pid, *vpage).ok(); }
                shared_ro_count += 1;
            }
        }

        if let Some(parent) = self.processes.get_mut(&parent_pid) {
            for (vpage, rid) in parent_cow_updates {
                if let Some(m) = parent.get_mapping_mut(vpage) { m.make_cow(rid); }
            }
            parent.children.push(child_pid);
        }
        self.processes.insert(child_pid, child_pt);

        let flushed = self.tlb_queue.flush_batch();
        self.stats.record_tlb_flush(flushed > 1);
        self.audit_log(CowAuditEvent::TlbFlush, parent_pid, None, None, &format!("flushed {} entries", flushed));

        self.stats.record_fork(cow_page_count);
        self.stats.total_pages_shared += cow_page_count + shared_ro_count;
        for (_, cp) in &self.shared_pages {
            if cp.ref_count > self.stats.max_sharers_seen { self.stats.max_sharers_seen = cp.ref_count; }
        }

        let result = ForkResult { child_pid, region_id: region_id.unwrap_or(0), total_pages,
            cow_pages: cow_page_count, shared_pages: cow_page_count + shared_ro_count,
            copied_pages: 0, fork_time_us: 1, memory_saved: cow_page_count * PAGE_SIZE };

        self.audit_log(CowAuditEvent::ForkCompleted, parent_pid, None, None,
                       &format!("fork complete: {} cow, {} shared, {} saved", cow_page_count, shared_ro_count, result.memory_saved));
        self.tick();
        Ok(result)
    }

    pub fn fork_into_container(&mut self, parent_pid: Pid, child_pid: Pid, container_id: u32) -> Result<ForkResult, CowError> {
        let result = self.fork(parent_pid, child_pid)?;
        if !self.containers.contains_key(&container_id) {
            self.containers.insert(container_id, ContainerCowState::new(container_id, child_pid));
        }
        let cs = self.containers.get_mut(&container_id).unwrap();
        cs.forked_pids.push(child_pid);
        cs.total_shared += result.shared_pages;
        Ok(result)
    }

    // ── CoW Fault Handling ────────────────────────────────────

    pub fn handle_cow_fault(&mut self, pid: Pid, vpage: VirtPageNumber) -> FaultResolution {
        self.tick();
        let pt = match self.processes.get_mut(&pid) { Some(p) => p, None => return FaultResolution::PageNotPresent };
        let mapping = match pt.get_mapping(vpage) { Some(m) => m.clone(), None => return FaultResolution::PageNotPresent };
        if !mapping.is_cow { return FaultResolution::NotCow; }

        let old_frame = mapping.frame;
        let region_id = mapping.cow_region;
        let new_frame = match self.alloc_frame() { Some(f) => f, None => return FaultResolution::Oom };

        if let Some(m) = pt.get_mapping_mut(vpage) { m.break_cow(new_frame); }

        if let Some(cp) = self.shared_pages.get_mut(&old_frame) {
            cp.remove_sharer(pid, vpage); cp.bump_generation();
            if cp.is_unique() {
                let remaining_pid = cp.origin_pid;
                let remaining_vpage = cp.sharers.first().map(|(_, v)| *v);
                if let (Some(pt2), Some(vp)) = (self.processes.get_mut(&remaining_pid), remaining_vpage) {
                    if let Some(m) = pt2.get_mapping_mut(vp) { m.break_cow(old_frame); }
                }
                self.shared_pages.remove(&old_frame);
            }
        }

        let perms = mapping.perms;
        self.shared_pages.insert(new_frame, CowPage::new(new_frame, pid, vpage, perms));

        if let Some(rid) = region_id {
            if let Some(region) = self.regions.get_mut(&rid) {
                region.break_page();
                if region.fully_broken {
                    self.stats.total_regions_broken += 1;
                    self.audit_log(CowAuditEvent::RegionBroken, pid, Some(vpage), Some(new_frame), &format!("region {} fully broken", rid));
                }
            }
        }

        self.tlb_queue.schedule(pid, vpage);
        if self.tlb_queue.pending_count() >= TLB_FLUSH_BATCH_SIZE {
            let flushed = self.tlb_queue.flush_batch();
            self.stats.record_tlb_flush(flushed > 1);
        }

        self.stats.record_fault();
        self.stats.record_break();
        self.audit_log(CowAuditEvent::CowFault, pid, Some(vpage), Some(new_frame), &format!("cow broken: old={}, new={}", old_frame, new_frame));
        self.tick();
        FaultResolution::CowBroken
    }

    pub fn break_cow_page(&mut self, pid: Pid, vpage: VirtPageNumber) -> Result<FrameNumber, CowError> {
        let resolution = self.handle_cow_fault(pid, vpage);
        match resolution {
            FaultResolution::CowBroken => {
                let pt = self.processes.get(&pid).ok_or(CowError::ProcessNotFound)?;
                let mapping = pt.get_mapping(vpage).ok_or(CowError::PageNotMapped)?;
                Ok(mapping.frame)
            }
            FaultResolution::NotCow => Err(CowError::PageNotCow),
            FaultResolution::PageNotPresent => Err(CowError::PageNotMapped),
            FaultResolution::Oom => Err(CowError::FrameAllocFailed),
            FaultResolution::Success => Err(CowError::PageNotCow),
        }
    }

    pub fn break_all_cow(&mut self, pid: Pid) -> Result<u64, CowError> {
        let pt = self.processes.get(&pid).ok_or(CowError::ProcessNotFound)?;
        let cow_vpages: Vec<VirtPageNumber> = pt.mappings.iter().filter(|(_, m)| m.is_cow).map(|(v, _)| *v).collect();
        drop(pt);
        let mut broken = 0u64;
        for vpage in cow_vpages {
            if self.handle_cow_fault(pid, vpage) == FaultResolution::CowBroken { broken += 1; }
        }
        Ok(broken)
    }

    // ── Region Management ────────────────────────────────────

    pub fn get_region(&self, region_id: u32) -> Option<&CowRegion> { self.regions.get(&region_id) }
    pub fn region_progress(&self, region_id: u32) -> Option<u8> { self.regions.get(&region_id).map(|r| r.progress()) }
    pub fn active_regions(&self) -> usize { self.regions.values().filter(|r| !r.fully_broken).count() }
    pub fn broken_regions(&self) -> usize { self.regions.values().filter(|r| r.fully_broken).count() }
    pub fn region_pages_copied(&self, region_id: u32) -> u64 { self.regions.get(&region_id).map(|r| r.pages_copied).unwrap_or(0) }
    pub fn region_pages_shared(&self, region_id: u32) -> u64 { self.regions.get(&region_id).map(|r| r.pages_shared).unwrap_or(0) }

    // ── KSM ───────────────────────────────────────────────────

    pub fn ksm_enable(&mut self) { self.ksm.enabled = true; }
    pub fn ksm_disable(&mut self) { self.ksm.enabled = false; }
    pub fn ksm_is_enabled(&self) -> bool { self.ksm.enabled }

    pub fn ksm_scan(&mut self) -> Result<u64, CowError> {
        if !self.ksm.enabled { return Err(CowError::KsmDisabled); }
        self.tick();
        self.ksm.start_scan(self.tick_count);
        self.audit_log(CowAuditEvent::KsmScanStart, 0, None, None, &format!("scan {}", self.ksm.scan_id));

        let mut hash_map: BTreeMap<u64, Vec<FrameNumber>> = BTreeMap::new();
        let frames: Vec<FrameNumber> = self.shared_pages.keys().cloned().collect();
        for frame in frames.iter().take(KSM_MAX_PAGES_PER_SCAN) {
            let cp = match self.shared_pages.get(frame) { Some(c) => c, None => continue };
            let hash = cp.content_hash;
            self.ksm.pages_scanned += 1;
            if hash > 0 { hash_map.entry(hash).or_default().push(*frame); }
        }

        let mut merged = 0u64;
        for (_, frames) in &hash_map {
            if frames.len() < 2 { continue; }
            let target = frames[0];
            for &src in &frames[1..] { if self.ksm_merge_pages(target, src) { merged += 1; } }
        }
        self.ksm.pages_merged += merged;
        self.stats.total_ksm_merges += merged;
        self.audit_log(CowAuditEvent::KsmScanEnd, 0, None, None, &format!("scan {}: {} merged", self.ksm.scan_id, merged));
        self.tick();
        Ok(merged)
    }

    fn ksm_merge_pages(&mut self, target_frame: FrameNumber, src_frame: FrameNumber) -> bool {
        let src_cp = match self.shared_pages.get(&src_frame) { Some(c) => c.clone(), None => return false };
        let sharers_to_move: Vec<(Pid, VirtPageNumber)> = src_cp.sharers.clone();
        for (pid, vpage) in &sharers_to_move {
            if let Some(pt) = self.processes.get_mut(pid) {
                if let Some(m) = pt.get_mapping_mut(*vpage) { m.frame = target_frame; m.is_cow = true; }
            }
        }
        if let Some(target) = self.shared_pages.get_mut(&target_frame) {
            for (pid, vpage) in &sharers_to_move { target.add_sharer(*pid, *vpage).ok(); }
        }
        self.shared_pages.remove(&src_frame);
        self.audit_log(CowAuditEvent::KsmMerged, 0, None, Some(target_frame), &format!("merged {} into {}", src_frame, target_frame));
        true
    }

    pub fn set_page_hash(&mut self, frame: FrameNumber, hash: u64) {
        if let Some(cp) = self.shared_pages.get_mut(&frame) { cp.content_hash = hash; cp.ksm_candidate = hash > 0; }
    }
    pub fn ksm_stats(&self) -> &KsmScan { &self.ksm }

    // ── Process Tree ──────────────────────────────────────────

    pub fn get_parent(&self, pid: Pid) -> Option<Pid> { self.processes.get(&pid).and_then(|p| p.parent_pid) }
    pub fn get_children(&self, pid: Pid) -> Vec<Pid> { self.processes.get(&pid).map(|p| p.children.clone()).unwrap_or_default() }
    pub fn is_child_of(&self, child: Pid, parent: Pid) -> bool { self.get_parent(child) == Some(parent) }

    pub fn process_tree_depth(&self, pid: Pid) -> u32 {
        let mut depth = 0; let mut current = pid;
        while let Some(parent) = self.get_parent(current) { depth += 1; current = parent; if depth > 100 { break; } }
        depth
    }

    pub fn process_descendants(&self, pid: Pid) -> Vec<Pid> {
        let mut result = Vec::new(); let mut queue = vec![pid];
        while let Some(p) = queue.pop() {
            for child in self.get_children(p) { result.push(child); queue.push(child); }
        }
        result
    }

    // ── TLB ───────────────────────────────────────────────────

    pub fn tlb_flush_pending(&self) -> usize { self.tlb_queue.pending_count() }
    pub fn tlb_flush_now(&mut self) -> usize {
        let count = self.tlb_queue.flush_batch();
        if count > 0 { self.stats.record_tlb_flush(count > 1); }
        count
    }
    pub fn tlb_stats(&self) -> (u64, u64) { (self.tlb_queue.flushed, self.tlb_queue.batched) }

    // ── Page Query ────────────────────────────────────────────

    pub fn get_page_info(&self, pid: Pid, vpage: VirtPageNumber) -> Option<PageInfo> {
        let pt = self.processes.get(&pid)?;
        let mapping = pt.get_mapping(vpage)?;
        let cp = self.shared_pages.get(&mapping.frame);
        Some(PageInfo { vpage, frame: mapping.frame, perms: mapping.perms, is_cow: mapping.is_cow,
            ref_count: cp.map(|c| c.ref_count).unwrap_or(1), sharer_count: cp.map(|c| c.sharers.len()).unwrap_or(1),
            cow_region: mapping.cow_region, dirty: mapping.dirty, accessed: mapping.accessed })
    }

    pub fn is_page_cow(&self, pid: Pid, vpage: VirtPageNumber) -> bool { self.get_page_info(pid, vpage).map(|i| i.is_cow).unwrap_or(false) }
    pub fn page_ref_count(&self, frame: FrameNumber) -> u32 { self.shared_pages.get(&frame).map(|cp| cp.ref_count).unwrap_or(0) }
    pub fn shared_page_count(&self) -> usize { self.shared_pages.len() }
    pub fn total_shared_pages(&self) -> u64 { self.shared_pages.values().filter(|cp| cp.is_shared()).count() as u64 }

    // ── Stats ─────────────────────────────────────────────────

    pub fn stats(&self) -> &CowStats { &self.stats }
    pub fn process_count(&self) -> usize { self.processes.len() }
    pub fn total_page_mappings(&self) -> usize { self.processes.values().map(|p| p.page_count()).sum() }
    pub fn total_cow_mappings(&self) -> usize { self.processes.values().map(|p| p.cow_pages()).sum() }

    // ── Audit ─────────────────────────────────────────────────

    pub fn audit_entries(&self) -> &Vec<CowAuditEntry> { &self.audit }
    pub fn audit_count(&self) -> usize { self.audit.len() }
    pub fn audit_for_process(&self, pid: Pid) -> Vec<&CowAuditEntry> { self.audit.iter().filter(|e| e.pid == pid).collect() }
    pub fn audit_by_event(&self, event: CowAuditEvent) -> Vec<&CowAuditEntry> { self.audit.iter().filter(|e| e.event == event).collect() }

    // ── Container CoW ─────────────────────────────────────────

    pub fn container_register(&mut self, container_id: u32, base_pid: Pid) { self.containers.insert(container_id, ContainerCowState::new(container_id, base_pid)); }
    pub fn container_fork(&mut self, container_id: u32, parent_pid: Pid, child_pid: Pid) -> Result<ForkResult, CowError> { self.fork_into_container(parent_pid, child_pid, container_id) }
    pub fn container_cow_stats(&self, container_id: u32) -> Option<&ContainerCowState> { self.containers.get(&container_id) }
    pub fn container_total_shared(&self, container_id: u32) -> u64 { self.containers.get(&container_id).map(|c| c.total_shared).unwrap_or(0) }
    pub fn container_total_copied(&self, container_id: u32) -> u64 { self.containers.get(&container_id).map(|c| c.total_copied).unwrap_or(0) }
    pub fn container_forked_pids(&self, container_id: u32) -> Vec<Pid> { self.containers.get(&container_id).map(|c| c.forked_pids.clone()).unwrap_or_default() }

    // ── Snapshot ──────────────────────────────────────────────

    pub fn snapshot(&self) -> CowSnapshot {
        CowSnapshot { tick: self.tick_count, processes: self.processes.len(),
            total_pages: self.total_page_mappings(), cow_pages: self.total_cow_mappings(),
            shared_frames: self.shared_pages.len(), active_regions: self.active_regions(),
            broken_regions: self.broken_regions(), ksm_enabled: self.ksm.enabled,
            ksm_merges: self.ksm.pages_merged, tlb_pending: self.tlb_queue.pending_count(),
            tlb_flushed: self.tlb_queue.flushed, containers: self.containers.len(),
            audit_entries: self.audit.len(), forks: self.stats.total_forks,
            cow_faults: self.stats.total_cow_faults, cow_breaks: self.stats.total_cow_breaks,
            ksm_total: self.stats.total_ksm_merges, memory_saved: self.stats.total_pages_shared * PAGE_SIZE }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTS
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn make_manager() -> CowManager { CowManager::new() }

    // ── Page Mapping Tests ────────────────────────────────────

    #[test] fn test_register_process() {
        let mut m = make_manager(); m.register_process(1);
        assert!(m.process_exists(1)); assert!(!m.process_exists(2));
    }

    #[test] fn test_unregister_process() {
        let mut m = make_manager(); m.register_process(1);
        m.unregister_process(1); assert!(!m.process_exists(1));
    }

    #[test] fn test_map_page() {
        let mut m = make_manager(); m.register_process(1);
        let frame = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        assert!(frame > 0); assert_eq!(m.process_page_count(1), 1);
    }

    #[test] fn test_map_page_nonexistent_process() {
        let mut m = make_manager();
        assert_eq!(m.map_page(99, 0x1000, PagePerms::rw()).unwrap_err(), CowError::ProcessNotFound);
    }

    #[test] fn test_unmap_page() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.unmap_page(1, 0x1000).unwrap();
        assert_eq!(m.process_page_count(1), 0);
    }

    #[test] fn test_unmap_page_not_mapped() {
        let mut m = make_manager(); m.register_process(1);
        assert_eq!(m.unmap_page(1, 0x1000).unwrap_err(), CowError::PageNotMapped);
    }

    #[test] fn test_map_range() {
        let mut m = make_manager(); m.register_process(1);
        let frames = m.map_range(1, 0x1000, 10, PagePerms::rw()).unwrap();
        assert_eq!(frames.len(), 10); assert_eq!(m.process_page_count(1), 10);
    }

    #[test] fn test_map_range_unique_frames() {
        let mut m = make_manager(); m.register_process(1);
        let frames = m.map_range(1, 0x1000, 5, PagePerms::rw()).unwrap();
        let mut unique = frames.clone(); unique.sort_unstable(); unique.dedup();
        assert_eq!(unique.len(), frames.len());
    }

    #[test] fn test_page_info() {
        let mut m = make_manager(); m.register_process(1);
        let frame = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        let info = m.get_page_info(1, 0x1000).unwrap();
        assert_eq!(info.frame, frame); assert!(!info.is_cow); assert_eq!(info.ref_count, 1);
    }

    #[test] fn test_is_page_cow_false() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        assert!(!m.is_page_cow(1, 0x1000));
    }

    #[test] fn test_page_ref_count() {
        let mut m = make_manager(); m.register_process(1);
        let frame = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        assert_eq!(m.page_ref_count(frame), 1);
    }

    // ── Fork Tests ────────────────────────────────────────────

    #[test] fn test_fork_basic() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x2000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x3000, PagePerms::ro()).unwrap();
        let r = m.fork(1, 2).unwrap();
        assert_eq!(r.child_pid, 2); assert_eq!(r.total_pages, 3);
        assert_eq!(r.cow_pages, 2); assert_eq!(r.shared_pages, 3);
    }

    #[test] fn test_fork_creates_child() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        assert!(m.process_exists(2)); assert_eq!(m.process_page_count(2), 1);
    }

    #[test] fn test_fork_parent_pages_become_cow() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        assert!(m.is_page_cow(1, 0x1000)); assert!(m.is_page_cow(2, 0x1000));
    }

    #[test] fn test_fork_readonly_pages_shared_not_cow() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::ro()).unwrap();
        m.fork(1, 2).unwrap();
        assert!(!m.is_page_cow(1, 0x1000)); assert!(!m.is_page_cow(2, 0x1000));
    }

    #[test] fn test_fork_executable_pages_shared() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rx()).unwrap();
        let r = m.fork(1, 2).unwrap();
        assert_eq!(r.cow_pages, 0); assert_eq!(r.shared_pages, 1);
    }

    #[test] fn test_fork_parent_not_found() {
        let mut m = make_manager();
        assert_eq!(m.fork(99, 2).unwrap_err(), CowError::ProcessNotFound);
    }

    #[test] fn test_fork_memory_saved() {
        let mut m = make_manager(); m.register_process(1);
        for i in 0..100 { m.map_page(1, 0x1000 + i, PagePerms::rw()).unwrap(); }
        let r = m.fork(1, 2).unwrap();
        assert_eq!(r.memory_saved, 100 * PAGE_SIZE);
    }

    #[test] fn test_fork_multiple_children() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); m.fork(1, 3).unwrap(); m.fork(1, 4).unwrap();
        let children = m.get_children(1);
        assert_eq!(children.len(), 3);
    }

    #[test] fn test_fork_ref_count_increases() {
        let mut m = make_manager(); m.register_process(1);
        let frame = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); assert_eq!(m.page_ref_count(frame), 2);
        m.fork(1, 3).unwrap(); assert_eq!(m.page_ref_count(frame), 3);
    }

    #[test] fn test_fork_grandchild() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); m.fork(2, 3).unwrap();
        assert!(m.is_child_of(3, 2)); assert!(!m.is_child_of(3, 1));
    }

    #[test] fn test_fork_efficiency() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x2000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x3000, PagePerms::ro()).unwrap();
        let r = m.fork(1, 2).unwrap();
        assert_eq!(r.efficiency(), 2.0 / 3.0);
    }

    #[test] fn test_fork_success() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        let r = m.fork(1, 2).unwrap();
        assert!(r.success());
    }

    // ── CoW Fault Tests ───────────────────────────────────────

    #[test] fn test_cow_fault_breaks_cow() {
        let mut m = make_manager(); m.register_process(1);
        let frame = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        assert_eq!(m.page_ref_count(frame), 2);
        let res = m.handle_cow_fault(1, 0x1000);
        assert_eq!(res, FaultResolution::CowBroken);
        assert!(!m.is_page_cow(1, 0x1000));
    }

    #[test] fn test_cow_fault_child_breaks() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        let res = m.handle_cow_fault(2, 0x1000);
        assert_eq!(res, FaultResolution::CowBroken);
        assert!(!m.is_page_cow(2, 0x1000));
        assert!(m.is_page_cow(1, 0x1000));
    }

    #[test] fn test_cow_fault_not_cow() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        assert_eq!(m.handle_cow_fault(1, 0x1000), FaultResolution::NotCow);
    }

    #[test] fn test_cow_fault_page_not_present() {
        let mut m = make_manager(); m.register_process(1);
        assert_eq!(m.handle_cow_fault(1, 0x1000), FaultResolution::PageNotPresent);
    }

    #[test] fn test_cow_fault_process_not_found() {
        let mut m = make_manager();
        assert_eq!(m.handle_cow_fault(99, 0x1000), FaultResolution::PageNotPresent);
    }

    #[test] fn test_cow_fault_new_frame_allocated() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        let before = m.get_page_info(1, 0x1000).unwrap();
        m.handle_cow_fault(1, 0x1000);
        let after = m.get_page_info(1, 0x1000).unwrap();
        assert_ne!(before.frame, after.frame);
    }

    #[test] fn test_cow_fault_both_break_when_two_sharers() {
        let mut m = make_manager(); m.register_process(1);
        let frame = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        m.handle_cow_fault(1, 0x1000);
        assert_eq!(m.page_ref_count(frame), 1);
        assert!(!m.is_page_cow(1, 0x1000));
        assert!(!m.is_page_cow(2, 0x1000));
    }

    #[test] fn test_cow_fault_three_sharers() {
        let mut m = make_manager(); m.register_process(1);
        let frame = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); m.fork(1, 3).unwrap();
        m.handle_cow_fault(1, 0x1000);
        assert_eq!(m.page_ref_count(frame), 2);
        assert!(m.is_page_cow(2, 0x1000));
        assert!(m.is_page_cow(3, 0x1000));
    }

    #[test] fn test_break_cow_page_explicit() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        let new_frame = m.break_cow_page(1, 0x1000).unwrap();
        assert!(new_frame > 0); assert!(!m.is_page_cow(1, 0x1000));
    }

    #[test] fn test_break_cow_page_not_cow() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        assert_eq!(m.break_cow_page(1, 0x1000).unwrap_err(), CowError::PageNotCow);
    }

    #[test] fn test_break_all_cow() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x2000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x3000, PagePerms::ro()).unwrap();
        m.fork(1, 2).unwrap();
        assert_eq!(m.process_cow_pages(1), 2);
        let broken = m.break_all_cow(1).unwrap();
        assert_eq!(broken, 2); assert_eq!(m.process_cow_pages(1), 0);
    }

    // ── Region Tests ──────────────────────────────────────────

    #[test] fn test_region_created_on_fork() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x2000, PagePerms::rw()).unwrap();
        let r = m.fork(1, 2).unwrap();
        let region = m.get_region(r.region_id).unwrap();
        assert_eq!(region.page_count, 2); assert_eq!(region.pages_shared, 2);
    }

    #[test] fn test_region_progress() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x2000, PagePerms::rw()).unwrap();
        let r = m.fork(1, 2).unwrap();
        assert_eq!(m.region_progress(r.region_id), Some(0));
        m.handle_cow_fault(1, 0x1000);
        assert_eq!(m.region_progress(r.region_id), Some(50));
        m.handle_cow_fault(1, 0x2000);
        assert_eq!(m.region_progress(r.region_id), Some(100));
    }

    #[test] fn test_region_fully_broken() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        let r = m.fork(1, 2).unwrap();
        m.handle_cow_fault(1, 0x1000);
        assert!(m.get_region(r.region_id).unwrap().fully_broken);
    }

    #[test] fn test_active_regions() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x2000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); m.fork(1, 3).unwrap();
        assert_eq!(m.active_regions(), 2);
    }

    #[test] fn test_broken_regions() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        let r = m.fork(1, 2).unwrap();
        m.handle_cow_fault(1, 0x1000);
        assert_eq!(m.broken_regions(), 1); assert_eq!(m.active_regions(), 0);
    }

    // ── Process Tree Tests ────────────────────────────────────

    #[test] fn test_get_parent() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        assert_eq!(m.get_parent(2), Some(1)); assert_eq!(m.get_parent(1), None);
    }

    #[test] fn test_get_children() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); m.fork(1, 3).unwrap();
        assert_eq!(m.get_children(1).len(), 2);
    }

    #[test] fn test_is_child_of() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        assert!(m.is_child_of(2, 1)); assert!(!m.is_child_of(1, 2));
    }

    #[test] fn test_process_tree_depth() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); m.fork(2, 3).unwrap(); m.fork(3, 4).unwrap();
        assert_eq!(m.process_tree_depth(4), 3); assert_eq!(m.process_tree_depth(1), 0);
    }

    #[test] fn test_process_descendants() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); m.fork(1, 3).unwrap();
        m.fork(2, 4).unwrap(); m.fork(2, 5).unwrap();
        let desc = m.process_descendants(1);
        assert_eq!(desc.len(), 4);
    }

    // ── TLB Tests ─────────────────────────────────────────────

    #[test] fn test_tlb_flush_now() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.tlb_queue.schedule(1, 0x1000); m.tlb_queue.schedule(1, 0x2000);
        assert_eq!(m.tlb_flush_now(), 2);
    }

    #[test] fn test_tlb_stats() {
        let mut m = make_manager();
        m.tlb_queue.schedule(1, 0x1000); m.tlb_queue.schedule(1, 0x2000);
        m.tlb_flush_now();
        let (flushed, batched) = m.tlb_stats();
        assert!(flushed >= 2); assert!(batched >= 1);
    }

    // ── KSM Tests ─────────────────────────────────────────────

    #[test] fn test_ksm_disabled_by_default() { assert!(!make_manager().ksm_is_enabled()); }

    #[test] fn test_ksm_enable_disable() {
        let mut m = make_manager();
        m.ksm_enable(); assert!(m.ksm_is_enabled());
        m.ksm_disable(); assert!(!m.ksm_is_enabled());
    }

    #[test] fn test_ksm_scan_disabled() {
        let mut m = make_manager();
        assert_eq!(m.ksm_scan().unwrap_err(), CowError::KsmDisabled);
    }

    #[test] fn test_ksm_scan_no_pages() {
        let mut m = make_manager(); m.ksm_enable();
        assert_eq!(m.ksm_scan().unwrap(), 0);
    }

    #[test] fn test_ksm_scan_with_identical_hashes() {
        let mut m = make_manager(); m.ksm_enable();
        m.register_process(1); m.register_process(2);
        let f1 = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        let f2 = m.map_page(2, 0x1000, PagePerms::rw()).unwrap();
        m.set_page_hash(f1, 0xDEAD); m.set_page_hash(f2, 0xDEAD);
        assert_eq!(m.ksm_scan().unwrap(), 1);
    }

    #[test] fn test_ksm_scan_different_hashes() {
        let mut m = make_manager(); m.ksm_enable();
        m.register_process(1); m.register_process(2);
        let f1 = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        let f2 = m.map_page(2, 0x1000, PagePerms::rw()).unwrap();
        m.set_page_hash(f1, 0xDEAD); m.set_page_hash(f2, 0xBEEF);
        assert_eq!(m.ksm_scan().unwrap(), 0);
    }

    #[test] fn test_ksm_merge_reduces_shared_count() {
        let mut m = make_manager(); m.ksm_enable();
        m.register_process(1); m.register_process(2);
        let f1 = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        let f2 = m.map_page(2, 0x1000, PagePerms::rw()).unwrap();
        m.set_page_hash(f1, 0x1234); m.set_page_hash(f2, 0x1234);
        let before = m.shared_page_count();
        m.ksm_scan().unwrap();
        assert!(m.shared_page_count() < before);
    }

    // ── Stats Tests ───────────────────────────────────────────

    #[test] fn test_stats_initial() {
        let s = make_manager().stats();
        assert_eq!(s.total_forks, 0); assert_eq!(s.total_cow_faults, 0);
    }

    #[test] fn test_stats_after_fork() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x2000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        let s = m.stats();
        assert_eq!(s.total_forks, 1); assert_eq!(s.total_cow_pages, 2);
    }

    #[test] fn test_stats_after_cow_fault() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        m.handle_cow_fault(1, 0x1000);
        let s = m.stats();
        assert_eq!(s.total_cow_faults, 1); assert_eq!(s.total_cow_breaks, 1);
    }

    #[test] fn test_stats_multiple_forks() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); m.fork(1, 3).unwrap(); m.fork(1, 4).unwrap();
        let s = m.stats();
        assert_eq!(s.total_forks, 3); assert!(s.avg_pages_per_fork > 0.0);
    }

    #[test] fn test_max_sharers_seen() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); m.fork(1, 3).unwrap(); m.fork(1, 4).unwrap();
        assert!(m.stats().max_sharers_seen >= 4);
    }

    // ── Audit Tests ───────────────────────────────────────────

    #[test] fn test_audit_fork_events() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        assert!(!m.audit_by_event(CowAuditEvent::ForkInitiated).is_empty());
        assert!(!m.audit_by_event(CowAuditEvent::ForkCompleted).is_empty());
    }

    #[test] fn test_audit_cow_fault() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        m.handle_cow_fault(1, 0x1000);
        assert!(!m.audit_by_event(CowAuditEvent::CowFault).is_empty());
    }

    #[test] fn test_audit_for_process() {
        let mut m = make_manager(); m.register_process(1); m.register_process(2);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        let p1_events = m.audit_for_process(1);
        assert!(p1_events.iter().any(|e| e.event == CowAuditEvent::ForkInitiated));
    }

    #[test] fn test_audit_count() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        assert!(m.audit_count() > 0);
    }

    // ── Container CoW Tests ───────────────────────────────────

    #[test] fn test_container_register() {
        let mut m = make_manager(); m.register_process(1);
        m.container_register(100, 1);
        assert!(m.container_cow_stats(100).is_some());
    }

    #[test] fn test_container_fork() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x2000, PagePerms::rw()).unwrap();
        m.container_register(100, 1);
        let r = m.container_fork(100, 1, 2).unwrap();
        assert_eq!(r.child_pid, 2); assert_eq!(r.cow_pages, 2);
    }

    #[test] fn test_container_forked_pids() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.container_register(100, 1);
        m.container_fork(100, 1, 2).unwrap();
        m.container_fork(100, 1, 3).unwrap();
        let pids = m.container_forked_pids(100);
        assert_eq!(pids.len(), 2);
    }

    #[test] fn test_container_total_shared() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x2000, PagePerms::rw()).unwrap();
        m.container_register(100, 1);
        m.container_fork(100, 1, 2).unwrap();
        assert!(m.container_total_shared(100) > 0);
    }

    #[test] fn test_container_not_found() {
        let m = make_manager();
        assert!(m.container_cow_stats(999).is_none());
        assert_eq!(m.container_total_shared(999), 0);
    }

    // ── Snapshot Tests ───────────────────────────────────────

    #[test] fn test_snapshot_initial() {
        let s = make_manager().snapshot();
        assert_eq!(s.processes, 0); assert_eq!(s.forks, 0);
    }

    #[test] fn test_snapshot_after_fork() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        let s = m.snapshot();
        assert_eq!(s.processes, 2); assert_eq!(s.forks, 1);
        assert!(s.cow_pages > 0); assert!(s.memory_saved > 0);
    }

    #[test] fn test_snapshot_after_break() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        m.handle_cow_fault(1, 0x1000);
        let s = m.snapshot();
        assert!(s.cow_faults > 0); assert!(s.cow_breaks > 0);
    }

    // ── Cleanup Tests ─────────────────────────────────────────

    #[test] fn test_unregister_removes_shared_pages() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        let before = m.shared_page_count();
        m.unregister_process(2);
        assert!(m.shared_page_count() <= before);
    }

    #[test] fn test_unregister_parent_keeps_child() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        m.unregister_process(1);
        assert!(m.process_exists(2)); assert!(!m.process_exists(1));
    }

    // ── PagePerms Tests ───────────────────────────────────────

    #[test] fn test_page_perms_default() {
        let p = PagePerms::default();
        assert!(p.read); assert!(p.write); assert!(!p.exec);
    }

    #[test] fn test_page_perms_ro() {
        let p = PagePerms::ro();
        assert!(p.read); assert!(!p.write); assert!(!p.exec);
    }

    #[test] fn test_page_perms_rx() {
        let p = PagePerms::rx();
        assert!(p.read); assert!(!p.write); assert!(p.exec);
    }

    #[test] fn test_page_perms_rwx() {
        let p = PagePerms::rwx();
        assert!(p.read); assert!(p.write); assert!(p.exec);
    }

    // ── Complex Scenario Tests ───────────────────────────────

    #[test] fn test_fork_chain_break_selective() {
        let mut m = make_manager(); m.register_process(1);
        for i in 0..10 { m.map_page(1, 0x1000 + i, PagePerms::rw()).unwrap(); }
        m.fork(1, 2).unwrap();
        m.handle_cow_fault(1, 0x1000);
        m.handle_cow_fault(1, 0x1001);
        m.handle_cow_fault(1, 0x1002);
        assert_eq!(m.process_cow_pages(1), 7);
        assert_eq!(m.process_cow_pages(2), 10);
    }

    #[test] fn test_fork_write_heavy_workload() {
        let mut m = make_manager(); m.register_process(1);
        for i in 0..50 { m.map_page(1, 0x1000 + i, PagePerms::rw()).unwrap(); }
        let r = m.fork(1, 2).unwrap();
        assert_eq!(r.cow_pages, 50);
        for i in 0..50 { m.handle_cow_fault(2, 0x1000 + i); }
        assert_eq!(m.stats().total_cow_faults, 50);
        assert_eq!(m.stats().total_cow_breaks, 50);
    }

    #[test] fn test_fork_read_heavy_workload() {
        let mut m = make_manager(); m.register_process(1);
        for i in 0..50 { m.map_page(1, 0x1000 + i, PagePerms::ro()).unwrap(); }
        let r = m.fork(1, 2).unwrap();
        assert_eq!(r.cow_pages, 0); assert_eq!(r.shared_pages, 50);
        assert_eq!(m.process_cow_pages(1), 0); assert_eq!(m.process_cow_pages(2), 0);
    }

    #[test] fn test_deep_fork_tree() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        let mut current = 1;
        for i in 2..10 { m.fork(current, i).unwrap(); current = i; }
        assert_eq!(m.process_tree_depth(9), 8);
    }

    #[test] fn test_fork_unmap_cleanup() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap();
        m.unmap_page(1, 0x1000).unwrap();
        assert!(m.get_page_info(2, 0x1000).is_some());
    }

    #[test] fn test_multiple_forks_ref_count() {
        let mut m = make_manager(); m.register_process(1);
        let frame = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        for child in 2..6 { m.fork(1, child).unwrap(); }
        assert_eq!(m.page_ref_count(frame), 5);
    }

    #[test] fn test_break_one_of_many_sharers() {
        let mut m = make_manager(); m.register_process(1);
        let frame = m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.fork(1, 2).unwrap(); m.fork(1, 3).unwrap(); m.fork(1, 4).unwrap();
        m.handle_cow_fault(2, 0x1000);
        assert_eq!(m.page_ref_count(frame), 3);
        assert!(!m.is_page_cow(2, 0x1000));
        assert!(m.is_page_cow(1, 0x1000));
        assert!(m.is_page_cow(3, 0x1000));
        assert!(m.is_page_cow(4, 0x1000));
    }

    #[test] fn test_region_pages_after_partial_break() {
        let mut m = make_manager(); m.register_process(1);
        m.map_page(1, 0x1000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x2000, PagePerms::rw()).unwrap();
        m.map_page(1, 0x3000, PagePerms::rw()).unwrap();
        let r = m.fork(1, 2).unwrap();
        m.handle_cow_fault(1, 0x1000);
        m.handle_cow_fault(1, 0x2000);
        assert_eq!(m.region_pages_copied(r.region_id), 2);
        assert_eq!(m.region_pages_shared(r.region_id), 1);
        assert!(!m.get_region(r.region_id).unwrap().fully_broken);
    }
}

// ShivaCore — K-Sprint 44: Virtual Memory Management
// Copyright (c) 2026 Michael Wroblewski. All rights reserved.
//
// Erweitert die Basis-Speicherverwaltung (K2 paging, K8 memory_manager) um:
//   1. VMA (Virtual Memory Area) — Per-Process Speicherregionen mit Flags
//   2. COPY-ON-WRITE — Fork-Optimierung (Seiten teilen, bei Write kopieren)
//   3. DEMAND PAGING — Seiten werden erst bei erstem Zugriff allokiert
//   4. MMAP/MUNMAP — Anonymous und File-Backed Memory Mappings
//   5. MPROTECT — Schutz-Flags einer Region ändern (R/W/X)
//   6. SHARED MEMORY — IPC über geteilte Seiten zwischen Prozessen
//   7. SWAP MANAGEMENT — Inaktive Seiten auslagern, LRU Page Replacement
//   8. GUARD PAGES — Stack-Overflow Protection
//   9. PAGE STATISTICS — Per-Process Memory Metriken
//  10. OOM KILLER — Bei Speichermangel: größter Verbraucher terminieren

#![allow(dead_code)]

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use spin::Mutex;

// ════════════════════════════════════════════════════════════════
//  CONSTANTS
// ════════════════════════════════════════════════════════════════

const PAGE_SIZE: u64 = 4096;
const PAGE_SHIFT: u64 = 12;
const MAX_VMAS_PER_PROCESS: usize = 256;
const MAX_SHARED_REGIONS: usize = 128;
const MAX_SWAP_SLOTS: usize = 1024;
const MAX_PROCESSES: usize = 512;
const DEFAULT_STACK_SIZE: u64 = 8 * 1024 * 1024;   // 8 MB
const DEFAULT_STACK_PAGES: usize = 2048;
const GUARD_PAGE_SIZE: u64 = PAGE_SIZE;
const MAX_MMAP_SIZE: u64 = 256 * 1024 * 1024;       // 256 MB
const MIN_MMAP_SIZE: u64 = PAGE_SIZE;
const LRU_HAND_START: usize = 0;

// ════════════════════════════════════════════════════════════════
//  PAGE FLAGS
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PageFlags {
    pub read: bool,
    pub write: bool,
    pub exec: bool,
    pub shared: bool,
    pub cow: bool,          // Copy-on-Write
    pub guard: bool,        // Guard page (access = fault)
    pub locked: bool,       // Page locked in memory (no swap)
    pub dirty: bool,        // Modified since load
    pub accessed: bool,     // Accessed since last check
    pub swapped: bool,      // Currently in swap
    pub anonymous: bool,    // Anonymous (no file backing)
}

impl PageFlags {
    pub fn rw() -> Self {
        Self { read: true, write: true, anonymous: true, ..Self::default() }
    }

    pub fn rx() -> Self {
        Self { read: true, exec: true, anonymous: true, ..Self::default() }
    }

    pub fn ro() -> Self {
        Self { read: true, anonymous: true, ..Self::default() }
    }

    pub fn shared_rw() -> Self {
        Self { read: true, write: true, shared: true, anonymous: true, ..Self::default() }
    }

    pub fn cow() -> Self {
        Self { read: true, cow: true, anonymous: true, ..Self::default() }
    }

    pub fn guard() -> Self {
        Self { guard: true, ..Self::default() }
    }

    pub fn can_read(&self) -> bool { self.read }
    pub fn can_write(&self) -> bool { self.write && !self.cow }
    pub fn can_exec(&self) -> bool { self.exec }

    pub fn to_bits(&self) -> u32 {
        let mut bits = 0u32;
        if self.read { bits |= 0x01; }
        if self.write { bits |= 0x02; }
        if self.exec { bits |= 0x04; }
        if self.shared { bits |= 0x08; }
        if self.cow { bits |= 0x10; }
        if self.guard { bits |= 0x20; }
        if self.locked { bits |= 0x40; }
        if self.dirty { bits |= 0x80; }
        if self.accessed { bits |= 0x100; }
        if self.swapped { bits |= 0x200; }
        if self.anonymous { bits |= 0x400; }
        bits
    }

    pub fn from_bits(bits: u32) -> Self {
        Self {
            read: bits & 0x01 != 0,
            write: bits & 0x02 != 0,
            exec: bits & 0x04 != 0,
            shared: bits & 0x08 != 0,
            cow: bits & 0x10 != 0,
            guard: bits & 0x20 != 0,
            locked: bits & 0x40 != 0,
            dirty: bits & 0x80 != 0,
            accessed: bits & 0x100 != 0,
            swapped: bits & 0x200 != 0,
            anonymous: bits & 0x400 != 0,
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  VMA (Virtual Memory Area)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct Vma {
    pub vma_id: u32,
    pub start_addr: u64,
    pub end_addr: u64,           // Exclusive
    pub flags: PageFlags,
    pub backing: VmaBacking,
    pub offset: u64,             // File offset (for file-backed)
    pub pages_resident: u64,     // Pages currently in memory
    pub pages_swapped: u64,      // Pages in swap
    pub pages_cow: u64,           // CoW-shared pages
    pub protection: MemoryProtection,
}

impl Vma {
    pub fn new(vma_id: u32, start: u64, size: u64, flags: PageFlags) -> Self {
        Self {
            vma_id,
            start_addr: start,
            end_addr: start + size,
            flags,
            backing: VmaBacking::Anonymous,
            offset: 0,
            pages_resident: 0,
            pages_swapped: 0,
            pages_cow: 0,
            protection: MemoryProtection::from_flags(&flags),
        }
    }

    pub fn new_file(vma_id: u32, start: u64, size: u64, file_id: u32, offset: u64, flags: PageFlags) -> Self {
        Self {
            vma_id,
            start_addr: start,
            end_addr: start + size,
            flags,
            backing: VmaBacking::File { file_id, size },
            offset,
            pages_resident: 0,
            pages_swapped: 0,
            pages_cow: 0,
            protection: MemoryProtection::from_flags(&flags),
        }
    }

    pub fn new_shared(vma_id: u32, start: u64, size: u64, shm_id: u32, flags: PageFlags) -> Self {
        Self {
            vma_id,
            start_addr: start,
            end_addr: start + size,
            flags,
            backing: VmaBacking::Shared { shm_id },
            offset: 0,
            pages_resident: 0,
            pages_swapped: 0,
            pages_cow: 0,
            protection: MemoryProtection::from_flags(&flags),
        }
    }

    pub fn size(&self) -> u64 { self.end_addr - self.start_addr }
    pub fn page_count(&self) -> u64 { self.size() / PAGE_SIZE }
    pub fn contains(&self, addr: u64) -> bool { addr >= self.start_addr && addr < self.end_addr }

    pub fn set_protection(&mut self, flags: PageFlags) {
        self.flags = flags;
        self.protection = MemoryProtection::from_flags(&flags);
    }

    pub fn is_anonymous(&self) -> bool { matches!(self.backing, VmaBacking::Anonymous) }
    pub fn is_file_backed(&self) -> bool { matches!(self.backing, VmaBacking::File { .. }) }
    pub fn is_shared(&self) -> bool { matches!(self.backing, VmaBacking::Shared { .. }) }
    pub fn is_cow(&self) -> bool { self.flags.cow }
    pub fn is_guard(&self) -> bool { self.flags.guard }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VmaBacking {
    Anonymous,
    File { file_id: u32, size: u64 },
    Shared { shm_id: u32 },
}

// ════════════════════════════════════════════════════════════════
//  MEMORY PROTECTION
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryProtection {
    None,
    ReadOnly,
   ReadWrite,
    ReadExec,
    ReadWriteExec,
    Shared,
    SharedReadWrite,
}

impl MemoryProtection {
    pub fn from_flags(flags: &PageFlags) -> Self {
        if flags.guard { return Self::None; }
        match (flags.read, flags.write, flags.exec, flags.shared) {
            (true, true, true, _) => Self::ReadWriteExec,
            (true, true, false, true) => Self::SharedReadWrite,
            (true, true, false, false) => Self::ReadWrite,
            (true, false, true, _) => Self::ReadExec,
            (true, false, false, true) => Self::Shared,
            (true, false, false, false) => Self::ReadOnly,
            _ => Self::None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            MemoryProtection::None => "---",
            MemoryProtection::ReadOnly => "r--",
            MemoryProtection::ReadWrite => "rw-",
            MemoryProtection::ReadExec => "r-x",
            MemoryProtection::ReadWriteExec => "rwx",
            MemoryProtection::Shared => "r-s",
            MemoryProtection::SharedReadWrite => "rw-s",
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  PAGE TABLE ENTRY (simulated)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct PageEntry {
    pub addr: u64,        // Virtual address (page-aligned)
    pub frame: u64,        // Physical frame number (simulated)
    pub flags: PageFlags,
    pub ref_count: u32,    // Reference count (for CoW)
    pub swap_slot: Option<u32>,  // Swap slot if swapped out
    pub last_accessed: u64,  // Timestamp for LRU
}

impl PageEntry {
    pub fn new(addr: u64, frame: u64, flags: PageFlags) -> Self {
        Self {
            addr,
            frame,
            flags,
            ref_count: 1,
            swap_slot: None,
            last_accessed: 0,
        }
    }

    pub fn is_present(&self) -> bool { !self.flags.swapped }
    pub fn is_cow(&self) -> bool { self.flags.cow }
    pub fn is_shared(&self) -> bool { self.flags.shared }
    pub fn is_dirty(&self) -> bool { self.flags.dirty }
    pub fn is_swapped(&self) -> bool { self.flags.swapped }

    pub fn touch(&mut self, timestamp: u64) {
        self.flags.accessed = true;
        self.last_accessed = timestamp;
    }

    pub fn mark_dirty(&mut self) { self.flags.dirty = true; }
    pub fn clear_dirty(&mut self) { self.flags.dirty = false; }

    pub fn share(&mut self) {
        self.ref_count += 1;
        self.flags.shared = true;
    }

    pub fn unshare(&mut self) -> bool {
        if self.ref_count > 0 { self.ref_count -= 1; }
        self.ref_count == 0
    }

    pub fn make_cow(&mut self) {
        self.flags.cow = true;
        self.flags.write = false;
    }

    pub fn copy_for_write(&mut self, new_frame: u64) {
        self.flags.cow = false;
        self.flags.write = true;
        self.flags.dirty = true;
        self.frame = new_frame;
    }
}

// ════════════════════════════════════════════════════════════════
//  PAGE FAULT TYPES
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageFaultType {
    NotPresent,       // Page not in memory
    ProtectionFault,  // Permission violation
    CowFault,          // Copy-on-Write fault
    GuardPage,         // Guard page accessed
    StackOverflow,     // Stack guard page
    SwapIn,             // Page needs to be swapped in
}

impl PageFaultType {
    pub fn name(&self) -> &'static str {
        match self {
            PageFaultType::NotPresent => "not_present",
            PageFaultType::ProtectionFault => "protection_fault",
            PageFaultType::CowFault => "cow_fault",
            PageFaultType::GuardPage => "guard_page",
            PageFaultType::StackOverflow => "stack_overflow",
            PageFaultType::SwapIn => "swap_in",
        }
    }
}

#[derive(Clone, Debug)]
pub struct PageFault {
    pub fault_addr: u64,
    pub fault_type: PageFaultType,
    pub pid: u32,
    pub vma_id: Option<u32>,
    pub write_access: bool,
    pub resolution: PageFaultResolution,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageFaultResolution {
    Handled,          // Fault resolved (page mapped)
    Killed,           // Process killed (OOM or violation)
    Segfault,         // Invalid access (SIGSEGV)
    SwappedIn,        // Page swapped back in
    CowCopied,        // CoW page copied
    GuardPageHit,     // Guard page violation
    NotHandled,       // Could not resolve
}

// ════════════════════════════════════════════════════════════════
//  SHARED MEMORY REGION
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SharedMemory {
    pub shm_id: u32,
    pub name: String,
    pub size: u64,
    pub owner_pid: u32,
    pub attached_pids: Vec<u32>,
    pub pages: Vec<u64>,  // Physical frames
    pub flags: PageFlags,
    pub created_at: u64,
}

impl SharedMemory {
    pub fn new(shm_id: u32, name: &str, size: u64, owner: u32, flags: PageFlags) -> Self {
        Self {
            shm_id,
            name: name.to_string(),
            size,
            owner_pid: owner,
            attached_pids: vec![owner],
            pages: Vec::new(),
            flags,
            created_at: 0,
        }
    }

    pub fn attach(&mut self, pid: u32) -> bool {
        if !self.attached_pids.contains(&pid) {
            self.attached_pids.push(pid);
            true
        } else {
            false
        }
    }

    pub fn detach(&mut self, pid: u32) -> bool {
        let before = self.attached_pids.len();
        self.attached_pids.retain(|&p| p != pid);
        self.attached_pids.len() < before
    }

    pub fn attached_count(&self) -> usize { self.attached_pids.len() }
    pub fn is_orphaned(&self) -> bool { self.attached_pids.is_empty() }
}

// ════════════════════════════════════════════════════════════════
//  SWAP SLOT
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SwapSlot {
    pub slot_id: u32,
    pub pid: u32,
    pub vaddr: u64,
    pub frame: u64,       // Original frame (for swap-in)
    pub flags: PageFlags,
    pub timestamp: u64,
}

impl SwapSlot {
    pub fn new(slot_id: u32, pid: u32, vaddr: u64, frame: u64, flags: PageFlags) -> Self {
        Self { slot_id, pid, vaddr, frame, flags, timestamp: 0 }
    }
}

// ════════════════════════════════════════════════════════════════
//  PROCESS MEMORY STATE
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ProcessMemory {
    pub pid: u32,
    pub vmas: Vec<Vma>,
    pub page_table: BTreeMap<u64, PageEntry>,  // vaddr → entry
    pub stack_start: u64,
    pub stack_end: u64,        // Top of stack (grows down)
    pub heap_start: u64,
    pub heap_end: u64,
    pub brk: u64,              // Current break (heap top)
    pub total_pages: u64,
    pub resident_pages: u64,
    pub shared_pages: u64,
    pub swapped_pages: u64,
    pub cow_pages: u64,
    pub faults: u64,
    pub cow_faults: u64,
    pub swap_outs: u64,
    pub swap_ins: u64,
    pub oom_killed: bool,
    pub vma_counter: u32,
}

impl ProcessMemory {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            vmas: Vec::new(),
            page_table: BTreeMap::new(),
            stack_start: 0,
            stack_end: 0,
            heap_start: 0,
            heap_end: 0,
            brk: 0,
            total_pages: 0,
            resident_pages: 0,
            shared_pages: 0,
            swapped_pages: 0,
            cow_pages: 0,
            faults: 0,
            cow_faults: 0,
            swap_outs: 0,
            swap_ins: 0,
            oom_killed: false,
            vma_counter: 1,
        }
    }

    pub fn next_vma_id(&mut self) -> u32 {
        let id = self.vma_counter;
        self.vma_counter += 1;
        id
    }

    pub fn add_vma(&mut self, vma: Vma) -> bool {
        if self.vmas.len() >= MAX_VMAS_PER_PROCESS { return false; }
        self.vmas.push(vma);
        true
    }

    pub fn remove_vma(&mut self, vma_id: u32) -> bool {
        let before = self.vmas.len();
        self.vmas.retain(|v| v.vma_id != vma_id);
        self.vmas.len() < before
    }

    pub fn find_vma(&self, addr: u64) -> Option<&Vma> {
        self.vmas.iter().find(|v| v.contains(addr))
    }

    pub fn find_vma_mut(&mut self, addr: u64) -> Option<&mut Vma> {
        self.vmas.iter_mut().find(|v| v.contains(addr))
    }

    pub fn find_vma_by_id(&self, vma_id: u32) -> Option<&Vma> {
        self.vmas.iter().find(|v| v.vma_id == vma_id)
    }

    pub fn find_vma_by_id_mut(&mut self, vma_id: u32) -> Option<&mut Vma> {
        self.vmas.iter_mut().find(|v| v.vma_id == vma_id)
    }

    pub fn get_page(&self, vaddr: u64) -> Option<&PageEntry> {
        self.page_table.get(&page_align(vaddr))
    }

    pub fn get_page_mut(&mut self, vaddr: u64) -> Option<&mut PageEntry> {
        self.page_table.get_mut(&page_align(vaddr))
    }

    pub fn map_page(&mut self, vaddr: u64, frame: u64, flags: PageFlags) -> bool {
        let aligned = page_align(vaddr);
        if self.page_table.contains_key(&aligned) { return false; }
        self.page_table.insert(aligned, PageEntry::new(aligned, frame, flags));
        self.total_pages += 1;
        self.resident_pages += 1;
        true
    }

    pub fn unmap_page(&mut self, vaddr: u64) -> bool {
        let aligned = page_align(vaddr);
        if self.page_table.remove(&aligned).is_some() {
            self.total_pages = self.total_pages.saturating_sub(1);
            self.resident_pages = self.resident_pages.saturating_sub(1);
            true
        } else {
            false
        }
    }

    pub fn set_brk(&mut self, new_brk: u64) -> bool {
        if new_brk < self.heap_start { return false; }
        self.brk = new_brk;
        self.heap_end = new_brk;
        true
    }

    pub fn memory_used_bytes(&self) -> u64 {
        self.resident_pages * PAGE_SIZE
    }

    pub fn memory_virtual_bytes(&self) -> u64 {
        self.total_pages * PAGE_SIZE
    }

    pub fn vma_count(&self) -> usize { self.vmas.len() }
    pub fn page_count(&self) -> usize { self.page_table.len() }
}

// ════════════════════════════════════════════════════════════════
//  VMM ERRORS
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmmError {
    ProcessNotFound,
    VmaNotFound,
    PageNotPresent,
    InvalidAddress,
    PermissionDenied,
    OutOfMemory,
    InvalidSize,
    InvalidFlags,
    RegionOverlap,
    MaxVmasExceeded,
    MaxSharedExceeded,
    SwapFull,
    NotSwapped,
    AlreadyMapped,
    NotMapped,
    NotCOW,
    NotShared,
    AlreadyAttached,
    ShmNotFound,
}

impl VmmError {
    pub fn name(&self) -> &'static str {
        match self {
            VmmError::ProcessNotFound => "process_not_found",
            VmmError::VmaNotFound => "vma_not_found",
            VmmError::PageNotPresent => "page_not_present",
            VmmError::InvalidAddress => "invalid_address",
            VmmError::PermissionDenied => "permission_denied",
            VmmError::OutOfMemory => "out_of_memory",
            VmmError::InvalidSize => "invalid_size",
            VmmError::InvalidFlags => "invalid_flags",
            VmmError::RegionOverlap => "region_overlap",
            VmmError::MaxVmasExceeded => "max_vmas_exceeded",
            VmmError::MaxSharedExceeded => "max_shared_exceeded",
            VmmError::SwapFull => "swap_full",
            VmmError::NotSwapped => "not_swapped",
            VmmError::AlreadyMapped => "already_mapped",
            VmmError::NotMapped => "not_mapped",
            VmmError::NotCOW => "not_cow",
            VmmError::NotShared => "not_shared",
            VmmError::AlreadyAttached => "already_attached",
            VmmError::ShmNotFound => "shm_not_found",
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  VMM STATS
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct VmmStats {
    pub total_processes: usize,
    pub total_vmas: usize,
    pub total_pages: u64,
    pub resident_pages: u64,
    pub swapped_pages: u64,
    pub shared_regions: usize,
    pub shared_pages: u64,
    pub cow_pages: u64,
    pub total_faults: u64,
    pub cow_faults: u64,
    pub swap_outs: u64,
    pub swap_ins: u64,
    pub oom_kills: u64,
    pub free_frames: u64,
}

// ════════════════════════════════════════════════════════════════
//  HELPER FUNCTIONS
// ════════════════════════════════════════════════════════════════

pub fn page_align(addr: u64) -> u64 {
    addr & !(PAGE_SIZE - 1)
}

pub fn page_number(addr: u64) -> u64 {
    addr >> PAGE_SHIFT
}

pub fn is_page_aligned(addr: u64) -> bool {
    addr & (PAGE_SIZE - 1) == 0
}

pub fn pages_for(size: u64) -> u64 {
    (size + PAGE_SIZE - 1) / PAGE_SIZE
}

pub fn round_up_to_page(size: u64) -> u64 {
    pages_for(size) * PAGE_SIZE
}

// ════════════════════════════════════════════════════════════════
//  VIRTUAL MEMORY MANAGER
// ════════════════════════════════════════════════════════════════

pub struct VirtualMemoryManager {
    processes: BTreeMap<u32, ProcessMemory>,
    shared_regions: BTreeMap<u32, SharedMemory>,
    swap_slots: BTreeMap<u32, SwapSlot>,
    next_frame: u64,
    next_shm_id: u32,
    next_swap_slot: u32,
    lru_hand: Mutex<usize>,
    total_faults: u64,
    total_cow_faults: u64,
    total_swap_outs: u64,
    total_swap_ins: u64,
    total_oom_kills: u64,
    free_frame_count: u64,
    total_frame_count: u64,
    current_timestamp: u64,
    addr_counter: u64,
    // For process spawning: track parent-child for CoW
    cow_parents: BTreeMap<u32, u32>,  // child_pid → parent_pid
}

impl VirtualMemoryManager {
    pub fn new(total_frames: u64) -> Self {
        Self {
            processes: BTreeMap::new(),
            shared_regions: BTreeMap::new(),
            swap_slots: BTreeMap::new(),
            next_frame: 1,
            next_shm_id: 1,
            next_swap_slot: 1,
            lru_hand: Mutex::new(LRU_HAND_START),
            total_faults: 0,
            total_cow_faults: 0,
            total_swap_outs: 0,
            total_swap_ins: 0,
            total_oom_kills: 0,
            free_frame_count: total_frames,
            total_frame_count: total_frames,
            current_timestamp: 0,
            addr_counter: 0x10000000,
            cow_parents: BTreeMap::new(),
        }
    }

    fn alloc_frame(&mut self) -> Option<u64> {
        if self.free_frame_count == 0 {
            // Try swap
            self.swap_one_page()
        } else {
            self.free_frame_count -= 1;
            let frame = self.next_frame;
            self.next_frame += 1;
            Some(frame)
        }
    }

    fn free_frame(&mut self, _frame: u64) {
        self.free_frame_count += 1;
    }

    fn tick(&mut self) {
        self.current_timestamp += 1;
    }

    // ── Process Management ──────────────────────────────────

    pub fn register_process(&mut self, pid: u32) -> Result<(), VmmError> {
        if self.processes.contains_key(&pid) {
            return Err(VmmError::AlreadyMapped);
        }
        let mut pm = ProcessMemory::new(pid);
        // Set up default heap and stack
        pm.heap_start = self.addr_counter;
        pm.heap_end = pm.heap_start;
        pm.brk = pm.heap_start;
        self.addr_counter += DEFAULT_STACK_SIZE * 2;

        pm.stack_end = self.addr_counter;
        pm.stack_start = self.addr_counter + DEFAULT_STACK_SIZE;
        self.addr_counter += DEFAULT_STACK_SIZE * 2;

        self.processes.insert(pid, pm);
        Ok(())
    }

    pub fn unregister_process(&mut self, pid: u32) {
        // Free all pages
        if let Some(pm) = self.processes.get_mut(&pid) {
            let count = pm.page_table.len() as u64;
            self.free_frame_count += count;
        }
        // Detach from shared memory
        for (_, shm) in self.shared_regions.iter_mut() {
            shm.detach(pid);
        }
        // Remove orphaned shared regions
        let orphaned: Vec<u32> = self.shared_regions.iter()
            .filter(|(_, s)| s.is_orphaned() && s.owner_pid == pid)
            .map(|(id, _)| *id)
            .collect();
        for id in orphaned {
            self.shared_regions.remove(&id);
        }
        // Remove swap slots
        let swap_to_remove: Vec<u32> = self.swap_slots.iter()
            .filter(|(_, s)| s.pid == pid)
            .map(|(id, _)| *id)
            .collect();
        for id in swap_to_remove {
            self.swap_slots.remove(&id);
            self.free_frame_count += 1;
        }
        self.cow_parents.remove(&pid);
        self.processes.remove(&pid);
    }

    pub fn is_registered(&self, pid: u32) -> bool {
        self.processes.contains_key(&pid)
    }

    pub fn get_process(&self, pid: u32) -> Option<&ProcessMemory> {
        self.processes.get(&pid)
    }

    pub fn get_process_mut(&mut self, pid: u32) -> Option<&mut ProcessMemory> {
        self.processes.get_mut(&pid)
    }

    // ── mmap / munmap ────────────────────────────────────────

    pub fn mmap(&mut self, pid: u32, size: u64, flags: PageFlags) -> Result<u64, VmmError> {
        if size < MIN_MMAP_SIZE || size > MAX_MMAP_SIZE {
            return Err(VmmError::InvalidSize);
        }

        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;
        let aligned_size = round_up_to_page(size);
        let addr = self.addr_counter;
        self.addr_counter += aligned_size;

        let vma_id = pm.next_vma_id();
        let vma = Vma::new(vma_id, addr, aligned_size, flags);
        pm.add_vma(vma);

        self.tick();
        Ok(addr)
    }

    pub fn mmap_file(&mut self, pid: u32, file_id: u32, size: u64, offset: u64, flags: PageFlags) -> Result<u64, VmmError> {
        if size == 0 || size > MAX_MMAP_SIZE {
            return Err(VmmError::InvalidSize);
        }

        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;
        let aligned_size = round_up_to_page(size);
        let addr = self.addr_counter;
        self.addr_counter += aligned_size;

        let vma_id = pm.next_vma_id();
        let vma = Vma::new_file(vma_id, addr, aligned_size, file_id, offset, flags);
        pm.add_vma(vma);

        self.tick();
        Ok(addr)
    }

    pub fn munmap(&mut self, pid: u32, addr: u64) -> Result<(), VmmError> {
        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;
        let vma = pm.find_vma(addr).ok_or(VmmError::VmaNotFound)?.clone();
        let vma_id = vma.vma_id;

        // Unmap all pages in VMA
        let mut pages_to_free = 0u64;
        let start = vma.start_addr;
        let end = vma.end_addr;
        let mut pages = pm.page_table.iter().filter(|(&a, _)| a >= start && a < end).map(|(&a, _)| a).collect::<Vec<_>>();
        for page_addr in pages.drain(..) {
            if pm.page_table.remove(&page_addr).is_some() {
                pages_to_free += 1;
            }
        }
        pm.total_pages = pm.total_pages.saturating_sub(pages_to_free);
        pm.resident_pages = pm.resident_pages.saturating_sub(pages_to_free);
        self.free_frame_count += pages_to_free;

        pm.remove_vma(vma_id);
        self.tick();
        Ok(())
    }

    pub fn mprotect(&mut self, pid: u32, addr: u64, new_flags: PageFlags) -> Result<(), VmmError> {
        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;
        let vma = pm.find_vma_mut(addr).ok_or(VmmError::VmaNotFound)?;
        vma.set_protection(new_flags);

        // Update page table entries in this VMA
        let start = vma.start_addr;
        let end = vma.end_addr;
        for (_, entry) in pm.page_table.iter_mut() {
            if entry.addr >= start && entry.addr < end {
                entry.flags.read = new_flags.read;
                entry.flags.write = new_flags.write && !new_flags.cow;
                entry.flags.exec = new_flags.exec;
            }
        }

        self.tick();
        Ok(())
    }

    // ── Page Fault Handling ─────────────────────────────────

    pub fn handle_page_fault(&mut self, pid: u32, fault_addr: u64, write_access: bool) -> PageFault {
        self.tick();
        self.total_faults += 1;

        let aligned_addr = page_align(fault_addr);

        let pm = match self.processes.get_mut(&pid) {
            Some(p) => p,
            None => return PageFault {
                fault_addr, fault_type: PageFaultType::NotPresent, pid,
                vma_id: None, write_access, resolution: PageFaultResolution::NotHandled,
            },
        };

        pm.faults += 1;

        let vma = match pm.find_vma(aligned_addr) {
            Some(v) => v.clone(),
            None => return PageFault {
                fault_addr, fault_type: PageFaultType::NotPresent, pid,
                vma_id: None, write_access, resolution: PageFaultResolution::Segfault,
            },
        };

        let vma_id = vma.vma_id;

        // Guard page check
        if vma.is_guard() {
            let fault_type = if fault_addr < pm.stack_start && fault_addr >= pm.stack_start.saturating_sub(GUARD_PAGE_SIZE * 2) {
                PageFaultType::StackOverflow
            } else {
                PageFaultType::GuardPage
            };
            return PageFault {
                fault_addr, fault_type, pid,
                vma_id: Some(vma_id), write_access, resolution: PageFaultResolution::GuardPageHit,
            };
        }

        // Check permissions
        if write_access && !vma.flags.write && !vma.flags.cow {
            return PageFault {
                fault_addr, fault_type: PageFaultType::ProtectionFault, pid,
                vma_id: Some(vma_id), write_access, resolution: PageFaultResolution::Segfault,
            };
        }

        // Check if page exists
        let existing = pm.get_page(aligned_addr).cloned();

        match existing {
            // Page present — check for CoW
            Some(entry) if entry.is_cow() && write_access => {
                pm.cow_faults += 1;
                self.total_cow_faults += 1;
                let new_frame = self.alloc_frame();
                match new_frame {
                    Some(frame) => {
                        let entry = pm.get_page_mut(aligned_addr).unwrap();
                        entry.copy_for_write(frame);
                        pm.cow_pages = pm.cow_pages.saturating_sub(1);
                        self.tick();
                        PageFault {
                            fault_addr, fault_type: PageFaultType::CowFault, pid,
                            vma_id: Some(vma_id), write_access, resolution: PageFaultResolution::CowCopied,
                        }
                    }
                    None => {
                        pm.oom_killed = true;
                        self.total_oom_kills += 1;
                        PageFault {
                            fault_addr, fault_type: PageFaultType::NotPresent, pid,
                            vma_id: Some(vma_id), write_access, resolution: PageFaultResolution::Killed,
                        }
                    }
                }
            }
            // Page present and swapped
            Some(entry) if entry.is_swapped() => {
                let swap_slot = entry.swap_slot.unwrap();
                self.swap_in(pid, aligned_addr, swap_slot);
                self.tick();
                PageFault {
                    fault_addr, fault_type: PageFaultType::SwapIn, pid,
                    vma_id: Some(vma_id), write_access, resolution: PageFaultResolution::SwappedIn,
                }
            }
            // Page present, normal access — shouldn't fault
            Some(_) => {
                PageFault {
                    fault_addr, fault_type: PageFaultType::NotPresent, pid,
                    vma_id: Some(vma_id), write_access, resolution: PageFaultResolution::NotHandled,
                }
            }
            // Page not present — demand paging
            None => {
                let frame = match self.alloc_frame() {
                    Some(f) => f,
                    None => {
                        pm.oom_killed = true;
                        self.total_oom_kills += 1;
                        return PageFault {
                            fault_addr, fault_type: PageFaultType::NotPresent, pid,
                            vma_id: Some(vma_id), write_access, resolution: PageFaultResolution::Killed,
                        };
                    }
                };

                let mut page_flags = vma.flags;
                page_flags.accessed = true;
                pm.map_page(aligned_addr, frame, page_flags);

                // Update VMA stats
                if let Some(v) = pm.find_vma_mut(aligned_addr) {
                    v.pages_resident += 1;
                }

                self.tick();
                PageFault {
                    fault_addr, fault_type: PageFaultType::NotPresent, pid,
                    vma_id: Some(vma_id), write_access, resolution: PageFaultResolution::Handled,
                }
            }
        }
    }

    // ── Copy-on-Write Fork ───────────────────────────────────

    pub fn fork(&mut self, parent_pid: u32, child_pid: u32) -> Result<(), VmmError> {
        let parent = self.processes.get(&parent_pid).ok_or(VmmError::ProcessNotFound)?;

        // Create child process memory
        let mut child = ProcessMemory::new(child_pid);
        child.heap_start = parent.heap_start;
        child.heap_end = parent.heap_end;
        child.brk = parent.brk;
        child.stack_start = parent.stack_start;
        child.stack_end = parent.stack_end;

        // Clone VMAs
        child.vmas = parent.vmas.clone();
        child.vma_counter = parent.vma_counter;

        // Clone page table entries as CoW
        for (&vaddr, entry) in &parent.page_table {
            let mut child_entry = entry.clone();
            child_entry.ref_count = entry.ref_count + 1;
            child_entry.make_cow();
            child_entry.flags.dirty = false;
            child.page_table.insert(vaddr, child_entry);
        }

        // Mark parent pages as CoW too
        let parent = self.processes.get_mut(&parent_pid).unwrap();
        for (_, entry) in parent.page_table.iter_mut() {
            if !entry.flags.shared {  // Don't CoW shared pages
                entry.make_cow();
                entry.ref_count += 1;
                parent.cow_pages += 1;
            }
        }

        child.total_pages = parent.total_pages;
        child.resident_pages = parent.resident_pages;
        child.cow_pages = parent.cow_pages;

        self.cow_parents.insert(child_pid, parent_pid);
        self.processes.insert(child_pid, child);

        self.tick();
        Ok(())
    }

    // ── brk (heap management) ────────────────────────────────

    pub fn brk(&mut self, pid: u32, new_brk: u64) -> Result<u64, VmmError> {
        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;
        if new_brk < pm.heap_start {
            return Err(VmmError::InvalidAddress);
        }
        let old_brk = pm.brk;
        pm.set_brk(new_brk);

        // If expanding, create/update heap VMA
        if new_brk > old_brk {
            // Check if heap VMA exists
            let heap_vma = pm.vmas.iter_mut().find(|v| v.start_addr == pm.heap_start);
            if let Some(vma) = heap_vma {
                vma.end_addr = new_brk;
            } else {
                let vma_id = pm.next_vma_id();
                let vma = Vma::new(vma_id, pm.heap_start, new_brk - pm.heap_start, PageFlags::rw());
                pm.add_vma(vma);
            }
        }

        self.tick();
        Ok(new_brk)
    }

    // ── Shared Memory ────────────────────────────────────────

    pub fn shm_create(&mut self, name: &str, size: u64, owner_pid: u32, flags: PageFlags) -> Result<u32, VmmError> {
        if self.shared_regions.len() >= MAX_SHARED_REGIONS {
            return Err(VmmError::MaxSharedExceeded);
        }

        let shm_id = self.next_shm_id;
        self.next_shm_id += 1;

        let aligned_size = round_up_to_page(size);
        let mut shm = SharedMemory::new(shm_id, name, aligned_size, owner_pid, flags);

        // Allocate physical frames
        let page_count = aligned_size / PAGE_SIZE;
        for _ in 0..page_count {
            if let Some(frame) = self.alloc_frame() {
                shm.pages.push(frame);
            } else {
                return Err(VmmError::OutOfMemory);
            }
        }

        self.shared_regions.insert(shm_id, shm);
        self.tick();
        Ok(shm_id)
    }

    pub fn shm_attach(&mut self, shm_id: u32, pid: u32, addr: u64) -> Result<(), VmmError> {
        let shm = self.shared_regions.get_mut(&shm_id).ok_or(VmmError::ShmNotFound)?;
        shm.attach(pid);

        // Create a VMA for the attachment
        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;
        let vma_id = pm.next_vma_id();
        let vma = Vma::new_shared(vma_id, addr, shm.size, shm_id, shm.flags);
        pm.add_vma(vma);

        // Map shared pages
        for (i, &frame) in shm.pages.iter().enumerate() {
            let page_addr = addr + (i as u64) * PAGE_SIZE;
            let mut flags = shm.flags;
            flags.shared = true;
            pm.map_page(page_addr, frame, flags);
        }

        pm.shared_pages += shm.pages.len() as u64;
        self.tick();
        Ok(())
    }

    pub fn shm_detach(&mut self, shm_id: u32, pid: u32) -> Result<(), VmmError> {
        let shm = self.shared_regions.get_mut(&shm_id).ok_or(VmmError::ShmNotFound)?;
        shm.detach(pid);

        // Remove VMA and unmap pages
        if let Some(pm) = self.processes.get_mut(&pid) {
            let shm_size = shm.size;
            // Find and remove shared VMA
            let vma_to_remove = pm.vmas.iter()
                .find(|v| v.is_shared() && v.contains(0))  // Will be found properly
                .map(|v| v.vma_id);
            // More precise: find VMA with matching shm_id
            let vma_id = pm.vmas.iter()
                .find(|v| matches!(&v.backing, VmaBacking::Shared { shm_id: id } if *id == shm_id))
                .map(|v| (v.vma_id, v.start_addr, v.end_addr));

            if let Some((vid, start, end)) = vma_id {
                pm.remove_vma(vid);
                // Unmap pages
                let pages: Vec<u64> = pm.page_table.iter()
                    .filter(|(&a, _)| a >= start && a < end)
                    .map(|(&a, _)| a)
                    .collect();
                for addr in pages {
                    pm.unmap_page(addr);
                }
                pm.shared_pages = pm.shared_pages.saturating_sub(shm_size / PAGE_SIZE);
            }
        }

        // If orphaned, free frames
        if let Some(shm) = self.shared_regions.get(&shm_id) {
            if shm.is_orphaned() {
                self.free_frame_count += shm.pages.len() as u64;
                self.shared_regions.remove(&shm_id);
            }
        }

        self.tick();
        Ok(())
    }

    pub fn shm_destroy(&mut self, shm_id: u32) -> Result<(), VmmError> {
        let shm = self.shared_regions.get(&shm_id).ok_or(VmmError::ShmNotFound)?;
        let frame_count = shm.pages.len() as u64;
        self.shared_regions.remove(&shm_id);
        self.free_frame_count += frame_count;
        self.tick();
        Ok(())
    }

    pub fn shm_count(&self) -> usize { self.shared_regions.len() }
    pub fn get_shm(&self, shm_id: u32) -> Option<&SharedMemory> { self.shared_regions.get(&shm_id) }

    // ── Swap Management ──────────────────────────────────────

    pub fn swap_out(&mut self, pid: u32, vaddr: u64) -> Result<(), VmmError> {
        if self.swap_slots.len() >= MAX_SWAP_SLOTS {
            return Err(VmmError::SwapFull);
        }

        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;
        let entry = pm.get_page_mut(vaddr).ok_or(VmmError::PageNotPresent)?;

        if entry.flags.locked {
            return Err(VmmError::PermissionDenied);
        }
        if entry.flags.swapped {
            return Err(VmmError::NotSwapped);  // Already swapped
        }

        let slot_id = self.next_swap_slot;
        self.next_swap_slot += 1;

        let slot = SwapSlot::new(slot_id, pid, vaddr, entry.frame, entry.flags);
        self.swap_slots.insert(slot_id, slot);

        entry.flags.swapped = true;
        entry.swap_slot = Some(slot_id);
        pm.swapped_pages += 1;
        pm.resident_pages = pm.resident_pages.saturating_sub(1);
        pm.swap_outs += 1;
        self.total_swap_outs += 1;

        // Free the physical frame
        self.free_frame_count += 1;

        self.tick();
        Ok(())
    }

    pub fn swap_in(&mut self, pid: u32, vaddr: u64, slot_id: u32) -> Result<(), VmmError> {
        let slot = self.swap_slots.remove(&slot_id).ok_or(VmmError::NotSwapped)?;

        let frame = match self.alloc_frame() {
            Some(f) => f,
            None => return Err(VmmError::OutOfMemory),
        };

        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;
        let entry = pm.get_page_mut(vaddr).ok_or(VmmError::PageNotPresent)?;

        entry.frame = frame;
        entry.flags.swapped = false;
        entry.swap_slot = None;
        pm.swapped_pages = pm.swapped_pages.saturating_sub(1);
        pm.resident_pages += 1;
        pm.swap_ins += 1;
        self.total_swap_ins += 1;

        self.tick();
        Ok(())
    }

    fn swap_one_page(&mut self) -> Option<u64> {
        // LRU page replacement
        let mut oldest: Option<(u32, u64, u64)> = None;  // (pid, vaddr, timestamp)

        for (&pid, pm) in self.processes.iter() {
            for (&vaddr, entry) in pm.page_table.iter() {
                if entry.flags.locked || entry.flags.swapped {
                    continue;
                }
                match oldest {
                    None => oldest = Some((pid, vaddr, entry.last_accessed)),
                    Some((_, _, ts)) if entry.last_accessed < ts => {
                        oldest = Some((pid, vaddr, entry.last_accessed));
                    }
                    _ => {}
                }
            }
        }

        if let Some((pid, vaddr, _)) = oldest {
            // Swap out the oldest page
            let pm = self.processes.get_mut(&pid).unwrap();
            let entry = pm.get_page_mut(vaddr).unwrap();
            let frame = entry.frame;

            let slot_id = self.next_swap_slot;
            self.next_swap_slot += 1;
            self.swap_slots.insert(slot_id, SwapSlot::new(slot_id, pid, vaddr, frame, entry.flags));

            entry.flags.swapped = true;
            entry.swap_slot = Some(slot_id);
            pm.swapped_pages += 1;
            pm.resident_pages = pm.resident_pages.saturating_sub(1);
            pm.swap_outs += 1;
            self.total_swap_outs += 1;

            return Some(frame);
        }

        None
    }

    pub fn swap_count(&self) -> usize { self.swap_slots.len() }

    // ── Guard Pages ──────────────────────────────────────────

    pub fn setup_stack_guard(&mut self, pid: u32) -> Result<(), VmmError> {
        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;

        // Guard page just below the stack
        let guard_addr = pm.stack_end.saturating_sub(GUARD_PAGE_SIZE);
        let vma_id = pm.next_vma_id();
        let guard_vma = Vma::new(vma_id, guard_addr, GUARD_PAGE_SIZE, PageFlags::guard());
        pm.add_vma(guard_vma);

        self.tick();
        Ok(())
    }

    pub fn setup_heap_guard(&mut self, pid: u32) -> Result<(), VmmError> {
        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;

        // Guard page at end of heap region
        let guard_addr = pm.heap_start.saturating_sub(GUARD_PAGE_SIZE);
        let vma_id = pm.next_vma_id();
        let guard_vma = Vma::new(vma_id, guard_addr, GUARD_PAGE_SIZE, PageFlags::guard());
        pm.add_vma(guard_vma);

        self.tick();
        Ok(())
    }

    // ── OOM Killer ───────────────────────────────────────────

    pub fn oom_kill(&mut self) -> Option<u32> {
        // Find the process using the most memory
        let mut target: Option<(u32, u64)> = None;
        for (&pid, pm) in self.processes.iter() {
            if pm.oom_killed { continue; }
            let usage = pm.memory_used_bytes();
            match target {
                None => target = Some((pid, usage)),
                Some((_, u)) if usage > u => target = Some((pid, usage)),
                _ => {}
            }
        }

        if let Some((pid, _)) = target {
            if let Some(pm) = self.processes.get_mut(&pid) {
                pm.oom_killed = true;
            }
            self.total_oom_kills += 1;
            Some(pid)
        } else {
            None
        }
    }

    // ── Access Tracking ──────────────────────────────────────

    pub fn touch_page(&mut self, pid: u32, vaddr: u64) -> Result<(), VmmError> {
        let pm = self.processes.get_mut(&pid).ok_or(VmmError::ProcessNotFound)?;
        let entry = pm.get_page_mut(vaddr).ok_or(VmmError::PageNotPresent)?;
        entry.touch(self.current_timestamp);
        Ok(())
    }

    // ── Statistics ───────────────────────────────────────────

    pub fn stats(&self) -> VmmStats {
        let mut total_vmas = 0usize;
        let mut total_pages = 0u64;
        let mut resident_pages = 0u64;
        let mut swapped_pages = 0u64;
        let mut shared_pages = 0u64;
        let mut cow_pages = 0u64;

        for pm in self.processes.values() {
            total_vmas += pm.vma_count();
            total_pages += pm.total_pages;
            resident_pages += pm.resident_pages;
            swapped_pages += pm.swapped_pages;
            shared_pages += pm.shared_pages;
            cow_pages += pm.cow_pages;
        }

        VmmStats {
            total_processes: self.processes.len(),
            total_vmas,
            total_pages,
            resident_pages,
            swapped_pages,
            shared_regions: self.shared_regions.len(),
            shared_pages,
            cow_pages,
            total_faults: self.total_faults,
            cow_faults: self.total_cow_faults,
            swap_outs: self.total_swap_outs,
            swap_ins: self.total_swap_ins,
            oom_kills: self.total_oom_kills,
            free_frames: self.free_frame_count,
        }
    }

    pub fn process_count(&self) -> usize { self.processes.len() }
    pub fn free_frames(&self) -> u64 { self.free_frame_count }
    pub fn used_frames(&self) -> u64 { self.total_frame_count - self.free_frame_count }
}

// ════════════════════════════════════════════════════════════════
//  TESTS
// ════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helper Functions ─────────────────────────────────────

    #[test]
    fn test_page_align() {
        assert_eq!(page_align(0x1000), 0x1000);
        assert_eq!(page_align(0x1FFF), 0x1000);
        assert_eq!(page_align(0x2000), 0x2000);
        assert_eq!(page_align(0x1234), 0x1000);
    }

    #[test]
    fn test_page_number() {
        assert_eq!(page_number(0x1000), 1);
        assert_eq!(page_number(0x2000), 2);
        assert_eq!(page_number(0x0), 0);
    }

    #[test]
    fn test_is_page_aligned() {
        assert!(is_page_aligned(0x1000));
        assert!(is_page_aligned(0x2000));
        assert!(!is_page_aligned(0x1001));
        assert!(!is_page_aligned(0x1234));
    }

    #[test]
    fn test_pages_for() {
        assert_eq!(pages_for(1), 1);
        assert_eq!(pages_for(PAGE_SIZE), 1);
        assert_eq!(pages_for(PAGE_SIZE + 1), 2);
        assert_eq!(pages_for(PAGE_SIZE * 3), 3);
    }

    #[test]
    fn test_round_up_to_page() {
        assert_eq!(round_up_to_page(1), PAGE_SIZE);
        assert_eq!(round_up_to_page(PAGE_SIZE), PAGE_SIZE);
        assert_eq!(round_up_to_page(PAGE_SIZE + 1), PAGE_SIZE * 2);
    }

    // ── PageFlags Tests ──────────────────────────────────────

    #[test]
    fn test_page_flags_rw() {
        let flags = PageFlags::rw();
        assert!(flags.can_read());
        assert!(flags.can_write());
        assert!(!flags.can_exec());
        assert!(flags.anonymous);
    }

    #[test]
    fn test_page_flags_rx() {
        let flags = PageFlags::rx();
        assert!(flags.can_read());
        assert!(!flags.can_write());
        assert!(flags.can_exec());
    }

    #[test]
    fn test_page_flags_ro() {
        let flags = PageFlags::ro();
        assert!(flags.can_read());
        assert!(!flags.can_write());
        assert!(!flags.can_exec());
    }

    #[test]
    fn test_page_flags_cow() {
        let flags = PageFlags::cow();
        assert!(flags.can_read());
        assert!(!flags.can_write());  // CoW can't write directly
        assert!(flags.cow);
    }

    #[test]
    fn test_page_flags_guard() {
        let flags = PageFlags::guard();
        assert!(!flags.can_read());
        assert!(!flags.can_write());
        assert!(flags.guard);
    }

    #[test]
    fn test_page_flags_to_from_bits() {
        let flags = PageFlags { read: true, write: true, exec: true, cow: true, dirty: true, anonymous: true, ..PageFlags::default() };
        let bits = flags.to_bits();
        let restored = PageFlags::from_bits(bits);
        assert_eq!(flags, restored);
    }

    #[test]
    fn test_page_flags_shared_rw() {
        let flags = PageFlags::shared_rw();
        assert!(flags.can_read());
        assert!(flags.can_write());
        assert!(flags.shared);
    }

    // ── MemoryProtection Tests ────────────────────────────────

    #[test]
    fn test_memory_protection_names() {
        assert_eq!(MemoryProtection::None.name(), "---");
        assert_eq!(MemoryProtection::ReadOnly.name(), "r--");
        assert_eq!(MemoryProtection::ReadWrite.name(), "rw-");
        assert_eq!(MemoryProtection::ReadExec.name(), "r-x");
        assert_eq!(MemoryProtection::ReadWriteExec.name(), "rwx");
        assert_eq!(MemoryProtection::SharedReadWrite.name(), "rw-s");
    }

    #[test]
    fn test_memory_protection_from_flags() {
        assert_eq!(MemoryProtection::from_flags(&PageFlags::rw()), MemoryProtection::ReadWrite);
        assert_eq!(MemoryProtection::from_flags(&PageFlags::ro()), MemoryProtection::ReadOnly);
        assert_eq!(MemoryProtection::from_flags(&PageFlags::rx()), MemoryProtection::ReadExec);
        assert_eq!(MemoryProtection::from_flags(&PageFlags::guard()), MemoryProtection::None);
    }

    // ── VMA Tests ─────────────────────────────────────────────

    #[test]
    fn test_vma_creation() {
        let vma = Vma::new(1, 0x100000, 0x1000, PageFlags::rw());
        assert_eq!(vma.vma_id, 1);
        assert_eq!(vma.start_addr, 0x100000);
        assert_eq!(vma.end_addr, 0x101000);
        assert_eq!(vma.size(), 0x1000);
        assert!(vma.is_anonymous());
        assert!(!vma.is_file_backed());
    }

    #[test]
    fn test_vma_contains() {
        let vma = Vma::new(1, 0x100000, 0x1000, PageFlags::rw());
        assert!(vma.contains(0x100000));
        assert!(vma.contains(0x100FFF));
        assert!(!vma.contains(0x101000));  // Exclusive end
        assert!(!vma.contains(0x0FFFF));   // Below
    }

    #[test]
    fn test_vma_page_count() {
        let vma = Vma::new(1, 0x100000, 0x8000, PageFlags::rw());
        assert_eq!(vma.page_count(), 8);
    }

    #[test]
    fn test_vma_file_backed() {
        let vma = Vma::new_file(1, 0x100000, 0x1000, 42, 0, PageFlags::ro());
        assert!(vma.is_file_backed());
        assert!(!vma.is_anonymous());
    }

    #[test]
    fn test_vma_shared() {
        let vma = Vma::new_shared(1, 0x100000, 0x1000, 1, PageFlags::shared_rw());
        assert!(vma.is_shared());
    }

    #[test]
    fn test_vma_set_protection() {
        let mut vma = Vma::new(1, 0x100000, 0x1000, PageFlags::rw());
        vma.set_protection(PageFlags::ro());
        assert!(!vma.flags.can_write());
    }

    #[test]
    fn test_vma_guard() {
        let vma = Vma::new(1, 0xFF000, 0x1000, PageFlags::guard());
        assert!(vma.is_guard());
    }

    // ── PageEntry Tests ──────────────────────────────────────

    #[test]
    fn test_page_entry_new() {
        let entry = PageEntry::new(0x1000, 42, PageFlags::rw());
        assert_eq!(entry.addr, 0x1000);
        assert_eq!(entry.frame, 42);
        assert!(entry.is_present());
        assert!(!entry.is_cow());
        assert!(!entry.is_swapped());
    }

    #[test]
    fn test_page_entry_touch() {
        let mut entry = PageEntry::new(0x1000, 42, PageFlags::rw());
        entry.touch(100);
        assert!(entry.flags.accessed);
        assert_eq!(entry.last_accessed, 100);
    }

    #[test]
    fn test_page_entry_mark_dirty() {
        let mut entry = PageEntry::new(0x1000, 42, PageFlags::rw());
        assert!(!entry.is_dirty());
        entry.mark_dirty();
        assert!(entry.is_dirty());
    }

    #[test]
    fn test_page_entry_share() {
        let mut entry = PageEntry::new(0x1000, 42, PageFlags::rw());
        assert_eq!(entry.ref_count, 1);
        entry.share();
        assert_eq!(entry.ref_count, 2);
        assert!(entry.is_shared());
    }

    #[test]
    fn test_page_entry_unshare() {
        let mut entry = PageEntry::new(0x1000, 42, PageFlags::rw());
        entry.share();
        entry.share();
        assert_eq!(entry.ref_count, 3);
        assert!(!entry.unshare());  // Still 2 refs
        assert!(entry.unshare());   // Still 1 ref
        assert!(!entry.unshare());   // Now 0 — fully unshared
    }

    #[test]
    fn test_page_entry_make_cow() {
        let mut entry = PageEntry::new(0x1000, 42, PageFlags::rw());
        entry.make_cow();
        assert!(entry.is_cow());
        assert!(!entry.flags.write);
    }

    #[test]
    fn test_page_entry_copy_for_write() {
        let mut entry = PageEntry::new(0x1000, 42, PageFlags::cow());
        entry.copy_for_write(99);
        assert!(!entry.is_cow());
        assert!(entry.flags.write);
        assert!(entry.is_dirty());
        assert_eq!(entry.frame, 99);
    }

    // ── PageFaultType Tests ──────────────────────────────────

    #[test]
    fn test_page_fault_type_names() {
        assert_eq!(PageFaultType::NotPresent.name(), "not_present");
        assert_eq!(PageFaultType::ProtectionFault.name(), "protection_fault");
        assert_eq!(PageFaultType::CowFault.name(), "cow_fault");
        assert_eq!(PageFaultType::GuardPage.name(), "guard_page");
        assert_eq!(PageFaultType::StackOverflow.name(), "stack_overflow");
        assert_eq!(PageFaultType::SwapIn.name(), "swap_in");
    }

    // ── SharedMemory Tests ───────────────────────────────────

    #[test]
    fn test_shared_memory_creation() {
        let shm = SharedMemory::new(1, "test_shm", 0x1000, 100, PageFlags::shared_rw());
        assert_eq!(shm.shm_id, 1);
        assert_eq!(shm.name, "test_shm");
        assert_eq!(shm.owner_pid, 100);
        assert_eq!(shm.attached_count(), 1);
    }

    #[test]
    fn test_shared_memory_attach() {
        let mut shm = SharedMemory::new(1, "test", 0x1000, 100, PageFlags::shared_rw());
        assert!(shm.attach(101));
        assert_eq!(shm.attached_count(), 2);
        // Can't attach twice
        assert!(!shm.attach(101));
    }

    #[test]
    fn test_shared_memory_detach() {
        let mut shm = SharedMemory::new(1, "test", 0x1000, 100, PageFlags::shared_rw());
        shm.attach(101);
        assert!(shm.detach(101));
        assert_eq!(shm.attached_count(), 1);
        // Can't detach twice
        assert!(!shm.detach(101));
    }

    #[test]
    fn test_shared_memory_orphaned() {
        let mut shm = SharedMemory::new(1, "test", 0x1000, 100, PageFlags::shared_rw());
        assert!(!shm.is_orphaned());
        shm.detach(100);
        assert!(shm.is_orphaned());
    }

    // ── ProcessMemory Tests ──────────────────────────────────

    #[test]
    fn test_process_memory_new() {
        let pm = ProcessMemory::new(100);
        assert_eq!(pm.pid, 100);
        assert!(pm.vmas.is_empty());
        assert!(pm.page_table.is_empty());
        assert_eq!(pm.total_pages, 0);
    }

    #[test]
    fn test_process_memory_add_remove_vma() {
        let mut pm = ProcessMemory::new(100);
        let vma = Vma::new(1, 0x100000, 0x1000, PageFlags::rw());
        assert!(pm.add_vma(vma));
        assert_eq!(pm.vma_count(), 1);
        assert!(pm.remove_vma(1));
        assert_eq!(pm.vma_count(), 0);
    }

    #[test]
    fn test_process_memory_find_vma() {
        let mut pm = ProcessMemory::new(100);
        pm.add_vma(Vma::new(1, 0x100000, 0x2000, PageFlags::rw()));
        assert!(pm.find_vma(0x100000).is_some());
        assert!(pm.find_vma(0x101FFF).is_some());
        assert!(pm.find_vma(0x102000).is_none());
    }

    #[test]
    fn test_process_memory_map_unmap_page() {
        let mut pm = ProcessMemory::new(100);
        assert!(pm.map_page(0x1000, 42, PageFlags::rw()));
        assert_eq!(pm.total_pages, 1);
        assert!(pm.get_page(0x1000).is_some());

        // Can't map same page twice
        assert!(!pm.map_page(0x1000, 43, PageFlags::rw()));

        assert!(pm.unmap_page(0x1000));
        assert_eq!(pm.total_pages, 0);
        assert!(pm.get_page(0x1000).is_none());
    }

    #[test]
    fn test_process_memory_set_brk() {
        let mut pm = ProcessMemory::new(100);
        pm.heap_start = 0x10000;
        assert!(pm.set_brk(0x20000));
        assert_eq!(pm.brk, 0x20000);
        // Can't go below heap_start
        assert!(!pm.set_brk(0x0));
    }

    #[test]
    fn test_process_memory_memory_stats() {
        let mut pm = ProcessMemory::new(100);
        pm.map_page(0x1000, 1, PageFlags::rw());
        pm.map_page(0x2000, 2, PageFlags::rw());
        assert_eq!(pm.memory_used_bytes(), PAGE_SIZE * 2);
        assert_eq!(pm.memory_virtual_bytes(), PAGE_SIZE * 2);
    }

    // ── VMM: Process Management Tests ────────────────────────

    #[test]
    fn test_vmm_register_process() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        assert!(vmm.is_registered(100));
        assert!(vmm.get_process(100).is_some());
    }

    #[test]
    fn test_vmm_unregister_process() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        vmm.unregister_process(100);
        assert!(!vmm.is_registered(100));
    }

    #[test]
    fn test_vmm_register_duplicate() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let result = vmm.register_process(100);
        assert_eq!(result, Err(VmmError::AlreadyMapped));
    }

    // ── VMM: mmap/munmap Tests ────────────────────────────────

    #[test]
    fn test_vmm_mmap() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();
        assert!(addr > 0);
        let pm = vmm.get_process(100).unwrap();
        assert_eq!(pm.vma_count(), 1);
        assert!(pm.find_vma(addr).is_some());
    }

    #[test]
    fn test_vmm_mmap_invalid_size() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        assert_eq!(vmm.mmap(100, 0, PageFlags::rw()), Err(VmmError::InvalidSize));
        assert_eq!(vmm.mmap(100, MAX_MMAP_SIZE + 1, PageFlags::rw()), Err(VmmError::InvalidSize));
    }

    #[test]
    fn test_vmm_mmap_file() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap_file(100, 42, 0x1000, 0, PageFlags::ro()).unwrap();
        let pm = vmm.get_process(100).unwrap();
        let vma = pm.find_vma(addr).unwrap();
        assert!(vma.is_file_backed());
    }

    #[test]
    fn test_vmm_munmap() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();
        vmm.munmap(100, addr).unwrap();
        let pm = vmm.get_process(100).unwrap();
        assert_eq!(pm.vma_count(), 0);
    }

    #[test]
    fn test_vmm_munmap_not_found() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        assert_eq!(vmm.munmap(100, 0xDEAD), Err(VmmError::VmaNotFound));
    }

    #[test]
    fn test_vmm_mprotect() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();
        vmm.mprotect(100, addr, PageFlags::ro()).unwrap();
        let pm = vmm.get_process(100).unwrap();
        let vma = pm.find_vma(addr).unwrap();
        assert!(!vma.flags.can_write());
    }

    // ── VMM: Page Fault Tests ────────────────────────────────

    #[test]
    fn test_vmm_page_fault_demand_paging() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();

        // Access triggers demand paging
        let fault = vmm.handle_page_fault(100, addr, false);
        assert_eq!(fault.resolution, PageFaultResolution::Handled);
        assert_eq!(fault.fault_type, PageFaultType::NotPresent);

        let pm = vmm.get_process(100).unwrap();
        assert!(pm.get_page(addr).is_some());
        assert_eq!(pm.faults, 1);
    }

    #[test]
    fn test_vmm_page_fault_no_vma() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let fault = vmm.handle_page_fault(100, 0xDEAD, false);
        assert_eq!(fault.resolution, PageFaultResolution::Segfault);
    }

    #[test]
    fn test_vmm_page_fault_protection() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::ro()).unwrap();

        // Write to read-only page
        let fault = vmm.handle_page_fault(100, addr, true);
        assert_eq!(fault.fault_type, PageFaultType::ProtectionFault);
        assert_eq!(fault.resolution, PageFaultResolution::Segfault);
    }

    #[test]
    fn test_vmm_page_fault_guard_page() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        vmm.setup_stack_guard(100).unwrap();

        // Find the guard VMA
        let pm = vmm.get_process(100).unwrap();
        let guard_vma = pm.vmas.iter().find(|v| v.is_guard()).unwrap();
        let guard_addr = guard_vma.start_addr;

        drop(pm);
        let fault = vmm.handle_page_fault(100, guard_addr, false);
        assert_eq!(fault.resolution, PageFaultResolution::GuardPageHit);
    }

    // ── VMM: CoW Fork Tests ──────────────────────────────────

    #[test]
    fn test_vmm_fork() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();

        // Trigger demand paging to get a page mapped
        vmm.handle_page_fault(100, addr, false);

        // Fork
        vmm.register_process(101).unwrap();
        vmm.fork(100, 101).unwrap();

        // Child should have same pages marked CoW
        let child = vmm.get_process(101).unwrap();
        assert!(!child.page_table.is_empty());
        for (_, entry) in &child.page_table {
            assert!(entry.is_cow());
        }

        // Parent pages should also be CoW now
        let parent = vmm.get_process(100).unwrap();
        for (_, entry) in &parent.page_table {
            if !entry.is_shared() {
                assert!(entry.is_cow());
            }
        }
    }

    #[test]
    fn test_vmm_fork_cow_fault() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();

        // Map a page
        vmm.handle_page_fault(100, addr, false);

        // Fork
        vmm.register_process(101).unwrap();
        vmm.fork(100, 101).unwrap();

        // Child writes — CoW fault
        let fault = vmm.handle_page_fault(101, addr, true);
        assert_eq!(fault.fault_type, PageFaultType::CowFault);
        assert_eq!(fault.resolution, PageFaultResolution::CowCopied);

        // Child should now have its own copy
        let child = vmm.get_process(101).unwrap();
        let entry = child.get_page(addr).unwrap();
        assert!(!entry.is_cow());
        assert!(entry.flags.write);
    }

    #[test]
    fn test_vmm_fork_parent_not_found() {
        let mut vmm = VirtualMemoryManager::new(1024);
        let result = vmm.fork(999, 101);
        assert_eq!(result, Err(VmmError::ProcessNotFound));
    }

    // ── VMM: brk Tests ────────────────────────────────────────

    #[test]
    fn test_vmm_brk() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let pm = vmm.get_process(100).unwrap();
        let heap_start = pm.heap_start;

        let new_brk = vmm.brk(100, heap_start + 0x10000).unwrap();
        assert_eq!(new_brk, heap_start + 0x10000);

        let pm = vmm.get_process(100).unwrap();
        assert_eq!(pm.brk, new_brk);
    }

    #[test]
    fn test_vmm_brk_invalid() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let pm = vmm.get_process(100).unwrap();
        let heap_start = pm.heap_start;
        drop(pm);

        let result = vmm.brk(100, 0);
        assert_eq!(result, Err(VmmError::InvalidAddress));
    }

    // ── VMM: Shared Memory Tests ──────────────────────────────

    #[test]
    fn test_vmm_shm_create() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let shm_id = vmm.shm_create("test_shm", 0x4000, 100, PageFlags::shared_rw()).unwrap();
        assert!(shm_id > 0);
        assert_eq!(vmm.shm_count(), 1);
    }

    #[test]
    fn test_vmm_shm_attach() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        vmm.register_process(101).unwrap();

        let shm_id = vmm.shm_create("test_shm", 0x4000, 100, PageFlags::shared_rw()).unwrap();
        let addr = vmm.mmap(101, 0x4000, PageFlags::shared_rw()).unwrap();
        vmm.shm_attach(shm_id, 101, addr).unwrap();

        let shm = vmm.get_shm(shm_id).unwrap();
        assert_eq!(shm.attached_count(), 2);
    }

    #[test]
    fn test_vmm_shm_detach() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        vmm.register_process(101).unwrap();

        let shm_id = vmm.shm_create("test_shm", 0x4000, 100, PageFlags::shared_rw()).unwrap();
        let addr = vmm.mmap(101, 0x4000, PageFlags::shared_rw()).unwrap();
        vmm.shm_attach(shm_id, 101, addr).unwrap();

        vmm.shm_detach(shm_id, 101).unwrap();
        let shm = vmm.get_shm(shm_id).unwrap();
        assert_eq!(shm.attached_count(), 1);
    }

    #[test]
    fn test_vmm_shm_destroy() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let shm_id = vmm.shm_create("test_shm", 0x4000, 100, PageFlags::shared_rw()).unwrap();
        vmm.shm_destroy(shm_id).unwrap();
        assert_eq!(vmm.shm_count(), 0);
    }

    #[test]
    fn test_vmm_shm_not_found() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        assert_eq!(vmm.shm_destroy(999), Err(VmmError::ShmNotFound));
    }

    // ── VMM: Swap Tests ───────────────────────────────────────

    #[test]
    fn test_vmm_swap_out() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();

        // Map a page
        vmm.handle_page_fault(100, addr, false);
        vmm.touch_page(100, addr).unwrap();

        // Swap it out
        vmm.swap_out(100, addr).unwrap();
        let pm = vmm.get_process(100).unwrap();
        assert_eq!(pm.swapped_pages, 1);
        assert!(vmm.swap_count() > 0);
    }

    #[test]
    fn test_vmm_swap_in() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();

        vmm.handle_page_fault(100, addr, false);
        vmm.swap_out(100, addr).unwrap();

        // Swap back in (via page fault)
        let fault = vmm.handle_page_fault(100, addr, false);
        assert_eq!(fault.resolution, PageFaultResolution::SwappedIn);
    }

    #[test]
    fn test_vmm_swap_locked_page() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();

        vmm.handle_page_fault(100, addr, false);

        // Lock the page
        let pm = vmm.get_process_mut(100).unwrap();
        let entry = pm.get_page_mut(addr).unwrap();
        entry.flags.locked = true;
        drop(pm);

        let result = vmm.swap_out(100, addr);
        assert_eq!(result, Err(VmmError::PermissionDenied));
    }

    #[test]
    fn test_vmm_swap_not_present() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let result = vmm.swap_out(100, 0xDEAD);
        assert_eq!(result, Err(VmmError::PageNotPresent));
    }

    // ── VMM: Guard Page Tests ─────────────────────────────────

    #[test]
    fn test_vmm_setup_stack_guard() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        vmm.setup_stack_guard(100).unwrap();
        let pm = vmm.get_process(100).unwrap();
        assert!(pm.vmas.iter().any(|v| v.is_guard()));
    }

    #[test]
    fn test_vmm_setup_heap_guard() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        vmm.setup_heap_guard(100).unwrap();
        let pm = vmm.get_process(100).unwrap();
        assert!(pm.vmas.iter().any(|v| v.is_guard()));
    }

    // ── VMM: OOM Killer Tests ─────────────────────────────────

    #[test]
    fn test_vmm_oom_kill() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        vmm.register_process(101).unwrap();

        // Process 100 uses more memory
        let addr1 = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();
        vmm.handle_page_fault(100, addr1, false);
        vmm.handle_page_fault(100, addr1 + PAGE_SIZE, false);

        let addr2 = vmm.mmap(101, 0x10000, PageFlags::rw()).unwrap();
        vmm.handle_page_fault(101, addr2, false);

        let killed = vmm.oom_kill();
        assert_eq!(killed, Some(100));  // Largest consumer
        assert!(vmm.get_process(100).unwrap().oom_killed);
    }

    #[test]
    fn test_vmm_oom_no_processes() {
        let mut vmm = VirtualMemoryManager::new(1024);
        let result = vmm.oom_kill();
        assert_eq!(result, None);
    }

    // ── VMM: Stats Tests ──────────────────────────────────────

    #[test]
    fn test_vmm_stats() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        vmm.register_process(101).unwrap();

        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();
        vmm.handle_page_fault(100, addr, false);

        let stats = vmm.stats();
        assert_eq!(stats.total_processes, 2);
        assert!(stats.total_vmas > 0);
        assert!(stats.resident_pages > 0);
        assert_eq!(stats.total_faults, 1);
    }

    #[test]
    fn test_vmm_free_frames() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();
        vmm.handle_page_fault(100, addr, false);

        assert!(vmm.used_frames() > 0);
        assert!(vmm.free_frames() < 1024);
    }

    // ── VMM: Error Tests ──────────────────────────────────────

    #[test]
    fn test_vmm_error_names() {
        assert_eq!(VmmError::ProcessNotFound.name(), "process_not_found");
        assert_eq!(VmmError::OutOfMemory.name(), "out_of_memory");
        assert_eq!(VmmError::InvalidSize.name(), "invalid_size");
        assert_eq!(VmmError::ShmNotFound.name(), "shm_not_found");
        assert_eq!(VmmError::SwapFull.name(), "swap_full");
    }

    // ── Integration Tests ─────────────────────────────────────

    #[test]
    fn test_integration_full_vm_lifecycle() {
        let mut vmm = VirtualMemoryManager::new(4096);

        // 1. Register processes
        vmm.register_process(100).unwrap();
        vmm.register_process(101).unwrap();

        // 2. mmap anonymous region
        let addr1 = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();
        let addr2 = vmm.mmap(101, 0x10000, PageFlags::rx()).unwrap();

        // 3. Demand paging
        let fault1 = vmm.handle_page_fault(100, addr1, false);
        assert_eq!(fault1.resolution, PageFaultResolution::Handled);

        let fault2 = vmm.handle_page_fault(101, addr2, false);
        assert_eq!(fault2.resolution, PageFaultResolution::Handled);

        // 4. Fork with CoW
        vmm.register_process(102).unwrap();
        vmm.fork(100, 102).unwrap();

        // 5. Child CoW fault
        let cow_fault = vmm.handle_page_fault(102, addr1, true);
        assert_eq!(cow_fault.resolution, PageFaultResolution::CowCopied);

        // 6. Create shared memory
        let shm_id = vmm.shm_create("shared_buf", 0x8000, 100, PageFlags::shared_rw()).unwrap();
        let shm_addr = vmm.mmap(101, 0x8000, PageFlags::shared_rw()).unwrap();
        vmm.shm_attach(shm_id, 101, shm_addr).unwrap();

        // 7. brk expansion
        let pm = vmm.get_process(100).unwrap();
        let new_brk = vmm.brk(100, pm.heap_start + 0x20000).unwrap();
        assert!(new_brk > pm.heap_start);

        // 8. Guard pages
        vmm.setup_stack_guard(100).unwrap();

        // 9. Swap out a page
        vmm.swap_out(100, addr1).unwrap();

        // 10. Swap back in
        let swap_fault = vmm.handle_page_fault(100, addr1, false);
        assert_eq!(swap_fault.resolution, PageFaultResolution::SwappedIn);

        // 11. mprotect
        vmm.mprotect(100, addr1, PageFlags::ro()).unwrap();

        // 12. Stats
        let stats = vmm.stats();
        assert_eq!(stats.total_processes, 3);
        assert!(stats.total_faults > 0);
        assert!(stats.cow_faults > 0);
        assert!(stats.swap_outs > 0);
        assert!(stats.swap_ins > 0);
        assert_eq!(stats.shared_regions, 1);
    }

    #[test]
    fn test_integration_memory_pressure_oom() {
        let mut vmm = VirtualMemoryManager::new(16);  // Very limited memory

        vmm.register_process(100).unwrap();
        vmm.register_process(101).unwrap();

        // Process 100 uses most memory
        let addr1 = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();
        for i in 0..8 {
            vmm.handle_page_fault(100, addr1 + i * PAGE_SIZE, false);
        }

        // Process 101 uses less
        let addr2 = vmm.mmap(101, 0x10000, PageFlags::rw()).unwrap();
        vmm.handle_page_fault(101, addr2, false);

        // OOM should kill process 100 (largest consumer)
        let killed = vmm.oom_kill();
        assert_eq!(killed, Some(100));

        let stats = vmm.stats();
        assert!(stats.oom_kills > 0);
    }

    #[test]
    fn test_integration_fork_write_sequence() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();

        // Map several pages
        for i in 0..4 {
            vmm.handle_page_fault(100, addr + i * PAGE_SIZE, false);
        }

        // Fork
        vmm.register_process(101).unwrap();
        vmm.fork(100, 101).unwrap();

        // Child writes to all pages (CoW)
        for i in 0..4 {
            let fault = vmm.handle_page_fault(101, addr + i * PAGE_SIZE, true);
            assert_eq!(fault.resolution, PageFaultResolution::CowCopied);
        }

        // Parent also writes (should also CoW since ref_count > 1)
        let fault = vmm.handle_page_fault(100, addr, true);
        assert_eq!(fault.resolution, PageFaultResolution::CowCopied);

        let stats = vmm.stats();
        assert!(stats.cow_faults >= 5);
    }

    #[test]
    fn test_integration_shared_memory_ipc() {
        let mut vmm = VirtualMemoryManager::new(1024);
        vmm.register_process(100).unwrap();
        vmm.register_process(101).unwrap();
        vmm.register_process(102).unwrap();

        // Create shared memory
        let shm_id = vmm.shm_create("ipc_buffer", 0x4000, 100, PageFlags::shared_rw()).unwrap();

        // Attach to all processes
        let addr1 = vmm.mmap(100, 0x4000, PageFlags::shared_rw()).unwrap();
        vmm.shm_attach(shm_id, 100, addr1).unwrap();

        let addr2 = vmm.mmap(101, 0x4000, PageFlags::shared_rw()).unwrap();
        vmm.shm_attach(shm_id, 101, addr2).unwrap();

        let addr3 = vmm.mmap(102, 0x4000, PageFlags::shared_rw()).unwrap();
        vmm.shm_attach(shm_id, 102, addr3).unwrap();

        let shm = vmm.get_shm(shm_id).unwrap();
        assert_eq!(shm.attached_count(), 3);

        // Detach one
        vmm.shm_detach(shm_id, 102).unwrap();
        let shm = vmm.get_shm(shm_id).unwrap();
        assert_eq!(shm.attached_count(), 2);
    }

    #[test]
    fn test_integration_swap_lru_replacement() {
        let mut vmm = VirtualMemoryManager::new(4);  // Only 4 frames

        vmm.register_process(100).unwrap();
        let addr = vmm.mmap(100, 0x10000, PageFlags::rw()).unwrap();

        // Map 4 pages (fills memory)
        for i in 0..4 {
            let fault = vmm.handle_page_fault(100, addr + i * PAGE_SIZE, false);
            assert_eq!(fault.resolution, PageFaultResolution::Handled);
        }
        assert_eq!(vmm.free_frames(), 0);

        // Touch pages with different timestamps
        for i in 0..4 {
            vmm.touch_page(100, addr + i * PAGE_SIZE).unwrap();
        }
        // Touch page 0 and 1 more recently (they should be kept)
        vmm.touch_page(100, addr).unwrap();
        vmm.touch_page(100, addr + PAGE_SIZE).unwrap();

        // Map a 5th page — should trigger LRU swap
        let fault = vmm.handle_page_fault(100, addr + 4 * PAGE_SIZE, false);
        assert_eq!(fault.resolution, PageFaultResolution::Handled);

        // Some page should have been swapped out
        let stats = vmm.stats();
        assert!(stats.swap_outs > 0);
    }
}

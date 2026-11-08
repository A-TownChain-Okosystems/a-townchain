// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 32: Page Fault Handler + Demand Paging
// Copyright (c) Michael Wroblewski. All rights reserved.
//
// User-Space Memory Management: Page Faults, Demand Paging, Copy-on-Write,
// Virtual Memory Areas (VMA), mmap, Stack/Heap Growth.

use crate::ats1000::Pid;

// ═══════════════════════════════════════════════════════════════════════════════
// Page Fault Error Codes (x86-64)
// ═══════════════════════════════════════════════════════════════════════════════

/// Page fault error code bits (from CPU CR2 + error word)
const PF_PRESENT:  u32 = 1 << 0;  // Page was present
const PF_WRITE:    u32 = 1 << 1;  // Caused by write
const PF_USER:     u32 = 1 << 2;  // User-mode access
const PF_RESERVED: u32 = 1 << 3;  // Reserved bit set in page table
const PF_INSTR:    u32 = 1 << 4;  // Instruction fetch

/// Page fault cause classification
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PageFaultType {
    /// Page not present — demand paging needed
    NotPresent,
    /// Write to read-only page — possible CoW
    Protection,
    /// User-mode access to kernel page
    PrivilegeViolation,
    /// Reserved bit violation in page table
    ReservedBit,
    /// Instruction fetch from non-executable page (NX)
    ExecViolation,
    /// Unknown/undefined fault
    Unknown,
}

impl PageFaultType {
    pub fn is_fatal(&self) -> bool {
        matches!(self, PageFaultType::PrivilegeViolation | PageFaultType::ReservedBit | PageFaultType::Unknown)
    }
    pub fn can_demand_page(&self) -> bool {
        matches!(self, PageFaultType::NotPresent)
    }
    pub fn can_cow(&self) -> bool {
        matches!(self, PageFaultType::Protection)
    }
}

impl core::fmt::Display for PageFaultType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            PageFaultType::NotPresent        => write!(f, "page not present"),
            PageFaultType::Protection         => write!(f, "protection violation (CoW?)"),
            PageFaultType::PrivilegeViolation => write!(f, "privilege violation"),
            PageFaultType::ReservedBit        => write!(f, "reserved bit set"),
            PageFaultType::ExecViolation      => write!(f, "NX violation"),
            PageFaultType::Unknown            => write!(f, "unknown page fault"),
        }
    }
}

/// Decoded page fault info from CPU
#[derive(Clone, Copy, Debug)]
pub struct PageFaultInfo {
    pub fault_addr:   u64,   // CR2 — faulting virtual address
    pub error_code:    u32,
    pub is_write:      bool,
    pub is_user:       bool,
    pub is_present:    bool,
    pub is_instr:      bool,
    pub fault_type:    PageFaultType,
}

impl PageFaultInfo {
    pub fn from_registers(cr2: u64, error_code: u32) -> Self {
        let is_write   = (error_code & PF_WRITE)    != 0;
        let is_user    = (error_code & PF_USER)     != 0;
        let is_present = (error_code & PF_PRESENT)   != 0;
        let is_instr   = (error_code & PF_INSTR)    != 0;
        let reserved   = (error_code & PF_RESERVED)  != 0;

        let fault_type = if reserved {
            PageFaultType::ReservedBit
        } else if is_instr && !is_present {
            PageFaultType::ExecViolation
        } else if is_user && !is_present {
            PageFaultType::NotPresent
        } else if is_present && is_write {
            PageFaultType::Protection
        } else if is_user && is_present {
            PageFaultType::PrivilegeViolation
        } else if !is_present {
            PageFaultType::NotPresent
        } else {
            PageFaultType::Unknown
        };

        Self {
            fault_addr: cr2,
            error_code,
            is_write, is_user, is_present, is_instr,
            fault_type,
        }
    }

    pub fn is_user_fault(&self) -> bool { self.is_user }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Virtual Memory Areas (VMA)
// ═══════════════════════════════════════════════════════════════════════════════

/// VMA permissions
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VmaFlags {
    pub read:     bool,
    pub write:    bool,
    pub execute:  bool,
    pub cow:      bool,   // Copy-on-Write
}

impl VmaFlags {
    pub const fn rw()    -> Self { Self { read: true, write: true, execute: false, cow: false } }
    pub const fn rx()    -> Self { Self { read: true, write: false, execute: true, cow: false } }
    pub const fn rwx()   -> Self { Self { read: true, write: true, execute: true, cow: false } }
    pub const fn ro()    -> Self { Self { read: true, write: false, execute: false, cow: false } }
    pub const fn cow()   -> Self { Self { read: true, write: false, execute: false, cow: true } }
}

/// A virtual memory area (like Linux VMA)
#[derive(Clone, Debug)]
pub struct VirtualMemoryArea {
    pub start:   u64,
    pub end:     u64,
    pub flags:   VmaFlags,
    pub vma_type: VmaType,
    pub backing: Option<BackingStore>,
}

impl VirtualMemoryArea {
    pub fn new(start: u64, end: u64, flags: VmaFlags, vma_type: VmaType) -> Self {
        Self { start, end, flags, vma_type, backing: None }
    }
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.start && addr < self.end
    }
    pub fn size(&self) -> u64 {
        self.end - self.start
    }
    pub fn pages(&self, page_size: u64) -> u64 {
        (self.size() + page_size - 1) / page_size
    }
    pub fn allows_write(&self) -> bool { self.flags.write }
    pub fn allows_read(&self) -> bool { self.flags.read }
    pub fn allows_exec(&self) -> bool { self.flags.execute }
    pub fn is_cow(&self) -> bool { self.flags.cow }
}

/// VMA type
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VmaType {
    Code,
    Data,
    Stack,
    Heap,
    Mmap,       // mmap'd region
    Shared,     // shared memory
}

impl VmaType {
    pub fn name(&self) -> &'static str {
        match self {
            VmaType::Code  => "code",
            VmaType::Data  => "data",
            VmaType::Stack => "stack",
            VmaType::Heap  => "heap",
            VmaType::Mmap  => "mmap",
            VmaType::Shared => "shared",
        }
    }
    pub fn can_grow(&self) -> bool {
        matches!(self, VmaType::Stack | VmaType::Heap | VmaType::Mmap)
    }
}

/// Backing store for demand paging
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackingStore {
    /// Backed by a file (offset in file)
    File { file_id: u64, offset: u64 },
    /// Backed by swap space
    Swap { swap_slot: u64 },
    /// Anonymous (zero-filled on demand)
    Anonymous,
    /// Backed by physical frame (for CoW)
    Physical { frame: u64 },
}

// ═══════════════════════════════════════════════════════════════════════════════
// Page Frame Allocator (simulated)
// ═══════════════════════════════════════════════════════════════════════════════

const DEFAULT_PAGE_SIZE: u64 = 4096;
const MAX_FRAMES: usize = 16384; // 64 MiB of physical frames

pub struct FrameAllocator {
    frames: Vec<bool>,  // true = allocated
    next_frame: usize,
    total_allocated: u64,
    total_freed: u64,
}

impl Default for FrameAllocator {
    fn default() -> Self { Self::new() }
}

impl FrameAllocator {
    pub fn new() -> Self {
        Self {
            frames: vec![false; MAX_FRAMES],
            next_frame: 0,
            total_allocated: 0,
            total_freed: 0,
        }
    }

    pub fn alloc(&mut self) -> Option<u64> {
        // Linear scan for free frame
        for i in 0..MAX_FRAMES {
            let idx = (self.next_frame + i) % MAX_FRAMES;
            if !self.frames[idx] {
                self.frames[idx] = true;
                self.next_frame = idx + 1;
                self.total_allocated += 1;
                return Some(idx as u64);
            }
        }
        None
    }

    pub fn alloc_contiguous(&mut self, count: usize) -> Option<u64> {
        // Find count contiguous free frames
        let mut run_start = None;
        let mut run_len = 0;
        for i in 0..MAX_FRAMES {
            if !self.frames[i] {
                if run_start.is_none() { run_start = Some(i); }
                run_len += 1;
                if run_len >= count {
                    let start = run_start.unwrap();
                    for j in start..start + count {
                        self.frames[j] = true;
                    }
                    self.total_allocated += count as u64;
                    return Some(start as u64);
                }
            } else {
                run_start = None;
                run_len = 0;
            }
        }
        None
    }

    pub fn free(&mut self, frame: u64) -> bool {
        let idx = frame as usize;
        if idx < MAX_FRAMES && self.frames[idx] {
            self.frames[idx] = false;
            self.total_freed += 1;
            true
        } else {
            false
        }
    }

    pub fn free_range(&mut self, start: u64, count: usize) -> usize {
        let mut freed = 0;
        for i in 0..count {
            if self.free(start + i as u64) { freed += 1; }
        }
        freed
    }

    pub fn is_allocated(&self, frame: u64) -> bool {
        let idx = frame as usize;
        idx < MAX_FRAMES && self.frames[idx]
    }

    pub fn allocated_count(&self) -> usize {
        self.frames.iter().filter(|&&f| f).count()
    }
    pub fn free_count(&self) -> usize {
        self.frames.iter().filter(|&&f| !f).count()
    }
    pub fn total_allocated(&self) -> u64 { self.total_allocated }
    pub fn total_freed(&self) -> u64 { self.total_freed }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Page Table Entry (simulated)
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PageTableEntry {
    pub frame:       u64,
    pub present:     bool,
    pub writable:    bool,
    pub executable:  bool,
    pub user:        bool,
    pub cow:         bool,
    pub dirty:       bool,
    pub accessed:    bool,
}

impl PageTableEntry {
    pub fn new(frame: u64, writable: bool, executable: bool, user: bool) -> Self {
        Self {
            frame, present: true, writable, executable, user,
            cow: false, dirty: false, accessed: false,
        }
    }

    pub fn absent() -> Self {
        Self { frame: 0, present: false, writable: false, executable: false,
               user: false, cow: false, dirty: false, accessed: false }
    }

    pub fn make_cow(&mut self) {
        self.cow = true;
        self.writable = false;
    }

    pub fn is_present(&self) -> bool { self.present }
    pub fn is_cow(&self) -> bool { self.cow }
}

// ═══════════════════════════════════════════════════════════════════════════════
// User Memory Manager
// ═══════════════════════════════════════════════════════════════════════════════

/// Per-process address space with page tables and VMAs
pub struct ProcessAddressSpace {
    pub pid:        Pid,
    pub page_table:  Vec<(u64, PageTableEntry)>,  // (vaddr, PTE)
    pub vmas:        Vec<VirtualMemoryArea>,
    pub page_size:   u64,
}

impl ProcessAddressSpace {
    pub fn new(pid: Pid) -> Self {
        Self {
            pid,
            page_table: Vec::new(),
            vmas: Vec::new(),
            page_size: DEFAULT_PAGE_SIZE,
        }
    }

    pub fn add_vma(&mut self, vma: VirtualMemoryArea) {
        self.vmas.push(vma);
    }

    pub fn find_vma(&self, addr: u64) -> Option<&VirtualMemoryArea> {
        self.vmas.iter().find(|v| v.contains(addr))
    }

    pub fn find_vma_mut(&mut self, addr: u64) -> Option<&mut VirtualMemoryArea> {
        self.vmas.iter_mut().find(|v| v.contains(addr))
    }

    pub fn map_page(&mut self, vaddr: u64, frame: u64, writable: bool, executable: bool) {
        let pte = PageTableEntry::new(frame, writable, executable, true);
        // Remove existing entry if present
        self.page_table.retain(|(v, _)| *v != vaddr);
        self.page_table.push((vaddr, pte));
    }

    pub fn unmap_page(&mut self, vaddr: u64) -> Option<PageTableEntry> {
        let idx = self.page_table.iter().position(|(v, _)| *v == vaddr)?;
        Some(self.page_table.remove(idx).1)
    }

    pub fn get_pte(&self, vaddr: u64) -> Option<&PageTableEntry> {
        self.page_table.iter().find(|(v, _)| *v == vaddr).map(|(_, p)| p)
    }

    pub fn get_pte_mut(&mut self, vaddr: u64) -> Option<&mut PageTableEntry> {
        self.page_table.iter_mut().find(|(v, _)| *v == vaddr).map(|(_, p)| p)
    }

    pub fn mapped_pages(&self) -> usize { self.page_table.len() }
    pub fn vma_count(&self) -> usize { self.vmas.len() }

    /// Grow a VMA (stack or heap) to accommodate a faulting address
    pub fn grow_vma(&mut self, vma_idx: usize, new_end: u64) -> bool {
        if vma_idx >= self.vmas.len() { return false; }
        if !self.vmas[vma_idx].vma_type.can_grow() { return false; }
        self.vmas[vma_idx].end = new_end;
        true
    }

    /// Find VMA index by address
    pub fn find_vma_index(&self, addr: u64) -> Option<usize> {
        self.vmas.iter().position(|v| v.contains(addr))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Page Fault Handler
// ═══════════════════════════════════════════════════════════════════════════════

/// Result of handling a page fault
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FaultResult {
    /// Page was demand-paged in — retry the faulting instruction
    Fixed,
    /// CoW page was copied — retry
    CowFixed,
    /// VMA was grown (stack/heap growth) — retry
    Grown,
    /// Page was mapped for the first time
    Mapped,
    /// Fatal fault — deliver SIGSEGV to process
    Segfault,
    /// Permission denied — deliver SIGSEGV
    PermissionDenied,
    /// No VMA for this address — deliver SIGSEGV
    NoVma,
    /// Out of physical memory
    OutOfMemory,
}

impl core::fmt::Display for FaultResult {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            FaultResult::Fixed           => write!(f, "fixed (demand paged)"),
            FaultResult::CowFixed        => write!(f, "fixed (CoW)"),
            FaultResult::Grown           => write!(f, "fixed (VMA grown)"),
            FaultResult::Mapped          => write!(f, "fixed (mapped)"),
            FaultResult::Segfault        => write!(f, "SIGSEGV"),
            FaultResult::PermissionDenied => write!(f, "permission denied"),
            FaultResult::NoVma           => write!(f, "no VMA"),
            FaultResult::OutOfMemory     => write!(f, "out of memory"),
        }
    }
}

/// Statistics for page fault handler
#[derive(Clone, Copy, Debug, Default)]
pub struct FaultStats {
    pub total_faults:       u64,
    pub demand_pages:       u64,
    pub cow_faults:          u64,
    pub growth_faults:      u64,
    pub segfaults:          u64,
    pub permission_faults:  u64,
    pub oom_faults:         u64,
}

/// The main page fault handler
pub struct PageFaultHandler {
    frame_alloc: FrameAllocator,
    stats:      FaultStats,
}

impl Default for PageFaultHandler {
    fn default() -> Self { Self::new() }
}

impl PageFaultHandler {
    pub fn new() -> Self {
        Self {
            frame_alloc: FrameAllocator::new(),
            stats: FaultStats::default(),
        }
    }

    /// Handle a page fault for a process
    pub fn handle_fault(
        &mut self,
        fault:   &PageFaultInfo,
        space:   &mut ProcessAddressSpace,
    ) -> FaultResult {
        self.stats.total_faults += 1;

        // Only handle user-mode faults
        if !fault.is_user {
            self.stats.segfaults += 1;
            return FaultResult::Segfault;
        }

        let addr = fault.fault_addr;

        // Find the VMA for this address
        let vma_idx = match space.find_vma_index(addr) {
            Some(idx) => idx,
            None => {
                // Check if it's a stack growth (address just below current stack VMA)
                if let Some(stack_idx) = space.vmas.iter().position(|v| v.vma_type == VmaType::Stack) {
                    let stack = &space.vmas[stack_idx];
                    if addr >= stack.start - space.page_size * 64 && addr < stack.start {
                        // Stack growth — extend the stack VMA
                        space.grow_vma(stack_idx, stack.end); // Keep end, lower start
                        if let Some(v) = space.vmas.get_mut(stack_idx) {
                            v.start = addr & !(space.page_size - 1); // Page-align
                        }
                        self.stats.growth_faults += 1;
                        return self.demand_page(space, addr, VmaFlags::rw());
                    }
                }
                self.stats.segfaults += 1;
                return FaultResult::NoVma;
            }
        };

        let vma = &space.vmas[vma_idx];

        // Check permissions
        if fault.is_write && !vma.allows_write() && !vma.is_cow() {
            self.stats.permission_faults += 1;
            return FaultResult::PermissionDenied;
        }
        if !fault.is_write && !vma.allows_read() {
            self.stats.permission_faults += 1;
            return FaultResult::PermissionDenied;
        }
        if fault.is_instr && !vma.allows_exec() {
            self.stats.permission_faults += 1;
            return FaultResult::PermissionDenied;
        }

        // Handle based on fault type
        match fault.fault_type {
            PageFaultType::NotPresent => {
                // Demand page: allocate frame and map it
                self.stats.demand_pages += 1;
                self.demand_page(space, addr, vma.flags)
            }
            PageFaultType::Protection => {
                // Possible CoW
                if vma.is_cow() || self.is_cow_pte(space, addr) {
                    self.stats.cow_faults += 1;
                    self.handle_cow(space, addr, vma.flags)
                } else {
                    self.stats.permission_faults += 1;
                    FaultResult::PermissionDenied
                }
            }
            _ => {
                self.stats.segfaults += 1;
                FaultResult::Segfault
            }
        }
    }

    /// Demand page: allocate a physical frame and map it
    fn demand_page(&mut self, space: &mut ProcessAddressSpace, addr: u64, flags: VmaFlags) -> FaultResult {
        let page_addr = addr & !(space.page_size - 1); // Page-align down

        // Check if already mapped (shouldn't be, but be safe)
        if space.get_pte(page_addr).map(|p| p.present).unwrap_or(false) {
            return FaultResult::Fixed;
        }

        // Allocate a frame
        let frame = match self.frame_alloc.alloc() {
            Some(f) => f,
            None => {
                self.stats.oom_faults += 1;
                return FaultResult::OutOfMemory;
            }
        };

        // Map the page
        space.map_page(page_addr, frame, flags.write, flags.execute);

        // If it's a CoW VMA, mark the PTE as CoW
        if flags.cow {
            if let Some(pte) = space.get_pte_mut(page_addr) {
                pte.make_cow();
            }
        }

        FaultResult::Mapped
    }

    /// Handle Copy-on-Write fault
    fn handle_cow(&mut self, space: &mut ProcessAddressSpace, addr: u64, _flags: VmaFlags) -> FaultResult {
        let page_addr = addr & !(space.page_size - 1);

        // Get the old PTE
        let old_pte = match space.get_pte(page_addr) {
            Some(p) => *p,
            None => return FaultResult::NoVma,
        };

        // Allocate a new frame
        let new_frame = match self.frame_alloc.alloc() {
            Some(f) => f,
            None => {
                self.stats.oom_faults += 1;
                return FaultResult::OutOfMemory;
            }
        };

        // Create new PTE with write access (CoW resolved)
        let new_pte = PageTableEntry {
            frame: new_frame,
            present: true,
            writable: true,
            executable: old_pte.executable,
            user: true,
            cow: false,
            dirty: true,
            accessed: true,
        };

        // Replace the old PTE
        space.page_table.retain(|(v, _)| *v != page_addr);
        space.page_table.push((page_addr, new_pte));

        FaultResult::CowFixed
    }

    /// Check if a PTE is marked as CoW
    fn is_cow_pte(&self, space: &ProcessAddressSpace, addr: u64) -> bool {
        space.get_pte(addr).map(|p| p.cow).unwrap_or(false)
    }

    /// mmap: create a new VMA and map pages
    pub fn mmap(
        &mut self,
        space:    &mut ProcessAddressSpace,
        addr:     u64,
        length:   u64,
        flags:    VmaFlags,
    ) -> Result<u64, FaultResult> {
        let page_size = space.page_size;
        let aligned_addr = if addr == 0 {
            // Find a free region
            self.find_free_region(space, length)
                .ok_or(FaultResult::OutOfMemory)?
        } else {
            addr & !(page_size - 1)
        };

        let end = aligned_addr + ((length + page_size - 1) & !(page_size - 1));

        // Check for overlap with existing VMAs
        for vma in &space.vmas {
            if aligned_addr < vma.end && end > vma.start {
                return Err(FaultResult::PermissionDenied);
            }
        }

        // Create VMA
        let vma = VirtualMemoryArea {
            start: aligned_addr,
            end,
            flags,
            vma_type: VmaType::Mmap,
            backing: Some(BackingStore::Anonymous),
        };
        space.add_vma(vma);

        Ok(aligned_addr)
    }

    /// munmap: remove a VMA and unmap its pages
    pub fn munmap(&mut self, space: &mut ProcessAddressSpace, addr: u64, length: u64) -> bool {
        let page_size = space.page_size;
        let aligned_addr = addr & !(page_size - 1);
        let end = aligned_addr + ((length + page_size - 1) & !(page_size - 1));

        // Find and remove matching VMA
        let idx = space.vmas.iter().position(|v| v.start == aligned_addr && v.end >= end);
        if let Some(i) = idx {
            // Unmap all pages in this VMA
            let (start, end) = (space.vmas[i].start, space.vmas[i].end);
            space.page_table.retain(|(vaddr, pte)| {
                if *vaddr >= start && *vaddr < end {
                    self.frame_alloc.free(pte.frame);
                    false
                } else {
                    true
                }
            });
            space.vmas.remove(i);
            true
        } else {
            false
        }
    }

    /// Find a free virtual address region
    fn find_free_region(&self, space: &ProcessAddressSpace, length: u64) -> Option<u64> {
        let page_size = space.page_size;
        let aligned_len = (length + page_size - 1) & !(page_size - 1);

        // Start searching from 0x10000000 (256 MiB)
        let mut search_start = 0x10000000u64;

        for vma in &space.vmas {
            if vma.start >= search_start + aligned_len {
                return Some(search_start);
            }
            search_start = vma.end;
        }

        // If no VMAs overlap, use search_start
        if search_start + aligned_len <= 0x7FFF0000 {
            Some(search_start)
        } else {
            None
        }
    }

    /// fork(): create a copy of address space with CoW pages
    pub fn fork_address_space(
        &mut self,
        parent: &ProcessAddressSpace,
        child_pid: Pid,
    ) -> ProcessAddressSpace {
        let mut child = ProcessAddressSpace::new(child_pid);
        child.page_size = parent.page_size;

        // Copy VMAs
        for vma in &parent.vmas {
            child.add_vma(vma.clone());
        }

        // Copy page table entries, marking all writable pages as CoW
        for (vaddr, pte) in &parent.page_table {
            let mut child_pte = *pte;
            if pte.writable {
                child_pte.make_cow();
            }
            child.page_table.push((*vaddr, child_pte));
        }

        // Also mark parent's writable pages as CoW
        // (In a real OS, we'd modify parent's PTEs too)

        child
    }

    /// Statistics
    pub fn stats(&self) -> FaultStats { self.stats }
    pub fn frame_allocator(&self) -> &FrameAllocator { &self.frame_alloc }
    pub fn frame_allocator_mut(&mut self) -> &mut FrameAllocator { &mut self.frame_alloc }

    /// Create a default address space for a new user process
    pub fn create_user_space(&mut self, pid: Pid, code_start: u64, code_size: u64, stack_top: u64) -> ProcessAddressSpace {
        let mut space = ProcessAddressSpace::new(pid);
        let ps = space.page_size;

        // Code VMA (r-x)
        space.add_vma(VirtualMemoryArea::new(
            code_start,
            code_start + code_size,
            VmaFlags::rx(),
            VmaType::Code,
        ));

        // Heap VMA (rw-, grows)
        space.add_vma(VirtualMemoryArea::new(
            code_start + code_size,
            code_start + code_size + 0x200000, // 2 MiB initial heap
            VmaFlags::rw(),
            VmaType::Heap,
        ));

        // Stack VMA (rw-, grows down)
        space.add_vma(VirtualMemoryArea::new(
            stack_top - 0x10000, // 64 KiB initial stack
            stack_top,
            VmaFlags::rw(),
            VmaType::Stack,
        ));

        // Map initial code page
        let code_frame = self.frame_alloc.alloc().unwrap_or(0);
        space.map_page(code_start & !(ps - 1), code_frame, false, true);

        // Map initial stack page
        let stack_frame = self.frame_alloc.alloc().unwrap_or(1);
        space.map_page((stack_top - ps) & !(ps - 1), stack_frame, true, false);

        space
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // --- PageFaultInfo tests ---

    #[test]
    fn test_fault_not_present_user() {
        let info = PageFaultInfo::from_registers(0x400000, PF_USER);
        assert!(info.is_user);
        assert!(!info.is_present);
        assert!(info.is_user_fault());
        assert_eq!(info.fault_type, PageFaultType::NotPresent);
        assert!(info.fault_type.can_demand_page());
    }

    #[test]
    fn test_fault_protection_cow() {
        let info = PageFaultInfo::from_registers(0x500000, PF_USER | PF_WRITE | PF_PRESENT);
        assert!(info.is_write);
        assert!(info.is_present);
        assert_eq!(info.fault_type, PageFaultType::Protection);
        assert!(info.fault_type.can_cow());
    }

    #[test]
    fn test_fault_reserved_bit() {
        let info = PageFaultInfo::from_registers(0x600000, PF_USER | PF_RESERVED);
        assert_eq!(info.fault_type, PageFaultType::ReservedBit);
        assert!(info.fault_type.is_fatal());
    }

    #[test]
    fn test_fault_privilege_violation() {
        let info = PageFaultInfo::from_registers(0x800000, PF_USER | PF_PRESENT);
        assert_eq!(info.fault_type, PageFaultType::PrivilegeViolation);
        assert!(info.fault_type.is_fatal());
    }

    #[test]
    fn test_fault_type_display() {
        assert_eq!(format!("{}", PageFaultType::NotPresent), "page not present");
        assert_eq!(format!("{}", PageFaultType::Protection), "protection violation (CoW?)");
        assert_eq!(format!("{}", PageFaultType::ReservedBit), "reserved bit set");
    }

    // --- VMA tests ---

    #[test]
    fn test_vma_contains() {
        let vma = VirtualMemoryArea::new(0x400000, 0x500000, VmaFlags::rx(), VmaType::Code);
        assert!(vma.contains(0x400000));
        assert!(vma.contains(0x499999));
        assert!(!vma.contains(0x500000));
        assert!(!vma.contains(0x3FFFFF));
    }

    #[test]
    fn test_vma_size_and_pages() {
        let vma = VirtualMemoryArea::new(0x400000, 0x404000, VmaFlags::rx(), VmaType::Code);
        assert_eq!(vma.size(), 0x4000);
        assert_eq!(vma.pages(4096), 4);
    }

    #[test]
    fn test_vma_flags() {
        assert!(VmaFlags::rw().write);
        assert!(!VmaFlags::rw().execute);
        assert!(VmaFlags::rx().execute);
        assert!(!VmaFlags::rx().write);
        assert!(VmaFlags::rwx().write && VmaFlags::rwx().execute);
        assert!(!VmaFlags::ro().write);
        assert!(VmaFlags::cow().cow);
        assert!(!VmaFlags::cow().write); // CoW is read-only initially
    }

    #[test]
    fn test_vma_type_can_grow() {
        assert!(VmaType::Stack.can_grow());
        assert!(VmaType::Heap.can_grow());
        assert!(VmaType::Mmap.can_grow());
        assert!(!VmaType::Code.can_grow());
        assert!(!VmaType::Data.can_grow());
    }

    #[test]
    fn test_vma_type_names() {
        assert_eq!(VmaType::Code.name(), "code");
        assert_eq!(VmaType::Stack.name(), "stack");
        assert_eq!(VmaType::Heap.name(), "heap");
        assert_eq!(VmaType::Mmap.name(), "mmap");
    }

    // --- BackingStore tests ---

    #[test]
    fn test_backing_store_types() {
        let anon = BackingStore::Anonymous;
        let file = BackingStore::File { file_id: 1, offset: 0 };
        let swap = BackingStore::Swap { swap_slot: 42 };
        let phys = BackingStore::Physical { frame: 100 };

        assert_eq!(anon, BackingStore::Anonymous);
        assert_ne!(anon, file);
        assert_eq!(file, BackingStore::File { file_id: 1, offset: 0 });
        assert_eq!(swap, BackingStore::Swap { swap_slot: 42 });
        assert_eq!(phys, BackingStore::Physical { frame: 100 });
    }

    // --- FrameAllocator tests ---

    #[test]
    fn test_frame_alloc_basic() {
        let mut alloc = FrameAllocator::new();
        let f1 = alloc.alloc();
        assert!(f1.is_some());
        let f2 = alloc.alloc();
        assert!(f2.is_some());
        assert_ne!(f1.unwrap(), f2.unwrap());
        assert_eq!(alloc.allocated_count(), 2);
    }

    #[test]
    fn test_frame_alloc_free() {
        let mut alloc = FrameAllocator::new();
        let f = alloc.alloc().unwrap();
        assert!(alloc.is_allocated(f));
        assert!(alloc.free(f));
        assert!(!alloc.is_allocated(f));
        assert_eq!(alloc.total_freed(), 1);
    }

    #[test]
    fn test_frame_alloc_contiguous() {
        let mut alloc = FrameAllocator::new();
        let start = alloc.alloc_contiguous(4);
        assert!(start.is_some());
        let start = start.unwrap();
        for i in 0..4 {
            assert!(alloc.is_allocated(start + i));
        }
        assert_eq!(alloc.allocated_count(), 4);
    }

    #[test]
    fn test_frame_alloc_free_range() {
        let mut alloc = FrameAllocator::new();
        let start = alloc.alloc_contiguous(8).unwrap();
        let freed = alloc.free_range(start, 8);
        assert_eq!(freed, 8);
        assert_eq!(alloc.allocated_count(), 0);
    }

    #[test]
    fn test_frame_alloc_double_free() {
        let mut alloc = FrameAllocator::new();
        let f = alloc.alloc().unwrap();
        assert!(alloc.free(f));
        assert!(!alloc.free(f)); // Second free fails
    }

    #[test]
    fn test_frame_alloc_stats() {
        let mut alloc = FrameAllocator::new();
        alloc.alloc();
        alloc.alloc();
        alloc.alloc();
        assert_eq!(alloc.total_allocated(), 3);
        let f = alloc.alloc().unwrap();
        alloc.free(f);
        assert_eq!(alloc.total_allocated(), 4);
        assert_eq!(alloc.total_freed(), 1);
    }

    #[test]
    fn test_frame_free_count() {
        let mut alloc = FrameAllocator::new();
        let initial_free = alloc.free_count();
        alloc.alloc();
        assert_eq!(alloc.free_count(), initial_free - 1);
    }

    // --- PageTableEntry tests ---

    #[test]
    fn test_pte_new() {
        let pte = PageTableEntry::new(42, true, false, true);
        assert!(pte.is_present());
        assert_eq!(pte.frame, 42);
        assert!(pte.writable);
        assert!(!pte.executable);
        assert!(pte.user);
        assert!(!pte.is_cow());
    }

    #[test]
    fn test_pte_absent() {
        let pte = PageTableEntry::absent();
        assert!(!pte.is_present());
        assert!(!pte.is_cow());
    }

    #[test]
    fn test_pte_make_cow() {
        let mut pte = PageTableEntry::new(10, true, false, true);
        assert!(pte.writable);
        assert!(!pte.is_cow());
        pte.make_cow();
        assert!(pte.is_cow());
        assert!(!pte.writable); // CoW pages are read-only
    }

    // --- ProcessAddressSpace tests ---

    #[test]
    fn test_address_space_new() {
        let space = ProcessAddressSpace::new(Pid(1000));
        assert_eq!(space.pid, Pid(1000));
        assert_eq!(space.mapped_pages(), 0);
        assert_eq!(space.vma_count(), 0);
        assert_eq!(space.page_size, DEFAULT_PAGE_SIZE);
    }

    #[test]
    fn test_add_and_find_vma() {
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x400000, 0x500000, VmaFlags::rx(), VmaType::Code));
        assert_eq!(space.vma_count(), 1);
        let vma = space.find_vma(0x450000);
        assert!(vma.is_some());
        assert_eq!(vma.unwrap().vma_type, VmaType::Code);
        assert!(space.find_vma(0x600000).is_none());
    }

    #[test]
    fn test_map_and_get_pte() {
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.map_page(0x400000, 42, true, false);
        assert_eq!(space.mapped_pages(), 1);
        let pte = space.get_pte(0x400000);
        assert!(pte.is_some());
        assert_eq!(pte.unwrap().frame, 42);
        assert!(pte.unwrap().writable);
    }

    #[test]
    fn test_unmap_page() {
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.map_page(0x400000, 42, true, false);
        let removed = space.unmap_page(0x400000);
        assert!(removed.is_some());
        assert_eq!(space.mapped_pages(), 0);
        assert!(space.get_pte(0x400000).is_none());
    }

    #[test]
    fn test_grow_vma() {
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x600000, 0x610000, VmaFlags::rw(), VmaType::Heap));
        assert!(space.grow_vma(0, 0x620000));
        assert_eq!(space.vmas[0].end, 0x620000);
    }

    #[test]
    fn test_grow_code_vma_fails() {
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x400000, 0x500000, VmaFlags::rx(), VmaType::Code));
        assert!(!space.grow_vma(0, 0x600000)); // Code can't grow
    }

    #[test]
    fn test_find_vma_index() {
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x400000, 0x500000, VmaFlags::rx(), VmaType::Code));
        space.add_vma(VirtualMemoryArea::new(0x600000, 0x700000, VmaFlags::rw(), VmaType::Heap));
        assert_eq!(space.find_vma_index(0x450000), Some(0));
        assert_eq!(space.find_vma_index(0x650000), Some(1));
        assert_eq!(space.find_vma_index(0x550000), None);
    }

    // --- PageFaultHandler tests ---

    #[test]
    fn test_handler_new() {
        let handler = PageFaultHandler::new();
        assert_eq!(handler.stats().total_faults, 0);
        assert_eq!(handler.frame_allocator().allocated_count(), 0);
    }

    #[test]
    fn test_demand_page_fault() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x400000, 0x500000, VmaFlags::rx(), VmaType::Code));

        let fault = PageFaultInfo::from_registers(0x400000, PF_USER); // Not present, user
        let result = handler.handle_fault(&fault, &mut space);
        assert_eq!(result, FaultResult::Mapped);
        assert!(space.get_pte(0x400000).is_some());
        assert!(space.get_pte(0x400000).unwrap().is_present());
        assert_eq!(handler.stats().demand_pages, 1);
    }

    #[test]
    fn test_fault_no_vma() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));

        let fault = PageFaultInfo::from_registers(0x900000, PF_USER);
        let result = handler.handle_fault(&fault, &mut space);
        assert_eq!(result, FaultResult::NoVma);
        assert_eq!(handler.stats().segfaults, 1);
    }

    #[test]
    fn test_fault_permission_denied_write() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x400000, 0x500000, VmaFlags::ro(), VmaType::Code));

        let fault = PageFaultInfo::from_registers(0x400000, PF_USER | PF_WRITE | PF_PRESENT);
        let result = handler.handle_fault(&fault, &mut space);
        assert_eq!(result, FaultResult::PermissionDenied);
        assert_eq!(handler.stats().permission_faults, 1);
    }

    #[test]
    fn test_cow_fault() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x500000, 0x600000, VmaFlags::cow(), VmaType::Data));

        // First, demand-page it in
        let fault1 = PageFaultInfo::from_registers(0x500000, PF_USER);
        let r1 = handler.handle_fault(&fault1, &mut space);
        assert_eq!(r1, FaultResult::Mapped);

        // Now trigger CoW by writing
        let fault2 = PageFaultInfo::from_registers(0x500000, PF_USER | PF_WRITE | PF_PRESENT);
        let r2 = handler.handle_fault(&fault2, &mut space);
        assert_eq!(r2, FaultResult::CowFixed);

        // PTE should now be writable and not CoW
        let pte = space.get_pte(0x500000).unwrap();
        assert!(pte.writable);
        assert!(!pte.is_cow());
        assert_eq!(handler.stats().cow_faults, 1);
    }

    #[test]
    fn test_stack_growth() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x7FFFE000, 0x7FFFF000, VmaFlags::rw(), VmaType::Stack));

        // Access just below current stack start
        let fault = PageFaultInfo::from_registers(0x7FFFD000, PF_USER);
        let result = handler.handle_fault(&fault, &mut space);
        assert_eq!(result, FaultResult::Mapped);
        assert_eq!(handler.stats().growth_faults, 1);
        // Stack VMA should have grown
        let stack_vma = space.find_vma(0x7FFFD000).unwrap();
        assert_eq!(stack_vma.vma_type, VmaType::Stack);
    }

    #[test]
    fn test_kernel_fault_ignored() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x400000, 0x500000, VmaFlags::rx(), VmaType::Code));

        // Kernel-mode fault (no PF_USER bit)
        let fault = PageFaultInfo::from_registers(0x400000, 0);
        let result = handler.handle_fault(&fault, &mut space);
        assert_eq!(result, FaultResult::Segfault);
    }

    #[test]
    fn test_mmap_basic() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));

        let addr = handler.mmap(&mut space, 0, 0x10000, VmaFlags::rw());
        assert!(addr.is_ok());
        let addr = addr.unwrap();
        assert_eq!(addr, 0x10000000); // Default free region
        assert_eq!(space.vma_count(), 1);
        assert_eq!(space.vmas[0].vma_type, VmaType::Mmap);
        assert!(space.vmas[0].flags.write);
    }

    #[test]
    fn test_mmap_fixed_address() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));

        let addr = handler.mmap(&mut space, 0x20000000, 0x4000, VmaFlags::rw());
        assert_eq!(addr, Ok(0x20000000));
        assert_eq!(space.vmas[0].start, 0x20000000);
        assert_eq!(space.vmas[0].end, 0x20004000);
    }

    #[test]
    fn test_mmap_overlap_rejected() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));

        let _r = handler.mmap(&mut space, 0x20000000, 0x4000, VmaFlags::rw());
        let r2 = handler.mmap(&mut space, 0x20001000, 0x4000, VmaFlags::rw());
        assert!(r2.is_err());
    }

    #[test]
    fn test_munmap() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));

        let addr = handler.mmap(&mut space, 0x20000000, 0x4000, VmaFlags::rw()).unwrap();
        assert_eq!(space.vma_count(), 1);

        // Map a page in the VMA
        let fault = PageFaultInfo::from_registers(addr, PF_USER);
        handler.handle_fault(&fault, &mut space);

        assert!(handler.munmap(&mut space, addr, 0x4000));
        assert_eq!(space.vma_count(), 0);
        assert_eq!(space.mapped_pages(), 0);
    }

    #[test]
    fn test_munmap_not_found() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));

        assert!(!handler.munmap(&mut space, 0x99900000, 0x4000));
    }

    #[test]
    fn test_fork_address_space() {
        let mut handler = PageFaultHandler::new();
        let mut parent = ProcessAddressSpace::new(Pid(1000));
        parent.add_vma(VirtualMemoryArea::new(0x400000, 0x500000, VmaFlags::rx(), VmaType::Code));
        parent.add_vma(VirtualMemoryArea::new(0x600000, 0x700000, VmaFlags::rw(), VmaType::Heap));

        // Map some pages
        parent.map_page(0x400000, 10, false, true);
        parent.map_page(0x600000, 20, true, false);

        let child = handler.fork_address_space(&parent, Pid(1001));

        // Child should have same VMAs
        assert_eq!(child.vma_count(), 2);
        assert_eq!(child.mapped_pages(), 2);

        // Child's writable pages should be CoW
        let child_heap_pte = child.get_pte(0x600000).unwrap();
        assert!(child_heap_pte.is_cow());
        assert!(!child_heap_pte.writable);

        // Child's read-only pages should be same
        let child_code_pte = child.get_pte(0x400000).unwrap();
        assert!(!child_code_pte.is_cow());
    }

    #[test]
    fn test_create_user_space() {
        let mut handler = PageFaultHandler::new();
        let space = handler.create_user_space(Pid(1000), 0x400000, 0x10000, 0x7FFFF000);

        assert_eq!(space.pid, Pid(1000));
        assert_eq!(space.vma_count(), 3); // Code + Heap + Stack

        // Check VMA types
        assert_eq!(space.vmas[0].vma_type, VmaType::Code);
        assert_eq!(space.vmas[1].vma_type, VmaType::Heap);
        assert_eq!(space.vmas[2].vma_type, VmaType::Stack);

        // Check initial pages mapped (code + stack)
        assert_eq!(space.mapped_pages(), 2);
        assert!(space.get_pte(0x400000).is_some());
    }

    #[test]
    fn test_fault_result_display() {
        assert_eq!(format!("{}", FaultResult::Fixed), "fixed (demand paged)");
        assert_eq!(format!("{}", FaultResult::CowFixed), "fixed (CoW)");
        assert_eq!(format!("{}", FaultResult::Segfault), "SIGSEGV");
        assert_eq!(format!("{}", FaultResult::NoVma), "no VMA");
        assert_eq!(format!("{}", FaultResult::OutOfMemory), "out of memory");
    }

    #[test]
    fn test_full_demand_paging_lifecycle() {
        let mut handler = PageFaultHandler::new();
        let mut space = handler.create_user_space(Pid(1000), 0x400000, 0x10000, 0x7FFFF000);

        // Touch a heap page (not yet mapped)
        let heap_fault = PageFaultInfo::from_registers(0x600000, PF_USER);
        let r1 = handler.handle_fault(&heap_fault, &mut space);
        assert_eq!(r1, FaultResult::Mapped);
        assert!(space.get_pte(0x600000).is_some());

        // Touch another heap page
        let heap_fault2 = PageFaultInfo::from_registers(0x601000, PF_USER);
        let r2 = handler.handle_fault(&heap_fault2, &mut space);
        assert_eq!(r2, FaultResult::Mapped);
        assert_eq!(space.mapped_pages(), 4); // 2 initial + 2 demand

        // Stats
        let stats = handler.stats();
        assert_eq!(stats.total_faults, 2);
        assert_eq!(stats.demand_pages, 2);
        assert_eq!(stats.segfaults, 0);
    }

    #[test]
    fn test_stats_tracking() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x400000, 0x500000, VmaFlags::rx(), VmaType::Code));

        // Demand page
        handler.handle_fault(&PageFaultInfo::from_registers(0x400000, PF_USER), &mut space);
        // Segfault (no VMA)
        handler.handle_fault(&PageFaultInfo::from_registers(0x900000, PF_USER), &mut space);
        // Permission denied
        handler.handle_fault(&PageFaultInfo::from_registers(0x400000, PF_USER | PF_WRITE | PF_PRESENT), &mut space);

        let stats = handler.stats();
        assert_eq!(stats.total_faults, 3);
        assert_eq!(stats.demand_pages, 1);
        assert_eq!(stats.segfaults, 1);
        assert_eq!(stats.permission_faults, 1);
    }

    #[test]
    fn test_multiple_mmap_and_unmap() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));

        let a1 = handler.mmap(&mut space, 0, 0x4000, VmaFlags::rw()).unwrap();
        let a2 = handler.mmap(&mut space, 0, 0x4000, VmaFlags::rw()).unwrap();
        let a3 = handler.mmap(&mut space, 0, 0x4000, VmaFlags::rw()).unwrap();

        assert_ne!(a1, a2);
        assert_ne!(a2, a3);
        assert_eq!(space.vma_count(), 3);

        // Unmap middle one
        assert!(handler.munmap(&mut space, a2, 0x4000));
        assert_eq!(space.vma_count(), 2);

        // Can mmap at a2's old location now
        let a4 = handler.mmap(&mut space, a2, 0x4000, VmaFlags::rw());
        assert!(a4.is_ok());
        assert_eq!(space.vma_count(), 3);
    }

    #[test]
    fn test_oom_on_frame_exhaustion() {
        let mut handler = PageFaultHandler::new();
        let mut space = ProcessAddressSpace::new(Pid(1000));
        space.add_vma(VirtualMemoryArea::new(0x400000, 0x500000, VmaFlags::rw(), VmaType::Heap));

        // Exhaust all frames
        let alloc = handler.frame_allocator_mut();
        while alloc.alloc().is_some() {}

        // Now a page fault should fail with OOM
        let fault = PageFaultInfo::from_registers(0x400000, PF_USER);
        let result = handler.handle_fault(&fault, &mut space);
        assert_eq!(result, FaultResult::OutOfMemory);
        assert_eq!(handler.stats().oom_faults, 1);
    }
}

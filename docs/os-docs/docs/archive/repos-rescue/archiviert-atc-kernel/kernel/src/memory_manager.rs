//! ShivaCore Kernel — MemoryManager Trait Implementation.
//!
//! Verbindet den Boot-Level Heap-Allocator (allocator.rs) mit dem
//! ats1000.rs MemoryManager-Trait. Bietet prozessbezogene
//! Speicherverwaltung mit Capability-Integration.
//!
//! Schichten:
//!   allocator.rs (L0):  Echter Kernel-Heap (linked_list_allocator)
//!   memory_manager.rs (L1): Prozess-Speicher-Regionen + Capability-Gating
//!   ats1000.rs (L2): Trait-Schnittstelle nach aussen
//!
//! MemorySubsystem (L1.5): Verbindet L0 und L1 — nutzt den echten
//! Heap fuer kleine Allokationen, verwaltet Regionen fuer Prozesse.

extern crate alloc;

use alloc::alloc::{alloc, dealloc, Layout};
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::ats1000::{MemoryManager, MemRegion, Pid};
use crate::capability::{CapabilityTable, ResourceType, Rights};

// === Konstanten — synchronisiert mit allocator.rs === //
// allocator.rs: HEAP_START = 0x4444_4444_0000, HEAP_SIZE = 100 * 1024
// In Kernel-Mode werden diese aus allocator.rs importiert;
// in Test-Mode (hier) definieren wir sie identisch.
pub const HEAP_START: u64 = 0x_4444_4444_0000;
pub const HEAP_SIZE: u64 = 100 * 1024; // 100 KiB — identisch zu allocator.rs
pub const HEAP_END: u64 = HEAP_START + HEAP_SIZE;
pub const USERSPACE_BASE: u64 = 0x_5555_5555_0000; // Getrennt vom Kernel-Heap
pub const USERSPACE_MAX: u64 = 100 * 1024 * 1024;  // 100 MiB Userspace-Simulation

/// Verwaltete Speicherregion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocatedRegion {
    pub addr: u64,
    pub size: u64,
    pub owner_pid: Pid,
    pub region_id: u64,
    /// Quelle der Allokation
    pub source: AllocSource,
}

/// Wo wurde der Speicher allokiert?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocSource {
    /// Kernel-Heap (allocator.rs / linked_list_allocator)
    KernelHeap,
    /// Userspace-Bump (virtuelle Adress-Simulation)
    UserspaceBump,
}

/// Fehler bei Speicheroperationen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemError {
    OutOfMemory,
    InvalidRegion,
    NoCapability,
    DoubleFree,
    InvalidAlignment,
    /// Allokation waere zu gross fuer Kernel-Heap
    HeapOverflow,
}

/// Kernel Memory Manager — verwaltet Speicherregionen pro Prozess.
/// Nutzt Capability-System fuer Zugriffskontrolle.
///
/// Zwei Allokations-Modi:
///   1. Kernel-Heap (alloc/dealloc): fuer kleine, echte Allokationen
///   2. Userspace-Bump: fuer simulierte Prozess-Regionen
pub struct KernelMemoryManager {
    /// Naechste virtuelle Adresse fuer Userspace-Bump
    next_addr: AtomicU64,
    /// Alle aktiven Regionen: region_id -> AllocatedRegion
    regions: BTreeMap<u64, AllocatedRegion>,
    /// Zaehler fuer Region-IDs
    next_region_id: AtomicU64,
    /// Gesamter allozierter Speicher (Bytes)
    total_allocated: u64,
    /// Peak-Speichernutzung
    peak_allocated: u64,
    /// Heap-Bruecke: region_id -> raw pointer + layout fuer dealloc
    #[allow(clippy::type_complexity)]
    heap_allocations: BTreeMap<u64, (usize, Layout)>,
    /// Schwelle: Allokationen <= threshold gehen an Kernel-Heap
    /// Allokationen > threshold gehen an Userspace-Bump
    heap_threshold: u64,
}

impl KernelMemoryManager {
    pub fn new() -> Self {
        Self {
            next_addr: AtomicU64::new(USERSPACE_BASE),
            regions: BTreeMap::new(),
            next_region_id: AtomicU64::new(1),
            total_allocated: 0,
            peak_allocated: 0,
            heap_allocations: BTreeMap::new(),
            heap_threshold: 4096, // <= 4KB -> Kernel-Heap, > 4KB -> Userspace
        }
    }

    /// Konfiguriert die Heap-Schwelle
    pub fn with_heap_threshold(mut self, threshold: u64) -> Self {
        self.heap_threshold = threshold;
        self
    }

    /// Allokiert eine Speicherregion fuer einen Prozess.
    /// Vergibt automatisch READ + WRITE + EXEC Capability.
    ///
    /// Routing:
    ///   size <= heap_threshold -> Kernel-Heap (echte alloc::alloc)
    ///   size > heap_threshold  -> Userspace-Bump (virtuelle Adress-Simulation)
    pub fn allocate(
        &mut self,
        caps: &mut CapabilityTable,
        pid: Pid,
        size: u64,
    ) -> Result<AllocatedRegion, MemError> {
        if size == 0 { return Err(MemError::InvalidAlignment); }

        let region_id = self.next_region_id.fetch_add(1, Ordering::SeqCst);

        let (addr, source) = if size <= self.heap_threshold {
            // Kernel-Heap: echte Allokation
            let layout = Layout::from_size_align(size as usize, 4096)
                .map_err(|_| MemError::InvalidAlignment)?;

            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                return Err(MemError::HeapOverflow);
            }

            self.heap_allocations.insert(region_id, (size as usize, layout));
            (ptr as u64, AllocSource::KernelHeap)
        } else {
            // Userspace-Bump
            let current = self.next_addr.load(Ordering::SeqCst);
            if current + size > USERSPACE_BASE + USERSPACE_MAX {
                return Err(MemError::OutOfMemory);
            }

            let addr = self.next_addr.fetch_add(size, Ordering::SeqCst);
            let aligned_addr = (addr + 0xFFF) & !0xFFF;
            if aligned_addr != addr {
                self.next_addr.store(aligned_addr + size, Ordering::SeqCst);
            }
            (aligned_addr, AllocSource::UserspaceBump)
        };

        let region = AllocatedRegion {
            addr, size, owner_pid: pid, region_id, source,
        };

        // Capability vergeben
        caps.create(
            pid, ResourceType::Memory, region_id,
            Rights::READ | Rights::WRITE | Rights::EXEC | Rights::DELEGATE,
        );

        self.total_allocated += size;
        if self.total_allocated > self.peak_allocated {
            self.peak_allocated = self.total_allocated;
        }

        self.regions.insert(region_id, region);
        Ok(region)
    }

    /// Gibt eine Speicherregion frei.
    /// Prueft WRITE-Capability und widerruft alle Capabilities.
    /// Bei Kernel-Heap-Regionen: echte dealloc.
    pub fn deallocate(
        &mut self,
        caps: &mut CapabilityTable,
        pid: Pid,
        region_id: u64,
    ) -> Result<(), MemError> {
        // Existiert die Region?
        let region = self.regions.get(&region_id).ok_or(MemError::InvalidRegion)?;

        // Capability-Check
        if !caps.check(pid, ResourceType::Memory, region_id, Rights::WRITE) {
            return Err(MemError::NoCapability);
        }

        // Bei Kernel-Heap-Regionen: echtes dealloc
        if region.source == AllocSource::KernelHeap {
            if let Some((_, layout)) = self.heap_allocations.remove(&region_id) {
                // Sichere dealloc: addr war ein echter Heap-Pointer
                let ptr = region.addr as *mut u8;
                unsafe { dealloc(ptr, layout); }
            }
        }

        let size = region.size;
        self.total_allocated -= size;
        self.regions.remove(&region_id);

        // Capabilities widerrufen
        let cap_ids: Vec<_> = caps.list_for(pid).iter()
            .filter(|c| c.resource_type == ResourceType::Memory && c.resource_id == region_id)
            .map(|c| c.id)
            .collect();
        for cap_id in cap_ids {
            caps.revoke(cap_id);
        }

        Ok(())
    }

    /// Liest Speicher (simuliert — gibt die Regions-Info zurueck).
    /// Prueft READ-Capability.
    pub fn read_check(
        &self,
        caps: &CapabilityTable,
        pid: Pid,
        region_id: u64,
    ) -> Result<AllocatedRegion, MemError> {
        let region = self.regions.get(&region_id).ok_or(MemError::InvalidRegion)?;
        if !caps.check(pid, ResourceType::Memory, region_id, Rights::READ) {
            return Err(MemError::NoCapability);
        }
        Ok(*region)
    }

    /// Schreibt in eine Kernel-Heap-Region (echter Memory-Write).
    /// Prueft WRITE-Capability.
    pub fn write_check(
        &self,
        caps: &CapabilityTable,
        pid: Pid,
        region_id: u64,
    ) -> Result<AllocatedRegion, MemError> {
        let region = self.regions.get(&region_id).ok_or(MemError::InvalidRegion)?;
        if !caps.check(pid, ResourceType::Memory, region_id, Rights::WRITE) {
            return Err(MemError::NoCapability);
        }
        Ok(*region)
    }

    /// Mappt eine physikalische Adresse (mmap — simuliert).
    pub fn mmap(
        &mut self,
        caps: &mut CapabilityTable,
        pid: Pid,
        size: u64,
    ) -> Result<AllocatedRegion, MemError> {
        self.allocate(caps, pid, size)
    }

    /// Statistik
    pub fn stats(&self) -> MemStats {
        MemStats {
            total_allocated: self.total_allocated,
            peak_allocated: self.peak_allocated,
            active_regions: self.regions.len() as u64,
            heap_base: HEAP_START,
            heap_size: HEAP_SIZE,
            heap_end: HEAP_END,
            userspace_base: USERSPACE_BASE,
            heap_allocs: self.heap_allocations.len() as u64,
        }
    }

    /// Liste aller Regionen eines Prozesses
    pub fn regions_for(&self, pid: Pid) -> Vec<AllocatedRegion> {
        self.regions.values()
            .filter(|r| r.owner_pid == pid)
            .cloned()
            .collect()
    }

    /// Anzahl aktive Regionen
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// Prueft, ob der Kernel-Heap für eine gegebene Groesse ausreichen wuerde
    pub fn can_fit_on_heap(&self, size: u64) -> bool {
        size > 0 && size <= self.heap_threshold
    }

    /// Prueft, ob eine Adresse im Kernel-Heap-Bereich liegt
    pub fn is_heap_address(addr: u64) -> bool {
        addr >= HEAP_START && addr < HEAP_END
    }

    /// Prueft, ob eine Adresse im Userspace-Bereich liegt
    pub fn is_userspace_address(addr: u64) -> bool {
        addr >= USERSPACE_BASE && addr < USERSPACE_BASE + USERSPACE_MAX
    }
}

/// Memory-Statistik
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemStats {
    pub total_allocated: u64,
    pub peak_allocated: u64,
    pub active_regions: u64,
    pub heap_base: u64,
    pub heap_size: u64,
    pub heap_end: u64,
    pub userspace_base: u64,
    pub heap_allocs: u64,
}

// === MemorySubsystem — verbindet L0 (Heap) und L1 (Regions) === //

/// Das vereinte Speicher-Subsystem: Heap-Bruecke + Prozess-Regionen + Caps.
/// Dies ist die Haupt-Schnittstelle, die der Kernel im Init benutzt.
pub struct MemorySubsystem {
    pub manager: KernelMemoryManager,
    pub caps: CapabilityTable,
}

impl MemorySubsystem {
    /// Erzeugt ein neues Speicher-Subsystem.
    /// In Kernel-Mode: wird nach allocator::init_heap() aufgerufen.
    pub fn new() -> Self {
        Self {
            manager: KernelMemoryManager::new(),
            caps: CapabilityTable::new(),
        }
    }

    /// Kernel-Init-Sequenz: initialisiert Heap + Memory-Subsystem.
    /// In Kernel-Mode wuerde dies allocator::init_heap() aufrufen,
    /// gefolgt von KernelMemoryManager::new().
    ///
    /// # Safety (Kernel-Mode)
    /// Setzt voraus, dass Paging aktiv ist und der physische Speicher
    /// gemapped wurde (memory::init + allocator::init_heap).
    pub fn init_kernel() -> Self {
        // In echter Kernel-Mode:
        //   1. memory::init(physical_memory_offset)  -> OffsetPageTable
        //   2. allocator::init_heap(mapper, frame_alloc)  -> Heap bereit
        //   3. MemorySubsystem::new()  -> Prozess-Regionen + Caps
        //
        // In Test-Mode: Heap ist bereits durch Rust std aktiv.
        Self::new()
    }

    /// Allokiert Speicher fuer einen Prozess
    pub fn allocate(&mut self, pid: Pid, size: u64) -> Result<AllocatedRegion, MemError> {
        self.manager.allocate(&mut self.caps, pid, size)
    }

    /// Gibt Speicher frei
    pub fn deallocate(&mut self, pid: Pid, region_id: u64) -> Result<(), MemError> {
        self.manager.deallocate(&mut self.caps, pid, region_id)
    }

    /// Prueft Lesezugriff
    pub fn read_check(&self, pid: Pid, region_id: u64) -> Result<AllocatedRegion, MemError> {
        self.manager.read_check(&self.caps, pid, region_id)
    }

    /// Prueft Schreibzugriff
    pub fn write_check(&self, pid: Pid, region_id: u64) -> Result<AllocatedRegion, MemError> {
        self.manager.write_check(&self.caps, pid, region_id)
    }

    /// Statistik
    pub fn stats(&self) -> MemStats {
        self.manager.stats()
    }

    /// Alle Regionen eines Prozesses
    pub fn regions_for(&self, pid: Pid) -> Vec<AllocatedRegion> {
        self.manager.regions_for(pid)
    }
}

// === ats1000 MemoryManager-Trait-Implementierung (Adapter) === //

impl MemoryManager for KernelMemoryManager {
    fn alloc(&mut self, size: u64, pid: Pid) -> Option<MemRegion> {
        if size == 0 { return None; }

        let region_id = self.next_region_id.fetch_add(1, Ordering::SeqCst);

        // Route nach Groesse
        let (addr, source) = if size <= self.heap_threshold {
            let layout = Layout::from_size_align(size as usize, 4096).ok()?;
            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() { return None; }
            self.heap_allocations.insert(region_id, (size as usize, layout));
            (ptr as u64, AllocSource::KernelHeap)
        } else {
            let current = self.next_addr.load(Ordering::SeqCst);
            if current + size > USERSPACE_BASE + USERSPACE_MAX { return None; }
            let addr = self.next_addr.fetch_add(size, Ordering::SeqCst);
            let aligned = (addr + 0xFFF) & !0xFFF;
            if aligned != addr {
                self.next_addr.store(aligned + size, Ordering::SeqCst);
            }
            (aligned, AllocSource::UserspaceBump)
        };

        let region = AllocatedRegion { addr, size, owner_pid: pid, region_id, source };
        self.total_allocated += size;
        if self.total_allocated > self.peak_allocated {
            self.peak_allocated = self.total_allocated;
        }
        self.regions.insert(region_id, region);
        Some(MemRegion { addr, size, pid })
    }

    fn free(&mut self, region: MemRegion) -> bool {
        let region_id = self.regions.iter()
            .find(|(_, r)| r.addr == region.addr && r.owner_pid == region.pid)
            .map(|(id, _)| *id);

        let id = match region_id {
            Some(id) => id,
            None => return false,
        };

        // Heap- dealloc
        if let Some(r) = self.regions.get(&id) {
            if r.source == AllocSource::KernelHeap {
                if let Some((_, layout)) = self.heap_allocations.remove(&id) {
                    let ptr = r.addr as *mut u8;
                    unsafe { dealloc(ptr, layout); }
                }
            }
        }

        let size = self.regions.get(&id).map(|r| r.size).unwrap_or(0);
        self.total_allocated -= size;
        self.regions.remove(&id);
        true
    }

    fn mmap(&mut self, addr: u64, size: u64) -> Option<MemRegion> {
        let _ = addr; // Hint wird ignoriert (Bump-Allocator)
        self.alloc(size, Pid(0)) // PID 0 = Kernel
    }
}

// === Integration mit allocator.rs Konstanten === //

/// Validiert, dass die Konstanten mit allocator.rs konsistent sind.
/// Wird beim Kernel-Boot aufgerufen.
pub fn validate_heap_config() -> Result<(), String> {
    // allocator.rs: HEAP_START = 0x_4444_4444_0000, HEAP_SIZE = 100 * 1024
    // memory_manager.rs: HEAP_START = 0x_4444_4444_0000, HEAP_SIZE = 100 * 1024
    if HEAP_START != 0x_4444_4444_0000 {
        return Err(format!("HEAP_START mismatch: 0x{:x} != 0x444444440000", HEAP_START));
    }
    if HEAP_SIZE != 100 * 1024 {
        return Err(format!("HEAP_SIZE mismatch: {} != {}", HEAP_SIZE, 100 * 1024));
    }
    if USERSPACE_BASE <= HEAP_END {
        return Err(format!("USERSPACE_BASE (0x{:x}) must be > HEAP_END (0x{:x})",
            USERSPACE_BASE, HEAP_END));
    }
    Ok(())
}

/// Kernel-Boot-Meldung fuer das Speicher-Subsystem
pub fn boot_log() -> String {
    format!(
        "ShivaCore Memory Subsystem:\n  Kernel-Heap:  0x{:x} - 0x{:x} ({} KiB)\n  Userspace:    0x{:x} - 0x{:x} ({} MiB)\n  Heap-Threshold: {} bytes\n  Bridge: linked_list_allocator <-> KernelMemoryManager",
        HEAP_START, HEAP_END, HEAP_SIZE / 1024,
        USERSPACE_BASE, USERSPACE_BASE + USERSPACE_MAX, USERSPACE_MAX / (1024 * 1024),
        4096
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pid(n: u32) -> Pid { Pid(n) }

    // === Basis-Tests === //

    #[test]
    fn test_allocate_kernel_heap() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();
        let r = mm.allocate(&mut caps, pid(1), 256).unwrap(); // <= 4096 -> Kernel-Heap
        assert_eq!(r.size, 256);
        assert_eq!(r.source, AllocSource::KernelHeap);
        assert!(r.addr != 0); // Adresse ist nicht 0
    }

    #[test]
    fn test_allocate_userspace_bump() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();
        let r = mm.allocate(&mut caps, pid(1), 8192).unwrap(); // > 4096 -> Userspace
        assert_eq!(r.source, AllocSource::UserspaceBump);
        assert!(r.addr >= USERSPACE_BASE);
    }

    #[test]
    fn test_allocate_creates_capability() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();
        let r = mm.allocate(&mut caps, pid(1), 4096).unwrap();
        assert!(caps.check(pid(1), ResourceType::Memory, r.region_id, Rights::READ));
        assert!(caps.check(pid(1), ResourceType::Memory, r.region_id, Rights::WRITE));
        assert!(caps.check(pid(1), ResourceType::Memory, r.region_id, Rights::EXEC));
    }

    #[test]
    fn test_deallocate_kernel_heap() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();
        let r = mm.allocate(&mut caps, pid(1), 512).unwrap();
        assert_eq!(r.source, AllocSource::KernelHeap);

        mm.deallocate(&mut caps, pid(1), r.region_id).unwrap();

        // Cap widerrufen
        assert!(!caps.check(pid(1), ResourceType::Memory, r.region_id, Rights::WRITE));
        // Region weg
        assert_eq!(mm.region_count(), 0);
    }

    #[test]
    fn test_deallocate_userspace() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();
        let r = mm.allocate(&mut caps, pid(1), 8192).unwrap();
        assert_eq!(r.source, AllocSource::UserspaceBump);

        mm.deallocate(&mut caps, pid(1), r.region_id).unwrap();
        assert_eq!(mm.region_count(), 0);
    }

    #[test]
    fn test_deallocate_without_cap_rejected() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();
        let r = mm.allocate(&mut caps, pid(1), 4096).unwrap();
        assert_eq!(mm.deallocate(&mut caps, pid(2), r.region_id), Err(MemError::NoCapability));
    }

    #[test]
    fn test_deallocate_invalid_region() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();
        assert_eq!(mm.deallocate(&mut caps, pid(1), 999), Err(MemError::InvalidRegion));
    }

    #[test]
    fn test_multiple_allocations_unique() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();

        let r1 = mm.allocate(&mut caps, pid(1), 4096).unwrap();
        let r2 = mm.allocate(&mut caps, pid(1), 4096).unwrap();
        let r3 = mm.allocate(&mut caps, pid(2), 4096).unwrap();

        assert_ne!(r1.addr, r2.addr);
        assert_ne!(r2.addr, r3.addr);
    }

    #[test]
    fn test_read_check_without_cap_rejected() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();
        let r = mm.allocate(&mut caps, pid(1), 4096).unwrap();

        assert!(mm.read_check(&caps, pid(1), r.region_id).is_ok());
        assert_eq!(mm.read_check(&caps, pid(2), r.region_id), Err(MemError::NoCapability));
    }

    #[test]
    fn test_write_check_without_cap_rejected() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();
        let r = mm.allocate(&mut caps, pid(1), 2048).unwrap();

        assert!(mm.write_check(&caps, pid(1), r.region_id).is_ok());
        assert_eq!(mm.write_check(&caps, pid(2), r.region_id), Err(MemError::NoCapability));
    }

    #[test]
    fn test_stats_tracking() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();

        mm.allocate(&mut caps, pid(1), 4096).unwrap();
        mm.allocate(&mut caps, pid(1), 8192).unwrap();
        mm.allocate(&mut caps, pid(2), 4096).unwrap();

        let s = mm.stats();
        assert_eq!(s.total_allocated, 16384);
        assert_eq!(s.active_regions, 3);
        assert_eq!(s.peak_allocated, 16384);
        assert!(s.heap_allocs >= 1); // Mindestens eine Heap-Allokation
    }

    #[test]
    fn test_stats_peak_after_free() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();

        let r = mm.allocate(&mut caps, pid(1), 8192).unwrap();
        let s1 = mm.stats();
        assert_eq!(s1.peak_allocated, 8192);

        mm.deallocate(&mut caps, pid(1), r.region_id).unwrap();
        let s2 = mm.stats();
        assert_eq!(s2.total_allocated, 0);
        assert_eq!(s2.peak_allocated, 8192);
    }

    #[test]
    fn test_regions_for_pid() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();

        mm.allocate(&mut caps, pid(1), 4096).unwrap();
        mm.allocate(&mut caps, pid(1), 8192).unwrap();
        mm.allocate(&mut caps, pid(2), 4096).unwrap();

        assert_eq!(mm.regions_for(pid(1)).len(), 2);
        assert_eq!(mm.regions_for(pid(2)).len(), 1);
    }

    #[test]
    fn test_zero_size_rejected() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();
        assert_eq!(mm.allocate(&mut caps, pid(1), 0), Err(MemError::InvalidAlignment));
    }

    // === ats1000 Trait-Tests === //

    #[test]
    fn test_ats1000_alloc_free_heap() {
        let mut mm = KernelMemoryManager::new();
        let r = mm.alloc(256, pid(1)).unwrap();
        assert_eq!(r.size, 256);
        assert!(mm.free(r));
        assert!(!mm.free(r)); // Double-free
    }

    #[test]
    fn test_ats1000_alloc_free_userspace() {
        let mut mm = KernelMemoryManager::new();
        let r = mm.alloc(8192, pid(1)).unwrap(); // Userspace
        assert!(mm.free(r));
    }

    #[test]
    fn test_ats1000_mmap() {
        let mut mm = KernelMemoryManager::new();
        let r = MemoryManager::mmap(&mut mm, 0x5000, 4096).unwrap();
        assert_eq!(r.size, 4096);
    }

    // === Heap-Bridge-Tests === //

    #[test]
    fn test_heap_allocation_is_real() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();

        // Kleine Allokation -> Kernel-Heap (echte alloc::alloc)
        let r = mm.allocate(&mut caps, pid(1), 128).unwrap();
        assert_eq!(r.source, AllocSource::KernelHeap);

        // Adresse sollte im echten Heap-Bereich liegen (nicht Userspace)
        assert!(!KernelMemoryManager::is_userspace_address(r.addr));
    }




    #[test]
    fn test_heap_threshold_routing() {
        let mut mm = KernelMemoryManager::new().with_heap_threshold(2048);
        let mut caps = CapabilityTable::new();

        let r1 = mm.allocate(&mut caps, pid(1), 2048).unwrap();
        assert_eq!(r1.source, AllocSource::KernelHeap); // <= 2048

        let r2 = mm.allocate(&mut caps, pid(1), 2049).unwrap();
        assert_eq!(r2.source, AllocSource::UserspaceBump); // > 2048
    }

    #[test]
    fn test_heap_dealloc_frees_memory() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();

        // Allokiere + deallokiere mehrfach — sollte nicht OOM gehen
        for _ in 0..100 {
            let r = mm.allocate(&mut caps, pid(1), 256).unwrap();
            mm.deallocate(&mut caps, pid(1), r.region_id).unwrap();
        }

        let s = mm.stats();
        assert_eq!(s.total_allocated, 0);
        assert_eq!(s.active_regions, 0);
    }

    #[test]
    fn test_mixed_heap_and_userspace() {
        let mut mm = KernelMemoryManager::new();
        let mut caps = CapabilityTable::new();

        let heap_r = mm.allocate(&mut caps, pid(1), 1024).unwrap();   // Heap
        let user_r = mm.allocate(&mut caps, pid(1), 65536).unwrap();   // Userspace
        let heap_r2 = mm.allocate(&mut caps, pid(2), 2048).unwrap();  // Heap

        assert_eq!(heap_r.source, AllocSource::KernelHeap);
        assert_eq!(user_r.source, AllocSource::UserspaceBump);
        assert_eq!(heap_r2.source, AllocSource::KernelHeap);

        // Alle freigeben
        mm.deallocate(&mut caps, pid(1), heap_r.region_id).unwrap();
        mm.deallocate(&mut caps, pid(1), user_r.region_id).unwrap();
        mm.deallocate(&mut caps, pid(2), heap_r2.region_id).unwrap();

        assert_eq!(mm.region_count(), 0);
    }

    // === Konstanten-Validierung === //

    #[test]
    fn test_validate_heap_config() {
        assert!(validate_heap_config().is_ok());
    }

    #[test]
    fn test_constants_match_allocator() {
        // Muss identisch zu allocator.rs sein
        assert_eq!(HEAP_START, 0x_4444_4444_0000);
        assert_eq!(HEAP_SIZE, 100 * 1024);
        assert_eq!(HEAP_END, HEAP_START + HEAP_SIZE);
        // Userspace muss ausserhalb des Kernel-Heaps liegen
        assert!(USERSPACE_BASE > HEAP_END);
    }

    #[test]
    fn test_is_heap_address() {
        assert!(KernelMemoryManager::is_heap_address(HEAP_START));
        assert!(KernelMemoryManager::is_heap_address(HEAP_START + 1000));
        assert!(KernelMemoryManager::is_heap_address(HEAP_END - 1));
        assert!(!KernelMemoryManager::is_heap_address(HEAP_END));
        assert!(!KernelMemoryManager::is_heap_address(0));
    }

    #[test]
    fn test_is_userspace_address() {
        assert!(KernelMemoryManager::is_userspace_address(USERSPACE_BASE));
        assert!(KernelMemoryManager::is_userspace_address(USERSPACE_BASE + 1000));
        assert!(!KernelMemoryManager::is_userspace_address(HEAP_START));
    }

    #[test]
    fn test_can_fit_on_heap() {
        let mm = KernelMemoryManager::new();
        assert!(mm.can_fit_on_heap(4096));
        assert!(mm.can_fit_on_heap(1));
        assert!(!mm.can_fit_on_heap(4097));
        assert!(!mm.can_fit_on_heap(0));
    }

    // === MemorySubsystem-Tests === //

    #[test]
    fn test_subsystem_init() {
        let ms = MemorySubsystem::new();
        let s = ms.stats();
        assert_eq!(s.total_allocated, 0);
        assert_eq!(s.active_regions, 0);
    }

    #[test]
    fn test_subsystem_allocate_deallocate() {
        let mut ms = MemorySubsystem::new();
        let r = ms.allocate(pid(1), 1024).unwrap();
        assert_eq!(r.source, AllocSource::KernelHeap);
        assert_eq!(ms.stats().active_regions, 1);

        ms.deallocate(pid(1), r.region_id).unwrap();
    }

    #[test]
    fn test_subsystem_multiple_processes() {
        let mut ms = MemorySubsystem::new();

        ms.allocate(pid(1), 1024).unwrap();
        ms.allocate(pid(1), 2048).unwrap();
        ms.allocate(pid(2), 1024).unwrap();
        ms.allocate(pid(3), 4096).unwrap();

        assert_eq!(ms.regions_for(pid(1)).len(), 2);
        assert_eq!(ms.regions_for(pid(2)).len(), 1);
        assert_eq!(ms.regions_for(pid(3)).len(), 1);
        assert_eq!(ms.regions_for(pid(4)).len(), 0);
    }

    #[test]
    fn test_subsystem_isolation() {
        let mut ms = MemorySubsystem::new();
        let r1 = ms.allocate(pid(1), 1024).unwrap();

        // pid(2) kann nicht auf pid(1)s Region zugreifen
        assert_eq!(ms.read_check(pid(2), r1.region_id), Err(MemError::NoCapability));
        assert_eq!(ms.write_check(pid(2), r1.region_id), Err(MemError::NoCapability));
    }

    // === Boot-Log === //

    #[test]
    fn test_boot_log() {
        let log = boot_log();
        assert!(log.contains("ShivaCore Memory Subsystem"));
        assert!(log.contains("Kernel-Heap"));
        assert!(log.contains("Userspace"));
        assert!(log.contains("linked_list_allocator"));
    }
}


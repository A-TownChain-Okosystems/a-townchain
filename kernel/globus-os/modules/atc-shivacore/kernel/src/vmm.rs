// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore M1.2 VMM — fail-closed validation primitives.
//!
//! This module contains the deterministic validation and policy primitives
//! (M1.2 core) plus the PML4 → PDPT → PD → PT hardware page-table mutation
//! path with transactional intermediate-table rollback (M1.2 follow-up 1+2).
//! Real-boot wiring (HHDM backend + BootInfoFrameAllocator) follows in the
//! hardware chain; all mutation logic is evidence-tested deterministically.
#![allow(dead_code)]

pub const PAGE_SIZE: u64 = 4096;
pub const PAGE_MASK: u64 = PAGE_SIZE - 1;

/// User virtual address space is a half-open interval: [BASE, TOP).
pub const USER_SPACE_BASE: u64 = 0x0000_0000_0001_0000;
pub const USER_SPACE_TOP_EXCLUSIVE: u64 = 0x0000_8000_0000_0000;

/// Supervisor-only HHDM window, also half-open: [BASE, END).
pub const HHDM_BASE: u64 = 0xFFFF_8000_0000_0000;
pub const HHDM_END: u64 = 0xFFFF_C000_0000_0000;

/// M1.2 physical-address baseline. Future hardware discovery may derive this
/// from CPUID, but this milestone intentionally freezes a 48-bit baseline.
pub const MAX_PHYS_ADDR_WIDTH: u64 = 48;
pub const MAX_PHYS_ADDR: u64 = (1u64 << MAX_PHYS_ADDR_WIDTH) - 1;
pub const PHYS_ADDR_MASK: u64 = 0x0000_FFFF_FFFF_F000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmmError {
    AddressNotCanonical,
    InvalidPhysicalAddress,
    InvalidPageFlags,
    PrivilegeViolation,
    RangeOutOfBounds,
    HugePageNotSupported,
    FrameAllocationFailed,
    AlreadyMapped,
    NotMapped,
    AddressMisaligned,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtAddr(pub u64);

impl VirtAddr {
    pub const fn new(addr: u64) -> Result<Self, VmmError> {
        let upper = addr >> 48;
        let expected = if ((addr >> 47) & 1) == 0 { 0 } else { 0xFFFF };
        if upper != expected {
            return Err(VmmError::AddressNotCanonical);
        }
        Ok(Self(addr))
    }

    pub const fn is_user(self) -> bool {
        self.0 >= USER_SPACE_BASE && self.0 < USER_SPACE_TOP_EXCLUSIVE
    }

    pub const fn is_hhdm(self) -> bool {
        self.0 >= HHDM_BASE && self.0 < HHDM_END
    }

    pub const fn page_offset(self) -> u64 {
        self.0 & PAGE_MASK
    }

    pub const fn page_base(self) -> u64 {
        self.0 & !PAGE_MASK
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysFrame(pub u64);

impl PhysFrame {
    pub const fn new(addr: u64) -> Result<Self, VmmError> {
        if addr & PAGE_MASK != 0 || addr > (MAX_PHYS_ADDR & !PAGE_MASK) {
            return Err(VmmError::InvalidPhysicalAddress);
        }
        Ok(Self(addr))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MappingFlags {
    pub writable: bool,
    pub user_accessible: bool,
    pub executable: bool,
    pub global: bool,
    pub cache_disable: bool,
}

impl MappingFlags {
    pub const fn user_read_only() -> Self {
        Self {
            writable: false,
            user_accessible: true,
            executable: false,
            global: false,
            cache_disable: false,
        }
    }

    pub const fn user_read_write() -> Self {
        Self {
            writable: true,
            user_accessible: true,
            executable: false,
            global: false,
            cache_disable: false,
        }
    }

    pub const fn user_execute_read() -> Self {
        Self {
            writable: false,
            user_accessible: true,
            executable: true,
            global: false,
            cache_disable: false,
        }
    }

    pub const fn kernel_rw_nx() -> Self {
        Self {
            writable: true,
            user_accessible: false,
            executable: false,
            global: false,
            cache_disable: false,
        }
    }

    pub const fn validate(self) -> Result<(), VmmError> {
        if self.writable && self.executable {
            Err(VmmError::InvalidPageFlags)
        } else {
            Ok(())
        }
    }

    pub fn to_x86_flags(self) -> Result<u64, VmmError> {
        self.validate()?;
        let mut flags = 1u64; // Present
        if self.writable {
            flags |= 1 << 1;
        }
        if self.user_accessible {
            flags |= 1 << 2;
        }
        if self.cache_disable {
            flags |= 1 << 4;
        }
        if self.global {
            flags |= 1 << 8;
        }
        if !self.executable {
            flags |= 1 << 63;
        }
        Ok(flags)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAccess {
    Read,
    Write,
    Execute,
}

pub const fn validate_user_range(ptr: u64, len: u64) -> Result<(), VmmError> {
    if ptr == 0 || len == 0 || ptr < USER_SPACE_BASE || ptr >= USER_SPACE_TOP_EXCLUSIVE {
        return Err(VmmError::RangeOutOfBounds);
    }

    let end = match ptr.checked_add(len) {
        Some(value) => value,
        None => return Err(VmmError::RangeOutOfBounds),
    };

    if end > USER_SPACE_TOP_EXCLUSIVE {
        return Err(VmmError::RangeOutOfBounds);
    }

    Ok(())
}

/// Validate the destination address before any page-table allocation.
/// User mappings are allowed only inside the exact userspace interval;
/// merely being below HHDM is not sufficient.
pub fn validate_mapping_target(virt: u64, flags: MappingFlags) -> Result<(), VmmError> {
    flags.validate()?;

    // A user-accessible mapping outside the user range is a privilege
    // violation even if the address is also non-canonical: the security
    // rule must dominate the address-form check.
    if flags.user_accessible && !(USER_SPACE_BASE..USER_SPACE_TOP_EXCLUSIVE).contains(&virt) {
        return Err(VmmError::PrivilegeViolation);
    }

    VirtAddr::new(virt)?;
    Ok(())
}

/// HHDM entries must remain supervisor-only, writable, and NX.
pub const fn validate_hhdm_entry(
    virt: u64,
    writable: bool,
    user: bool,
    nx: bool,
) -> Result<(), VmmError> {
    if virt < HHDM_BASE || virt >= HHDM_END {
        return Err(VmmError::RangeOutOfBounds);
    }
    if !writable || user || !nx {
        return Err(VmmError::PrivilegeViolation);
    }
    Ok(())
}

/// x86_64 bit 7 is PS only at PDPT/PD levels. At PT level it is PAT, and
/// PML4 has no PS bit. Keeping this level-aware prevents accidental rejection
/// or acceptance of entries based on the wrong architectural semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageTableLevel {
    Pml4,
    Pdpt,
    Pd,
    Pt,
}

impl PageTableLevel {
    pub const fn huge_page_bit_is_ps(self) -> bool {
        matches!(self, Self::Pdpt | Self::Pd)
    }

    pub const fn reject_huge_page(self, entry: u64) -> Result<(), VmmError> {
        if self.huge_page_bit_is_ps() && (entry & (1 << 7)) != 0 {
            Err(VmmError::HugePageNotSupported)
        } else {
            Ok(())
        }
    }
}

/// Bounded bootstrap mapping used before the full VMM is active.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BootstrapMapping {
    pub phys_base: u64,
    pub phys_end: u64,
    pub virt_offset: u64,
}

impl BootstrapMapping {
    pub const fn new(phys_base: u64, phys_end: u64, virt_offset: u64) -> Result<Self, VmmError> {
        if phys_base >= phys_end || phys_base & PAGE_MASK != 0 || phys_end & PAGE_MASK != 0 {
            return Err(VmmError::InvalidPhysicalAddress);
        }
        Ok(Self {
            phys_base,
            phys_end,
            virt_offset,
        })
    }

    pub const fn translate(&self, phys: u64, size: u64) -> Result<u64, VmmError> {
        if size == 0 {
            return Err(VmmError::InvalidPhysicalAddress);
        }

        let end = match phys.checked_add(size) {
            Some(value) => value,
            None => return Err(VmmError::InvalidPhysicalAddress),
        };

        if phys < self.phys_base || end > self.phys_end {
            return Err(VmmError::InvalidPhysicalAddress);
        }

        match phys.checked_add(self.virt_offset) {
            Some(value) => Ok(value),
            None => Err(VmmError::InvalidPhysicalAddress),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// M1.2 Follow-up 1+2: Hardware-Page-Table-Mutation + transaktionaler Rollback
// ═══════════════════════════════════════════════════════════════════════════

/// Physisches Speicher-Backend. Im Kernel-Boot wird dies durch die
/// HHDM-Translation + den echten Frame-Allokator implementiert
/// (x86-boot/Hardware-Kette); im Test durch eine deterministische Quelle.
/// Alle Methoden sind fail-closed: unbekannte Adressen sind ein Fehler.
pub trait FrameBackend {
    /// Neuen 4-KiB-Frame allokieren (None = erschöpft, fail-closed).
    fn alloc_frame(&mut self) -> Option<PhysFrame>;
    /// Frame zurückgeben (Rollback / Unmap). Doppelt-Freigabe ist ein Bug
    /// und muss im Backend als Panik/Fehler sichtbar werden.
    fn free_frame(&mut self, frame: PhysFrame);
    /// 8-Byte-aligneden u64 aus dem physischen Speicher lesen.
    fn read_phys_u64(&mut self, addr: u64) -> Result<u64, VmmError>;
    /// 8-Byte-aligneden u64 in den physischen Speicher schreiben.
    fn write_phys_u64(&mut self, addr: u64, value: u64) -> Result<(), VmmError>;
}

/// Transaktions-Journal: jede vergebene InterTable-Entry (Adresse, alter Wert)
/// und jeder neu allokierter Frame wird vermerkt, damit ein Fehlschlag
/// mitten im Walk die Seitentabellen-Hierarchie vollständig zurückrollt.
#[derive(Default)]
pub struct MapJournal {
    allocated: alloc::vec::Vec<PhysFrame>,
    writes: alloc::vec::Vec<(u64, u64)>,
}

/// Index-Berechnung je Level (9 Bits pro Ebene).
pub const fn table_index(virt: u64, shift: u32) -> usize {
    ((virt >> shift) & 0o777) as usize
}

/// Hardware-Seitentababen-Mapper: führt `map_page` über die echte
/// 4-Level-Hierarchie aus. Vor jeder Mutation läuft die M1.2-Validierung.
pub struct HardwareMapper<'a> {
    pml4: PhysFrame,
    backend: &'a mut dyn FrameBackend,
}

impl<'a> HardwareMapper<'a> {
    pub fn new(pml4: PhysFrame, backend: &'a mut dyn FrameBackend) -> Self {
        Self { pml4, backend }
    }

    /// Eine Seite (4 KiB) mappen. Fail-closed und transaktional: schlägt
    /// irgendein Schritt fehl, werden alle allokierten InterTables
    /// zurückgegeben und alle Entry-Modifikationen zurückgeschrieben.
    pub fn map_page(
        &mut self,
        virt: u64,
        frame: PhysFrame,
        flags: MappingFlags,
    ) -> Result<(), VmmError> {
        // Validierung VOR der ersten Allokation (M1.2-Kern, inkl. W^X,
        // User-Intervall, Kanonizität, HHDM-Policy).
        self.validate_target_policy(virt, flags)?;
        PhysFrame::new(frame.0)?;

        let mut journal = MapJournal::default();
        let result = self.map_page_inner(virt, frame, flags, &mut journal);
        if result.is_err() {
            self.rollback(journal);
        }
        result
    }

    fn map_page_inner(
        &mut self,
        virt: u64,
        frame: PhysFrame,
        flags: MappingFlags,
        journal: &mut MapJournal,
    ) -> Result<(), VmmError> {
        let user = flags.user_accessible;

        let pdpt = self.descend(
            self.pml4,
            table_index(virt, 39),
            PageTableLevel::Pml4,
            user,
            journal,
        )?;
        let pd = self.descend(
            pdpt,
            table_index(virt, 30),
            PageTableLevel::Pdpt,
            user,
            journal,
        )?;
        let pt = self.descend(pd, table_index(virt, 21), PageTableLevel::Pd, user, journal)?;

        // Leaf-Entry in PT: existierende Mapping ist ein Fehler (kein
        // stillschweigendes Remap).
        let entry_addr = pt.0 + (table_index(virt, 12) * 8) as u64;
        let old = self.backend.read_phys_u64(entry_addr)?;
        if old & 1 != 0 {
            return Err(VmmError::AlreadyMapped);
        }
        let x86 = flags.to_x86_flags()?;
        journal.writes.push((entry_addr, old));
        self.backend.write_phys_u64(entry_addr, frame.0 | x86)
    }

    /// Eine Hierarchie-Ebene absteigen: Entry lesen; fehlt sie, wird ein neuer
    /// InterTable transaktional allokiert, genullt und verdrahtet.
    fn descend(
        &mut self,
        table: PhysFrame,
        index: usize,
        level: PageTableLevel,
        user: bool,
        journal: &mut MapJournal,
    ) -> Result<PhysFrame, VmmError> {
        let entry_addr = table.0 + (index * 8) as u64;
        let old = self.backend.read_phys_u64(entry_addr)?;

        if old & 1 != 0 {
            // Existierenden InterTable nutzen; Huge-Page-Bits sind auf
            // PDPT/PD-Ebene in M1.2 verboten (level-spezifische PS-Semantik).
            level.reject_huge_page(old)?;
            return PhysFrame::new(old & PHYS_ADDR_MASK);
        }

        // Neuen InterTable allokieren und nullen.
        let new_table = self
            .backend
            .alloc_frame()
            .ok_or(VmmError::FrameAllocationFailed)?;
        journal.allocated.push(new_table);
        for i in 0..512u64 {
            self.backend.write_phys_u64(new_table.0 + i * 8, 0)?;
        }

        // Entry verdrahten: Present | Writable (+ User nur bei User-Mapping).
        let mut entry = 1u64 | (1u64 << 1);
        if user {
            entry |= 1u64 << 2;
        }
        journal.writes.push((entry_addr, old));
        self.backend
            .write_phys_u64(entry_addr, new_table.0 | entry)?;
        Ok(new_table)
    }

    /// Seitenauflösung ohne Mutation (Diagnose/Verifikation).
    pub fn translate(&mut self, virt: u64) -> Result<PhysFrame, VmmError> {
        VirtAddr::new(virt)?;
        let mut table = self.pml4;
        for (shift, level) in [
            (39u32, PageTableLevel::Pml4),
            (30, PageTableLevel::Pdpt),
            (21, PageTableLevel::Pd),
        ] {
            let entry = self
                .backend
                .read_phys_u64(table.0 + (table_index(virt, shift) * 8) as u64)?;
            if entry & 1 == 0 {
                return Err(VmmError::NotMapped);
            }
            level.reject_huge_page(entry)?;
            table = PhysFrame::new(entry & PHYS_ADDR_MASK)?;
        }
        let leaf = self
            .backend
            .read_phys_u64(table.0 + (table_index(virt, 12) * 8) as u64)?;
        if leaf & 1 == 0 {
            return Err(VmmError::NotMapped);
        }
        PhysFrame::new(leaf & PHYS_ADDR_MASK)
    }

    /// Ziel-Policy vor jeder Allokation: M1.2-Kern + HHDM-Policy.
    /// HHDM-Mappings sind zwingend Supervisor/RW/NX.
    fn validate_target_policy(&mut self, virt: u64, flags: MappingFlags) -> Result<(), VmmError> {
        validate_mapping_target(virt, flags)?;
        // Mappings sind seitenscharf: eine unaligned virt-Adresse würde
        // eine andere Seite mappen als die Registry bucht (Divergenz).
        if virt & PAGE_MASK != 0 {
            return Err(VmmError::AddressMisaligned);
        }
        if VirtAddr::new(virt)?.is_hhdm() && (!flags.writable || flags.executable) {
            return Err(VmmError::PrivilegeViolation);
        }
        Ok(())
    }

    /// Transaktionaler Rollback: Entry-Werte in umgekehrter Reihenfolge
    /// zurückschreiben, dann allokierte Frames in umgekehrter Reihenfolge
    /// freigeben.
    fn rollback(&mut self, journal: MapJournal) {
        for (addr, old) in journal.writes.iter().rev() {
            let _ = self.backend.write_phys_u64(*addr, *old);
        }
        for frame in journal.allocated.iter().rev() {
            self.backend.free_frame(*frame);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// M1.3: Mapping-Lifetime — Unmap, InterTable-Kollaps, Frame-Refcounting
// ═══════════════════════════════════════════════════════════════════════════

impl<'a> HardwareMapper<'a> {
    /// Eine Seite unmappen: Leaf-Entry löschen und anschließend —
    /// bottom-up — leere InterTables freigeben (Entry im Parent clearen,
    /// Frame zurückgeben). Die PML4 selbst wird nie freigegeben.
    /// Reale TLB-Invalidierung folgt in der Hardware-Kette; hier bleibt
    /// die Mutation rein tabellarisch und deterministisch.
    /// Reihenfolge: Leaf VOR Frame-Freigabe (kein Present-Eintrag zeigt je
    /// auf einen freigegebenen Frame).
    pub fn unmap_page(&mut self, virt: u64) -> Result<PhysFrame, VmmError> {
        VirtAddr::new(virt)?;
        if virt & PAGE_MASK != 0 {
            return Err(VmmError::AddressMisaligned);
        }

        // Pfad sammeln: je Ebene (Parent-Entry-Adresse, Tabellen-Frame).
        // Level-spezifische PS-Prüfung wie bei translate(): ein Huge-Page-
        // Entry ist KEINE Tabelle — descendieren würde einen beliebigen
        // Frame als Tabelle interpretieren und später freigeben.
        let mut path: alloc::vec::Vec<(u64, PhysFrame)> = alloc::vec::Vec::new();
        let mut table = self.pml4;
        for (shift, level) in [
            (39u32, PageTableLevel::Pml4),
            (30, PageTableLevel::Pdpt),
            (21, PageTableLevel::Pd),
        ] {
            let entry_addr = table.0 + (table_index(virt, shift) * 8) as u64;
            let entry = self.backend.read_phys_u64(entry_addr)?;
            if entry & 1 == 0 {
                return Err(VmmError::NotMapped);
            }
            level.reject_huge_page(entry)?;
            let next = PhysFrame::new(entry & PHYS_ADDR_MASK)?;
            path.push((entry_addr, next));
            table = next;
        }
        let leaf_addr = table.0 + (table_index(virt, 12) * 8) as u64;
        let leaf = self.backend.read_phys_u64(leaf_addr)?;
        if leaf & 1 == 0 {
            return Err(VmmError::NotMapped);
        }
        let frame = PhysFrame::new(leaf & PHYS_ADDR_MASK)?;

        // 1) Leaf löschen.
        self.backend.write_phys_u64(leaf_addr, 0)?;

        // 2) Bottom-up InterTable-Kollaps: leere Tabellen freigeben.
        for (parent_entry_addr, child_table) in path.into_iter().rev() {
            if self.table_has_present_entries(child_table)? {
                break;
            }
            self.backend.write_phys_u64(parent_entry_addr, 0)?;
            self.backend.free_frame(child_table);
        }
        Ok(frame)
    }

    fn table_has_present_entries(&mut self, table: PhysFrame) -> Result<bool, VmmError> {
        for i in 0..512u64 {
            if self.backend.read_phys_u64(table.0 + i * 8)? & 1 != 0 {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// VMM mit Mapping-Lifetime-Modell: jede `map_owned`-Seite hält eine
/// Referenz auf ihren Frame; fällt der letzte Reference-Count auf 0, wird
/// der Frame an das Backend zurückgegeben. Raw-Mappings ohne Ownership
/// (MMIO, Kernel-Statisches) bleiben `map_page`-Mechanik ohne Registry.
pub struct Vmm {
    pml4: PhysFrame,
    registry: alloc::collections::BTreeMap<u64, PhysFrame>,
    frame_refs: alloc::collections::BTreeMap<u64, usize>,
}

impl Vmm {
    pub fn new(pml4: PhysFrame) -> Self {
        Self {
            pml4,
            registry: alloc::collections::BTreeMap::new(),
            frame_refs: alloc::collections::BTreeMap::new(),
        }
    }

    /// Seite mit Frame-Ownership mappen. Der Frame wird erst beim letzten
    /// Unmap freigegeben (Referenzmodell).
    pub fn map_owned(
        &mut self,
        backend: &mut dyn FrameBackend,
        virt: u64,
        frame: PhysFrame,
        flags: MappingFlags,
    ) -> Result<(), VmmError> {
        HardwareMapper::new(self.pml4, backend).map_page(virt, frame, flags)?;
        // map_page hat AlreadyMapped abgelehnt — Registry-Insert ist sicher.
        if self.registry.insert(virt, frame).is_some() {
            // Kann nach AlreadyMapped-Gate nicht eintreten; fail-closed
            // trotzdem zurückrollen, damit Registry nie Phantom-Einträge hat.
            HardwareMapper::new(self.pml4, backend).unmap_page(virt)?;
            return Err(VmmError::AlreadyMapped);
        }
        *self.frame_refs.entry(frame.0).or_insert(0) += 1;
        Ok(())
    }

    /// Seite unmappen und Referenz abgeben. Liefert den Frame zurück;
    /// der Aufrufer sieht an `refcount`, ob der Frame physisch freigegeben
    /// wurde (0 = freigegeben).
    pub fn unmap(
        &mut self,
        backend: &mut dyn FrameBackend,
        virt: u64,
    ) -> Result<PhysFrame, VmmError> {
        if virt & PAGE_MASK != 0 {
            return Err(VmmError::AddressMisaligned);
        }
        let frame = match self.registry.get(&virt) {
            Some(f) => *f,
            None => return Err(VmmError::NotMapped),
        };

        let freed = HardwareMapper::new(self.pml4, backend).unmap_page(virt)?;
        assert_eq!(freed.0, frame.0, "Registry/PT-Divergenz: Buchungsfehler");

        self.registry.remove(&virt);
        let refs = self
            .frame_refs
            .get_mut(&frame.0)
            .expect("Refcount fehlt — Registry/Refcount-Divergenz");
        *refs -= 1;
        if *refs == 0 {
            self.frame_refs.remove(&frame.0);
            backend.free_frame(frame);
        }
        Ok(frame)
    }

    /// Aktueller Referenz-Count eines Frames (0 = nicht im VMM registriert).
    pub fn refcount(&self, frame: PhysFrame) -> usize {
        self.frame_refs.get(&frame.0).copied().unwrap_or(0)
    }

    /// Anzahl registrierter Owned-Mappings (Diagnose/Invariante).
    pub fn mapping_count(&self) -> usize {
        self.registry.len()
    }

    pub fn translate(
        &mut self,
        backend: &mut dyn FrameBackend,
        virt: u64,
    ) -> Result<PhysFrame, VmmError> {
        HardwareMapper::new(self.pml4, backend).translate(virt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_bounds_are_half_open() {
        assert!(validate_user_range(USER_SPACE_BASE, 1).is_ok());
        assert!(validate_user_range(USER_SPACE_TOP_EXCLUSIVE - 1, 1).is_ok());
        assert_eq!(
            validate_user_range(USER_SPACE_TOP_EXCLUSIVE, 1),
            Err(VmmError::RangeOutOfBounds)
        );
        assert_eq!(
            validate_user_range(USER_SPACE_TOP_EXCLUSIVE - 1, 2),
            Err(VmmError::RangeOutOfBounds)
        );
        assert_eq!(
            validate_user_range(USER_SPACE_BASE, u64::MAX),
            Err(VmmError::RangeOutOfBounds)
        );
    }

    #[test]
    fn exact_upper_boundary_is_valid_only_when_end_equals_top() {
        assert!(validate_user_range(USER_SPACE_TOP_EXCLUSIVE - 0x1000, 0x1000).is_ok());
        assert_eq!(
            validate_user_range(USER_SPACE_TOP_EXCLUSIVE - 0x1000, 0x1001),
            Err(VmmError::RangeOutOfBounds)
        );
        assert_eq!(
            validate_user_range(USER_SPACE_TOP_EXCLUSIVE, 1),
            Err(VmmError::RangeOutOfBounds)
        );
    }

    #[test]
    fn kernel_high_half_cannot_be_user() {
        assert_eq!(
            validate_mapping_target(HHDM_BASE - 0x1000, MappingFlags::user_read_only()),
            Err(VmmError::PrivilegeViolation)
        );
        assert_eq!(
            validate_mapping_target(HHDM_BASE, MappingFlags::user_read_only()),
            Err(VmmError::PrivilegeViolation)
        );
        assert_eq!(
            validate_mapping_target(USER_SPACE_TOP_EXCLUSIVE, MappingFlags::user_read_only()),
            Err(VmmError::PrivilegeViolation)
        );
    }

    #[test]
    fn user_mapping_inside_exact_interval_is_allowed() {
        assert!(validate_mapping_target(USER_SPACE_BASE, MappingFlags::user_read_only()).is_ok());
        assert!(validate_mapping_target(
            USER_SPACE_TOP_EXCLUSIVE - PAGE_SIZE,
            MappingFlags::user_read_only()
        )
        .is_ok());
    }

    #[test]
    fn wx_is_rejected() {
        let flags = MappingFlags {
            writable: true,
            user_accessible: true,
            executable: true,
            global: false,
            cache_disable: false,
        };
        assert_eq!(flags.to_x86_flags(), Err(VmmError::InvalidPageFlags));
    }

    #[test]
    fn physical_baseline_is_48_bit() {
        assert!(PhysFrame::new(MAX_PHYS_ADDR & !PAGE_MASK).is_ok());
        assert_eq!(
            PhysFrame::new(1 << 48),
            Err(VmmError::InvalidPhysicalAddress)
        );
        assert_eq!(
            PhysFrame::new(0x1001),
            Err(VmmError::InvalidPhysicalAddress)
        );
    }

    #[test]
    fn ps_is_level_specific() {
        assert!(!PageTableLevel::Pml4.huge_page_bit_is_ps());
        assert!(PageTableLevel::Pdpt.huge_page_bit_is_ps());
        assert!(PageTableLevel::Pd.huge_page_bit_is_ps());
        assert!(!PageTableLevel::Pt.huge_page_bit_is_ps());
        assert!(PageTableLevel::Pt.reject_huge_page(1 << 7).is_ok()); // PAT
        assert_eq!(
            PageTableLevel::Pd.reject_huge_page(1 << 7),
            Err(VmmError::HugePageNotSupported)
        );
    }

    #[test]
    fn hhdm_is_supervisor_rw_nx() {
        assert!(validate_hhdm_entry(HHDM_BASE, true, false, true).is_ok());
        assert_eq!(
            validate_hhdm_entry(HHDM_BASE, false, false, true),
            Err(VmmError::PrivilegeViolation)
        );
        assert_eq!(
            validate_hhdm_entry(HHDM_BASE, true, true, true),
            Err(VmmError::PrivilegeViolation)
        );
        assert_eq!(
            validate_hhdm_entry(HHDM_BASE, true, false, false),
            Err(VmmError::PrivilegeViolation)
        );
        assert_eq!(
            validate_hhdm_entry(HHDM_END, true, false, true),
            Err(VmmError::RangeOutOfBounds)
        );
    }

    #[test]
    fn bootstrap_translation_is_contained() {
        let mapping = BootstrapMapping::new(0x1000, 0x5000, HHDM_BASE).unwrap();
        assert_eq!(
            mapping.translate(0x2000, 0x1000).unwrap(),
            HHDM_BASE + 0x2000
        );
        assert!(mapping.translate(0x4000, 0x1000).is_ok());
        assert_eq!(
            mapping.translate(0x4000, 0x1001),
            Err(VmmError::InvalidPhysicalAddress)
        );
        assert_eq!(
            mapping.translate(u64::MAX, 1),
            Err(VmmError::InvalidPhysicalAddress)
        );
    }

    // ── HardwareMapper: deterministisches Backend ─────────────────────────
    use alloc::collections::{BTreeMap, BTreeSet};

    struct TestBackend {
        next_frame: u64,
        outstanding: usize,         // aktuell vergebene Frames (Leak-Detektor)
        alloc_quota: Option<usize>, // Some(n): nur noch n Frames, dann None
        outstanding_addrs: BTreeSet<u64>, // Frame-Identität (Double-Free-Detektor)
        memory: BTreeMap<u64, u64>,
    }

    impl TestBackend {
        fn new() -> Self {
            Self {
                next_frame: 0x1000,
                outstanding: 0,
                alloc_quota: None,
                outstanding_addrs: BTreeSet::new(),
                memory: BTreeMap::new(),
            }
        }
    }

    impl FrameBackend for TestBackend {
        fn alloc_frame(&mut self) -> Option<PhysFrame> {
            if let Some(quota) = self.alloc_quota {
                if quota == 0 {
                    return None;
                }
                self.alloc_quota = Some(quota - 1);
            }
            let f = PhysFrame::new(self.next_frame).ok()?;
            self.next_frame += PAGE_SIZE;
            self.outstanding += 1;
            assert!(
                self.outstanding_addrs.insert(f.0),
                "Frame doppelt vergeben: {:#x}",
                f.0
            );
            Some(f)
        }

        fn free_frame(&mut self, frame: PhysFrame) {
            assert!(self.outstanding > 0, "free ohne alloc (double free)");
            assert!(
                self.outstanding_addrs.remove(&frame.0),
                "Frame {:#x} doppelt freigegeben (Double-Free)",
                frame.0
            );
            self.outstanding -= 1;
        }

        fn read_phys_u64(&mut self, addr: u64) -> Result<u64, VmmError> {
            Ok(self.memory.get(&addr).copied().unwrap_or(0))
        }

        fn write_phys_u64(&mut self, addr: u64, value: u64) -> Result<(), VmmError> {
            self.memory.insert(addr, value);
            Ok(())
        }
    }

    fn leaf_entry(backend: &mut TestBackend, pml4: PhysFrame, virt: u64) -> u64 {
        let mut table = pml4;
        for shift in [39, 30, 21] {
            let entry = backend
                .read_phys_u64(table.0 + (table_index(virt, shift) * 8) as u64)
                .unwrap();
            table = PhysFrame(entry & PHYS_ADDR_MASK);
        }
        backend
            .read_phys_u64(table.0 + (table_index(virt, 12) * 8) as u64)
            .unwrap()
    }

    #[test]
    fn map_user_page_creates_full_hierarchy() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let frame = PhysFrame::new(0x9000).unwrap();

        HardwareMapper::new(pml4, &mut backend)
            .map_page(USER_SPACE_BASE, frame, MappingFlags::user_read_write())
            .unwrap();

        assert_eq!(
            HardwareMapper::new(pml4, &mut backend)
                .translate(USER_SPACE_BASE)
                .unwrap()
                .0,
            frame.0
        );
        // Volle Hierarchie: PDPT + PD + PT = 3 neue InterTables.
        assert_eq!(backend.outstanding, 3);
        // Leaf: Present | RW | User | NX (read_write ist nicht executable).
        let leaf = leaf_entry(&mut backend, pml4, USER_SPACE_BASE);
        assert_eq!(leaf & 1, 1);
        assert_eq!(leaf & 2, 2);
        assert_eq!(leaf & 4, 4);
        assert_eq!(leaf >> 63, 1);
        assert_eq!(leaf & PHYS_ADDR_MASK, frame.0);
    }

    #[test]
    fn second_page_in_same_pt_reuses_hierarchy() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();

        HardwareMapper::new(pml4, &mut backend)
            .map_page(
                USER_SPACE_BASE,
                PhysFrame::new(0x9000).unwrap(),
                MappingFlags::user_read_write(),
            )
            .unwrap();
        assert_eq!(backend.outstanding, 3);

        // Zweite Seite im selben 2-MiB-PT: keine neuen InterTables.
        HardwareMapper::new(pml4, &mut backend)
            .map_page(
                USER_SPACE_BASE + PAGE_SIZE,
                PhysFrame::new(0xa000).unwrap(),
                MappingFlags::user_read_only(),
            )
            .unwrap();
        assert_eq!(backend.outstanding, 3);
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend)
                .translate(USER_SPACE_BASE + PAGE_SIZE)
                .unwrap()
                .0,
            0xa000
        );
    }

    #[test]
    fn double_map_is_rejected_without_leak() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let flags = MappingFlags::user_read_write();
        HardwareMapper::new(pml4, &mut backend)
            .map_page(USER_SPACE_BASE, PhysFrame::new(0x9000).unwrap(), flags)
            .unwrap();

        // Erneutes Mappen derselben Seite: AlreadyMapped, kein Leak.
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend).map_page(
                USER_SPACE_BASE,
                PhysFrame::new(0x91000).unwrap(),
                flags
            ),
            Err(VmmError::AlreadyMapped)
        );
        assert_eq!(backend.outstanding, 3);
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend)
                .translate(USER_SPACE_BASE)
                .unwrap()
                .0,
            0x9000 // altes Mapping unangetastet
        );
    }

    #[test]
    fn validation_failure_consumes_no_frames() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();

        // W^X
        let wx = MappingFlags {
            writable: true,
            user_accessible: true,
            executable: true,
            global: false,
            cache_disable: false,
        };
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend).map_page(
                USER_SPACE_BASE,
                PhysFrame::new(0x9000).unwrap(),
                wx
            ),
            Err(VmmError::InvalidPageFlags)
        );
        // User-Mapping in den Kernel-Halbraum
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend).map_page(
                HHDM_BASE,
                PhysFrame::new(0x9000).unwrap(),
                MappingFlags::user_read_only()
            ),
            Err(VmmError::PrivilegeViolation)
        );
        // Nicht-kanonische Adresse (User-Intervall verlassen)
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend).map_page(
                0x0000_8000_0000_0000,
                PhysFrame::new(0x9000).unwrap(),
                MappingFlags::user_read_only()
            ),
            Err(VmmError::PrivilegeViolation)
        );
        assert_eq!(
            backend.outstanding, 0,
            "Validierungsfehler darf keine Frames verbrauchen"
        );
    }

    #[test]
    fn hhdm_policy_is_supervisor_rw_nx() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();

        // HHDM + executable → PrivilegeViolation
        let rx = MappingFlags {
            writable: false,
            user_accessible: false,
            executable: true,
            global: true,
            cache_disable: false,
        };
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend).map_page(
                HHDM_BASE,
                PhysFrame::new(0x9000).unwrap(),
                rx
            ),
            Err(VmmError::PrivilegeViolation)
        );
        assert_eq!(backend.outstanding, 0);
        // HHDM + kernel_rw_nx → erlaubt
        assert!(HardwareMapper::new(pml4, &mut backend)
            .map_page(
                HHDM_BASE,
                PhysFrame::new(0x9000).unwrap(),
                MappingFlags::kernel_rw_nx()
            )
            .is_ok());
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend)
                .translate(HHDM_BASE)
                .unwrap()
                .0,
            0x9000
        );
    }

    #[test]
    fn allocator_exhaustion_rolls_back_transaction() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();

        // Erste Map: konsumiert 3 InterTables.
        HardwareMapper::new(pml4, &mut backend)
            .map_page(
                HHDM_BASE,
                PhysFrame::new(0x9000).unwrap(),
                MappingFlags::kernel_rw_nx(),
            )
            .unwrap();
        assert_eq!(backend.outstanding, 3);

        // Zweite Map in anderer 1-GiB-Region: PML4/PDPT existieren schon,
        // es werden nur PD + PT angelegt (+2).
        HardwareMapper::new(pml4, &mut backend)
            .map_page(
                HHDM_BASE + 0x4000_0000,
                PhysFrame::new(0xb000).unwrap(),
                MappingFlags::kernel_rw_nx(),
            )
            .unwrap();
        assert_eq!(backend.outstanding, 5);

        // Dritte Map in einer weiteren 1-GiB-Region: braucht PD + PT (+2),
        // aber das Allokations-Quota erlaubt nur EINEN Frame → der Walk
        // scheitert MITTLEREN (nach der PD-Allokation) → echter
        // transaktionaler Rollback, nicht nur Fehler vor Allokation.
        backend.alloc_quota = Some(1);
        let r = HardwareMapper::new(pml4, &mut backend).map_page(
            HHDM_BASE + 0x8000_0000,
            PhysFrame::new(0xc000).unwrap(),
            MappingFlags::kernel_rw_nx(),
        );
        assert_eq!(r, Err(VmmError::FrameAllocationFailed));

        // Rollback-Evidenz: der allokierte PD-Frame wurde zurückgegeben,
        // die PDPT-Entry (idx 8) ist auf dem alten Wert 0 zurückgeschrieben.
        assert_eq!(
            backend.outstanding, 5,
            "Rollback muss alle Neuallokationen freigeben"
        );
        // PDPT-Tabelle = erster allozierter Frame (0x1000) der ersten Map.
        let pdpt_entry = backend
            .read_phys_u64(0x1000 + (table_index(HHDM_BASE + 0x8000_0000, 30) * 8) as u64)
            .unwrap();
        assert_eq!(
            pdpt_entry, 0,
            "InterTable-Entry muss auf den alten Wert zurückgeschrieben sein"
        );
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend)
                .translate(HHDM_BASE)
                .unwrap()
                .0,
            0x9000
        );
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend)
                .translate(HHDM_BASE + 0x4000_0000)
                .unwrap()
                .0,
            0xb000
        );
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend).translate(HHDM_BASE + 0x8000_0000),
            Err(VmmError::NotMapped)
        );
    }

    #[test]
    fn translate_unmapped_is_not_mapped() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend).translate(USER_SPACE_BASE),
            Err(VmmError::NotMapped)
        );
    }

    #[test]
    fn huge_page_entry_rejects_walk() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();

        // Manuell eine PS-Huge-Page in die PDPT-Ebene schreiben.
        let pdpt_frame = PhysFrame::new(0x9000).unwrap();
        backend
            .write_phys_u64(
                pml4.0 + (table_index(USER_SPACE_BASE, 39) * 8) as u64,
                pdpt_frame.0 | 1,
            )
            .unwrap();
        backend
            .write_phys_u64(
                pdpt_frame.0 + (table_index(USER_SPACE_BASE, 30) * 8) as u64,
                0x1000 | 1 | (1 << 7), // Present + PS (1-GiB) → M1.2 verboten
            )
            .unwrap();

        assert_eq!(
            HardwareMapper::new(pml4, &mut backend).map_page(
                USER_SPACE_BASE,
                PhysFrame::new(0xb000).unwrap(),
                MappingFlags::user_read_write()
            ),
            Err(VmmError::HugePageNotSupported)
        );
        assert_eq!(backend.outstanding, 0);
    }

    // ── M1.3: Lifetime-/Referenzmodell ───────────────────────────────────

    #[test]
    fn unmap_collapses_empty_hierarchy() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let mut vmm = Vmm::new(pml4);

        let frame = backend.alloc_frame().unwrap();
        vmm.map_owned(
            &mut backend,
            USER_SPACE_BASE,
            frame,
            MappingFlags::user_read_write(),
        )
        .unwrap();
        // 3 InterTables + 1 gemapter Frame.
        assert_eq!(backend.outstanding, 4);

        // Unmap: Leaf gelöscht, PT/PD/PDPT leer → Kollaps + Frame-Freigabe.
        let freed = vmm.unmap(&mut backend, USER_SPACE_BASE).unwrap();
        assert_eq!(freed.0, frame.0);
        assert_eq!(
            backend.outstanding, 0,
            "Leere Hierarchie muss vollständig kollabieren"
        );
        assert_eq!(
            vmm.translate(&mut backend, USER_SPACE_BASE),
            Err(VmmError::NotMapped)
        );
        assert_eq!(vmm.mapping_count(), 0);
        assert_eq!(vmm.refcount(frame), 0);
    }

    #[test]
    fn unmap_keeps_nonempty_tables() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let mut vmm = Vmm::new(pml4);

        let f1 = backend.alloc_frame().unwrap();
        let f2 = backend.alloc_frame().unwrap();
        vmm.map_owned(
            &mut backend,
            USER_SPACE_BASE,
            f1,
            MappingFlags::user_read_write(),
        )
        .unwrap();
        vmm.map_owned(
            &mut backend,
            USER_SPACE_BASE + PAGE_SIZE,
            f2,
            MappingFlags::user_read_write(),
        )
        .unwrap();
        assert_eq!(backend.outstanding, 5); // 3 Tabellen + 2 Frames

        // Erste Seite unmapen: PT ist noch nicht leer (2. Seite aktiv).
        vmm.unmap(&mut backend, USER_SPACE_BASE).unwrap();
        assert_eq!(
            backend.outstanding, 4,
            "Nicht-leere Tabellen müssen bleiben"
        );
        assert_eq!(
            vmm.translate(&mut backend, USER_SPACE_BASE + PAGE_SIZE)
                .unwrap()
                .0,
            f2.0
        );

        // Zweite Seite: jetzt kollabiert die komplette Hierarchie.
        vmm.unmap(&mut backend, USER_SPACE_BASE + PAGE_SIZE)
            .unwrap();
        assert_eq!(backend.outstanding, 0);
    }

    #[test]
    fn shared_frame_outlives_partial_unmap() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let mut vmm = Vmm::new(pml4);

        // Ein physikalischer Frame, zwei virtuelle Seiten in verschiedenen
        // 2-MiB-Regionen (shared page, z.B. fork/CoW-Vorbereitung).
        let frame = backend.alloc_frame().unwrap();
        let v1 = USER_SPACE_BASE;
        let v2 = USER_SPACE_BASE + 0x200000; // gleiche PD-Region, neuer PT
        vmm.map_owned(&mut backend, v1, frame, MappingFlags::user_read_write())
            .unwrap();
        vmm.map_owned(&mut backend, v2, frame, MappingFlags::user_read_only())
            .unwrap();
        assert_eq!(vmm.refcount(frame), 2);
        // 3 InterTables + 1 zusätzlicher PT + 1 Frame.
        assert_eq!(backend.outstanding, 5);

        // Unmap v1: Frame bleibt (Refcount 2→1), PT1 kollabiert.
        vmm.unmap(&mut backend, v1).unwrap();
        assert_eq!(vmm.refcount(frame), 1);
        assert_eq!(
            backend.outstanding, 4,
            "Shared-Frame darf nicht freigegeben werden"
        );
        assert_eq!(vmm.translate(&mut backend, v2).unwrap().0, frame.0);
        assert_eq!(vmm.translate(&mut backend, v1), Err(VmmError::NotMapped));

        // Unmap v2: letzter Referenz-Count → Frame + Kaskade frei.
        vmm.unmap(&mut backend, v2).unwrap();
        assert_eq!(vmm.refcount(frame), 0);
        assert_eq!(backend.outstanding, 0);
    }

    #[test]
    fn unmap_unmapped_is_rejected() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let mut vmm = Vmm::new(pml4);

        assert_eq!(
            vmm.unmap(&mut backend, USER_SPACE_BASE),
            Err(VmmError::NotMapped)
        );
        assert_eq!(backend.outstanding, 0);

        // Registry-lose Adresse (raw map ohne Ownership): unmap über Vmm
        // muss trotzdem als NotMapped abgelehnt werden — Vmm verwalten nur
        // registrierte Mappings.
        HardwareMapper::new(pml4, &mut backend)
            .map_page(
                USER_SPACE_BASE + 0x400000,
                PhysFrame::new(0x9000).unwrap(),
                MappingFlags::user_read_write(),
            )
            .unwrap();
        assert_eq!(
            vmm.unmap(&mut backend, USER_SPACE_BASE + 0x400000),
            Err(VmmError::NotMapped)
        );
    }

    #[test]
    fn map_owned_validation_leaves_no_phantom_registry() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let mut vmm = Vmm::new(pml4);

        // W^X wird vor der Allokation abgelehnt — keine Registry-Einträge.
        let wx = MappingFlags {
            writable: true,
            user_accessible: true,
            executable: true,
            global: false,
            cache_disable: false,
        };
        assert!(vmm
            .map_owned(
                &mut backend,
                USER_SPACE_BASE,
                PhysFrame::new(0x9000).unwrap(),
                wx
            )
            .is_err());
        assert_eq!(vmm.mapping_count(), 0);
        assert_eq!(backend.outstanding, 0);

        // Kernel-Adresse mit User-Flags: likewise.
        assert!(vmm
            .map_owned(
                &mut backend,
                HHDM_BASE,
                PhysFrame::new(0x9000).unwrap(),
                MappingFlags::user_read_only()
            )
            .is_err());
        assert_eq!(vmm.mapping_count(), 0);
        assert_eq!(backend.outstanding, 0);
    }

    #[test]
    fn remap_after_collapse_reallocates_tables() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let mut vmm = Vmm::new(pml4);

        let frame = backend.alloc_frame().unwrap();
        vmm.map_owned(
            &mut backend,
            USER_SPACE_BASE,
            frame,
            MappingFlags::user_read_write(),
        )
        .unwrap();
        vmm.unmap(&mut backend, USER_SPACE_BASE).unwrap();
        assert_eq!(backend.outstanding, 0);

        // Erneutes Mappen derselben Region muss die Hierarchie NEU anlegen —
        // ein vergessener Entry-Rest würde sofort AlreadyMapped schmeißen.
        let frame2 = backend.alloc_frame().unwrap();
        vmm.map_owned(
            &mut backend,
            USER_SPACE_BASE,
            frame2,
            MappingFlags::user_read_write(),
        )
        .unwrap();
        assert_eq!(backend.outstanding, 4);
        assert_eq!(
            vmm.translate(&mut backend, USER_SPACE_BASE).unwrap().0,
            frame2.0
        );
    }

    // ── M1.2 Security-Audit-Regressionen ────────────────────────────────

    #[test]
    fn unmap_rejects_huge_page_entry() {
        // Finding: unmap_page descendiert PS-Entries nicht als Tabelle —
        // ein Huge-Page-Entry darf beim Walk nicht als Tabellen-Frame
        // interpretiert (und später freigegeben) werden.
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();

        let pdpt_frame = PhysFrame::new(0x9000).unwrap();
        backend
            .write_phys_u64(
                pml4.0 + (table_index(USER_SPACE_BASE, 39) * 8) as u64,
                pdpt_frame.0 | 1,
            )
            .unwrap();
        backend
            .write_phys_u64(
                pdpt_frame.0 + (table_index(USER_SPACE_BASE, 30) * 8) as u64,
                0x1000 | 1 | (1 << 7), // Present + PS (1-GiB-Huge-Page)
            )
            .unwrap();

        {
            let mut mapper = HardwareMapper::new(pml4, &mut backend);
            assert_eq!(
                mapper.unmap_page(USER_SPACE_BASE),
                Err(VmmError::HugePageNotSupported)
            );
        }
        assert_eq!(
            backend.outstanding, 0,
            "kein Frame darf freigegeben worden sein"
        );
        // Der Huge-Page-Entry ist unangetastet.
        let entry = backend
            .read_phys_u64(pdpt_frame.0 + (table_index(USER_SPACE_BASE, 30) * 8) as u64)
            .unwrap();
        assert_eq!(entry, 0x1000 | 1 | (1 << 7));
    }

    #[test]
    fn map_rejects_misaligned_virt() {
        // Finding: unaligned virt würde eine andere Seite mappen als die
        // Registry bucht — seitenverschränkte Buchung ist verboten.
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let mut vmm = Vmm::new(pml4);

        assert_eq!(
            vmm.map_owned(
                &mut backend,
                USER_SPACE_BASE + 1,
                PhysFrame::new(0x9000).unwrap(),
                MappingFlags::user_read_write()
            ),
            Err(VmmError::AddressMisaligned)
        );
        assert_eq!(vmm.mapping_count(), 0);
        drop(vmm);
        assert_eq!(
            backend.outstanding, 0,
            "Alignment-Fehler darf keine Frames verbrauchen"
        );
    }

    #[test]
    fn unmap_rejects_misaligned_virt() {
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let mut vmm = Vmm::new(pml4);
        vmm.map_owned(
            &mut backend,
            USER_SPACE_BASE,
            PhysFrame::new(0x9000).unwrap(),
            MappingFlags::user_read_write(),
        )
        .unwrap();

        assert_eq!(
            vmm.unmap(&mut backend, USER_SPACE_BASE + 0x800),
            Err(VmmError::AddressMisaligned)
        );
        // Das echte Mapping ist unangetastet.
        assert_eq!(
            vmm.translate(&mut backend, USER_SPACE_BASE).unwrap().0,
            0x9000
        );
        assert_eq!(vmm.mapping_count(), 1);
    }

    #[test]
    fn hhdm_read_only_is_rejected() {
        // Ergänzung: HHDM muss RW sein — read-only ist ein Policy-Verstoß.
        let mut backend = TestBackend::new();
        let pml4 = PhysFrame::new(0).unwrap();
        let ro = MappingFlags {
            writable: false,
            user_accessible: false,
            executable: false,
            global: true,
            cache_disable: false,
        };
        assert_eq!(
            HardwareMapper::new(pml4, &mut backend).map_page(
                HHDM_BASE,
                PhysFrame::new(0x9000).unwrap(),
                ro
            ),
            Err(VmmError::PrivilegeViolation)
        );
    }
}

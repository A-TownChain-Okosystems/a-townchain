// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Capability-System (Rust).
//!
//! Portiert capabilities.py (Python, 08.07.2026) nach Rust.
//! Jede Ressource (Speicher, IPC-Kanal, Beschleuniger) ist nur ueber eine
//! Capability zugaenglich. Capabilities koennen delegiert werden, aber nur
//! mit Einschraenkung (Attenuation) — niemals mit Erweiterung.
//!
//! Eigenschaften (durch Rusts Ownership/Typsystem erzwungen):
//! - Rechte-Monotonie: delegierte Cap kann nie mehr Rechte haben als das Original
//! - Keine Forge: Capabilities werden vom Kernel erzeugt, nicht von Prozessen
//! - Widerrufbar: free() entzieht alle abgeleiteten Capabilities
//! - Atomic: Erzeugung/Delegation/Widerruf sind atomare Operationen (Spinlock)

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::ops::{BitOr, BitAnd};

/// Ressourcen-Typen im Kernel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Memory,
    IpcChannel,
    Accelerator,
    FileSystem,
    Network,
}

/// Zugriffsrechte als Bitfield
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rights(pub u8);

impl Rights {
    pub const NONE:  Rights = Rights(0);
    pub const READ:  Rights = Rights(1);
    pub const WRITE: Rights = Rights(2);
    pub const EXEC:  Rights = Rights(4);
    pub const DELEGATE: Rights = Rights(8);
    pub const ALL:   Rights = Rights(1 | 2 | 4 | 8);

    pub fn has(self, other: Rights) -> bool { (self.0 & other.0) == other.0 }
    pub fn from_bits_truncate(bits: u8) -> Rights { Rights(bits & 0x0F) }
    pub fn bits(self) -> u8 { self.0 }
    pub fn is_empty(self) -> bool { self.0 == 0 }
}

impl BitOr for Rights {
    type Output = Rights;
    fn bitor(self, rhs: Rights) -> Rights { Rights(self.0 | rhs.0) }
}

impl BitAnd for Rights {
    type Output = Rights;
    fn bitand(self, rhs: Rights) -> Rights { Rights(self.0 & rhs.0) }
}

/// Eindeutige Capability-ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapId(pub u64);

/// Pid wird aus ats1000 re-exportiert (einheitlicher Typ)
pub use crate::ats1000::Pid;

/// Eine Capability — das Herzstueck des Sicherheitsmodells
#[derive(Debug, Clone)]
pub struct Capability {
    pub id: CapId,
    pub resource_type: ResourceType,
    pub resource_id: u64,
    pub rights: Rights,
    pub owner: Pid,
    /// Eltern-Capability, falls delegiert. None = vom Kernel direkt erzeugt.
    pub parent: Option<CapId>,
}

/// Fehler bei Capability-Operationen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityError {
    NotFound,
    NotOwner,
    NoDelegateRight,
    RightsExceeded,
}

/// Capability-Tabelle — Kernel-intern, geschuetzt durch Spinlock
#[derive(Clone)]
pub struct CapabilityTable {
    caps: BTreeMap<CapId, Capability>,
    by_pid: BTreeMap<Pid, Vec<CapId>>,
    next_id: u64,
}

impl CapabilityTable {
    pub const fn new() -> Self {
        Self { caps: BTreeMap::new(), by_pid: BTreeMap::new(), next_id: 1 }
    }

    /// Erzeugt eine neue Capability fuer einen Prozess.
    pub fn create(&mut self, pid: Pid, resource_type: ResourceType, resource_id: u64, rights: Rights) -> CapId {
        let id = CapId(self.next_id);
        self.next_id += 1;
        let cap = Capability { id, resource_type, resource_id, rights, owner: pid, parent: None };
        self.caps.insert(id, cap);
        self.by_pid.entry(pid).or_default().push(id);
        id
    }

    /// Delegiert eine Capability an einen anderen Prozess.
    /// Rechte koennen nur EINGESCHRAENKT werden (Attenuation).
    pub fn delegate(&mut self, source_pid: Pid, source_cap: CapId, target_pid: Pid, attenuated_rights: Rights) -> Result<CapId, CapabilityError> {
        let parent_cap = self.caps.get(&source_cap).ok_or(CapabilityError::NotFound)?.clone();
        if parent_cap.owner != source_pid { return Err(CapabilityError::NotOwner); }
        if !parent_cap.rights.has(Rights::DELEGATE) { return Err(CapabilityError::NoDelegateRight); }
        if (attenuated_rights.0 & !parent_cap.rights.0) != 0 { return Err(CapabilityError::RightsExceeded); }
        let id = CapId(self.next_id);
        self.next_id += 1;
        let child = Capability { id, resource_type: parent_cap.resource_type, resource_id: parent_cap.resource_id, rights: attenuated_rights, owner: target_pid, parent: Some(source_cap) };
        self.caps.insert(id, child);
        self.by_pid.entry(target_pid).or_default().push(id);
        Ok(id)
    }

    /// Prueft, ob ein Prozess eine bestimmte Capability mit den geforderten Rechten besitzt.
    pub fn check(&self, pid: Pid, resource_type: ResourceType, resource_id: u64, required: Rights) -> bool {
        self.by_pid.get(&pid).map(|caps| caps.iter().any(|&cap_id| {
            if let Some(cap) = self.caps.get(&cap_id) {
                cap.resource_type == resource_type && cap.resource_id == resource_id && cap.rights.has(required)
            } else { false }
        })).unwrap_or(false)
    }

    /// Check capability by cap_id only (any resource type).
    pub fn check_any(&self, pid: Pid, cap_id: u64, required: Rights) -> bool {
        self.caps.get(&CapId(cap_id)).map(|cap| cap.owner == pid && cap.rights.has(required)).unwrap_or(false)
    }

    pub fn get(&self, id: CapId) -> Option<&Capability> { self.caps.get(&id) }

    /// Widerruft eine Capability und alle davon abgeleiteten (rekursiv).
    pub fn revoke(&mut self, cap_id: CapId) {
        let mut to_remove = Vec::new();
        self.collect_descendants(cap_id, &mut to_remove);
        for id in &to_remove { self.caps.remove(id); }
        for pid_caps in self.by_pid.values_mut() { pid_caps.retain(|c| !to_remove.contains(c)); }
    }

    fn collect_descendants(&self, cap_id: CapId, out: &mut Vec<CapId>) {
        out.push(cap_id);
        for (id, cap) in &self.caps {
            if cap.parent == Some(cap_id) { self.collect_descendants(*id, out); }
        }
    }

    pub fn list_for(&self, pid: Pid) -> Vec<&Capability> {
        self.by_pid.get(&pid).map(|caps| caps.iter().filter_map(|id| self.caps.get(id)).collect()).unwrap_or_default()
    }

    pub fn count(&self) -> usize { self.caps.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pid(n: u32) -> Pid { Pid(n) }

    #[test]
    fn test_create_and_check() {
        let mut table = CapabilityTable::new();
        let cap = table.create(pid(1), ResourceType::Memory, 0x1000, Rights::READ | Rights::WRITE);
        assert!(cap.0 > 0);
        assert!(table.check(pid(1), ResourceType::Memory, 0x1000, Rights::READ));
        assert!(table.check(pid(1), ResourceType::Memory, 0x1000, Rights::WRITE));
        assert!(!table.check(pid(1), ResourceType::Memory, 0x1000, Rights::EXEC));
        assert!(!table.check(pid(2), ResourceType::Memory, 0x1000, Rights::READ));
    }

    #[test]
    fn test_delegate_attenuation() {
        let mut table = CapabilityTable::new();
        let parent = table.create(pid(1), ResourceType::IpcChannel, 42, Rights::READ | Rights::WRITE | Rights::DELEGATE);
        let child = table.delegate(pid(1), parent, pid(2), Rights::READ).unwrap();
        assert!(table.check(pid(2), ResourceType::IpcChannel, 42, Rights::READ));
        assert!(!table.check(pid(2), ResourceType::IpcChannel, 42, Rights::WRITE));
    }

    #[test]
    fn test_delegate_rejects_rights_expansion() {
        let mut table = CapabilityTable::new();
        let parent = table.create(pid(1), ResourceType::Memory, 0, Rights::READ | Rights::DELEGATE);
        let result = table.delegate(pid(1), parent, pid(2), Rights::READ | Rights::WRITE);
        assert_eq!(result, Err(CapabilityError::RightsExceeded));
    }

    #[test]
    fn test_delegate_requires_delegate_right() {
        let mut table = CapabilityTable::new();
        let parent = table.create(pid(1), ResourceType::Memory, 0, Rights::READ | Rights::WRITE);
        let result = table.delegate(pid(1), parent, pid(2), Rights::READ);
        assert_eq!(result, Err(CapabilityError::NoDelegateRight));
    }

    #[test]
    fn test_delegate_wrong_owner() {
        let mut table = CapabilityTable::new();
        let parent = table.create(pid(1), ResourceType::Memory, 0, Rights::ALL);
        let result = table.delegate(pid(2), parent, pid(3), Rights::READ);
        assert_eq!(result, Err(CapabilityError::NotOwner));
    }

    #[test]
    fn test_revoke_cascade() {
        let mut table = CapabilityTable::new();
        let root = table.create(pid(1), ResourceType::Memory, 0, Rights::ALL);
        let child = table.delegate(pid(1), root, pid(2), Rights::READ | Rights::DELEGATE).unwrap();
        let grandchild = table.delegate(pid(2), child, pid(3), Rights::READ).unwrap();
        assert!(table.check(pid(1), ResourceType::Memory, 0, Rights::READ));
        assert!(table.check(pid(2), ResourceType::Memory, 0, Rights::READ));
        assert!(table.check(pid(3), ResourceType::Memory, 0, Rights::READ));
        table.revoke(root);
        assert!(!table.check(pid(1), ResourceType::Memory, 0, Rights::READ));
        assert!(!table.check(pid(2), ResourceType::Memory, 0, Rights::READ));
        assert!(!table.check(pid(3), ResourceType::Memory, 0, Rights::READ));
        assert_eq!(table.count(), 0);
    }

    #[test]
    fn test_list_for_process() {
        let mut table = CapabilityTable::new();
        table.create(pid(1), ResourceType::Memory, 0, Rights::READ);
        table.create(pid(1), ResourceType::IpcChannel, 5, Rights::WRITE);
        table.create(pid(2), ResourceType::Memory, 1, Rights::READ);
        let pid1_caps = table.list_for(pid(1));
        assert_eq!(pid1_caps.len(), 2);
        let pid2_caps = table.list_for(pid(2));
        assert_eq!(pid2_caps.len(), 1);
    }

    #[test]
    fn test_rights_operations() {
        let r = Rights::READ | Rights::WRITE;
        assert!(r.has(Rights::READ));
        assert!(r.has(Rights::WRITE));
        assert!(!r.has(Rights::EXEC));
        assert!(!Rights::NONE.has(Rights::READ));
        assert!(Rights::ALL.has(Rights::READ | Rights::WRITE | Rights::EXEC | Rights::DELEGATE));
    }
}

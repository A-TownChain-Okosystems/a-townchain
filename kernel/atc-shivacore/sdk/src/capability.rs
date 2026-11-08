// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Capability System implementation (SC-005).

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CapabilityId(pub u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CapabilityType {
    Endpoint,
    Memory,
    Thread,
    Notification,
    CNode,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CapabilityRights {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub send: bool,
    pub recv: bool,
    pub delegate: bool,
    pub notify: bool,
    pub mint: bool,
    pub revoke: bool,
}

impl CapabilityRights {
    pub fn all() -> Self {
        Self {
            read: true, write: true, execute: true,
            send: true, recv: true, delegate: true,
            notify: true, mint: true, revoke: true,
        }
    }

    pub fn none() -> Self {
        Self {
            read: false, write: false, execute: false,
            send: false, recv: false, delegate: false,
            notify: false, mint: false, revoke: false,
        }
    }

    pub fn read_only() -> Self {
        Self { read: true, ..Self::none() }
    }

    pub fn is_subset_of(&self, parent: &Self) -> bool {
        (!self.read || parent.read) &&
        (!self.write || parent.write) &&
        (!self.execute || parent.execute) &&
        (!self.send || parent.send) &&
        (!self.recv || parent.recv) &&
        (!self.delegate || parent.delegate) &&
        (!self.notify || parent.notify) &&
        (!self.mint || parent.mint) &&
        (!self.revoke || parent.revoke)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capability {
    pub id: CapabilityId,
    pub cap_type: CapabilityType,
    pub rights: CapabilityRights,
    pub badge: u64,
}

impl Capability {
    pub fn new(id: u64, cap_type: CapabilityType, rights: CapabilityRights, badge: u64) -> Self {
        Self {
            id: CapabilityId(id),
            cap_type,
            rights,
            badge,
        }
    }

    pub fn derive(&self, new_id: u64, requested_rights: CapabilityRights, new_badge: u64) -> Result<Self, String> {
        if !self.rights.mint && !self.rights.delegate {
            return Err(String::from("Capability lacks mint/delegate rights"));
        }
        if !requested_rights.is_subset_of(&self.rights) {
            return Err(String::from("Derived rights exceed parent rights (REQ-SC005-07a violation)"));
        }
        let badge = if new_badge != 0 { new_badge } else { self.badge };
        Ok(Self::new(new_id, self.cap_type, requested_rights, badge))
    }
}

pub struct CNode {
    slots: Vec<Option<Capability>>,
    capacity: usize,
}

impl CNode {
    pub fn new(capacity: usize) -> Self {
        let mut slots = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            slots.push(None);
        }
        Self { slots, capacity }
    }

    pub fn get(&self, slot: usize) -> Option<&Capability> {
        if slot < self.capacity {
            self.slots[slot].as_ref()
        } else {
            None
        }
    }

    pub fn insert(&mut self, slot: usize, cap: Capability) -> Result<(), String> {
        if slot >= self.capacity {
            return Err(String::from("Slot out of bounds"));
        }
        if self.slots[slot].is_some() {
            return Err(String::from("Slot already occupied"));
        }
        self.slots[slot] = Some(cap);
        Ok(())
    }

    pub fn revoke(&mut self, slot: usize) -> Result<Capability, String> {
        if slot >= self.capacity {
            return Err(String::from("Slot out of bounds"));
        }
        self.slots[slot].take().ok_or_else(|| String::from("Slot is empty"))
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

pub struct CSpace {
    nodes: Vec<CNode>,
}

impl CSpace {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_cnode(&mut self, node: CNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn lookup(&self, node_idx: usize, slot: usize) -> Option<&Capability> {
        self.nodes.get(node_idx)?.get(slot)
    }
}

impl Default for CSpace {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CapabilityResolver {
    fn resolve(&self, id: CapabilityId) -> bool;
}

impl CapabilityResolver for CSpace {
    fn resolve(&self, id: CapabilityId) -> bool {
        for node in &self.nodes {
            for slot in 0..node.capacity() {
                if let Some(cap) = node.get(slot) {
                    if cap.id == id {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_derivation_limit_semantics() {
        let parent = Capability::new(1, CapabilityType::Endpoint, CapabilityRights::all(), 100);
        let mut read_only_rights = CapabilityRights::read_only();
        read_only_rights.send = true;

        let derived = parent.derive(2, read_only_rights, 200).expect("Derivation should succeed");
        assert_eq!(derived.id, CapabilityId(2));
        assert_eq!(derived.badge, 200);

        // Try deriving with rights exceeding parent
        let mut excessive_rights = CapabilityRights::all();
        let restricted_parent = Capability::new(3, CapabilityType::Endpoint, CapabilityRights::read_only(), 0);
        assert!(restricted_parent.derive(4, excessive_rights, 0).is_err());
    }

    #[test]
    fn test_cnode_and_cspace_lifecycle() {
        let mut cspace = CSpace::new();
        let mut cnode = CNode::new(16);
        let cap = Capability::new(42, CapabilityType::Memory, CapabilityRights::all(), 0);
        cnode.insert(0, cap.clone()).unwrap();

        let node_idx = cspace.add_cnode(cnode);
        assert!(cspace.resolve(CapabilityId(42)));
        assert!(!cspace.resolve(CapabilityId(99)));

        let found = cspace.lookup(node_idx, 0).unwrap();
        assert_eq!(found.id, CapabilityId(42));
    }
}

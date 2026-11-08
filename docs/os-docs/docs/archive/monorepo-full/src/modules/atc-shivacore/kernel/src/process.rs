// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Prozessverwaltung (Rust).
//!
//! Implementiert den ProcessManager-Trait aus ats1000.rs.
//! Jeder Prozess bekommt beim Spawn automatisch Capabilities fuer seinen
//! Adressraum. kill() widerruft alle Caps kaskadierend — der Prozess
//! verliert sofort alle Ressourcen.
//!
//! Eigenschaften:
//! - Kein Prozess ohne Capability (Speicher ist nur ueber Cap zugaenglich)
//! - kill() ist atomar: Caps widerrufen + Prozess als Terminiert markiert
//! - wait() blockiert (hier: spin-loop, im echten Kernel yield/Wait-Queue)
//! - Prozesse haben Prioritaeten (0-255, ATC-0008)

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::capability::{CapabilityTable, CapId, Pid, ResourceType, Rights};

/// Prozess-Typen laut ATS-1000
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessType {
    Agent,
    Service,
    Contract,
    System,
    Validator,
}

/// Prozess-Zustand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated(ExitCode),
}

pub type ExitCode = i32;

/// Prozess-Kontrollblock (PCB)
#[derive(Debug, Clone)]
pub struct ProcessControlBlock {
    pub pid: Pid,
    pub ptype: ProcessType,
    pub priority: u8,
    pub state: ProcessState,
    pub parent: Option<Pid>,
    pub children: Vec<Pid>,
}

/// Der Prozess-Manager — verfuegt ueber Capability-Tabelle und Prozess-Map
pub struct ProcessManager {
    processes: BTreeMap<Pid, ProcessControlBlock>,
    next_pid: AtomicU32,
    /// Referenz zur Capability-Tabelle (Kernel-intern)
    pub caps: CapabilityTable,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            next_pid: AtomicU32::new(1),
            caps: CapabilityTable::new(),
        }
    }

    /// Erzeugt einen neuen Prozess und gibt ihm automatisch eine
    /// Memory-Capability fuer seinen eigenen Adressraum.
    pub fn spawn(&mut self, ptype: ProcessType, priority: u8) -> Pid {
        let pid = Pid(self.next_pid.fetch_add(1, Ordering::SeqCst));
        let pcb = ProcessControlBlock {
            pid,
            ptype,
            priority,
            state: ProcessState::Ready,
            parent: None,
            children: Vec::new(),
        };
        self.processes.insert(pid, pcb);

        // Automatische Memory-Cap fuer den eigenen Adressraum
        let addr_space = pid.0 as u64; // Eindeutige Resource-ID pro Prozess
        self.caps.create(pid, ResourceType::Memory, addr_space, Rights::READ | Rights::WRITE | Rights::EXEC | Rights::DELEGATE);

        pid
    }

    /// Erzeugt einen Kind-Prozess unter einem Elterprozess
    pub fn spawn_child(&mut self, parent: Pid, ptype: ProcessType, priority: u8) -> Option<Pid> {
        if !self.processes.contains_key(&parent) { return None; }
        let child_pid = self.spawn(ptype, priority);

        // Kind-Prozess mit Parent verknuepfen
        if let Some(child) = self.processes.get_mut(&child_pid) {
            child.parent = Some(parent);
        }
        if let Some(parent_pcb) = self.processes.get_mut(&parent) {
            parent_pcb.children.push(child_pid);
        }
        Some(child_pid)
    }

    /// Beendet einen Prozess und widerruft ALLE seine Capabilities.
    /// Atomar: Cap-Widerruf + Zustandsaenderung in einem Schritt.
    pub fn kill(&mut self, pid: Pid, exit_code: ExitCode) -> bool {
        let pcb = match self.processes.get_mut(&pid) {
            Some(p) => p,
            None => return false,
        };

        if pcb.state == ProcessState::Terminated(0) || matches!(pcb.state, ProcessState::Terminated(_)) {
            return false; // Bereits terminiert
        }

        // Alle Capabilities dieses Prozesses widerrufen
        let cap_ids: Vec<CapId> = self.caps.list_for(pid).iter().map(|c| c.id).collect();
        for cap_id in cap_ids {
            self.caps.revoke(cap_id);
        }

        // Zustand auf Terminiert setzen
        pcb.state = ProcessState::Terminated(exit_code);

        // Aus Eltern-Kind-Liste entfernen
        if let Some(parent_pid) = pcb.parent {
            if let Some(parent) = self.processes.get_mut(&parent_pid) {
                parent.children.retain(|&c| c != pid);
            }
        }

        true
    }

    /// Wartet auf die Terminierung eines Prozesses.
    /// Gibt den Exit-Code zurueck, oder None wenn der Prozess nicht existiert.
    /// Im echten Kernel: Blockiert den aufrufenden Thread. Hier: sofortige Rueckgabe.
    pub fn wait(&self, pid: Pid) -> Option<ExitCode> {
        match self.processes.get(&pid)?.state {
            ProcessState::Terminated(code) => Some(code),
            _ => None, // Noch nicht terminiert (echter Kernel: hier blockieren)
        }
    }

    /// Listet alle Prozesse auf
    pub fn list_processes(&self) -> Vec<&ProcessControlBlock> {
        self.processes.values().collect()
    }

    /// Holt einen Prozess per PID
    pub fn get(&self, pid: Pid) -> Option<&ProcessControlBlock> {
        self.processes.get(&pid)
    }

    /// Setzt einen Ready-Prozess auf Running (Scheduler-Aufruf)
    pub fn set_running(&mut self, pid: Pid) -> bool {
        match self.processes.get_mut(&pid) {
            Some(p) if p.state == ProcessState::Ready => {
                p.state = ProcessState::Running;
                true
            }
            _ => false,
        }
    }

    /// Setzt einen laufenden Prozess zurueck auf Ready (Preemption)
    pub fn set_ready(&mut self, pid: Pid) -> bool {
        match self.processes.get_mut(&pid) {
            Some(p) if p.state == ProcessState::Running => {
                p.state = ProcessState::Ready;
                true
            }
            _ => false,
        }
    }

    /// Blockiert einen Prozess (wartet auf IPC/IO)
    pub fn set_blocked(&mut self, pid: Pid) -> bool {
        match self.processes.get_mut(&pid) {
            Some(p) if p.state == ProcessState::Running || p.state == ProcessState::Ready => {
                p.state = ProcessState::Blocked;
                true
            }
            _ => false,
        }
    }

    /// Weckt einen blockierten Prozess auf
    pub fn unblock(&mut self, pid: Pid) -> bool {
        match self.processes.get_mut(&pid) {
            Some(p) if p.state == ProcessState::Blocked => {
                p.state = ProcessState::Ready;
                true
            }
            _ => false,
        }
    }

    /// Anzahl aktiver (nicht-terminierter) Prozesse
    pub fn active_count(&self) -> usize {
        self.processes.values()
            .filter(|p| !matches!(p.state, ProcessState::Terminated(_)))
            .count()
    }

    /// Capability-Check: Hat der Prozess die geforderten Rechte?
    pub fn check_capability(&self, pid: Pid, resource_type: ResourceType, resource_id: u64, required: Rights) -> bool {
        self.caps.check(pid, resource_type, resource_id, required)
    }

    /// Delegiert eine Capability an einen anderen Prozess
    pub fn delegate_capability(&mut self, source: Pid, cap_id: CapId, target: Pid, rights: Rights) -> Result<CapId, crate::capability::CapabilityError> {
        self.caps.delegate(source, cap_id, target, rights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_creates_process_and_memory_cap() {
        let mut pm = ProcessManager::new();
        let pid1 = pm.spawn(ProcessType::Agent, 128);
        assert!(pid1.0 > 0);

        // Prozess ist Ready
        let pcb = pm.get(pid1).unwrap();
        assert_eq!(pcb.state, ProcessState::Ready);
        assert_eq!(pcb.ptype, ProcessType::Agent);

        // Hat automatisch Memory-Cap fuer eigenen Adressraum
        assert!(pm.check_capability(pid1, ResourceType::Memory, pid1.0 as u64, Rights::READ));
        assert!(pm.check_capability(pid1, ResourceType::Memory, pid1.0 as u64, Rights::WRITE));
    }

    #[test]
    fn test_spawn_multiple_processes() {
        let mut pm = ProcessManager::new();
        let p1 = pm.spawn(ProcessType::Agent, 100);
        let p2 = pm.spawn(ProcessType::Service, 200);
        let p3 = pm.spawn(ProcessType::Validator, 255);
        assert_ne!(p1, p2);
        assert_ne!(p2, p3);
        assert_eq!(pm.list_processes().len(), 3);
    }

    #[test]
    fn test_kill_revokes_all_capabilities() {
        let mut pm = ProcessManager::new();
        let p1 = pm.spawn(ProcessType::Agent, 128);
        let p2 = pm.spawn(ProcessType::Service, 100);

        // p1 delegiert READ an p2
        let p1_mem_cap = pm.caps.list_for(p1)[0].id;
        pm.delegate_capability(p1, p1_mem_cap, p2, Rights::READ | Rights::DELEGATE).unwrap();
        assert!(pm.check_capability(p2, ResourceType::Memory, p1.0 as u64, Rights::READ));

        // Kill p1 -> alle Caps weg (auch p2s delegierte)
        assert!(pm.kill(p1, 0));
        assert!(!pm.check_capability(p1, ResourceType::Memory, p1.0 as u64, Rights::READ));
        // p2s delegierte Cap von p1 sollte auch weg sein (kaskadierender Widerruf)
        assert!(!pm.check_capability(p2, ResourceType::Memory, p1.0 as u64, Rights::READ));
        // p2s eigene Cap bleibt
        assert!(pm.check_capability(p2, ResourceType::Memory, p2.0 as u64, Rights::READ));

        // Exit-Code abfragbar
        assert_eq!(pm.wait(p1), Some(0));
    }

    #[test]
    fn test_kill_nonexistent_process() {
        let mut pm = ProcessManager::new();
        assert!(!pm.kill(Pid(999), 1));
    }

    #[test]
    fn test_double_kill_rejected() {
        let mut pm = ProcessManager::new();
        let p1 = pm.spawn(ProcessType::Agent, 100);
        assert!(pm.kill(p1, 0));
        assert!(!pm.kill(p1, 1)); // Bereits terminiert
    }

    #[test]
    fn test_spawn_child_linking() {
        let mut pm = ProcessManager::new();
        let parent = pm.spawn(ProcessType::System, 255);
        let child = pm.spawn_child(parent, ProcessType::Agent, 100).unwrap();

        let child_pcb = pm.get(child).unwrap();
        assert_eq!(child_pcb.parent, Some(parent));

        let parent_pcb = pm.get(parent).unwrap();
        assert!(parent_pcb.children.contains(&child));
    }

    #[test]
    fn test_kill_removes_from_parent_children() {
        let mut pm = ProcessManager::new();
        let parent = pm.spawn(ProcessType::System, 255);
        let child = pm.spawn_child(parent, ProcessType::Agent, 100).unwrap();

        assert!(pm.kill(child, 42));
        let parent_pcb = pm.get(parent).unwrap();
        assert!(!parent_pcb.children.contains(&child));
    }

    #[test]
    fn test_state_transitions() {
        let mut pm = ProcessManager::new();
        let p1 = pm.spawn(ProcessType::Agent, 100);

        // Ready -> Running
        assert!(pm.set_running(p1));
        assert_eq!(pm.get(p1).unwrap().state, ProcessState::Running);

        // Running -> Blocked
        assert!(pm.set_blocked(p1));
        assert_eq!(pm.get(p1).unwrap().state, ProcessState::Blocked);

        // Blocked -> Ready
        assert!(pm.unblock(p1));
        assert_eq!(pm.get(p1).unwrap().state, ProcessState::Ready);

        // Ready -> Running -> Ready (Preemption)
        assert!(pm.set_running(p1));
        assert!(pm.set_ready(p1));
        assert_eq!(pm.get(p1).unwrap().state, ProcessState::Ready);
    }

    #[test]
    fn test_active_count() {
        let mut pm = ProcessManager::new();
        let p1 = pm.spawn(ProcessType::Agent, 100);
        let p2 = pm.spawn(ProcessType::Service, 200);
        assert_eq!(pm.active_count(), 2);

        pm.kill(p1, 0);
        assert_eq!(pm.active_count(), 1);

        pm.kill(p2, 0);
        assert_eq!(pm.active_count(), 0);
    }

    #[test]
    fn test_priority_preserved() {
        let mut pm = ProcessManager::new();
        let p1 = pm.spawn(ProcessType::System, 255);
        let p2 = pm.spawn(ProcessType::Agent, 0);
        let p3 = pm.spawn(ProcessType::Validator, 128);

        assert_eq!(pm.get(p1).unwrap().priority, 255);
        assert_eq!(pm.get(p2).unwrap().priority, 0);
        assert_eq!(pm.get(p3).unwrap().priority, 128);
    }
}

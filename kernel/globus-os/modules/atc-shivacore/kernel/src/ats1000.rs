// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ATS-1000 — ShivaCore Interface (siehe atc-kernel/docs/ATS_STANDARDS.md)
//
// Dieses Modul bildet die im Standard definierte KERNEL_API 1:1 als Rust-
// Traits/Stubs ab. In K-Sprint 0 sind die Funktionen noch nicht implementiert
// — sie werden Schritt für Schritt in K-Sprint 1-7 gefüllt:
//   K2 Speicher (alloc/free/mmap)   K3 Prozesse (spawn/kill/wait)
//   K5 Dateisystem (open/read/write/close)   K7 Netzwerk (connect/send/recv)
//
// Hinweis: Nutzt noch KEIN `alloc` (kein Heap bis K-Sprint 2) — bewusst nur
// feste Buffer/Slices, damit dieses Modul schon in K-Sprint 0 kompiliert.
//
// Ziel: Der Standard ist die Spezifikation, dieser Code ist die
// Referenzimplementierung — nicht umgekehrt.

#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pid(pub u32);
pub type Address = [u8; 37]; // ATC-Adresse, siehe ATC-0002
pub type ExitCode = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessType {
    Agent,
    Service,
    Contract,
    System,
    Validator,
}

#[derive(Debug, Clone, Copy)]
pub struct MemRegion {
    pub addr: u64,
    pub size: u64,
    pub pid: Pid,
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessInfo {
    pub pid: Pid,
    pub ptype: ProcessType,
    pub priority: u8, // 0=niedrig, 255=System — ATC-0008
}

/// ATS-1000 KERNEL_API — Prozessverwaltung
/// Status: Stub. Wird in K-Sprint 3 (Multitasking) implementiert.
pub trait ProcessManager {
    fn spawn(&mut self, ptype: ProcessType, priority: u8) -> Pid;
    fn kill(&mut self, pid: Pid) -> bool;
    fn wait(&mut self, pid: Pid) -> ExitCode;
    fn list_processes(&self) -> &[ProcessInfo];
}

/// ATS-1000 KERNEL_API — Speicher
/// Status: Stub. Wird in K-Sprint 2 (Paging, Heap-Allocator) implementiert.
pub trait MemoryManager {
    fn alloc(&mut self, size: u64, pid: Pid) -> Option<MemRegion>;
    fn free(&mut self, region: MemRegion) -> bool;
    fn mmap(&mut self, addr: u64, size: u64) -> Option<MemRegion>;
}

/// ATS-1000 KERNEL_API — Dateisystem (siehe ATS-1002/ATCFS)
/// Status: Stub. Wird in K-Sprint 5 implementiert.
pub trait FileSystem {
    fn open(&mut self, path: &str, mode: u8) -> Option<u64>;
    fn read(&mut self, fh: u64, buf: &mut [u8]) -> u64;
    fn write(&mut self, fh: u64, data: &[u8]) -> u64;
    fn close(&mut self, fh: u64) -> bool;
}

/// ATS-1000 KERNEL_API — Netzwerk (siehe ATS-1004/ATCNet, ATC-0007)
/// Status: Stub. Wird in K-Sprint 7 implementiert.
pub trait NetworkStack {
    fn connect(&mut self, peer_node_id: &[u8; 32]) -> Option<u64>;
    fn send(&mut self, conn: u64, msg: &[u8]) -> bool;
    fn recv(&mut self, conn: u64, buf: &mut [u8]) -> u64;
}

/// Kernel-Garantien laut ATS-1000 — werden hier als Boot-Log dokumentiert,
/// sobald die jeweilige Eigenschaft technisch durchgesetzt wird.
pub const KERNEL_GUARANTEES: &[&str] = &[
    "Kein Single Point of Failure (dezentral) — ab K-Sprint 8 (P2P)",
    "Jeder Prozess laeuft isoliert in eigenem MemRegion — ab K-Sprint 6 (Userspace)",
    "Alle System-Calls sind auditierbar (auf-Chain) — ab K-Sprint 8",
    "Gas-basierte Ressourcen-Abrechnung — ab K-Sprint 6 (Syscalls)",
];

//! KAI-OS Health — Watchdog + Health-Gossip (S19-S20, Issue #108).
//!
//! Kernprinzip (KAI-CORE-RUNTIME-001 §9):
//! **Liveness wird NICHT angenommen, sondern beobachtet.**
//! Fail-closed: ein Service gilt nur als lebendig, solange seine
//! Herzschläge innerhalb der Toleranz eintreffen. Alle Zeit ist
//! Tick-basiert — deterministisch, keine Wall-Clock (REQ-ENG-002).

pub mod gossip;
pub mod watchdog;

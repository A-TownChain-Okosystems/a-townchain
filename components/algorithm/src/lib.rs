// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC Algorithm — migrierter R1-Konsens-Prototyp; Capability-Ownership pending SCR-0131.
//! Specs: ATC-CONSENSUS-301..307 (DRAFT, SCR-0071) · Evidence: .atc/evidence/
//! Ehrlichkeit: MVP-Hash FNV-1a ist NICHT kryptografisch. EIGENSTAENDIGER
//! Algorithmus ATC-HASH-001 (SCR-0119, Owner-Direktive 12.09.2026) ersetzt
//! die SHA-256-Richtung; Adoption im Devnet-Pfad als eigene Welle, danach
//! keine FNV-Platzhalter mehr. Kein Mainnet-Claim ohne externe Krypto-Pruefung.
//!
//! Monetary-policy code is colocated in this migrated component. Its normative
//! authority and capability ownership remain subject to approved governance.

pub mod economics;
pub mod hash;
pub mod poh;
pub mod selection;
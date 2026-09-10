// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC Consensus Algorithm — kanonische Konsens-Implementierung (MVP-Start).
//! Specs: ATC-CONSENSUS-301..307 (DRAFT, SCR-0071) · Evidence: .atc/evidence/
//! Ehrlichkeit: MVP-Hash FNV-1a ist NICHT kryptografisch; kanonischer PoH
//! erfordert SHA-256 nach Spec-Freeze (F-067). Kein Mainnet-Claim.

pub mod poh;
pub mod selection;

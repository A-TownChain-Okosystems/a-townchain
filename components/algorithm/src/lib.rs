// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC Consensus Algorithm — kanonische Konsens-Implementierung (MVP-Start).
//! Specs: ATC-CONSENSUS-301..307 (DRAFT, SCR-0071) · Evidence: .atc/evidence/
//! Status: PoH/selection use SHA-256 as a deterministic MVP primitive.
//! ATC-HASH-001 remains a separate, non-cryptographically-reviewed devnet
//! hash and is not a mainnet/security primitive. Consensus remains DRAFT.

pub mod hash;
pub mod poh;
pub mod selection;

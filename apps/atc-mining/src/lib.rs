// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC Mining — execution, deterministic hashing and bounded reward primitives.
//! Evidence: `.atc/evidence/`.
//!
//! Mining never defines consensus. Canonical consensus rules remain in
//! `atc-algorithm` and contract settlement remains in `atc-contracts`.

pub mod executor;
pub mod miner;
pub mod reward;

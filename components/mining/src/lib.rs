// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC Mining — execution, deterministic hashing and bounded reward primitives.
//! Evidence: `.atc/evidence/`.
//!
//! Mining never defines consensus. The migrated algorithm implementation is
//! consumed from `components/algorithm`; capability ownership and normative
//! protocol authority remain subject to SCR-0131 approval. Contract settlement
//! remains in `atc-contracts`.

pub mod algorithms;
pub mod executor;
pub mod manager;
pub mod miner;
pub mod network;
pub mod reward;
pub mod telemetry;

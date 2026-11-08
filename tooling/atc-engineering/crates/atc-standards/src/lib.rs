//! atc-standards — Adapter für die kanonische Standards-Registry (Phase-1-Subset).
//!
//! Spezifikation: ATC-ENGINEERING-SPEC-001 §5 (atc-standards), §24 (Standards-Anbindung),
//! §25 (Version Pinning). atc-standards bleibt SSOT — dieses Crate LIEST nur.
//!
//! Fail-closed: fehlende Standards, ungültige Lifecycle-Status, doppelte IDs und
//! nicht normative/freigegebene Standards bei Anforderung sind Fehler — BLOCK.

pub mod flow;
pub mod registry;
pub mod repositories;

pub use flow::parse_flow_map;
pub use registry::{StandardEntry, StandardsRegistry, Status};
pub use repositories::{RepoEntry, RepoRegistry};

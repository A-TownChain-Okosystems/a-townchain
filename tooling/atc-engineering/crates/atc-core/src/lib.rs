//! atc-core — Fundamentale Domain Types der ATC Engineering Control Plane.
//!
//! Spezifikation: ATC-ENGINEERING-SPEC-001 §5 (atc-core).
//! Prinzipien: fail-closed (kein Panicking, kein unwrap in Bibliothekspfaden),
//! deterministisch, abhängigkeitsfrei.

pub mod error;
pub mod ids;
pub mod state;
pub mod timestamp;
pub mod version;

pub use error::{CoreError, Result};
pub use ids::{EvidenceId, RepositoryId, StandardId};
pub use state::DerivedState;
pub use timestamp::Timestamp;
pub use version::{is_semver, semver_key};

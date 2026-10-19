//! Verification Engine public API.
//!
//! This module re-exports the registry-backed verification boundary. It does not
//! ship a concrete SNARK/STARK backend; production verification remains blocked
//! until an audited backend is implemented and its cryptographic tests pass.
pub use crate::verification_engine::{
    BackendError, CircuitSpec, ProofBackend, VerificationEngine, VerificationError,
    VerificationRequest,
};

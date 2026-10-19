//! Fail-closed ZKP verification engine interface.
//!
//! This module defines the boundary between circuit registration and a concrete,
//! audited proof-system backend. It does NOT implement SNARK/STARK cryptography.
//! Production callers must provide a reviewed backend and must not treat a mock
//! backend or a successful interface test as cryptographic verification.
use core::marker::PhantomData;

/// Metadata registered for one circuit and one compatible proof-system version.
#[derive(Clone, Copy, Debug)]
pub struct CircuitSpec<'a> {
    /// Stable, canonical circuit identifier.
    pub circuit_id: &'a str,
    /// Stable proof-system identifier, e.g. an audited backend's registry ID.
    pub proof_system_id: &'a str,
    /// Version of the proof-system wire/interface contract.
    pub proof_system_version: u16,
    /// Verification key bytes parsed/validated by the backend.
    pub verification_key: &'a [u8],
    /// Maximum accepted proof size for this circuit.
    pub max_proof_bytes: usize,
    /// Maximum accepted public-input size for this circuit.
    pub max_public_input_bytes: usize,
}

/// One verification request. Inputs are borrowed and are never copied by this API.
#[derive(Clone, Copy, Debug)]
pub struct VerificationRequest<'a> {
    pub circuit_id: &'a str,
    pub public_inputs: &'a [u8],
    pub proof: &'a [u8],
}

/// Errors surfaced by the verification boundary. No error contains secret data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationError {
    EmptyCircuitId,
    EmptyProof,
    CircuitNotRegistered,
    DuplicateCircuitRegistration,
    EmptyVerificationKey,
    ProofTooLarge,
    PublicInputsTooLarge,
    ProofSystemMismatch,
    UnsupportedProofSystemVersion,
    InvalidProof,
    BackendFailure,
}

/// Result reported by a concrete backend. Backends must distinguish a cryptographically
/// invalid proof from operational or configuration failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendError {
    InvalidProof,
    UnsupportedVersion,
    InternalFailure,
}

/// Implement this trait only for a concrete, independently reviewed proof verifier.
///
/// Implementations must validate the proof cryptographically against the supplied
/// verification key and public inputs. Returning Ok without performing that check
/// violates this trait's contract. Timeouts/resource limits must be enforced by the
/// integrating runtime; this no_std interface does not itself provide a clock.
pub trait ProofBackend {
    fn proof_system_id(&self) -> &'static str;
    fn proof_system_version(&self) -> u16;

    fn verify(
        &self,
        circuit: &CircuitSpec<'_>,
        public_inputs: &[u8],
        proof: &[u8],
    ) -> Result<(), BackendError>;
}

/// Registry-backed verification boundary. The registry is immutable for the lifetime
/// of the engine, preventing request-time circuit substitution.
pub struct VerificationEngine<'a, B: ProofBackend> {
    circuits: &'a [CircuitSpec<'a>],
    backend: &'a B,
    _marker: PhantomData<&'a B>,
}

impl<'a, B: ProofBackend> VerificationEngine<'a, B> {
    pub const fn new(circuits: &'a [CircuitSpec<'a>], backend: &'a B) -> Self {
        Self {
            circuits,
            backend,
            _marker: PhantomData,
        }
    }

    /// Verify a proof only when exactly one registered circuit matches and all
    /// registry/backend/version/size checks pass. Any mismatch fails closed.
    pub fn verify(&self, request: VerificationRequest<'_>) -> Result<(), VerificationError> {
        if request.circuit_id.is_empty() {
            return Err(VerificationError::EmptyCircuitId);
        }
        if request.proof.is_empty() {
            return Err(VerificationError::EmptyProof);
        }

        let mut matching: Option<&CircuitSpec<'_>> = None;
        for circuit in self.circuits {
            if circuit.circuit_id == request.circuit_id {
                if matching.is_some() {
                    return Err(VerificationError::DuplicateCircuitRegistration);
                }
                matching = Some(circuit);
            }
        }
        let circuit = matching.ok_or(VerificationError::CircuitNotRegistered)?;

        if circuit.verification_key.is_empty() {
            return Err(VerificationError::EmptyVerificationKey);
        }
        if request.proof.len() > circuit.max_proof_bytes {
            return Err(VerificationError::ProofTooLarge);
        }
        if request.public_inputs.len() > circuit.max_public_input_bytes {
            return Err(VerificationError::PublicInputsTooLarge);
        }
        if circuit.proof_system_id != self.backend.proof_system_id() {
            return Err(VerificationError::ProofSystemMismatch);
        }
        if circuit.proof_system_version != self.backend.proof_system_version() {
            return Err(VerificationError::UnsupportedProofSystemVersion);
        }

        match self.backend.verify(circuit, request.public_inputs, request.proof) {
            Ok(()) => Ok(()),
            Err(BackendError::InvalidProof) => Err(VerificationError::InvalidProof),
            Err(BackendError::UnsupportedVersion) => {
                Err(VerificationError::UnsupportedProofSystemVersion)
            }
            Err(BackendError::InternalFailure) => Err(VerificationError::BackendFailure),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBackend {
        accepts: bool,
        system_id: &'static str,
        version: u16,
    }

    impl ProofBackend for TestBackend {
        fn proof_system_id(&self) -> &'static str {
            self.system_id
        }

        fn proof_system_version(&self) -> u16 {
            self.version
        }

        fn verify(
            &self,
            _circuit: &CircuitSpec<'_>,
            _public_inputs: &[u8],
            _proof: &[u8],
        ) -> Result<(), BackendError> {
            if self.accepts {
                Ok(())
            } else {
                Err(BackendError::InvalidProof)
            }
        }
    }

    const CIRCUIT: CircuitSpec<'static> = CircuitSpec {
        circuit_id: "atc.test.circuit.v1",
        proof_system_id: "test-only",
        proof_system_version: 1,
        verification_key: b"test-key",
        max_proof_bytes: 16,
        max_public_input_bytes: 8,
    };

    fn request<'a>(proof: &'a [u8]) -> VerificationRequest<'a> {
        VerificationRequest {
            circuit_id: CIRCUIT.circuit_id,
            public_inputs: b"public",
            proof,
        }
    }

    #[test]
    fn accepts_only_backend_approved_proof() {
        let backend = TestBackend { accepts: true, system_id: "test-only", version: 1 };
        let engine = VerificationEngine::new(&[CIRCUIT], &backend);
        assert_eq!(engine.verify(request(b"proof")), Ok(()));
    }

    #[test]
    fn rejects_invalid_proof() {
        let backend = TestBackend { accepts: false, system_id: "test-only", version: 1 };
        let engine = VerificationEngine::new(&[CIRCUIT], &backend);
        assert_eq!(engine.verify(request(b"proof")), Err(VerificationError::InvalidProof));
    }

    #[test]
    fn rejects_unregistered_circuit() {
        let backend = TestBackend { accepts: true, system_id: "test-only", version: 1 };
        let engine = VerificationEngine::new(&[CIRCUIT], &backend);
        let req = VerificationRequest { circuit_id: "unknown", public_inputs: b"x", proof: b"p" };
        assert_eq!(engine.verify(req), Err(VerificationError::CircuitNotRegistered));
    }

    #[test]
    fn rejects_empty_proof_before_backend_call() {
        let backend = TestBackend { accepts: true, system_id: "test-only", version: 1 };
        let engine = VerificationEngine::new(&[CIRCUIT], &backend);
        assert_eq!(engine.verify(request(b"")), Err(VerificationError::EmptyProof));
    }

    #[test]
    fn rejects_oversized_proof() {
        let backend = TestBackend { accepts: true, system_id: "test-only", version: 1 };
        let engine = VerificationEngine::new(&[CIRCUIT], &backend);
        assert_eq!(engine.verify(request(b"0123456789abcdefg")), Err(VerificationError::ProofTooLarge));
    }

    #[test]
    fn rejects_backend_version_mismatch() {
        let backend = TestBackend { accepts: true, system_id: "test-only", version: 2 };
        let engine = VerificationEngine::new(&[CIRCUIT], &backend);
        assert_eq!(engine.verify(request(b"proof")), Err(VerificationError::UnsupportedProofSystemVersion));
    }

    #[test]
    fn rejects_duplicate_circuit_registration() {
        let backend = TestBackend { accepts: true, system_id: "test-only", version: 1 };
        let engine = VerificationEngine::new(&[CIRCUIT, CIRCUIT], &backend);
        assert_eq!(engine.verify(request(b"proof")), Err(VerificationError::DuplicateCircuitRegistration));
    }
}

---
protocol:
  id: ATC-PROTO-AI-001
  name: "ATC AI Protocol"
  version: 1.0.0
  status: draft
  domain: AI
  chain_id: 658467
---

# ATC-PROTO-AI-001 — AI Protocol v1.0.0

## §1 Scope
Canonical contract for model identity, deployment, inference, AI capabilities, neural context, LLM routing, resource accounting and deterministic AI-runtime boundaries.

## §2 Existing-first evidence
Gate 0 found an existing AI implementation in canonical GlobusOS/ShivaCore:
- Repository: `A-TownChain-Okosystems/globus-os`
- Source SHA: `0bb6dc06ce0a270ea2110bfbe7cfa468d93e2847`
- Path: `modules/atc-shivacore/kernel/src/ai.rs`
- Existing components: Tensor/Layer/Model, ModelRegistry, AiCapability/AiCapabilityGuard, NeuralContextStore, LlmRouter and AiEngine.
The implementation is therefore reused as the existing baseline; this protocol does not create a duplicate AI runtime.

## §3 Canonical identity and model ABI
Every model MUST have a deterministic model identifier, ABI/version, input/output schema, parameter/weight encoding, owner identity and capability policy. Model metadata MUST be canonically encoded before it can affect consensus, audit or cross-node state.

## §4 Deterministic inference boundary
Consensus-critical AI decisions MUST NOT depend on platform-dependent floating-point behavior, nondeterministic kernels, unordered iteration, wall-clock state, external model endpoints or unspecified hardware behavior. Such computation MUST use a frozen deterministic representation/quantization contract or remain explicitly outside consensus.

## §5 Deployment and capabilities
Deployment, inference, training, query, deletion and neural-context operations MUST require explicit capabilities. Capability grants/revocations MUST be idempotent and deterministically represented. No ambient AI authority is permitted.

## §6 Resource and gas accounting
Inference and other AI operations MUST have deterministic resource limits and accounting. Caller-supplied budgets, model cost and actual resource consumption MUST have a defined relation; unused/insufficient budget behavior MUST be explicit.

## §7 Neural context
Context entries MUST define canonical key, embedding encoding, metadata encoding, owner, version/timestamp semantics and retention policy. Similarity search MUST specify deterministic distance arithmetic, tie-breaking and top-K ordering.

## §8 LLM routing
LLM requests MUST bind caller, model, protocol version, limits and policy. External/non-deterministic generation MUST be treated as an explicitly non-consensus operation unless a deterministic execution contract is activated.

## §9 Security
Model loading, execution, context access and tool integration MUST be capability-gated. Cryptographic identities and audit records MUST use approved ATC cryptographic contracts. Consensus/security paths MUST NOT rely on an unspecified or legacy non-cryptographic hash.

## §10 Failure semantics
Unknown model, malformed input, shape/ABI mismatch, missing capability, exhausted budget, invalid model ABI and unavailable runtime MUST produce deterministic errors. Silent model fallback is prohibited for consensus-critical execution.

## §11 Conformance vectors
Positive vectors MUST cover model registration, deterministic ABI encoding, capability authorization, inference accounting, context storage/retrieval and deterministic top-K ordering. Negative vectors MUST cover unauthorized operations, malformed model metadata, ABI mismatch, resource exhaustion, nondeterministic ordering and invalid model/domain binding. Cross-language/runtime vectors are required before activation.

## §12 Existing implementation gaps
The existing source is implementation evidence, not protocol conformance. In particular, the current source uses `f64` tensor arithmetic, a simple hash in the LLM router, mutable in-memory registries/context and incomplete canonical wire/ABI definitions. These MUST NOT be promoted to consensus-critical protocol behavior without the contracts in this specification.

## §13 Activation blockers
Status remains draft until the canonical AI Model ABI, deterministic numeric representation, model identity/hash contract, capability wire format, gas/resource formula, context encoding and exact-SHA conformance suite are frozen and verified.

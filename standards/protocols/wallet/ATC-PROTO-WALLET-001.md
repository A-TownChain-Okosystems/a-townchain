---
protocol:
  id: ATC-PROTO-WALLET-001
  name: "ATC Wallet Protocol"
  version: 1.0.0
  status: draft
  domain: WALLET
  layer: L3/L5
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-WALLET-001 — Wallet Protocol v1.0.0

## §1 Scope
Defines deterministic wallet key custody, account derivation, transaction construction/signing, local validation, nonce handling and broadcast preparation. Transaction wire/signing semantics remain owned by ATC-PROTO-TX-001.

## §2 Existing-first evidence
Gate 0 searched the canonical wallet and SDK repositories for transaction signing, account, nonce and secp256k1 implementation traces. No exact canonical source result was returned by repository code search in this pass. Existing CI history may contain wallet implementation evidence, but this specification makes no conformance claim without exact source-SHA evidence.

## §3 Key custody
Wallets MUST keep private key material out of logs, telemetry and serialized public state. Secret material MUST NOT be transmitted to remote services as part of ordinary signing.

## §4 Account identity
Account derivation MUST use a versioned canonical algorithm. The resulting account identifier MUST be deterministic from the canonical public-key representation and network/chain context where applicable.

## §5 Transaction construction
Wallets MUST construct transactions using the exact ATC-PROTO-TX-001 canonical fields and encoding. Economic amounts MUST remain u128 end-to-end. Nonce MUST be u64. The wallet MUST NOT silently reinterpret numeric values through floating-point types.

## §6 Signing
Transaction signing MUST use the activated TX contract: secp256k1/ECDSA, RFC6979 deterministic nonce generation, low-S normalization, SHA-256 and `ATC-TX-DOMAIN-V2`. The wallet MUST sign the exact canonical signing bytes and MUST reject ambiguous or non-canonical input.

## §7 Nonce management
A wallet MUST distinguish confirmed, locally reserved and pending nonces. Concurrent signing MUST NOT produce accidental duplicate nonces for the same account. Remote chain state remains authoritative when reconciling conflicts.

## §8 Local validation
Before signing, the wallet SHOULD validate chain ID, field widths, amount bounds, nonce, recipient/address encoding, signature-domain version and transaction-size/resource limits. Validation order SHOULD be deterministic.

## §9 Broadcast
Broadcast MUST submit canonical wire bytes or a canonical transaction representation. Wallets MUST NOT mutate signed bytes between signing and broadcast.

## §10 Recovery and backup
Backup/restore MUST preserve the exact key material and version metadata required for deterministic account derivation. Restored wallets MUST revalidate chain context before signing.

## §11 Security
Wallets MUST support explicit lock/unlock boundaries, memory hygiene appropriate to the runtime, and refusal to sign when key state or transaction encoding is ambiguous.

## §12 Conformance
Positive vectors MUST cover account derivation, u128 boundary amounts, nonce sequencing, deterministic signing and canonical serialization. Negative vectors MUST cover wrong chain, u128 overflow, malformed recipient, invalid nonce, non-canonical signature, legacy signing domain, mutated signed bytes and duplicate nonce reservation. Cross-language results MUST match the TX golden vectors.

## §13 Activation
Status remains draft until exact wallet source-SHA evidence, TX-vector conformance, key-custody tests and exact-SHA CI evidence are available.
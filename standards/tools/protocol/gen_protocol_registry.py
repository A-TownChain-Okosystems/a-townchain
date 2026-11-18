#!/usr/bin/env python3
"""Generiert registry/protocol-registry.yaml — die ATC-Protocol-Registry (ATC-STD-PROTOCOL-001 §20/§21).
SSOT der ATC-PROTO-*-Familien; Regenerierung nur via SCR (REQ-PROTO-020)."""

import yaml

# (Domain, Name, Priorität, Status, Layer, Notiz) — 26 Familien gem. Owner-Matrix
PROTOCOLS = [
    (
        "P2P",
        "ATC Peer-to-Peer Protocol",
        "P1",
        "draft",
        "L1/L3",
        "v1.0.0 IMPLEMENTIERT (08.09. 01:35, K14-Upgrade p2p_secure.rs, SCR-0028): Envelope 9+1, 6-Phasen-Handshake, Replay-Schutz, Rate-Limiting, v0.9-Kompatibilitaetsmodus; Kernel 423/423 Tests gruen; Status draft bis Testnet-Verifikation/Activation (PROTOCOL-001 Par.19)",
    ),
    (
        "NODE",
        "ATC Node Communication",
        "P1",
        "planned",
        "L1",
        "Node-Discovery/Management auf P2P aufbauend",
    ),
    (
        "CONSENSUS",
        "ATC Consensus Protocol",
        "P1",
        "draft",
        "L1/L3",
        "Impl.-Spuren: ShivaCore K16 (DAG+PoH+Validator+Voting+Finality)",
    ),
    ("BLOCK", "ATC Block Propagation", "P1", "draft", "L3", "Impl.-Spuren: a-townchain Blockchain"),
    (
        "TX",
        "ATC Transaction Protocol",
        "P1",
        "draft",
        "L3",
        "Impl.-Spuren: a-townchain + ShivaCore Tx-Pipeline",
    ),
    (
        "MEMPOOL",
        "ATC Mempool Protocol",
        "P1",
        "draft",
        "L3",
        "Impl.-Spuren: mempool.rs (Referenzimplementierung)",
    ),
    (
        "VALIDATOR",
        "ATC Validator Protocol",
        "P1",
        "draft",
        "L3",
        "Impl.-Spuren: ShivaCore K16 Validator",
    ),
    ("STAKING", "ATC Staking Protocol", "P1", "planned", "L3", ""),
    (
        "MINING",
        "ATC Mining Protocol",
        "P1",
        "draft",
        "L3/L5",
        "Repo atc-mining vorhanden (L5-Dev-Plattform)",
    ),
    (
        "WALLET",
        "ATC Wallet Protocol",
        "P1",
        "draft",
        "L3/L5",
        "Repo atc-wallet vorhanden (L5-Dev-Plattform)",
    ),
    (
        "IDENTITY",
        "ATC Identity Protocol",
        "P1",
        "draft",
        "L1/L3",
        "Impl.-Spuren: ShivaCore K6 (DID+RCT) + K6b (Ed25519)",
    ),
    (
        "REPUTATION",
        "ATC Reputation Protocol",
        "P2",
        "draft",
        "L1/L3",
        "Impl.-Spuren: ShivaCore K15 (Reputation)",
    ),
    ("GOVERNANCE", "ATC Governance Protocol", "P2", "planned", "L3", ""),
    ("ORACLE", "ATC Oracle Protocol", "P1", "planned", "L5", "Repo atc-oracle vorhanden"),
    ("ZKP", "ATC Zero Knowledge Protocol", "P1", "planned", "L5", "Repo atc-zkp vorhanden"),
    ("BRIDGE", "ATC Cross-Chain Bridge", "P1", "planned", "L5", "Repo atc-interop vorhanden"),
    ("IBC", "ATC Interoperability (IBC)", "P1", "planned", "L5", "Repo atc-interop vorhanden"),
    ("DATA", "ATC Data Layer Protocol", "P2", "planned", "L3/L5", ""),
    ("STORAGE", "ATC Decentralized Storage", "P2", "planned", "L5", "Repo atc-storage vorhanden"),
    (
        "AI",
        "ATC AI Agent Protocol",
        "P2",
        "draft",
        "L2",
        "aurora-ai (L2); Kopplung Kernel-Event-Bridge (ATC-M-003)",
    ),
    (
        "AGENT",
        "ATC Agent-to-Agent Protocol",
        "P2",
        "planned",
        "L2",
        "AAS-Kopplung (ATC-AAS-001..025)",
    ),
    ("API", "ATC API Protocol", "P2", "planned", "L5", "Kopplung ATC-STD-202 (API-Konventionen)"),
    (
        "EVENT",
        "ATC Event Protocol",
        "P2",
        "planned",
        "L2/L4",
        "Kopplung Kernel-Event-Bridge (ATC-M-003)",
    ),
    ("AUDIT", "ATC Audit Protocol", "P2", "planned", "L3/L7", "Kopplung AUDIT-001/LogChain"),
    (
        "UPGRADE",
        "ATC Protocol Upgrade",
        "P1",
        "planned",
        "L0-L7",
        "Prozessnorm in ATC-STD-PROTOCOL-001 §19 verankert",
    ),
    (
        "KERNEL",
        "ATC Kernel Interface Protocol",
        "P2",
        "planned",
        "L1",
        "Syscall-Interface ATC-96 (ShivaCore K9)",
    ),
]
# Formale Spezifikationen (SCR-dokumentiert) — Override des Standard-Spezifikations-Texts
SPEC_OVERRIDES = {
    "P2P": "protocols/p2p/ATC-PROTO-P2P-001.md — Spezifikation v1.0.0 APPROVED (SCR-0027) und IMPLEMENTIERT in ShivaCore K14-Upgrade (SCR-0028, p2p_secure.rs, 29 Unit-Tests, Kernel 423/423): Envelope 9+1, 13 Message-Types, 6-Phasen-Handshake, Replay-Schutz, Rate-Limiting, v0.9-Kompatibilitaetsmodus; Status draft bis Testnet-Verifikation/Activation",
    "BLOCK": "protocols/block/ATC-PROTO-BLOCK-001.md — Spezifikation v1.0.0: kanonischer Block-Header, chain_id 658467, parent continuity, tx_root/state_root, u128 total_fees und deterministische Block-/Propagation-Bytes; Status draft bis Exact-SHA-Conformance.",
    "STORAGE": "protocols/storage/ATC-PROTO-STORAGE-001.md — Spezifikation v1.0.0: Namespace/Keys, Canonical Values, Atomic State, Integrity, Snapshots, Recovery, Concurrency und Conformance; Status draft bis Exact-SHA-Conformance.",
    "DATA": "protocols/data/ATC-PROTO-DATA-001.md — Spezifikation v1.0.0: Object Identity, Canonical Encoding/Chunking, Commitments, Availability, Retrieval, Retention, Integrity, Recovery und Conformance; Status draft bis Exact-SHA-Conformance.",
    "IBC": "protocols/ibc/ATC-PROTO-IBC-001.md — Spezifikation v1.0.0: Channel Identity/Handshake, Packets, Commitments, Acknowledgements, Ordering, Timeouts, Capabilities, Recovery und Conformance; Status draft bis Exact-SHA-Conformance.",
    "BRIDGE": "protocols/bridge/ATC-PROTO-BRIDGE-001.md — Spezifikation v1.0.0: Cross-Domain Messages, Proof/Finality, Replay-Schutz, Ordering, Asset Accounting, Relayers, Recovery und Conformance; Status draft bis Exact-SHA-Conformance.",
    "ZKP": "protocols/zkp/ATC-PROTO-ZKP-001.md — Spezifikation v1.0.0: Proof Envelope, Statement Binding, Verification, Replay-Schutz, Crypto-Agility, Resource Limits, Privacy und Conformance; Status draft bis Exact-SHA-Conformance.",
    "ORACLE": "protocols/oracle/ATC-PROTO-ORACLE-001.md — Spezifikation v1.0.0: Source Identity, Observations, Attestation, Freshness, Ordering, Aggregation, Failure/Dispute Handling und Conformance; Status draft bis Exact-SHA-Conformance.",
    "REPUTATION": "protocols/reputation/ATC-PROTO-REPUTATION-001.md — Spezifikation v1.0.0: Identity Binding, Evidence, Deterministic Scoring, Decay, Sybil Resistance, Disputes, Privacy und Conformance; Status draft bis Exact-SHA-Conformance.",
    "WALLET": "protocols/wallet/ATC-PROTO-WALLET-001.md — Spezifikation v1.0.0: Key Custody, Account Derivation, TX Construction/Signing, Nonce Management, Local Validation, Broadcast, Recovery und Conformance; Status draft bis Exact-SHA-Conformance.",
    "MINING": "protocols/mining/ATC-PROTO-MINING-001.md — Spezifikation v1.0.0: PoW-Domain, Proof Validation, Difficulty, Nonce, Rewards, Claims/Replay, Propagation und Conformance; Status draft bis Exact-SHA-Conformance.",
    "IDENTITY": "protocols/identity/ATC-PROTO-IDENTITY-001.md — Spezifikation v1.0.0: kanonische Identity Records, DID/RCT, Key Separation, Authentication, Rotation, Revocation, Recovery und Proof Verification; Status draft bis Exact-SHA-Conformance.",
    "STAKING": "protocols/staking/ATC-PROTO-STAKING-001.md — Spezifikation v1.0.0: Bonding, Unbonding, Delegation, Validator-Backing, Rewards, Slashing, Replay-Schutz und deterministische State-Transitions; Status draft bis Exact-SHA-Conformance.",
    "VALIDATOR": "protocols/validator/ATC-PROTO-VALIDATOR-001.md — Spezifikation v1.0.0: Validator Identity, Eligibility, Set-Derivation, Voting Authority, Block/Vote Validation, Weight, Quorum, Lifecycle, Key Rotation und Equivocation; Status draft bis Exact-SHA-Conformance.",
    "CONSENSUS": "protocols/consensus/ATC-PROTO-CONSENSUS-001.md — Spezifikation v1.0.0: deterministische Proposal-, PoH-, Voting-, DAG-, Fork-Choice-, Finality-, Validator-Set- und Timeout-Regeln; Status draft bis Exact-SHA-Conformance.",
    "MEMPOOL": "protocols/mempool/ATC-PROTO-MEMPOOL-001.md — Spezifikation v1.0.0: deterministische Admission, Duplicate-/Nonce-Schutz, bounded capacity, Ordering, Replacement, Eviction und Block-Selection; Status draft bis Exact-SHA-Conformance.",
    "TX": "protocols/tx/ATC-PROTO-TX-001.md — Spezifikation v1.0.0: kanonische u128-Amounts, chain_id 658467, ATC-TX-DOMAIN-V2, secp256k1/ECDSA RFC6979 low-S, 33-Byte compressed public key, deterministische Wire-/Signing-Bytes; Status draft bis Exact-SHA-Conformance.",
    "AI": "protocols/ai/ATC-PROTO-AI-001.md — Spezifikation v1.0.0; bestehende ShivaCore-AI-Implementierung als Baseline; Aktivierung erst nach ABI/Determinismus/Conformance.",
}

assert len(PROTOCOLS) == 26, f"Erwartet 26 Protokollfamilien, gefunden {len(PROTOCOLS)}"
STATUSES = {"planned", "draft", "active", "experimental", "deprecated"}
assert all(s in STATUSES for _, _, _, s, _, _ in PROTOCOLS)
assert all(p in {"P0", "P1", "P2"} for _, _, p, *_ in PROTOCOLS)

if __name__ == "__main__":
    data = {
        "protocol-registry": {
            "standard": "ATC-STD-PROTOCOL-001",
            "version": "1.0.0",
            "generated": "2026-09-08",
            "note": (
                "ATC Protocol Registry gem. ATC-STD-PROTOCOL-001 §20/§21 — SSOT der ATC-PROTO-*-Familien. "
                "Eintrag = Autorisierung: Ein Protokoll OHNE Registry-Eintrag ist kein ATC-Protokoll. "
                "Status-Ehrlichkeitsregel (REQ-PROTO-021): planned = Baseline ohne Implementierung, "
                "draft = Teilimplementierung existiert (Impl.-Spuren), active/experimental nur mit "
                "verifizierter Implementierung, deprecated = aktiv abgelöst. Änderungen nur via SCR; "
                "Validator S-23 prüft Integrität je CI-Lauf."
            ),
            "domains": [d for d, *_ in PROTOCOLS],
            "protocols": [
                {
                    "id": f"ATC-PROTO-{d}-001",
                    "name": n,
                    "domain": d,
                    "version": "1.0.0",
                    "status": s,
                    "priority": p,
                    "layer": lay,
                    "specification": SPEC_OVERRIDES.get(
                        d,
                        "Zu spezifizieren gem. ATC-STD-PROTOCOL-001 (SCR erforderlich)"
                        if s == "planned"
                        else "Teilweise implementiert; formale Spezifikation gem. ATC-STD-PROTOCOL-001 nachzuziehen",
                    ),
                    "note": note,
                }
                for d, n, p, s, lay, note in PROTOCOLS
            ],
        }
    }
    with open("registry/protocol-registry.yaml", "w", encoding="utf-8") as fh:
        fh.write("# ATC Protocol Registry — ATC-STD-PROTOCOL-001 §20/§21 (SSOT)\n")
        fh.write(
            "# Generiert von tools/protocol/gen_protocol_registry.py — Änderungen nur via SCR.\n"
        )
        yaml.dump(data, fh, allow_unicode=True, sort_keys=False, width=200)

    counts = {}
    for _, _, _, s, *_ in PROTOCOLS:
        counts[s] = counts.get(s, 0) + 1
    print(
        f"protocol-registry.yaml: {len(PROTOCOLS)} Protokollfamilien, Status: {counts}, "
        f"P0-Fundament = Dachstandard ATC-STD-PROTOCOL-001 selbst"
    )

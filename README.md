# atc-zkp

> **ATC ZKP-Layer** — Zero-Knowledge Proof Layer der A-TownChain: Verifikationsschicht zwischen L1 und Anwendungen.

**Prioritaet:** P1 (AD-045) | **Chain-ID:** 658467 (AD-004) | **Org:** [A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems)

> ## Fuer KI-Agenten - Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo [`a-townchain-os-docs`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs):
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md)
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md)
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — insb. AD-045 (dieses Repo), AD-021 (Rust-first), AD-023 (qualitaetsgetrieben)
> 4. Standards: [ATC-STD-ZKP-001…010](https://github.com/A-TownChain-Okosystems/atc-standards) (ZKP-Serie, AD-045)

---

## Architektur (Kurzform — Details: [docs/ZKP_ARCHITECTURE.md](docs/ZKP_ARCHITECTURE.md))

```
dApps | Wallet | GameFi | DeFi | Marketplace | AI
----------------------------------------------------
            ZKP Application / SDK Layer
----------------------------------------------------
    ZKP-Layer  (dieses Repo — KEIN eigenes Netzwerk)
    Proof Generator | Circuit Registry | Verification Engine
    Commitment Manager | Nullifier Manager | Proof Cache
----------------------------------------------------
            ATC Cryptographic Core (SHA-256, AD-001)
----------------------------------------------------
    A-TownChain L1: Consensus | State | Tx | Validators | DA
```

**Wichtigste Designentscheidung (AD-045):** Die ZKP-Layer ist eine kryptografische
Infrastruktur- und Verifikationsschicht **innerhalb** der A-TownChain — kein eigenes
Netzwerk, kein eigener Konsens/State-Layer.

## Crate-Layout (Canonical Implementation: Rust)

| Crate | Aufgabe (Standard-Verweis) |
|---|---|
| `zkp-core` | Kern-Typen, ProofSystem-Trait, Registry-Interfaces (ZKP-001/002) |
| `zkp-verifier` | On-Chain-Verifikations-Engine (ZKP-004) |
| `zkp-prover` | Proof-Generator, off-chain (ZKP-002/007) |
| `zkp-circuits` | Circuit Registry, Versionierung, Compiler-Bindung (ZKP-003) |
| `zkp-crypto` | Commitments, Nullifier, Pedersen/Merkle-Primitiv; SHA-256 (AD-001, ZKP-005) |
| `zkp-vm` | ZKVM: ATCLang-Bytecode → Execution Trace → Proof (ZKP-009) |
| `zkp-sdk` | Entwickler-API fuer dApps/Wallet (ZKP-006/007) |

Pluggable Proof Architecture: **Groth16 · PLONK · Halo2 · STARK** + zukuenftige
Systeme — austauschbar ohne Blockchain-Umbau (ZKP-002).

## Status (AD-020-Rebuild-Aera)

- **Stand:** R1-Skeleton (ATC-STD-201) — Struktur + Governance + Standards stehen.
- **Qualitaets-Gates:** Kryptografie = S4-kritisch — G18 Security-Audit vor jedem
  Freeze (AD-023); Trusted-Setup-Kriterien (ZKP-010).
- Implementierung folgt im qualitaetsgetriebenen Rebuild (AD-023).

## Lizenz

Proprietaer — All Rights Reserved (ATC-LIC). Siehe [LICENSE](LICENSE).

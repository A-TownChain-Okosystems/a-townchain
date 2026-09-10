---
spec_id: ATC-NETWORK-ID-001
title: "Network Identity Specification (Chain-ID + Genesis-Lock)"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain
layer: L3-Chain
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Network Identity Specification (Chain-ID + Genesis-Lock) (ATC-NETWORK-ID-001)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Die unveränderliche Netz-Identität aus gekoppelten Feldern — Replay-Schutz und Upgrade-Governance auf Netz-Ebene.

## 2. Scope (gilt für)

- Identitäts-Felder
- Genesis-Hash-Bindung
- Netz-Tiers (658467 Mainnet / 658468 Testnet / 658469 Devnet)

## 3. Normative Anforderungen (MUST)

- **REQ-NID-001:** Network Identity = (chain_id, genesis_hash, protocol_version, consensus_version, vm_version) — die Kombination identifiziert ein Netz eindeutig; alle Felder sind Teil des Genesis-Commitments — *Nachweis: unit+vector*
- **REQ-NID-002:** chain_id ist replay-bindend in jeder Tx (WAL-REPLAY-001); TXs anderer Netze werden verworfen — *Nachweis: negative*
- **REQ-NID-003:** Versionswechsel (protocol/consensus/vm) ist MAJOR + Owner-Freigabe (COMPAT-001) — kein Silent-Upgrade; inkompatible Peers werden abgewiesen — *Nachweis: governance+negative*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Genesis-Commitment ist nach Genesis unveränderlich (genesis-locked)

## 6. Conformance-Tests (Mindestkategorien)

- network_id_vectors.json
- cross_chain_tx ⇒ Reject
- version_mismatch.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (10: Chain-ID als genesis-locked invariant)
- registry/networks.yaml (Tiers)

---
document_id: ATC-DOC-ARC-ATCH-001
title: Repository Architecture Specification
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-13
updated: 2026-09-13
standard: ATC-STD-MD-001
---

# Architecture Specification — a-townchain

## Übersicht

`a-townchain` ist der kanonische L1-Blockchain-Core (Protocol Layer L2) der A-TownChain: Chain-ID 658467, Transaktionen, Authorization Entry, Mempool, Block-/Finality-Protokoll und Economics. ATC-VM und deterministische State-Transition-Ausführung gehören zum separaten Protocol Layer L3 unter `a-townchain/components/vm`. Die Implementierung des Konsensusalgorithmus gehört zum kanonischen Algorithmus-Modul unter `a-townchain/components/algorithm`. Diese Repository-Architektur darf keine ältere L3-Zuordnung oder nicht eingefrorene Konsensusvariante als bereits kanonische Implementierung darstellen.

## Subsysteme

1. **Chain Protocol Core:** Block-/Transaktionsmodell, Chain-ID 658467, Validierungsregeln (`modules/`).
2. **Konsens-Bindung:** Das Konsensusprotokoll wird über den kanonischen Algorithmus-Vertrag eingebunden. Eine konkrete PoW/PoS/PoH- oder PoA-Kombination gilt erst nach einem eingefrorenen Standard und Exact-SHA-Conformance als kanonische Implementierung.
3. **ZKP-Integration:** Proof-Verifikation — kryptografische Primitive kanonisch in `atc-zkp`.
4. **On-Chain-Governance:** Abstimmungs- und Parameter-Änderungsmechanismen.
5. **Chain-DNS:** Namensauflösung auf der Chain.
6. **Kernel-Service (AD-012):** Dienste-Integration in das KAI-OS.

## Verantwortungsgrenzen

- `atc-node` betreibt das Protokoll (Full-Node-Binary/Runtime) — hier wird es definiert.
- `atc-algorithm` implementiert den Konsens — dieses Repo bindet ihn.
- `a-townchain/components/vm` führt Contracts aus — das Protokoll definiert die Ausführungssemantik.

## Registry-Einordnung

| Property | Value |
|---|---|
| Layer | L2 — Blockchain Core |
| Criticality | C1 |
| Security-Klasse | S4 |
| Maturity | R-Level laut `.atc/repository.yaml` · Statusleiter in `.atc/evidence/evidence.yaml` (SCR-0080) |
| Canonical | a-townchain (Chain-Protokoll, AD-012 Kernel-Service) |
| Domäne | domaene laut registry/repositories.yaml |

> Ehrlichkeitsregel: CLAIMED ≠ PASS · IMPLEMENTED ≠ VERIFIED — der verbindliche Implementierungsstand
> liegt ausschließlich in `.atc/evidence/evidence.yaml`, nicht in dieser Spezifikation.

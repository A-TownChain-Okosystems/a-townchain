# Governance & Security — Architektur

> **Stand:** 05.07.2026 | **Sprint:** 2.6 | **Standards:** ATC-02–05, 17, 18, 91

## Übersicht

Das Governance-System umfasst DAO-Voting, Treasury-Management, Timelock-Controller und Multi-Sig-Wallets — alles in ATCLang.

## Module (6 .atc Dateien)

| Modul | Datei | Zeilen | Beschreibung |
|-------|-------|--------|--------------|
| DAO Live | blockchain/governance/dao_live.atc | 234 | DAO with FFT+ATC voting, timelock |
| Treasury | blockchain/governance/treasury.atc | 219 | DAO Treasury, expense workflow, fund locking |
| Timelock | blockchain/governance/timelock.atc | 149 | Timelock controller, queue/execute/cancel |
| Governance Contract | modules/contracts/governance/governance_contract.atc | 236 | DAO voting, proposal lifecycle, quorum |
| Governance Simple | blockchain/contracts/governance/governance_contract.atc | 201 | On-chain voting (simplified variant) |
| Multisig | blockchain/wallet/multisig.atc | 267 | Multi-sig wallet v0.3, threshold, owners |

## Features

### DAO Voting
- Proposal Lifecycle: Create → Vote → Finalize → Execute
- Quorum (10% default) + Threshold (51% default)
- Timelock (48h default)
- Flash-Loan Protection: Voting-Power Snapshot bei Proposal-Erstellung (AD-003 RESOLVED)

### Treasury
- 7 Expense Categories (Dev, Infra, Marketing, Audit, Community, Reserve, Emergency)
- Fund Locking mit Timelock
- Guardian Emergency Pause
- Expense Workflow: Propose → Approve → Execute

### Multi-Sig
- Threshold-based (m-of-n)
- Add/Remove Owners
- Revoke Pending Transactions
- Auto-Execute bei Threshold-Ereichung

## Offene Tasks

- ATC-02: Liquid State Migration — pending
- ATC-03: DID & Zero-Trust IAM — pending (did.atc v0.1→v0.3)
- ATC-04: DAG Consensus — pending
- ATC-05: Quantum-Resistant Signatures — pending
- ATC-91: Cross-Chain Bridge → REVIEW status

---

*Governance Architecture · Sprint 2.6 · 05.07.2026 · Aurora*

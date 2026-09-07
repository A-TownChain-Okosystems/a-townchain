---
title: "ZKP-Layer Architektur"
summary: "Owner-Entwurf der ATC ZKP-Layer: Zielarchitektur, Kernkomponenten, Anwendungsfaelle (Private Tx, Balances, GameFi, Identity), pluggable Proof Systems, Rollups, ZKVM."
standard: ATC-STD-ZKP-001
decision: "AD-045"
date: "2026-09-07"
author: "Michael Wroblewski (Owner), Aurora (Superagent)"
status: "Owner-Entwurf — verbindlich via ATC-STD-ZKP-001…010 (candidate)"
---

# ZKP-Layer Architektur (AD-045)

## 1. Zielarchitektur

Die ZKP-Layer fungiert als eigenstaendige Protokollschicht **zwischen Core/L1 und
den Anwendungen bzw. Rollups**. Sie verifiziert die Korrektheit einer Berechnung
oder eines Zustandsnachweises, ohne die zugrunde liegenden Daten offenzulegen.

```
┌─────────────────────────────────────────────────────────┐
│                    A-TownChain Ecosystem                │
├─────────────────────────────────────────────────────────┤
│ dApps │ Wallet │ GameFi │ DeFi │ Marketplace │ AI       │
├─────────────────────────────────────────────────────────┤
│              ZKP Application / SDK Layer                │
├─────────────────────────────────────────────────────────┤
│                 ZKP-Layer                               │
│  Proof Generator │ Circuit Registry │ Verification Eng. │
│  Commitment Mgr  │ Nullifier Mgr    │ Proof Cache       │
├─────────────────────────────────────────────────────────┤
│              ATC Cryptographic Core                     │
├─────────────────────────────────────────────────────────┤
│                 A-TownChain L1                          │
│ Consensus │ State │ Transactions │ Validators │ DA      │
└─────────────────────────────────────────────────────────┘
```

## 2. Kernkomponenten

| Modul | Aufgabe | Standard |
|---|---|---|
| ZKP Verifier | Verifiziert Zero-Knowledge-Proofs on-chain | ATC-STD-ZKP-004 |
| Proof Generator | Erzeugt Proofs off-chain | ATC-STD-ZKP-002 |
| Circuit Registry | Registriert und versioniert zugelassene Circuits | ATC-STD-ZKP-003 |
| Commitment Manager | Verwaltet kryptografische Commitments | ATC-STD-ZKP-005 |
| Nullifier Manager | Verhindert Double-Spending bei privaten Assets | ATC-STD-ZKP-005 |
| Proof Registry | Speicherung/Referenzierung verifizierter Proofs | ATC-STD-ZKP-004 |
| ZKP SDK | Entwickler-API fuer dApps | ATC-STD-ZKP-006/007 |
| ZKP Wallet Module | Private Transaktionen und Credentials | ATC-STD-ZKP-007 |
| Circuit Compiler | Uebersetzt Berechnungen in ZKP-Circuits | ATC-STD-ZKP-003 |
| ZKP Governance | Freigabe, Deprecation und Upgrade von Circuits | ATC-STD-ZKP-003/010 |

## 3. ZKP-Funktionen fuer ATC

### 3.1 Private Transaktionen

```
Alice → (private transaction) → ZKP Prover → (Proof) → A-TownChain
A-TownChain: Verify Proof → Check Nullifier → Update State
```

Das Netzwerk stellt fest: „Die Transaktion ist gueltig." — ohne offenzulegen,
welche privaten Eingabedaten verwendet wurden (ATC-STD-ZKP-007).

### 3.2 Private Token-Balances

- **Public:** Commitment = C
- **Private:** Balance = 12.500 ATC, Owner Secret = S
- **Proof:** „Der Besitzer darf mindestens 1.000 ATC ausgeben."
- Der konkrete Kontostand wird nicht oeffentlich (ATC-STD-ZKP-005).

### 3.3 ZKP fuer GameFi (Genesis Chronicles)

Beweisbare Aussagen ohne On-Chain-Publikation der Spieldaten:
Item-Existenz, Item-Besitz, Item-Authentizitaet, Regel-Einhaltung;
Spieler-Level >= 50, NFT-Besitz, Quest-Abschluss, Score, Ressourcen,
PvP-Zugangsvoraussetzungen (ATC-STD-ZKP-007).

### 3.4 ZK Identity (ATC-ZK-Identity Protocol)

Privacy-preserving Identity: Alter >= erforderliches Alter, Credential
gueltig, nicht widerrufen, Identitaet nicht offengelegt (ATC-STD-ZKP-006).

## 4. Pluggable Proof Architecture

Keine harte Kopplung an einen Algorithmus. Beweissysteme: **Groth16, PLONK,
Halo2, STARK** + zukuenftige Systeme. Interface (ATC-STD-ZKP-002):

```
ProofSystem
├── setup()
├── prove()
├── verify()
├── serialize()
├── deserialize()
└── version()
```

Austausch eines Proof-Systems ohne Umbau der Blockchain-Architektur.

## 5. ZKP-Layer und Rollups (ATC-STD-ZKP-008)

Langfristige Skalierungsschicht: zkRollup (DeFi/GameFi), zkVM (General
Compute), zkApp (Private Applications). Die L1 prueft nur:

```
State_old + Proof → State_new
```

## 6. ZKVM (ATC-STD-ZKP-009)

```
ATCLang → ATCLang Compiler → ATC Bytecode → ZKVM
      → Execution Trace → ZKP → A-TownChain L1
```

Relevant, sobald ATCLang Smart Contracts oder deterministische
Compute-Workloads unterstuetzt (AD-006/99).

## 7. Wichtigste Designentscheidung

Die ZKP-Layer ist **zunaechst eine kryptografische Infrastruktur- und
Verifikationsschicht** — nicht sofort ein eigenes Netzwerk. Keine separate
„ZKP-Blockchain" mit fruehem zusaetzlichem Konsens-, Netzwerk- und State-Layer:

```
ATC L1
├── Consensus ├── State ├── Execution ├── DA
└── ZKP Verification
    ├── Private Transactions ├── ZK Identity
    ├── GameFi Proofs ├── zkApps └── zkRollups
```

---
document_id: ATC-MASTER-BACKLOG-001
title: "Repositoryuebergreifendes Master-Backlog"
generated: "2022-01-01"
status: GENERATED
normative: false
generator: backlog/tools/build_backlog.py
---

# Master-Backlog — A-TownChain / KAI-OS

> **Nicht-normativ, generiert.** Quelle ist das Blocker-Register in
> `a-townchain-ecosystem` (PR #247, Stand 2026-11-17) plus die live
> geprueften Zahlen vom 2026-11-19. Diese Datei
> enthaelt keine eigene Fachwahrheit und keine handgepflegten Zaehler.

## Bestand

| Kennzahl | Wert |
|---|---|
| Repositories (gesamt / aktiv / archiviert) | 33 / 26 / 7 |
| Offene Pull Requests | 244 |
| Offene Issues (davon P0 / P1) | 149 (13 / 10) |
| Branch Protection (geschuetzt / ungeschuetzt) | 25 / 1 |
| Aufgaben im Backlog | 37 |

## Widersprueche zwischen Bericht und Live-Bestand

| Behauptung | Live-Ergebnis | Methode |
|---|---|---|
| 4 von 26 aktiven Repositories geschuetzt, 22 ohne Branch Protection | 25 von 26 geschuetzt; ungeschuetzt ist nur demo-repository | GitHub API /repos/{org}/{repo}/branches/main/protection, 26 aktive Repositories, 2026-11-19 |
| Das Register enthaelt 32 Blocker-Eintraege | 31 Eintraege im gepinnten Snapshot (PR #247, head d691e51d) | yaml.safe_load auf docs/system-readiness/BLOCKER_REGISTER.yaml, len(blockers) |
| Der PR-Text nennt 15 Blocker | 31 Eintraege in der YAML, davon 4 CLOSED | Vergleich PR-Beschreibung gegen Dateiinhalt |

## Evidence-Bindung

| Kennzahl | Wert |
|---|---|
| Aufgaben mit Evidence-Kopf | 31 |
| davon vollstaendiger 40-stelliger SHA | 0 |
| davon Kurz-SHA (nicht releasefaehig) | 31 |
| Aufgaben ohne Evidence | 6 |

Ein Kurz-SHA ist ein Hinweis, kein Nachweis. Fuer Release-Gates muss der
vollstaendige SHA samt Workflow-Run, Job und Exit-Code vorliegen.

## Reihenfolge

**Phase 0 — Repository- und CI-Stabilisierung** (19 Aufgaben)

ATC-TASK-P0-101, ATC-TASK-P0-102, BLK-P0-01, ATC-TASK-P1-101, ATC-TASK-P1-102, ATC-TASK-P1-103, BLK-P1-05, BLK-P1-06, BLK-P1-07, BLK-P1-08, BLK-P1-09, ATC-TASK-P2-101, BLK-P2-02, BLK-P2-03, BLK-P2-08, BLK-P2-09, BLK-P2-10, BLK-P2-11, BLK-P2-12

**Phase 1 — Normative Grundlagen** (8 Aufgaben)

BLK-P0-02, BLK-P1-04, BLK-P1-10, BLK-P1-11, BLK-P1-12, BLK-P1-13, BLK-P1-14, BLK-P2-01

**Phase 2 — Blockchain-Basis** (0 Aufgaben)

—

**Phase 3 — ATCLang und ATC-VM** (2 Aufgaben)

BLK-P0-03, BLK-P0-04

**Phase 4 — ShivaCore und GlobusOS** (4 Aufgaben)

BLK-P1-01, BLK-P1-02, BLK-P1-03, BLK-P2-13

**Phase 5 — Aurora AI** (1 Aufgaben)

BLK-P2-04

**Phase 6 — Genesis Engine und Oekosystemdienste** (1 Aufgaben)

BLK-P2-05

**Phase 7 — Gesamtsystem-Verifikation** (2 Aufgaben)

BLK-P2-06, BLK-P2-07

## Aufgaben

| ID | Prio | Phase | Status | SSOT | Titel |
|---|---|---|---|---|---|
| ATC-TASK-P0-101 | P0 | 0 | OPEN | A-TownChain-Okosystems (org-weit) | PR-Triage: 244 offene PRs klassifizieren und schliessen |
| ATC-TASK-P0-102 | P0 | 0 | OPEN | A-TownChain-Okosystems (org-weit) | Issue-Triage: 149 offene Issues auf Duplikate, Erledigung und Abhaengigkeiten pruefen |
| BLK-P0-01 | P0 | 0 | CLOSED | .github (Org) | CI startup_failure org-wide |
| BLK-P0-02 | P0 | 1 | BLOCKED | atc-standards (ATC-BC-001) + a-townchain/components/vm | ATC-BC-001: normative Bytecode-Spezifikation |
| BLK-P0-03 | P0 | 3 | BLOCKED | atclang | Contract-Lowering: ATCLang .atc -> Bytecode |
| BLK-P0-04 | P0 | 3 | BLOCKED | a-townchain/components/node | E2E-Nachweis: consensus-verifizierte Contract-Ausfuehrung |
| ATC-TASK-P1-101 | P1 | 0 | OPEN | .github (Org) | Governance-Abdeckung gegen den Live-Bestand reconciliieren |
| ATC-TASK-P1-102 | P1 | 0 | OPEN | a-townchain/components/vm | SSOT-Freeze: atc-vm und atc-algorithm duerfen nicht parallel weiterentwickelt werden |
| ATC-TASK-P1-103 | P1 | 0 | OPEN | a-townchain (Monorepo) | Zustaendigkeit der sieben archivierten Repositories uebertragen |
| BLK-P1-05 | P1 | 0 | CLOSED | a-townchain-ecosystem | Required Context matrix ist auf main rot und blockiert alle PRs in atc-standards |
| BLK-P1-06 | P1 | 0 | FIX_PENDING_OWNER_PUSH | a-townchain-ecosystem | Ecosystem-Integritaets-Gate widerspricht Entscheidung E-4 (components/globus-os) |
| BLK-P1-07 | P1 | 0 | IN_PROGRESS | a-townchain (Rust-Workspace) | Rust-Gates - drei getrennte Ursachen statt eines Snapshot-Defekts |
| BLK-P1-08 | P1 | 0 | FIX_IN_SOURCE_PR | a-townchain/apps/atc-explorer | atc-explorer - TypeScript-Abhaengigkeit kollidiert mit ts-jest (ERESOLVE) |
| BLK-P1-09 | P1 | 0 | IN_PROGRESS | a-townchain (Python-Bestand) | Python-Befunde - 242 ruff-Funde in 10 von 21 Projekten, zwei Klassen |
| BLK-P1-04 | P1 | 1 | BLOCKED | a-townchain-ecosystem (ATC-ARCH-001) | Architektur-Konsolidierung: ATC-ARCH-001 als einzige normative Quelle |
| BLK-P1-10 | P1 | 1 | IN_PROGRESS | atc-standards + a-townchain/apps/atc-contracts | Max Supply widerspruechlich: 360.000.000 ATC gegen 21.000.000 |
| BLK-P1-11 | P1 | 1 | OPEN | atc-standards (ATC-STD-600) | Signaturverfahren widerspruechlich: Ed25519 normativ, secp256k1 in Katalog und Code |
| BLK-P1-12 | P1 | 1 | OPEN | atc-standards (ATC-STD-600) | Feldbreite widerspruechlich: amount als u64 gegen 18 Dezimalstellen und 360M Supply |
| BLK-P1-13 | P1 | 1 | FIX_IN_SOURCE_PR | a-townchain/apps/atc-contracts | Governance-Vertrag fuehrt total_supply mit 21 Mio, kanonisch sind 360 Mio |
| BLK-P1-14 | P1 | 1 | OPEN | a-townchain/apps/atc-contracts | Vertragsbaum dreifach vorhanden: apps/atc-contracts, components/contracts, modules/atc-contracts |
| BLK-P1-01 | P1 | 4 | BLOCKED | globus-os/modules/atc-shivacore/kernel | E-1 bis E-7: ABI- und Rechteentscheidungen |
| BLK-P1-02 | P1 | 4 | BLOCKED | globus-os/modules/atc-shivacore/kernel | Hardware-Kette und VMM-Conformance-Freeze |
| BLK-P1-03 | P1 | 4 | BLOCKED | globus-os/modules/atc-shivacore/kernel | Kernel: Service-Space im Image, Entropie, eine Capability-/Geraeteautoritaet, SMP |
| ATC-TASK-P2-101 | P2 | 0 | OPEN | a-townchain-ecosystem | Register-Zaehlung korrigieren: 31 Eintraege, nicht 15 und nicht 32 |
| BLK-P2-02 | P2 | 0 | BLOCKED | a-townchain-ecosystem | Evidence-Bindung: exact-SHA statt Prosa-Zahlen |
| BLK-P2-03 | P2 | 0 | BLOCKED | .github (Org) | Governance-Abdeckung und PR-Bypass |
| BLK-P2-08 | P2 | 0 | CLOSED | atc-standards | FILE_REGISTER.md in atc-standards ohne Bezug zum Bestand |
| BLK-P2-09 | P2 | 0 | CLOSED | a-townchain-ecosystem | ATC-ARCH-002 ist auf main, aber nicht freigegeben |
| BLK-P2-10 | P2 | 0 | OPEN | atc-standards | regenerate auf atc-standards/main ist strukturell rot: Views tragen eine Wall-Clock-Zeitangabe |
| BLK-P2-11 | P2 | 0 | TODO | a-townchain (CI) | Fuenf Test- und Build-Gates ohne isolierte Ursache |
| BLK-P2-12 | P2 | 0 | OPEN | atc-standards | SCR-ID-Kollision: SCR-0129 ist vier Gegenständen zugeordnet |
| BLK-P2-01 | P2 | 1 | BLOCKED | atc-standards | Standards-Umsetzung: 330 von 533 specification_only |
| BLK-P2-13 | P2 | 4 | IN_PROGRESS | globus-os | Keine normative Datenarchitektur fuer GlobusOS; Klassifizierung fehlt im Datenpfad |
| BLK-P2-04 | P2 | 5 | BLOCKED | aurora-ai | Aurora AI: Integrationstiefe nachweisen |
| BLK-P2-05 | P2 | 6 | BLOCKED | genesis-engine | Genesis Engine: Reifegrad-Assessment |
| BLK-P2-06 | P2 | 7 | BLOCKED | a-townchain | Chain-Sicherheitsarchitektur: unabhaengige Tiefenanalyse |
| BLK-P2-07 | P2 | 7 | BLOCKED | a-townchain | Release-Gate-Kriterien erweitern |

## Abhaengigkeiten

- **BLK-P0-02** setzt voraus: BLK-P0-01
- **BLK-P0-03** setzt voraus: BLK-P0-02
- **BLK-P0-04** setzt voraus: BLK-P0-03
- **ATC-TASK-P1-101** setzt voraus: BLK-P2-03
- **BLK-P1-06** setzt voraus: BLK-P1-04
- **BLK-P1-07** setzt voraus: BLK-P0-01
- **BLK-P1-08** setzt voraus: BLK-P0-01
- **BLK-P1-09** setzt voraus: BLK-P0-01
- **BLK-P1-13** setzt voraus: BLK-P1-10
- **BLK-P1-14** setzt voraus: BLK-P1-10
- **BLK-P1-01** setzt voraus: BLK-P0-01
- **BLK-P1-02** setzt voraus: BLK-P1-01
- **BLK-P1-03** setzt voraus: BLK-P1-01
- **BLK-P2-03** setzt voraus: BLK-P0-01
- **BLK-P2-10** setzt voraus: BLK-P2-08
- **BLK-P2-11** setzt voraus: BLK-P0-01
- **BLK-P2-13** setzt voraus: BLK-P1-01
- **BLK-P2-04** setzt voraus: BLK-P1-01
- **BLK-P2-06** setzt voraus: BLK-P0-04
- **BLK-P2-07** setzt voraus: BLK-P2-06

## Evidence-Regel

Ein Task gilt erst als geschlossen, wenn der Nachweis an einen Commit gebunden ist:
Repository, Branch, vollstaendiger SHA, Datei und Symbol, Workflow-Run, Job,
Step, Exit-Code und Log. Ein gruener CI-Lauf allein ist kein Funktionsnachweis.

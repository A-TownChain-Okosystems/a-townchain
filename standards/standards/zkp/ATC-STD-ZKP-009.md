---
standard:
  id: ATC-STD-ZKP-009
  title: "ATC-STD-ZKP-009 — ZKVM Standard"
  version: "1.1.0"
  status: approved
  category: zkp
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  supersedes: null
  superseded_by: null

  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-ZKP-009 — ZKVM Standard (v1.1.0, APPROVED)

> **Version:** 1.1.0 (FORMAL)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Reihe:** ATC-STD-ZKP-001–010 (ZKP-Layer, AD-045) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Scope:** ATC ZKP-Layer (Repo atc-zkp) und ihre Integration in die A-TownChain L1 (Chain-ID 658467, AD-004).
> **Verweise:** ATC-STD-ZKP-001 (Architektur), ATC-STD-ZKP-010 (Security), AD-045, AD-001 (SHA-256)

---

## Abstract

ZKVM: ATCLang → ATCLang Compiler → ATC Bytecode → ZKVM → Execution Trace → ZKP → L1. Relevant fuer deterministische Compute-Workloads und Smart Contracts in ATCLang (AD-006/99).

## REQ-Matrix (normative Anforderungen)

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-ZKP-901 | Die ZKVM MUSS ATC-Bytecode (ATCLang-Compiler-Artefakt) als Eingabe verwenden — keine Fremd-Bytecode-Standards ohne AD. | §S.2 |
| id: REQ-ZKP-902 | Execution Traces MUESSEN deterministisch reproduzierbar sein. | §S.2 |
| id: REQ-ZKP-903 | Die ZKVM MUESSEN als eigener Crate (zkp-vm) im atc-zkp-Workspace gefuehrt werden. | §S.2 |

| id: REQ-ZKP-904 | Die ZKVM MUSS die maximale Anzahl von Befehlsschritten pro Ausführungstrace strikt begrenzen. | §S.2 |
| id: REQ-ZKP-905 | Der Instruction-Decoder MUSS mathematisch bewiesen frei von unvollständigen Opcode-Decodings sein. | §S.2 |
| id: REQ-ZKP-906 | Execution-Trace-Chunks MÜSSEN unabhängig voneinander parallelisierbar bewiesen werden können. | §S.3 |
| id: REQ-ZKP-907 | Speicherzugriffe innerhalb der ZKVM MÜSSEN über RAM-Consistency-Circuits verifiziert werden. | §S.3 |

## Metriken & Akzeptanzkriterien

- **Metrik 1:** Maximal unterstützte Ausführungsschritte pro Trace MUSS 1.000.000.000 Schritte betragen.
- **Metrik 2:** Parallelisierungseffizienz der Chunk-Beweise MUSS mindestens 85% erreichen.
- **Metrik 3:** Decoder-Korrektheitsrate MUSS exakt 100% betragen.
- **Metrik 4:** Speicher-Konsistenz-Prüfrate MUSS bei 100% liegen.


## Metriken & Akzeptanzkriterien

- **Metrik 1:** Maximal unterstützte Ausführungsschritte pro Trace MUSS 1.000.000.000 Schritte betragen.
- **Metrik 2:** Parallelisierungseffizienz der Chunk-Beweise MUSS mindestens 85% erreichen.
- **Metrik 3:** Decoder-Korrektheitsrate MUSS exakt 100% betragen.
- **Metrik 4:** Speicher-Konsistenz-Prüfrate MUSS bei 100% liegen.
- **Metrik 5:** Befehlsausführungsgeschwindigkeit der ZKVM MUSS kontinuierlich gemessen werden.

## Compliance & Verifikation

Geprüft durch: (1) `atc-std-validator` (Registry-Sync S-14, Kopf-Sync S-19, Testläufe S-01 bis S-25), (2) ZKP-Review im G18-Security-Audit (AD-023), (3) Repo-Audit R3 (ATC-STD-201ff). Abweichungen MÜSSEN als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 geführt und im Ticket-System protokolliert werden.

## Security Considerations

- Opcodes mit unbestimmter Laufzeit MÜSSEN in der ZKVM verboten oder mit festen Gas-Limits versehen werden.
- Register-Zustände MÜSSEN nach jedem Ausführungsschritt auf Gültigkeit geprüft werden.
- Der ZKVM-Crate MUSS gegen Memory-Safety-Bugs in Rust abgesichert sein.
- Stack-Overflows und Underflows in der ZKVM MÜSSEN zu sofortigen Abbruch-Circuits führen.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung aus dem Owner-Entwurf ZKP-Layer (AD-045). Status: APPROVED |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard (Verfassung, v1.1.0 APPROVED)
- ATC-STD-100
- ATC-STD-201
- ATC-STD-BUG-001
- ATC-STD-ZKP-001
- ATC-STD-ZKP-009
- ATC-STD-ZKP-010

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- AD-045 — ZKP-Layer-Festlegung (Repo atc-zkp, Standards-Serie)

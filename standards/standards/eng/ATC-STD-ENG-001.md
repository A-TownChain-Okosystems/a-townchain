---
standard:
  id: ATC-STD-ENG-001
  title: "ATC-STD-ENG-001 — Software Engineering & Code Quality Standard"
  version: "1.0.0"
  status: approved
  category: eng
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf)
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  supersedes: null
  superseded_by: null
  effective_date: "2026-09-11"
  dependencies: [ATC-STD-000, ATC-STD-081, ATC-STD-140, ATC-STD-VERSION-001, ATC-AI-GOV-001]
  conformance: {required: true}
  implementation: {status: specification_only}
---

# ATC-STD-ENG-001 — Software Engineering & Code Quality Standard (v1.0.0, APPROVED)

**Standard-ID:** ATC-STD-ENG-001
**Version:** 1.0.0
**Status:** APPROVED — §9-Freigabe Michael 11.09.2026 21:21, §30-eingefroren
**Autorität:** A-TownChain-Okosystems
**Owner:** Michael (Owner-Entwurf 11.09.2026)
**Kategorie:** eng
**Normativ:** Ja
**Mandat:** SCR-0092 — Owner-Manifest »Guter Code ist deterministischer, testbarer, sicherer, wartbarer und standardkonformer Systemcode« (18 Punkte, 11.09.2026 21:00)

---

## 1. Zweck (Purpose)

Dieser Standard definiert, was »guter Code« im A-TownChain-Ökosystem bedeutet — nicht
ästhetisch, sondern systemisch:

> **Correct by design → deterministic where required → secure by default → testable →
> auditable → reproducible → versioned → standards-compliant.**

Weil Blockchain, ATCLang/VM, ShivaCore, Aurora und GlobusOS verbunden sind, gilt eine
strengere Engineering-Disziplin als bei gewöhnlichen Webprojekten. Ein Fehler im Consensus-
oder VM-Code ist kein Bug — er ist ein Konsens-Bruch, ein Geldverlust oder ein Fork.

## 2. Geltungsbereich (Scope)

Verbindlich für **alle 26 Repositories** der Organisation A-TownChain-Okosystems
(ausgenommen: EXCLUDED_REPOS gemäß Org-Governance). Verschärfte Pflichten (MUST-STRICT)
gelten für Konsens-, VM-, Krypto-, Kernel- und Storage-Code (Determinism-Klasse D-CRITICAL,
siehe Anhang A).

## 3. Begriffe und Definitionen

| Begriff | Definition |
|---|---|
| **D-CRITICAL** | Code, der Konsens- oder Sicherheitsentscheidungen beeinflusst (Consensus, VM, Krypto, Kernel, Storage Core) |
| **Determinismus** | F(state, tx, block) → identisches Ergebnis auf allen Knoten, zu jedem Zeitpunkt |
| **Quality Gate** | Automatisierter CI-Check, der Merges blockiert (Fail Closed) |
| **Code Quality Matrix** | Maschinenlesbares Profil je Repository (registry/code-quality-matrix.yaml) |
| **Regressionstest** | Test, der einen ehemals behobenen Fehler dauerhaft absichert |
| **Source of Truth** | Eindeutige, autoritative Definition (Spec, Referenz-Implementierung, Conformance-Tests) |

## 4. Normative Anforderungen

### REQ-ENG-001 — Klare Verantwortung je Komponente

Jedes Repository hat genau eine Systemverantwortung ( siehe Architekturbaum, Anhang B).
Ein Repository DARF NICHT die Aufgaben eines anderen übernehmen.
Entscheidungsfluss bei AI-Komponenten ist verpflichtend:

```
Aurora AI → Vorschlag → ATC → deterministische Entscheidung → State Transition → Blockchain State
```

**VERBOTEN:** `AI → entscheidet selbst → Blockchain State`. AI schlägt vor; die Kette
entscheidet deterministisch. Dies ist eine zentrale Architekturregel des Gesamtsystems.

### REQ-ENG-002 — Determinismus als First-Class-Requirement (MUST-STRICT für D-CRITICAL)

Der gleiche Input MUSS auf allen Knoten das identische Ergebnis liefern.
**Verboten in konsens-relevantem Code:** `random()`, `current_time()`, `system_clock()`,
Thread-Races, `unordered_map`-Iteration (ohne kanonische Sortierung),
plattformspezifisches Verhalten — sofern diese Werte den Konsens beeinflussen.

Die ATC-VM MUSS einen expliziten Determinism Layer besitzen: deterministic memory,
arithmetic, execution, gas, storage, serialization, errors.

### REQ-ENG-003 — Explizite Fehlerbehandlung (MUST-STRICT für D-CRITICAL)

Fehler dürfen NIEMALS versteckt werden. In produktivem Consensus-/Security-Code ist
`unwrap()` / `panic!()` / leere `except` VERBOTEN. Fehlerverarbeitung folgt der Kette:

```
Fehler → klassifizieren → dokumentieren → behandeln → testen → Regressionstest
```

Jeder behobene kritische Bug MUSS einen Regressionstest erhalten — ein einmal gefundener
Bug darf nie ein zweites Mal auftreten können.

### REQ-ENG-004 — Security by Design

Die Reihenfolge ist verbindlich:
Requirement → Threat Model → Design → Implementation → Unit Test → Property Test →
Fuzzing → Security Review → Integration Test → Release.

Private Keys DÜRFEN NIEMALS in Logs, Exceptions oder Debug-Ausgaben erscheinen
(Key-Hygiene entlang der Signatur-Grenze).

### REQ-ENG-005 — Testbarkeit & Testpyramide

Jede kritische Funktion MUSS testbar sein (isolierte Phasen: validate → authorize →
execute → charge_gas → commit statt Monolithen). Verbindliche Ebenen: Static Analysis →
Unit → Property → Integration → Acceptance. Für D-CRITICAL zusätzlich: Fuzzing,
Differential Testing; für VM/Consensus/Krypto/ShivaCore ernsthaft zu prüfen: Formal
Verification und Independent Implementation (zweite Referenz).

### REQ-ENG-006 — CI-Quality-Gates (Fail Closed)

Ein Pull Request MUSS automatisch bestehen: Formatting, Lint, Build, Unit Tests,
Integration Tests, Security Scan, Dependency Audit, License Check, API Compatibility,
Documentation Check, Determinism Tests (D-CRITICAL) — plus Required Review.
**Kein grünes Gate → kein Merge.** Standards müssen den Code erzwingen (Standard →
GitHub Rules → CI → automatischer Check → Merge-Verbot), nicht nur dokumentieren.

### REQ-ENG-007 — Dependency-Budgets je Layer

Core-Komponenten minimale Abhängigkeiten. ShivaCore: `no_std`, minimal dependencies,
kryptografisch geprüft. Consensus: minimal, deterministisch, auditierbar. VM:
kontrolliert, deterministisch, version-locked. UI: liberal erlaubt. Das Budget ist je
Layer in der Code Quality Matrix (Anhang A) festgelegt.

### REQ-ENG-008 — Source of Truth

Für jedes kritische System existiert genau eine autoritative Kette, z. B. ATCLang:
Language Specification → Reference Compiler → Conformance Tests; VM: VM Specification →
Canonical Rust VM → Independent Verifier → Conformance Suite. Divergenz der Form
»Spec sagt A, Python macht B, Rust macht C, Tests erwarten D« ist ein Zustand der Kategorie
CONFLICT und MUSS sofortig behoben werden. Python darf nicht zur Quelle der Wahrheit
werden, wenn Rust kanonisch ist.

### REQ-ENG-009 — API-First & Versionierung

Schnittstellen werden VOR der Implementierung definiert (Interface → Contract →
Implementation → Tests). Jede öffentliche API MUSS SemVer folgen (MAJOR = Breaking,
MINOR = kompatibel neu, PATCH = Bugfix). Breaking Changes erfordern einen Change Request
(SCR). Code, Spezifikation und Implementierung MÜSSEN zusammenpassen.

### REQ-ENG-010 — Modulstruktur & Code-Organisation

1 Modul = 1 klarer Verantwortungsbereich. Riesige Mehrzweck-Dateien (Lexer+Parser+CodeGen
in einer Datei) sind VERBOTEN; Aufteilung in kohäsive Module ist verpflichtend.
Magic Numbers sind VERBOTEN: benannte Konstanten bzw. zentrale, versionierbare Definitionen
(z. B. `GasSchedule::base_transaction()`). Kommentare sind kein Ersatz für Architektur;
sie erklären das Nicht-Offensichtliche (z. B. »Consensus-critical: ordering must remain
canonical across all nodes«).

### REQ-ENG-011 — ATC Code Quality Matrix (maschinenlesbar)

Für jedes Repository liegt ein verbindliches Profil in
`registry/code-quality-matrix.yaml` (SSOT): Sprache, Architektur-Layer, erlaubte
Dependencies, Coding Rules, Security Requirements, Test Requirements, Coverage-Ziel,
Fuzzing, Performance, Determinismus-Klasse, API Stability, Dokumentation, Lizenz,
CI Gates, Release Gates. Die Matrix ist automatisiert zu erheben und zu prüfen
(tools/code_quality_matrix.py); Abweichungen erzeugen Findings (ATC-STD-BUG-001).

### REQ-ENG-012 — Evidenz-Pflicht

Keine synthetischen PASS-Nachweise ohne ausführbare Evidenz (Testläufe, Check-Runs,
API-Bestätigungen gemäß ATC-AI-GOV). Kompatibel mit der Evidence-only Governance:
Behauptungen über Qualität sind nur mit Evidence gültig.

## 5. Rollen und Verantwortlichkeiten

| Rolle | Verantwortung |
|---|---|
| Owner (Michael) | §9-Freigabe, Release Gates, DECISION-Items |
| CodingAgent / Aurora (ATC-AI-ARCH-001) | Umsetzung, Standards-Konformität, Evidence |
| QAAgent (ATC-AI-TEST-001) | Testpyramide, Regressionstests, Coverage |
| SecurityAgent (ATC-AI-SEC-001) | Threat Models, Security Review, Key-Hygiene |
| Governance / CI | Automatische Gates, Matrix-Erhebung, Findings |

## 6. Schnittstellen und Abhängigkeiten

- **ATC-STD-000** (Verfassung): Lifecycle, Registry, §9-Freigabe
- **ATC-STD-081** (Coding Standard): Funktionsebene-Regeln — ENG-001 ist das disziplinäre Dach
- **ATC-STD-140/141/142** (QA Framework, Unit-, Integration Testing): Testpyramide im Detail
- **ATC-STD-AUDIT-001**: Audit-Erhebung der Matrix-Werte
- **ATC-STD-BUG-001/002/003**: Findings aus Matrix-Abweichungen
- **ATC-STD-VERSION-001**: SemVer, Release-IDs
- **ATC-AI-GOV-001**: Evidence-First, Fail Closed, No Self-Certification

## 7. Compliance / Prüfungen

- **COM-ENG-001:** Registry-Eintrag + DRAFT→APPROVED-Lifecycle konform (ATC-STD-DESC-001)
- **COM-ENG-002:** `registry/code-quality-matrix.yaml` vorhanden, vollständig (26 Repos), schemavalide
- **COM-ENG-003:** Je Repo: CI-Gates gemäß Matrix aktiv (API-verifizierbar)
- **COM-ENG-004:** D-CRITICAL-Repos: Determinism-Tests und unwrap-/panic-Freiheit geprüft
- **COM-ENG-005:** Regressionstests für behobene kritische Bugs vorhanden

## 8. Ausnahmeverfahren

Ausnahmen nur als ATC-EXC-NNN mit Owner-Approval und Ablaufdatum (ATC-AI-GOV-Standard);
abgelaufene Ausnahmen sind automatisch inaktiv. Keine stillen Abweichungen.

## 9. Versionierung und Änderungsmanagement

Änderungen ausschließlich via SCR (Standard Change Request) gemäß ATC-STD-000 §9;
nach APPROVED §30-eingefroren. Versionierung nach ATC-STD-VERSION-001 (SemVer).

## Anhang A — ATC Code Quality Matrix (Belegung)

Maschinenlesbar in `registry/code-quality-matrix.yaml`. Determinismus-Klassen:
D-CRITICAL (Konsens/VM/Krypto/Kernel), D-HIGH (Storage, Indexer, Interop), D-STANDARD
(UI, AI, Docs). Dependency-Budgets: B-ZERO (keine externen), B-MINIMAL (<5, geprüft),
B-CONTROLLED (<15, version-locked), B-LIBERAL (UI).

## Anhang B — Architekturbaum & Entwicklungszyklus

**Verantwortungsbaum:** atc-standards (Regeln/Verträge/Governance) · a-townchain
(Chain/Consensus/State) · atc-vm (deterministische Ausführung) · atclang (Sprache/Compiler/
Toolchain) · atc-shivacore (Kernel/Hardware/Isolation) · atc-storage (persistente Daten) ·
atc-wallet (Keys/Accounts/Signing) · atc-compute (Compute-Ressourcen) · aurora-ai
(KI/Interpretation/UX) · globus-os (OS/Plattform) · a-townchain-os (Integration/
Orchestrierung). L0–L7-Layer gemäß Org-Architektur (SCR-0062/0072).

**Entwicklungszyklus (verbindlich):**
IDEA → REQUIREMENT → SPECIFICATION → ARCHITECTURE → THREAT MODEL → IMPLEMENTATION →
TEST → (SECURITY ∥ FUZZING) → CODE REVIEW → CI GATES → CONFORMANCE → MERGE → RELEASE →
MONITORING → BUG/INCIDENT → REGRESSION TEST.

**Sprachstrategie (Primärsprachen):** ShivaCore/VM/Blockchain-Core/Consensus/Krypto/
Storage-Core/ATCLang-Compiler: **Rust** · AI: **Python** · Backend: Rust/TypeScript ·
Web-UI: **TypeScript** · Smart Contracts: **ATCLang** · Tests/Research: Python.
Python wird nicht Quelle der Wahrheit, wo Rust kanonisch ist (REQ-ENG-008).

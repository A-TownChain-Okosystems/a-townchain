---
standard:
  id: ATC-STD-V2S-005
  title: "Traceability Standard"
  version: "1.0.0"
  status: candidate
  category: v2s
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle ATC-Projekte gemäß ATC-STD-V2S-000 Scope"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-V2S-005 — Traceability Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-005 etabliert die verbindlichen Regeln zur Herstellung, Pflege und Auditierung
einer lückenlosen Rückverfolgbarkeit (Traceability) im gesamten Entwicklungszyklus.
Der Standard fordert eine zentrale Traceability-Matrix als Pflichtartefakt über die
Kette REQ → SPEC → ARCH → CODE → TEST → RELEASE. Er garantiert, dass bei jedem Audit
präzise nachgewiesen werden kann, durch welchen Code und welchen Test eine Anforderung
implementiert und verifiziert wird.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/DARF NICHT — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** alle ATC-Softwareprojekte, Repositories, Smart Contracts und KI-Systeme
über sämtliche Lifecycle-Phasen hinweg.
**Nicht-Gilt:** isolierte Dokumentations-Repositories ohne direkten Bezug zu
Laufzeitkomponenten oder Quellcode.

## §1 Gegenstand & Verortung

Dieser Standard ist eine universelle Querschnittsdisziplin, die sich über alle 20 Stationen
des Lifecycle Master (ATC-STD-V2S-000) erstreckt. Er verknüpft die artefakterzeugenden
Phasen von Station 4 (REQUIREMENTS) bis Station 13 (RELEASE). Er beantwortet direkt
die kanonische Audit-Frage: „Welche Anforderung wird durch welchen Code implementiert
und durch welchen Test nachgewiesen?"

## §2 Kernregeln

1. **KR-1:** Jedes ATC-Softwareprojekt MUSS eine zentrale, maschinell oder textuell lesbare Traceability-Matrix als Pflichtartefakt führen (MUST — Bezug Master §11).
2. **KR-2:** Die Traceability-Matrix MUSS für jedes Requirement die Kette REQ → SPEC → ARCH (ADR) → CODE (Modul) → TEST → RELEASE abdecken (MUST).
3. **KR-3:** Quellcode ohne nachgewiesene Rückverfolgbarkeit auf ein gültiges Requirement gilt als nicht-konform (Orphaned Code) und MUSS korrigiert werden.
4. **KR-4:** Ein Audit MUSS anhand der Traceability-Matrix jederzeit ohne manuelle Rekonstruktion den Implementierungs- und Teststatus abrufen können.
5. **KR-5:** Bei jeder Änderung an Anforderungen, Spezifikationen, Architekturen, Modulen oder Tests MUSS die Matrix synchron aktualisiert werden.
6. **KR-6:** Requirements ohne lückenlose Zuordnung zu Tests und Releases DÜRFEN NICHT in ein Production-Release übernommen werden (MUST NOT).

## §3 Artefakte & Nachweispflichten

Im Rahmen des Traceability-Prozesses entstehen folgende verbindliche Artefakte:

- **Traceability Matrix:** Pflichtdokument (`docs/TRACEABILITY_MATRIX.md` oder `.atc/traceability.yaml`) mit den Spalten REQ | SPEC | ADR | Modul | Test | Version.
- **Traceability Audit Log:** Protokollierung automatisierter Prüfläufe zur Erkennung von Lücken.
- **Orphan Code Report:** Liste aller Quellcode-Module ohne zugewiesenes Requirement.
- **Unmapped REQ Report:** Liste aller Requirements ohne vollständige Test- oder Code-Zuordnung.

## §4 Quality-Gate-Bezug

Dieser Standard ist für alle Quality Gates **V2S-G1 bis V2S-G8** von zentraler Bedeutung:

- **Prüffrage:** Ist die Traceability-Kette für die betroffenen Artefakte lückenlos und konsistent?
- **Release-Blocker:** Ein Verstoß gegen die Traceability-Kette blockiert das Gate **V2S-G8** (Release-Freigabe).
- **Gate-Record-Pflicht:** Nachweis der Traceability-Prüfung im jeweiligen Gate-Record.
- **Reifegrad:** Die lückenlose Traceability ist Grundvoraussetzung für das Erreichen von Reifegrad **M5 (VERIFIED)** und **M7 (RELEASED)**.

## §5 Schnittstellen

- **Upstream Phase:** Requirements (ATC-STD-V2S-004) und Spezifikation (ATC-STD-V2S-006).
- **Downstream Phase:** Architecture (ATC-STD-V2S-007), Development (ATC-STD-V2S-009), Testing (ATC-STD-BUG-001) und Release (ATC-STD-203).
- **Andock-Standards:**
  - ATC-STD-V2S-000 (Lifecycle Master)
  - ATC-STD-202 (Requirements Management)
  - ATC-STD-BUG-001 (Finding-Klassifizierung)
  - ATC-STD-AUDIT-001 (Audit-Standards)

## Metriken & Akzeptanzkriterien

- **M1:** 100% Traceability-Abdeckung für alle im Release enthaltenen Requirements.
- **M2:** 0 Orphaned-Code-Module im Hauptzweig (Main/Master Branch).
- **M3:** 100% aller Tests sind eindeutigen REQ-IDs oder SPEC-IDs zugeordnet.
- **M4:** Automatische Validierbarkeit der Matrix durch CI/CD ohne Fehler.

## Compliance & Verifikation

Die Compliance wird durch automatische Traceability-Checks in CI/CD-Pipelines sowie
durch `atc_std_validator.py` geprüft. Lücken in der Traceability-Kette führen zu
Findings der Kategorie F-NNN gemäß ATC-STD-BUG-001 und blockieren automatisiert den
Build-Status.

## Security Considerations

Die Traceability-Matrix dient als Nachweis, dass alle geforderten Sicherheitskontrollen
(NFR-SEC) tatsächlich in Code umgesetzt und durch Security-Tests (z.B. SAST, DAST, Audit)
verifiziert wurden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000; **INFORMATIVE:** SCR-0042, ATC-STD-V2S-004, ATC-STD-BUG-001, ATC-STD-AUDIT-001

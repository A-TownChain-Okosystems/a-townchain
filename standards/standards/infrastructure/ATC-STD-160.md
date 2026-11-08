---
standard:
  id: ATC-STD-160
  title: "CI/CD Standard"
  version: "1.2.0"
  status: approved
  category: infrastructure
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
  related_standards: []
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-160 — CI/CD (v1.2.0, APPROVED)

> **Status:** APPROVED (v1.2.0, §30-eingefroren) — Standard aus Katalog-Slot der Familie
> CI/CD & DevOps (FAM-09); §9-FREIGEGEBEN 08.09.2026, 02:15 UTC+2 — APPROVED, normativ,
> §30-eingefroren. Struktur-Elaboration v1.1.0 via SCR-0031; Slot-Fertigbau v1.2.0 via
> SCR-0034 (08.09.2026, 03:15 UTC+2): slot-spezifische Pruefkriterien (§6) mit eigener REQ-Menge
> je Gegenstand. Engineering-Bindung (Code/Tests) entsteht bei Slot-Aktivierung
> via SCR/MINOR (ehrlich dokumentiert).

## Abstract

ATC-STD-160 (CI/CD) ist der Standard für den gleichnamigen Katalog-Slot der Familie
**CI/CD & DevOps** (FAM-09, Range ATC-STD-160..174) im ATC Enterprise Standards Framework. Er
definiert den Gegenstand, seine Verortung im Ökosystem, die familienweiten
Kernregeln, die slot-spezifischen Prüfkriterien (§6) mit je-Kriterium-Nachweis,
Compliance- und Verifikationspflichten sowie Security-Betrachtungen. Der Standard
ist APPROVED, normativ in Kraft und §30-eingefroren (Owner-§9-Freigabe 08.09.2026,
02:15 UTC+2, SCR-0030-Batch); Elaboration SCR-0031 (v1.1.0) und Slot-Fertigbau
SCR-0034 (v1.2.0) sind additive MINOR-Updates (ATC-STD-UPDATE-001 UPD-G03).

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Der durch den Slot-Titel (CI/CD) bezeichnete Gegenstandsbereich im
Zuständigkeitsfeld der Familie CI/CD & DevOps. **Gilt nicht:** Bereiche, die durch
fachlich konkretere Standards derselben Familie verbindlich geregelt sind
(Subsidiarität: der konkrete Standard geht vor).

## §1 Gegenstand & Ökosystem-Verortung

Dieser Standard adressiert **CI/CD** im Katalog-Slot ATC-STD-160 der Familie CI/CD & DevOps
(FAM-09); die Zuordnung folgt registry/framework.yaml (S-21) und darf nur via SCR
geändert werden.

CI/CD-Schicht: governance-ci-Workflows (Naming/Governance), Dependabot in Manifest-Repos, Agent-Token-Scope-Restriktion (GH013).

**Katalog-Referenz:** governance-ci + validator-Workflows existieren als Bestandspraxis

## §2 Kernregeln (familienweit, elaboriert)

1. **KR-1:** Jeder Commit MUSS die CI-Gates passieren; rote Haupt-Pipelines blockieren.
2. **KR-2:** Workflow-Dateien DÜRFEN nur vom Owner geändert werden (Token ohne workflow-Scope — GH013).
3. **KR-3:** Abhängigkeits-Alerts MÜSSEN aktiviert sein (Dependabot/CodeQL-Rollout gemanagt).
4. **KR-4:** CI-Meldungen MÜSSEN eindeutig benannte Checks liefern (S-01..S-26).
5. **KR-5:** CI-Failures MÜSSEN vor dem nächsten Merge behoben oder dokumentiert blockiert werden.
6. **KR-6:** Selbst-Änderungen an Gates durch Agenten sind VERBOTEN (keine Selbstanerkennung).

## §3 Schnittstellen & Kopplungen

- **Registry-Kopplung:** standards.yaml/versions.yaml (Version/Status S-14/S-19),
  Katalog-Slot in registry/framework.yaml.
- **Governance-Kette:** SCR → VERSION → UPDATE (UPD-G03 MINOR) → COMPAT (bei MAJOR)
  → AUDIT; Findings via registry/findings.yaml (F-NNN).
- **Nachbarfamilien:** Familie CI/CD & DevOps — Subsidiarität: konkretere Standards gehen vor.
- **Agenten-Bindung:** Vollmandat via .github/ai/agent.yaml; Umsetzungspflicht nach
  AGENT_MANIFEST.

## §4 Metriken & Akzeptanzkriterien

- **M1:** Pipeline-Erfolgsquote
- **M2:** 0 unautorisierte Workflow-Änderungen
- **M3:** Alert-Bearbeitungszeit

- **M4:** Slot-Fertigbau: 6 verbindliche Prüfkriterien (§6) mit Nachweisangabe
  deklariert; Abdeckung nachzuweisen via AUD-Record bei Slot-Aktivierung.

Akzeptanz gilt als nachgewiesen, wenn die genannten Kriterien in einem AUD-Record
oder Validator-Lauf dokumentiert sind; fehlende Nachweise werden als Findings
geführt und nach ATC-STD-BUG-005 (RCA) bearbeitet.

## §5 Compliance & Verifikation

Compliance wird über die Gesamt-Validierung (CI, S-01..S-25) je Registry-Eintrag
geprüft: Metadaten-Vollständigkeit, Naming, Status-/Version-Konsistenz und
Registry-Konsistenz. Abweichungen werden als Findings (F-NNN) geführt und nach
ATC-STD-BUG-005 (RCA) bearbeitet.

## §6 Slot-Spezifikation CI/CD — verbindliche Prüfkriterien

Jedes Kriterium ist normativ (MUSS). Nachweis je Kriterium: Konzept-/Design-Dokument
plus AUD-Record, oder Validator-/Testlauf — je nach Art des Kriteriums; bei
Slot-Aktivierung wird der Nachweis je Kriterium einzeln erbracht.

- **P1** (MUSS): Pipeline-Stufen und Gate-Definitionen — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P2** (MUSS): Blockaderegeln (rot blockiert) — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung
- **P3** (MUSS): Zugriffsschutz fuer Gate-Aenderungen (GH013) — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P4** (MUSS): Meldungs- und Evidence-Qualitaet — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung
- **P5** (MUSS): Selbstanerkennungsverbot fuer Agenten — Nachweis: Design-/Konzeptdokument + AUD-Record
- **P6** (MUSS): Familienkonformitaet: CI/CD & DevOps-Kernregeln KR-1..KR-6 eingehalten — Nachweis: Validator-/Testlauf bzw. dokumentierte Prüfung

## Requirements (normativ)

- **REQ-STD-001** (§1): Gegenstand eindeutig definiert und im Katalog verortet.
- **REQ-STD-002** (§2): Fachliche Regeln als deklarierte, verifizierbare REQ-IDs.
- **REQ-STD-003** (§2): Compliance nachweisbar über Validator-Gates oder Prüfung.
- **REQ-STD-004** (§2): Änderungen ausschließlich über die Change-Control-Kette.
- **REQ-STD-005** (§2): Sicherheitsaspekte dokumentiert (Security Considerations).
- **REQ-STD-006**: Ökosystem-Verortung nachgewiesen (§1).
- **REQ-STD-007**: Familien-Kernregeln KR-1..KR-6 eingehalten und verifizierbar (§2).
- **REQ-STD-008**: Schnittstellen zu Registry/Governance-Kette/Agenten gebunden (§3).
- **REQ-STD-009**: Metriken M-1..M-4 definiert, Nachweis via AUD-Record (§4).
- **REQ-STD-010**: Familienspezifische Security-Bedrohungen katalogisiert (§5).
- **REQ-STD-011** (§6/P1): Pipeline-Stufen und Gate-Definitionen
- **REQ-STD-012** (§6/P2): Blockaderegeln (rot blockiert)
- **REQ-STD-013** (§6/P3): Zugriffsschutz fuer Gate-Aenderungen (GH013)
- **REQ-STD-014** (§6/P4): Meldungs- und Evidence-Qualitaet
- **REQ-STD-015** (§6/P5): Selbstanerkennungsverbot fuer Agenten
- **REQ-STD-016** (§6/P6): Familienkonformitaet: CI/CD & DevOps-Kernregeln KR-1..KR-6 eingehalten

## Security Considerations

Selbstanerkennung: Agenten schwächen Gates, die sie selbst prüfen; unbehandelte Abhängigkeits-Alerts werden zur Schwachstelle.

Ehrlichkeitsregel: Sicherheitszustände MÜSSEN ehrlich benannt sein
(ACTIVE/PARTIAL/PLANNED); erfundene Sicherheitszusagen sind verboten (vgl.
ATC-STD-PROTOCOL-003).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.2.0** (2026-09-08): MINOR via SCR-0034 — Slot-Fertigbau: slot-spezifische
  Prüfkriterien (§6, 6 Kriterien mit je-Kriterium-Nachweis), REQ-STD-006..016
  slot-spezifisch, M4-Abdeckungsmetrik ergänzt. Additiv, abwärtskompatibel
  (ATC-STD-UPDATE-001 UPD-G03).
- **1.1.0** (2026-09-08): MINOR via SCR-0031 — Struktur-Elaboration: familien-spezifische
  Kernregeln (§2), Ökosystem-Verortung (§1), Schnittstellen (§3), Metriken (§4)
  und Security-Bedrohungen; REQ-STD-006..010 additiv.
- **1.0.0** (2026-09-08): Initial Release — Grundgerüst-Standard aus
  Katalog-Slot ATC-STD-160 (Familie CI/CD & DevOps, FAM-09) via SCR-0030; §9-FREIGEGEBEN
  08.09.2026, 02:15 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- ATC-STD-000 (Standards Governance & Specification), ATC-STD-FRAMEWORK-001
- ATC-STD-UPDATE-001/CHANGE-001 (Change-Control), ATC-STD-BUG-005 (RCA)
- registry/framework.yaml (FAM-09), registry/standards.yaml + versions.yaml

*ATC-STD-160 v1.2.0 · Slot-Fertigbau via SCR-0034 · Aurora (Superagent) · 08.09.2026*

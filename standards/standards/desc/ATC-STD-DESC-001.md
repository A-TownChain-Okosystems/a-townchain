---
standard:
  id: ATC-STD-DESC-001
  title: "ATC-STD-DESC-001 — Standard zur Beschreibung von Standards (Standard Description Standard)"
  version: "1.0.0"
  status: approved
  category: desc
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  effective_date: "2026-09-07"
  review_date: ""
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
  related_standards:
    - ATC-STD-MD-001
    - ATC-STD-README-001
  requirements:
    - REQ-DESC-001
    - REQ-DESC-002
    - REQ-DESC-003
    - REQ-DESC-004
    - REQ-DESC-005
    - REQ-DESC-006
    - REQ-DESC-007
    - REQ-DESC-008
    - REQ-DESC-009
    - REQ-DESC-010
    - REQ-DESC-011
    - REQ-DESC-012
    - REQ-DESC-013
    - REQ-DESC-014
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-DESC-001 — Standard zur Beschreibung von Standards (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-§9-Freigabe 07.09.2026, 21:57 UTC+2 (Sammelfreigabe
> Builder-Chat); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-DESC-VERSION-v1.0.0.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** Standard Description Standards (ATC-STD-DESC-001..999) — Kategorie `desc`, SCR-0008.

> Dieser Standard definiert die verbindliche Struktur, Metadaten, Lifecycle-Modelle und Prüfmechanismen für die Beschreibung von ATC-Standards. Er operationalisiert die Verfassung ATC-STD-000 (§7 Naming, §8 Metadaten, §9 Struktur, §10 Lifecycle, §13 Versionierung, §20 Änderungsmanagement) zu einer vollständigen Beschreibungsnorm und erweitert sie um Documentation-Layer, Ausnahmeverfahren und Quality Gates.

## 1. Zweck (Purpose)

Ziel dieses Standards:

- einheitliche Dokumentationsstruktur für alle ATC-Standards
- eindeutige Verantwortlichkeiten
- maschinenlesbare Metadaten
- nachvollziehbare Änderungen
- Auditierbarkeit
- Vermeidung widersprüchlicher Standards
- eindeutige Zuordnung von Anforderungen und Prüfungen

Der Zweck darf keine technischen Detailanforderungen enthalten.

## 2. Geltungsbereich (Scope)

**Gilt:**

- ATC-Standards in `atc-standards/` (governance/, standards/)
- ATC-Repositories (Dokumentations- und Standards-Referenzen)
- Softwareprojekte des Ökosystems
- Smart Contracts (Standards-Referenzierung)
- KI-Agenten (Standards-Verarbeitung)
- Blockchain-Protokolle
- Infrastruktur-Standards
- Governance-Prozesse

**Nicht im Geltungsbereich (Nicht-Gilt):**

- externe Projekte ohne ATC-Verantwortung
- private Entwicklerprojekte
- Legacy-Familien (`atc/`, `ats/`) — diese bleiben über ihre Aggregate-Einträge in `legacy_series` referenziert

## 3. Begriffe und Definitionen

| Begriff | Definition |
|---|---|
| Standard | Verbindliche technische oder organisatorische Regel mit Standard-ID, Version und Status |
| Requirement | Konkrete überprüfbare Anforderung (REQ-<DOM>-NNN) |
| Finding | Festgestellte Abweichung (F-NNN) |
| Change Request | Antrag auf Änderung (SCR-NNNN) |
| Compliance | Nachweis der Einhaltung durch definierte Prüfkriterien (COM-<DOM>-NNN) |
| Owner | Verantwortliche Stelle (fachliche und normative Verantwortung) |
| Auditor | Stelle, die die Einhaltung prüft (AUD-NNN-Records) |
| Quality Gate | Definiertes Prüfkriterium, das vor Statuswechsel erfüllt sein MUSS |

## 4. Normative Anforderungen

Die normativen Begriffe folgen RFC-2119-Äquivalenten in deutscher Form:

| Begriff | Bedeutung |
|---|---|
| MUSS | verpflichtend |
| DARF NICHT | verboten |
| SOLL | empfohlen |
| SOLL NICHT | sollte vermieden werden |
| KANN | optional |

### REQ-DESC-001 — Pflichtstruktur

id: REQ-DESC-001

Jeder ATC-Standard MUSS die Pflichtstruktur gemäß Abschnitt 5 vollständig enthalten.

### REQ-DESC-002 — Metadaten-Header

id: REQ-DESC-002

Jeder ATC-Standard MUSS mit dem standardisierten Metadaten-Header (Abschnitt 6) beginnen. Der Header MUSS maschinenlesbar sein.

### REQ-DESC-003 — Eindeutige Standard-ID

id: REQ-DESC-003

Jeder ATC-Standard MUSS eine eindeutige Standard-ID gemäß ATC-STD-000 §7 besitzen. Die ID DARF NICHT ohne SCR geändert werden.

### REQ-DESC-004 — Lifecycle-Status

id: REQ-DESC-004

Jeder ATC-Standard MUSS einen eindeutigen Lebenszyklusstatus besitzen (Abschnitt 7). Nur ACTIVE-äquivalente Standards (Registry-Status `approved`/`stable` mit `normative: true`) DÜRFEN als verbindlich referenziert werden.

### REQ-DESC-005 — Zweck

id: REQ-DESC-005

Jeder Standard MUSS einen Zweck-Abschnitt besitzen, der beschreibt, warum der Standard existiert. Der Zweck DARF KEINE technischen Detailanforderungen enthalten.

### REQ-DESC-006 — Geltungsbereich

id: REQ-DESC-006

Jeder Standard MUSS eindeutig festlegen, wo er gilt und wo er NICHT gilt (Gilt/Nicht-Gilt).

### REQ-DESC-007 — Begriffe

id: REQ-DESC-007

Zentrale Begriffe MÜSSEN normiert werden. Bei Domain-spezifischen Begriffen MUSS der Standard sie definieren, bevor er sie verwendet.

### REQ-DESC-008 — Nummerierte Anforderungen

id: REQ-DESC-008

Normative Anforderungen MÜSSEN mit REQ-IDs gemäß dem Schema `REQ-<DOM>-NNN` nummeriert werden. Jede Anforderung MUSS überprüfbar sein.

### REQ-DESC-009 — Rollen

id: REQ-DESC-009

Jeder Standard MUSS festlegen, wer verantwortlich ist (Owner, Maintainer, Reviewer, Approver, Auditor).

### REQ-DESC-010 — Abhängigkeiten

id: REQ-DESC-010

Jeder Standard MUSS seine Abhängigkeiten in `registry/dependencies.yaml` deklarieren. Der Graph MUSS azyklisch sein (DAG). Standards DÜRFEN NICHT isoliert betrachtet werden.

### REQ-DESC-011 — Compliance-Kriterien

id: REQ-DESC-011

Jeder Standard MUSS definierte Prüfkriterien (COM-<DOM>-NNN) besitzen, die von der CI automatisch prüfbar sind.

### REQ-DESC-012 — Ausnahmeverfahren

id: REQ-DESC-012

Ausnahmen MÜSSEN explizit dokumentiert werden (Abschnitt 11). Jede Ausnahme MUSS eine EXC-ID, einen Grund, ein Risiko, einen Genehmiger und ein Ablaufdatum besitzen.

### REQ-DESC-013 — Versionierung

id: REQ-DESC-013

ATC-Standards MÜSSEN Semantic Versioning (MAJOR.MINOR.PATCH) verwenden. Breaking Changes erfordern MAJOR-Inkrement.

### REQ-DESC-014 — Änderungsmanagement

id: REQ-DESC-014

Keine direkte Änderung eines aktiven Standards ohne nachvollziehbaren Änderungsprozess gemäß ATC-STD-000 §20 (SCR-Verfahren). Jede Änderung MUSS in Changelog und versions.yaml nachvollziehbar sein.

## 5. Pflichtstruktur eines ATC-Standards

Jeder Standard MUSS mindestens folgende Bereiche enthalten:

1. Metadata (Header)
2. Zweck (Purpose)
3. Geltungsbereich (Scope: Gilt/Nicht-Gilt)
4. Begriffe und Definitionen
5. Normative Anforderungen (REQ-<DOM>-NNN)
6. Rollen und Verantwortlichkeiten
7. Prozesse / Verfahren
8. Schnittstellen und Abhängigkeiten
9. Compliance / Prüfungen (COM-<DOM>-NNN)
10. Ausnahmen
11. Versionierung
12. Änderungsmanagement
13. Referenzen (kategorisiert NORMATIVE/INFORMATIVE)
14. Changelog

## 6. Standard-Metadaten

Jeder Standard beginnt mit dem standardisierten Metadata-Block (YAML-Frontmatter). Dieser Standard erweitert ATC-STD-000 §8 um folgende optionale Felder: `effective_date`, `review_date`, `classification`, `language`, `superseded_by`, `dependencies`, `related_standards`, `requirements`, `changelog`.

Beispiel:

```yaml
standard:
  id: ATC-STD-XXX-001
  title: "Name des Standards"
  version: "1.0.0"
  status: draft
  category: "CATEGORY"
  owner: "ATC"
  authority: "ATC Standards Governance"
  effective_date: "YYYY-MM-DD"
  review_date: "YYYY-MM-DD"
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies: []
  related_standards: []
  requirements:
    - REQ-STD-001
```

### 6.1 Minimaler Standard-Header

Die kleinste zulässige Einheit:

```yaml
standard:
  id: ATC-STD-XXX-001
  title: "Standard Title"
  version: "1.0.0"
  status: draft
  owner: "ATC"
  category: "CATEGORY"
```

## 7. Status-Lifecycle

Ein Standard MUSS einen eindeutigen Lebenszyklusstatus besitzen. Das dokumentarische 7-Status-Modell dieses Standards wird wie folgt auf die Registry-Lifecycle-Status von ATC-STD-000 §10 abgebildet:

| Dokumentations-Status | Registry-Status | Bedeutung |
|---|---|---|
| DRAFT | `draft` | Entwurf |
| REVIEW | `review` | befindet sich in Prüfung |
| APPROVED | `approved` | offiziell genehmigt (§9-Freigabe erfolgt) |
| ACTIVE | `stable` + `normative: true` | verbindlich gültig |
| DEPRECATED | `deprecated` | nicht mehr empfohlen |
| RETIRED | `retired` | außer Kraft |
| REJECTED | *(kein Registry-Status)* | abgelehnt — Ablehnung wird im SCR dokumentiert |

**Regel:** Nur ACTIVE-äquivalente Standards (Registry: `approved`/`stable` mit `normative: true`) DÜRFEN als verbindliche Standards referenziert werden.

## 8. Rollen und Verantwortlichkeiten

```
Owner → Maintainer → Reviewer → Approver → Auditor
```

| Rolle | Verantwortung |
|---|---|
| Standard Owner | fachliche und normative Verantwortung |
| Maintainer | Pflege (Patches, Format, Links) |
| Reviewer | fachliche Prüfung |
| Approver | Genehmigung (Owner-§9-Freigabe) |
| Auditor | Compliance-Prüfung (AUD-Records) |
| Developer | Umsetzung in Code und Dokumentation |

## 9. Schnittstellen und Abhängigkeiten

Standards DÜRFEN NICHT isoliert betrachtet werden. Abhängigkeiten werden deklariert in:

- Datei-Header: `dependencies:` und `related_standards:`
- Registry: `registry/dependencies.yaml` (normativ, DAG-geprüft)

Beispiel:

```yaml
dependencies:
  - ATC-STD-000
related_standards:
  - ATC-STD-MD-001
```

## 10. Compliance / Prüfungen

Jeder Standard MUSS definierte Prüfkriterien besitzen. Damit wird aus Dokumentation ein prüfbarer Governance-Mechanismus.

### COM-DESC-001

id: COM-DESC-001

Der Standard MUSS auf das Vorhandensein einer gültigen Standard-ID geprüft werden (S-02).

### COM-DESC-002

id: COM-DESC-002

Die Version MUSS dem SemVer-Schema entsprechen (S-03).

### COM-DESC-003

id: COM-DESC-003

Alle MUSS-Anforderungen MÜSSEN überprüfbar sein (S-08: REQ-ID-Deklaration, eindeutig).

### COM-DESC-004

id: COM-DESC-004

Die Pflichtstruktur (Abschnitt 5) MUSS vollständig vorliegen (S-01, S-06, S-07).

### COM-DESC-005

id: COM-DESC-005

Der Abhängigkeitsgraph MUSS azyklisch sein (S-15).

### ATC Standard Quality Gate

Ein Standard DARF erst ACTIVE werden, wenn alle folgenden Kriterien erfüllt sind:

- [ ] eindeutige ID
- [ ] eindeutiger Titel
- [ ] Version vorhanden
- [ ] Status vorhanden
- [ ] Owner definiert
- [ ] Geltungsbereich definiert
- [ ] Zweck definiert
- [ ] Anforderungen nummeriert
- [ ] normative Sprache eindeutig
- [ ] Verantwortlichkeiten definiert
- [ ] Abhängigkeiten geprüft
- [ ] Compliance-Kriterien vorhanden
- [ ] Änderungsmanagement definiert
- [ ] Changelog vorhanden
- [ ] Referenzen geprüft
- [ ] Review abgeschlossen
- [ ] Approval erfolgt (Owner-§9)

## 11. Ausnahmen

Ausnahmen MÜSSEN explizit dokumentiert werden:

```yaml
exceptions:
  allowed: true
  approval_required: true
  documentation_required: true
  expiry_required: true
```

Eine Ausnahme benötigt: EXC-ID, Grund, betroffener Standard, betroffene Anforderung, Risiko, Genehmiger, Startdatum, Ablaufdatum.

## 12. Versionierung

ATC-Standards verwenden Semantic Versioning:

- **MAJOR** — Breaking Change (1.0.0 → 2.0.0)
- **MINOR** — Neue kompatible Funktion oder Anforderung (1.0.0 → 1.1.0)
- **PATCH** — Korrektur ohne fachliche Änderung (1.0.0 → 1.0.1)

## 13. Änderungsmanagement

Keine direkte Änderung eines aktiven Standards ohne nachvollziehbaren Änderungsprozess (ATC-STD-000 §20, CHANGE_CONTROL.md):

```
Change Request (SCR) → Impact Analysis → Review → Approval (Owner-§9)
→ Version Update → Changelog → Compliance Check → Release
```

## 14. Security Considerations

Standards MIT Security-Relevanz MÜSSEN einen Security-Considerations-Abschnitt enthalten (ATC-STD-000 §38). Standards OHNE Security-Relevanz SOLLEN dies explizit aussprechen. Ausnahmen von Sicherheitsanforderungen DARFEN NICHT ohne Owner-Genehmigung erteilt werden.

## 15. Maschinenlesbarkeit

Zwei Documentation-Ebenen:

```
Human Documentation          Machine Documentation
        │                           │
        ├── STANDARD.md             ├── standard.yaml
        └── README.md               ├── requirements.yaml
                                    └── compliance.yaml
```

Damit KÖNNEN KI-Agenten, CI/CD, Validatoren und Audit-Systeme Standards automatisch erkennen. Maschinenlesbare Header (YAML-Frontmatter) sind verpflichtend; maschinelle Sekundärdateien sind optional und SOLLten bei hohem Automatisierungsgrad ergänzt werden.

## 16. Referenzen (References)

### NORMATIVE Referenzen

- ATC-STD-000 — Standards Governance & Specification Standard (Verfassung; §7 Naming, §8 Metadaten, §9 Struktur, §10 Lifecycle, §13 Versionierung, §20 Änderungsmanagement)
- ATC-STD-201 — Repository Structure Standard (Ablageort standards/)

### INFORMATIVE Referenzen

- ATC-STD-MD-001 — ATC Markdown & Documentation Standard
- ATC-STD-README-001 — README als Einstiegsschnittstelle
- CHANGE_CONTROL.md — SCR-Verfahren (operationalisiert ATC-STD-000 §20)
- SemVer 2.0.0 — https://semver.org/

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Keine Zugangsdaten in Artefakten; Security-Review-Pflicht bei sicherheitsrelevanten Aenderungen (ATC-STD-203).

## Changelog

### 1.0.0 — 2026-09-07
- Initial Release (Owner-Entwurf Michael Wroblewski, harmonisiert mit ATC-STD-000 v1.2.0 durch Standards Governance)
- Owner-§9-Freigabe: APPROVED (21:57 UTC+2, Sammelfreigabe) — normativ ab sofort
- 14 normative Anforderungen (REQ-DESC-001..014), 5 Compliance-Gates (COM-DESC-001..005)
- 7-Status-Dokumentationsmodell auf Registry-Lifecycle abgebildet
- Quality-Gate-Checkliste (17 Kriterien) definiert

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203 · INFORMATIVE: Registry-SSOT standards.yaml

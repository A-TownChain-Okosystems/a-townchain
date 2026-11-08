---
standard:
  id: ATC-STD-README-001
  title: "ATC-STD-README-001 — README Standard"
  version: "1.0.0"
  status: approved
  category: readme
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf) / ATC-AI-ARCH-001 (Formalfassung)
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  applies_to: "Alle öffentlichen und internen ATC-Repositories"
  supersedes: []
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  superseded_by: null
---

# ATC-STD-README-001 — README Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 20:36 UTC+2 (ATC-STD-000 §9,
> „Freigeben"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-README-001.md.
> Immutabilität per §30 — Änderungen nur via SCR.

## Abstract

ATC-STD-README-001 definiert den README als standardisierte Einstiegsschnittstelle
jedes ATC-Repositories — nicht nur als Dokumentation. Der README MUSS das
Repository eindeutig identifizieren, Zweck, Status, Architektur, Installation,
Testing, Security, Standards-Compliance und Governance verbindlich beschreiben
und mit dem tatsächlichen Repository-Zustand übereinstimmen (CI-geprüft).
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle öffentlichen und internen Repositories der
A-TownChain-Okosystems (26 aktive Repos), insbesondere R2+-Repositories; den
README-Compliance-Validator und den Documentation Agent.
Nicht-Gilt: vollständige Projektdokumentation (liegt in docs/ bzw. Wiki —
siehe §11 Dokumentationshierarchie); die inhaltliche Roadmap-Pflege (liegt in
ROADMAP.md bzw. ATC Development Management).

## 1. Pflichtstruktur (REQ-README-001)

Jedes Repository MUSS — soweit zutreffend („soweit zutreffend" MUSS je
Repository-Typ begründet werden) — diese Abschnitts-Reihenfolge verwenden:

`# Repository Name` · `> Kurzbeschreibung` · `## Overview` · `## Purpose` ·
`## Status` · `## Architecture` · `## Features` · `## Repository Structure` ·
`## Requirements` · `## Installation` · `## Configuration` · `## Usage` ·
`## Development` · `## Testing` · `## Security` · `## Documentation` ·
`## Governance` · `## Standards & Compliance` · `## Roadmap` ·
`## Contributing` · `## License` · `## Maintainers` · `## Repository Metadata`

## 2. Header-Identifikation (REQ-README-002)

Der README-Anfang MUSS eindeutig identifizieren:

```markdown
# ATC <Project Name>

> <One-sentence description>

**Project:** <project-name>
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `<license>`
```

## 3. Status-Enum (REQ-README-003)

Der Status MUSS aus dieser verbindlichen Wertemenge stammen — kein freier
Status-Text ist zulässig:

`planning` · `prototype` · `development` · `alpha` · `beta` ·
`release-candidate` · `stable` · `deprecated` · `archived`

## 4. Zweck (REQ-README-004)

Jedes Repository MUSS erklären: Was ist das Projekt? Welches Problem löst es?
Welche Funktion besitzt es im ATC-Ökosystem? Welche anderen ATC-Komponenten
hängen davon ab?

```markdown
## Purpose

ATC Example provides the canonical implementation of <system/function>
within the A-TownChain ecosystem. It is responsible for: - ... - ... - ...
```

## 5. Architektur (REQ-README-005)

Bei technischen Repositories ist die Architektur-Sektion VERPFLICHTEND und
MUSS enthalten: Components, Data Flow, Dependencies-Tabelle
(`| Component | Purpose | Required |`). Bei komplexen Projekten SOLLTE
zusätzlich referenziert werden: Architecture Diagram, Dependency Graph
(ATC-STD-204 §7), Trust Boundaries, External Interfaces, Security Boundaries.

## 6. Repository Structure (REQ-README-006)

Die tatsächliche Struktur MUSS dokumentiert werden (Verzeichnisbaum als
Codeblock). Der README DARF KEINE veraltete Verzeichnisstruktur beschreiben —
das ist README-Gate-13 (CI).

## 7. Installation & Nutzung (REQ-README-007)

Installation MUSS reproduzierbar sein: Requirements (Versionen) + Setup-Schritte
(Kommando-Folge). Danach MUSS mindestens ein funktionierendes
Nutzungsbeispiel (`## Usage`) stehen.

## 8. Testing (REQ-README-008)

Die Tests MUSSEN beschrieben sein: Kommando zur kompletten Test-Suite +
erwartetes Ergebnis (PASS). Bei relevanten Projekten SOLLTE die Test-Pyramide
ausgewiesen werden: Unit, Integration, Security, E2E, Performance.

## 9. Security (REQ-README-009)

Jedes sicherheitsrelevante Repository MUSS den Security-Hinweis enthalten:
Security-Issues werden NICHT über GitHub Issues gemeldet, sondern über den
offiziellen ATC-Security-Reporting-Prozess. Sicherheitskritische Komponenten
SOLLLEN ihre Threat Model / Security Architecture (ATC-STD-203) referenzieren.

## 10. Standards & Compliance (REQ-README-010)

Jeder ATC-README MUSS die zutreffenden A-TownChain-Standards referenzieren
(Version + Compliance-Status je Zeile) — damit ist der README selbst
Bestandteil des ATC Standards Governance Systems:

```markdown
## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.2.0 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-201 | 1.0.0 | ✅ |
```

## 11. Dokumentationshierarchie (REQ-README-011)

Der README DARF NICHT zum zentralen Speicher für alles werden.
Regel: README = Einstiegspunkt (Identität, Zweck, Installation, wichtigste
Links). Vollständige Dokumentation liegt in docs/ bzw. Wiki. Der README MUSS
auf die vertiefende Dokumentation verlinken.

## 12. Roadmap (REQ-README-012)

Der README DARF KEINE frei erfundene Roadmap enthalten. Stattdessen MUSS er
auf die kanonischen Quellen verlinken (`ROADMAP.md`, ATC Development
Management, GitHub Issues/Projects). Die README-Roadmap MUSS mit dem
kanonischen Entwicklungsstand übereinstimmen.

## 13. Governance & Maintainer (REQ-README-013)

Der README MUSS Governance (A-TownChain Enterprise Governance Framework;
Review-/Approval-Pflicht für Architektur-, API-, Standards-, Security- und
Consensus-kritische Änderungen) und Maintainer-Sektion (Organisation,
Repository Ownership) enthalten.

## 14. Maschinenlesbare Metadaten (REQ-README-014)

Jeder README MUSS einen standardisierten Metadaten-Block enthalten
(HTML-Kommentar oder YAML-Abschlussblock):

```yaml
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-XXX
  name: example
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
governance:
  security_class: ...
  criticality: ...
```

Zielzustand (SOLLTE): Metadaten werden aus der zentralen Repository-Registry
(`repositories.yaml`, ATC-ENT-009) generiert — nicht manuell gepflegt.

## 15. Empfohlene Dateistruktur (REQ-README-015)

Die Repo-Wurzel SOLLTE dieser Struktur folgen:

```text
README.md  LICENSE  SECURITY.md  CONTRIBUTING.md  CODE_OF_CONDUCT.md
CHANGELOG.md  ROADMAP.md  ARCHITECTURE.md  docs/  tests/  .github/
```

## 16. Quality Gates (normativ)

Ein Repository gilt erst als README-konform, wenn ALLE zutreffenden Gates
bestanden sind. Gate-13 MUSS ein automatisiertes CI-Gate sein:

| Gate | Prüfung | REQ |
|---|---|---|
| README-01 | Repository eindeutig identifiziert | REQ-README-002 |
| README-02 | Zweck beschrieben | REQ-README-004 |
| README-03 | Status vorhanden (Enum §3) | REQ-README-003 |
| README-04 | Version vorhanden | REQ-README-002 |
| README-05 | Architektur dokumentiert | REQ-README-005 |
| README-06 | Installation reproduzierbar | REQ-README-007 |
| README-07 | Nutzung dokumentiert | REQ-README-007 |
| README-08 | Tests beschrieben | REQ-README-008 |
| README-09 | Security-Hinweise vorhanden | REQ-README-009 |
| README-10 | Standards referenziert | REQ-README-010 |
| README-11 | Roadmap verknüpft | REQ-README-012 |
| README-12 | Maintainer/Governance definiert | REQ-README-013 |
| README-13 | README entspricht tatsächlichem Repository-Zustand | REQ-README-006 |

## 17. Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-README-001 | Pflichtstruktur & Reihenfolge | MUSS (soweit zutreffend) |
| REQ-README-002 | Header-Identifikation (Projekt/Org/Status/Version/Lizenz) | MUSS |
| REQ-README-003 | Status nur aus Enum (9 Werte) | MUSS |
| REQ-README-004 | Zweck-Sektion (4 Fragen) | MUSS |
| REQ-README-005 | Architektur-Sektion für technische Repos | MUSS |
| REQ-README-006 | Repository-Struktur aktuell (kein Veraltungs-Dokumentiertes) | MUSS |
| REQ-README-007 | Installation reproduzierbar + Nutzungsbeispiel | MUSS |
| REQ-README-008 | Test-Kommando + erwartetes Ergebnis | MUSS |
| REQ-README-009 | Security-Reporting-Hinweis | MUSS (sicherheitsrelevant) |
| REQ-README-010 | Standards-Compliance-Tabelle | MUSS |
| REQ-README-011 | README = Einstiegspunkt, Links auf docs/wiki | MUSS |
| REQ-README-012 | Roadmap nur kanonisch verlinkt | MUSS |
| REQ-README-013 | Governance- und Maintainer-Sektion | MUSS |
| REQ-README-014 | Maschinenlesbarer Metadaten-Block | MUSS |
| REQ-README-015 | Empfohlene Dateistruktur | SOLLTE |

## 18. Compliance

Die Einhaltung wird durch den README-Compliance-Validator
(`tools/atc-readme-validator/check_readme.py`) geprüft: Gates README-01..13.
README-13 ist ein automatisiertes CI-Gate und verhindert, dass Code und
Dokumentation auseinanderlaufen. Verstöße werden als Finding nach
ATC-STD-BUG-001 (Severity nach BUG-002) dokumentiert und MUSSen behoben
werden. Der Documentation Agent kann die Prüfung je Push/PR auslösen.

## 19. Security Considerations

Der Standard erzwingt einen Security-Reporting-Hinweis je README (§9) und
damit eine einheitliche, nicht-öffentliche Schwachstellen-Meldungskette
(ATC-STD-203). Die Metadaten (§14) DÜRFEN KEINE Secrets, Tokens oder
internen URLs enthalten. Klassifikation (`security_class`) folgt
ATC-STD-203 und repositories.yaml (ATC-ENT-009).

## 20. Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Entwurf Michael, Formalfassung ATC-AI-ARCH-001) — Pflichtstruktur, Status-Enum, Gates README-01..13, Metadaten-Block |

## 21. References

- **NORMATIVE:** ATC-STD-000 (Verfassung, §9 Struktur), ATC-STD-201
  (Repository), ATC-STD-202 (Naming), ATC-STD-203 (Security),
  ATC-STD-204 (Dependencies/Interfaces), ATC-ENT-009 (Repository Governance)
- **INFORMATIVE:** RFC 2119, README-Compliance-Validator
  (tools/atc-readme-validator/check_readme.py), Dokumentationshierarchie
  (§11), ATC Development Management (kanonische Roadmap)

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Keine Zugangsdaten in Artefakten; Security-Review-Pflicht bei sicherheitsrelevanten Aenderungen (ATC-STD-203).

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203 · INFORMATIVE: Registry-SSOT standards.yaml

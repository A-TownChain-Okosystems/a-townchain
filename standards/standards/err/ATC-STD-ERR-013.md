---
standard:
  id: ATC-STD-ERR-013
  title: "Knowledge Capture"
  version: "1.0.0"
  status: approved
  category: err
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Gesamtes A-TownChain-Oekosystem: Repositories, Software, Smart Contracts, Doku, Standards, APIs, KI-Agenten, Infrastruktur, Prozesse"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-ERR-013 — Knowledge Capture (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P1 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Fehlerwissen dauerhaft dokumentieren: Jeder wichtige Fehler erhaelt einen
Error Knowledge Record.

## Kernregeln — Record-Format (YAML, SSOT: docs/error-records/)

```yaml
error_id: ATC-ERR-0001
title: <Fehlername>
severity: P0
category: [software, documentation]
first_detected: <date>
affected_repositories: [repository-a, repository-b]
affected_components: [component-a, component-b]
root_cause:
  type: <root-cause-category>
  description: <description>
error_pattern:
  description: <general-pattern>
  pattern_ref: ATC-ERR-PATTERN-NNN
propagation_scan:
  performed: true
  repositories_checked: 12
  occurrences_found: 4
fix: {implemented: true}
regression_test: {required: true, implemented: true}
preventive_control: {required: true, implemented: true}
documentation_updated: true
post_fix_scan: {performed: true}
status: CLOSED
```

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-130 | Knowledge Record im YAML-Format je wichtigem Fehler (P0/P1 zwingend) |
| REQ-ER-131 | Ablage im Docs-Hub (docs/error-records/), Verweis vom BUG-002-Record |

## Implementierungsstatus
SPECIFIED — Umsetzung: Verzeichnis docs/error-records/ im Docs-Hub anlegen;
error_id-Serie fortlaufend.

## Security Considerations

Knowledge-Records unterliegen der Sicherheitsregel: keine Klartext-Zugangsdaten, nur $ENV-Platzhalter; Ablage revisionssicher (git-getrackt).

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-ERR-000/-001/-010, ATC-STD-BUG-002 · INFORMATIVE: SCR-0044

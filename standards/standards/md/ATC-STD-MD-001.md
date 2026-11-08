---
standard:
  id: ATC-STD-MD-001
  title: "ATC-STD-MD-001 — ATC Markdown & Documentation Standard"
  version: "1.0.0"
  status: approved
  category: md
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf) / ATC-AI-ARCH-001 (Formalfassung)
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  applies_to: "Alle ATC-Repositories (Markdown-Dokumentation)"
  supersedes: []
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  superseded_by: null
---

# ATC-STD-MD-001 — ATC Markdown & Documentation Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Freigabe 07.09.2026, 21:05 UTC+2 (ATC-STD-000 §9,
> „Freigabe"); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-MD-001.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Normenhierarchie:** Für `README.md` gilt lex specialis **ATC-STD-README-001**
> (spezifisch) vor diesem Standard (allgemein). ATC-STD-000 §9 bleibt für
> Standard-Dokumente selbst maßgeblich (eigene Frontmatter-Pflicht).

## Abstract

ATC-STD-MD-001 definiert Markdown nicht nur als Formatierungsregeln, sondern
als verbindliches Documentation-Standard-Modell für jedes ATC-Repository:
Pflicht- und bedingte Dokumente, feste Dokumentationshierarchie,
UPPER_SNAKE_CASE-Dateinamen, Überschriften-Struktur, Dokument-Metadaten,
einheitliche Status- und Versionswerte, maschinenlesbare Tabellen,
deklarierte Codeblöcke, relative Links, ID-basierte Cross-References,
CHANGELOG-Format, maschinenlesbare STATUS.md, AI-Agenten-Instruktionen und
ein Documentation-Consistency-Gate gegen Documentation Drift.

## Scope

**Scope:** Gilt für alle Markdown-Dokumente in ATC-Repositories: Pflichtdateien
der Repository-Wurzel, Dokumentationshierarchie, Dateinamen, Struktur,
Metadaten, Status/Versionierung von Dokumenten, CHANGELOG-/STATUS-Format,
AI-Agenten-Instruktionen, Konsistenz-Gate.
**Nicht-Gilt:** (a) der Inhalt und die Struktur von `README.md` — dafür gilt
ATC-STD-README-001; (b) Standard-Dokumente unter `standards/` — für die gilt
die §9-Frontmatter-Struktur der Verfassung ATC-STD-000; (c) Versions-
Semantik von Software-Releases (dafür gilt ATC-STD-000 §13); (d) Legacy-
Serien (atc/, ats/) bis zu deren Migration.

## 1. Verbindliche Markdown-Dateien (REQ-MD-001)

| Datei | Zweck | Pflicht |
|---|---|---|
| README.md | Repository-Einstieg | MUSS (Details: ATC-STD-README-001) |
| LICENSE / LICENSE.md | Lizenzinformationen | MUSS |
| CONTRIBUTING.md | Beitragsregeln | MUSS |
| SECURITY.md | Security-Prozess | MUSS |
| CHANGELOG.md | Änderungen/Versionen | MUSS |
| STATUS.md | aktueller Projektstatus | MUSS |
| ROADMAP.md | geplante Entwicklung | MUSS bei aktiver Entwicklung |
| ARCHITECTURE.md | technische Architektur | MUSS bei Software |
| TODO.md | konkrete offene Aufgaben | SOLLTE bei Entwicklung |
| CODE_OF_CONDUCT.md | Verhalten/Community | MUSS bei öffentlichen Repos |
| GOVERNANCE.md | Entscheidungs-/Governance-Regeln | MUSS bei Governance-Repos |

## 2. Normenhierarchie & Dokumentationshierarchie (REQ-MD-002)

Die Markdown-Dokumentation MUSS einer festen Hierarchie folgen — damit wird
verhindert, dass wichtige Informationen ungeordnet über 20 verschiedene
.md-Dateien verteilt werden:

```text
Repository
├── README.md · STATUS.md · ROADMAP.md · TODO.md · CHANGELOG.md
├── ARCHITECTURE.md · GOVERNANCE.md · SECURITY.md · CONTRIBUTING.md
└── docs/
    ├── architecture/ · specifications/ · standards/ · guides/
    ├── api/ · adr/ · requirements/ · audits/ · reports/
```

Vertiefende Dokumentation liegt unter `docs/` (ATC-STD-README-001 §11:
README = Einstiegspunkt).

## 3. Dateinamen-Standard (REQ-MD-003)

Markdown-Dateien der Repository-Wurzel und von `docs/` MUSSEN
UPPER_SNAKE_CASE.md benannt sein (README.md, ARCHITECTURE.md,
API_REFERENCE.md, DEPLOYMENT_GUIDE.md …). Mischformen (readme.md,
Architecture.md, roadmap-final.md, TodoNew.md) sind unzulässig.
Repository-Wurzel-Pfade folgen zusätzlich ATC-STD-202.

## 4. Überschriften-Standard (REQ-MD-004)

Maximal sechs Ebenen (H1–H6). Pro Dokument SOLLTE genau ein `#` H1
(Dokumenttitel) existieren. Ebenen MÜSSEN ohne Lücken absteigen (kein
H2 → H4-Sprung).

## 5. Dokument-Metadaten (REQ-MD-005)

Technische ATC-Dokumente (ARCHITECTURE.md, GOVERNANCE.md, Dokumente unter
`docs/`) SOLLEN einen standardisierten YAML-Header besitzen, damit KI-Agenten,
Audits und Tools sie maschinell auswerten können:

```yaml
---
document_id: ATC-DOC-001
title: Repository Architecture
version: 1.0.0
status: draft
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---
```

Standard-Dokumente (`standards/`) verwenden stattdessen die §9-Frontmatter der
Verfassung. Zielzustand (SOLLTE): Generierung aus zentralen Registries.

## 6. Dokument-Statusstandard (REQ-MD-006)

Der **Dokument-Status** MUSS aus dieser Wertemenge stammen — keine freien
Werte, keine Mischformen (FINAL, Done, finished, complete, approved-final):

`draft` · `proposed` · `review` · `approved` · `active` · `deprecated` ·
`superseded` · `archived`

Abgrenzung: Der **Repository-Status** (README-Header) folgt ATC-STD-README-001
§3 (planning … archived) — unterschiedliche Achsen, keine Konfliktregel nötig.

## 7. Dokument-Versionsstandard (REQ-MD-007)

Standards und technische Dokumente MÜSSEN nach MAJOR.MINOR.PATCH versioniert
sein (ATC-STD-000 §13 SemVer): MAJOR = Breaking Change, MINOR = neue
Funktion/Anforderung, PATCH = Korrektur/Redaktion.

## 8. Tabellenstandard (REQ-MD-008)

Tabellen MÜSSEN einfach maschinenlesbar bleiben (Pipe-Format, Header-Zeile).
HTML-Tabellen in Markdown-Dokumenten SOLLLEN vermieden werden.

## 9. Codeblock-Standard (REQ-MD-009)

Bei jedem Fence MUSS die Programmiersprache deklariert werden
(rust, typescript, python, yaml, json, toml, bash, dockerfile, sql, solidity,
markdown, text …). Unbeschriftete Fences sind nur zulässig, wenn die
Datei nicht Code enthält.

## 10. Link-Standard (REQ-MD-010)

Interne Dokumente MÜSSEN bevorzugt relativ verlinkt werden
(`[Architecture](ARCHITECTURE.md)`) — damit bleiben Repositories portabel.

## 11. Cross-Reference-Standard (REQ-MD-011)

ATC-Dokumente SOLLEN IDs statt ausschließlich Dateinamen verwenden
(Related Standards: ATC-STD-000, ATC-STD-MD-001, ATC-STD-README-001), damit
KI-Agenten und Audit-Systeme Abhängigkeiten erkennen können.

## 12. CHANGELOG-Format (REQ-MD-012)

CHANGELOG.md MUSS Keep-a-Changelog-artig strukturiert sein:
`## [x.y.z] - JJJJ-MM-TT` mit `### Added / Changed / Fixed / Security`.

## 13. STATUS.md — maschinenlesbar (REQ-MD-013)

Jedes aktive Repository MUSS seinen Zustand maschinenlesbar als
Property-Value-Tabelle darstellen (Repository, Version, Status, Build, Tests,
Security, Documentation, Last Audit).

## 14. KI-Agenten-Kompatibilität (REQ-MD-014)

Jedes Repository MUSS einen Bereich „AI Agent Instructions" besitzen (in
AGENTS.md oder README), der mindestens definiert: Identity (maßgebliche
Standards), Entry Point (README → STATUS → ARCHITECTURE → ROADMAP → TODO),
Required Workflow (Status prüfen → Standards lesen → Architektur inspizieren
→ Aufgabe identifizieren → implementieren → Tests → Dokumentation →
Changelog → Konsistenz prüfen).

## 15. Documentation-Consistency-Gate (REQ-MD-015)

Vor einem Merge MUSS automatisch die Kette geprüft werden:
CODE → ARCHITECTURE.md → README.md → STATUS.md → ROADMAP.md → TODO.md →
CHANGELOG.md. Enthält der Code z. B. eine neue API, die README/ARCHITECTURE
noch nicht beschreiben, gilt `DOCUMENTATION_DRIFT = TRUE` — der Pull Request
darf dann NICHT automatisch als vollständig gelten. Prüfung durch
`tools/atc-md-validator/check_md.py` (Agenten-lokal) und nach
F-009/F-010-Behebung als CI-Gate.

## 16. Dokumenttyp-Klassifikation (REQ-MD-016)

ATC-Dokumente SOLLEN einen Typ-Präfix tragen:

STD · SPEC · ARCH · REQ · GUIDE · ADR · PLAN · REPORT · AUDIT
(Beispiele: ATC-SPEC-API-001, ATC-ARCH-CORE-001, ATC-ADR-001). Zielbild:
standardisiertes Dokumentationsmodell je Repository (§2 + docs/-Struktur).

## 17. Anforderungs-Verzeichnis

| REQ-ID | Anforderung | Verbindlichkeit |
|---|---|---|
| REQ-MD-001 | Pflicht-/bedingte Dokumente (§1) | MUSS/SOLLTE je Zeile |
| REQ-MD-002 | Dokumentationshierarchie (§2) | MUSS |
| REQ-MD-003 | UPPER_SNAKE_CASE-Dateinamen (§3) | MUSS |
| REQ-MD-004 | Überschriften ≤ 6 Ebenen, 1× H1 (§4) | MUSS / SOLLTE |
| REQ-MD-005 | Dokument-Metadaten-Header (§5) | SOLLTE |
| REQ-MD-006 | Dokument-Status-Enum (§6) | MUSS |
| REQ-MD-007 | SemVer für Dokumente (§7) | MUSS |
| REQ-MD-008 | Tabellen maschinenlesbar (§8) | MUSS/SOLLTE |
| REQ-MD-009 | Codeblöcke deklariert (§9) | MUSS |
| REQ-MD-010 | Relative Links (§10) | MUSS (bevorzugt) |
| REQ-MD-011 | ID-basierte Cross-References (§11) | SOLLTE |
| REQ-MD-012 | CHANGELOG-Format (§12) | MUSS |
| REQ-MD-013 | STATUS.md maschinenlesbar (§13) | MUSS |
| REQ-MD-014 | AI Agent Instructions (§14) | MUSS |
| REQ-MD-015 | Documentation-Consistency-Gate (§15) | MUSS |
| REQ-MD-016 | Dokumenttyp-Klassifikation (§16) | SOLLTE |

## 18. Compliance

Prüfung durch `tools/atc-md-validator/check_md.py` (Gates MD-01..MD-10):
Pflichtdateien, Hierarchie, Dateinamen, Überschriften, Status-Enum, SemVer,
Codeblöcke, CHANGELOG-/STATUS-Format, AI-Agenten-Bereich, Drift-Hinweise.
Verstöße werden als Finding nach ATC-STD-BUG-001 dokumentiert. Rollout auf
alle 26 Repos analog README-001 (Übergangsfrist 07.10.2026).

## 19. Security Considerations

Dokument-Metadaten (§5) DÜRFEN KEINE Secrets, Tokens oder internen URLs
enthalten. STATUS.md (§13) DARF keine Schwachstellen-Details veröffentlichen
(Verweis auf SECURITY.md / ATC-STD-203). Dokumentklassifikation (§16) MUSS
die Sicherheitsklassifikation von ATC-STD-203 nicht widersprechen.

## 20. Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Entwurf Michael, Formalfassung ATC-AI-ARCH-001) — Pflichtdateien, Hierarchie, Naming, Metadaten, Status/SemVer, Gates MD-01..10, AI-Agenten-Schnittstelle, Drift-Gate |

## 21. References

- **NORMATIVE:** ATC-STD-000 (Verfassung: §9, §13, §30), ATC-STD-201
  (Repository), ATC-STD-202 (Naming), ATC-STD-README-001 (README — lex
  specialis), ATC-STD-BUG-001..004 (Findings)
- **INFORMATIVE:** Keep-a-Changelog, CommonMark, RFC 2119,
  tools/atc-md-validator/check_md.py

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

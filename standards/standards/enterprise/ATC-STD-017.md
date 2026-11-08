---
standard:
  id: ATC-STD-017
  title: "Obsolete & Orphaned Artifact Management Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-10"
  updated: "2026-09-10"
  normative: true
  effective_date: "2026-09-10"
  review_date: "2027-09-10"
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-016
  related_standards:
    - ATC-STD-201
    - ATC-STD-REPO-AUDIT-001
    - ATC-STD-UPDATE-001
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-017 — Obsolete & Orphaned Artifact Management (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0, §30-eingefroren) — Owner-Entwurf + §9-FREIGEGEBEN via
> Owner-Direktive 10.09.2026, ~15:05 UTC+2. Priorität: P1. Kerngrundsatz:
> **»Nicht referenziert« ist ein Audit-Signal, kein Löschkriterium.**
> Änderungen ausschließlich via ATC-STD-UPDATE-001 (SCR/MINOR/MAJOR).

## Abstract

ATC-STD-017 definiert Identifikation, Klassifizierung, Migration, Archivierung und
Entfernung von Dateien, die veraltet, dupliziert, nicht mehr referenziert,
funktionslos oder keinem aktiven Standard/Prozess mehr zugeordnet sind. Es ist mehr
als Cleanup: ein Lifecycle-, Traceability- und Repository-Governance-Standard.
Partner: ATC-STD-016 (Inventur, »Welche Dateien existieren?«).

## Scope

**Gilt:** Der Gegenstandsbereich dieses Standards im ATC Enterprise Standards Framework
(FAM-01 Enterprise & Governance, Slot ATC-STD-017) — verbindlich für das Repository atc-standards
(Phase 1) und alle ATC-Repositories (Phase 2 gemäß §Roadmap im SCR-0087).

**Gilt nicht:** Automatisierte Löschung ohne Audit-Verfahren; inhaltliche Fachnormen
anderer Familien; externe Archivsysteme außerhalb der Git-Historie.

## §1 Zweck

Verhindern, dass alte Drafts, überholte Spezifikationen und nicht mehr verwendete
Dateien mit aktuellen Standards konkurrieren — bei vollständigem Erhalt der
historischen Nachvollziehbarkeit (Governance-Artefakte und Nachweise dürfen nie
versehentlich verschwinden).

## §2 Geltungsbereich

Gilt für: Markdown/Dokumentation, Standards und Spezifikationen, Schemas,
Konfigurationen, Quellcode, Tests, Scripts, CI/CD-Dateien, Assets, Beispiele,
Templates, Reports, generierte Artefakte, historische Drafts, Root-Level-Dateien
sowie Dateien in docs/, specs/, standards/, archive/ usw. (voller Scan-Scope;
Klassifizierungspflicht je Relevanzschwelle im Audit-Verfahren).

## §3 Klassifizierung (REQ-STD-001)

Jedes relevante Artefakt MUSS einer von neun Klassen zugeordnet sein:

| Klasse | Bedeutung | Aktion |
|---|---|---|
| ACTIVE | Aktuell benötigt und referenziert | Behalten |
| LEGACY | Alt, aber noch kompatibilitätsrelevant | Kennzeichnen + Migrationsplan (REQ-STD-002) |
| DEPRECATED | Nicht mehr empfohlen | Deprecation-Hinweis + Nachfolgeartefakt (REQ-STD-002) |
| ORPHANED | Keine gültige Zuordnung/Referenz | Prüfen (NICHT automatisch löschen, REQ-STD-012) |
| OBSOLETE | Inhalt fachlich überholt | Archivieren oder entfernen |
| DUPLICATE | Inhalt/Funktion redundant | Konsolidieren |
| GENERATED | Automatisch erzeugt | Quelle statt Output verwalten (REQ-STD-003) |
| ARCHIVED | Historisch bewusst erhalten | Nur noch Referenzzweck |
| UNKNOWN | Status nicht nachweisbar | Audit erforderlich |

## §4 Entscheidungslogik (REQ-STD-004)

Die Klassifizierung MUSS der verbindlichen Logik folgen:

```
Datei gefunden
     │  Ist sie aktiv referenziert?
     ├── JA  → ACTIVE
     └── NEIN → Gibt es einen Owner?
                ├── JA  → LEGACY/DEPRECATED (Inhalt gültig? → ansonsten OBSOLETE→REMOVE)
                └── NEIN → ORPHANED → Inhalt noch gültig?
                            ├── JA  → ARCHIVE
                            └── NEIN → OBSOLETE → REMOVE (nur mit §7-Löschschutz erfüllt)
```

## §5 Mindestzuordnung (REQ-STD-005)

Jede relevante Datei MUSS mindestens einem dieser Objekte zugeordnet werden können:
Repository, Standard, Specification, Requirement, Implementation, Test, ADR,
Governance Process, Build/CI Process, Documentation, Archive. Nachweisform ist das
Artefakt-Manifest gemäß ATC-STD-016 §4 (SSOT: registry/artifacts.yaml bzw.
.atc/artifacts.yaml).

## §6 Verwaist-Bestimmung (REQ-STD-006, REQ-STD-007)

»Verwaist« MUSS objektiv über ZWEI Dimensionen bestimmt werden:

1. **Referenzen**: interne Markdown-Links, relative Pfade, Standard-/Schema-/ADR-/
   Requirement-IDs, CI-Referenzen, Script-Referenzen, Build-Systeme, GitHub Actions,
   Package-/Module-Imports, Konfigurationsreferenzen.
2. **Semantische Zuordnung**: Governance-Relevanz unabhängig von Text-Referenzen
   (Beispiel: ein Standard ist über Registry, README, CI, Audit und Repository-
   Inventory getragen, auch wenn ihn keine Datei per Link nennt).

REQ-STD-007 (Kerngrundsatz): »Nicht referenziert« ist ein Audit-Signal, KEIN
Löschkriterium. Eine Datei kann technisch unreferenziert, aber governance-relevant sein.

## §7 Löschschutz (REQ-STD-008, REQ-STD-009)

REMOVE erfordert ALLE sechs Kriterien erfüllt:

1. ORPHANED
2. NOT REQUIRED
3. NOT GOVERNANCE-RELEVANT
4. NOT HISTORICALLY REQUIRED
5. NO SUCCESSOR DEPENDENCY
6. NO ACTIVE REFERENCE

Keine automatische Löschung aufgrund eines einzelnen Signals (REQ-STD-008).
REMOVE MUSS ferner garantieren (REQ-STD-009): Git-Historie bleibt erhalten,
Audit-Record (AUD) angelegt, optional CHANGELOG-Eintrag. Damit bleibt die
historische Nachvollziehbarkeit vollständig erhalten.

## §8 CI-Gate (REQ-STD-010)

Der Artefakt-Audit MUSS automatisiert werden (Tool `tools/artifacts/audit_artifacts.py`),
Prüfkatalog: Broken links, Missing references, Duplicate artifacts, Deprecated artifacts,
Orphaned files, Unowned files, Stale specifications, Superseded standards, Dead
configuration, Generated artifacts.

- **Stufe 1 (initial)**: CI warnt nur — z. B. `ORPHANED docs/old-roadmap.md`,
  `LEGACY specs/v0-spec.md`, `DUPLICATE standards/repo.md`,
  `BROKEN docs/index.md → missing file`.
- **Stufe 2 (nach Stabilisierung + Owner-Freigabe)**: P0 → CI FAIL, P1 → CI FAIL,
  P2 → WARNING, P3 → INFORMATIONAL.

## §9 Verknüpfung & Governance-Kette (REQ-STD-011)

```
ATC-STD-000 (Standards Governance)
     ├── ATC-STD-016 (Repository Artifact & File Inventory)
     ├── ATC-STD-017 (dieser Standard)
     ├── Repository-Audit-Standard (REPO-AUDIT-001)
     ├── Versioning Standard (UPDATE-001)
     └── Deprecation/Retirement (Teil von §3/§4 hier)
```

Verbindliche Prozesskette: DISCOVER → INVENTORY → CLASSIFY → TRACE → VALIDATE →
MIGRATE / ARCHIVE / REMOVE → VERIFY → REGISTRY UPDATE.

## §10 Freigabe

FREIGEGEBEN am 10.09.2026, ~15:05 UTC+2, durch Owner-Direktive (Alexander Wroblewski):
»Für atc-standards sollte veraltete und verwaiste Dateien als eigener verbindlicher
Standard behandelt werden« — Einstufung P1 per Owner-Vorgabe. Vollständiger
normativer Inhalt liegt als Owner-Entwurf vor und ist unverändert übernommen.

## §30 Freeze & Change-Control

Dieser Standard ist §30-eingefroren. Änderungen ausschließlich via ATC-STD-UPDATE-001:
PATCH (redaktionell), MINOR (z. B. neue Prüfkategorien, Schema-Erweiterung), MAJOR
(breaking, COMPAT-001-Gate erforderlich).

## Security Considerations

Der Löschschutz ist eine Integritäts-Anforderung: Versehentlich entfernte
Governance-Artefakte oder historische Nachweise wären nicht mehr auditierbar.
ORPHANED-Klassifizierung ohne Audit-Verfahren DARF NICHT zu REMOVE führen (§6/§7).

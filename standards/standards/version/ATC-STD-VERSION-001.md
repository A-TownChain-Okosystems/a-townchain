---
standard:
  id: ATC-STD-VERSION-001
  title: "ATC-STD-VERSION-001 — ATC Versioning Standard"
  version: "1.0.0"
  status: approved
  category: version
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
    - ATC-STD-DESC-001
    - ATC-STD-MD-001
    - ATC-STD-SC-001
  requirements:
    - REQ-VERSION-001
    - REQ-VERSION-002
    - REQ-VERSION-003
    - REQ-VERSION-004
    - REQ-VERSION-005
    - REQ-VERSION-006
    - REQ-VERSION-007
    - REQ-VERSION-008
    - REQ-VERSION-009
    - REQ-VERSION-010
    - REQ-VERSION-011
    - REQ-VERSION-012
    - REQ-VERSION-013
    - REQ-VERSION-014
    - REQ-VERSION-015
    - REQ-VERSION-016
    - REQ-VERSION-017
    - REQ-VERSION-018
    - REQ-VERSION-019
    - REQ-VERSION-020
    - REQ-VERSION-021
    - REQ-VERSION-022
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-VERSION-001 — ATC Versioning Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-§9-Freigabe 07.09.2026, 21:57 UTC+2 (Sammelfreigabe
> Builder-Chat); dokumentiert in approval/APPROVAL-DECISION-2026-09-07-DESC-VERSION-v1.0.0.md.
> Immutabilität per §30 — Änderungen nur via SCR.
> **Familie:** Versioning Standards (ATC-STD-VERSION-001..999) — Kategorie `version`, SCR-0009.

> Dieser Standard definiert eine einheitliche Versionierungsstrategie für das gesamte A-TownChain-Ökosystem: Software, Standards, APIs, Smart Contracts, Protokolle, Dokumentation und Releases werden eindeutig versioniert, nachvollziehbar geändert und auditierbar released.

## 1. Zweck (Purpose)

Ziele dieses Standards:

- eindeutige Identifikation von Entwicklungsständen
- nachvollziehbare Änderungen
- kontrollierte Breaking Changes
- reproduzierbare Releases
- Kompatibilitätsmanagement
- Auditierbarkeit
- automatisierbare Release-Prozesse
- eindeutige Zuordnung von Code, Dokumentation und Standards

## 2. Geltungsbereich (Scope)

**Gilt:**

- Alle ATC-Repositories
- Softwarekomponenten (Node, Wallet, Miner, SDK, APIs)
- ATC-Standards (alle Familien gemäß ATC-STD-000 §7)
- Protokolle (Protocol, Consensus, State, Network)
- APIs
- Smart Contracts
- Dokumentation (versionsabhängige Teile)
- KI-Agenten
- Releases und Builds

**Nicht im Geltungsbereich (Nicht-Gilt):**

- externe Projekte ohne ATC-Verantwortung
- Legacy-Familien (`atc/`, `ats/`) in ihrer bestehenden Form

## 3. Begriffe und Definitionen

| Begriff | Definition |
|---|---|
| Version | Eindeutige Kennung eines Entwicklungsstands (SemVer-kodiert) |
| Breaking Change | Inkompatible Änderung an Interface, Protokoll, Datenformat oder normativer Regel |
| Pre-Release | Entwicklungsstand vor der stabilen Veröffentlichung (alpha/beta/rc) |
| Release | Veröffentlichter, getaggter, CHANGELOG-dokumentierter Quellstand |
| Build | Konkreter Kompilationsstand aus einem Commit (Build-ID mit Commit-SHA) |
| Kanonische Versionsquelle | Die eine, verbindliche Quelle der Repository-Version |
| Golden Rule | Eine Version wird nur einmal veröffentlicht und ist eindeutig zugeordnet |

## 4. Normative Anforderungen

Normative Begriffe gemäß ATC-STD-DESC-001 (MUSS / DARF NICHT / SOLL / KANN).

### REQ-VERSION-001 — SemVer als Primärstandard

id: REQ-VERSION-001

Alle ATC-Komponenten MÜSSEN Semantic Versioning 2.0.0 (MAJOR.MINOR.PATCH) verwenden. Kanonische Registry- und Standard-Versionen MÜSSEN dem Kernmuster `X.Y.Z` (schema: `versionPatterns.semver`) entsprechen.

### REQ-VERSION-002 — MAJOR

id: REQ-VERSION-002

MAJOR MUSS erhöht werden bei inkompatiblen Änderungen: API, Protokoll, Datenformat, Smart-Contract-Interface, entfernte Funktion, geänderte Konsens-/Netzwerkregeln.

### REQ-VERSION-003 — MINOR

id: REQ-VERSION-003

MINOR MUSS erhöht werden bei rückwärtskompatiblen neuen Funktionen (neue API-Funktion, Feature, optionale Konfiguration, Plattform-Unterstützung, kompatible Protokollfunktion).

### REQ-VERSION-004 — PATCH

id: REQ-VERSION-004

PATCH MUSS erhöht werden bei rückwärtskompatiblen Korrekturen (Bugfix, Security Fix, Dokumentationskorrektur, Performance-Optimierung ohne API-Änderung, interne Implementierungsänderung).

### REQ-VERSION-005 — Pre-Release-Versionen

id: REQ-VERSION-005

Entwicklungsstände MÜSSEN über Pre-Release-Identifier gekennzeichnet werden (Reihenfolge: alpha → beta → rc → stable). Beispiel: `1.0.0-alpha.1 → 1.0.0-beta.1 → 1.0.0-rc.1 → 1.0.0`.

### REQ-VERSION-006 — Entwicklungsstatus

id: REQ-VERSION-006

Zusätzlich zur Version MUSS der Entwicklungsstatus geführt werden (DRAFT, ALPHA, BETA, RC, STABLE, DEPRECATED, EOL). Status und Version SIND unabhängig. Für ATC-Standards im engeren Sinne gilt der Lifecycle aus ATC-STD-000 §10; dieser Status gilt für Software-Releases.

### REQ-VERSION-007 — Git-Tags

id: REQ-VERSION-007

Ein offizielles Release MUSS durch einen unveränderlichen Git-Tag im Format `vMAJOR.MINOR.PATCH` identifizierbar sein (Pre-Releases: `v1.0.0-rc.1`).

### REQ-VERSION-008 — Kanonische Versionsquelle

id: REQ-VERSION-008

Jedes produktive Repository MUSS genau eine kanonische Versionsquelle besitzen (empfohlen: Datei `VERSION`; alternativ Build-System). Es DARF NICHT mehrere widersprüchliche kanonische Versionsquellen geben.

### REQ-VERSION-009 — API-Versionierung

id: REQ-VERSION-009

APIs MÜSSEN separat versioniert werden (`/api/v1/`); Breaking Changes erfordern eine neue API-Major-Version (`/api/v2/`). Eine API-Version DARF NICHT allein wegen eines PATCH-Releases geändert werden.

### REQ-VERSION-010 — Protokoll-Versionierung

id: REQ-VERSION-010

Blockchain-Protokolle MÜSSEN getrennt versioniert werden: `protocol_version`, `consensus_version`, `state_version`, `network_version` — unabhängig von der Software-Version.

### REQ-VERSION-011 — Smart-Contract-Versionierung

id: REQ-VERSION-011

Smart Contracts MÜSSEN mit `version` und `interface_version` versioniert werden. Bei bereits deployed Contracts MUSS die Deployment-/Address-Identität erhalten bleiben. Eine neue Contract-Version ist NICHT automatisch ein Upgrade desselben Contracts.

### REQ-VERSION-012 — KI-Agenten-Versionierung

id: REQ-VERSION-012

KI-Agenten MÜSSEN eindeutig versioniert werden (`agent.id`, `agent.version`, `role`, `protocol_version`, `capabilities_version`), damit Audits feststellen können, welcher Agent mit welchen Fähigkeiten welche Änderung erzeugt hat.

### REQ-VERSION-013 — Dokumentations-Versionierung

id: REQ-VERSION-013

Versionsabhängige Dokumentation MUSS versioniert werden (`docs/v1/`, `docs/v2/` oder `version-1/`-Struktur).

### REQ-VERSION-014 — CHANGELOG-Verknüpfung

id: REQ-VERSION-014

Jede veröffentlichte Version MUSS einen CHANGELOG-Eintrag besitzen, der mit dem Release-Tag übereinstimmt.

### REQ-VERSION-015 — Release-ID

id: REQ-VERSION-015

Jedes produktive Release MUSS eine eindeutige Release-ID besitzen (Schema `releaseId`): versionsbasiert `ATC-REL-X.Y.Z` oder datumsbasiert `ATC-REL-YYYYMMDD-NNN` (beide Formate zulässig, je Repository konsistent zu verwenden).

### REQ-VERSION-016 — Build-ID

id: REQ-VERSION-016

Ein Release KANN mehrere Builds besitzen. Ein Build MUSS per Build-ID (Format `ATC-BUILD-NNN`) und Commit-SHA eindeutig identifizierbar sein.

### REQ-VERSION-017 — Commit-Versionierung

id: REQ-VERSION-017

Commits ERHALTEN keine eigene Semantic Version. Die Version gehört zum Release; die Git-Historie bleibt sauber.

### REQ-VERSION-018 — Monorepo-Regel

id: REQ-VERSION-018

In Monorepos MUSS eine Variante dokumentiert sein: (A) gemeinsame Version (ATC Platform) oder (B) unabhängige Komponenten-Versionierung. Für das ATC-Ökosystem SOLL langfristig Variante B mit übergeordnetem ATC Platform Release verwendet werden.

### REQ-VERSION-019 — Versionskompatibilität

id: REQ-VERSION-019

Jede Komponente SOLL ihre Kompatibilitätsanforderungen deklarieren (`requires: node: ">=2.0.0 <3.0.0"`, `api: "^1.4.0"`), damit inkompatible Kombinationen automatisiert erkannt werden.

### REQ-VERSION-020 — Release Manifest

id: REQ-VERSION-020

Jedes produktive Release SOLL ein maschinenlesbares Manifest besitzen (release_id, version, status, repository, components, documentation, changelog, security) — für Audits, CI/CD, KI-Agenten und reproduzierbare Builds.

### REQ-VERSION-021 — Verbotene Praktiken

id: REQ-VERSION-021

Folgende Bezeichner DÜRFEN NICHT für formale Releases verwendet werden: `latest`, `stable`, `final`, `new`, `new2`, `release-final`, `release-final2`, `v1-final`, `v1-final-fixed`, sowie unvollständige Versionen (`1.0`, `1`, `Version 1`). Korrekt ist ausschließlich `1.0.0`.

### REQ-VERSION-022 — Golden Rule

id: REQ-VERSION-022

Eine Version DARF nur einmal veröffentlicht werden und MUSS eindeutig einem definierten Quellstand, CHANGELOG, Release und Build zugeordnet werden können.

## 5. Primärer Versionsstandard

ATC verwendet grundsätzlich Semantic Versioning 2.0.0:

```
1.4.7
│ │ │
│ │ └── PATCH  (rückwärtskompatible Korrekturen)
│ └──── MINOR  (rückwärtskompatible neue Funktionen)
└────── MAJOR  (inkompatible Änderungen)
```

### 5.1 MAJOR — Beispiele

- API inkompatibel geändert
- Protokoll inkompatibel geändert
- Datenformat inkompatibel geändert
- Smart-Contract-Interface inkompatibel geändert
- bestehende Funktion entfernt
- Konsens-/Netzwerkregeln inkompatibel geändert

### 5.2 MINOR — Beispiele

- neue API-Funktion
- neues Feature
- neue optionale Konfiguration
- neue unterstützte Plattform
- neue kompatible Protokollfunktion

### 5.3 PATCH — Beispiele

- Bugfix
- Security Fix
- Dokumentationskorrektur
- Performance-Optimierung ohne API-Änderung
- interne Implementierungsänderung

## 6. Pre-Release-Versionen

```
alpha → beta → rc → stable
```

Beispiel:

```
0.8.0-alpha.1 → 0.8.0-beta.1 → 0.8.0-rc.1 → 1.0.0
```

## 7. Entwicklungsstatus

| Status | Bedeutung |
|---|---|
| DRAFT | erster Entwurf |
| ALPHA | frühe Entwicklung |
| BETA | funktional, aber noch nicht final |
| RC | Release Candidate |
| STABLE | stabile Version |
| DEPRECATED | abgekündigt |
| EOL | End of Life |

Status und Version sind unabhängig. Beispiel: `version: 1.4.0`, `status: STABLE`.

Für ATC-Standards gilt der Lifecycle aus ATC-STD-000 §10 (idea → proposed → draft → review → candidate → approved → stable → deprecated → retired); dieser Status-Katalog gilt für Software-Releases und Release-Manifeste.

## 8. ATC Standard-Versionierung

ATC-Standards erhalten eine eigene Standard-ID und eine Semantic Version. Struktur gemäß ATC-STD-000 §7: `ATC-STD-[DOMAIN]-[NUMBER]` (z. B. `ATC-STD-README-001`, `ATC-STD-VERSION-001`).

## 9. Standardänderungen

Änderungen an Standards werden nach denselben Regeln bewertet:

| Stufe | Anlass | Beispiel |
|---|---|---|
| PATCH | keine normative Änderung (Tippfehler, Formatierung, Referenz-Korrektur) | 1.0.0 → 1.0.1 |
| MINOR | neue, kompatible Anforderungen | 1.0.0 → 1.1.0 |
| MAJOR | normative Inkompatibilität (verpflichtende Regel entfernt oder inkompatibel geändert) | 1.9.0 → 2.0.0 |

## 10. Git-Versionierung

Git-Tags sind die technische Referenz für Releases. Format: `vMAJOR.MINOR.PATCH` (Pre-Releases: `v1.0.0-alpha.1`).

**Regel:** Ein offizielles Release MUSS durch einen unveränderlichen Git-Tag identifizierbar sein.

## 11. Repository-Version

Empfohlene kanonische Quelle: Datei `VERSION` mit dem Inhalt `1.4.2` — alternativ das Build-System. Es DARF NICHT mehrere widersprüchliche kanonische Versionsquellen geben.

## 12. Versionierung von APIs

- Neue kompatible Funktionen: `/api/v1/` bleibt
- Breaking Change: `/api/v2/`

Eine API-Version DARF NICHT allein deshalb geändert werden, weil ein PATCH-Release veröffentlicht wird.

## 13. Protokoll-Versionierung

```yaml
protocol_version: 2.1.0
consensus_version: 1.3.0
state_version: 4
network_version: 2
```

Damit wird unterschieden zwischen: Software-Version, Protokoll-Version, Konsens-Version, Blockchain-State-Version.

## 14. Smart-Contract-Versionierung

Smart Contracts benötigen besonders strenge Versionierung:

```yaml
contract:
  name: ATCToken
  version: 1.2.0
  interface_version: 1.1.0
```

Bei bereits deployed Contracts MUSS die Deployment-/Address-Identität erhalten bleiben. Eine neue Contract-Version ist nicht automatisch ein Upgrade desselben Contracts (Registry gemäß ATC-STD-SC-019).

## 15. Blockchain-Versionierung

Mindestens folgende Ebenen MÜSSEN unterschieden werden:

```
ATC Software
    ├── Node Version
    ├── Wallet Version
    ├── Miner Version
    ├── SDK Version
    ├── API Version
    ├── Protocol Version
    ├── Consensus Version
    ├── State Version
    └── Network Version
```

Beispiel:

```yaml
release:
  version: 2.4.0
node:
  version: 2.4.0
protocol:
  version: 2.1.0
consensus:
  version: 1.5.0
state:
  version: 7
network:
  version: 3
```

## 16. KI-Agenten-Versionierung

```yaml
agent:
  id: ATC-AI-DEV-001
  name: ATC Development Agent
  version: 1.4.0
  role: software-development
  protocol_version: 1.2.0
  capabilities_version: 1.1.0
```

Damit kann ein Audit feststellen: Welcher Agent, welche Version und welche Fähigkeiten haben diese Änderung erzeugt?

## 17. Dokumentations-Versionierung

Versionsabhängige Dokumentation MUSS versioniert werden (`docs/v1/`, `docs/v2/` bzw. `version-1/`, `version-2/`).

## 18. CHANGELOG-Verknüpfung

Jede veröffentlichte Version MUSS einen CHANGELOG-Eintrag besitzen:

```markdown
## [1.4.0] - 2026-09-07

### Added
- New ATC API endpoint

### Changed
- Improved node synchronization

### Fixed
- Miner connection issue

### Security
- Updated dependency
```

Der CHANGELOG MUSS mit dem Release-Tag übereinstimmen.

## 19. Release-ID und Build-ID

```yaml
release_id: ATC-REL-20260907-001
version: 1.4.0
git_tag: v1.4.0
```

Ein Release KANN mehrere Builds besitzen:

```yaml
version: 1.4.0
build:
  number: 1827
  id: ATC-BUILD-1827
  commit: <COMMIT_SHA>
```

Damit kann exakt festgestellt werden, welcher Build ausgeliefert wurde.

## 20. Commit-Versionierung

Commits erhalten keine eigene Semantic Version. Die Version gehört zum Release. Commit: `fix: correct wallet balance calculation` → Release: `v1.4.1`.

## 21. Monorepo-Regel

- **Variante A** — gemeinsame Version: `ATC Platform v2.0.0`, alle Komponenten werden gemeinsam released.
- **Variante B** — unabhängige Versionierung: `wallet v1.4.0`, `node v2.1.0`, `miner v3.0.1`, `sdk v1.8.0`.

Für das große ATC-Ökosystem SOLL langfristig Variante B mit übergeordnetem ATC Platform Release verwendet werden.

## 22. Versionskompatibilität

```yaml
requires:
  node: ">=2.0.0 <3.0.0"
  protocol: ">=2.1.0 <3.0.0"
  api: "^1.4.0"
```

## 23. Release Manifest

Maschinenlesbares Manifest je produktivem Release:

```yaml
release_id: ATC-REL-20260907-001
version: 1.4.0
status: STABLE
repository:
  name: atc-node
  commit: "<COMMIT_SHA>"
  tag: v1.4.0
components:
  node: 1.4.0
  protocol: 2.1.0
  consensus: 1.5.0
  api: 1.3.0
documentation:
  version: 1.4.0
changelog:
  version: 1.4.0
security:
  audit_required: true
```

## 24. Verbotene Praktiken

ATC-weit verboten für formale Releases: `latest`, `stable`, `final`, `new`, `new2`, `release-final`, `release-final2`, `v1-final`, `v1-final-fixed` sowie unvollständige Versionen (`1.0`, `1`, `Version 1`). Korrekt: ausschließlich `1.0.0`.

## 25. Compliance / Prüfungen

Jede Anforderung dieses Standards MUSS durch definierte Prüfkriterien überprüfbar sein (CI-automatisierbar):

### COM-VERSION-001

id: COM-VERSION-001

Die Version MUSS dem SemVer-Kernmuster `X.Y.Z` entsprechen (Schema `versionPatterns.semver`; S-03 für Standards).

### COM-VERSION-002

id: COM-VERSION-002

Jedes offizielle Release MUSS einen Git-Tag `vX.Y.Z` besitzen; Tag und CHANGELOG-Eintrag MÜSSEN übereinstimmen.

### COM-VERSION-003

id: COM-VERSION-003

Pro Repository DARF es nur eine kanonische Versionsquelle geben (VERSION-Datei oder Build-System, nicht beides widersprüchlich).

### COM-VERSION-004

id: COM-VERSION-004

Jede veröffentlichte Version MUSS einen CHANGELOG-Eintrag besitzen; Release-Manifeste MÜSSEN release_id, version und git_tag konsistent führen.

### COM-VERSION-005

id: COM-VERSION-005

Verbotene Release-Bezeichner (latest, stable, final, new, …) und unvollständige Versionen (1.0, 1) MÜSSEN durch automatisierten Scan erkannt und abgelehnt werden.

## 26. Golden Rule

> Eine Version darf nur einmal veröffentlicht werden und muss eindeutig einem definierten Quellstand, CHANGELOG, Release und Build zugeordnet werden können.

Damit gilt die durchgängige Kette:

```
Requirement → Standard → Change Request → Code Change → Commit → Build
→ Test → Audit → Release → Git Tag → Version → Deployment
```

## 27. Security Considerations

Security Fixes sind PATCH-Releases und MÜSSEN im CHANGELOG unter `### Security` dokumentiert werden. Release-Manifeste mit `audit_required: true` DÜRFEN NICHT ohne abgeschlossenes Audit deployed werden. Deployed Smart Contracts DARFEN NICHT mutiert werden — neue Versionen erfolgen als neue Deployment-Instanz (ATC-STD-SC-019 Registry-First).

## 28. Ausnahmen

Ausnahmen von diesem Standard MÜSSEN gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC-Verfahren) dokumentiert und durch den Owner genehmigt werden.

## 29. Referenzen (References)

### NORMATIVE Referenzen

- ATC-STD-000 — Standards Governance & Specification Standard (Verfassung; §7 Naming, §10 Lifecycle, §13 Versionierung)
- ATC-STD-DESC-001 — Standard Description Standard (Pflichtstruktur, Metadaten)
- ATC-STD-SC-019 — Contract Registry Standard (Deployment-Identität)
- SemVer 2.0.0 — https://semver.org/

### INFORMATIVE Referenzen

- schemas/naming-conventions.schema.json — `versionPatterns.semver`, `releaseId`, `buildId` (maschinenlesbare Regeldefinition)
- ATC-STD-MD-001 — ATC Markdown & Documentation Standard
- CHANGE_CONTROL.md — SCR-Verfahren (ATC-STD-000 §20)
- Geplante Folge-Standards: ATC-STD-CHANGELOG-001, ATC-STD-RELEASE-001, ATC-STD-GIT-001, ATC-STD-AUDIT-001 (noch nicht allockiert — bedürfen jeweils eigener SCRs)

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Keine Zugangsdaten in Artefakten; Security-Review-Pflicht bei sicherheitsrelevanten Aenderungen (ATC-STD-203).

## Changelog

### 1.0.0 — 2026-09-07
- Initial Release (Owner-Entwurf Michael Wroblewski, harmonisiert mit ATC-STD-000 v1.2.0 und ATC-STD-DESC-001)
- 22 normative Anforderungen (REQ-VERSION-001..022)
- Release-ID-Dualformat (versionsbasiert + datumsbasiert) ins Schema übernommen; buildId-Pattern allockiert
- Verbotene Praktiken und Golden Rule verbindlich definiert
- Owner-§9-Freigabe: APPROVED (21:57 UTC+2, Sammelfreigabe) — normativ ab sofort

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203 · INFORMATIVE: Registry-SSOT standards.yaml

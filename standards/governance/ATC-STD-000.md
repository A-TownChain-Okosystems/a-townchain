---
standard:
  id: ATC-STD-000
  title: "Standards Governance & Specification Standard"
  version: "1.3.0"
  status: approved
  category: governance
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
---

# ATC-STD-000 — Standards Governance & Specification Standard (v1.3.0, APPROVED)

> ATC-STD-000 ist der Meta-Standard fuer alle ATC Standards: Er definiert das
> uebergeordnete Governance- und Spezifikationsmodell des gesamten
> ATC-Standardsystems. Kein untergeordneter Standard darf seinen
> grundlegenden Governance-Regeln widersprechen.

## 1. Purpose

ATC-STD-000 definiert das uebergeordnete Governance- und Spezifikationsmodell
fuer saemtliche offiziellen A-TownChain Standards. Dieser Standard legt fest:

- wie Standards erstellt werden,
- wie Standards strukturiert werden,
- wie Anforderungen formuliert werden,
- wie Standards versioniert werden,
- wie Aenderungen kontrolliert werden,
- wie Standards geprueft werden,
- wie Standards genehmigt werden,
- wie Standards miteinander verknuepft werden,
- wie Compliance festgestellt wird,
- wie Standards deprecated und retired werden.

ATC-STD-000 ist der Meta-Standard fuer alle ATC Standards.

## 2. Scope

Dieser Standard gilt fuer alle normativen Standards innerhalb des
ATC-Standardsystems. Insbesondere:

- Architecture
- Repository
- Development
- Security
- Protocol
- Blockchain
- AI
- OS
- Infrastructure
- Applications
- Interoperability
- Token Standards
- GameFi

Nicht-normative Dokumentation ist davon ausgenommen, sofern sie nicht
ausdruecklich als ATC Standard registriert wird.

## 3. Governance Hierarchy

Die normative Hierarchie lautet:

```
ATC-STD-000
    |
    v
Standards Registry
    |
    +--------------+--------------+
    v              v              v
Architecture   Security      Protocol
    |              |              |
    +--------------+--------------+
    v
Implementations
    |
    v
Compliance
```

Kein untergeordneter Standard darf die grundlegenden Governance-Regeln von
ATC-STD-000 widersprechen.

## 4. Normative Language

ATC Standards verwenden folgende Begriffe:

| Keyword | Bedeutung |
|---|---|
| MUST | zwingende Anforderung |
| MUST NOT | zwingendes Verbot |
| REQUIRED | verpflichtend |
| SHOULD | starke Empfehlung |
| SHOULD NOT | starke Empfehlung dagegen |
| MAY | optional |

Normative Anforderungen muessen eindeutig interpretierbar sein.

## 5. Standard Identity

Jeder Standard MUST eine eindeutige Standard-ID besitzen. Primaeres Format:
`ATC-STD-XXX` — Beispiele: ATC-STD-000, ATC-STD-100, ATC-STD-200,
ATC-STD-400, ATC-STD-600.

Die ID darf nach Veroeffentlichung nicht wiederverwendet werden.

## 6. Standard Metadata

Jeder Standard MUST einen maschinenlesbaren Metadatenblock besitzen:

```yaml
standard:
  id: ATC-STD-000
  title: Standards Governance & Specification Standard
  version: "1.0.0"
  status: candidate
  normative: true
  category: governance
  authority: A-TownChain Ecosystems
  created: 2026-09-07
  updated: 2026-09-07
  supersedes: null
  superseded_by: null
```

## 7. ATC Naming, Identification and Namespace Convention

### 7.1 General Naming Principles

- Identifiers und Dateinamen bestehen ausschliesslich aus ASCII (`A-Z`,
  `a-z`, `0-9`, Bindestrich, Punkt gemaess Klassen-Muster). Unterstriche,
  Leerzeichen und Umlaute sind in IDs unzulaessig.
- Der Name ist klassenselbstbeschreibend: Das Praefix identifiziert die
  Objektklasse (ATC-STD-, REQ-, F-, SCR-, ADR-, GATE-, ...).
- Der Status ist NIEMALS Bestandteil der ID. Ungueltig:
  ATC-STD-000-DRAFT, ATC-STD-000-FINAL, ATC-STD-000-APPROVED. Gueltig:
  ID `ATC-STD-000` + Version `1.0.0` + Status `review` (getrennte Felder).
- Die Identitaet ist von der Version getrennt (7.3, 7.4).

### 7.2 Identifier Classes

| Objekt | Format | Beispiel |
|---|---|---|
| Standard | ATC-STD-NNN | ATC-STD-000 |
| Requirement | REQ-STD-NNN | REQ-STD-001 |
| Finding | F-NNN | F-017 |
| Change Request | SCR-NNN | SCR-001 |
| Architecture Decision | ADR-NNN | ADR-001 |
| Security Advisory | ATC-SA-NNN | ATC-SA-001 |
| Test Case | TC-NNN | TC-001 |
| Test Suite | TS-NNN | TS-001 |
| Validation Gate | GATE-NNN | GATE-001 |
| Schema | ATC-SCHEMA-NNN | ATC-SCHEMA-001 |
| Protocol | ATC-PROTO-NNN | ATC-PROTO-001 |
| Specification | ATC-SPEC-NNN | ATC-SPEC-001 |
| Document | ATC-DOC-NNN | ATC-DOC-001 |
| Release | ATC-REL-X.Y.Z | ATC-REL-1.0.0 |

Regeln:

- NNN ist eine fortlaufende numerische Kennung mit MINDESTENS drei Stellen.
  `ATC-STD-1` und `ATC-STD-01` sind UNGUELTIG.
- Requirements: `REQ-STD-NNN` ist die kanonische Form fuer ATC-STD-000.
  Domaenen-qualifizierte Subklassen gemaess Abschnitt 10 (REQ-REPO-001,
  REQ-SEC-001, REQ-PROTO-001) sind zulaessig; die Menge der Domaenen-Codes
  ist GESCHLOSSEN und wird nur per SCR erweitert (7.7).
- Architecture Decisions: neue eigenstaendige ADR-Dokumente verwenden
  ADR-NNN; das zentrale DECISIONS_REGISTER behaelt seine grandfathered
  AD-NNN-Form (7.9).

### 7.3 Identifier Immutability

> The canonical object identifier MUST remain immutable throughout the
> complete lifecycle of the object.

- `ATC-STD-042` bleibt `ATC-STD-042` — unabhaengig von Titel, Inhalt,
  Kategorie und Status. Die Version entwickelt sich weiter
  (v1.0.0 -> v1.1.0 -> v2.0.0), die Identitaet nicht.
- NICHT ZULAESSIG: `ATC-STD-042` -> `ATC-STD-043` weil sich Titel, Inhalt,
  Kategorie oder Status geaendert haben.
- Eine einmal vergebene ID wird NIE wiederverwendet (auch nicht nach
  Retirement).

### 7.4 Version Identification

- Versionen folgen X.Y.Z (SemVer, Abschnitt 21) und sind von der Identitaet
  getrennt.
- Referenz-Pinning: `ATC-STD-042@1.0.0` bezeichnet exakt diese Fassung
  (Abschnitt 30, Immutability & Auditability).

### 7.5 Repository Naming

- Kanonische Form fuer technische Repositories: `atc-<domain>-<component>`
  (z.B. atc-core, atc-node, atc-wallet, atc-consensus, atc-standards,
  atc-standards-schema, atc-standards-validator).
- Projektfamilien-Namespaces: `atc-*`, `atclang-*` (atclang-compiler,
  atclang-vm, atclang-stdlib), `globus-*` (globus-kernel, globus-runtime),
  `aurora-*` (aurora-image-ai, aurora-model-runtime). Die organisatorische
  Zugehoerigkeit ist am Repository-Namen ablesbar.
- Bestands-Repositories behalten ihre Namen (Immutabilitaet 7.3):
  a-townchain, a-townchain-os, a-townchain-os-docs, genesis-engine,
  genesis-chronicles sind grandfathered.

### 7.6 File Naming

- Dateinamen: ausschliesslich ASCII, Gross-/Kleinschreibung gemaess Schema,
  Bindestriche, definierte Extensions.
- Standard-Dokumente: `ATC-STD-NNN.md`. Schemas: `<name>.schema.json` /
  `<name>.schema.yaml` (z.B. standard-meta.schema.json,
  integrity-manifest.schema.json). Begleit-Metadaten:
  `ATC-STD-NNN.integrity.yaml`, `ATC-STD-NNN.review.yaml`,
  `ATC-STD-NNN.compliance.yaml`.
- UNGUELTIG: `atc_std_000.md`, `ATC_STD_000.md`, `ATC Standard 000.md`,
  `standard-final.md`, `standard-final-v2.md`.

### 7.7 Namespace Allocation

- Die Allokation erfolgt ausschliesslich ueber die Registry (naechste freie
  Nummer je Bereich; SCR-0001). Neue Domaenen-Codes, Familien-Namespaces und
  Bereichs-Reservierungen werden per SCR entschieden (ATC-STD-000 §20).
- Die verbindliche Bereichs-Reservierung der 10 Domaenen-Familien (000–900)
  ist ab v1.3.0 in ATC-STD-002 (Standards Family ID & Artifact Identifier
  Architecture, SCR-0096) festgelegt; neue Standards MUSSSEN im Bereich
  ihrer Domaene allokiert werden (Grandfathering s. ATC-STD-002 §3).
- Eine vergebene ID wird nie wieder frei (7.3).

### 7.8 Duplicate Prevention

- Das Namens-Schema allein kann Doppelvergaben nicht erkennen. Dafuer ist
  die Registry die Allokations-Autoritaet: ID existiert nicht -> allokieren;
  ID existiert -> gleiches Objekt = gueltig, anderes Objekt = FAIL.
- CI prueft Doppelvergaben dateiuebergreifend (7.11, Validator-Regel S-17).

### 7.9 Reserved Identifiers

Reserviert und nicht allokierbar:

- Platzhalter: `NNN`, `X.Y.Z` (keine IDs).
- Status-Marker in IDs: DRAFT, REVIEW, CANDIDATE, APPROVED, STABLE, FINAL.
- `ATC-STD-000` selbst (die Verfassung).
- Legacy-Serien: ATC-01...99, ATC-0001...0008, ATS-1000...1007 (historische
  Nummerierung bleibt gueltig; keine Neuallokation in diesen Bereichen).
- Die AD-NNN-Form des zentralen DECISIONS_REGISTER (grandfathered
  Legacy-Form von ADR-NNN; bestehende Nummern unveraendert).

### 7.10 Machine-Readable Naming Rules

- MUST: `schemas/naming-conventions.schema.json` ist die ZENTRALE
  Regeldefinition mit identifier patterns, repository patterns, filename
  patterns, version patterns, extension rules und reserved names.
- MUST: Das Schema verwendet `additionalProperties: false` und explizite
  Typdefinitionen.
- MUST: Validator und CI leiten ihre Pruefungen NUR aus dieser Datei ab
  (Single Source of Truth) — Regelquelle und Pruefungsquelle fallen zusammen.

### 7.11 CI Enforcement

- MUST: CI validiert bei jedem Pull Request und Push: ID-Gueltigkeit,
  Doppelvergaben (Registry), ID-Immutabilitaet, Dateinamen,
  Repository-Namen, Versionen und Referenzen.
- PASS erlaubt Merge; FAIL blockiert. Damit ist die Naming Convention eine
  Governance Control, keine Dokumentationsregel.

## 8. Standard Lifecycle

Ein Standard durchlaeuft einen kontrollierten Lifecycle:

```
IDEA -> PROPOSED -> DRAFT -> REVIEW -> CANDIDATE -> APPROVED -> STABLE -> DEPRECATED -> RETIRED
```

Ein Statuswechsel muss nachvollziehbar dokumentiert werden.

## 9. Standard Structure

Ein normativer ATC Standard sollte mindestens folgende Struktur besitzen:

1. Purpose · 2. Scope · 3. Terminology · 4. Normative Language ·
5. Requirements · 6. Architecture / Model · 7. Security Considerations ·
8. Compatibility · 9. Validation · 10. Compliance · 11. Migration ·
12. Versioning · 13. References · 14. Changelog

Zusaetzliche Abschnitte sind zulaessig.

## 10. Requirements

Normative Anforderungen erhalten eindeutige IDs (`REQ-XXX`), bei
domaenenspezifischen Standards z.B. `REQ-REPO-001`, `REQ-SEC-001`,
`REQ-PROTO-001`. Jede normative Anforderung MUST eindeutig sein.

## 11. Requirement Definition

Eine Requirement Definition sollte mindestens enthalten: Requirement ID,
Normative Level, Description, Applicability, Validation Method.

```yaml
requirement:
  id: REQ-STD-001
  level: MUST
  description: "Every normative ATC Standard MUST have a unique identifier."
  applicability: all_standards
  validation: {automated: true}
```

## 12. Standard Dependencies

Standards duerfen andere Standards referenzieren (`dependencies`).
Zirkulaere Abhaengigkeiten sind VERBOTEN (STD-A -> STD-B -> STD-C -> STD-A
= INVALID). Der Validator erkennt Zyklen.

## 13. Normative Authority

Ein Standard muss eindeutig festlegen, welche Teile normativ sind:

- **Normative:** Requirements, Definitions, Protocol Rules, Compliance Rules
- **Informative:** Examples, Explanations, Background, Rationale

Informative Inhalte duerfen keine widerspruechlichen Regeln enthalten.

## 14. Review Chain

Ein Standard darf nicht ohne Pruefung den Status APPROVED oder STABLE
erhalten. Die minimale Review Chain:

```
Author -> Technical Review -> Security Review -> Architecture Review -> Approval
```

Fuer bestimmte Standards koennen zusaetzliche Reviews erforderlich sein.

### 14.1 Rollen- und Berechtigungsmodell

Die Rollen des Standardsystems (verbindlich ab v1.1.0; SCR-0004,
Owner-Mandat 07.09.2026):

| Rolle | Berechtigungen | Besetzung |
|-------|----------------|-----------|
| **Owner** | Standard-Approval (§18), SCR-Entscheidungen (§20), Lifecycle-Uebergaenge STABLE/DEPRECATED/RETIRED, Rollenvergabe | Genau 1 (Michael Wroblewski) |
| **Approver** | Approval im Auftrag des Owners (delegiert, dokumentiert) | 0..n; jede Delegation durch Owner-Entscheid verzeichnet |
| **Reviewer** | Technical/Security/Architecture Review (§15-17) | Je Review-Typ mind. 1 bei REVIEW-Eintritt; Reviewer != Author |
| **Maintainer** | Schreibrechte je Registry-Domain (atc/, ats/, standards/, registry/) | beliebig, je Domain zuordnungspflichtig |
| **Agent** | Executor mit dokumentierter Policy (AGENT_POLICY.md, AGENT_COORDINATION.md, signierte Commits) | beliebig; niemals Approver; ausdrueckliche Owner-Freigabe je Massnahme |  

**Erlass-Regeln:**

1. Approvals und SCR-Entscheidungen liegen unuebertragbar beim Owner, sofern
   nicht per dokumentierter Entscheidung an einen Approver delegiert.
2. Agents fuehren Beschluesse aus, fassen sie nicht: jede normative Aenderung
   durch einen Agent benoetigt ein Owner-Mandat (Commits + SCR verweisen
   darauf). Agent-Commits ohne Mandat sind nicht konform (§34).
3. Fuer einfache Standards (Kategorie documentation, informative Standards)
   kann der Owner die Review-Chain per dokumentiertem Mandat auf einen
   Review-Typ reduzieren ("Erlass"). Der Erlass ist im Approval-Paket zu
   vermerken.
4. Rollenaenderungen (Delegation, Maintainer-Zuordnung) werden in der
   Registry (registry/teams.yaml) dokumentiert — Registry schlaegt Chat (§25).

## 15. Technical Review

Bewertet: technische Konsistenz, Vollstaendigkeit, Implementierbarkeit,
Requirement-Qualitaet, Terminologie, Testbarkeit, Versionierung,
Kompatibilitaet. Ergebnis: PASS | REQUEST CHANGES | REJECT.

## 16. Security Review

Bewertet: Security Impact, Attack Surface, Trust Boundaries,
Missbrauchsmoeglichkeiten, Governance-Manipulation, Supply-Chain-Risiken,
Secrets und Credentials, Recovery-Mechanismen. Ergebnis:
PASS | REQUEST CHANGES | REJECT.

## 17. Architecture Review

Bewertet: Systemarchitektur, Dependency Direction, Layering,
Interoperabilitaet, langfristige Erweiterbarkeit, Konflikte mit bestehenden
Standards, Breaking Changes. Ergebnis: PASS | REQUEST CHANGES | REJECT.

## 18. Approval

Ein Standard darf APPROVED werden, wenn Technical Review PASS, Security
Review PASS, Architecture Review PASS — und keine offenen kritischen
Findings vorhanden sind.

## 19. Stable

STABLE bedeutet: Der Standard ist offiziell, normativ und fuer produktive
ATC-Systeme verbindlich. Ein Stable Standard darf nicht stillschweigend
geaendert werden — jede Aenderung erzeugt eine neue Version oder einen
kontrollierten Change Process.

## 20. Change Management

Aenderungen an Standards werden ueber einen Standard Change Request
gesteuert (`SCR-XXXX`) mit Schema: SCR ID, Affected Standard, Current
Version, Proposed Version, Motivation, Technical Impact, Security Impact,
Architecture Impact, Compatibility Impact, Migration, Decision.

## 21. Versioning

ATC Standards verwenden Semantic Versioning MAJOR.MINOR.PATCH:

- MAJOR: Breaking normative change (1.0.0 -> 2.0.0)
- MINOR: Kompatible Erweiterung (1.0.0 -> 1.1.0)
- PATCH: Fehlerkorrektur ohne Aenderung der normativen Bedeutung (1.0.0 -> 1.0.1)

## 22. Breaking Changes

Gelten als Breaking Change: MUST -> MUST NOT, SHOULD -> MUST, Entfernung
einer REQUIRED Rule, Aenderung einer normativen Semantik, inkompatibles
Datenformat/Protokoll/API, Aenderung von Compliance-Kriterien. Diese
benoetigen eine MAJOR-Version.

## 23. Compliance

Jede normative Anforderung muss grundsaetzlich pruefbar sein. Zustaende:
PASS | FAIL | PARTIAL | NOT-APPLICABLE (je REQ; ein FAIL = Compliance FAIL).

## 24. Validation

Standards sollten, soweit moeglich, automatisch validiert werden:
Schema Validation -> Requirement Validation -> Reference Validation ->
Dependency Validation -> Compliance Validation.

## 25. Standard Registry

Alle offiziellen Standards muessen in einer zentralen Registry registriert
werden. Ein nicht registrierter Standard ist kein offizieller ATC Standard.

## 26. Standard Categories

Empfohlene Numm erraeume: 000-099 Governance, 100-199 Architecture,
200-299 Repository/Git, 300-399 Development, 400-499 Security, 500-599
Protocol, 600-699 Blockchain, 700-799 AI, 800-899 OS/Runtime, 900-999
Infrastructure, 1000+ Applications/Ecosystem.

## 27. Supersession

Ein Standard kann einen bestehenden Standard ersetzen (`supersedes`; die
alte Version wird DEPRECATED). Die alte Version bleibt fuer historische
Nachvollziehbarkeit erhalten.

## 28. Deprecation

Bei DEPRECATED muessen dokumentiert werden: Grund, Ersatz-Standard,
Migration, End-of-Support, Auswirkungen.

## 29. Retirement

RETIRED bedeutet: Der Standard besitzt keine normative Gueltigkeit mehr.
Ein retired Standard darf nicht fuer neue Implementierungen verwendet
werden. Die historische Fassung bleibt archiviert.

## 30. Immutability & Auditability

Eine veroeffentlichte Stable-Version sollte unveraenderlich behandelt
werden: ATC-STD-000@1.0.0 bleibt exakt diese Fassung. Aenderungen erzeugen
1.0.1 / 1.1.0 / 2.0.0. Jede Version muss nachvollziehbar sein.

## 31. Conflict Resolution

Bei widerspruechlichen normativen Anforderungen zweier Standards:
Conflict -> Identify Authority -> Dependency Analysis -> Architecture
Review -> Resolution -> Change Request. ATC-STD-000 besitzt dabei die
hoechste Governance-Prioritaet innerhalb des Standard-Frameworks.

## 32. Emergency Changes

Fuer kritische Security-Probleme darf ein Emergency-Prozess existieren:
Security Incident -> Emergency Review -> Temporary Decision ->
Implementation -> Formal Standard Revision. Emergency Changes duerfen die
normale Governance nicht dauerhaft ersetzen.

## 33. Conformance Statement

Ein Repository oder System kann Konformitaet erklaeren (Standard, Version,
Status, Validation, Datum). Die Behauptung muss durch ueberpruefbare
Evidence unterstuetzt werden.

## 34. Standard Integrity

Die Integritaet des Standardsystems muss geschuetzt werden. Fuer das
zentrale atc-standards-Repository: Protected main, Required Reviews,
CODEOWNERS, CI Validation, Secret Scanning, Dependency Scanning, Signed
Releases, Immutable Tags, Audit Log.

## 35. Meta-Compliance

ATC-STD-000 darf nicht vollstaendig durch einen darunterliegenden Standard
definiert werden (sonst: STD-000 -> STD-001 -> STD-000, Zirkel). Die
Meta-Governance muss unabhaengig bleiben.

## 36. Final Governance Model

```
ATC-STD-000 (Standards Constitution)
    |
    +---------------+---------------+
    v               v               v
Registry       Lifecycle      Change Control
    |               |               |
    +---------------+---------------+
    v
Official ATC Standards
    |
    +-------+-------+-------+-------+
    v       v       v       v       v
Architecture Security Protocol Blockchain AI
    |       |       |       |       |
    +-------+-------+---+---+-------+
                    v
             Implementations
                    v
               Compliance
                    v
               Production
```

## 37. ID-Allokationsprozess (SCR-0001)

Neue Standard-IDs werden wie folgt allokliert (Registry-First):

1. **Bereich:** Die ID liegt im fuer die Kategorie reservierten Nummernbereich
   (§7.11 Namespace Allocation, z. B. 200-299 fuer repository-Standards).
2. **Regel:** Allokliert wird die **naechste freie Nummer ab
   Bereichsbeginn aufsteigend** (z. B. nach ATC-STD-203 ist die naechste
   freie ID im 200er-Bereich ATC-STD-204).
3. **Registry-Zeitgleichkeit:** Die Registry-Zeile wird zeitgleich mit der
   ID-Vergabe angelegt (Status `idea` oder `proposed`) — eine ID existiert
   erst mit ihrem Registry-Eintrag (keine Schatten-IDs).
4. **Keine Nachbesetzung:** Luecken durch OBSOLETE- oder RETIRED-Standards
   werden **nicht nachbesetzt** — Luecken sind historische Marker und
   bleiben Teil der Allokationshistorie.

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-STD-002 | Neue Standard-IDs MUESSEN nach der Allokationsregel dieses Abschnitts (naechste freie Nummer ab Bereichsbeginn, Registry-Zeitgleichkeit, keine Luecken-Nachbesetzung) vergeben werden. | §37 |

## 38. Security Considerations (F-004)

Die Sicherheit des Standardsystems selbst ist normativ geschuetzt:

1. **Secrets-Policy:** Tokens, API-Keys und Zugangsdaten duerfen NIEMALS im
   Klartext in Standards, Registry-Dateien, Wikis, Issues, Commits, Logs
   oder Reports erscheinen. Zulaessig sind ausschliesslich
   Umgebungsvariablen-Platzhalter (`$TOKEN_NAME`) oder maskierte Werte
   (`****`). Verstoss = SECURITY-Finding (S1).
2. **Vertrauenskette des Governance Root:** Registry-Aenderungen laufen
   ueber die Branch-Protection des Governance Root (required reviews,
   kein Force-Push, Secret-Scanning mit Push-Protection).
3. **Signierung:** Releases des Governance Root werden per GPG signiert
   (immutable Tags, ATC-STD-203); bis zur Bereitstellung des
   Release-Keys gilt das Ausnahmeprotokoll in DECISIONS_REGISTER.
4. **Integritaet der Standards:** Der Wortlaut eines APPROVED/STABLE
   Standards ist immutable; Aenderungen nur per Versionsbump und SCR
   (§34 Standard Integrity).
5. **Reviewer-Unabhaengigkeit:** GATE-SECURITY-Entscheidungen muessen
   unabhaengig von der implementierenden Partei dokumentiert werden
   (ATC-STD-300 §23).

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-STD-003 | Standards, Registry-Dateien und Governance-Artefakte MUESSEN die Secrets-Policy (Absatz 1) einhalten; Verstoesse sind SECURITY-Findings. | §38 |

## Aktueller Status

Review-Chain nach §14 gegen diese Fassung durchgefuehrt und dokumentiert
(approval/TECHNICAL-REVIEW.md, approval/SECURITY-REVIEW.md,
approval/ARCHITECTURE-REVIEW.md):

```
ATC-STD-000 v1.0.0
    |
    +-- Technical Review    -> PASS   (07.09.2026)
    +-- Security Review      -> PASS   (07.09.2026)
    +-- Architecture Review  -> PASS   (07.09.2026)
    +-- Approval             -> APPROVED (07.09.2026, Owner-Mandat)
```

Pfad: DRAFT -> REVIEW -> CANDIDATE -> APPROVED (07.09.2026, Owner-Freigabe).
Damit ist ATC-STD-000 keine Ausnahme von den Regeln, die es definiert —
die Verfassung ist vollstaendig durch die eigene Kette gelaufen.

**Fassungsstand:**
- v1.3.0 = CANDIDATE (11.09.2026, SCR-0096): §7.7 um Verweis auf die
  verbindliche Domaenen-Familien-Bereichs-Reservierung in ATC-STD-002
  (Standards Family ID & Artifact Identifier Architecture) erweitert —
  Allokation kuenftig im Bereich der Domaene, Bestand grandfathered. Bis
  zur §9-Freigabe gilt v1.2.0 APPROVED.
- v1.1.0 = APPROVED-Fassung (07.09.2026, SCR-0004)
- v1.2.0 = APPROVED (07.09.2026, 20:00 UTC+2, Owner-Freigabe "Alles freigeben"):
  §37 ID-Allokationsprozess (SCR-0001), §38 Security Considerations (F-004).
  Review-Chain §14 gegen v1.2.0: Technical PASS · Security PASS ·
  Architecture PASS (07.09.2026). Dokumentiert in
  approval/APPROVAL-DECISION-2026-09-07-000-v1.2.0.md. v1.2.0 ist die
  gültige Verfassungsfassung; Änderungen nur noch via SCR (§30).

## Changelog

- 1.2.0 (07.09.2026, APPROVED): §37 ID-Allokationsprozess (REQ-STD-002,
  Registry-First, keine Lückennachbesetzung) + §38 Security Considerations
  (F-004: Branch Protection, CODEOWNERS, Immutable Tags, CI-Scanning —
  physisch umgesetzt per SCR-0003 Option B). Owner-Freigabe 07.09.2026
  20:00 UTC+2 ("Alles freigeben").

- 1.1.0 (07.09.2026): SCR-0004 (Owner-Mandat) — neuer Abschnitt 14.1
  Rollen- und Berechtigungsmodell (Owner/Approver/Reviewer/Maintainer/Agent,
  Delegations-, Mandats- und Erlass-Regeln). MINOR, nicht-breaking.

- 1.0.0 (07.09.2026): Owner-Formalfassung (35 Abschnitte) als verbindlicher
  v1.0.0-Text angenommen; Review-Chain §14 durchlaufen (3/3 PASS); Status
  CANDIDATE. Ergänzung 1: Naming-Convention-Vorversion als §36.
- 1.0.0, Ergaenzung 2 (07.09.): NORMATIVES NAMING-HARDENING per Owner-Mandat —
  neuer §7 "ATC Naming, Identification and Namespace Convention" mit 7.1-7.11
  (General Principles, Identifier Classes, Immutability, Version
  Identification, Repository/File Naming, Namespace Allocation, Duplicate
  Prevention, Reserved Identifiers, Machine-Readable Rules, CI Enforcement).
  Abschnitte 7-35 → 8-36 verschoben; die §36-Vorversion ist in §7 aufgegangen.
  Schema additionalProperties:false gehärtet; Validator um S-17 (Duplicate
  Detection) erweitert; CI-Workflow errichtet. Mapping für frühere Referenzen:
  alt N → neu N+1 (N=7…35), alt 36 → neu 7.

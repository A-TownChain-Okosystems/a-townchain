---
standard:
  id: ATC-STD-COMPAT-001
  title: "ATC Major Version Compatibility & Recovery Standard — Verbindliche Kompatibilitätsprüfung, -Wiederherstellung und -Migration nach MAJOR-Updates"
  version: "1.0.0"
  status: approved
  category: compat
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  effective_date: "2026-09-07"
  review_date: "2027-09-07"
  classification: PUBLIC
  approved_by: "Michael Wroblewski (Owner, §9-Freigabe 07.09.2026, 23:02 UTC+2)"
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-VERSION-001
    - ATC-STD-UPDATE-001
    - ATC-STD-AUDIT-001
  related_standards:
    - ATC-STD-BUG-005
    - ATC-STD-AI-DECISION-001
    - ATC-STD-MD-001
    - ATC-AAS-009
    - ATC-AAS-017
    - ATC-AAS-018
  requirements:
    - REQ-COMPAT-001
    - REQ-COMPAT-002
    - REQ-COMPAT-003
    - REQ-COMPAT-004
    - REQ-COMPAT-005
    - REQ-COMPAT-006
    - REQ-COMPAT-007
    - REQ-COMPAT-008
    - REQ-COMPAT-009
    - REQ-COMPAT-010
    - REQ-COMPAT-011
    - REQ-COMPAT-012
    - REQ-COMPAT-013
    - REQ-COMPAT-014
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-COMPAT-001 — ATC Major Version Compatibility & Recovery Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — §9-Freigabe Michael Wroblewski (Builder-Chat 07.09.2026, 23:02 UTC+2);
> normativ in Kraft ab 07.09.2026, §30-eingefroren (ATC-STD-000).
> **Familie:** Compat Standards (ATC-STD-COMPAT-001..999) — Kategorie `compat`, Companion zur Update-Familie (SCR-0015).
> **Rolle in der Change-Control-Kette:** SCR (ATC-STD-000 §30) → VERSION-001 → UPDATE-001 → **COMPAT-001 (MAJOR-Gate)** → AUDIT-001 → (geplant CHANGE-001, RELEASE-001).
> **Verankerung:** Pflicht-Gate in ATC-STD-UPDATE-001 REQ-UPD-008, Gate UPD-G04 (Compatibility) — bei MAJOR-Updates verstärkt durch diesen Standard.

## 1. Zweck (Purpose)

Der Standard definiert: dass nach jedem MAJOR-Version-Update eine vollständige Kompatibilitätsprüfung aller betroffenen und abhängigen Komponenten verpflichtend ist, wie Kompatibilität klassifiziert wird, wie die Prüfung auf fünf Ebenen erfolgt, wie Inkompatibilitäten wiederhergestellt werden, wie Wiederherstellungen identifiziert und dokumentiert werden, und wann ein MAJOR-Update als vollständig integriert gilt.

**Geltungsbereich:** Alle ATC-Repositories, Standards, Software, APIs, Protokolle, Smart Contracts, KI-Agenten, Datenmodelle und Infrastrukturkomponenten des A-TownChain-Ökosystems.

**Grundsatz:**

> Bei einem MAJOR-Version-Update reicht normales Regression Testing nicht aus: Das gesamte abhängige Ökosystem muss auf Kompatibilität geprüft und bei Bedarf migriert bzw. wieder kompatibel gemacht werden.

**Kernregeln (harte Gates):**

> **ATC-MAJOR-COMPATIBILITY-GATE:** Eine Major-Version darf nicht als vollständig freigegeben gelten, solange die Kompatibilität der betroffenen ATC-Komponenten nicht geprüft, dokumentiert und entweder bestätigt oder durch eine genehmigte Migration bzw. einen genehmigten Compatibility Layer wiederhergestellt wurde.

> **ATC-COMPATIBILITY-RESTORATION:** Jede festgestellte kritische Inkompatibilität muss entweder behoben, migriert, durch einen genehmigten Compatibility Layer überbrückt oder als bewusst akzeptierte Breaking Change formal freigegeben werden. Offenes Dokumentieren und Stehenlassen ist nicht zulässig.

## 2. Geltungsbereich (Scope)

**Gilt:** Jedes MAJOR-Update (SemVer-MAJOR gemäß ATC-STD-VERSION-001) im gesamten Ökosystem — unabhängig davon, ob es als Software-Release, Standardänderung (Standard MAJOR gemäß ATC-STD-UPDATE-001), Protokoll- oder Contract-Änderung klassifiziert ist.

**Nicht im Gilt (Abgrenzung):** Versionsvergabe und Release-Identifikation (ATC-STD-VERSION-001); der allgemeine Update-Lifecycle und Change Request (ATC-STD-UPDATE-001, ATC-STD-000 §30); Audit-Methodik (ATC-STD-AUDIT-001); Fehleranalyse (ATC-STD-BUG-005). COMPAT-001 definiert die MAJOR-spezifische Kompatibilitätspflicht ÜBER diesen Standards und ist in UPD-G04 als Pflicht-Gate verankert.

## 3. Anforderungen (Requirements)

### REQ-COMPAT-001 — MAJOR-Kompatibilitätspflicht (Grundregel)

id: REQ-COMPAT-001

Nach jedem MAJOR-Version-Update MUSS eine vollständige Kompatibilitätsprüfung aller betroffenen und abhängigen Komponenten DURCHGEFÜHRT werden. Eine neue Major-Version DARF erst als vollständig integriert gelten, wenn der folgende Prozess DURCHLAUFEN und abgeschlossen ist:

```
MAJOR UPDATE
     ↓
COMPATIBILITY AUDIT
     ↓
INCOMPATIBILITIES IDENTIFY
     ↓
IMPACT CLASSIFICATION
     ↓
COMPATIBILITY RESTORATION
     ↓
MIGRATION
     ↓
FULL REGRESSION TEST
     ↓
AUDIT
     ↓
APPROVAL
     ↓
RELEASE
```

### REQ-COMPAT-002 — Kompatibilitätsziel (Prüfbereiche)

id: REQ-COMPAT-002

Nach dem Update MUSS für jeden der folgenden Bereiche festgestellt werden, ob er mit der neuen Major-Version kompatibel ist (je ✓ geprüft / ✗ nicht anwendbar, begründet):

| Bereich | Prüfung |
|---|---|
| Standards | ✓ |
| Code | ✓ |
| APIs | ✓ |
| Datenmodelle | ✓ |
| Datenbanken | ✓ |
| Smart Contracts | ✓ |
| Blockchain-Protokoll | ✓ |
| Node | ✓ |
| Wallet | ✓ |
| Miner | ✓ |
| Marketplace | ✓ |
| SDKs | ✓ |
| Frontend | ✓ |
| Mobile | ✓ |
| KI-Agenten | ✓ |
| CI/CD | ✓ |
| Docker | ✓ |
| Kubernetes | ✓ |
| Dokumentation | ✓ |
| Wiki | ✓ |
| README | ✓ |
| CHANGELOG | ✓ |
| Konfiguration | ✓ |

### REQ-COMPAT-003 — Keine automatische Annahme von Kompatibilität

id: REQ-COMPAT-003

Nach einem Major Update gilt NICHT: `OLD VERSION → COMPATIBLE BY DEFAULT → NEW VERSION`. Stattdessen gilt für jede Komponente: `Compatibility Status = UNKNOWN`, bis die Prüfung abgeschlossen ist. Ein erfolgreicher Build, Compile oder Launch der neuen Version selbst BEDEUTET KEINE Kompatibilität des abhängigen Ökosystems und DARF nicht als solche interpretiert werden (Agenten- wie Entwicklerfehlerquelle).

### REQ-COMPAT-004 — Kompatibilitätsklassen

id: REQ-COMPAT-004

Jede Abhängigkeit MUSS einen der folgenden Status erhalten:

| Klasse | Bedeutung |
|---|---|
| `COMPATIBLE` | Keine Anpassung erforderlich. |
| `COMPATIBLE_AFTER_MIGRATION` | Kompatibilität kann durch eine definierte Migration hergestellt werden. |
| `INCOMPATIBLE` | Die Komponente funktioniert mit der neuen Major-Version nicht. |
| `BLOCKED` | Eine Abhängigkeit verhindert die Migration. |
| `UNKNOWN` | Kompatibilität wurde noch nicht ausreichend geprüft. |
| `DEPRECATED` | Kompatibel, soll aber zukünftig ersetzt werden. |

UNKNOWN DARF bei Release des Major-Updates nicht verbleiben — bis dahin MUSS jede relevante Komponente auf eine der anderen fünf Klassen festgelegt sein.

### REQ-COMPAT-005 — Kompatibilitäts-Matrix

id: REQ-COMPAT-005

Für jedes Major Update MUSS eine Matrix erstellt und versioniert dokumentiert werden:

| Komponente | Alt | Neu | Status | Maßnahme |
|---|---|---|---|---|
| API | 1.x | 2.x | INCOMPATIBLE | Adapter |
| Wallet | 1.x | 2.x | COMPATIBLE | – |
| Node | 1.x | 2.x | COMPATIBLE_AFTER_MIGRATION | Config Update |
| SDK | 1.x | 2.x | INCOMPATIBLE | SDK v2 |
| Marketplace | 1.x | 2.x | COMPATIBLE | – |
| Wiki | 1.x | 2.x | DEPRECATED | Update |
| Smart Contract | 1.x | 2.x | BLOCKED | Migration |
| KI-Agent | 1.x | 2.x | UNKNOWN → prüfen | Agent Audit |

(Beispielwerte — die Matrix wird je Update mit echten Komponenten und Versionen gefüllt.) Die Matrix ist Teil des Compatibility Reports (REQ-COMPAT-010).

### REQ-COMPAT-006 — Kompatibilitätsprüfung auf fünf Ebenen

id: REQ-COMPAT-006

Die Prüfung MUSS mindestens auf fünf Ebenen erfolgen:

**Level 1 — Syntax:** Funktioniert die technische Schnittstelle? (API, CLI, Config, Schema, Protocol, ABI)

**Level 2 — Semantik:** Bedeutet eine Funktion nach dem Update noch dasselbe? Beispiel: `transfer(amount)` funktioniert technisch weiterhin, könnte aber eine geänderte Gebührenlogik besitzen.

**Level 3 — Daten:** Prüfung von Datenstrukturen, Datenbank-Schema, Serialisierung, Deserialisierung, Migrationen, State, Cache, Storage.

**Level 4 — Runtime:** Prüfung des tatsächlichen Betriebs: Build → Start → Kommunikation → Verarbeitung → Fehlerbehandlung → Performance → Recovery.

**Level 5 — System:** Das gesamte Ökosystem wird als Gesamtsystem getestet:

```
User → Wallet → API → Node → Blockchain → Smart Contract → Indexer → Marketplace
```

### REQ-COMPAT-007 — Compatibility Restoration Process

id: REQ-COMPAT-007

Wenn eine Inkompatibilität gefunden wird, MUSS ein definierter Restoration Process starten — NICHT darf die Inkompatibilität lediglich dokumentiert und offen gelassen werden:

```
INCOMPATIBLE
     ↓
ROOT CAUSE
     ↓
SOLUTION DESIGN
     ↓
IMPLEMENT
     ↓
TEST
     ↓
VERIFY
     ↓
COMPATIBLE
```

### REQ-COMPAT-008 — Methoden zur Wiederherstellung

id: REQ-COMPAT-008

Je nach Ursache DARF eine der folgenden Strategien verwendet werden:

| Methode | Beschreibung |
|---|---|
| **A — Adapter** | Old Interface → Compatibility Adapter → New Interface |
| **B — Migration** | Old Data → Migration → New Data Model |
| **C — Wrapper** | Eine alte Schnittstelle wird auf die neue Implementierung abgebildet. |
| **D — Compatibility Layer** | Eine dedizierte Kompatibilitätsschicht wird zwischen alten und neuen Komponenten eingesetzt. |
| **E — Upgrade der abhängigen Komponente** | z. B. ATC Core v2 → SDK v2 → Wallet v2 (kaskadiertes Upgrade). |
| **F — Breaking Change akzeptieren** | Wenn Kompatibilität technisch oder wirtschaftlich nicht sinnvoll wiederherstellbar ist, MUSS die Inkompatibilität explizit als Breaking Change freigegeben werden (formale Owner-Freigabe, ATC-AAS-017 Human Approval). |

Strategie F DARF nur mit dokumentierter Begründung und formaler Freigabe erfolgen; sie ist ein bewusster Entscheid, kein Default.

### REQ-COMPAT-009 — Keine versteckten Kompatibilitäts-Fixes (Agentenregel)

id: REQ-COMPAT-009

Besonders verbindlich für KI-Agenten (Kopplung: ATC-AAS-009, ATC-STD-AI-DECISION-001):

```
Agent erkennt Inkompatibilität
     ↓
Agent DARF Lösung entwickeln
     ↓
Agent DARF Fix implementieren
     ↓
Tests
     ↓
Audit
     ↓
Approval
```

NICHT zulässig:

```
Agent → erkennt Problem → ändert heimlich abhängiges Repository → Problem verschwindet
```

Jede Wiederherstellung MUSS eine eigene ID erhalten: `COMP-NNN` (COMP-001, COMP-002, COMP-003, …), je Inkompatibilität fortlaufend, mit Root Cause, gewählter Methode (REQ-COMPAT-008), Tests, Audit und Approval dokumentiert.

### REQ-COMPAT-010 — Major Update Compatibility Report

id: REQ-COMPAT-010

Nach jedem Major Update MUSS ein Report entstehen und versioniert archiviert werden (Verzeichnis: `audits/` bzw. Audit-Ablage gemäß ATC-STD-AUDIT-001):

```yaml
compatibility_audit:
  update_id: UPD-002
  previous_version: 1.x
  target_version: 2.x

  audit_status: COMPLETE

  components:
    total: 25
    compatible: 18
    migration_required: 4
    incompatible: 2
    blocked: 1

  restoration:
    required: true
    completed: true

  regression_tests:
    status: PASS

  documentation:
    status: PASS

  final_status: APPROVED
```

### REQ-COMPAT-011 — Definition of Done (MAJOR)

id: REQ-COMPAT-011

Ein Major Update ist NICHT DONE, wenn neuer Code kompiliert. Es ist erst DONE, wenn ALLE folgenden Punkte erfüllt sind:

- MAJOR Version implementiert
- Abhängigkeiten identifiziert
- Compatibility Matrix erstellt
- alle relevanten Komponenten geprüft (kein UNKNOWN verblieben)
- Inkompatibilitäten identifiziert
- Root Causes analysiert
- Kompatibilität wiederhergestellt (COMP-NNN je Fund)
- Migrationen durchgeführt
- Regression Tests bestanden
- Security Tests bestanden
- Standards aktualisiert
- README aktualisiert
- Wiki aktualisiert
- CHANGELOG aktualisiert
- Registry aktualisiert
- Agenten-Kontext aktualisiert
- Audit bestanden
- Release freigegeben

### REQ-COMPAT-012 — ATC-MAJOR-COMPATIBILITY-GATE (harte Regel)

id: REQ-COMPAT-012

> **ATC-MAJOR-COMPATIBILITY-GATE:** Eine Major-Version darf nicht als vollständig freigegeben gelten, solange die Kompatibilität der betroffenen ATC-Komponenten nicht geprüft, dokumentiert und entweder bestätigt oder durch eine genehmigte Migration bzw. einen Compatibility Layer wiederhergestellt wurde.

### REQ-COMPAT-013 — ATC-COMPATIBILITY-RESTORATION (harte Regel)

id: REQ-COMPAT-013

> **ATC-COMPATIBILITY-RESTORATION:** Jede festgestellte kritische Inkompatibilität muss entweder behoben, migriert, durch einen genehmigten Compatibility Layer überbrückt oder als bewusst akzeptierte Breaking Change formal freigegeben werden.

### REQ-COMPAT-014 — Verankerung in ATC-STD-UPDATE-001

id: REQ-COMPAT-014

Dieser Standard ist als Pflicht-Gate im übergeordneten ATC-STD-UPDATE-001 verankert: Gate UPD-G04 (Compatibility) MUSS bei MAJOR-Updates durch ein vollständiges Verfahren nach COMPAT-001 erfüllt werden (Compatibility Audit → Matrix → Restoration → Regression → Report). Der geschlossene Gesamtprozess:

```
┌──────────────────────┐
│    MAJOR UPDATE      │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ COMPATIBILITY AUDIT  │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ FIND INCOMPATIBILITY │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ RESTORE COMPATIBILITY│
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ MIGRATE / ADAPT      │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│ FULL REGRESSION TEST │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│      AUDIT PASS      │
└──────────┬───────────┘
           ↓
┌──────────────────────┐
│      RELEASE         │
└──────────────────────┘
```

## 4. Verweise (References)

- ATC-STD-000 — Standards Governance (Verfassung; §9-Freigabe, §30-Change Requests)
- ATC-STD-VERSION-001 — Versioning (MAJOR-Definition, Release-Manifest)
- ATC-STD-UPDATE-001 — Update Standard (Change-Control-Kette; UPD-G04-Verankerung)
- ATC-STD-AUDIT-001 — Completeness & Audit (Audit-Ablage, Release-Gates)
- ATC-STD-BUG-005 — Fehleranalyse (Root Cause Analysis)
- ATC-AAS-009 / ATC-AAS-017 / ATC-AAS-018 — Agent Change, Human Approval, Audit Trail

## 5. Metadaten-Zusammenfassung

- 14 normative Anforderungen (REQ-COMPAT-001..014), 2 harte Kernregeln
- 6 Kompatibilitätsklassen, 5 Prüfebenen, 6 Wiederherstellungsmethoden (A–F)
- COMP-NNN-Wiederherstellungs-IDs, Compatibility Matrix, YAML-Reportformat
- Verankert als Pflicht-Gate in ATC-STD-UPDATE-001 (UPD-G04, MAJOR)
- Nach §9-Freigabe: normativ in Kraft, §30-eingefroren

---
*ATC-STD-COMPAT-001 v1.0.0 · Owner-Entwurf Michael Wroblewski · Aurora (Superagent) · 07.09.2026 · §9-APPROVED 23:02 UTC+2 — normativ, §30-eingefroren*

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Kompatibilitaets-Pruefungen duerfen keine Sicherheitsluecken alter Versionen reintroduzieren; veraltete Krypto-/API-Versionen werden abgelehnt, nicht geduldet; Rollback-Pfade auf als unsicher bekannte Stando sind gesperrt.

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-204, ATC-STD-VERSION-001 · INFORMATIVE: Registry-Kategorie compat

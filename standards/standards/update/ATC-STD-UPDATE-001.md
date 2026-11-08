---
standard:
  id: ATC-STD-UPDATE-001
  title: "ATC Update Standard — Change Control für Artefakt-Updates (Lifecycle, Gates, Migration, Rollback, Emergency)"
  version: "1.0.0"
  status: approved
  category: update
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  effective_date: "2026-09-07"
  review_date: "2027-09-07"
  classification: PUBLIC
  approved_by: "Michael Wroblewski (Owner, §9-Freigabe 07.09.2026, 23:28 UTC+2)"
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-VERSION-001
    - ATC-STD-AUDIT-001
    - ATC-STD-BUG-005
  related_standards:
    - ATC-STD-AI-DECISION-001
    - ATC-STD-BUG-001
    - ATC-STD-BUG-003
    - ATC-STD-COMPAT-001
    - ATC-STD-DESC-001
    - ATC-STD-MD-001
    - ATC-AAS-009
    - ATC-AAS-017
    - ATC-AAS-022
  requirements:
    - REQ-UPD-001
    - REQ-UPD-002
    - REQ-UPD-003
    - REQ-UPD-004
    - REQ-UPD-005
    - REQ-UPD-006
    - REQ-UPD-007
    - REQ-UPD-008
    - REQ-UPD-009
    - REQ-UPD-010
    - REQ-UPD-011
    - REQ-UPD-012
    - REQ-UPD-013
    - REQ-UPD-014
    - REQ-UPD-015
    - REQ-UPD-016
    - REQ-UPD-017
    - REQ-UPD-018
    - REQ-UPD-019
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-UPDATE-001 — ATC Update Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — §9-Freigabe Michael Wroblewski (Builder-Chat 07.09.2026, 23:28 UTC+2);
> normativ in Kraft ab 07.09.2026, §30-eingefroren (ATC-STD-000). Konfliktanalyse SCR-0015 akzeptiert.
> **Familie:** Update Standards (ATC-STD-UPDATE-001..999) — Kategorie `update`, SCR-0015.
> **Rolle in der Change-Control-Kette:** SCR (ATC-STD-000 §30) → VERSION-001 → **UPDATE-001** → AUDIT-001 → CHANGELOG (geplant: CHANGE-001, RELEASE-001). Separate, miteinander verknüpfte Standards statt eines überladenen Einzelstandards.

## 1. Zweck (Purpose)

Der Standard definiert: wann ein Artefakt aktualisiert werden darf, wer ein Update initiieren darf, wie Änderungen klassifiziert werden, wie Auswirkungen analysiert werden, welche Prüfungen erforderlich sind, wann ein Update freigegeben werden darf, wie abhängige Systeme synchronisiert werden, wie alte Versionen behandelt werden, wie die Änderung revisionssicher dokumentiert wird.

**Geltungsbereich:** Alle ATC-Standards, Repositories, Spezifikationen, Software, Smart Contracts, KI-Agenten, APIs, Protokolle und Governance-Dokumente.

**Grundsatz:**

> Kein produktives ATC-Artefakt darf durch eine undokumentierte oder ungeprüfte Änderung aktualisiert werden.

**Kernregel (ATC-NO-SILENT-UPDATE):**

> ATC-NO-SILENT-UPDATE: Keine normative, sicherheitsrelevante, protokollrelevante oder produktive Änderung darf ohne identifizierbaren Change Request, Versionsstatus und nachvollziehbaren Änderungsnachweis durchgeführt werden.

## 2. Geltungsbereich (Scope)

**Gilt:** Alle ATC-Artefakte des Ökosystems — Standards, Repositories, Spezifikationen, Software, Smart Contracts, KI-Agenten, APIs, Protokolle und Governance-Dokumente — für jedes Update (PATCH/MINOR/MAJOR/SECURITY/EMERGENCY/GOVERNANCE). Verbindlich normativ nach §9-Freigabe.

**Nicht im Gilt (Abgrenzung, SCR-0015):** SemVer-Vergabe und Release-/Build-ID-Formate (ATC-STD-VERSION-001); SCR-Governance-Review normativer Änderungen selbst (ATC-STD-000 §30 — UPDATE-001 koppelt, ersetzt nicht); System-Audits (ATC-STD-AUDIT-001); Fehleranalyse-Methodik (ATC-STD-BUG-005); CHANGELOG-Detailformat (geplant ATC-STD-CHANGELOG-001); Release-Prozess-Orchestrierung (geplant ATC-STD-RELEASE-001); Agenten-Change-Abläufe im Betrieb (ATC-AAS-009). UPDATE-001 definiert die übergreifende Change-Control-Pflicht über diesen Standards. Die MAJOR-spezifische Kompatibilitätsprüfung, -Wiederherstellung und -Migration ist im Companion-Standard ATC-STD-COMPAT-001 geregelt (Pflicht-Gate: UPD-G04 bei MAJOR).

## 3. Begriffe und Definitionen

| Begriff | Definition |
|---|---|
| Update | kontrollierte, versionierte, geprüfte und freigegebene Änderung eines ATC-Artefakts |
| Update Request | eindeutige Anfrage `UPD-NNN` mit Pflichtfeldern (Abschnitt 4) |
| Update-Kategorie | PATCH / MINOR / MAJOR (VERSION-001-Kopplung) · SECURITY / EMERGENCY / GOVERNANCE (prozessoral) |
| Update-Gate | Freigabepunkt UPD-G01..UPD-G09 (abgegrenzt zu AUD-G01..G08, ATC-STD-AUDIT-001) |
| Canonical Source | die eine kanonische Quelle je Artefakt, von der alle anderen Darstellungen abgeleitet sind |

## 4. Normative Anforderungen

### REQ-UPD-001 — Grundsatz und No-Silent-Update

id: REQ-UPD-001

Kein produktives ATC-Artefakt DARF durch eine undokumentierte oder ungeprüfte Änderung aktualisiert werden. ATC-NO-SILENT-UPDATE gilt verbindlich: Jede normative, sicherheitsrelevante, protokollrelevante oder produktive Änderung ERFORDERT identifizierbaren Change Request, Versionsstatus und nachvollziehbaren Änderungsnachweis.

### REQ-UPD-002 — Update-Kategorien

id: REQ-UPD-002

Jedes Update MUSS klassifiziert werden: PATCH (Fehlerkorrektur, z. B. Tippfehler/nichtfunktional), MINOR (Erweiterung ohne Breaking Change), MAJOR (inkompatible API-/Protokoll-Änderung) — Versionslogik gemäß ATC-STD-VERSION-001 (SemVer-Kopplung, keine Neudefinition); SECURITY (Sicherheitsupdate), EMERGENCY (Notfallupdate bei aktivem Exploit/kritischem Ausfall), GOVERNANCE (neue verbindliche Regel) — prozessorale Kategorien mit eigenem Pfad (REQ-UPD-009/011).

### REQ-UPD-003 — Update-Lifecycle

id: REQ-UPD-003

Jedes Update DURCHLÄUFT grundsätzlich: IDENTIFY → REQUEST → IMPACT ANALYSIS → DESIGN → IMPLEMENT → TEST → AUDIT → APPROVAL → RELEASE → MIGRATION → VERIFY → CLOSE. Für kritische Systeme DARF kein Schritt stillschweigend übersprungen werden; Überspringen MUSS dokumentiert und begründet werden.

### REQ-UPD-004 — Update Request

id: REQ-UPD-004

Jede Änderung MUSS eine eindeutige ID `UPD-NNN` (Schema `updateRequestId`) erhalten mit Pflichtfeldern: update_id, standard_id/artifact_id, current_version, target_version, type, reason, scope, impact, requested_by, created_at, dependencies, affected_repositories, affected_services, risk_level, status.

### REQ-UPD-005 — Impact Analysis

id: REQ-UPD-005

Vor jeder relevanten Änderung MÜSSEN geprüft werden: direkte Auswirkungen (Code, API, Datenmodell, Smart Contracts, Blockchain-Protokoll, Datenbank, KI-Agenten, Konfiguration, Dokumentation); indirekte Auswirkungen (abhängige Repositories, andere Standards, Wallet, Node, Miner, Marketplace, Bridges, SDKs, Frontend, Mobile Apps, externe Integrationen); Governance-Auswirkungen (andere Standards, Rollen, Compliance, Auditierbarkeit, Sicherheitsmodell, Benutzerrechte).

### REQ-UPD-006 — Dependency Check

id: REQ-UPD-006

Ein Update gilt NICHT als vollständig, solange abhängige Artefakte nicht überprüft wurden. Wird ein Standard geändert (z. B. Token-Standard), MÜSSEN alle abhängigen Komponenten (Contract, Wallet, Explorer, Marketplace, SDK, Documentation) gegen die neue Version geprüft werden. Abhängigkeitsgraph: registry/dependencies.yaml (DAG).

### REQ-UPD-007 — Source-of-Truth und Synchronisationspflicht

id: REQ-UPD-007

Für jedes Artefakt MUSS eine kanonische Quelle definiert sein (Standard Source → Git Repository → Release → Documentation → Wiki). Nach einem Update MUSS die Synchronisationspflicht erfüllt sein: Standard, README, CHANGELOG, Wiki, Code (falls betroffen), Tests, API-Doku (falls betroffen), Architektur (falls betroffen), Smart Contracts (falls betroffen), KI-Agenten (falls betroffen), Registry, Version Registry — je ✓/begründet. NICHT ERLAUBT: Wiki geändert, Code nicht geändert, System inkonsistent.

### REQ-UPD-008 — Update-Gates

id: REQ-UPD-008

Ein Update DARF nur veröffentlicht werden, wenn die Gates erfüllt sind: UPD-G01 Validity (fachlich begründet) · UPD-G02 Impact (alle Auswirkungen bekannt) · UPD-G03 Security (keine neuen Risiken) · UPD-G04 Compatibility (Rückwärtskompatibilität; bei MAJOR-Updates verpflichtend als vollständiges Verfahren nach ATC-STD-COMPAT-001 — ATC-MAJOR-COMPATIBILITY-GATE) · UPD-G05 Testing (Tests erfolgreich) · UPD-G06 Documentation (synchron) · UPD-G07 Audit (nachvollziehbar/reproduzierbar) · UPD-G08 Approval (zuständige Instanz) · UPD-G09 Release (Version eindeutig identifiziert).

### REQ-UPD-009 — Statusmodell

id: REQ-UPD-009

Updates DURCHLAUFEN: DRAFT → REQUESTED → ANALYSIS → APPROVED → IN_PROGRESS → TESTING → AUDIT → RELEASE_CANDIDATE → RELEASED → VERIFIED → CLOSED. Fehlerpfad: ANY → REJECTED → REWORK → ANALYSIS. Notfallpfad: INCIDENT → EMERGENCY_UPDATE → HOTFIX → VALIDATION → POST_UPDATE_AUDIT.

### REQ-UPD-010 — Rollback-Standard

id: REQ-UPD-010

Jedes kritische Update BENÖTIGT vor dem Release einen definierten Rollback-Plan (trigger_conditions, previous_version, rollback_procedure, rollback_owner, verification_required). Für Blockchain-Systeme MUSS zwischen Software-Rollback, State-Rollback, Daten-Rollback, Smart-Contract-Migration und Netzwerk-/Consensus-Migration unterschieden werden — ein Blockchain-Update DARF NICHT nach dem Muster einer normalen Webanwendung zurückgerollt werden. (Versions-/Release-Ebenen: ATC-STD-VERSION-001.)

### REQ-UPD-011 — Emergency Update

id: REQ-UPD-011

Für kritische Sicherheitsprobleme ist ein verkürzter Prozess zugelassen: DETECT → CLASSIFY → CONTAIN → EMERGENCY PATCH → TEST → APPROVE → DEPLOY → VERIFY → POST-MORTEM. Der verkürzte Prozess DARF NICHT bedeuten, dass Auditierbarkeit entfällt. Nach einem Emergency Update ist VERPFLICHTEND: Post-Update Audit + Root Cause Analysis (ATC-STD-BUG-005) + CHANGELOG-Eintrag + Incident Record + Security Review. (Incident-Infrastruktur: geplant ATC-STD-INC-001.)

### REQ-UPD-012 — AI-Agent Update-Regel

id: REQ-UPD-012

Ein KI-Agent DARF KEINE normative ATC-Standardänderung eigenständig als verbindlich veröffentlichen. Der Agent DARF: Analyse, Vorschlag, Implementierung, Test, Audit-Vorbereitung, Dokumentation. Er MUSS zwischen PROPOSED und APPROVED unterscheiden (approved_by: human_or_authorized_governance). (Human Approval Gates: ATC-STD-AI-DECISION-001 REQ-AIDEC-013; Agent Change: ATC-AAS-009; Human Approval: ATC-AAS-017.)

### REQ-UPD-013 — Update Manifest

id: REQ-UPD-013

Für jedes Release SOLL ein maschinenlesbares Manifest existieren: update_id, artifact (id/previous_version/new_version), classification (type/risk), change (summary/breaking_change), validation (tests/security/audit/documentation), dependencies (checked/affected), approval (status/approved_by), release (status/released_at), rollback (available). (Kopplung: Release-Manifest und Release-/Build-IDs gemäß ATC-STD-VERSION-001; Update Manifest = releasebegleitende Erweiterung.)

### REQ-UPD-014 — CHANGELOG-Pflicht

id: REQ-UPD-014

Jedes veröffentlichte Update MUSS einen CHANGELOG-Eintrag besitzen (Added/Changed/Fixed/Security/Breaking Changes/Migration/Audit). Das Detailformat regelt der geplante Standard ATC-STD-CHANGELOG-001 (eigene SCR); bis dahin gilt das etablierte Format der Repository-CHANGELOGs.

### REQ-UPD-015 — Update Audit

id: REQ-UPD-015

Der Auditor MUSS mindestens die 15 Update-Audit-Fragen beantworten: Was/Warum geändert? Wer initiiert/implementiert? Welche Dateien/Standards/Abhängigkeiten betroffen? Welche Tests/Sicherheitsprüfungen? Wer freigegeben? Vorherige/aktuelle Version? Dokumentation synchron? Rollback möglich? Update reproduzierbar? (Evidence-Kette: ATC-STD-AUDIT-001 REQ-AUDIT-026.)

### REQ-UPD-016 — Update Integrity

id: REQ-UPD-016

Für kritische Releases SOLL zusätzlich ein Integrity Manifest erzeugt werden: Git Commit, Version, Build, Tests, Audit, Dependencies — als überprüfbare Kette CHANGE REQUEST → SOURCE CHANGE → COMMIT → BUILD → TEST → AUDIT → RELEASE → DEPLOYMENT. (Build-/Release-IDs: ATC-STD-VERSION-001.)

### REQ-UPD-017 — Regelmäßiger Standard-Review

id: REQ-UPD-017

Jeder ATC-Standard MUSS regelmäßig überprüft werden (anlassbezogen zusätzlich): Critical Standard mind. jährlich · High-Risk Standard mind. jährlich · Normal Standard alle 2 Jahre · Low-Risk Documentation alle 2–3 Jahre. ATC-Governance-Regel (Review-Kadenz als `review_date` im Standard-Frontmatter führen). Systematische Review-Verfahren sind etablierte Standardisierungspraxis (z. B. sieht ISO für International Standards grundsätzlich max. 5 Jahre bis zur systematischen Überprüfung vor) — die ATC-Kadenzen sind eigenständig und strenger.

### REQ-UPD-018 — Update Control Matrix

id: REQ-UPD-018

Der Kontrolaufwand MUSS der Änderungsart entsprechen (✓ = erforderlich, ✓✓ = doppelt/verstärkt): Tippfehler (Review ✓, Approval: Owner) · Dokumentationsänderung (Review ✓, Audit ✓, Owner) · Code PATCH (Review/Test/Audit ✓, Maintainer) · API MINOR (✓, Technical Lead) · API MAJOR (✓, Governance) · Smart Contract (✓✓, Governance) · Consensus (✓✓, Governance) · Security Critical (✓✓, Emergency Authority) · Standard MAJOR (✓✓, ggf. Test, ✓✓ Audit, Standards Authority). Rollen gemäß ATC-STD-000 §14.1 Rollenmodell (Owner alleiniger Approver normativer Standards).

### REQ-UPD-019 — SCR-Kopplung

id: REQ-UPD-019

Für normative Änderungen approbierter Standards bleibt ATC-STD-000 §30 verbindlich: Änderungen NUR via SCR (SCR-NNNN). Der Update Request `UPD-NNN` koppelt daran: UPD-NNN MUSS bei normativen Standardänderungen auf das zugehörige SCR verweisen; SCR deckt Governance-Review und Freigabe, UPD-NNN deckt Release-Tracking, Manifest und Synchronisation. Keine Doppelspur: SCR ersetzt UPD nicht, UPD ersetzt SCR nicht.

## 5. Change-Control-Kette (Einordnung)

```
ATC ENTERPRISE GOVERNANCE (tatsächlicher Bestand, Stand 07.09.2026)
│
├── ATC-STD-000          Standards Governance (SCR, §30-Immutabilität)
├── ATC-STD-VERSION-001  Versionierung (SemVer, Release-/Build-IDs, Manifeste)
├── ATC-STD-UPDATE-001   Change Control für Updates (DIESER STANDARD)
├── ATC-STD-AUDIT-001    Completeness & Audit (Kontrollschicht)
├── ATC-STD-BUG-001..005 Bug-Familie (Finding/Lifecycle/Analyse)
├── ATC-STD-AI-DECISION-001  Agent Decision-Making (Human Gates)
├── ATC-AAS-001..025     Agentenbetrieb (inkl. AAS-009 Change, AAS-022 Versioning)
├── ATC-STD-201..204     Repository-Standards
├── ATC-STD-README/MD-001  Dokumentations-Standards
└── Geplant: ATC-STD-CHANGELOG-001 · ATC-STD-RELEASE-001 · ATC-STD-INC-001 · ATC-STD-REG-001
```

> Korrektur gegenüber dem Owner-Entwurf: ATC-STD-ERROR-001 wurde als ATC-STD-BUG-005 in die Bug-Familie integriert (SCR-0012); ATC-STD-AGENT-001 ist die AAS-Familie; ATC-STD-REPOSITORY-001 ist der Legacy-Name (heute 201–204); ATC-STD-CODING-001/CHANGE-001 sind geplante Standards. CHANGE, UPDATE, VERSION, AUDIT und RELEASE BLEIBEN separate, miteinander verknüpfte Standards — damit entsteht eine belastbare Change-Control-Kette.

## 6. Compliance / Prüfungen

### COM-UPD-001

id: COM-UPD-001

Jedes relevante Update MUSS einen UPD-NNN-Request mit allen Pflichtfeldern (REQ-UPD-004) haben — No Silent Update.

### COM-UPD-002

id: COM-UPD-002

Impact Analysis (REQ-UPD-005) und Dependency Check (REQ-UPD-006) MÜSSEN vor Approval dokumentiert sein.

### COM-UPD-003

id: COM-UPD-003

Vor Release MÜSSEN die Gates UPD-G01..UPD-G09 erfüllt sein; MAJOR/Consensus/Smart-Contract-Updates erfordern verstärkte Review/Audit (Control Matrix, REQ-UPD-018).

### COM-UPD-004

id: COM-UPD-004

Die Synchronisationspflicht (REQ-UPD-007) MUSS je Release erfüllt und nachgewiesen sein (Standard/README/CHANGELOG/Wiki/Registry/Version Registry).

### COM-UPD-005

id: COM-UPD-005

Kritische Updates MÜSSEN vor Release einen Rollback-Plan (REQ-UPD-010) haben; Blockchain-Updates MÜSSEN die fünf Rollback-Typen unterscheiden.

### COM-UPD-006

id: COM-UPD-006

Emergency Updates MÜSSEN Post-Update-Audit + RCA + Incident Record + Security Review nachholen (REQ-UPD-011) — Auditierbarkeit entfällt nie.

## 7. Security Considerations

Update-Manifeste DÜRFEN keine Secrets enthalten. Emergency-Prozesse dürfe Approval-Pflichten verkürzen (APPROVE bleibt Pflicht), aber NIE Auditierbarkeit. Rollback-Verfahren für Consensus-/Smart-Contract-Änderungen sind sicherheitskritisch und MÜSSEN vor Release getestet sein. KI-Agenten-Publish-Rechte: kein autonomes normatives Publishing (REQ-UPD-012; ATC-AAS-014 Security).

## 8. Ausnahmen

Ausnahmen MÜSSEN gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC-Verfahren) dokumentiert und durch den Owner genehmigt werden.

## 9. Referenzen (References)

### NORMATIVE Referenzen

- ATC-STD-000 — Standards Governance (SCR, §30, §14.1 Rollen)
- ATC-STD-VERSION-001 — Versioning (SemVer-Kopplung, Release-/Build-IDs, Release-Manifest)
- ATC-STD-AUDIT-001 — Completeness & Audit (Evidence, Audit Trail AUD-YYYY-NNNN)
- ATC-STD-BUG-005 — Fehleranalyse & RCA (Post-Morttem-Pflichten)
- ATC-STD-AI-DECISION-001 — Agent Decision-Making (Human Approval Gates)
- ATC-AAS-009 — Agent Change Standard · ATC-AAS-017 — Human Approval · ATC-AAS-022 — Agent Versioning
- ATC-STD-DESC-001 — Standard Description Standard

### INFORMATIVE Referenzen

- registry/dependencies.yaml — Abhängigkeitsgraph (Dependency-Check-Basis)
- Geplante Folge-Standards: ATC-STD-CHANGELOG-001 (CHANGELOG-Format), ATC-STD-RELEASE-001 (Release-Process), ATC-STD-INC-001 (Incident Management), ATC-STD-REG-001 (Regression Testing) — je eigene SCR + ID-Allokation
- ISO/IEC-Praxis: systematische Reviews für International Standards (max. 5 Jahre) — informativ; ATC-Kadenzen eigenständig

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Keine Zugangsdaten in Artefakten; Security-Review-Pflicht bei sicherheitsrelevanten Aenderungen (ATC-STD-203).

## Changelog

### 1.0.0 — 2026-09-07
- Initial Release (Owner-Entwurf Michael Wroblewski 22:40, harmonisiert mit VERSION-001, AUDIT-001, BUG-005, AI-DECISION-001, AAS-009/017/022, DESC-001)
- 19 normative Anforderungen (REQ-UPD-001..019), 6 COM-UPD-Gates
- Update-Kategorien, 13-stufiger Lifecycle, UPD-NNN-Requests, UPD-G01..G09-Gates, Statusmodell (Normal-/Fehler-/Notfallpfad), Rollback (5 Blockchain-Typen), Emergency-Prozess, Update Manifest, CHANGELOG-Pflicht, Update Audit (15 Fragen), Integrity Manifest, Review-Kadenzen, Control Matrix
- SCR-0015: UPD↔SCR-Kopplung (keine Doppelspur), Hierarchie auf echten Bestand korrigiert, Gates UPD-G vs AUD-G abgegrenzt
- Status APPROVED — §9-Freigabe Michael Wroblewski 07.09.2026, 23:28 UTC+2, normativ, §30-eingefroren

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203 · INFORMATIVE: Registry-SSOT standards.yaml

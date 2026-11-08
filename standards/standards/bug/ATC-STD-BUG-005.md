---
standard:
  id: ATC-STD-BUG-005
  title: "ATC-STD-BUG-005 — Fehleranalyse- und Root-Cause-Analysis-Standard"
  version: "1.0.0"
  status: approved
  category: bug
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
    - ATC-STD-BUG-001
    - ATC-STD-BUG-003
  related_standards:
    - ATC-STD-DESC-001
    - ATC-STD-VERSION-001
    - ATC-STD-BUG-002
    - ATC-STD-BUG-004
  requirements:
    - REQ-STD-141
    - REQ-STD-142
    - REQ-STD-143
    - REQ-STD-144
    - REQ-STD-145
    - REQ-STD-146
    - REQ-STD-147
    - REQ-STD-148
    - REQ-STD-149
    - REQ-STD-150
    - REQ-STD-151
    - REQ-STD-152
    - REQ-STD-153
    - REQ-STD-154
    - REQ-STD-155
    - REQ-STD-156
    - REQ-STD-157
    - REQ-STD-158
    - REQ-STD-159
    - REQ-STD-160
    - REQ-STD-161
    - REQ-STD-162
    - REQ-STD-163
    - REQ-STD-164
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-BUG-005 — Fehleranalyse- und Root-Cause-Analysis-Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0, Owner-Sammelfreigabe 07.09.2026 22:25 UTC+2, ATC-STD-000 §9) — normativ in Kraft, §30-eingefroren (Änderungen nur via SCR). Owner-Entwurf 07.09.2026, 22:06 UTC+2;
> Ursprünglich als ATC-STD-ERR-001 entworfen (SCR-0011), per Owner-Entscheid „ERR in Bug einarbeiten"
> als ATC-STD-BUG-005 in die Bug-Familie integriert (SCR-0012, Builder-Chat 07.09.2026, 22:35 UTC+2).
> **Familie:** Bug & Konsistenz-Lebenszyklus (ATC-STD-BUG-001..499) — Kategorie `bug`, SCR-0011/0012.
> Mit der Sammelfreigabe 22:25 UTC+2: APPROVED/ACTIVE.

> Einheitlicher Prozess für Fehleranalyse und Root-Cause-Analysis im gesamten Ökosystem: Fehler erkennen, reproduzieren, klassifizieren, Auswirkungen bestimmen, Ursache ermitteln, Ursache von Symptomen trennen, beheben, verifizieren, Regression ausschließen, Dokumentation synchronisieren, Lessons Learned erfassen, Wiederholung verhindern. Dieser Standard ist das Analyse- und QMS-Mitglied der Bug-Familie (ATC-STD-BUG-001..004 regeln Finding, Dokumentation, Fix-Lifecycle und Merge-Gate; dieser Standard regelt Analyse-Tiefe und Quality Management) — ein vollständiger Defect & Incident Management Lifecycle in einer Familie.

## 1. Zweck (Purpose)

Der Standard definiert einen einheitlichen Prozess für:

1. Fehler erkennen
2. Fehler reproduzieren
3. Fehler klassifizieren
4. Auswirkungen bestimmen
5. Ursache ermitteln
6. Ursache von Symptomen trennen
7. Fehler beheben
8. Behebung verifizieren
9. Regression ausschließen
10. Dokumentation synchronisieren
11. Lessons Learned erfassen
12. Wiederholung des Fehlers verhindern

**Grundprinzip:**

> Kein Fehler gilt als vollständig bearbeitet, solange Ursache, Auswirkung, Behebung und Verifikation nicht nachvollziehbar dokumentiert sind.

## 2. Geltungsbereich (Scope)

**Gilt:**

- Gesamtes A-TownChain-Ökosystem
- Code-, Architektur-, Sicherheits-, Dokumentations-, KI-Agenten-, Infrastruktur- und Governance-Fehler
- Alle Fehlerklassen gemäß Abschnitt 4
- KI-Agenten-erzeugte Änderungen und deren Analyse

**Nicht im Geltungsbereich (Nicht-Gilt):**

- Finding-Erfassung, -Dokumentation und Fix-Lifecycle im Detail (ATC-STD-BUG-001..003)
- Repository-Synchronisation und Merge-Gates (ATC-STD-BUG-004)
- Ticket-Priorisierung und Sprint-Planung

## 3. Begriffe und Definitionen

| Begriff | Definition |
|---|---|
| Finding | Registrierter Fehler mit F-NNN-ID (ATC-STD-BUG-001) |
| Fehlerklasse | WAS kaputt ist (ERR-CL-*, Abschnitt 4) |
| Severity | WIE schwerwiegend der Fehler ist (S0–S4, ATC-STD-BUG-001 REQ-STD-104) |
| Symptom | Sichtbare Auswirkung (Ebene 1) |
| Root Cause | Fundamentale Ursache, die den Fehler ermöglicht hat (Ebene 3) |
| Systemische Ursache | Warum der Fehler nicht früher erkannt wurde (Ebene 4) |
| Corrective Action | Behebt den aktuellen Fehler |
| Preventive Action | Verhindert, dass derselbe Fehlertyp erneut entsteht |

## 4. Normative Anforderungen

Normative Begriffe gemäß ATC-STD-DESC-001 (MUSS / DARF NICHT / SOLL / KANN).

### REQ-STD-141 — Grundprinzip

id: REQ-STD-141

Kein Fehler DARF als vollständig bearbeitet gelten, solange Ursache, Auswirkung, Behebung und Verifikation nicht nachvollziehbar dokumentiert sind. Ein Fehler MUSS den 12-Schritte-Prozess (Abschnitt 1) durchlaufen.

### REQ-STD-142 — Fehlerklassen

id: REQ-STD-142

Jeder Fehler MUSS mindestens eine primäre Fehlerklasse (ERR-CL-*) erhalten. Zulässige Klassen:

| ID | Klasse | Beispiele |
|---|---|---|
| ERR-CL-CODE | Codefehler | Exception, falsche Logik |
| ERR-CL-BUILD | Buildfehler | Compilation, Dependency |
| ERR-CL-TEST | Testfehler | fehlender/falscher Test |
| ERR-CL-SEC | Sicherheitsfehler | Vulnerability |
| ERR-CL-SC | Smart-Contract-Fehler | Contract Logic |
| ERR-CL-CONS | Konsensfehler | Node-/Consensus-Abweichung |
| ERR-CL-NET | Netzwerkfehler | P2P, RPC |
| ERR-CL-DATA | Datenfehler | Korruption, Inkonsistenz |
| ERR-CL-DB | Datenbankfehler | Firestore/SQL etc. |
| ERR-CL-API | API-Fehler | Contract/API mismatch |
| ERR-CL-UI | UI/UX-Fehler | Rendering, Interaction |
| ERR-CL-AI | KI-Fehler | falsche Agentenentscheidung |
| ERR-CL-AGENT | KI-Agentenfehler | falscher Workflow |
| ERR-CL-DOC | Dokumentationsfehler | Wiki ≠ Code |
| ERR-CL-CONFIG | Konfigurationsfehler | falsche Parameter |
| ERR-CL-INFRA | Infrastrukturfehler | Docker/K8s/Cloud |
| ERR-CL-DEP | Dependency-Fehler | externe Bibliothek |
| ERR-CL-GOV | Governancefehler | Prozessverletzung |

### REQ-STD-143 — Severity

id: REQ-STD-143

Die Severity MUSS gemäß ATC-STD-BUG-001 REQ-STD-104 (S0 Critical bis S4 Informational) bestimmt werden. Fehlerklasse (WAS) und Severity (WIE SCHWER) SIND getrennt zu bestimmen. S0 (Existenzbedrohung: Verlust von Assets, Konsensbruch, kritische Sicherheitslücke, Private-Key-Kompromittierung, unkontrollierte Token-Erzeugung, Mainnet-Ausfall) erfordert SOFORTIGE Eskalation.

### REQ-STD-144 — Statusmaschine

id: REQ-STD-144

Der Fehler-Status MUSS konsistent zum Fix-Lifecycle aus ATC-STD-BUG-003 (REQ-STD-121, 12 Stufen) sein. Die Analyse-Phasen verfeinern die Stufe ANALYZED wie folgt:

```
DETECTED → TRIAGED → REPRODUCING → ANALYZING → ROOT_CAUSE_IDENTIFIED
→ FIX_PLANNED → FIXING → FIXED → VERIFYING → REGRESSION_CHECK
→ DOCUMENTATION_SYNC → CLOSED          (BUG-003: ANALYZED … VERIFIED/CLOSED)
```

Alternative Pfade: TRIAGED → DUPLICATE / TRIAGED → INVALID; ANALYZING → CANNOT_REPRODUCE; VERIFYING → REOPENED; REGRESSION_CHECK → REOPENED. Stufen DÜRFEN NICHT übersprungen werden (BUG-003 REQ-STD-122: Ausnahme nur S4 ohne Verhaltensänderung, begründet).

### REQ-STD-145 — Fehler-IDs

id: REQ-STD-145

Fehler-IDs MÜSSEN gemäß ATC-STD-BUG-001 REQ-STD-106 als F-NNN (fortlaufend, immutable, kanonisch `registry/findings.yaml`) vergeben werden. Sicherheits-, Smart-Contract- und KI-Fehler erhalten KEINE separaten ID-Serien (konsolidierte Entscheidung SCR-0011) — sie werden über die Fehlerklasse (ERR-CL-SEC, ERR-CL-SC, ERR-CL-AI/ERR-CL-AGENT) abgebildet. Reproduktionstests nutzen TEST-NNN (BUG-001).

### REQ-STD-146 — Pflichtinformationen

id: REQ-STD-146

Jeder Fehlerdatensatz MUSS die Pflichtfelder aus ATC-STD-BUG-001 REQ-STD-103 erfüllen (Repository, Branch/Commit, Datei, Komponente, Beschreibung, Reproduktionsschritte, erwartetes/tatsächliches Verhalten, Severity) und zusätzlich die Analyse-Felder: detected_at, detected_by, environment, impact, affected_components, root_cause, contributing_factors, fix, verification, regression_test, documentation_impact, related_issues, closed_at, closed_by.

### REQ-STD-147 — Vier-Ebenen-Analyse

id: REQ-STD-147

Jede Fehleranalyse MUSS vier Ebenen unterscheiden:

| Ebene | Frage | Beispiel (Wallet-Balance) |
|---|---|---|
| 1 — Symptom | Was sieht man? | Wallet zeigt falschen ATC-Bestand |
| 2 — Unmittelbare Ursache | Warum passiert das? | Balance wird aus veraltetem Cache gelesen |
| 3 — Root Cause | Warum konnte der Fehler entstehen? | Cache-Invalidierung ist nicht an den bestätigten Blockchain-State gekoppelt |
| 4 — Systemische Ursache | Warum wurde der Fehler nicht früher erkannt? | Kein automatisierter Konsistenztest zwischen Wallet-Cache und Chain-State |

Die Fehlerbehebung DARF NICHT beim Symptom enden.

### REQ-STD-148 — Five Whys

id: REQ-STD-148

Die Five-Whys-Technik MUSS als Standard-RCA-Methode angewendet werden, bis die Root Cause eine architectonische/systemische Ebene erreicht (Beispiel Abschnitt 5.1).

### REQ-STD-149 — Fault Tree Analysis

id: REQ-STD-149

Bei S0-Fehlern und ausgewählten S1-Fehlern MUSS eine Fault-Tree-Analyse (System Failure: Component / Configuration / Dependency / Human-Agent Error / Security Event / Data Integrity) durchgeführt werden.

### REQ-STD-150 — Reproduzierbarkeit

id: REQ-STD-150

Jeder Fehler MUSS mit einem Reproduzierbarkeitsgrad klassifiziert werden: R0 (nicht reproduzierbar), R1 (selten), R2 (reproduzierbar), R3 (deterministisch). Für S0/S1 ist eine reproduzierbare Testumgebung ANZUSTREBEN.

### REQ-STD-151 — Evidence Standard

id: REQ-STD-151

Eine Fehleranalyse DARF NICHT ausschließlich auf Behauptungen basieren. Zulässige Evidence: Log, Stack Trace, Screenshot, Video, Test Output, Transaction Hash, Block Height, Commit SHA, Build-ID (ATC-STD-VERSION-001), Docker Image, Configuration, Metrics, Trace, Database Record, AI Agent Log, Audit Log. Für kritische Fehler (S0/S1) MUSS Evidence immutable und referenzierbar sein.

### REQ-STD-152 — Zeitliche Analyse

id: REQ-STD-152

Bei komplexen Fehlern (S0/S1, R0/R1) MUSS eine Timeline dokumentiert werden (T0 Deployment, T1 Configuration Change, T2 First Warning, T3 First Error, T4 User Impact, T5 Detection, T6 Mitigation, T7 Fix, T8 Verification, T9 Closure), um Trigger, Fehlerentstehung und Fehlererkennung zu unterscheiden.

### REQ-STD-153 — Impact Analysis

id: REQ-STD-153

Jeder Fehler MUSS eine Auswirkungsanalyse über die betroffenen Ebenen (Repository → Service → Application → Blockchain → Smart Contract → Wallet → User → Ecosystem) und Dimensionen (Verfügbarkeit, Integrität, Vertraulichkeit, Performance, finanzielle Auswirkungen, Sicherheitsauswirkungen, Governance-Auswirkungen) erhalten.

### REQ-STD-154 — Fix-Standard

id: REQ-STD-154

Ein Fix MUSS der Kette aus ATC-STD-BUG-003 folgen (RCA → Fix → Unit Test → Integration Test → Regression Test → Security Check → Documentation Sync → Audit → CLOSED) und dabei die Root Cause adressieren. NICHT akzeptabel: „Bug → Code geändert → funktioniert lokal → CLOSED".

### REQ-STD-155 — Regression-Standard

id: REQ-STD-155

Für jeden behobenen Fehler MUSS die Regression-Entscheidung dokumentiert werden (`regression_test: required/type: unit | integration | e2e | security | manual`). Bei wiederkehrenden Fehlern MUSS der Regressionstest als automatisierter CI-Test Bestandteil des dauerhaften Qualitätssystems werden.

### REQ-STD-156 — Dokumentations-Synchronisation

id: REQ-STD-156

Nach einem Fix MÜSSEN Code, Tests, README, Wiki, Architecture, Specification, API-Dokumentation, CHANGELOG und ROADMAP auf Konsistenz geprüft werden (Merge-Gate: ATC-STD-BUG-004). Wenn sich die Implementierung gegenüber einem Standard verändert, MUSS eine Documentation Impact Analysis mit Standard-/Wiki-Update und Review erfolgen.

### REQ-STD-157 — KI-Agenten-Fehleranalyse

id: REQ-STD-157

Für von KI-Agenten erkannte, verursachte oder behobene Fehler MÜSSEN agent_id, agent_version, model, task_id, decision, action_taken, tools_used, repositories_accessed, files_changed, validation_performed, human_approval_required, human_approval und agent_confidence gespeichert werden. Damit MUSS später beantwortet werden können: Welcher Agent hat was erkannt, analysiert, geändert und wie wurde die Änderung verifiziert? (Agentenbetrieb: ATC-AAS-001..025)

### REQ-STD-158 — Root-Cause-Kategorien

id: REQ-STD-158

Jede ermittelte Root Cause MUSS für Analytics standardisiert klassifiziert werden: RC-CODE, RC-ARCH, RC-DATA, RC-CONFIG, RC-DEPENDENCY, RC-SECURITY, RC-TEST, RC-DOCUMENTATION, RC-INFRA, RC-HUMAN, RC-AI, RC-PROCESS, RC-GOVERNANCE.

### REQ-STD-159 — Error Metrics

id: REQ-STD-159

Das ATC Quality System SOLL mindestens messen: MTTD (Mean Time To Detect), MTTA (Mean Time To Acknowledge), MTTR (Mean Time To Repair), MTTV (Mean Time To Verify), Reopen Rate, Regression Rate, Critical Error Count (S0/S1), Root Cause Rate, Automation Rate, Detection Coverage.

### REQ-STD-160 — Closure Gate

id: REQ-STD-160

Ein Fehler DARF nur auf CLOSED gesetzt werden, wenn das Closure Gate vollständig erfüllt ist: Fehler identifiziert, Severity bestimmt, Impact analysiert, Ursache bestimmt, Fix implementiert, Test vorhanden, Regression geprüft, Security geprüft (falls relevant), Dokumentation synchronisiert, CHANGELOG geprüft, Review abgeschlossen, Evidence vorhanden. Für S0 ZUSÄTZLICH: Incident Review, Root Cause Analysis, Corrective Action, Preventive Action, Management-/Owner-Approval. (DoD-Grundlage: ATC-STD-BUG-003 REQ-STD-126.)

### REQ-STD-161 — Corrective vs. Preventive Action

id: REQ-STD-161

ATC MUSS zwischen Corrective Action (behebt den aktuellen Fehler) und Preventive Action (verhindert, dass derselbe Fehlertyp erneut entsteht) unterscheiden. Beispiel: Corrective — Cache-Bug beheben; Preventive — automatischen Chain-State-vs-Cache-Konsistenztest einführen.

### REQ-STD-162 — S0-Incident-Review

id: REQ-STD-162

Bei S0-Fehlern MUSS ein Incident Review mit Owner-Approval erfolgen (REQ-STD-160-S0-Zusatzgates).

### REQ-STD-163 — Wiederholungsverhinderung

id: REQ-STD-163

Bei wiederkehrenden Fehlermustern (gleiche RC-Kategorie ≥ 3×) MUSS eine Preventive Action definiert und umgesetzt sein, bevor der letzte zugehörige Fehler CLOSED wird.

### REQ-STD-164 — Analytics

id: REQ-STD-164

Root-Cause-Klassifikationen MÜSSEN statistisch auswertbar erfasst werden (Fehlerklasse × Severity × RC-Kategorie × Detection Source), damit wiederkehrende Ursachen systematisch erkennbar werden.

## 5. RCA-Techniken

### 5.1 Five Whys (Beispiel)

Problem: Wallet zeigt falsche Balance.

1. Warum? → Cache enthält alten Wert.
2. Warum? → Cache wird nicht nach Blockbestätigung invalidiert.
3. Warum? → Event fehlt.
4. Warum? → Wallet-State-Architektur definiert keinen verbindlichen Confirmation-Event.
5. Warum? → Architekturstandard enthält keine State-Synchronisationsregel.

Root Cause: Fehlende verbindliche State-Synchronisationsarchitektur.

### 5.2 Fault Tree Analysis (bei S0 und ausgewählten S1)

```
SYSTEM FAILURE
      |
      +-- Component Failure
      +-- Configuration Failure
      +-- Dependency Failure
      +-- Human/Agent Error
      +-- Security Event
      +-- Data Integrity Failure
```

## 6. Fehlerstatus und Analysepfade

Statusmaschine gemäß REQ-STD-144: die Analyse-Phasen (DETECTED → TRIAGED → REPRODUCING → ANALYZING → ROOT_CAUSE_IDENTIFIED) verfeinern BUG-003s ANALYZED; FIX_PLANNED → FIXING → FIXED → VERIFYING → REGRESSION_CHECK → DOCUMENTATION_SYNC → CLOSED entsprechen FIX PLANNED → IMPLEMENTED → … → VERIFIED → CLOSED. Alternative Pfade: DUPLICATE, INVALID, CANNOT_REPRODUCE, REOPENED.

## 7. Fehler-IDs und Finding-Registry

Kanonisch: F-NNN gemäß ATC-STD-BUG-001 REQ-STD-106 (`registry/findings.yaml`, immutable). Spezialklassen werden über ERR-CL-SEC / ERR-CL-SC / ERR-CL-AI abgebildet, NICHT über separate ID-Serien (SCR-0011/0012-Harmonisierung). Reproduktionstests: TEST-NNN.

## 8. Pflichtinformationen eines Findings

Gemäß REQ-STD-146: BUG-001-Pflichtfelder + Analyse-Erweiterung (root_cause, contributing_factors, timeline, evidence, impact, regression_test, documentation_impact, agent-Metadaten bei KI-Beteiligung).

## 9. Evidence und Reproduzierbarkeit

Evidence-Arten gemäß REQ-STD-151; Reproduzierbarkeitsgrade R0–R3 gemäß REQ-STD-150. Kritische Fehler: Evidence MUSS immutable/referenceable sein (Commit SHA, Build-ID nach ATC-STD-VERSION-001, Block Height, Transaction Hash).

## 10. Impact-Analyse

Ebenen: Repository → Service → Application → Blockchain → Smart Contract → Wallet → User → Ecosystem. Dimensionen: Verfügbarkeit, Integrität, Vertraulichkeit, Performance, finanzielle / sicherheitsrelevante / governance-relevante Auswirkungen.

## 11. Fix- und Regression-Standard

Fix-Kette gemäß BUG-003 (REQ-STD-154). Regression-Entscheidung je Fehler (REQ-STD-155); wiederkehrende Fehler → automatisierter CI-Test.

## 12. Dokumentations-Synchronisation

```
Code ↕ Tests ↕ README ↕ Wiki ↕ Architecture ↕ Specification
↕ API-Dokumentation ↕ CHANGELOG ↕ ROADMAP
```

Implementierungsabweichung vom Standard → Documentation Impact Analysis → Standard-/Wiki-Update → Review. Merge-Gate: ATC-STD-BUG-004.

## 13. KI-Agenten-Metadaten

Struktur gemäß REQ-STD-157; Agentenbetrieb-Referenz: ATC-AAS-001..025 (Agent-Identität, Versionierung: ATC-STD-VERSION-001 §16).

## 14. Error Metrics

KPI-Set gemäß REQ-STD-159 (MTTD/MTTA/MTTR/MTTV, Reopen/Regression Rate, Critical Error Count, Root Cause Rate, Automation Rate, Detection Coverage).

## 15. Corrective vs. Preventive Action

Gemäß REQ-STD-161/023. Der Unterschied zwischen einfacher Bug-Behebung und professionellem Quality Management System besteht genau darin: Corrective behebt den Einzelfall, Preventive schließt die Fehlerklasse systematisch.

## 16. Einordnung in die Standard-Hierarchie

```
ATC Enterprise Standards
├── Governance Standards
├── Software Development Standards (Coding, Repository, Versioning, CHANGELOG, Testing)
├── Quality Standards
│   └── Bug & Konsistenz-Lebenszyklus (EINE Familie): Finding BUG-001 · Documentation BUG-002 ·
│       Fix Lifecycle BUG-003 · Merge Gate BUG-004 · Error Analysis/RCA BUG-005 (DIESER STANDARD)
│       (geplante Erweiterungen: Regression REG-001 · Incident INC-001 · Audit AUDIT-001 — je eigene SCR)
├── Security Standards
├── AI Agent Standards (ATC-AAS-001..025, ATC-ENT-001..015)
├── Blockchain Standards (ATC-STD-SC-001..020)
└── Documentation Standards (ATC-STD-MD-001, ATC-STD-README-001)
```

Fehleranalyse und Bugmanagement sind gemäß Owner-Entscheid (07.09., SCR-0012) in EINER Familie integriert: BUG-001..004 = Finding/Lifecycle/Docs/Merge-Gate; BUG-005 = Analyse/RCA/QMS.

## 17. Compliance / Prüfungen

### COM-BUG-501

id: COM-BUG-501

Fehlerklasse MUSS aus der 18-Klassen-Enumeration stammen (ERR-CL-*, automatisierbar gegen Schema `errorClassId`).

### COM-BUG-502

id: COM-BUG-502

Bei S0/S1 MÜSSEN alle vier Analyse-Ebenen dokumentiert sein (Symptom, unmittelbare Ursache, Root Cause, systemische Ursache).

### COM-BUG-503

id: COM-BUG-503

Evidence MUSS vorliegen (nicht nur Behauptungen); S0/S1: immutable und referenzierbar.

### COM-BUG-504

id: COM-BUG-504

Ab Status ROOT_CAUSE_IDENTIFIED MUSS eine RC-Kategorie (RC-*) gesetzt sein.

### COM-BUG-505

id: COM-BUG-505

Closure Gate MUSS vollständig geprüft sein (12 Kriterien; S0: +5 Zusatzgates inkl. Owner-Approval) vor CLOSED.

### COM-BUG-506

id: COM-BUG-506

Regressionstest-Entscheidung (required/type) MUSS dokumentiert sein; wiederkehrende Muster: CI-Test vorhanden.

## 18. Security Considerations

Sicherheitsfehler (ERR-CL-SEC) mit S0/S1 MÜSSEN vor Disclosure-Entscheidungen klassifiziert werden. Private-Key-Kompromittierung erfordert SOFORT-Eskalation. KI-Agenten-Metadaten sind Evidence-relevant und DÜRFEN nicht nachträglich verändert werden (Audit-Trail). Evidence-ketten DÜRFEN keine Geheimnisse (Keys, Tokens) im Klartext enthalten.

## 19. Ausnahmen

Ausnahmen von diesem Standard MÜSSEN gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC-Verfahren) dokumentiert und durch den Owner genehmigt werden. BUG-003 REQ-STD-122 (S4-Trivialitätsausnahme) bleibt unberührt.

## 20. Referenzen (References)

### NORMATIVE Referenzen

- ATC-STD-000 — Standards Governance & Specification Standard (Verfassung)
- ATC-STD-BUG-001 — Bug Finding Standard (F-NNN, Pflichtfelder, Severity S0–S4)
- ATC-STD-BUG-002 — Bug Documentation Standard
- ATC-STD-BUG-003 — Bug Fix Lifecycle Standard (12-Stufen-Lifecycle, DoD)
- ATC-STD-BUG-004 — Repository Synchronization & Merge Gate
- ATC-STD-DESC-001 — Standard Description Standard (Pflichtstruktur, Metadaten)
- ATC-STD-VERSION-001 — ATC Versioning Standard (Build-ID, Agenten-Versionierung)

### INFORMATIVE Referenzen

- ATC-AAS-001..025 — AI Agent Standards (Agentenbetrieb)
- schemas/naming-conventions.schema.json — errErrorClassId, errRootCauseCategoryId
- Geplante Folge-Standards: ATC-STD-REG-001 (Regression Testing), ATC-STD-INC-001 (Incident Management), ATC-STD-AUDIT-001 (Completeness & Audit) — je eigene SCR + ID-Allokation erforderlich

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Keine Zugangsdaten in Artefakten; Security-Review-Pflicht bei sicherheitsrelevanten Aenderungen (ATC-STD-203).

## Changelog

### 1.0.0 — 2026-09-07
- Initial Release (Owner-Entwurf Michael Wroblewski, harmonisiert mit ATC-STD-BUG-001..004, ATC-STD-DESC-001, ATC-STD-VERSION-001)
- 24 normative Anforderungen (REQ-STD-141..164), 6 COM-BUG-Gates (COM-BUG-501..506)
- SCR-0011/0012: Konfliktlösung F-NNN vs. BUG-/SEC-/SCBUG-/AIBUG-NNNNN dokumentiert
- Analyse-Schicht über BUG-Lifecycle; Status DRAFT — §9-Freigabe ausstehend

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203 · INFORMATIVE: Registry-SSOT standards.yaml

---
standard:
  id: ATC-AI-GOV-INCIDENT-001
  title: "ATC Agent Governance — Incident & Fehlerbehandlung (Klassen, Lifecycle, RCA, Postmortem)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-005]
  related_standards: [ATC-AI-GOV-AUDIT-001, ATC-STD-IMPROVEMENT-001, ATC-STD-UPDATE-001]
  requirements: [REQ-AGOV-INC-001, REQ-AGOV-INC-002, REQ-AGOV-INC-003, REQ-AGOV-INC-004]
---

# ATC-AI-GOV-INCIDENT-001 — Incident & Fehlerbehandlung (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 09.09.; operativer SSOT: `.github`-Hub `ai/incident.yaml`; §9-Freigabe ausstehend.

## 1. Zweck (Purpose)

Vorfallsklassen, Lifecycle und Pflichten bei agentengetriebenen Fehlern: erfassen statt still beheben, Ursache statt Symptom, Prävention statt Wiederholung.

## 2. Geltungsbereich (Scope)

**Gilt:** Agent-getriebene Incidents (Produktionsstörung, Governance-Verstoß, Security, Datenverlust, Qualitätsvorfall, Prozessvorfall). **Nicht im Gelt:** Finding-Erfassung selbst (BUG-001), RCA-Methodik (BUG-005), Emergency-Change-Pfad (UPDATE-001).

## 3. Normative Anforderungen

### REQ-AGOV-INC-001 — Klassifikation

id: REQ-AGOV-INC-001

Jeder Incident MUSS klassifiziert werden: INC-CLS-PROD (P0) · INC-CLS-GOV (P0) · INC-CLS-SEC (P0) · INC-CLS-DATA (P0) · INC-CLS-QUAL (P1) · INC-CLS-PROC (P1) mit definierter Erstreaktion (Stopp/Rollback/Owner-Benachrichtigung).

### REQ-AGOV-INC-002 — Lifecycle

id: REQ-AGOV-INC-002

DETECTED → CONTAINED → ANALYZED → REMEDIATED → VERIFIED → CLOSED. Jeder Incident MUSS als F-NNNN und/oder ATC-IMP-Record erfasst sein — kein stilles Behandeln.

### REQ-AGOV-INC-003 — Root Cause und Regression

id: REQ-AGOV-INC-003

P0/P1-Incidents erfordern zwingend RCA nach BUG-005 (bis zur systemischen Ursache) + Regression Prevention („Kann derselbe Fehler woanders auftreten?", IMP-Standard REQ-IMP-008) + AUD-Record mit Zeitstempel-Kette.

### REQ-AGOV-INC-004 — Postmortem

id: REQ-AGOV-INC-004

P0/P1-Incidents erhalten eine Postmortem-Dokumentation (docs/incidents/INCIDENT-YYYY-MM-DD-<ref>.md) mit Ursache, Wirkung, Fix, Prävention. Owner-Benachrichtigung bei allen P0-Incidents; Agent dokumentiert, Owner entscheidet.

## 4. Compliance / Prüfungen

id: COM-AGOV-INC-001

COM-AGOV-INC-001: P0-Incident ohne AUD-Record = FAIL. COM-AGOV-INC-002: Postmortem vorhanden je P0/P1. COM-AGOV-INC-003: INC-Klassen konsistent zu F/IMP-Referenzen.

## 5. Security Considerations

Security-Incidents (INC-CLS-SEC): Sofort-Stopp, Owner sofort, keine Inline-Geheimnisse in Postmortems (Verweis auf geschützte Evidence).

## 6. Ausnahmen

Gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung.

## 7. Referenzen (References)

### NORMATIVE Referenzen
- ATC-STD-BUG-001/005 — Findings/RCA · ATC-AI-GOV-AUDIT-001 — AUD-Records · ATC-STD-IMPROVEMENT-001 — Regression Prevention · ATC-STD-UPDATE-001 — Emergency-Pfad

### INFORMATIVE Referenzen
- `.github`-Hub: `ai/incident.yaml` (Klassen, Lifecycle, Rules)

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release (Owner-Entwurf 09.09., SCR-0061): 4 REQ, 3 COM-Gates; operational über ai/incident.yaml (SCR-0061)

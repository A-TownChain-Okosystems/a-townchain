---
standard:
  id: ATC-AI-GOV-HANDOFF-001
  title: "ATC Agent Governance — Agent-to-Agent Übergabe (Handoff-Pflichtfelder, Kontinuität, Auditierbarkeit)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000, ATC-AI-GOV-MANIFEST-001]
  related_standards: [ATC-AI-GOV-INCIDENT-001, ATC-STD-BUG-001]
  requirements: [REQ-AGOV-HOF-001, REQ-AGOV-HOF-002, REQ-AGOV-HOF-003]
---

# ATC-AI-GOV-HANDOFF-001 — Agent-to-Agent Übergabe (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 09.09.; operativer SSOT: `.github`-Hub `ai/handoff.yaml` (Protokoll + Template); §9-Freigabe ausstehend.

## 1. Zweck (Purpose)

Bei Übergabe an einen anderen Agenten geht keine Information verloren; jede Übergabe ist auditierbar.

## 2. Geltungsbereich (Scope)

**Gilt:** Jede Agent-zu-Agent-Task-Übergabe in der A-TownChain-Okosystems. **Nicht im Gelt:** Task-Beginn ohne Vorgänger (Session-Mandat AOS-001), Übergabe an Owner/Menschen (normaler Report).

## 3. Normative Anforderungen

### REQ-AGOV-HOF-001 — Pflichtfelder

id: REQ-AGOV-HOF-001

Jede Übergabe ÜBERTRÄGT mindestens: source_agent, target_agent, task, status, completed_work, changed_files, open_findings, tests, risks, next_action. Ohne vollständigen Record gilt der Task als NICHT übergeben.

### REQ-AGOV-HOF-002 — Kontinuitätsregeln

id: REQ-AGOV-HOF-002

Der Ziel-Agent MUSS den Record vor Fortführung lesen (inspect-before-modify). Scope-Wechsel mit Übergabe = neuer Task + neuer Snapshot. Widerspruch Handoff vs. Repository-Realität → Repository gewinnt, Differenz = Finding. Offene Findings NUR mit Auflösung oder bewusstem Carry-Over.

### REQ-AGOV-HOF-003 — Auditierbarkeit

id: REQ-AGOV-HOF-003

Handoff-Records MÜSSEN revisionierbar abgelegt werden (docs/handoffs/ oder Task-Dokument; Commit-Trailer [handoff: source→target] bei Repository-Changes) mit Zeitstempel-Kette.

## 4. Compliance / Prüfungen

id: COM-AGOV-HOF-001

COM-AGOV-HOF-001: Handoff-Records vollständig (10 Pflichtfelder). COM-AGOV-HOF-002: Ziel-Agent bestätigt Lektüre (next_action-Kette konsistent).

## 5. Security Considerations

Handoff-Records DÜRFEN keine Secrets enthalten; Tests/Risiken ehrlich (truthful-reporting, ATC-POL-008).

## 6. Ausnahmen

Gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung.

## 7. Referenzen (References)

### NORMATIVE Referenzen
- ATC-AI-GOV-MANIFEST-001 — Identität/Scope · ATC-AI-GOV-POLICY-001 · ATC-STD-BUG-001 — Findings

### INFORMATIVE Referenzen
- `.github`-Hub: `ai/handoff.yaml` (Template, Storage, Continuity-Rules)

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release (Owner-Entwurf 09.09., SCR-0061): 3 REQ, 2 COM-Gates; operational über ai/handoff.yaml (SCR-0061)

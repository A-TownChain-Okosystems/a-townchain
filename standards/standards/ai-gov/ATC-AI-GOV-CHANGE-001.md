---
standard:
  id: ATC-AI-GOV-CHANGE-001
  title: "ATC Agent Governance — Governance Change Management (SCR-Pflicht, SemVer, Rollback)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000, ATC-STD-VERSION-001, ATC-STD-UPDATE-001]
  related_standards: [ATC-AI-GOV-MANIFEST-001, ATC-AI-GOV-POLICY-001, ATC-AI-GOV-CHECK-001]
  requirements: [REQ-AGOV-CHG-001, REQ-AGOV-CHG-002, REQ-AGOV-CHG-003]
---

# ATC-AI-GOV-CHANGE-001 — Governance Change Management (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 09.09.; operativer SSOT: `.github`-Hub `ai/change.yaml`; §9-Freigabe ausstehend.

## 1. Zweck (Purpose)

Änderungen an der Agent-Governance selbst sind governance-relevant: versioniert, geprüft, nachvollziehbar, dokumentiert — und reversibel.

## 2. Geltungsbereich (Scope)

**Gilt:** AGENT_MANIFEST.md, ai/agent.yaml, capabilities.yaml, policies.yaml, checks.yaml, governance-rules.yaml, handoff.yaml, incident.yaml, change.yaml, Org-AGENTS.md + agent-instructions/. **Nicht im Gelt:** Content-Änderungen in Fach-Repositories (normales UPDATE-001-Verfahren).

## 3. Normative Anforderungen

### REQ-AGOV-CHG-001 — SCR-Pflicht

id: REQ-AGOV-CHG-001

Jede Governance-Änderung ERFORDERT SCR im atc-standards-Repository (ATC-STD-000) mit Impact-Analyse. SemVer gemäß ATC-STD-VERSION-001: Verschärfung = MINOR, inkompatible Regeländerung = MAJOR, Redaktion = PATCH.

### REQ-AGOV-CHG-002 — Vorher/Nachher-Pflicht

id: REQ-AGOV-CHG-002

VOR Deploy: agov_check.py + readiness_check.py auf Hub + gebundene Repos (Regression Prevention). NACH Deploy: Snapshot-Regeneration + AGOV-Lauf; Differenzen = Findings (F-NNNN). Checks/Policies dürfen nur verschärft, nie still geschwächt werden; P0-Policy-Deaktivierung nur via Owner.

### REQ-AGOV-CHG-003 — Rollback

id: REQ-AGOV-CHG-003

Bei Gate-Failure nach Governance-Änderung: vorheriger Hub-Stand wird wiederhergestellt (Git), SCR mit Abbruchgrund geschlossen, P0-Auswirkung → Incident-Record (INCIDENT-001).

## 4. Compliance / Prüfungen

id: COM-AGOV-CHG-001

COM-AGOV-CHG-001: Governance-Commits mit SCR-Referenz + Version-Delta in der Headline. COM-AGOV-CHG-002: Post-Deploy-AGOV-Lauf mit Differenz-Report.

## 5. Security Considerations

Governance-Schwächung (Permissions aufweichen, Checks deaktivieren, Policies abschalten) ist ein Angriffsvektor: jede Schwächung = MAJOR + Owner-Gate + AUD-Record.

## 6. Ausnahmen

Gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung; Emergency nur über UPDATE-001-Notpfad mit Nachholen.

## 7. Referenzen (References)

### NORMATIVE Referenzen
- ATC-STD-000 — SCR · ATC-STD-VERSION-001 — SemVer · ATC-STD-UPDATE-001 — Change Control · ATC-AI-GOV-CHECK-001/AUDIT-001

### INFORMATIVE Referenzen
- `.github`-Hub: `ai/change.yaml` (change_scope, requirements, rollback)

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release (Owner-Entwurf 09.09., SCR-0061): 3 REQ, 2 COM-Gates; operational über ai/change.yaml (SCR-0061)

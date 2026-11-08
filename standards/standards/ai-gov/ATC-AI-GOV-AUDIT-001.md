---
standard:
  id: ATC-AI-GOV-AUDIT-001
  title: "ATC Agent Governance — Auditverfahren (AGOV-Läufe, Snapshot, Readiness, Post-Change-Audit)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000, ATC-STD-AUDIT-001, ATC-AI-GOV-CHECK-001]
  related_standards: [ATC-AI-GOV-INCIDENT-001, ATC-STD-BUG-001]
  requirements: [REQ-AGOV-AUD-001, REQ-AGOV-AUD-002, REQ-AGOV-AUD-003, REQ-AGOV-AUD-004]
---

# ATC-AI-GOV-AUDIT-001 — Auditverfahren (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 09.09.; operativer SSOT: `.github`-Hub `ai/audit.yaml` + `tools/agov_check.py`/`readiness_check.py` + `ai/audit/SNAPSHOT-*.json`; §9-Freigabe ausstehend. Abgegrenzt zu ATC-STD-AUDIT-001 (Standards-Completeness): hier Governance-Einhaltung der Repositories.

## 1. Zweck (Purpose)

Verfahren, mit dem die Organisation automatisiert feststellt: „Dieses Repository erfüllt diese Regeln tatsächlich."

## 2. Geltungsbereich (Scope)

**Gilt:** AGOV-Läufe über Repositories, Snapshot-Führung, Readiness, Post-Change-Audits. **Nicht im Gelt:** Standards-Completeness-Audit (ATC-STD-AUDIT-001), Audit-Findings-Bewertung (BUG-001), Unabhängige Review-Stufen (AUD-2026-0004-Modell).

## 3. Normative Anforderungen

### REQ-AGOV-AUD-001 — Lauf-Typen

id: REQ-AGOV-AUD-001

AGOV-FULL (täglich/gesteuert, alle Repos, Report in docs/AGOV-RUN-*.md) · AGOV-DELTA (je Task/Change, betroffene Repos, SCR/Issue-Referenz) · AGOV-GATE (vor Merge/Release, Merge-Gate).

### REQ-AGOV-AUD-002 — Evidence-Regeln

id: REQ-AGOV-AUD-002

Jeder Lauf MUSS Snapshot erzeugen/aktualisieren (`ai/audit/SNAPSHOT-*.json`, schema-valide) + Lauf-Report. Aussagen NUR mit API-Evidenz (Reality-Check). Widerspruch Check vs. Registry-SSOT → Registry gewinnt, Differenz = Finding.

### REQ-AGOV-AUD-003 — Post-Change-Audit

id: REQ-AGOV-AUD-003

Nach jeder Änderung: Diff-Review (ATC-POL-005) + Regression (POL-004/010) + Dokumentations-Sync (POL-006) + Recheck betroffener Checks bis VERIFIED.

### REQ-AGOV-AUD-004 — Continuous-Improvement-Kopplung

id: REQ-AGOV-AUD-004

Systemische Ursachen aus AGOV-Läufen MÜSSEN in IMP-Records überführt werden (ATC-STD-IMPROVEMENT-001 REQ-IMP-016/017); Audit-der-Audits gemäß ATC-STD-999.

## 4. Compliance / Prüfungen

id: COM-AGOV-AUD-001

COM-AGOV-AUD-001: Snapshot schema-valide und aktuell (je Lauf). COM-AGOV-AUD-002: Lauf-Reports in docs/ mit Verdict je Repository. COM-AGOV-AUD-003: Readiness-Check vor regulärem Betrieb PASS (SCR-0060).

## 5. Security Considerations

Snapshots enthalten Repository-Metadaten — keine Secrets; Snapshot-Manipulation = Incident (INC-CLS-GOV). Merge-Gate DARF nur via dokumentierte Ausnahme umgangen werden (exception_only, F-045-Blind-Spot bekannt).

## 6. Ausnahmen

Gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung.

## 7. Referenzen (References)

### NORMATIVE Referenzen
- ATC-AI-GOV-CHECK-001 — Checks · ATC-STD-AUDIT-001 — AUD-Records · ATC-STD-IMPROVEMENT-001 — IMP-Records · ATC-STD-999 — Master-Audit

### INFORMATIVE Referenzen
- `.github`-Hub: `ai/audit.yaml`, `docs/AGOV-RUN-2026-09-09.md`, `ai/audit/SNAPSHOT-2026-09-09.json`, `docs/READINESS-2026-09-09.md`

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release (Owner-Entwurf 09.09., SCR-0061): 4 REQ, 3 COM-Gates; operational vorbereitet durch SCR-0058/0060 (Läufe, Snapshot, Readiness)

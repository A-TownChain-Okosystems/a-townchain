---
standard:
  id: ATC-AI-GOV-POLICY-001
  title: "ATC Agent Governance — Maschinenlesbare Policies (ATC-POL-001..010, MUST/SHOULD/MAY)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000, ATC-AI-GOV-AGENTS-001]
  related_standards: [ATC-AI-GOV-CHECK-001, ATC-STD-BUG-005, ATC-STD-UPDATE-001, ATC-STD-CI-001]
  requirements: [REQ-AGOV-POL-001, REQ-AGOV-POL-002, REQ-AGOV-POL-003, REQ-AGOV-POL-004]
---

# ATC-AI-GOV-POLICY-001 — Maschinenlesbare Policies (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 09.09.; operativer SSOT: `.github`-Hub `ai/policies.yaml` (AP-001..016 Grundsätze + ATC-POL-001..010 maschinenprüfbar); §9-Freigabe ausstehend.

## 1. Zweck (Purpose)

Maschinenprüfbar machen, was Agenten tun müssen: Policies mit id, name, level (MUST/SHOULD/MAY), severity (P0-P3), enabled und requirement.

## 2. Geltungsbereich (Scope)

**Gilt:** Alle Agenten, alle Repositories, alle Tasks. **Nicht im Gelt:** Check-Katalog und Ausführung (CHECK-001), Finding-System (BUG-001), Change-Control für Releases (UPDATE-001).

## 3. Normative Anforderungen

### REQ-AGOV-POL-001 — Policy-Katalog

id: REQ-AGOV-POL-001

Der Policy-Katalog MUSS mindestens umfassen: ATC-POL-001 inspect-before-modify (P1) · 002 no-secrets (P0) · 003 validate-changes (P1) · 004 regression-check (P1) · 005 diff-review (P1) · 006 documentation-sync (P2) · 007 destructive-operation-protection (P0) · 008 truthful-reporting (P0) · 009 workflow-security (P1) · 010 cross-repository-regression (P1).

### REQ-AGOV-POL-002 — MUST/SHOULD/MAY-Modell

id: REQ-AGOV-POL-002

MUST = zwingende Compliance-Anforderung (Verstoß = FAIL). SHOULD = Standardvorgehen (Abweichung MUSS begründet werden; Verstoß = WARN). MAY = optionale Best Practice (N/A). Das Verdikt-Modell MUSS konsistent sein: PASS | FAIL | WARN | N/A.

### REQ-AGOV-POL-003 — Policy-Deaktivierung

id: REQ-AGOV-POL-003

Policies DÜRFEN NICHT ohne Owner-Freigabe deaktiviert werden (enabled=false nur via Owner). Neue Policies verschärfen, schwächen nie (§30-Logik).

### REQ-AGOV-POL-004 — Grundsatz-Ebene

id: REQ-AGOV-POL-004

Die menschenlesbaren Grundsätze (AP-001..016: inspect-before-modify, root causes statt Symptome, keine stillen Fehler, Validierung jeder Änderung u.a.) MÜSSEN mit den maschinenprüfbaren Policies verknüpft bleiben (refs je Grundsatz).

## 4. Compliance / Prüfungen

id: COM-AGOV-POL-001

COM-AGOV-POL-001: policies.yaml schema-valide (ID-Pattern ATC-POL-NNN, Level/Severity/Requirement je Policy). COM-AGOV-POL-002: P0-Policies enabled=true. COM-AGOV-POL-003: Policy-Verstoß erzeugt Finding (F-NNNN) bzw. Blockade (Merge-Gate).

## 5. Security Considerations

ATC-POL-002 (no-secrets) und ATC-POL-007 (destructive) sind P0: Verstoß = Incident (INCIDENT-001), kein automatisches Fortfahren.

## 6. Ausnahmen

Gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung; Ausnahme-Regel governance-rules.yaml (AUD-Record + Standard-Referenz + Nachholen).

## 7. Referenzen (References)

### NORMATIVE Referenzen
- ATC-AI-GOV-CHECK-001 — Prüfung der Policies · ATC-STD-BUG-001 — Findings · ATC-AI-GOV-INCIDENT-001 — P0-Behandlung

### INFORMATIVE Referenzen
- `.github`-Hub: `ai/policies.yaml` (Version 1.1.0, verdict_mapping)

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release (Owner-Entwurf 09.09., SCR-0061): 4 REQ, 3 COM-Gates; operational vorbereitet durch SCR-0057/0058

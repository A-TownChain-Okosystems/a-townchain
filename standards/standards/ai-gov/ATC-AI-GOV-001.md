---
standard:
  id: ATC-AI-GOV-001
  title: "ATC Agent Governance Framework v1.0 — Produktionsarchitektur (Governance-Kette, 4 Kontrollprinzipien, Severity-/Status-Modell, Zuständigkeiten, Phasen-Roadmap)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Architektur-Freeze 09.09.)"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000]
  related_standards: [ATC-AI-GOV-AGENTS-001, ATC-AI-GOV-MANIFEST-001, ATC-AI-GOV-CAPABILITY-001, ATC-AI-GOV-POLICY-001, ATC-AI-GOV-CHECK-001, ATC-AI-GOV-AUDIT-001, ATC-AI-GOV-FINDING-001, ATC-AI-GOV-HANDOFF-001, ATC-AI-GOV-INCIDENT-001, ATC-AI-GOV-CHANGE-001]
  requirements: [REQ-AIGOV-001, REQ-AIGOV-002, REQ-AIGOV-003, REQ-AIGOV-004, REQ-AIGOV-005, REQ-AIGOV-006, REQ-AIGOV-007]
---

# ATC-AI-GOV-001 — ATC Agent Governance Framework v1.0 (APPROVED — SCR-0124)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) v1.0.0 — **Owner-Architektur-Freeze 09.09.** („ATC-AI-GOV v1.0 Produktionsarchitektur einfrieren", SCR-0062). §9-Formalfreigabe der Familie empfohlen als Sammelfreigabe. Umbrella-Standard der Familie; die 10 Sub-Standards tragen die Details.
> **Grundsatz:** Agentenanweisungen behaupten Compliance nicht — die Checks beweisen sie.

## 1. Zweck (Purpose)

Erfassung der v1.0-Produktionsarchitektur: Governance-Kette, vier Kontrollprinzipien, formalisiertes Severity-Modell mit Standardreaktionen und Repository-Status-Berechnung, Verantwortlichkeits-Trennung der Artefakte, 7-Phasen-Implementierungs-Roadmap.

## 2. Geltungsbereich (Scope)

**Gilt:** Gesamte ATC-AI-GOV-Familie und alle Agenten der A-TownChain-Okosystems; operative Artefakte im `.github`-Hub. **Nicht im Gelt:** Sub-Standards-Inhalte (je eigener Standard), Software-Release-Governance (UPDATE-001), Standards-Vollständigkeit (ATC-STD-AUDIT-001).

## 3. Normative Anforderungen

### REQ-AIGOV-001 — Governance-Kette

id: REQ-AIGOV-001

Jede agentengetriebene Änderung folgt der Kette: IDENTITY → AUTHORIZATION → SCOPE → INSTRUCTIONS → POLICIES → DISCOVERY → AUDIT → TASK → IMPLEMENTATION → CHECKS → SECURITY → TESTS → REGRESSION → DIFF REVIEW → DOCUMENTATION → PR/RELEASE → POST-CHANGE AUDIT → VERIFIED. Kein Stufe-Auslassen bei material changes.

### REQ-AIGOV-002 — Familienstruktur

id: REQ-AIGOV-002

Das Framework besteht aus diesem Umbrella plus 10 Sub-Standards: AGENTS-001 (Verhalten) · MANIFEST-001 (Identität) · CAPABILITY-001 (Berechtigungsmodell) · POLICY-001 (Regeln) · CHECK-001 (Messung) · AUDIT-001 (Bewertung) · FINDING-001 (festgestellte Verstöße) · HANDOFF-001 · INCIDENT-001 · CHANGE-001.

### REQ-AIGOV-003 — Verantwortlichkeits-Trennung

id: REQ-AIGOV-003

Die Artefakte haben getrennte Zuständigkeiten: AGENTS.md = Verhalten, AGENT_MANIFEST.md = Identität, agent.yaml = Maschinenidentität, capabilities.yaml = Berechtigungsmodell, policies.yaml = Regeln, CHECK-NNN = Messung, FINDING = Verstoß, AUDIT = Bewertung, ISSUE = Behebung, RECHECK = Verifikation. Eine Datei, eine Verantwortung — keine Mega-AGENTS.md.

### REQ-AIGOV-004 — Maschinenlesbare Check-Objekte

id: REQ-AIGOV-004

Jeder Check ist ein eigenes maschinenlesbares Objekt (`ATC-CHECK-NNN`, ai/checks/CHECK-NNN.yaml, Schema ai/schemas/check.schema.yaml): id, version, status, metadata (name/category/severity/level/automation), scope, requirement (type/path/pattern/expect), evaluation (pass_when, fail_closed), result.allowed, remediation (required_on, creates_finding), evidence. Ein unabhängiger ATC Governance Auditor kann Checks OHNE den jeweiligen Agenten ausführen.

### REQ-AIGOV-005 — Severity-Modell und Repository-Status

id: REQ-AIGOV-005

Severity mit Standardreaktion: P0 (kritischer Governance/Security-Verstoß) = BLOCK · P1 (schwerwiegender Compliance-Verstoß) = BLOCK/MUST FIX · P2 (relevanter Qualitätsmangel) = FIX REQUIRED · P3 (Best Practice) = TRACK. Repository-Status-Berechnung durch den Auditor: P0 offen → BLOCKED · P1 offen → NON-COMPLIANT · nur P2 → COMPLIANT-WITH-FINDINGS · nur P3 → COMPLIANT · keine → VERIFIED.

### REQ-AIGOV-006 — Kontrollprinzipien (1+2)

id: REQ-AIGOV-006

**Fail Closed:** Bei P0/P1-Regeln gilt unknown → FAIL, missing evidence → FAIL, invalid configuration → FAIL — NIEMALS unknown → PASS. **Evidence First:** Jeder PASS beruht auf überprüfbarer Evidence (Check → Execution → Evidence → Result).

### REQ-AIGOV-007 — Kontrollprinzipien (3+4)

id: REQ-AIGOV-007

**Persistent Findings:** Jedes gefundene Problem erhält ATC-FINDING-YYYY-NNNNNN und bleibt bis VERIFIED verfolgbar (FINDING-001). **Keine Selbstzertifizierung:** Ein Agent behauptet Compliance nicht — die Checks beweisen sie; SECURITY = VERIFIED entsteht nur aus Check-Ausführung + Ergebnis + Evidence + Timestamp + Auditor (ATC-POL-008).

## 4. Compliance / Prüfungen

id: COM-AIGOV-001

COM-AIGOV-001: Alle 10 Sub-Standards registriert und konsistent zum Hub. COM-AIGOV-002: Check-Objekte schema-valide (check.schema.yaml). COM-AIGOV-003: Auditor berechnet Repository-Status deterministisch aus offenen Findings.

## 5. Security Considerations

Fail-Closed ist die Kern-Abwehr: unknown-Zustände sind Angriffsfläche für stilles PASS. P0-Check-Ergebnisse dürfen nicht übersteuert werden ohne AUD-Record + Owner. Keine Selbstzertifizierung verhindert getäuschte Compliance-Berichte.

## 6. Ausnahmen

Gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung; Ausnahme von Fail-Closed-Prinzipien ist UNZULÄSSIG (P0-Prinzip).

## 7. Referenzen (References)

### NORMATIVE Referenzen
- 10 Sub-Standards der Familie (related_standards) · ATC-STD-000 · ATC-STD-BUG-001 · ATC-POL-001..010 (POLICY-001)

### INFORMATIVE Referenzen
- `.github`-Hub: `ai/schemas/*.schema.yaml`, `ai/audit/audit-policy.yaml`, `ai/audit/finding-schema.yaml`, `ai/checks/CHECK-001.yaml`, `ai/checks/CHECK-010.yaml`, `ai/phases.yaml` (7-Phasen-Roadmap: Schemas ✓ → CHECK-Objekte → Auditor → GH-Actions-Enforcement → Issue/PR-Automation → Cross-Repo-Audit → Compliance-Dashboard)

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release: Owner-Architektur-Freeze v1.0 (SCR-0062): 18-stufige Governance-Kette, 4 Kontrollprinzipien (Fail Closed, Evidence First, Persistent Findings, No Self-Certification), Severity/Status-Modell (P0-P3 → BLOCKED/NON-COMPLIANT/COMPLIANT-WITH-FINDINGS/COMPLIANT/VERIFIED), Verantwortlichkeits-Trennung, maschinenlesbare Check-Objekte, 7-Phasen-Roadmap

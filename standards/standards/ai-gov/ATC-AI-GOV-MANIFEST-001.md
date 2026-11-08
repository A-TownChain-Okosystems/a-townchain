---
standard:
  id: ATC-AI-GOV-MANIFEST-001
  title: "ATC Agent Governance — Agent Identity & Scope (Manifest, Registry, Capabilities, Handoff-Felder)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000, ATC-AI-GOV-AGENTS-001]
  related_standards: [ATC-AI-GOV-POLICY-001, ATC-AI-GOV-HANDOFF-001, ATC-STD-AUDIT-001]
  requirements: [REQ-AGOV-MAN-001, REQ-AGOV-MAN-002, REQ-AGOV-MAN-003, REQ-AGOV-MAN-004, REQ-AGOV-MAN-005, REQ-AGOV-MAN-006, REQ-AGOV-MAN-007]
---

# ATC-AI-GOV-MANIFEST-001 — Agent Identity & Scope (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 09.09.; operativer SSOT: `.github`-Hub `AGENT_MANIFEST.md` v1.2.0 + `ai/agent.yaml` + `ai/capabilities.yaml`; §9-Freigabe ausstehend.

## 1. Zweck (Purpose)

Wer darf was? Das Manifest definiert organisatorische Identität, Zuständigkeit, Fähigkeiten und Governance-Zuordnung jedes Agenten — als organisatorische Referenz für Identity, Authorization, Scope, Capabilities, Handoff, Governance und Auditability.

## 2. Geltungsbereich (Scope)

**Gilt:** Alle registrierten Agenten der A-TownChain-Okosystems. **Nicht im Gelt:** Arbeitsregeln (AGENTS-001), Policy-Inhalte (POLICY-001), Übergabe-Ausführung (HANDOFF-001), Prüfkatalog (CHECK-001).

## 3. Normative Anforderungen

### REQ-AGOV-MAN-001 — Agent Identity

id: REQ-AGOV-MAN-001

Jeder registrierte Agent MUSS eindeutig identifizierbar sein: agent_id, name, role, version, status, owner, scope, capabilities (Pflichtfelder).

### REQ-AGOV-MAN-002 — Status-Modell

id: REQ-AGOV-MAN-002

Zulässige Statuswerte: PROPOSED · ACTIVE · SUSPENDED · DEPRECATED · RETIRED. NUR ACTIVE-Agenten DÜRFEN reguläre Organisationsaufgaben ausführen.

### REQ-AGOV-MAN-003 — Agent Registry

id: REQ-AGOV-MAN-003

Die Agent-Registry (Manifest Abschnitt 4) FÜHRT alle Agenten mit Rolle × Instanz ehrlich (keine Phantom-Einträge; Rollen-Profile statt erfundener Instanzen).

### REQ-AGOV-MAN-004 — Capability Model

id: REQ-AGOV-MAN-004

Capabilities MÜSSEN explizit definiert sein (READ_REPOSITORY, ANALYZE_CODE, WRITE_CODE, MODIFY_DOCUMENTATION, RUN_TESTS, CREATE_ISSUES, CREATE_PULL_REQUESTS, MODIFY_WORKFLOWS, PERFORM_AUDITS, MODIFY_INFRASTRUCTURE, PERFORM_RELEASE, SECURITY_ANALYSIS u.a.) und DÜRFEN NICHT automatisch aus der Rolle abgeleitet werden.

### REQ-AGOV-MAN-005 — Scope

id: REQ-AGOV-MAN-005

Jeder Agent MUSS einen definierten Scope besitzen (ORGANIZATION · REPOSITORY · DIRECTORY · MODULE · TASK). Außerhalb des autorisierten Scopes sind KEINE Änderungen zulässig.

### REQ-AGOV-MAN-006 — Maschinenlesbare Identität

id: REQ-AGOV-MAN-006

`ai/agent.yaml` bildet die Identität maschinenlesbar ab (schema_version, organization, agent, scope, capabilities, governance, requirements) — OHNE Duplikation der Gesamt-Governance.

### REQ-AGOV-MAN-007 — Manifest-Änderungen

id: REQ-AGOV-MAN-007

Änderungen am Manifest sind governance-relevant: versioniert, geprüft, nachvollziehbar, dokumentiert (→ CHANGE-001).

## 4. Compliance / Prüfungen

id: COM-AGOV-MAN-001

COM-AGOV-MAN-001: Jeder Agenten-Commit MUSS einer aktiven agent_id zuordenbar sein (Commit-Trailer). COM-AGOV-MAN-002: agent.yaml/AGENT_MANIFEST konsistent (Schema-valide, Status ACTIVE). COM-AGOV-MAN-003: Handoff-Pflichtfelder je Übergabe vollständig (HANDOFF-001).

## 5. Security Considerations

Capabilities wie MODIFY_WORKFLOWS/MODIFY_INFRASTRUCTURE sind P0-relevant: Erteilung nur via Owner-Gate; Capability-Ausnutzung über Scope hinaus = Incident (INCIDENT-001 INC-CLS-GOV).

## 6. Ausnahmen

Gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung.

## 7. Referenzen (References)

### NORMATIVE Referenzen
- ATC-AI-GOV-AGENTS-001 — Arbeitsregeln · ATC-AI-GOV-HANDOFF-001 — Übergabeprotokoll · ATC-AI-GOV-CHANGE-001 — Governance-Change-Management · ATC-STD-AUDIT-001 — Auditability

### INFORMATIVE Referenzen
- `.github`-Hub: `AGENT_MANIFEST.md` (10 Abschnitte), `ai/agent.yaml`, `ai/capabilities.yaml`

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release (Owner-Entwurf 09.09., SCR-0061): 7 REQ (Identity/Status/Registry/Capabilities/Scope/Maschinenlesbarkeit/Änderungen), 3 COM-Gates; operational vorbereitet durch SCR-0057/0058 (AGENT_MANIFEST.md v1.1.0 → 1.2.0)

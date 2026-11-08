---
standard:
  id: ATC-AI-GOV-AGENTS-001
  title: "ATC Agent Governance — Organisationsweite Arbeitsregeln (Discovery, Hierarchie, Binding)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000, ATC-AAS-001, ATC-STD-AOS-001]
  related_standards: [ATC-AI-GOV-MANIFEST-001, ATC-AI-GOV-POLICY-001, ATC-STD-AI-DECISION-001]
  requirements: [REQ-AGOV-001, REQ-AGOV-002, REQ-AGOV-003, REQ-AGOV-004, REQ-AGOV-005, REQ-AGOV-006]
---

# ATC-AI-GOV-AGENTS-001 — Organisationsweite Arbeitsregeln (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 09.09. (Zielbild ATC-AI-GOV, „prüfbares Governance-System"); operativer SSOT: `.github`-Hub `AGENTS.md` + `agent-instructions/00-11` (SCR-0057); §9-Freigabe ausstehend.

## 1. Zweck (Purpose)

Die AGENTS.md bleibt die menschlich und agentisch lesbare operative Leitlinie. Dieser Standard formalisiert die organisationsweiten Arbeitsregeln, damit die Organisation automatisiert feststellen kann: „Dieses Repository erfüllt diese Regeln tatsächlich."

## 2. Geltungsbereich (Scope)

**Gilt:** Alle Agenten (ATC-AI-*-001 ff.), alle 26+ Repositories, alle Tasks. **Nicht im Gelt:** Identität/Scope (MANIFEST-001), Policy-Inhalte (POLICY-001), Check-Katalog (CHECK-001), Auditverfahren (AUDIT-001), Handoff (HANDOFF-001), Incidents (INCIDENT-001), Governance-Änderungen (CHANGE-001).

## 3. Normative Anforderungen

### REQ-AGOV-001 — Instruction-Discovery

id: REQ-AGOV-001

Agenten MÜSSEN vor jeder Änderung die Repository-Anweisungen entdecken und berücksichtigen (AGENTS.md-Kaskade: Org-Hub → Repo → Verzeichnis). Grundlage: ATC-POL-001 inspect-before-modify; AGOV-CHECK-001/002/003 prüfen die Anbindung.

### REQ-AGOV-002 — Regel-Hierarchie

id: REQ-AGOV-002

Es GILT die Hierarchie: Org-Policy > AGENT_MANIFEST > Org-AGENTS.md > Repo-AGENTS.md > Verzeichnis-Regeln > Task. Spezifischere Regeln ERGÄNZEN, hebeln NIEMALS höhere Regeln aus.

### REQ-AGOV-003 — Registry-Binding

id: REQ-AGOV-003

Jeder Agent MUSS dynamisch an die Standards-Registry gebunden sein (AAS-001: agent.yaml required_standards = ALLE Registry-Standards; AGOV Registry-Binding-Checker leitet Lücken P0/P1/P2 maschinell ab, SCR-0059).

### REQ-AGOV-004 — Session-Mandat

id: REQ-AGOV-004

Jeder Agenten-Task folgt dem Session-Mandat (AOS-001): Verbindlichkeitsklausel, dynamische Bindung, Umsetzungspflicht, AUD-Records (AI-DEV-009).

### REQ-AGOV-005 — Scope-Treue

id: REQ-AGOV-005

Außerhalb des autorisierten Scopes DÜRFEN KEINE Änderungen durchgeführt werden (Details MANIFEST-001 REQ-AGOV-MAN-005). Scope-Wechsel während eines Tasks = neuer Task + neuer Snapshot (governance-rules.yaml).

### REQ-AGOV-006 — Readiness Control Plane

id: REQ-AGOV-006

Vor regulärem Betrieb MUSS die Readiness geprüft sein (readiness_check.py: Governance-Infrastruktur vollständig — Snapshot, Binding, Merge-Gate; SCR-0060).

## 4. Compliance / Prüfungen

id: COM-AGOV-001

COM-AGOV-001: AGOV-CHECK-001/002/003/020 je Repository PASS (MUST). COM-AGOV-002: Registry-Binding-Checker ohne P0-Lücke je gebundenem Repo. COM-AGOV-003: AGOV-FULL-Lauf täglich mit Lauf-Report (AUDIT-001).

## 5. Security Considerations

AGENTS.md-Kaskade ist Angriffsfläche: Repo-AGENTS.md DARF keine höheren Regeln aushebeln (REQ-AGOV-002, prüfbar via AGOV-CHECK-003 + Governance-Interpretations-Vergleich).

## 6. Ausnahmen

Ausnahmen NUR gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung; Merge-Gate exception_only (governance-rules.yaml).

## 7. Referenzen (References)

### NORMATIVE Referenzen
- ATC-STD-000 — Standards Governance · ATC-AAS-001 — Agent-Session-Manifest · ATC-STD-AOS-001 — Session-Mandat · ATC-AI-GOV-POLICY-001 — Policies · ATC-AI-GOV-CHECK-001 — Check-Katalog

### INFORMATIVE Referenzen
- `.github`-Hub: `AGENTS.md`, `agent-instructions/00-11`, `ai/governance-rules.yaml`, `tools/readiness_check.py`

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release (Owner-Entwurf Zielbild 09.09., SCR-0061): 6 REQ, 3 COM-Gates; operational vorbereitet durch SCR-0057/0059/0060 (Hub, Binding-Checker, Readiness)

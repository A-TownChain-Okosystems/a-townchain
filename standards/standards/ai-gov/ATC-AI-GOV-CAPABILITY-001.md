---
standard:
  id: ATC-AI-GOV-CAPABILITY-001
  title: "ATC Agent Governance — Capability & Authorization Model (explizite Berechtigungen, Owner-Gates, Suspendierung)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf 09.09.)"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000, ATC-AI-GOV-MANIFEST-001]
  related_standards: [ATC-AI-GOV-001, ATC-AI-GOV-POLICY-001, ATC-AI-GOV-INCIDENT-001]
  requirements: [REQ-AGOV-CAP-001, REQ-AGOV-CAP-002, REQ-AGOV-CAP-003, REQ-AGOV-CAP-004]
---

# ATC-AI-GOV-CAPABILITY-001 — Capability & Authorization Model (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 09.09. (Zielstruktur: capabilities.yaml als eigenständiges Berechtigungsmodell); operativer SSOT: `.github`-Hub `ai/capabilities.yaml` + Schema `ai/schemas/agent.schema.yaml`; SCR-0062.

## 1. Zweck (Purpose)

Wer darf was — technisch: ein explizites, prüfbares Berechtigungsmodell, das Autorisierung von der Rolle trennt und P0-relevante Befugnisse an Owner-Gates bindet.

## 2. Geltungsbereich (Scope)

**Gilt:** Alle registrierten Agenten, alle Capability-Vergaben. **Nicht im Gelt:** Identität/Status (MANIFEST-001), Verhaltensregeln (POLICY-001), Scope-Definition als solche (MANIFEST-001 REQ-AGOV-MAN-005 — Capability sagt WAS, Scope sagt WO).

## 3. Normative Anforderungen

### REQ-AGOV-CAP-001 — Explizite Capability-Vergabe

id: REQ-AGOV-CAP-001

Capabilities MÜSSEN explizit je Agent vergeben sein und DÜRFEN NICHT automatisch aus der Rolle abgeleitet werden (READ_REPOSITORY, ANALYZE_CODE, WRITE_CODE, MODIFY_DOCUMENTATION, RUN_TESTS, CREATE_ISSUES, CREATE_PULL_REQUESTS, MODIFY_WORKFLOWS, PERFORM_AUDITS, MODIFY_INFRASTRUCTURE, PERFORM_RELEASE, SECURITY_ANALYSIS).

### REQ-AGOV-CAP-002 — Autorisierungskette

id: REQ-AGOV-CAP-002

Autorisierung folgt IDENTITY → CAPABILITY → SCOPE: Eine Aktion ist nur zulässig, wenn Agent ACTIVE (MANIFEST REQ-AGOV-MAN-002), die Capability explizit vergeben UND der Zielort im autorisierten Scope liegt. Ein Fehlschlag = Incident INC-CLS-GOV bei P0-Auswirkung.

### REQ-AGOV-CAP-003 — Owner-Gates für P0-Capabilities

id: REQ-AGOV-CAP-003

MODIFY_WORKFLOWS, MODIFY_INFRASTRUCTURE, PERFORM_RELEASE und SECURITY_ANALYSIS sind P0-Capabilities: Vergabe, Erweiterung und Entzug NUR durch Owner-Gate (dokumentiert via SCR).

### REQ-AGOV-CAP-004 — Entzug und Suspendierung

id: REQ-AGOV-CAP-004

Bei Policy-Verstoß oder Incidents KÖNNEN Capabilities suspendiert werden (Manifest-Status SUSPENDED effectiv); Entzug ist dokumentiert (SCR + AUD-Referenz). capability-Änderungen folgen CHANGE-001.

## 4. Compliance / Prüfungen

id: COM-AGOV-CAP-001

COM-AGOV-CAP-001: capabilities.yaml schema-valide (agent.schema.yaml capabilities-Block). COM-AGOV-CAP-002: Keine Agent-Aktion ohne aktive Capability-Deckung (Auditor prüfbar via Commit-Trailer + Capability-Abgleich). COM-AGOV-CAP-003: P0-Capability-Vergaben mit SCR-Referenz.

## 5. Security Considerations

Rollen-Ableitung wäre Privilege-Escalation per Umbenennung; explizite Vergabe + Owner-Gates verhindern das. Capability-Nutzung über Scope hinaus = Incident.

## 6. Ausnahmen

Gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung; P0-Capability-Ausnahmen unzulässig.

## 7. Referenzen (References)

### NORMATIVE Referenzen
- ATC-AI-GOV-MANIFEST-001 — Identität/Status · ATC-AI-GOV-001 — Autorisierungskette · ATC-AI-GOV-INCIDENT-001 — Verstöße · ATC-AI-GOV-CHANGE-001

### INFORMATIVE Referenzen
- `.github`-Hub: `ai/capabilities.yaml`, `ai/schemas/agent.schema.yaml`

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release (Owner-Entwurf 09.09., SCR-0062): 4 REQ (explizite Vergabe, Autorisierungskette, Owner-Gates, Entzug), 3 COM-Gates

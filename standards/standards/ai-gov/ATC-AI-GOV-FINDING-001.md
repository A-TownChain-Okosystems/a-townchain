---
standard:
  id: ATC-AI-GOV-FINDING-001
  title: "ATC Agent Governance — Persistent Findings (ATC-FINDING-YYYY-NNNNNN, Lifecycle, Verifikation, Registry-Kopplung)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf 09.09.)"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000, ATC-STD-BUG-001, ATC-AI-GOV-CHECK-001]
  related_standards: [ATC-AI-GOV-001, ATC-AI-GOV-AUDIT-001, ATC-STD-IMPROVEMENT-001]
  requirements: [REQ-AGOV-FIN-001, REQ-AGOV-FIN-002, REQ-AGOV-FIN-003, REQ-AGOV-FIN-004, REQ-AGOV-FIN-005]
---

# ATC-AI-GOV-FINDING-001 — Persistent Findings (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 09.09. (Kontrollprinzip 3 „Findings sind persistent"); operativer SSOT: `.github`-Hub `ai/audit/finding-schema.yaml`; SCR-0062.

## 1. Zweck (Purpose)

Ein gefundenes Problem verschwindet nicht: eindeutige ID, voller Pflichtdatensatz, verfolgbarer Lifecycle bis VERIFIED — als Kette CHECK → FINDING → ISSUE → FIX → RECHECK → VERIFIED.

## 2. Geltungsbereich (Scope)

**Gilt:** Alle vom Governance Auditor erzeugten Findings. **Abgrenzung (SCR-0062):** Die Registry `registry/findings.yaml` mit F-NNNN-Kurz-IDs (ATC-STD-BUG-001) bleibt Repository-Befundsystem der atc-standards; ATC-FINDING-YYYY-NNNNNN ist das kanonische, organisationsweite Persistenzformat des Governance Auditor. Beide sind über `registry_ref` verknüpft; ein Governance-Finding, das in der Registry geführt wird, trägt beide IDs (kanonisch + Kurz-ID). **Nicht im Gelt:** Finding-Erstschrift in BUG-001, IMP-Records (IMPROVEMENT-001).

## 3. Normative Anforderungen

### REQ-AGOV-FIN-001 — Finding-ID und Pflichtdatensatz

id: REQ-AGOV-FIN-001

Jedes Finding erhält `ATC-FINDING-YYYY-NNNNNN` mit Pflichtdatensatz (finding-schema.yaml): finding_id, check_id (ATC-CHECK-NNN), repository, severity (P0-P3), status, evidence (kind + detail + timestamp), detected_at, detected_by (Auditor-Agent-ID), remediation (required), verification (status PENDING).

### REQ-AGOV-FIN-002 — Lifecycle

id: REQ-AGOV-FIN-002

OPEN → IN-PROGRESS → REMEDIATED → RECHECK-PENDING → VERIFIED (Fehlerpfad REJECTED mit Begründung). VERIFIED NUR nach erfolgreichem Recheck des auslösenden Checks (recheck_check_id) mit neuer Evidence.

### REQ-AGOV-FIN-003 — Keine Selbstzertifizierung bei der Verifikation

id: REQ-AGOV-FIN-003

verification.verifier MUSS nachvollziehbar sein (Auditor-Agent-ID oder Owner); der Recheck beruht auf frischer Evidence mit Timestamp — Behauptung ohne Ausführung ist unzulässig (ATC-POL-008, Kontrollprinzip 4).

### REQ-AGOV-FIN-004 — Severity und Status-Aggregation

id: REQ-AGOV-FIN-004

Offene Findings bestimmen den Repository-Status (ATC-AI-GOV-001 REQ-AIGOV-005): P0 → BLOCKED, P1 → NON-COMPLIANT, nur P2 → COMPLIANT-WITH-FINDINGS, nur P3 → COMPLIANT, keine → VERIFIED. Die Berechnung ist deterministisch und wiederholbar.

### REQ-AGOV-FIN-005 — Persistenz und Kopplung

id: REQ-AGOV-FIN-005

Findings sind persistent: Löschen ist unzulässig, nur Status-Übergänge mit Audit-Spur. Governance-Findings mit Repo-Relevanz werden über registry_ref in die Registry gespiegelt (F-NNNN) oder begründet nicht; P0/P1-Findings erzeugen Issues (CHECK-001 REQ-AGOV-CHK-004).

## 4. Compliance / Prüfungen

id: COM-AGOV-FIN-001

COM-AGOV-FIN-001: Finding-Records schema-valide (finding-schema.yaml). COM-AGOV-FIN-002: VERIFIED nur mit Recheck-Evidence (recheck_check_id + timestamp). COM-AGOV-FIN-003: Repository-Status-Aggregation stimmt mit offenen Findings überein (Auditor deterministisch).

## 5. Security Considerations

Finding-Manipulation (Severity-Downgrade, stilles Schließen) ist ein Angriffsvektor: Status-Übergänge nur mit Audit-Spur; P0-Downgrade = Owner-Gate. Evidence enthält keine Secrets.

## 6. Ausnahmen

Gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung; Ausnahme von REQ-AGOV-FIN-005 (Persistenz) unzulässig.

## 7. Referenzen (References)

### NORMATIVE Referenzen
- ATC-AI-GOV-CHECK-001 — Auslösende Checks · ATC-AI-GOV-001 — Kontrollprinzipien/Status-Modell · ATC-STD-BUG-001 — Registry-Findings (F-NNNN) · ATC-STD-IMPROVEMENT-001 — Systemische IMP-Records

### INFORMATIVE Referenzen
- `.github`-Hub: `ai/audit/finding-schema.yaml`, `ai/audit/audit-policy.yaml`

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release (Owner-Entwurf 09.09., SCR-0062): 5 REQ (ID/Datensatz, Lifecycle, Verifikations-Wahrheit, Status-Aggregation, Persistenz/Kopplung), 3 COM-Gates

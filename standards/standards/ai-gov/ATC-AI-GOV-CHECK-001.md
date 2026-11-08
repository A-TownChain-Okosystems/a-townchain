---
standard:
  id: ATC-AI-GOV-CHECK-001
  title: "ATC Agent Governance — Automatisierte Compliance Checks (AGOV-CHECK-001..020, versioniert)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-09"
  updated: "2026-09-09"
  normative: false
  dependencies: [ATC-STD-000, ATC-AI-GOV-POLICY-001, ATC-AI-GOV-AUDIT-001]
  related_standards: [ATC-STD-REPO-AUDIT-001, ATC-STD-REPO-AUDIT-002, ATC-STD-BUG-001]
  requirements: [REQ-AGOV-CHK-001, REQ-AGOV-CHK-002, REQ-AGOV-CHK-003, REQ-AGOV-CHK-004, REQ-AGOV-CHK-005]
---

# ATC-AI-GOV-CHECK-001 — Automatisierte Compliance Checks (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 09.09.; operativer SSOT: `.github`-Hub `ai/checks.yaml` (AGOV-CHECK-001..020, Version 1.0.0, Ausführung `tools/agov_check.py`); §9-Freigabe ausstehend.

## 1. Zweck (Purpose)

Aus Dokumenten wird ein automatisiertes Kontrollsystem: Repository → Governance Discovery → CHECK-001..NNN → PASS/FAIL/WARN/N/A → Finding → P0-P3 → Issue → Fix → Recheck → VERIFIED.

## 2. Geltungsbereich (Scope)

**Gilt:** Alle Repositories (AGOV-Sicht). **Nicht im Gelt:** Detail-Repo-Ebene CHECK-001..064 (ATC-STD-REPO-AUDIT-002, registry/repo-audit-checks.yaml) — AGOV prüft die Governance-Sicht, REPO-AUDIT die Repository-Ebene.

## 3. Normative Anforderungen

### REQ-AGOV-CHK-001 — Versionierter Check-Katalog

id: REQ-AGOV-CHK-001

Der CHECK-Katalog MUSS selbst versioniert sein (ID, Version, Status, Category, Severity je Check): AGOV-CHECK-001 AGENTS.md vorhanden (P1) · 002 AGENT_MANIFEST-Anbindung (P1) · 003 lokale Hierarchie erkennbar (P1) · 004 README (P2) · 005 LICENSE (P1) · 006 SECURITY.md (P1) · 007 CHANGELOG (P2) · 008 CODEOWNERS (P2) · 009 Actions-Permissions je Workflow (P1) · 010 Secret Scan (P0) · 011 Dependency Lockfile (P1) · 012 Tests vorhanden (P1) · 013 Build erfolgreich (P1) · 014 Dokumentationskonsistenz (P2) · 015 Versionierung konsistent (P2) · 016 Git Diff sauber (P1) · 017 keine Debug-Artefakte (P2) · 018 keine destruktiven Ops ohne Autorisierung (P0) · 019 Workflow-Security least-privilege (P1) · 020 AGENTS.md Schema/Struktur (P1).

### REQ-AGOV-CHK-002 — Automatisierungsmodell

id: REQ-AGOV-CHK-002

Jeder Check MUSS den Automatisierungsgrad führen: AUTO (GitHub-API) · MANUAL (Task-Zeit) · HYBRID. Nicht automatisierbare Checks DÜRFEN nicht als AUTO deklariert werden (truthful-reporting, ATC-POL-008).

### REQ-AGOV-CHK-003 — Verdikts-Modell

id: REQ-AGOV-CHK-003

Jeder Check liefert PASS | FAIL | WARN | N/A gemäß MUST/SHOULD/MAY (POLICY-001 REQ-AGOV-POL-002). MUST-Verstoß = FAIL (blockierend bei P0/P1), SHOULD-Verstoß = WARN (Begründungspflicht), MAY = N/A.

### REQ-AGOV-CHK-004 — Findings und Issues

id: REQ-AGOV-CHK-004

FAIL/WARN mit Substanz MUSS als Finding erfasst werden (F-NNNN); P0/P1-Findings erzeugen Issues (automatisiert erstellbar). Fix + Recheck bis VERIFIED.

### REQ-AGOV-CHK-005 — Check-Evolution

id: REQ-AGOV-CHK-005

Checks DÜRFEN nur verschärft, nie still geschwächt werden. Jede neue Katalog-Version DOKUMENTIERT das Delta (→ CHANGE-001).

## 4. Compliance / Prüfungen

id: COM-AGOV-CHK-001

COM-AGOV-CHK-001: checks.yaml schema-valide (AGOV-CHECK-NNN-Pattern, Level/Severity/Automation je Check). COM-AGOV-CHK-002: test_agov.py Suite grün (Hub-Selbstkonformität). COM-AGOV-CHK-003: AGOV-Lauf erzeugt Snapshot + Report je Lauf (AUDIT-001).

## 5. Security Considerations

CHECK-010 (Secret Scan) und CHECK-018 (destructive) sind P0-MUST: FAIL blockiert Merge (Merge-Gate), kein manuelles Übersteuern ohne AUD-Record + Owner.

## 6. Ausnahmen

Gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC) mit Owner-Genehmigung; automation: MANUAL-Checks mit Begründung im Task-Nachweis.

## 7. Referenzen (References)

### NORMATIVE Referenzen
- ATC-AI-GOV-POLICY-001 — MUST/SHOULD/MAY · ATC-AI-GOV-AUDIT-001 — Läufe/Evidence · ATC-STD-BUG-001 — Findings · ATC-STD-REPO-AUDIT-002 — Detail-Reposicht

### INFORMATIVE Referenzen
- `.github`-Hub: `ai/checks.yaml`, `tools/agov_check.py`, `tools/test_agov.py`, `docs/AGOV-RUN-2026-09-09.md`

## Changelog

### 1.0.0 — 2026-09-09
- Initial Release (Owner-Entwurf 09.09., SCR-0061): 5 REQ, 3 COM-Gates; operational vorbereitet durch SCR-0058 (checks.yaml 1.0.0, agov_check.py, 20 Checks mit AUTO/MANUAL/HYBRID)

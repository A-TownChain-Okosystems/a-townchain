# Exact-Pin Dependency Maintenance Contract (ATC-STD-standards-Python)

> Owner-Vertrag gemäß Issue #82 (APPROVE GOVERNANCE CONTRACT, 06.10.2026).
> Geltungsbereich: Python-Dependency-Maintenance (requirements.txt, == Pins).
> Das Exact-Pin-Gate wird NICHT abgeschwächt — keine Lockerung des Guards.

## 1. Ownership
Dependency Maintenance liegt beim **Owner** (Dependency-Maintenance-Owner).
Ausführung/Vorbereitung: Agent; Freigabe: ausschließlich Owner.

## 2. Trigger
- **Security Advisory (OSV/GHSA) = sofortiger Trigger** — Auswertung und
  Update-Vorschlag ohne Wartezeit.
- Regulärer Review-Zyklus: monatlich (mit jedem Governance-Review).

## 3. Discovery
Automatisierte Discovery: Dependabot/OSV-Gate je Push (bestehende
dependency-security-Workflows). Stale/vulnerable Pins **failen closed** —
kein stiller Rückfall auf Version-Ranges.

## 4. Update-Prozess
`== pin` → Security-/OSV-Auswertung → Owner-Approved-Update →
Exact-SHA-Evidence je Änderung (Run-/Job-/Check-Referenz im PR).

## 5. Review-Klasse
Dependency-Bump-PRs: Review durch Owner (Approver); Agent bereitet vor,
approvt nie (AD-035-Rollenmodell).

## 6. Failure Mode
Fehlende/veraltete Pins = BLOCKED (ATC-DEP-RESULT-1), niemals
Gate-Abschaltung. Behebung nur über diesen Vertrag.

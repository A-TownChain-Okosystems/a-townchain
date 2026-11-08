---
document_id: ATC-DOC-LPD-SECURITY-001
title: Security Policy — ATC Launchpad
status: active
standard: ATC-STD-MD-001
owner: A-TownChain-Okosystems
version: 1.0.0
created: 2026-09-08
updated: 2026-09-08
---

# Security Policy — ATC Launchpad

**Klassifizierung:** APPLICATION · **Maturity:** R1 · **Security-Level:** S1 (ATC-STD-202)
**Criticality:** low · **Standard:** ATC-STD-203 (Repository Security & Release)

## Melde- und Offenlegungspolitik

1. Sicherheitsrelevante Befunde und Schwachstellen dürfen **nicht öffentlich** als GitHub Issue gemeldet werden (official ATC security reporting). Melden Sie diese direkt an den Owner (ShivaCoreDev / ATC Security Team).
2. Sicherheitsrelevante Änderungen laufen als `security(...)`-Commits und werden im zentralen DECISIONS_REGISTER dokumentiert.
3. Kritische Vorkommnisse folgen dem Emergency-Prozess (ATC-STD-000 §32).

## Geltende Regeln

- Keine Secrets, Keys oder Credentials im Repository (Hygiene-Regel V-11).
- Release-Gates GATE-001…GATE-010 (ATC-STD-203) vor jedem Release.

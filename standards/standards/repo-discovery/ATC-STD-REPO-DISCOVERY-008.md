---
standard:
  id: ATC-STD-REPO-DISCOVERY-008
  title: "Security-Relevant Content Detection"
  version: "1.0.0"
  status: approved
  category: repo-discovery
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle ATC-Repositories; Agenten, CI, Audits, Wartungszyklen"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-REPO-DISCOVERY-008 — Security-Relevant Content Detection (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33
> (08.09.2026, SCR-0046). Familie FAM-49 (Repository Content Discovery,
> Kategorie repo-discovery). **Prioritaet:** P1 · **Grundregel der Familie:**
> Kein neuer Repository-Inhalt darf unbewertet bleiben.

## Zweck
Security-relevante Inhalte (Permissions, Secrets, Auth, Policies) und
P0-ausloesende Inhalte werden erkannt: Secret committed, critical security
bypass, consensus-breaking change → unmittelbarer Audit-Fund P0 (BUG-001),
kein wartender Scan-Zyklus.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-RD-080 | Security-Bereiche (Permissions/Auth/Secrets/Policies) werden als SECURITY_RELEVANT klassifiziert |
| REQ-RD-081 | P0-Kandidaten (Secret, Bypass, Konsens-Bruch) sofort als BUG-001-Fund P0 melden |
| REQ-RD-082 | Security-relevante Aenderungen erfordern Security Review (203) vor Abschluss |

## Implementierungsstatus
SPECIFIED — Secret-Scanning aktiv (203); P0-Sofortmeldungs-Hook an
Governance-CI/Signatur-Check folgt mit Scan-Integration.

## Security Considerations

P0-Sofortfund (Secret, Bypass, Konsens-Bruch) meldeklar an BUG-001; .gitignore/Secret-Scanning pruefen bei jedem Scan.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0046) |

## References
NORMATIV: ATC-STD-203, BUG-001, DISC-INV-008 · INFORMATIVE: SCR-0046

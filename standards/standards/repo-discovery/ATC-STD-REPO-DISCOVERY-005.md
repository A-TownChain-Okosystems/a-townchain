---
standard:
  id: ATC-STD-REPO-DISCOVERY-005
  title: "Cross-Repository Discovery"
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

# ATC-STD-REPO-DISCOVERY-005 — Cross-Repository Discovery (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33
> (08.09.2026, SCR-0046). Familie FAM-49 (Repository Content Discovery,
> Kategorie repo-discovery). **Prioritaet:** P1 · **Grundregel der Familie:**
> Kein neuer Repository-Inhalt darf unbewertet bleiben.

## Zweck
Ein neu erkannter Inhalt MUSS darauf geprueft werden, ob dieselbe fachliche
Domaene bereits in anderen ATC-Repositories existiert (z.B. Repo A:
ATC-STD-AI-AGENT-001, Repo B: AI_AGENT_RULES.md, Repo C: AGENT_POLICY.md) →
Cross-Repository Analysis → Standard-Konsolidierung. Das verhindert, dass sich
Regeln unkontrolliert in Einzelrepositories entwickeln.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-RD-050 | Je Standard-Kandidat: Cross-Repo-Suche ueber aktive Repos |
| REQ-RD-051 | Domae-Dubletten werden als Konsolidierungs-Fund markiert (BUG-001-Severity) |
| REQ-RD-052 | Konsolidierung via SCR/ERR-005-Mechanik, niemals stillschweigend |

## Implementierungsstatus
SPECIFIED — Cross-Repo-Mechanik existiert (ERR-005, kai_os_sync-Repo-Liste 26
Repos); Kandidaten-Anbindung folgt mit Scan-Integration.

## Security Considerations

Cross-Repo-Discovery nutzt nur autorisierte Repository-Zugriffe der Sync-Agents; Ergebnisse ohne Zugangsdaten.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0046) |

## References
NORMATIV: ERR-005 (Cross-Repo-Scan), RD-003/-004 · INFORMATIVE: SCR-0046

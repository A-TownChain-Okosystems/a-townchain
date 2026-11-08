---
standard:
  id: ATC-STD-ERR-005
  title: "Cross-Repository Error Scan"
  version: "1.0.0"
  status: approved
  category: err
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Gesamtes A-TownChain-Oekosystem: Repositories, Software, Smart Contracts, Doku, Standards, APIs, KI-Agenten, Infrastruktur, Prozesse"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-ERR-005 — Cross-Repository Error Scan (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P0 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Bei systemisch relevantem Fehler (P0/P1) wird die GESAMTE Organisation geprueft —
alle Repositories der Organisation.

## Kernregeln
Je betroffenem Repository werden mindestens geprueft (5 Pruefmatrizen):
1. **Source Code:** gleiche Implementierung/Funktion/Logik/Konstanten/Algorithmen/
   Konfiguration.
2. **Dokumentation:** README, Markdown, Wiki, Specs, Architecture Docs, API Docs,
   Tutorials, Examples.
3. **Konfiguration:** YAML, JSON, TOML, XML, .env.example, Docker, Kubernetes,
   Terraform, CI/CD.
4. **Tests:** Unit, Integration, E2E, Regression, Security.
5. **Standards:** ATC Standards, Protocol/Coding/Repository/Versionierungs-/
   Smart-Contract-Standards (Abgleich gegen Standards-Registry, SSOT).

Der Scan dokumentiert `repositories_checked` und je Repository das Ergebnis
(betroffen/nicht betroffen/UNKNOWN).

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-050 | Org-weiter Scan je P0/P1-Fehler ueber alle aktiven Repos (mind. 5 Matrizen) |
| REQ-ER-051 | Ergebnis je Repository dokumentiert; repositories_checked/occurrences_found im Record |

## Implementierungsstatus
SPECIFIED — Umsetzung: Cross-Repo-Scan-Bericht im Knowledge Record; Kopplung an
kai_os_sync-Repo-Liste (26 Repos).

## Security Considerations

Cross-Repo-Scans verwenden nur Repository-Zugriff mit read-only Rechten; Ergebnisse enthalten keine Secrets.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-ERR-000/-004, ATC-STD-REPO-AUDIT-002 (Check-Engine) ·
INFORMATIVE: SCR-0044

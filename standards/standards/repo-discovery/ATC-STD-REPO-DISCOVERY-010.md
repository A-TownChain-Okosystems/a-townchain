---
standard:
  id: ATC-STD-REPO-DISCOVERY-010
  title: "Discovery Audit & Reporting"
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

# ATC-STD-REPO-DISCOVERY-010 — Discovery Audit & Reporting (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33
> (08.09.2026, SCR-0046). Familie FAM-49 (Repository Content Discovery,
> Kategorie repo-discovery). **Prioritaet:** P1 · **Grundregel der Familie:**
> Kein neuer Repository-Inhalt darf unbewertet bleiben.

## Zweck
Maschinenlesbare, revisionssichere Discovery-Reports je Repository in
`.atc/discovery/`: baseline.json, latest-scan.json, changes.json, candidates.json,
impact-report.json. Report-Schema: scan_id (DISC-YYYY-MM-DD-NNN), baseline_commit,
current_commit, new/modified/deleted_content, standard_candidates,
security_review_required, documentation_required, cross_repository_review.

## Discovery-Invarianten (verbindlich)
DISC-INV-001 Jeder neue Repository-Inhalt muss erkannt werden.
DISC-INV-002 Jeder erkannte Inhalt muss klassifiziert werden.
DISC-INV-003 Jeder standardsrelevante Inhalt muss als Standard-Kandidat geprueft werden.
DISC-INV-004 Neue Standards duerfen keine bestehenden Standards duplizieren.
DISC-INV-005 Neue Standards muessen auf Cross-Repository-Konflikte geprueft werden.
DISC-INV-006 Jede relevante Aenderung muss auf Dokumentationsbedarf geprueft werden.
DISC-INV-007 Jede relevante Aenderung muss auf Testbedarf geprueft werden.
DISC-INV-008 Security-relevante Aenderungen benoetigen Security Review.
DISC-INV-009 Discovery-Ergebnisse muessen revisionssicher dokumentiert werden.
DISC-INV-010 Ein Discovery-Scan darf nicht als erfolgreich gelten, wenn seine Pruefung unvollstaendig ist.

## P0-P3-Einstufung (Kopplung BUG-001)
P0: unmittelbare kritische Risiken (Secret, Bypass, Konsens-Bruch) ·
P1: neue Funktionalitaet ohne erforderliche Standards/Doku/Tests ·
P2: Inkonsistenzen, fehlende Doku, kleinere Luecken · P3: Optimierungen.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-RD-100 | Report-Schema je Scan vollstaendig (12 Felder, 5 Dateien) |
| REQ-RD-101 | DISC-INV-001..010 je Scan als Checkliste abgezeichnet |
| REQ-RD-102 | P0-P3-Einstufung je Fund nach BUG-001-Schwere |
| REQ-RD-103 | Reports versioniert in .atc/discovery/ (git-getrackt = revisionssicher) |
| REQ-RD-104 | Unvollstaendige Pruefung = Scan FAIL (INV-010), kein SUCCESS |
| REQ-RD-105 | Discovery-Funde fliessen in REPO-AUDIT-002/Maintenance-Report ein |

## Implementierungsstatus
IMPLEMENTED (erster Zyklus) — Evidence: scan.py erzeugt baseline.json +
latest-scan.json + changes.json; candidates/impact folgen mit Ausbaustufe.

## Security Considerations

Discovery-Reports in .atc/discovery/ sind git-getrackt und muessen frei von Zugangsdaten sein; INV-010 verhindert Schein-Sicherheit bei unvollstaendigen Scans.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0046) |

## References
NORMATIV: BUG-001/002, REPO-AUDIT-002, REPO-MAINT-001 §16, IMPLEMENTATION-001 ·
INFORMATIVE: SCR-0046, .atc/discovery/

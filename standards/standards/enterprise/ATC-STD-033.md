---
standard:
  id: ATC-STD-033
  title: "Vulnerability Intelligence Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories und die Organisation gesamt"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
----

# ATC-STD-033 — Vulnerability Intelligence Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-033 · Advisory-Aggregation und ATC-Betroffenheitsanalyse · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-031" — §37 ergab 033.

## Abstract

ATC-STD-033 macht aus allgemeinen Advisories konkrete ATC-Befunde: Aggregation
über 10 Quellklassen und eine verbindliche Betroffenheitskette (Technology →
Repository → Version → Deployment → Risk Assessment).

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für externe und interne Schwachstellen-Informationen. Bewertung und
Priorisierung: ATC-STD-034; Reaktion: ATC-STD-020/022/035.

## §1 Quellen (REQ-STD-001, MUST)

Erfasst MÜSSEN mindestens werden: CVE, GHSA, OSV, RustSec, Vendor Advisories,
CERT Advisories, Security-Researcher-Berichte, ATC-interne Findings,
Penetration-Test-Befunde, Bug-Bounty-/Incident-Reports. Das System MUSS NICHT
von einer einzelnen Datenbank abhängig sein.

## §2 Betroffenheitskette (REQ-STD-002, MUST)

Jedes Advisory MUSS die Kette durchlaufen:

Advisory → Affected Technology? → Affected ATC Repository? → Affected Version?
→ Affected Deployment? → Risk Assessment

Nur bestätigte Betroffenheit erzeugt einen ATC-VULN-Record (ATC-STD-020 §8);
die Kette MUSS mit dem Technology-Registry-/Inventar-Abgleich (ATC-STD-018 §1)
nachvollziehbar sein.

## §3 Record-Pflicht (REQ-STD-003, MUST)

Bestätigte Betroffenheit MUSS als ATC-VULN-Record unter evidence/vulnerabilities/
geführt werden; Falsch-Positive MÜSSEN dokumentiert werden (kein stilles Verwerfen).

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Quellen: 10 Quellklassen MÜSSEN erfasst sein; keine Einzel-Datenbank-Abhängigkeit.
- id: REQ-STD-002 — Betroffenheitskette: 5-Stufen-Abgleich MUSS je Advisory nachvollziehbar sein.
- id: REQ-STD-003 — Records: bestätigte Betroffenheit MUSS VULN-Record erzeugen; Falsch-Positive MÜSSEN dokumentiert sein.

## Compliance

Prüfung: Intelligence-Feed-Verdrahtung (CI/Org-Audit), Record-Vollständigkeit.

## Security Considerations

- Feed-Poisoning: Advisories MÜSSEN vor Exploit-Details geprüft werden;
  automatisierte Reaktionen auf ungeprüfte Feeds sind verboten.
- Interne Findings mit Exploit-Details MÜSSEN zugriffsbeschränkt bleiben.

## Implementierungsstatus

**Status: SPECIFIED** — Feed-Aggregation mit Assurance-Engine-Rollout. SSOT:
registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) — Quellen,
  Betroffenheitskette, Record-Pflicht. CANDIDATE.

## References

- ATC-STD-020 — VULN-Records · ATC-STD-034 — Exploitability
- ATC-STD-018 §1 — Technologie-Inventar

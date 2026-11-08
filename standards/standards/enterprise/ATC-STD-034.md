---
standard:
  id: ATC-STD-034
  title: "Exploitability Assessment Standard"
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

# ATC-STD-034 — Exploitability Assessment Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-034 · kontextabhängige Schwachstellen-Priorisierung · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-032" — §37 ergab 034.

## Abstract

ATC-STD-034 verhindert Überreaktionen wie gefährliche Unterbewertung: Eine
Schwachstelle allein reicht nicht für korrekte Priorisierung — bewertet wird
im ATC-Kontext (Existence, Exposure, Exploitability, Impact, Privileges
Required, Attack Complexity, Network Reachability).

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle bestätigten Betroffenheiten (ATC-STD-033 §2). Das Assessment
verändert die PRIORISIERUNG, nie die Pflicht zur Behebung.

## §1 Bewertungsfaktoren (REQ-STD-001, MUST)

Jede bestätigte Schwachstelle MUSS bewertet werden über: Existence, Exposure,
Exploitability, Impact, Privileges Required, Attack Complexity, Network
Reachability. Beispiel: CVE betrifft ATC-Komponente, ist aber nicht öffentlich
erreichbar → Exploitability reduziert → trotzdem Patch erforderlich.

## §2 Kein Abwertungs-Fehlschluss (REQ-STD-002, MUST)

Reduzierte Exploitability DARF NICHT als "nicht betroffen" gewertet werden:
Ein Patch MUSS im normalen Lifecycle (ATC-STD-022 SLA der zutreffenden Stufe)
erfolgen; nur der SLA-Kontext darf sich verschieben — ausgenommen Critical
(ATC-STD-018 §3 bleibt unantastbar).

## §3 Dokumentation (REQ-STD-003, MUST)

Das Assessment MUSS im ATC-VULN-Record (Felder exploitability/exposure/impact)
nachvollziehbar sein; Bewertung MUSS bei Expositionsänderung (Deployment-Änderung)
neu geprüft werden.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — 7 Faktoren MÜSSEN je Schwachstelle bewertet werden.
- id: REQ-STD-002 — Reduzierte Exploitability MUSS NICHT Behebungspflicht aufheben.
- id: id: REQ-STD-003 — Assessment MUSS bei Expositionsänderung neu geprüft werden

## Compliance

Prüfung: Record-Vollständigkeit (Felder gefüllt), Re-Assessment bei
Expositionsänderungen.

## Security Considerations

- Assessment-Irrtum ist Risiko: konservative Bewertung SOLLTE Standard sein;
  Herabstufung MUSS begründet sein.
- Assessments über Expositions-Inventory (ATC-STD-028) — unbekannte Exposition
  macht das Assessment ungültig.

## Implementierungsstatus

**Status: SPECIFIED** — angewandt ab erstem VULN-Bestand. SSOT:
registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) — 7
  Bewertungsfaktoren, Abwertungsschutz, Dokumentationspflicht. CANDIDATE.

## References

- ATC-STD-033 — Vulnerability Intelligence · ATC-STD-020 §8 — VULN-Record
- ATC-STD-028 — Attack Surface · ATC-STD-022 — Patch-SLAs

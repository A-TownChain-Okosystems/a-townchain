---
standard:
  id: ATC-STD-026
  title: "Security Regression Prevention Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories mit behobenen Security-Vulnerabilities"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
    - ATC-STD-020
    - ATC-STD-022
----

# ATC-STD-026 — Security Regression Prevention (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Continuous Assurance 11.09.2026
> (SCR-0094; Owners Referenz "ATC-STD-025" — Slot 025 ist durch den Standards
> Review Standard belegt, §37 ID-Allokation ergab 026). Bis zur §9-Freigabe nicht
> wirksam. **Scope:** ATC-STD-026 · Fix-zu-Test-Pflicht · **Governance:** ATC-STD-000

## Abstract

ATC-STD-026 verankert das Prinzip: Every fixed security vulnerability becomes a
permanent test case. Jede behobene Schwachstelle MUSS einen dauerhaften
Regressionstest erzeugen, damit derselbe Fehler nicht unbemerkt zurückkehren kann.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle behobenen Security-Vulnerabilities (CVE/GHSA/OSV-Referenz oder
interne ATC-VULN-Records). Nicht-Gegenstand: allgemeine Bug-Regressionen
(ATC-STD-024 §2).

## §1 Fix-zu-Test-Pflicht (REQ-STD-001, MUST)

Jede behobene Security-Vulnerability MUSS einen permanenten Regressionstest
erzeugen:

CVE-XXXX (bzw. ATC-VULN-NNN) → Fix → Security Test → CI → Future Regression Protection

Der Test MUSS den Fehlerzustand vor dem Fix als rot-erwartenden Fall abbilden
(Proof of Fix), MUSS in der CI-Suite dauerhaft laufen und DARF NICHT entfernt
werden, außer via SCR mit Begründung.

## §2 Test-Qualität (REQ-STD-002, MUST)

Der Regressionstest MUSS die Schwachstellenklasse (nicht nur den Einzelfall)
adressieren, wo technisch möglich (Klassen-Test nach ATC-STD-018 §4
Angriffsklassen). Mutationstest-Feuerprobe SOLLTE bestätigen, dass der Test den
Fix wirklich verankert (M8-Prinzip der Governance-Tests).

## §3 Nachweis (REQ-STD-003, MUST)

Der Test MUSS im Vulnerability-Record (ATC-STD-020 v1.1.0: regression_test) und
als Regressionstest-Verweis in der CI-Konfiguration des Repos dokumentiert sein;
fehlender Regressionstest IST ein Finding (P1).

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Fix-zu-Test-Pflicht: jede behobene Vulnerability MUSS einen permanenten CI-Test erzeugen.
- id: REQ-STD-002 — Test-Qualität: Klassen-Adressierung MUSS wo möglich; Mutationstest SOLLTE.
- id: REQ-STD-003 — Nachweis: Record- und CI-Dokumentation MUSS vorhanden sein; Fehlen IST Finding.

## Compliance

Prüfung: Audit gegen abgeschlossene Vulnerability-Records ohne zugehörigen
Regressionstest (org-weiter Scan); CI-Verdrahtung je Repo.

## Security Considerations

- Regressionstests mit Exploit-Reproduktion MÜSSEN zugriffsbeschränkt sein, solange
  die Schwachstelle nicht öffentlich ist.
- Test-Sammlung selbst wartungsbedürftig: Review-Termin je Suite (REPO-MAINT-001).

## Implementierungsstatus

**Status: SPECIFIED** — normativ spezifiziert; Evidence ab erster angewandter
Fix-zu-Test-Kette. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Continuous Assurance (SCR-0094) —
  Fix-zu-Test-Pflicht, Qualitäts- und Nachweisanforderungen. CANDIDATE.

## References

- ATC-STD-000 — Governance Root
- ATC-STD-018 — Vulnerability Management
- ATC-STD-020 — Incident & Vulnerability Response (v1.1.0: Records)
- ATC-STD-022 — Security Patch Management (§3)
- ATC-STD-024 — Known Bug Management (Abgrenzung)

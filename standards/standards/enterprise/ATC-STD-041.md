---
standard:
  id: ATC-STD-041
  title: "Reproducible & Verifiable Builds Standard"
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

# ATC-STD-041 — Reproducible & Verifiable Builds Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-041 · beweisbare Source-zu-Artefakt-Kette · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-038" — Slot 040 belegt (Legacy),
> §37 ergab 041.

## Abstract

ATC-STD-041 verlangt: Ein Release MUSS beweisbar aus dem Source Code
entstanden sein — Source → Pinned Dependencies → Controlled Toolchain →
Reproducible Build → Artifact Hash → Signature → SBOM → Release. Damit wird
ein Supply-Chain-Angriff deutlich schwerer.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für Releases aller kritischen Repos (C1/S4). Sonstige Builds: Best-Effort
mit dokumentierter Abweichung.

## §1 Build-Kette (REQ-STD-001, MUST)

Releases MÜSSEN die Kette erfüllen: Pinned Dependencies (Lockfile,
ATC-STD-019 §1), Controlled Toolchain (Version dokumentiert, ATC-STD-100),
Reproducible Build (identischer Hash bei Rebuild, wo technisch möglich),
Artifact Hash, Signature (GPG/Key-Lage dokumentiert — Signierung bis
Release-Key verfügbar als dokumentierter Übergangsstand, AUD-001 F-006),
SBOM.

## §2 Reproduzierbarkeit (REQ-STD-002, MUST wo anwendbar)

Build MUSS deterministisch sein (Zeitstempel-/Umgebungsnormalisierung);
nicht-reproduzierbare Artefakte MÜSSEN als Einschränkung dokumentiert sein
und im Gate als Teilzeile erscheinen.

## §3 Verifikation (REQ-STD-003, MUST)

Artefakt-Hash MUSS bei Release mitgeliefert und gegen den Build-Nachweis
prüfbar sein; Third-Party-Rebuild SOLLTE denselben Hash liefern können.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Build-Kette MUSS vollständig erfüllt und dokumentiert sein.
- id: REQ-STD-002 — Deterministische Builds MUSS wo anwendbar; Abweichungen MÜSSEN dokumentiert sein.
- id: REQ-STD-003 — Hash-Verifikation MUSS möglich sein; Rebuild SOLLTE.

## Compliance

Prüfung: Release-Records (evidence/releases/), Hash-Abgleich, GATE-Zeile
Supply Chain.

## Security Considerations

- Toolchain selbst angreifbar (Build-Infrastruktur): Isolation und
  Least-Privilege MÜSSEN gelten (ATC-STD-019 §7).
- Signaturen ohne Key-Schutz wertlos: Key-Management nach ATC-STD-038.

## Implementierungsstatus

**Status: SPECIFIED** — erste Releases im Rebuild; GPG-Signierung hängt am
Release-Key (AUD-001 F-006-Familie, ATC-EXC-001-Kontext). SSOT:
registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) —
  Build-Kette, Determinismus, Hash-Verifikation. CANDIDATE.

## References

- ATC-STD-019 — Supply Chain · ATC-STD-043 — Artifact Integrity
- ATC-GATE-SEC-001 — Supply-Chain-Zeilen

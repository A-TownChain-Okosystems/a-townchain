---
standard:
  id: ATC-STD-029
  title: "Threat Modeling Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle kritischen ATC-Komponenten (§1-Liste)"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
    - ATC-STD-028
----

# ATC-STD-029 — Threat Modeling (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Continuous Assurance 11.09.2026
> (SCR-0094; Owners Referenz "ATC-STD-027" — Slot 027 ist durch den Standards
> Deprecation Standard belegt, §37 ergab 029). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-029 · Threat-Model-Pflicht für kritische Komponenten ·
> **Governance:** ATC-STD-000

## Abstract

ATC-STD-029 verlangt vor bzw. mit Bau größerer Komponenten ein Threat Model entlang
einer verbindlichen Analysekette — Sicherheit durch Design statt nachträglich.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für die kritischen Komponenten des Ökosystems: ATC Node, ATC-VM, ATCLang,
Wallet, Smart Contracts, Oracle, Interop, ZKP, Mining, Consensus, Storage,
Indexer, Explorer, Marketplace, Launchpad, Aurora, ShivaCore, GlobusOS — sowie
jede künftig als C1/S4 klassifizierte Komponente (ATC-STD-202/018).

## §1 Analysekette (REQ-STD-001, MUST)

Jedes Threat Model MUSS entlang der Kette erstellt und je Glied dokumentiert sein:

Asset → Actor → Attack Vector → Threat → Impact → Likelihood → Control →
Residual Risk

Residual Risk MUSS bewertet und akzeptiert (JUSTIFIED, ATC-STD-028 §2) oder als
Finding geführt werden.

## §2 Erstellungs-Zeitpunkt (REQ-STD-002, MUST)

Das Threat Model MUSS vor bzw. spätestens mit Implementierungsbeginn der
Komponente existieren; für bestehende Komponenten der §Scope-Liste MUSS es
nachgeholt werden (Technology-Review-Kopplung, ATC-STD-018 §7).

## §3 Auffrischung (REQ-STD-003, MUST)

Threat Models MÜSSEN bei relevanten Änderungen (neue Schnittstelle, neue
Angriffsklasse, Konsens-Änderung) aufgefrischt und im Review-Termin des
Technologie-Inventars mitgeprüft werden.

## §4 Evidence (REQ-STD-004, MUST)

Threat Models MÜSSEN unter evidence/security/ (je Komponente) abgelegt sein;
fehlendes Threat Model IST NOT VERIFIED für die Gate-Zeile (ATC-GATE-SEC-001,
kritikalitätsabhängig).

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Analysekette: 8-Glied-Kette MUSS je Threat Model vollständig dokumentiert sein.
- id: REQ-STD-002 — Zeitpunkt: MUSS vor/mit Implementierung existieren; Nachholpflicht für Bestandskomponenten.
- id: REQ-STD-003 — Auffrischung: MUSS bei relevanten Änderungen und im Review-Zyklus erfolgen.
- id: REQ-STD-004 — Evidence: Ablage unter evidence/security/ MUSS; Fehlen IST NOT VERIFIED.

## Compliance

Prüfung: Gate-Abgleich (ATC-GATE-SEC-001 Zeile Threat Model), Evidence-Verzeichnis,
Review-Termine.

## Security Considerations

- Threat Models enthalten Angriffspfade: MÜSSEN zugriffsbeschränkt sein; nur
  Existenz-/Vollständigkeitsnachweis ist öffentlich.
- Threat Modeling ersetzt NICHT Tests (Baseline: ATC-STD-018 §5) — Model
  informiert Tests, Tests verifizieren Model.

## Implementierungsstatus

**Status: SPECIFIED** — normativ spezifiziert; Evidence ab erstem Threat Model
je Komponente (Priorität: atc-node, atc-vm, atc-shivacore als TCB). SSOT:
registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Continuous Assurance (SCR-0094) —
  Analysekette, Zeitpunkt-/Nachholpflicht, Auffrischung, Evidence-Pflicht. CANDIDATE.

## References

- ATC-STD-000 — Governance Root
- ATC-STD-018 — Security Baseline (kritische Repos: Threat Model)
- ATC-STD-028 — Attack Surface Management (Vektor-Sicht des Modells)
- ATC-GATE-SEC-001 — Assurance Gate

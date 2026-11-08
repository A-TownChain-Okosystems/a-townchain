---
standard:
  id: ATC-STD-039
  title: "Cryptographic Agility Standard"
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

# ATC-STD-039 — Cryptographic Agility Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-039 · Austauschbarkeit kryptografischer Algorithmen · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-037" — §37 ergab 039.

## Abstract

ATC-STD-039 verlangt Krypto-Agilität: Kryptografie MUSS über austauschbare
Interfaces implementiert sein (Current/Alternative/Future), sodass
Algorithmuswechsel bei Cryptographic Break, Deprecation, Quantum Threat oder
Implementierungs-Schwachstelle ohne System-Redesign möglich ist. Besonders
relevant für A-TownChain, ATC-VM, Wallet, ZKP, Interop, ShivaCore,
Signaturen und Identity.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle kryptografisch aktiven Komponenten (L1-L3-TCB sowie ZKP).

## §1 Agilitäts-Interface (REQ-STD-001, MUST)

Kryptografie MUSS hinter definierten Interfaces liegen (Crypto Interface mit
Current/Alternative/Future-Slots); direkte Algorithmus-Hardcoding MUSS NICHT
in Konsens-/Signatur-/Wallet-Pfaden vorkommen.

## §2 Wechselkriterien (REQ-STD-002, MUST)

Ein Wechsel MUSS ausgelöst werden bei: Cryptographic Break,
Algorithm Deprecation (EOL, ATC-STD-018 §2), Quantum Threat (Transition
sauber geplant, ATC-STD-023 Lifecycle), Implementation Vulnerability
(Library-Advisory, ATC-STD-033).

## §3 Keine Eigenbau-Kryptografie (REQ-STD-003, MUST)

Es MUSS auf geprüfte, etablierte Krypto-Bibliotheken gesetzt werden
(ATC-STD-TUD-001 REQ-TUD-Honest-Claim/Eigenbau-Krypto-Verbot); eigene
Primitive sind VERBOTEN außer mit dokumentiertem, peer-geprüftem Nachweis.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Crypto-Interfaces MÜSSEN austauschbar sein; Hardcoding MUSS NICHT in Kernpfaden.
- id: REQ-STD-002 — Wechselkriterien MÜSSEN überwacht und ausgelöst werden.
- id: REQ-STD-003 — Eigenbau-Kryptografie ist VERBOTEN; geprüfte Bibliotheken MUSS verwendet werden.

## Compliance

Prüfung: Krypto-Use-Inventory je Komponente (evidence/security/), Advisory-
Kopplung (ATC-STD-033), Technology-Registry-Eintrag der Bibliotheken
(ATC-STD-023 §4).

## Security Considerations

- Agilitäts-Wrapper selbst sind Code: Review-Pflicht + Regressionstests bei
  Algorithmuswechsel (ATC-STD-026).
- Migrationen brauchen Parallelbetrieb (Dual-Verify) während des Übergangs,
  um Konsens-Kontinuität zu sichern.

## Implementierungsstatus

**Status: SPECIFIED** — Entwurfsverpflichtung für den Rebuild (ZKP-001..007
vor Rollup, G18 Security Audit vor Freeze); Crypto-Inventory je Komponente
ausstehend. SSOT: registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) —
  Agilitäts-Interfaces, Wechselkriterien, Eigenbau-Verbot. CANDIDATE.

## References

- ATC-STD-TUD-001 — Honest-Claim/Eigenbau-Krypto-Verbot
- ATC-STD-018/023 — Deprecation/Lifecycle · ATC-STD-026 — Regression

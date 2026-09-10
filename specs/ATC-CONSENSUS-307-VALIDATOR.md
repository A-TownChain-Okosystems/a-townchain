---
spec_id: ATC-CONSENSUS-307
title: "Validator Lifecycle Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: atc-algorithm
layer: L3-Konsens
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends: []
---

# Validator Lifecycle Specification (ATC-CONSENSUS-307)

> **Ehrlicher Status:** Spezifikations-Grundgerüst (SCR-0071, Owner-Audit-Backlog).
> Implementierung, Tests und Evidence PENDING — gemäß „No status without
> evidence" behauptet diese Datei keinerlei funktionierenden Zustand.

## 1. Zweck

Registration, Aktivierung, Pflichten, Exit und Strafen von Validatoren — getrennt vom Selection-Algorithmus (ATC-CONSENSUS-302).

## 2. Scope (gilt für)

- Registration & Mindest-Stake
- Aktivierungs-/Exit-Epochen
- Voting-Pflichten & Downtime-Penalties
- Slashing-Katalog

## 3. Normative Anforderungen (MUST)

- **REQ-VAL-001:** Registration ist on-chain und bindet Pubkey + Stake; Aktivierung frühestens zum Epochenstart nach Inclusion — *Nachweis: unit+integration*
- **REQ-VAL-002:** Exit (unbonding) ist erst nach UNBOND_EPOCHS (genesis-locked) wirksam; währenddessen bleiben Pflichten und Slashbarkeit bestehen — *Nachweis: vector+negative*
- **REQ-VAL-003:** Slashing-Katalog (verbindlich): Equivocation (ATC-CONSENSUS-306), Surround-Vote, ungültiger Proposal — jede Strafe ist im Vorhinein deterministisch, keine diskretionären Strafen — *Nachweis: adversarial+vector*
- **REQ-VAL-004:** „5-Validator-Key«-Set ist als Development-/Genesis-Bootstrap-Konfiguration klassifiziert (kein dauerhaftes Konsenslimit) und MÜSSTE als genesis_parameter mit Limiter dokumentiert sein — *Nachweis: negative+config*

## 4. Datenmodelle & Schnittstellen

(Datenmodelle werden beim Spec-Freeze finalisiert; diesem Grundgerüst liegen die untenstehenden Anforderungen zugrunde.)

## 5. Invarianten

- Validator-Set-Änderungen wirken ausschließlich an Epchengrenzen — nie mitten in einem Slot

## 6. Conformance-Tests (Mindestkategorien)

- validator_lifecycle.json
- epoch_boundary.json
- slash_schedule.json

## 7. Abhängigkeiten & Kompatibilität

Kompatibilität zu ATC-STD-COMPAT-001 (MAJOR-Gate); Änderungen nur via SCR/MINOR (ATC-STD-UPDATE-001).

## 8. Status-Gates (Reihenfolge verbindlich)

- [ ] Spec-Freeze (Owner-Review §9; danach normativ)
- [ ] Implementierung (Rust) mit je-Anforderung-Nachweis
- [ ] Conformance-Suite grün (CI-Evidence: Run-ID + Commit-SHA)
- [ ] Security-Review (threat-bezogen)

## 9. Referenzen

- Owner-Audit 10.09. (P1: „5-Validator-Key ist nicht ausreichend spezifiziert«)

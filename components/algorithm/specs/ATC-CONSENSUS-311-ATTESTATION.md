---
spec_id: ATC-CONSENSUS-311
title: "Consensus Attestation Contract"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain/components/algorithm
layer: L2-Blockchain-Core
owner: A-TownChain-Okosystems
created: 2026-10-02
depends:
  - ATC-CONSENSUS-306
  - ATC-CONSENSUS-307
  - ATC-CONSENSUS-309
  - ATC-CONSENSUS-310
---

# Consensus Attestation Contract — ATC-CONSENSUS-311

## 1. Canonical attestation input

Eine Attestation MUSS mindestens binden:

- chain_id
- consensus_version
- target proposal/block hash
- target height
- target slot / epoch
- validator identity/key reference
- validator-set snapshot identifier
- source/finality context
- canonical attestation domain/version
- validator signature

## 2. Validation order

`decode → canonical-encoding check → context binding → validator snapshot membership → signature verification → duplicate/equivocation detection → stake accounting`

Eine Attestation darf erst nach erfolgreicher Signatur- und Kontextprüfung Gewicht zum Quorum beitragen.

## 3. Voting scope

Die konkrete Source/Target-Voting-Regel bleibt bis zum Freeze offen. Implementierungen dürfen keine eigene Voting-Domäne erfinden.

## 4. Equivocation

Zwei konfliktierende Attestations desselben Validators sind nur dann Equivocation, wenn sie innerhalb der eingefrorenen Voting-Domäne gleichzeitig konfliktieren. Beweisformat und Slashing-Aktion werden durch ATC-CONSENSUS-306/307 eingefroren.

## 5. Determinism

Identische kanonische Attestation-Menge + identischer Validator-Snapshot => identisches Ergebnis der Validierung und identische gewichtete Eingabemenge für Finality.

## 6. Failure semantics

Ungültige Signatur, unbekannter Validator, falscher Snapshot, falscher Chain-ID, falsche Consensus-Version, Replay oder nicht-kanonische Encoding => REJECT; kein Gewicht darf angerechnet werden.

## 7. Conformance vectors

- valid attestation
- wrong chain
- wrong version
- wrong target
- wrong snapshot
- invalid signature
- replay
- duplicate
- equivocation
- canonical/non-canonical encoding

## 8. Freeze gate

Quorum-Arithmetik, Voting-Domäne, Snapshot-Regel, Signatur-/Domain-Vertrag und Wire-Encoding müssen vor Production-Freeze vollständig spezifiziert und Exact-SHA VERIFIED sein.

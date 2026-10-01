---
spec_id: ATC-CONSENSUS-310
title: "Consensus Proposal Contract"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain/components/algorithm
layer: L2-Blockchain-Core
owner: A-TownChain-Okosystems
created: 2026-10-02
depends:
  - ATC-CONSENSUS-301
  - ATC-CONSENSUS-304
  - ATC-CONSENSUS-307
  - ATC-CONSENSUS-309
  - ATC-STATE-001
---

# Consensus Proposal Contract — ATC-CONSENSUS-310

> Proposal ist ein kanonisches Consensus-Objekt, kein lokales Node-Event.

## 1. Canonical proposal input

Ein Proposal MUSS mindestens eindeutig binden:

- chain_id
- consensus_version
- parent_block_hash
- target height
- slot / epoch
- PoH state/seed commitment
- validator-set snapshot identifier
- proposer identity
- proposer-selection proof/context
- transaction commitment
- state-transition commitment
- resulting state-root commitment
- validated PoS/PoW/PoI contribution commitments
- canonical block/proposal encoding version

## 2. Validity

Ein Proposal ist nur gültig, wenn alle gebundenen Werte gegen den kanonischen State und die eingefrorenen Consensus-Verträge validieren.

Insbesondere MUSS gelten:

`parent.height + 1 = proposal.height`

und der Proposer MUSS im für den Slot eingefrorenen Validator-Snapshot zulässig sein.

## 3. Proposal ID

Die Proposal-ID MUSS deterministisch aus der kanonischen serialisierten Proposal-Repräsentation abgeleitet werden. Die Hashfunktion und Domain-Separation werden erst durch den entsprechenden eingefrorenen Crypto/Wire-Vertrag normativ.

## 4. State transition binding

Ein Proposal darf nur einen State-Transition-Kandidaten referenzieren, der gemäß ATC-STATE-001 deterministisch prüfbar ist. Proposal validation darf den State nicht partiell mutieren.

## 5. Failure semantics

Fehlende, inkonsistente, doppelte oder nicht kanonisch encodierte Felder => REJECT.

Ein Proposal mit ungültigem Consensus-Proof darf nicht attestiert werden.

## 6. Conformance vectors

- valid proposal
- parent mismatch
- height/slot mismatch
- wrong validator snapshot
- invalid proposer selection
- invalid PoH binding
- invalid PoS/PoW/PoI contribution
- transaction commitment mismatch
- state-root mismatch
- non-canonical encoding

## 7. Freeze gate

Keine Produktionsimplementierung darf diese Draft-Semantik als final betrachten, bevor Wire-Encoding, proposal-ID/hash domain, State-Root-Vertrag und alle referenzierten Consensus-Verträge Specification-Frozen und Exact-SHA VERIFIED sind.

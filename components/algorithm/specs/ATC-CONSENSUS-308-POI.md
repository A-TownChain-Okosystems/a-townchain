---
spec_id: ATC-CONSENSUS-308
title: "Proof-of-Intelligence Contribution Specification"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain/components/algorithm
layer: L2-Blockchain-Core
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-02
depends:
  - ATC-CONSENSUS-302
  - ATC-CONSENSUS-304
  - ATC-CONSENSUS-DET
---

# Proof-of-Intelligence Contribution Specification — ATC-CONSENSUS-308

> **Ehrlicher Status:** Neuer Draft auf Basis des Existing-First-Audits.
> Historische PoI-Artefakte existieren, enthalten aber keine kanonisch verifizierbare
> Proof-Semantik. Insbesondere ein frei gesetztes `poI_score` (z. B. `1.0`) ist
> **keine** Consensus-Proof und darf nicht als L1-Selection-Input verwendet werden.

## 1. Zweck

ATC-CONSENSUS-308 definiert die Schnittstelle für einen verifizierbaren Proof-of-Intelligence
(PoI), der als optionaler, deterministischer Beitrag in die PoH+PoS+PoW+PoI-Hybrid-Selection
eingehen kann.

PoI misst nicht pauschal "Intelligenz" oder die Qualität einer Person/eines Modells.
Consensus darf ausschließlich eine **prüfbare, reproduzierbare und protocol-bound
contribution** bewerten.

## 2. Nicht-Ziele

PoI definiert in diesem Draft **keine**:
- konkrete ML-Modellgüte als universellen Wert,
- subjektive Human-/AI-Bewertung,
- frei skalierbare Reputation,
- Float-basierte Score-Berechnung,
- Belohnungshöhe,
- finale Hybrid-Gewichte,
- finale Leader-Selection-Formel.

Diese Punkte bleiben bis zum Spec-Freeze offen und dürfen nicht implizit aus historischen Implementierungen übernommen werden.

## 3. Normative Anforderungen

- **REQ-POI-001:** Jeder PoI-Proof MUSS einen kanonischen Proof-Typ, eine Protokoll-/Consensus-Version und eine eindeutige Proof-ID enthalten.
- **REQ-POI-002:** Jeder Proof MUSS eindeutig an den Consensus-Kontext gebunden sein: Chain-ID, Slot/Epoch-Kontext, relevante PoH-Seed/Challenge und den Validator-/Contribution-Identitätsbezug.
- **REQ-POI-003:** Der Proof MUSS eine kanonische Beschreibung von Input-Commitment, Output-Commitment und Ausführungs-/Beitragsnachweis enthalten. Rohdaten oder Modellartefakte dürfen nicht als implizite Consensus-Inputs gelten.
- **REQ-POI-004:** Ein Proof MUSS deterministisch verifizierbar sein. Verifier dürfen keine Netzwerkabfragen, zufällige Seeds, lokale Uhrabweichungen oder nicht versionierte externe Zustände benötigen.
- **REQ-POI-005:** Ein Proof MUSS die verwendete Runtime-/Modell-/Provenance-Version kryptographisch bzw. über einen kanonischen Identifier binden, sofern der Proof-Typ diese Ausführung voraussetzt.
- **REQ-POI-006:** Replay über unterschiedliche Slots, Challenges, Chain-IDs, Validator-Identitäten oder Consensus-Versionen MUSS verhindert werden.
- **REQ-POI-007:** Der Proof MUSS eine definierte Gültigkeitsdomäne und Ablauf-/Epoch-Bindung besitzen. Ungebundene historische Proofs dürfen nicht dauerhaft als aktueller Selection-Beitrag wiederverwendet werden.
- **REQ-POI-008:** Sybil-/Duplication-Angriffe müssen durch eine explizite Identity-/Capability-Bindung und Proof-Eindeutigkeit adressiert werden. Eine bloße numerische Score-Erhöhung gilt nicht als Schutz.
- **REQ-POI-009:** Jeder PoI-Beitrag MUSS vor der Hybrid-Selection vollständig validiert werden. Invalid, expired, replayed, unsupported oder version-incompatible proofs MUST NOT contribute.
- **REQ-POI-010:** Consensus-State und Wire-Format dürfen PoI-Beiträge nicht als IEEE-754 Floating-Point speichern. Die kanonische Beitragsskala MUSS eine deterministische Integer-Repräsentation mit expliziten Bounds und Overflow-Regeln verwenden.
- **REQ-POI-011:** Proof-Verifikation und Contribution-Ableitung müssen logisch getrennt sein: ein gültiger Proof ist nicht automatisch ein maximaler Beitrag.
- **REQ-POI-012:** Der Beitrag MUSS deterministisch aus dem validierten Proof und dem kanonischen Consensus-Kontext abgeleitet werden; Implementierungen dürfen keine lokalen Heuristiken hinzufügen.
- **REQ-POI-013:** Ein Proof-Typ MUSS seine Verifikationskosten und Ressourcenannahmen deklarieren. Unbounded verifier work ist nicht zulässig.
- **REQ-POI-014:** Änderungen an Proof-Typen, Verifier-Semantik, Contribution-Skala oder Version-Binding erfordern eine neue/inkompatible Consensus-Version gemäß Governance- und Compatibility-Gates.
- **REQ-POI-015:** Jeder normative Proof-Typ MUSS vor Freeze mindestens einen positiven Golden Vector und negative Vectors für Mutation, Replay, Wrong-Chain, Wrong-Identity und Wrong-Version besitzen.

## 4. Canonical Proof Envelope (Draft)

Die folgenden Felder bilden die minimale Draft-Struktur; Byte-Encoding und exakte Feldbreiten werden im Encoding-/Freeze-Schritt verbindlich:

| Feld | Zweck |
|---|---|
| `proof_version` | Version des Proof-Formats |
| `proof_type` | kanonischer Proof-Typ |
| `consensus_version` | Bindung an Consensus-Semantik |
| `chain_id` | Bindung an Chain-Identity |
| `slot_or_epoch` | zeitliche Gültigkeitsdomäne |
| `validator_id` | Identitäts-/Capability-Bindung |
| `challenge` | PoH-/Consensus-abgeleitete Challenge |
| `input_commitment` | Commitment der kanonischen Inputs |
| `output_commitment` | Commitment des verifizierten Outputs |
| `runtime_id` | Runtime-/Ausführungsidentität, falls erforderlich |
| `model_id` | Modell-/Algorithmusidentität, falls erforderlich |
| `resource_commitment` | verifizierbare Ressourcen-/Ausführungsbindung, falls erforderlich |
| `proof_bytes` | eigentlicher kryptographischer/prüfbarer Nachweis |
| `contribution_bound` | maximale durch diesen Proof autorisierte Beitragsskala |

Keine dieser Draft-Felder darf als finale Wire-Spezifikation interpretiert werden, bevor der Encoding-/Conformance-Freeze abgeschlossen ist.

## 5. Contribution Semantics

Die Pipeline ist verbindlich in zwei Schritte getrennt:

1. `verify(proof, canonical_context) -> valid | invalid`
2. `derive_contribution(valid_proof, canonical_context) -> bounded_integer`

Der zweite Schritt MUSS:
- deterministisch,
- overflow-sicher,
- reproduzierbar,
- versionsgebunden,
- durch `contribution_bound` begrenzt

sein.

Die konkrete Bewertungsfunktion, Skalierung, Normalisierung und Hybrid-Gewichtung bleiben **UNFROZEN**.

## 6. PoH/PoS/PoW/PoI Integration

Die vorgesehene Consensus-Pipeline lautet:

```text
Validator State
   │
   ├── PoS authorization / stake input
   ├── PoI proof verification → bounded PoI contribution
   ├── PoW verification → bounded work contribution
   └── PoH state → canonical slot/seed context
             │
             ▼
      Hybrid Selection (ATC-CONSENSUS-304)
             │
             ▼
          Proposal
             │
             ▼
        Attestation
             │
             ▼
          Finality
             │
             ▼
        Fork Choice
             │
             ▼
       State Transition
```

PoH stellt in diesem Draft den deterministischen Kontext/Seed bereit; PoS autorisiert und
gewichtet Validatoren; PoW liefert einen verifizierten Work-Beitrag; PoI liefert einen
verifizierten Intelligence-/Compute-Contribution-Beitrag. Keine dieser Rollen allein
definiert die finale Selection.

## 7. Security Requirements

Threats, die vor Freeze mindestens behandelt werden müssen:

- replay / cross-slot replay
- cross-chain replay
- cross-validator attribution
- proof forgery / malformed proof
- duplicate proof submission
- Sybil identities
- model/runtime substitution
- benchmark gaming / task gaming
- verifier asymmetry
- resource exhaustion / verifier DoS
- contribution inflation
- version downgrade / cross-version acceptance
- correlated proof failure
- censorship of valid proofs

Ein Proof-of-Intelligence darf insbesondere nicht zu einer unbounded oracles-to-consensus
Abhängigkeit führen.

## 8. Conformance Minimum

Vor Spec-Freeze erforderlich:

- canonical serialization vectors
- positive verification vectors
- mutation-negative vectors
- wrong-chain vectors
- wrong-slot/challenge vectors
- wrong-validator vectors
- wrong-runtime/model vectors, sofern relevant
- replay vectors
- version-mismatch vectors
- contribution-boundary vectors
- overflow/underflow vectors
- differential verification across independent implementations

Exact-SHA evidence MUSS die vollständige Kette Source SHA → Run ID → Job → Step → Result binden.

## 9. Freeze Gates

- [ ] Proof-type registry frozen
- [ ] Canonical envelope and encoding frozen
- [ ] Verification algorithm(s) frozen
- [ ] Identity/capability binding frozen
- [ ] Challenge/PoH binding frozen
- [ ] Contribution derivation frozen
- [ ] Bounds and overflow semantics frozen
- [ ] Anti-replay / anti-Sybil rules frozen
- [ ] Verifier cost limits frozen
- [ ] Golden vectors committed
- [ ] Independent implementation differential pass
- [ ] Security review complete
- [ ] Exact-SHA conformance VERIFIED

Until every gate is complete, PoI is **SPEC-DRAFT / NON-FROZEN** and MUST NOT be treated as a production consensus authority.

## 10. References

- ATC-CONSENSUS-301 — Proof-of-History
- ATC-CONSENSUS-302 — Proof-of-Stake
- ATC-CONSENSUS-303 — Proof-of-Work
- ATC-CONSENSUS-304 — Hybrid Selection
- ATC-CONSENSUS-307 — Validator Contract
- ATC-CONSENSUS-DET — Determinism Contract
- Historical PoI simulation artifacts are retained only as migration evidence and are not normative.

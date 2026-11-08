---
document_id: ATC-DOC-CONSENSUS-001
title: "Konsens-Runden im atc-node (SCR-0117)"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-11-08
updated: 2026-11-08
standard: ATC-STD-MD-001
---

# Konsens-Runden im atc-node

## 1. Was jetzt läuft

Der Devnet-Node treibt einen Runden-basierten Konsens (`components/node/src/consensus.rs`).
Jede Runde wählt deterministisch einen Proposer, sammelt signierte Stimmen bis zum
2/3-Quorum und finalisiert den Block. Der Node finalisiert damit ohne äußeren Anstoß
alle zwei Sekunden einen Block; die Höhe wächst und ist über RPC abfragbar.

```
RPC (Zeilenprotokoll)         RPC (JSON-RPC)
  HEIGHT                        height
  BEST_HASH                     consensus
  CONSENSUS
```

## 2. Ablauf einer Runde

1. **Proposer-Wahl** — `ValidatorSet::select_proposer(height)` verteilt Slots nach
   Voting-Power (`height % total_power`, kumulativ). Die Wahl ist für alle Knoten
   mit gleichem Set identisch, ohne Absprache.
2. **Proposal** — der Proposer bildet die Payload aus den Transaktionen und den
   Blockhash **vor** dem Anhängen (`Chain::preview_hash`). Der Hash deckt Höhe,
   Vorgänger-Hash und Payload ab (SHA-256, ATC-BLOCK-V1).
3. **Votes** — jeder Validator signiert `height`, seine ID und den **vollen
   32-Byte-Blockhash** mit secp256k1/ECDSA. Domain-Trennung:
   `ATC-CONSENSUS-VOTE-V1 || chain_id || height || validator_id || block_hash`.
4. **Finalität** — erreicht die signierte Power 2/3 der aktiven Power, wird der
   Block angehängt und der Ledger committet. Sonst bleibt die Kette unverändert.

## 3. Quorum

`required_power(total) = ceil(2 * total / 3)`; Finalität gilt bei
`3 * signed_power >= 2 * total_power`. Bei drei Validatoren mit Power 1 sind zwei
Stimmen nötig, eine reicht nicht.

## 4. Was geprüft wird, bevor eine Stimme zählt

- Signatur gegen den im Set hinterlegten 33-Byte-Pubkey (öffentlich, kein Signierer-Schlüssel).
- Der Proposer des Votes muss der deterministisch gewählte Proposer dieser Höhe sein.
- Doppelstimme mit anderem Hash auf gleicher Höhe wird als **Equivocation** abgelehnt.
- Ein Vote ohne passenden Proposal oder für einen fremden Block hash zählt nicht.

## 5. Belege

- `components/node/src/consensus.rs` — 9 Unit-Tests: Quorum-Mathematik, Vote-Signatur
  deckt den vollen Hash ab, unzulässiger Proposer, Equivocation, Runde finalisiert
  genau einen Block, Proposer rotiert über Höhen, ohne Quorum bleibt die Kette
  unverändert, fremder Proposal blockiert die Runde.
- `components/node/src/rpc.rs` — 1 Test für `HEIGHT`, `BEST_HASH`, `CONSENSUS`
  (Zeilen- und JSON-RPC).
- Node-Suite: 68 Tests grün.
- Zwei unabhängig gestartete Knoten erreichten nach 14 Sekunden beide Höhe 7 mit
  identischem Blockhash (`ea2c7162b0353344af1268ad…`) — gleiche Proposer-Folge,
  gleiche Kette.
- `./tools/ecosystem.sh smoke` prüft jetzt zusätzlich: Höhe wächst, Quorum-Status
  lesbar, finaler Blockhash vorhanden.

## 6. Grenzen (ehrlich)

- **Kein Vote-Austausch zwischen Maschinen.** Der Gossip-Wire überträgt weiterhin nur
  Blöcke per Pull (SCR-0118). Runden laufen in einem Prozess über die dort gehaltenen
  Validator-Schlüssel. Das ist ein Devnet-Verbund, kein offenes Validator-Netz.
- **Kein Slashing, kein Reorg, kein View-Wechsel bei Timeout.** Equivocation wird
  erkannt und abgelehnt, aber nicht bestraft; eine hängende Runde blockiert statt zu
  rotieren.
- **Leere Blöcke sind zulässig** (Konsens-Takt ohne Transaktionen). Der Ledger wird
  dabei nicht berührt.
- **Devnet-Schlüssel** werden deterministisch aus `atc-devnet-validator-<id>`
  abgeleitet, damit Neustarts dieselben Validatoren ergeben. Für ein echtes Netz
  müssen Schlüssel extern erzeugt und die Registry autoritativ sein.
- **`BOOT_HASH` über RPC** liefert weiterhin eine 64-Bit-Zahl, obwohl Blockhashes
  256 Bit breit sind. Offener Punkt, unabhängig von diesem Ausbau.

## 7. Nächste Schritte

1. Proposal- und Vote-Nachrichten in den Gossip-Wire aufnehmen (Push), damit mehrere
   Maschinen dieselbe Runde bestätigen.
2. Timeout mit View-Wechsel, damit ein ausgefallener Proposer die Kette nicht anhält.
3. Persistenz der Kette und der Quorum-Bescheinigungen über Neustarts.

# ⚡ Konsensmechanismus — PoH + PoA Hybrid

> **Spezifikation des hybriden Proof of History + Proof of Authority Konsensmodells**

---

## 1. Übersicht

`atc-blockchain` verwendet ein hybrides Konsensverfahren, das zwei komplementäre Konzepte verbindet:

1. **Proof of History (PoH):** Ein kryptographischer Zeitstempel-Generator auf Basis sequentieller Hashketten (SHA-256 / Blake3). PoH ermöglicht es Nodes, die zeitliche Reihenfolge von Transaktionen ohne vorherige Netzwerk-Synchronisation nachzuweisen.
2. **Proof of Authority (PoA) / Proof of Stake (PoS):** Ein rotierendes Validator-Set signiert und finalisiert Blöcke basierend auf den erzeugten PoH-Zeitstempeln.

---

## 2. Netzwerke & Chain-IDs

| Netzwerk | Chain-ID | Zweck |
|----------|----------|-------|
| **Mainnet** | `9000` | Produktionsnetzwerk mit realen Verträgen und Verifikation |
| **Testnet** | `9001` | Staging- und Entwicklungsnetzwerk |

---

## 3. Ablauf der Blockproduktion

```
1. Transaktion trifft ein -> Validierung & Mempool-Einreihung
2. PoH Engine fügt Tx in die laufende Hash-Kette ein -> Erzeugt Tick & Sequence Hash
3. Aktueller Leader (PoA Slot) bündelt Ticks & Txs in einen Block-Kandidaten
4. Leader signiert Block mit seinem Validator-Schlüssel
5. Block wird via P2P (Gossip) an alle Validatoren verteilt
6. Validatoren verifizieren PoH-Kette & Transaktionen -> Finalitäts-Signatur
```

---

## 4. Fork Resolution & Gas Fee Model

- **Fork Resolution (`consensus/fork_resolution.atc`):** Sollten zwei Validatoren zeitgleich Blöcke vorschlagen, gewinnt der Block mit der höheren PoH-Sequenzlänge und gültigen Signaturen des geschätzten Validator-Mehrheitssets.
- **Gas Fee Model (`consensus/gas_fee.atc`):** Jede Transaktion verbrennt eine Grundgebühr in ATCoin und zahlt eine Prioritätsgebühr an den aktiven Validator.

---

*Teil des [A-TownChain Ökosystems](https://github.com/A-TownChain-Okosystems)*

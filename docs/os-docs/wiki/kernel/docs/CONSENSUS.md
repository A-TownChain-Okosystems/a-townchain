# ShivaConsensus — Hybrid PoH + PoS + PoW

## Phase 1: Proof of History (VDF)
- SHA-256 verkettete Hash-Sequenz
- Zeitstempel-Beweis ohne zentralen Clock
- Verifizierbar ohne Vertrauen

## Phase 2: Proof of Stake (Reputation)
- Minimum Stake: 10.000 ATC
- Validator-Auswahl: Weighted Random via SHA-256 Seed
- Slashing bei Fehlverhalten

## Phase 3: Proof of Work (SHA3-ATC)
- Difficulty 4 (Standard)
- Adjustiert alle 2.016 Blöcke
- Block-Reward: halbiert alle 210.000 Blöcke

## Block-Struktur
```python
Block:
  height, hash, prev_hash, timestamp
  poh_hash, pow_nonce, validator
  tx_count, merkle_root
```

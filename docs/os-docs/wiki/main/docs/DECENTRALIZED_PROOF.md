# Dezentraler Nachweis für Nutzer

## Was bedeutet "dezentral" in A-TownChain?

A-TownChain ist dezentral, weil:
1. Kein einzelner Server kontrolliert die Chain
2. Jeder Node hat eine vollständige Kopie aller Blöcke
3. Konsens wird durch Mehrheit der Validator-Nodes erreicht
4. Smart Contracts laufen deterministisch auf allen Nodes

## Nachweis-Mechanismen

### 1. Block-Nachweis (Proof of Block)
Jeder Block enthält:
```
{
  "height":      42,
  "hash":        "ATC8F3A2D...",   ← SHA3-256 dieses Blocks
  "prev_hash":   "ATCB2C5...",     ← Kettenverknüpfung
  "poh_hash":    "ATC9D1...",      ← Proof of History
  "validator":   "ATC1234...",     ← Wer hat validiert?
  "merkle_root": "ATCAAAA...",     ← Alle TXs verifizierbar
  "signature":   "3044...",        ← ECDSA des Validators
}
```
→ **Nutzer kann verifizieren:** Hash des Blocks stimmt mit prev_hash des Folgeblocks überein.

### 2. Transaktions-Nachweis (Proof of Transaction)
```bash
# TX-Nachweis abrufen
GET /api/blockchain/tx/{hash}
→ {
    "hash":      "TX-Hash",
    "block":     42,              ← In welchem Block?
    "merkle_proof": [...],        ← Merkle-Pfad
    "confirmations": 6            ← Anzahl Bestätigungen
  }
```
→ **Nutzer kann verifizieren:** TX-Hash in Merkle-Root des Blocks enthalten.

### 3. Wallet-Nachweis (Proof of Balance)
```bash
GET /api/wallet/balance/{ATC-Adresse}
→ { "balance": "1000.5 ATC", "at_block": 42 }
```
→ **Nutzer kann verifizieren:** Balance = Summe aller eingehenden TXs
   minus Summe aller ausgehenden TXs.

### 4. Smart-Contract-Nachweis
```bash
POST /api/contracts/call
{ "address": "ATC_CONTRACT_...", "method": "total_supply" }
→ { "result": "21000000000000000000000000" }
```
→ **Deterministisch:** Alle Nodes berechnen denselben Wert.

### 5. NFT-Eigentumsnachweis
```bash
POST /api/contracts/call
{ "address": "ATC_CONTRACT_SHIVAMON",
  "method":  "owner_of",
  "args":    {"token_id": 42} }
→ { "result": "ATC8F3A2D1E9B..." }
```
→ **Unveränderlich:** On-Chain gespeichert, nicht manipulierbar.

### 6. Consensus-Nachweis (PoH-Verifikation)
```python
# Jeder kann den PoH verifizieren:
expected = sha3_256(prev_poh_hash + str(counter))
assert block.poh_hash == expected
```

## Dezentralisierungs-Metriken

| Metrik | Aktuell (Testnet) | Ziel Mainnet |
|--------|------------------|--------------|
| Validator-Nodes | 2 | 100+ |
| Full-Nodes | 3 | 1.000+ |
| Geografische Verteilung | 1 Region | Global |
| Nakamoto-Koeffizient | 2 | ≥ 10 |
| Min. Stake für Validator | 10.000 ATC | TBD |

## Wie Nutzer die Dezentralisierung prüfen können

```bash
# 1. Alle verbundenen Nodes anzeigen
curl http://localhost:4000/api/nodes

# 2. Block selbst verifizieren
curl http://localhost:4000/api/blockchain/block/42 | python3 -c "
import json, hashlib, sys
block = json.load(sys.stdin)
# Header re-hashen
header = f"{block['height']}{block['prev_hash']}{block['timestamp']}"
expected = 'ATC' + hashlib.sha3_256(header.encode()).hexdigest()[:32]
print('✅ Hash korrekt' if block['hash'].startswith('ATC') else '❌ Hash falsch')
"

# 3. Node-Monitoring
curl http://localhost:8080/  # Echtzeit-Dashboard
curl http://localhost:9090/metrics  # Prometheus
```

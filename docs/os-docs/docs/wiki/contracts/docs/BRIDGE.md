# ATC Bridge Contract

## Übersicht
Der Bridge-Contract ermöglicht Asset-Transfer zwischen A-TownChain und anderen Blockchains (v2.2.0).

## Aktueller Status (v2.1.0)
- ✅ Implementiert und deployed (BridgeContract)
- ✅ Authority-System (Multi-Sig Relayer)
- ⏳ Aktive Bridge-Verbindungen: in Entwicklung (v2.2.0)

## Geplante Bridges (v2.2.0)
| Chain | Typ | Asset |
|-------|-----|-------|
| Ethereum | Lock/Mint | ATC ↔ WATC (Wrapped ATC) |
| Solana | Lock/Mint | ATC ↔ SATC |

## Bridge-Ablauf (Lock/Mint)
```
A-TownChain → Bridge Contract → Lock ATC
                    ↓
             (Relayer bestätigt)
                    ↓
Ethereum    → Bridge Contract → Mint WATC
```

## API (v2.1.0)
```python
bridge.add_authority(caller, authority_addr)   # Owner only
bridge.initiate_transfer(caller, to_chain, to_addr, amount)
bridge.complete_transfer(caller, tx_id, proof)
bridge.get_pending_transfers()
bridge.name()  # → "A-TownChain Cross-Chain Bridge"
```

## Security
- Multi-Sig: min. 2/3 Authorities müssen bestätigen
- Timelock: 24h Wartezeit für große Transfers (> 100.000 ATC)
- Rate-Limit: max 1.000.000 ATC/Tag

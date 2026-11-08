# 📥 Mempool & Transaktionsverarbeitung

> **Spezifikation des Mempool Subsystems und der Ingress Pipeline**

---

## 1. Transaktions-Lebenszyklus

```
[ Ingress: Gateway Port 4000 / P2P ]
               |
               v
    [ Mempool Validation ]
    ├─ Signaturprüfung (ECDSA)
    ├─ Balance-Check
    └─ Nonce-Reihenfolge
               |
               v
     [ Priority Queue ] (Sorted by Gas Price)
               |
               v
    [ Block Production / PoH Slot ]
```

---

## 2. Mempool Eigenschaften

- **Nonce Enforcement:** Transaktionen eines Senders müssen strikt aufsteigende Nonces aufweisen, um Replay-Attacken zu verhindern.
- **Gas Fee Ordering:** Der Mempool sortiert ausstehende Transaktionen nach der gebotenen Prioritätsgebühr (`gas_fee.atc`).
- **Rate Limiting & Memory Cap:** Bei hoher Auslastung verwirft der Mempool Transaktionen mit zu niedriger Gebühr, um den Arbeitsspeicher des Nodes zu schützen.

---

*Teil des [A-TownChain Ökosystems](https://github.com/A-TownChain-Okosystems)*

# 🛡️ Validatoren & Block-Produktion

> **Handbuch für Validator Nodes, Staking und Slashing**

---

## 1. Validator Rolle

Validatoren sind spezialisierte Nodes, die:
- Die PoH-Sequenz prüfen und verifizieren.
- Neue Blöcke vorschlagen, wenn sie als Slot Leader an der Reihe sind.
- Blöcke anderer Validatoren signieren und den Netzwerk-Konsens sichern.

---

## 2. Anforderungen an Validator Nodes

- **Software:** `atc-blockchain` Validator Daemon (`nodes/node.atc`)
- **Netzwerk:** Öffentliche IP mit erreichbarem Port 9000 (Mainnet) bzw. 9001 (Testnet)
- **Identität:** Valider ECDSA Schlüssel und registrierte Validator-Adresse im Governance-Contract

---

## 3. Slashing & Sicherheit

Um Fehlverhalten vorzubeugen, greifen automatisierte Slashing-Regeln (`consensus/pos.atc`):

| Vergehen | Strafe | Aktion |
|----------|--------|--------|
| **Double Signing** | 100% Slash | Permanenter Ausschluss aus dem Validator-Set |
| **Dauerhaftes Offline-Sein** | Downtime Penalty | Temporärer Ausschluss (Jailing) & Teil-Slash |
| **Ungültige PoH Kette** | Block Rejection | Annullierung des vorgeschlagenen Blocks |

---

*Teil des [A-TownChain Ökosystems](https://github.com/A-TownChain-Okosystems)*

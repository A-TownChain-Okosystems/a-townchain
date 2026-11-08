# 📄 Issue #1 — Smart Contract Implementation (ATC Token Standards)

> **Labels:** enhancement · blockchain
> **Priorität:** 🔴 High · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #1](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/1)

---

## Ziel

Implementierung der nativen ATC Smart Contracts für alle definierten Token-Standards des A-TownChain Ökosystems (ATC-001 bis ATC-9900) — zunächst als Python-Contracts, später als Solidity On-Chain Contracts (→ Issue #12).

---

## Hintergrund

Die aktuelle `blockchain/smart_contracts.py` ist ein Placeholder. Die vollständigen ATC Token Standards aus der ATC-Referenzmatrix müssen als vollwertige, testbare Contracts implementiert werden.

---

## Technische Spezifikation

### Verzeichnisstruktur (Ziel)

```
blockchain/contracts/
├── atc001/
│   └── genesis_token.py          # ATC-001: Genesis Block Token
├── atc8300/
│   └── atc_token.py              # ATC-8300: Fungible Token (ERC20)
├── atc9000/
│   └── shivamon_contract.py      # ATC-9000: NFT ✅ bereits implementiert
├── atc9900/
│   └── governance_contract.py    # ATC-9900: DAO/Governance (→ Issue #9)
└── base/
    └── base_contract.py          # Gemeinsame Basisklasse
```

### ATC-001 Genesis Token

```python
class GenesisToken:
    """
    ATC-001 — Der erste Token auf der A-TownChain.
    Einmalig geprägt beim Genesis Block.
    Nicht transferierbar — symbolischer Ursprungs-Token.
    """
    TOKEN_ID    = "ATC-001-GENESIS"
    TOTAL_SUPPLY = 1          # Genau 1 Genesis Token
    OWNER       = "Michael Wroblewski"
    MINTED_AT   = "2025-01-01T00:00:00Z"
    TRANSFERABLE = False
```

### ATC-8300 Fungible Token (vollständig)

Ergänzungen zur bestehenden `blockchain/atcoin/atcoin.py`:

```python
# Noch fehlende Methoden:
def burn(self, address: str, amount: Decimal) -> dict:
    """Token vernichten — reduziert total_supply permanent."""

def pause(self) -> bool:
    """Alle Transfers einfrieren (nur Contract-Owner)."""

def unpause(self) -> bool:
    """Transfers wieder freigeben."""

def snapshot(self) -> dict:
    """Snapshot aller Balances zu einem bestimmten Zeitpunkt."""

def get_tx_history(self, address: str, limit: int = 50) -> list:
    """TX-Historie einer Adresse abrufen."""
```

### Basisklasse für alle Contracts

```python
# blockchain/contracts/base/base_contract.py
class BaseContract:
    def __init__(self, owner: str):
        self.owner      = owner
        self.paused     = False
        self.deployed_at = int(time.time())
        self.version    = "1.0.0"

    def only_owner(self, caller: str) -> bool:
        if caller != self.owner:
            raise PermissionError("Only contract owner")
        return True

    def not_paused(self) -> bool:
        if self.paused:
            raise RuntimeError("Contract is paused")
        return True
```

---

## Implementierungs-Phasen

### Phase 1 — Grundstruktur ✅ teilweise
- [x] `ATCoin` Basis-Contract (ERC20-kompatibel, ATC-8300)
- [x] Transfer, Approve, Allowance
- [ ] `burn()`, `pause()`, `snapshot()`
- [ ] `BaseContract` Basisklasse

### Phase 2 — ATC Standards
- [ ] ATC-001 Genesis Token Contract
- [ ] ATC-8300: fehlende Methoden ergänzen
- [ ] ATC-9900 Governance Contract (→ koordiniert mit Issue #9)

### Phase 3 — Tests
- [ ] `tests/test_atc8300.py` — Unit Tests (Coverage > 90%)
- [ ] Test: Transfer, Approve, Allowance, Burn, Pause
- [ ] Test: Halving-Formel bei verschiedenen Block-Heights
- [ ] Test: Max-Supply Limit

### Phase 4 — Integration
- [ ] Contract ABI-Schema exportieren (`config/abis/`)
- [ ] Backend-Routes vollständig verdrahten
- [ ] Gateway Route `/api/blockchain/contract/:name`

---

## Akzeptanzkriterien

- [ ] Alle ATC Standards implementiert und testbar
- [ ] Unit Test Coverage > 90%
- [ ] API-Endpoints funktional via Gateway
- [ ] Keine Breaking Changes zu bestehenden Routes
- [ ] Dokumentation in `docs/contracts/`

---

## Referenzen

- `blockchain/atcoin/atcoin.py` — bestehende ATC-8300 Basis
- `blockchain/contracts/shivamon/shivamon_contract.py` — Referenz-Implementierung
- ATC Token Referenzmatrix (ATC-001 bis ATC-9900)

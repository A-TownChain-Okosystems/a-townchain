# 🪙 ATC Token Standards — Vollständige Dokumentation
**Stand:** 09.06.2026 | **Version:** v2.1.0
**Dateien:** `blockchain/contracts/atc8300/` + `blockchain/contracts/shivamon/`

---

## ATC-8300 — Fungible Token Standard

**Datei:** `blockchain/contracts/atc8300/atc8300_token.py` (125 Zeilen)

ATC-8300 ist der proprietäre Standard für fungible Token auf A-TownChain — analog zu ERC-20, aber vollständig unabhängig.

### Interface

```python
class ATC8300Token:
    """
    ATC-8300 Fungible Token Standard.
    Eigenschaften:
      - 18 Dezimalstellen (1 ATC = 10^18 ATC-Wei)
      - Mint/Burn durch Owner
      - Transfer + Allowance (ERC-20 Pattern, proprietäre Impl.)
      - Event-System für alle State-Änderungen
    """

    def name(self) -> str:           ...  # Token-Name
    def symbol(self) -> str:         ...  # Token-Symbol (3-5 Zeichen)
    def decimals(self) -> int:       ...  # 18
    def total_supply(self) -> int:   ...  # Gesamtmenge in Wei

    def balance_of(self, addr: str) -> int:
        """Balance einer Adresse in Wei."""

    def transfer(self, to: str, amount: int) -> bool:
        """Überweist amount Wei von caller zu to."""
        # require(balance >= amount)
        # require(is_valid_address(to))
        # emit Transfer(caller, to, amount)

    def approve(self, spender: str, amount: int) -> bool:
        """Erlaubt spender amount Wei vom caller auszugeben."""
        # emit Approval(caller, spender, amount)

    def allowance(self, owner: str, spender: str) -> int:
        """Aktuell genehmigte Menge."""

    def transfer_from(self, from_: str, to: str, amount: int) -> bool:
        """Überweist im Auftrag (nach approve())."""

    def mint(self, to: str, amount: int) -> bool:
        """Neue Token erzeugen (nur Owner)."""
        # require(caller == self.owner)
        # emit Mint(to, amount)

    def burn(self, amount: int) -> bool:
        """Token vernichten."""
        # emit Burn(caller, amount)
```

### Events
```
Transfer(from: Address, to: Address, amount: u128)
Approval(owner: Address, spender: Address, amount: u128)
Mint(to: Address, amount: u128)
Burn(from: Address, amount: u128)
```

### ATC Coin (Nativer Token)

**Datei:** `blockchain/atcoin/atcoin.py`

```python
class ATCoin(ATC8300Token):
    """
    Nativer ATC Token — implementiert ATC-8300 Standard.

    Eigenschaften:
      Name:          A-TownCoin
      Symbol:        ATC
      Decimals:      18
      Total Supply:  21.000.000 ATC (wie Bitcoin, in Wei)
      Chain ID:      9000
    """
    NAME         = "A-TownCoin"
    SYMBOL       = "ATC"
    DECIMALS     = 18
    MAX_SUPPLY   = 21_000_000 * (10 ** 18)  # 21 Mio ATC in Wei
    CHAIN_ID     = 9000
```

---

## ATC-9000 — NFT Standard (Shivamon)

**Datei:** `blockchain/contracts/shivamon/shivamon_contract.py` (269 Zeilen)

ATC-9000 ist der proprietäre Standard für Non-Fungible Token — speziell für Shivamon (Gaming NFTs).

### Interface

```python
class ShivamonContract:
    """
    ATC-9000 NFT Standard — Shivamon Gaming NFTs.
    Jedes Shivamon ist einzigartig mit Eigenschaften:
      - Element (Feuer, Wasser, Erde, Luft, Blitz, Eis)
      - Level (1-100)
      - Kampfwerte (HP, ATK, DEF, SPD)
      - Seltenheit (Common, Rare, Epic, Legendary)
      - DNA (einzigartiger genetischer Hash)
    """

    def mint(self, owner: str, metadata: dict) -> int:
        """
        Neues Shivamon erzeugen.
        Returns: token_id (eindeutige u64)
        metadata: {
          "name": "Pyravox",
          "element": "fire",
          "level": 1,
          "hp": 100, "atk": 80, "def": 60, "spd": 70,
          "rarity": "rare",
          "dna": sha256(owner + block_timestamp + random)
        }
        """

    def transfer(self, from_: str, to: str, token_id: int) -> bool:
        """NFT übertragen."""

    def owner_of(self, token_id: int) -> str:
        """Eigentümer eines NFT."""

    def token_uri(self, token_id: int) -> str:
        """Metadata-URI (JSON)."""

    def approve(self, to: str, token_id: int) -> bool:
        """Transfer-Genehmigung."""

    def set_approval_for_all(self, operator: str, approved: bool):
        """Alle NFTs genehmigen."""

    def tokens_of(self, owner: str) -> List[int]:
        """Alle Token-IDs eines Eigentümers."""

    def level_up(self, token_id: int) -> bool:
        """Shivamon leveln (nur Eigentümer)."""

    def evolve(self, token_id: int) -> bool:
        """Shivamon entwickeln (Level 20/40/60 erforderlich)."""
```

### Shivamon-Elemente & Typen

| Element | Stärke gegen | Schwäche gegen | Farbe |
|---------|------------|----------------|-------|
| 🔥 Feuer | Eis, Pflanze | Wasser, Erde | #FF4500 |
| 💧 Wasser | Feuer, Erde | Blitz, Pflanze | #1E90FF |
| ⚡ Blitz | Wasser, Metall | Erde, Pflanze | #FFD700 |
| 🌿 Pflanze | Wasser, Erde | Feuer, Eis | #32CD32 |
| 🌙 Dunkel | Geist, Psy | Licht, Kampf | #800080 |
| ☀️ Licht | Dunkel, Geist | — | #FFFACD |

### Seltenheitsstufen

| Seltenheit | Drop-Rate | Basis-Stats Bonus | Farbe |
|-----------|-----------|------------------|-------|
| Common | 60% | +0% | ⬜ Grau |
| Uncommon | 25% | +10% | 🟩 Grün |
| Rare | 10% | +25% | 🟦 Blau |
| Epic | 4% | +50% | 🟪 Lila |
| Legendary | 1% | +100% | 🟨 Gold |

### Events
```
ShivamonMinted(owner: Address, token_id: u64, element: string, rarity: string)
ShivamonTransferred(from: Address, to: Address, token_id: u64)
ShivamonLevelUp(token_id: u64, new_level: u64)
ShivamonEvolved(token_id: u64, new_form: string)
```

---

## ATC-9900 — Governance DAO Standard (In Development)

**Issue:** #9 | **Version:** v2.2.0

```python
class GovernanceContract:
    """
    ATC-9900 DAO Governance.
    Proposals → Voting → Execution
    """
    MIN_PROPOSAL_STAKE = 10_000  # ATC
    VOTING_PERIOD      = 7 * 24 * 3600  # 7 Tage
    QUORUM             = 0.10   # 10% der Staked-Token
    THRESHOLD          = 0.51   # 51% für Annahme

    def propose(self, description: str, actions: list) -> int: ...
    def vote(self, proposal_id: int, support: bool, weight: int): ...
    def execute(self, proposal_id: int): ...
    def cancel(self, proposal_id: int): ...
```

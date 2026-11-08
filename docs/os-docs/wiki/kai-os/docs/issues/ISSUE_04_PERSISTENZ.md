# 📄 Issue #4 — NFT Persistenz (SQLite)

> **Labels:** enhancement · backend · priority:high
> **Priorität:** 🔴 High · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #4](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/4)

---

## Ziel

Shivamon NFTs, ATC-Wallets und Transaktionen persistent in einer **SQLite-Datenbank** speichern. Aktuell gehen alle Daten bei Server-Neustart verloren — das ist ein Production-Blocker.

---

## Datenbankschema

```sql
-- Wallets
CREATE TABLE wallets (
    address     TEXT PRIMARY KEY,          -- ATC... (35 Zeichen)
    public_key  TEXT NOT NULL,
    label       TEXT DEFAULT 'Main Wallet',
    balance     REAL DEFAULT 0.0,
    created_at  INTEGER NOT NULL
    -- KEIN private_key! Niemals in DB speichern.
);

-- Transaktionen
CREATE TABLE transactions (
    tx_id       TEXT PRIMARY KEY,          -- SHA-256 Hash
    tx_type     TEXT NOT NULL,             -- transfer | mint | burn
    from_addr   TEXT,
    to_addr     TEXT NOT NULL,
    amount      REAL NOT NULL,
    fee         REAL DEFAULT 0.001,
    timestamp   INTEGER NOT NULL,
    block_height INTEGER
);

-- Shivamon NFTs
CREATE TABLE shivamon (
    token_id    TEXT PRIMARY KEY,          -- SHV-...
    name        TEXT NOT NULL,
    element     TEXT NOT NULL,
    rarity      TEXT NOT NULL,
    owner       TEXT NOT NULL,
    generation  INTEGER DEFAULT 1,
    level       INTEGER DEFAULT 1,
    xp          INTEGER DEFAULT 0,
    wins        INTEGER DEFAULT 0,
    losses      INTEGER DEFAULT 0,
    hp          INTEGER, attack INTEGER,
    defense     INTEGER, speed INTEGER, special INTEGER,
    dna_hash    TEXT NOT NULL,
    moves       TEXT,                      -- JSON Array
    minted_at   INTEGER NOT NULL,
    FOREIGN KEY (owner) REFERENCES wallets(address)
);

-- Blöcke
CREATE TABLE blocks (
    height      INTEGER PRIMARY KEY,
    hash        TEXT NOT NULL UNIQUE,
    prev_hash   TEXT NOT NULL,
    miner       TEXT,
    validator   TEXT,
    reward      REAL,
    difficulty  INTEGER,
    nonce       INTEGER,
    poh_hash    TEXT,
    timestamp   INTEGER NOT NULL,
    tx_count    INTEGER DEFAULT 0
);
```

---

## Implementierung

### Repository Pattern

```python
# backend/db/repository.py
import sqlite3, json
from pathlib import Path

DB_PATH = Path("data/atcoin.db")

class WalletRepository:
    def save(self, address, public_key, label, balance): ...
    def find(self, address) -> dict: ...
    def update_balance(self, address, new_balance): ...

class ShivamonRepository:
    def save(self, nft: ShivamonNFT): ...
    def find(self, token_id) -> ShivamonNFT: ...
    def find_by_owner(self, owner) -> list: ...
    def update(self, nft: ShivamonNFT): ...

class BlockRepository:
    def save(self, block: dict): ...
    def find_latest(self, n=10) -> list: ...
    def get_height(self) -> int: ...
```

### Migration (In-Memory → SQLite)

```python
# backend/db/migrate.py
def migrate_from_memory(wallet_instance, contract_instance, consensus_instance):
    """Einmalige Migration beim ersten Start."""
    repo_w = WalletRepository()
    repo_s = ShivamonRepository()
    repo_b = BlockRepository()

    for address, data in wallet_instance.accounts.items():
        repo_w.save(address, data["public_key"], data["label"],
                    float(wallet_instance.atcoin.balance_of(address)))

    for token_id, nft in contract_instance.tokens.items():
        repo_s.save(nft)

    for block in consensus_instance.blocks:
        repo_b.save(block)
```

---

## Aufgaben

- [ ] `data/` Verzeichnis anlegen + `.gitignore` (DB-Dateien nicht committen)
- [ ] `backend/db/__init__.py` + `backend/db/schema.sql`
- [ ] `backend/db/repository.py` — WalletRepository, ShivamonRepository, BlockRepository, TxRepository
- [ ] `backend/db/migrate.py` — Migration-Script
- [ ] `ATCWallet` — Persistenz-Layer einbinden
- [ ] `ShivamonContract` — Persistenz-Layer einbinden
- [ ] `HybridConsensus` — Blocks persistent speichern
- [ ] Tägliches Backup-Script (`build/backup.py`)
- [ ] Tests: `tests/test_persistence.py`

---

## Sicherheitshinweise

> ⚠️ **Private Keys werden NIEMALS in der Datenbank gespeichert.**
> Nur `address` (public) und `public_key` werden persistiert.
> Der Private Key existiert nur im Moment der Wallet-Erstellung im RAM.

---

## Akzeptanzkriterien

- [ ] Server-Neustart behält alle Wallets, NFTs und Blöcke
- [ ] Migration von In-Memory funktioniert fehlerfrei
- [ ] Keine Private Keys in der DB
- [ ] DB-Datei ist in `.gitignore`

-- A-TownChain OS — SQLite Schema
-- Issue #4: NFT Persistenz

CREATE TABLE IF NOT EXISTS wallets (
    address     TEXT PRIMARY KEY,
    public_key  TEXT NOT NULL,
    label       TEXT DEFAULT 'Main Wallet',
    balance     REAL DEFAULT 0.0,
    created_at  INTEGER NOT NULL
    -- KEIN private_key — niemals in DB speichern!
);

CREATE TABLE IF NOT EXISTS transactions (
    tx_id        TEXT PRIMARY KEY,
    tx_type      TEXT NOT NULL CHECK(tx_type IN ('transfer','mint','burn','reward')),
    from_addr    TEXT,
    to_addr      TEXT NOT NULL,
    amount       REAL NOT NULL,
    fee          REAL DEFAULT 0.001,
    nonce        INTEGER,
    signature    TEXT,
    timestamp    INTEGER NOT NULL,
    block_height INTEGER
);

CREATE TABLE IF NOT EXISTS shivamon (
    token_id    TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    element     TEXT NOT NULL,
    rarity      TEXT NOT NULL,
    owner       TEXT NOT NULL,
    generation  INTEGER DEFAULT 1,
    level       INTEGER DEFAULT 1,
    xp          INTEGER DEFAULT 0,
    wins        INTEGER DEFAULT 0,
    losses      INTEGER DEFAULT 0,
    hp          INTEGER, attack  INTEGER,
    defense     INTEGER, speed   INTEGER, special INTEGER,
    dna_hash    TEXT NOT NULL,
    moves       TEXT DEFAULT '[]',
    minted_at   INTEGER NOT NULL,
    FOREIGN KEY (owner) REFERENCES wallets(address)
);

CREATE TABLE IF NOT EXISTS blocks (
    height      INTEGER PRIMARY KEY,
    hash        TEXT NOT NULL UNIQUE,
    prev_hash   TEXT NOT NULL,
    miner       TEXT,
    validator   TEXT,
    reward      REAL DEFAULT 0.0,
    difficulty  INTEGER DEFAULT 3,
    nonce       INTEGER DEFAULT 0,
    poh_hash    TEXT,
    timestamp   INTEGER NOT NULL,
    tx_count    INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS mempool (
    tx_id       TEXT PRIMARY KEY,
    tx_data     TEXT NOT NULL,
    received_at INTEGER NOT NULL
);

-- Indices fuer Performance
CREATE INDEX IF NOT EXISTS idx_tx_from      ON transactions(from_addr);
CREATE INDEX IF NOT EXISTS idx_tx_to        ON transactions(to_addr);
CREATE INDEX IF NOT EXISTS idx_shiv_owner   ON shivamon(owner);
CREATE INDEX IF NOT EXISTS idx_blocks_hash  ON blocks(hash);

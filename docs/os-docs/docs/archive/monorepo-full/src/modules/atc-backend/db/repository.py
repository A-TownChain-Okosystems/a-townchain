# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Repository Pattern — Issue #4
CRUD fuer Wallets, Shivamon, Transactions, Blocks.
"""
import json, time, sqlite3
from typing import Optional
from backend.db.connection import get_connection


class WalletRepository:

    def save(self, address: str, public_key: str,
             label: str = "Main Wallet", balance: float = 0.0) -> bool:
        db = get_connection()
        try:
            db.execute(
                "INSERT OR REPLACE INTO wallets VALUES (?,?,?,?,?)",
                (address, public_key, label, balance, int(time.time()))
            )
            db.commit()
            return True
        except sqlite3.Error:
            return False

    def find(self, address: str) -> Optional[dict]:
        db  = get_connection()
        row = db.execute(
            "SELECT * FROM wallets WHERE address=?", (address,)
        ).fetchone()
        return dict(row) if row else None

    def update_balance(self, address: str, new_balance: float) -> bool:
        db = get_connection()
        db.execute("UPDATE wallets SET balance=? WHERE address=?",
                   (new_balance, address))
        db.commit()
        return db.total_changes > 0

    def list_all(self) -> list:
        db = get_connection()
        return [dict(r) for r in db.execute("SELECT * FROM wallets").fetchall()]

    def count(self) -> int:
        db = get_connection()
        return db.execute("SELECT COUNT(*) FROM wallets").fetchone()[0]


class ShivamonRepository:

    def save(self, nft: dict) -> bool:
        db = get_connection()
        try:
            db.execute("""
                INSERT OR REPLACE INTO shivamon
                (token_id,name,element,rarity,owner,generation,level,xp,
                 wins,losses,hp,attack,defense,speed,special,
                 dna_hash,moves,minted_at)
                VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            """, (
                nft["token_id"], nft["name"], nft["element"], nft["rarity"],
                nft["owner"],    nft.get("generation",1), nft.get("level",1),
                nft.get("xp",0), nft.get("wins",0), nft.get("losses",0),
                nft.get("hp"), nft.get("attack"), nft.get("defense"),
                nft.get("speed"), nft.get("special"),
                nft["dna_hash"], json.dumps(nft.get("moves",[])),
                nft.get("minted_at", int(time.time()))
            ))
            db.commit()
            return True
        except sqlite3.Error as e:
            print(f"ShivamonRepo.save error: {e}")
            return False

    def find(self, token_id: str) -> Optional[dict]:
        db  = get_connection()
        row = db.execute(
            "SELECT * FROM shivamon WHERE token_id=?", (token_id,)
        ).fetchone()
        if not row: return None
        d = dict(row)
        d["moves"] = json.loads(d.get("moves","[]"))
        return d

    def find_by_owner(self, owner: str) -> list:
        db = get_connection()
        rows = db.execute(
            "SELECT * FROM shivamon WHERE owner=?", (owner,)
        ).fetchall()
        result = []
        for r in rows:
            d = dict(r)
            d["moves"] = json.loads(d.get("moves","[]"))
            result.append(d)
        return result

    def update(self, token_id: str, fields: dict) -> bool:
        db      = get_connection()
        allowed = {"level","xp","wins","losses","hp","attack",
                   "defense","speed","special","owner","moves"}
        updates = {k:v for k,v in fields.items() if k in allowed}
        if not updates: return False
        set_clause = ", ".join(f"{k}=?" for k in updates)
        db.execute(
            f"UPDATE shivamon SET {set_clause} WHERE token_id=?",
            (*updates.values(), token_id)
        )
        db.commit()
        return db.total_changes > 0

    def count(self) -> int:
        return get_connection().execute(
            "SELECT COUNT(*) FROM shivamon").fetchone()[0]


class TransactionRepository:

    def save(self, tx: dict) -> bool:
        db = get_connection()
        try:
            db.execute("""
                INSERT OR IGNORE INTO transactions
                (tx_id,tx_type,from_addr,to_addr,amount,fee,
                 nonce,signature,timestamp,block_height)
                VALUES (?,?,?,?,?,?,?,?,?,?)
            """, (
                tx["tx_id"], tx.get("tx_type","transfer"),
                tx.get("from"), tx["to"],
                tx["amount"],   tx.get("fee", 0.001),
                tx.get("nonce"), tx.get("signature"),
                tx.get("timestamp", int(time.time())),
                tx.get("block_height")
            ))
            db.commit()
            return True
        except sqlite3.Error:
            return False

    def find(self, tx_id: str) -> Optional[dict]:
        db  = get_connection()
        row = db.execute(
            "SELECT * FROM transactions WHERE tx_id=?", (tx_id,)
        ).fetchone()
        return dict(row) if row else None

    def find_by_address(self, address: str, limit: int = 50) -> list:
        db = get_connection()
        rows = db.execute("""
            SELECT * FROM transactions
            WHERE from_addr=? OR to_addr=?
            ORDER BY timestamp DESC LIMIT ?
        """, (address, address, limit)).fetchall()
        return [dict(r) for r in rows]

    def count(self) -> int:
        return get_connection().execute(
            "SELECT COUNT(*) FROM transactions").fetchone()[0]


class BlockRepository:

    def save(self, block: dict) -> bool:
        db = get_connection()
        try:
            db.execute("""
                INSERT OR IGNORE INTO blocks
                (height,hash,prev_hash,miner,validator,reward,
                 difficulty,nonce,poh_hash,timestamp,tx_count)
                VALUES (?,?,?,?,?,?,?,?,?,?,?)
            """, (
                block["height"], block["hash"], block["prev_hash"],
                block.get("miner"), block.get("validator"),
                block.get("reward",0.0), block.get("difficulty",3),
                block.get("nonce",0), block.get("poh_hash"),
                block.get("timestamp", int(time.time())),
                len(block.get("transactions",[]))
            ))
            db.commit()
            return True
        except sqlite3.Error:
            return False

    def find_latest(self, n: int = 10) -> list:
        db = get_connection()
        return [dict(r) for r in db.execute(
            "SELECT * FROM blocks ORDER BY height DESC LIMIT ?", (n,)
        ).fetchall()]

    def get_height(self) -> int:
        row = get_connection().execute(
            "SELECT MAX(height) FROM blocks").fetchone()
        return row[0] or 0

    def count(self) -> int:
        return get_connection().execute(
            "SELECT COUNT(*) FROM blocks").fetchone()[0]

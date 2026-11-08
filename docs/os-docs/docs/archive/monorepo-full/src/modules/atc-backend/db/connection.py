# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Database Connection Manager — Issue #4
Singleton SQLite-Verbindung mit automatischer Schema-Initialisierung.
"""
import sqlite3, os, threading
from pathlib import Path

DB_PATH  = Path(os.getenv("ATC_DB_PATH", "data/atcoin.db"))
_lock    = threading.Lock()
_conn    = None


def get_connection() -> sqlite3.Connection:
    """Gibt die SQLite-Verbindung zurück (Singleton, thread-safe)."""
    global _conn
    if _conn is None:
        with _lock:
            if _conn is None:
                DB_PATH.parent.mkdir(parents=True, exist_ok=True)
                _conn = sqlite3.connect(str(DB_PATH), check_same_thread=False)
                _conn.row_factory = sqlite3.Row
                _conn.execute("PRAGMA journal_mode=WAL")
                _conn.execute("PRAGMA foreign_keys=ON")
                _init_schema(_conn)
    return _conn


def _init_schema(conn: sqlite3.Connection):
    schema_path = Path(__file__).parent / "schema.sql"
    if schema_path.exists():
        conn.executescript(schema_path.read_text())
        conn.commit()


def close():
    global _conn
    if _conn:
        _conn.close()
        _conn = None

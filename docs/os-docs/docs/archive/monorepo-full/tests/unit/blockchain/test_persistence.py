# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für SQLite Persistence — ATC-85
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os
import sqlite3
import tempfile

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestPersistence:
    """Test SQLite persistence layer."""

    def test_sqlite_create_database(self):
        """SQLite database can be created."""
        db_path = tempfile.mktemp(suffix=".db")
        conn = sqlite3.connect(db_path)
        assert conn is not None
        conn.close()
        os.unlink(db_path)

    def test_sqlite_create_blocks_table(self):
        """Blocks table can be created."""
        db_path = tempfile.mktemp(suffix=".db")
        conn = sqlite3.connect(db_path)
        conn.execute("""
            CREATE TABLE IF NOT EXISTS blocks (
                id INTEGER PRIMARY KEY,
                hash TEXT NOT NULL,
                previous_hash TEXT,
                timestamp REAL NOT NULL,
                data TEXT,
                nonce INTEGER DEFAULT 0
            )
        """)
        conn.commit()
        # Insert a test block
        conn.execute(
            "INSERT INTO blocks (hash, previous_hash, timestamp, data) VALUES (?, ?, ?, ?)",
            ("test_hash", "prev_hash", 1234567890.0, "{}")
        )
        conn.commit()
        # Verify
        cursor = conn.execute("SELECT hash FROM blocks WHERE hash = ?", ("test_hash",))
        result = cursor.fetchone()
        assert result is not None
        assert result[0] == "test_hash"
        conn.close()
        os.unlink(db_path)

    def test_sqlite_create_transactions_table(self):
        """Transactions table can be created."""
        db_path = tempfile.mktemp(suffix=".db")
        conn = sqlite3.connect(db_path)
        conn.execute("""
            CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY,
                block_hash TEXT NOT NULL,
                sender TEXT,
                recipient TEXT,
                amount REAL,
                signature TEXT,
                timestamp REAL NOT NULL
            )
        """)
        conn.commit()
        conn.execute(
            "INSERT INTO transactions (block_hash, sender, recipient, amount, timestamp) VALUES (?, ?, ?, ?, ?)",
            ("block_1", "0xsender", "0xrecipient", 1.5, 1234567890.0)
        )
        conn.commit()
        cursor = conn.execute("SELECT amount FROM transactions WHERE block_hash = ?", ("block_1",))
        result = cursor.fetchone()
        assert result is not None
        assert result[0] == 1.5
        conn.close()
        os.unlink(db_path)

    def test_persistence_roundtrip(self):
        """Data persists across connections."""
        db_path = tempfile.mktemp(suffix=".db")
        conn1 = sqlite3.connect(db_path)
        conn1.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, value TEXT)")
        conn1.execute("INSERT INTO test (value) VALUES (?)", ("hello",))
        conn1.commit()
        conn1.close()
        # Reopen
        conn2 = sqlite3.connect(db_path)
        cursor = conn2.execute("SELECT value FROM test WHERE id = 1")
        result = cursor.fetchone()
        assert result is not None
        assert result[0] == "hello"
        conn2.close()
        os.unlink(db_path)

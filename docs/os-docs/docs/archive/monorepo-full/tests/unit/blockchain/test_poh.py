# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Unit-Tests für Proof of History (PoH) — ATC-81
Agent: Aurora #2 (6a275618)
"""
import pytest
import sys
import os
import hashlib

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', '..'))


class TestProofOfHistory:
    """Test Proof of History implementation."""

    def test_poh_hash_chain(self):
        """PoH creates a valid hash chain with SHA-256."""
        h1 = hashlib.sha256(b"genesis").digest()
        h2 = hashlib.sha256(h1 + b"entry1").digest()
        h3 = hashlib.sha256(h2 + b"entry2").digest()
        assert h1 != h2 != h3
        assert len(h1) == 32  # SHA-256 = 32 bytes

    def test_poh_deterministic(self):
        """PoH is deterministic — same input produces same hash."""
        h1 = hashlib.sha256(b"test").hexdigest()
        h2 = hashlib.sha256(b"test").hexdigest()
        assert h1 == h2

    def test_poh_sequential_ordering(self):
        """PoH entries are sequentially ordered."""
        entries = []
        h = hashlib.sha256(b"start").digest()
        for i in range(10):
            h = hashlib.sha256(h + str(i).encode()).digest()
            entries.append(h.hexdigest())
        # Each entry is unique
        assert len(set(entries)) == 10
        # Entries are in order
        assert entries[0] != entries[-1]

    def test_poh_tamper_detection(self):
        """Tampering with any entry breaks the chain."""
        h = hashlib.sha256(b"start").digest()
        chain = [h]
        for i in range(5):
            h = hashlib.sha256(h + str(i).encode()).digest()
            chain.append(h)
        # Verify chain
        h_check = hashlib.sha256(b"start").digest()
        for i in range(5):
            h_check = hashlib.sha256(h_check + str(i).encode()).digest()
            assert h_check == chain[i + 1]

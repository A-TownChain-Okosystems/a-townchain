# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""ATCLang Standard Library Tests — ATC-94
Tests für alle 6 Module: crypto, collections, io, math, encoding, primitives
"""
import unittest
import sys
import os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from atclang.stdlib.crypto import ATCCrypto
from atclang.stdlib.collections import ATCCollections
from atclang.stdlib.io import ATCIO
from atclang.stdlib.math import ATCMath
from atclang.stdlib.encoding import ATCEncoding
from atclang.stdlib.primitives import ATCPrimitives, ATCAddress, ATCHash, ATCTransaction, ATCBlockHeader


class TestATCCrypto(unittest.TestCase):
    """ATC::Crypto — 8+ Tests, 95% Coverage Target"""

    def test_sha256_string(self):
        h = ATCCrypto.sha256("hello")
        self.assertEqual(len(h), 64)
        self.assertEqual(h, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")

    def test_sha256_bytes(self):
        h = ATCCrypto.sha256_bytes(b"hello")
        self.assertEqual(len(h), 32)

    def test_double_sha256(self):
        h = ATCCrypto.double_sha256("hello")
        self.assertEqual(len(h), 64)

    def test_hmac_sha256(self):
        h = ATCCrypto.hmac_sha256("key", "msg")
        self.assertEqual(len(h), 64)

    def test_base58_roundtrip(self):
        data = b"\x00\x01\x02\x03hello"
        encoded = ATCCrypto.base58_encode(data)
        decoded = ATCCrypto.base58_decode(encoded)
        self.assertEqual(data, decoded)

    def test_base64_roundtrip(self):
        data = b"hello world"
        encoded = ATCCrypto.base64_encode(data)
        decoded = ATCCrypto.base64_decode(encoded)
        self.assertEqual(data, decoded)

    def test_hex_roundtrip(self):
        data = b"hello"
        encoded = ATCCrypto.hex_encode(data)
        decoded = ATCCrypto.hex_decode(encoded)
        self.assertEqual(data, decoded)

    def test_sign_verify(self):
        priv, pub = ATCCrypto.generate_keypair()
        msg = "test message"
        sig = ATCCrypto.sign(msg, priv)
        self.assertTrue(ATCCrypto.verify(msg, sig, priv))

    def test_verify_wrong_sig(self):
        priv, pub = ATCCrypto.generate_keypair()
        sig = ATCCrypto.sign("message", priv)
        self.assertFalse(ATCCrypto.verify("different", sig, priv))

    def test_address_from_pubkey(self):
        addr = ATCCrypto.address_from_pubkey("pubkey123")
        self.assertTrue(addr.startswith("ATC"))
        self.assertEqual(len(addr), 35)

    def test_is_valid_address(self):
        addr = ATCCrypto.address_from_pubkey("pubkey123")
        self.assertTrue(ATCCrypto.is_valid_address(addr))
        self.assertFalse(ATCCrypto.is_valid_address("invalid"))
        self.assertFalse(ATCCrypto.is_valid_address("ATCshort"))

    def test_random_bytes(self):
        r = ATCCrypto.random_bytes(32)
        self.assertEqual(len(r), 32)

    def test_random_int(self):
        n = ATCCrypto.random_int(1, 100)
        self.assertGreaterEqual(n, 1)
        self.assertLessEqual(n, 100)


class TestATCCollections(unittest.TestCase):
    """ATC::Collections — 8+ Tests, 90% Coverage Target"""

    def test_map_operations(self):
        m = ATCCollections.map_new()
        ATCCollections.map_set(m, "key", "value")
        self.assertEqual(ATCCollections.map_get(m, "key"), "value")
        self.assertTrue(ATCCollections.map_contains(m, "key"))
        self.assertEqual(ATCCollections.map_size(m), 1)
        ATCCollections.map_delete(m, "key")
        self.assertFalse(ATCCollections.map_contains(m, "key"))

    def test_map_keys_values(self):
        m = {"a": 1, "b": 2}
        self.assertEqual(sorted(ATCCollections.map_keys(m)), ["a", "b"])
        self.assertEqual(sorted(ATCCollections.map_values(m)), [1, 2])

    def test_array_operations(self):
        arr = ATCCollections.array_new()
        ATCCollections.array_push(arr, 1)
        ATCCollections.array_push(arr, 2)
        ATCCollections.array_push(arr, 3)
        self.assertEqual(ATCCollections.array_len(arr), 3)
        self.assertEqual(ATCCollections.array_get(arr, 0), 1)
        self.assertTrue(ATCCollections.array_contains(arr, 2))
        self.assertEqual(ATCCollections.array_pop(arr), 3)
        self.assertEqual(ATCCollections.array_len(arr), 2)

    def test_array_slice_reverse_sort(self):
        arr = [3, 1, 2]
        self.assertEqual(ATCCollections.array_slice(arr, 0, 2), [3, 1])
        self.assertEqual(ATCCollections.array_reverse(arr), [2, 1, 3])
        self.assertEqual(ATCCollections.array_sort(arr), [1, 2, 3])
        self.assertEqual(ATCCollections.array_sort(arr, reverse=True), [3, 2, 1])

    def test_set_operations(self):
        s = ATCCollections.set_new()
        ATCCollections.set_add(s, 1)
        ATCCollections.set_add(s, 2)
        ATCCollections.set_add(s, 1)  # dup
        self.assertEqual(ATCCollections.set_size(s), 2)
        self.assertTrue(ATCCollections.set_contains(s, 1))

    def test_set_union_intersection(self):
        a = {1, 2, 3}
        b = {2, 3, 4}
        self.assertEqual(ATCCollections.set_union(a, b), {1, 2, 3, 4})
        self.assertEqual(ATCCollections.set_intersection(a, b), {2, 3})
        self.assertEqual(ATCCollections.set_difference(a, b), {1})

    def test_queue_operations(self):
        q = ATCCollections.queue_new()
        ATCCollections.queue_enqueue(q, "a")
        ATCCollections.queue_enqueue(q, "b")
        self.assertEqual(ATCCollections.queue_size(q), 2)
        self.assertEqual(ATCCollections.queue_peek(q), "a")
        self.assertEqual(ATCCollections.queue_dequeue(q), "a")
        self.assertEqual(ATCCollections.queue_dequeue(q), "b")

    def test_stack_operations(self):
        s = ATCCollections.stack_new()
        ATCCollections.stack_push(s, "x")
        ATCCollections.stack_push(s, "y")
        self.assertEqual(ATCCollections.stack_size(s), 2)
        self.assertEqual(ATCCollections.stack_peek(s), "y")
        self.assertEqual(ATCCollections.stack_pop(s), "y")
        self.assertEqual(ATCCollections.stack_pop(s), "x")


class TestATCIO(unittest.TestCase):
    """ATC::IO — 4+ Tests, 80% Coverage Target"""

    def test_print(self):
        # Just verify it doesn't crash
        ATCIO.print("hello")
        ATCIO.println("world")

    def test_format(self):
        result = ATCIO.format("Hello {}!", "ATCLang")
        self.assertEqual(result, "Hello ATCLang!")

    def test_file_write_read(self):
        import tempfile
        path = tempfile.mktemp()
        self.assertTrue(ATCIO.file_write(path, "test content"))
        self.assertTrue(ATCIO.file_exists(path))
        self.assertEqual(ATCIO.file_read(path), "test content")
        ATCIO.file_delete(path)

    def test_file_append(self):
        import tempfile
        path = tempfile.mktemp()
        ATCIO.file_write(path, "line1\n")
        ATCIO.file_append(path, "line2\n")
        self.assertEqual(ATCIO.file_read(path), "line1\nline2\n")
        ATCIO.file_delete(path)


class TestATCMath(unittest.TestCase):
    """ATC::Math — 5+ Tests, 95% Coverage Target"""

    def test_safe_add(self):
        self.assertEqual(ATCMath.safe_add(2, 3), 5)

    def test_safe_sub(self):
        self.assertEqual(ATCMath.safe_sub(10, 3), 7)

    def test_safe_mul(self):
        self.assertEqual(ATCMath.safe_mul(4, 5), 20)

    def test_safe_div(self):
        self.assertEqual(ATCMath.safe_div(20, 4), 5)

    def test_mod_exp(self):
        result = ATCMath.mod_exp(2, 10, 1000)
        self.assertEqual(result, 24)  # 2^10 = 1024, 1024 % 1000 = 24


class TestATCEncoding(unittest.TestCase):
    """ATC::Encoding — 5+ Tests, 90% Coverage Target"""

    def test_json_roundtrip(self):
        obj = {"name": "ATCLang", "version": 3, "features": ["lexer", "parser"]}
        encoded = ATCEncoding.json_encode(obj)
        decoded = ATCEncoding.json_decode(encoded)
        self.assertEqual(decoded["name"], "ATCLang")
        self.assertEqual(decoded["version"], 3)

    def test_cbor_int(self):
        data = ATCEncoding.cbor_encode(42)
        decoded = ATCEncoding.cbor_decode(data)
        self.assertEqual(decoded, 42)

    def test_cbor_string(self):
        data = ATCEncoding.cbor_encode("hello")
        decoded = ATCEncoding.cbor_decode(data)
        self.assertEqual(decoded, "hello")

    def test_cbor_list(self):
        data = ATCEncoding.cbor_encode([1, 2, 3])
        decoded = ATCEncoding.cbor_decode(data)
        self.assertEqual(decoded, [1, 2, 3])

    def test_cbor_map(self):
        data = ATCEncoding.cbor_encode({"key": "value"})
        decoded = ATCEncoding.cbor_decode(data)
        self.assertEqual(decoded["key"], "value")

    def test_hex_roundtrip(self):
        data = b"hello"
        encoded = ATCEncoding.hex_encode(data)
        decoded = ATCEncoding.hex_decode(encoded)
        self.assertEqual(data, decoded)

    def test_cbor_none_bool(self):
        self.assertEqual(ATCEncoding.cbor_decode(ATCEncoding.cbor_encode(None)), None)
        self.assertEqual(ATCEncoding.cbor_decode(ATCEncoding.cbor_encode(True)), True)
        self.assertEqual(ATCEncoding.cbor_decode(ATCEncoding.cbor_encode(False)), False)


class TestATCPrimitives(unittest.TestCase):
    """ATC::Primitives — 3+ Tests, 85% Coverage Target"""

    def test_address_creation(self):
        addr = ATCPrimitives.new_address("mypubkey")
        self.assertTrue(str(addr.as_string()).startswith("ATC"))
        self.assertEqual(len(addr.as_string()), 35)

    def test_address_validation(self):
        addr = ATCPrimitives.new_address("mypubkey")
        self.assertTrue(ATCPrimitives.is_valid_address(addr.as_string()))
        self.assertFalse(ATCPrimitives.is_valid_address("invalid"))

    def test_hash_computation(self):
        h = ATCPrimitives.compute_hash(b"block data")
        self.assertEqual(len(h.as_string()), 64)

    def test_transaction(self):
        tx = ATCPrimitives.new_transaction("ATCsender", "ATCreceiver", 1000)
        tx_hash = tx.compute_hash()
        self.assertEqual(len(tx_hash), 64)
        sig = tx.sign("private_key")
        self.assertIsNotNone(sig)

    def test_block_header(self):
        header = ATCPrimitives.new_block_header(1, "0" * 64, "merkle_root_hash")
        h = header.compute_hash()
        self.assertEqual(len(h), 64)
        self.assertEqual(header.number, 1)

    def test_sign_verify_cycle(self):
        """Integration: sign → verify → recover"""
        priv, pub = ATCCrypto.generate_keypair()
        msg = "test transaction"
        sig = ATCCrypto.sign(msg, priv)
        self.assertTrue(ATCCrypto.verify(msg, sig, priv))
        self.assertFalse(ATCCrypto.verify("tampered", sig, priv))

    def test_json_tx_roundtrip(self):
        """Integration: Transaction → JSON → Transaction"""
        import json
        tx = ATCPrimitives.new_transaction("ATCa", "ATCb", 500, nonce=1)
        tx.compute_hash()
        json_str = ATCEncoding.json_encode(tx.to_dict())
        decoded = ATCEncoding.json_decode(json_str)
        self.assertEqual(decoded["sender"], "ATCa")
        self.assertEqual(decoded["amount"], 500)


if __name__ == "__main__":
    unittest.main()

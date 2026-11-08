# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""ATCLang VM STDLIB_DISPATCH Tests — ATC-94
Tests für die Integration der Stdlib-Module in die VM.
"""
import unittest
import sys
import os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from atclang.vm.atcvm import STDLIB_DISPATCH, ATCVM, OP, Instruction


class TestCryptoDispatch(unittest.TestCase):
    """Crypto module dispatch via ATC::Crypto::*."""

    def test_sha256(self):
        result = STDLIB_DISPATCH["ATC::Crypto::sha256"](["hello"])
        self.assertEqual(len(result), 64)  # SHA-256 hex = 64 chars

    def test_sha256_bytes(self):
        result = STDLIB_DISPATCH["ATC::Crypto::sha256_bytes"](["hello"])
        self.assertEqual(len(result), 32)  # SHA-256 = 32 bytes

    def test_double_sha256(self):
        result = STDLIB_DISPATCH["ATC::Crypto::double_sha256"](["hello"])
        self.assertEqual(len(result), 64)

    def test_hmac_sha256(self):
        result = STDLIB_DISPATCH["ATC::Crypto::hmac_sha256"](["key", "msg"])
        self.assertEqual(len(result), 64)

    def test_base58_encode(self):
        result = STDLIB_DISPATCH["ATC::Crypto::base58_encode"]([b"hello"])
        self.assertIsInstance(result, str)

    def test_base58_decode(self):
        encoded = STDLIB_DISPATCH["ATC::Crypto::base58_encode"]([b"test"])
        decoded = STDLIB_DISPATCH["ATC::Crypto::base58_decode"]([encoded])
        self.assertEqual(decoded, b"test")

    def test_base64_roundtrip(self):
        encoded = STDLIB_DISPATCH["ATC::Crypto::base64_encode"]([b"hello"])
        decoded = STDLIB_DISPATCH["ATC::Crypto::base64_decode"]([encoded])
        self.assertEqual(decoded, b"hello")

    def test_hex_roundtrip(self):
        encoded = STDLIB_DISPATCH["ATC::Crypto::hex_encode"]([b"hello"])
        decoded = STDLIB_DISPATCH["ATC::Crypto::hex_decode"]([encoded])
        self.assertEqual(decoded, b"hello")

    def test_generate_keypair(self):
        priv, pub = STDLIB_DISPATCH["ATC::Crypto::generate_keypair"]([])
        self.assertIsInstance(priv, str)
        self.assertIsInstance(pub, str)

    def test_sign_verify(self):
        # HMAC-based stub: sign with priv, verify with same key
        priv, pub = STDLIB_DISPATCH["ATC::Crypto::generate_keypair"]([])
        sig = STDLIB_DISPATCH["ATC::Crypto::sign"](["message", priv])
        # Verify uses same key (HMAC stub, not real ECDSA yet)
        valid = STDLIB_DISPATCH["ATC::Crypto::verify"](["message", sig, priv])
        self.assertTrue(valid)

    def test_random_bytes(self):
        result = STDLIB_DISPATCH["ATC::Crypto::random_bytes"]([32])
        self.assertEqual(len(result), 32)

    def test_random_int(self):
        result = STDLIB_DISPATCH["ATC::Crypto::random_int"]([1, 100])
        self.assertGreaterEqual(result, 1)
        self.assertLessEqual(result, 100)

    def test_address_from_pubkey(self):
        priv, pub = STDLIB_DISPATCH["ATC::Crypto::generate_keypair"]([])
        addr = STDLIB_DISPATCH["ATC::Crypto::address_from_pubkey"]([pub])
        self.assertTrue(addr.startswith("ATC"))

    def test_is_valid_address(self):
        priv, pub = STDLIB_DISPATCH["ATC::Crypto::generate_keypair"]([])
        addr = STDLIB_DISPATCH["ATC::Crypto::address_from_pubkey"]([pub])
        valid = STDLIB_DISPATCH["ATC::Crypto::is_valid_address"]([addr])
        self.assertTrue(valid)


class TestCollectionsDispatch(unittest.TestCase):
    """Collections module dispatch."""

    def test_map_operations(self):
        m = STDLIB_DISPATCH["ATC::Collections::map_new"]([])
        m = STDLIB_DISPATCH["ATC::Collections::map_set"]([m, "key", "value"])
        val = STDLIB_DISPATCH["ATC::Collections::map_get"]([m, "key"])
        self.assertEqual(val, "value")

    def test_map_contains(self):
        m = STDLIB_DISPATCH["ATC::Collections::map_new"]([])
        m = STDLIB_DISPATCH["ATC::Collections::map_set"]([m, "key", "value"])
        self.assertTrue(STDLIB_DISPATCH["ATC::Collections::map_contains"]([m, "key"]))

    def test_map_keys(self):
        m = STDLIB_DISPATCH["ATC::Collections::map_new"]([])
        m = STDLIB_DISPATCH["ATC::Collections::map_set"]([m, "a", 1])
        m = STDLIB_DISPATCH["ATC::Collections::map_set"]([m, "b", 2])
        keys = STDLIB_DISPATCH["ATC::Collections::map_keys"]([m])
        self.assertEqual(len(keys), 2)

    def test_map_size(self):
        m = STDLIB_DISPATCH["ATC::Collections::map_new"]([])
        m = STDLIB_DISPATCH["ATC::Collections::map_set"]([m, "a", 1])
        self.assertEqual(STDLIB_DISPATCH["ATC::Collections::map_size"]([m]), 1)

    def test_array_operations(self):
        arr = STDLIB_DISPATCH["ATC::Collections::array_new"]([])
        arr = STDLIB_DISPATCH["ATC::Collections::array_push"]([arr, 1])
        arr = STDLIB_DISPATCH["ATC::Collections::array_push"]([arr, 2])
        self.assertEqual(STDLIB_DISPATCH["ATC::Collections::array_len"]([arr]), 2)
        val = STDLIB_DISPATCH["ATC::Collections::array_get"]([arr, 0])
        self.assertEqual(val, 1)

    def test_array_pop(self):
        arr = STDLIB_DISPATCH["ATC::Collections::array_new"]([])
        arr = STDLIB_DISPATCH["ATC::Collections::array_push"]([arr, 42])
        val = STDLIB_DISPATCH["ATC::Collections::array_pop"]([arr])
        self.assertEqual(val, 42)

    def test_set_operations(self):
        s = STDLIB_DISPATCH["ATC::Collections::set_new"]([])
        s = STDLIB_DISPATCH["ATC::Collections::set_add"]([s, "item"])
        self.assertTrue(STDLIB_DISPATCH["ATC::Collections::set_contains"]([s, "item"]))


class TestEncodingDispatch(unittest.TestCase):
    """Encoding module dispatch."""

    def test_json_encode(self):
        result = STDLIB_DISPATCH["ATC::Encoding::json_encode"]([{"key": "value"}])
        self.assertIn("key", result)

    def test_json_decode(self):
        result = STDLIB_DISPATCH["ATC::Encoding::json_decode"](['{"key": "value"}'])
        self.assertEqual(result["key"], "value")

    def test_json_roundtrip(self):
        obj = {"name": "test", "value": 42, "nested": {"a": 1}}
        encoded = STDLIB_DISPATCH["ATC::Encoding::json_encode"]([obj])
        decoded = STDLIB_DISPATCH["ATC::Encoding::json_decode"]([encoded])
        self.assertEqual(decoded["name"], "test")

    def test_cbor_encode(self):
        result = STDLIB_DISPATCH["ATC::Encoding::cbor_encode"]([42])
        self.assertIsInstance(result, bytes)

    def test_cbor_roundtrip(self):
        obj = {"name": "test", "values": [1, 2, 3]}
        encoded = STDLIB_DISPATCH["ATC::Encoding::cbor_encode"]([obj])
        decoded = STDLIB_DISPATCH["ATC::Encoding::cbor_decode"]([encoded])
        self.assertEqual(decoded["name"], "test")

    def test_hex_roundtrip(self):
        encoded = STDLIB_DISPATCH["ATC::Encoding::hex_encode"]([b"hello"])
        decoded = STDLIB_DISPATCH["ATC::Encoding::hex_decode"]([encoded])
        self.assertEqual(decoded, b"hello")


class TestMathDispatch(unittest.TestCase):
    """Math module dispatch."""

    def test_add(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::add"]([10, 20]), 30)

    def test_sub(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::sub"]([30, 10]), 20)

    def test_mul(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::mul"]([5, 6]), 30)

    def test_div(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::div"]([20, 4]), 5)

    def test_mod(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::mod"]([10, 3]), 1)

    def test_pow(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::pow"]([2, 8]), 256)

    def test_sqrt(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::sqrt"]([16]), 4)

    def test_mod_exp(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::mod_exp"]([2, 10, 1000]), 24)

    def test_gcd(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::gcd"]([12, 8]), 4)

    def test_lcm(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::lcm"]([4, 6]), 12)

    def test_abs(self):
        self.assertEqual(STDLIB_DISPATCH["ATC::Math::abs"]([-5]), 5)


class TestPrimitivesDispatch(unittest.TestCase):
    """Primitives module dispatch."""

    def test_address_new(self):
        addr = STDLIB_DISPATCH["ATC::Primitives::Address::new"](["ATC" + "a" * 32])
        self.assertIsNotNone(addr)

    def test_address_is_valid(self):
        valid = STDLIB_DISPATCH["ATC::Primitives::Address::is_valid"](["ATC" + "a" * 32])
        self.assertTrue(valid)

    def test_hash_compute(self):
        h = STDLIB_DISPATCH["ATC::Primitives::Hash::compute"](["hello"])
        self.assertIsNotNone(h)

    def test_transaction_new(self):
        tx = STDLIB_DISPATCH["ATC::Primitives::Transaction::new"]([
            "ATC_sender", "ATC_receiver", 100, 5, 1
        ])
        self.assertEqual(tx["sender"], "ATC_sender")
        self.assertEqual(tx["amount"], 100)

    def test_block_header_new(self):
        bh = STDLIB_DISPATCH["ATC::Primitives::BlockHeader::new"]([
            1, "prev_hash", "merkle_root", 1234567890
        ])
        self.assertEqual(bh["number"], 1)


class TestVMIntegration(unittest.TestCase):
    """VM-level integration: CALL_EXT with stdlib dispatch."""

    def test_vm_sha256(self):
        vm = ATCVM()
        insts = [
            Instruction(OP.PUSH, ["hello"]),
            Instruction(OP.CALL_EXT, ["ATC::Crypto::sha256", 1]),
            Instruction(OP.HALT),
        ]
        result = vm.execute(insts)
        self.assertEqual(len(str(result)), 64)

    def test_vm_json_encode(self):
        vm = ATCVM()
        insts = [
            Instruction(OP.PUSH, [{"key": "value"}]),
            Instruction(OP.CALL_EXT, ["ATC::Encoding::json_encode", 1]),
            Instruction(OP.HALT),
        ]
        result = vm.execute(insts)
        self.assertIn("key", str(result))

    def test_vm_math(self):
        vm = ATCVM()
        insts = [
            Instruction(OP.PUSH, [10]),
            Instruction(OP.PUSH, [20]),
            Instruction(OP.CALL_EXT, ["ATC::Math::add", 2]),
            Instruction(OP.HALT),
        ]
        result = vm.execute(insts)
        self.assertEqual(result, 30)

    def test_vm_base64_roundtrip(self):
        vm = ATCVM()
        # Encode
        insts = [
            Instruction(OP.PUSH, [b"hello"]),
            Instruction(OP.CALL_EXT, ["ATC::Crypto::base64_encode", 1]),
            Instruction(OP.HALT),
        ]
        encoded = vm.execute(insts)
        # Decode
        vm2 = ATCVM()
        insts2 = [
            Instruction(OP.PUSH, [encoded]),
            Instruction(OP.CALL_EXT, ["ATC::Crypto::base64_decode", 1]),
            Instruction(OP.HALT),
        ]
        decoded = vm2.execute(insts2)
        self.assertEqual(decoded, b"hello")


class TestDispatchCompleteness(unittest.TestCase):
    """Verify dispatch table covers all stdlib modules."""

    def test_has_crypto_functions(self):
        crypto_fns = [k for k in STDLIB_DISPATCH if k.startswith("ATC::Crypto::")]
        self.assertGreaterEqual(len(crypto_fns), 10)

    def test_has_collections_functions(self):
        coll_fns = [k for k in STDLIB_DISPATCH if k.startswith("ATC::Collections::")]
        self.assertGreaterEqual(len(coll_fns), 10)

    def test_has_encoding_functions(self):
        enc_fns = [k for k in STDLIB_DISPATCH if k.startswith("ATC::Encoding::")]
        self.assertGreaterEqual(len(enc_fns), 5)

    def test_has_math_functions(self):
        math_fns = [k for k in STDLIB_DISPATCH if k.startswith("ATC::Math::")]
        self.assertGreaterEqual(len(math_fns), 8)

    def test_has_primitives_functions(self):
        prim_fns = [k for k in STDLIB_DISPATCH if k.startswith("ATC::Primitives::")]
        self.assertGreaterEqual(len(prim_fns), 3)

    def test_total_dispatch_count(self):
        self.assertGreaterEqual(len(STDLIB_DISPATCH), 60)


if __name__ == "__main__":
    unittest.main()

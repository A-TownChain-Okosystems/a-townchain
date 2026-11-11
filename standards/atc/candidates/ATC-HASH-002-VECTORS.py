#!/usr/bin/env python3
"""ATC-HASH-002 Referenzvektoren. Unabhaengig von der geprueften Rust-Implementierung.
Backend: hashlib.sha256 (OpenSSL). Aufruf: python3 ATC-HASH-002-VECTORS.py"""
import hashlib, struct

H = lambda b: hashlib.sha256(b).digest()
hx = lambda b: b.hex()
d = lambda s: s.encode() + b"\x00"
BE16 = lambda v: struct.pack(">H", v)
BE32 = lambda v: struct.pack(">I", v)
BE64 = lambda v: struct.pack(">Q", v)

def fips_selftest():
    cases = {
        b"": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        b"abc": "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq":
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        b"a" * 1000000: "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0",
    }
    for msg, expected in cases.items():
        assert hx(H(msg)) == expected, "FIPS-Selbsttest fehlgeschlagen"
    print("FIPS-180-4 Selbsttest: 4/4 bestanden")

FIELDS = [("version",2),("chain_id",8),("height",8),("prev_hash",32),("tx_root",32),
          ("state_root",32),("receipt_root",32),("gas_used",8),("timestamp",8),("proposer",33)]
PROP = bytes.fromhex("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")

def header(version, chain, height, prev, txr, sr, rr, gas, ts):
    b = (BE16(version) + BE64(chain) + BE64(height) + prev + txr + sr + rr
         + BE64(gas) + BE64(ts) + PROP)
    assert len(b) == sum(w for _, w in FIELDS) == 195, len(b)
    return b

DOMAINS = ["ATC-BLOCK-V2","ATC-TXID-V1","ATC-MERKLE-L-V1","ATC-MERKLE-N-V1",
           "ATC-STATE-V1","ATC-STATE-N-V1","ATC-RECEIPT-V1",
           "ATC-MERKLE-EMPTY-V1","ATC-STATE-EMPTY-V1"]

leaf  = lambda t: H(d("ATC-MERKLE-L-V1") + t)
node  = lambda l, r: H(d("ATC-MERKLE-N-V1") + l + r)
sleaf = lambda k, v: H(d("ATC-STATE-V1") + BE32(len(k)) + k + BE32(len(v)) + v)

def main():
    fips_selftest()
    print(f"Header-Laenge: {sum(w for _, w in FIELDS)} Byte | "
          f"Blockhash-Eingabe: {len(d('ATC-BLOCK-V2')) + 195} Byte")

    print("\n# 8.2 leere Payload je Domain")
    seen = set()
    for dom in DOMAINS:
        v = hx(H(d(dom))); seen.add(v)
        print(f"{dom:22s} {v}")
    assert len(seen) == len(DOMAINS), "Domain-Kollision"

    Z = bytes(32)
    print("\n# 8.3 Blockhash")
    print("B1", hx(H(d("ATC-BLOCK-V2") + header(1, 658467, 1, b"\x11"*32,
            b"\x22"*32, b"\x33"*32, b"\x44"*32, 21000, 1762900000))))
    print("B0", hx(H(d("ATC-BLOCK-V2") + header(1, 658467, 0, Z, Z, Z, Z, 0, 0))))

    print("\n# 8.4 Tx-ID und Beleg")
    print("T1", hx(H(d("ATC-TXID-V1") + bytes.fromhex("deadbeef") + BE64(658467) + BE64(1))))
    print("R1", hx(H(d("ATC-RECEIPT-V1") + BE64(1) + BE32(0) + b"\x00" + BE64(21000) + b"\xaa"*32)))

    A, B, C = b"\xaa"*32, b"\xbb"*32, b"\xcc"*32
    lA, lB, lC = leaf(A), leaf(B), leaf(C)
    m3, m4 = node(node(lA, lB), lC), node(node(lA, lB), node(lC, C))
    print("\n# 8.5 Merkle")
    print("M0", hx(H(d("ATC-MERKLE-EMPTY-V1"))))
    print("M1", hx(lA)); print("M2", hx(node(lA, lB)))
    print("M3", hx(m3)); print("M4", hx(m4))
    assert m3 != m4 and H(d("ATC-MERKLE-EMPTY-V1")) != lA

    e1, e2 = sleaf(b"alice", b"50"), sleaf(b"bob", b"100")
    print("\n# 8.6 Zustand")
    print("S1", hx(H(d("ATC-STATE-N-V1") + e1 + e2)), "(sortiert nach key_bytes)")
    print("S0", hx(H(d("ATC-STATE-EMPTY-V1"))))

    print("\n# 8.7 Grenzlaengen (Padding-Kanten), Domain ATC-TXID-V1")
    for n in (0, 1, 55, 56, 57, 63, 64, 65, 119, 120, 121):
        print(f"len={n:3d} {hx(H(d('ATC-TXID-V1') + b'k'*n))}")

if __name__ == "__main__":
    main()

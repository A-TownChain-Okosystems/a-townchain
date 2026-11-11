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


# --- Rev. 3.1: Tiefen- und Grenzpruefungen (unabhaengig, referenzseitig) ---
MAX_LEAVES = 1 << 20

def depth_of(n):
    """Tiefe nach H-7.7: 0 fuer ein Blatt, ceil(log2(n)) ab zwei Blaettern."""
    if n == 0:
        return None            # Leerwurzel, eigener Fall (H-7.3)
    if n == 1:
        return 0
    d, leaves = 0, 1
    while leaves < n:
        leaves *= 2
        d += 1
    return d

def promote_root(leaves):
    """Merkle nach H-6.4/H-7.5: je Ebene paarweise, letzter Knoten wird hochgezogen."""
    level = list(leaves)
    while len(level) > 1:
        nxt = [node(level[i], level[i + 1]) for i in range(0, len(level) - 1, 2)]
        if len(level) % 2:
            nxt.append(level[-1])          # hochziehen, nicht duplizieren
        level = nxt
    return level[0]

def guard(count, payload_bytes):
    """H-7.7b: Pruefung VOR der Allokation."""
    if count > MAX_LEAVES:
        raise ValueError(f"ADM-LEN-01: {count} Blaetter ueberschreiten 2^20")
    if payload_bytes > 4 * 1024 * 1024:
        raise ValueError(f"ADM-LEN-02: {payload_bytes} Byte ueberschreiten 4 MiB")
    return True

def checks_rev31():
    print("\n# Rev. 3.1 Tiefenformel")
    for n in (0, 1, 2, 3, 4, 5, 8, 9, MAX_LEAVES):
        print(f"n={n:>8} -> Tiefe {depth_of(n)}")
    assert depth_of(MAX_LEAVES) == 20
    assert all(depth_of(n) <= 20 for n in range(1, 2000))

    print("# Rev. 3.1 Hochziehen ueber alle Blattzahlen 1..12 (Invariante: je Ebene schrumpft)")
    for n in range(1, 13):
        twigs = [leaf(i.to_bytes(32, "big")) for i in range(n)]
        r = promote_root(twigs)
        assert len(r) == 32
        print(f"n={n:2d} Tiefe {depth_of(n)} Wurzel {hx(r)[:16]}…")

    print("# Rev. 3.1 Grenzpruefung vor Allokation")
    for count, payload in ((MAX_LEAVES, 0), (MAX_LEAVES + 1, 0), (1, 4 * 1024 * 1024), (1, 4 * 1024 * 1024 + 1)):
        try:
            guard(count, payload); print(f"count={count} payload={payload}: akzeptiert")
        except ValueError as e:
            print(f"count={count} payload={payload}: abgelehnt ({e})")

if __name__ == "__main__":
    checks_rev31()

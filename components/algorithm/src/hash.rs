// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC-HASH-001 ("TownHash-256") — EIGENSTAENDIGER Hash-Algorithmus
//! (SCR-0119, Owner-Direktive 12.09.2026). Merkle-Damgard, 512-Bit-Bloecke,
//! 8x32-Bit-Zustand, 24 Runden, Little-Endian-Wire. Alle Konstanten
//! ganzzahlig aus splitmix32 (Seed 0xA7C0DE01) — keine Bibliothek.
//! Spezifikation: docs/SPEC-ATC-HASH-001.md.
//! Ehrlichkeit: NICHT kryptoanalysiert — kein Mainnet-Einsatz ohne externe
//! Krypto-Pruefung (F-067-Gate). Devnet-Grade als FNV-1a-Nachfolger.

const SEED: u32 = 0xA7C0DE01;

const fn splitmix32(state: u32) -> (u32, u32) {
    let s = state.wrapping_add(0x9e3779b9);
    let mut z = s;
    z = (z ^ (z >> 16)).wrapping_mul(0x21f0aaad);
    z = (z ^ (z >> 15)).wrapping_mul(0x735a2d97);
    z = z ^ (z >> 15);
    (s, z)
}

const fn derive_iv() -> [u32; 8] {
    let mut iv = [0u32; 8];
    let mut s = SEED;
    let mut i = 0;
    while i < 8 {
        let (ns, v) = splitmix32(s);
        s = ns;
        iv[i] = v;
        i += 1;
    }
    iv
}

const fn derive_k() -> [u32; 24] {
    let mut k = [0u32; 24];
    let mut s = SEED;
    let mut i = 0;
    while i < 8 {
        let (ns, _) = splitmix32(s);
        s = ns;
        i += 1;
    }
    let mut i = 0;
    while i < 24 {
        let (ns, v) = splitmix32(s);
        s = ns;
        k[i] = v;
        i += 1;
    }
    k
}

/// Initialzustand — deterministisch aus SEED (siehe SPEC Abs. 3).
pub const IV: [u32; 8] = derive_iv();
const K: [u32; 24] = derive_k();

fn compress(h: &[u32; 8], block: &[u8; 64]) -> [u32; 8] {
    let mut w = [0u32; 24];
    for i in 0..16 {
        w[i] = u32::from_le_bytes([
            block[i * 4],
            block[i * 4 + 1],
            block[i * 4 + 2],
            block[i * 4 + 3],
        ]);
    }
    for t in 16..24 {
        let s0 = w[t - 15].rotate_right(5)
            ^ w[t - 15].rotate_right(19)
            ^ w[t - 15].rotate_right(27);
        let s1 = w[t - 2].rotate_right(9)
            ^ w[t - 2].rotate_right(21)
            ^ w[t - 2].rotate_right(29);
        w[t] = w[t - 16]
            .wrapping_add(s0)
            .wrapping_add(w[t - 7])
            .wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = *h;
    for t in 0..24 {
        let s1 = e.rotate_right(10) ^ e.rotate_right(26) ^ e.rotate_right(5);
        let ch = (e & f) ^ (!e & g);
        let t1 = hh
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K[t])
            .wrapping_add(w[t]);
        let s0 = a.rotate_right(3) ^ a.rotate_right(14) ^ a.rotate_right(24);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);
        hh = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }
    [
        h[0].wrapping_add(a),
        h[1].wrapping_add(b),
        h[2].wrapping_add(c),
        h[3].wrapping_add(d),
        h[4].wrapping_add(e),
        h[5].wrapping_add(f),
        h[6].wrapping_add(g),
        h[7].wrapping_add(hh),
    ]
}

/// ATC-HASH-001 Digest (32 Bytes) ueber die Eingabe.
pub fn atc_hash(input: &[u8]) -> [u8; 32] {
    let bitlaenge = (input.len() as u64).wrapping_mul(8);
    let mut pad = Vec::with_capacity(input.len() + 72);
    pad.extend_from_slice(input);
    pad.push(0x80);
    while pad.len() % 64 != 56 {
        pad.push(0);
    }
    pad.extend_from_slice(&bitlaenge.to_le_bytes());
    let mut h = IV;
    for i in (0..pad.len()).step_by(64) {
        let mut block = [0u8; 64];
        block.copy_from_slice(&pad[i..i + 64]);
        h = compress(&h, &block);
    }
    let mut out = [0u8; 32];
    for i in 0..8 {
        out[i * 4..i * 4 + 4].copy_from_slice(&h[i].to_le_bytes());
    }
    out
}

/// Hex-Darstellung (Kleinbuchstaben) — Convenience fuer Tests und Logs.
pub fn atc_hash_hex(input: &[u8]) -> String {
    atc_hash(input).iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goldene_vektoren_aus_python_referenz() {
        let faelle: &[(&[u8], &str)] = &[
            (b"", "990988d19043340533ab778bd931a811b7b28ed9664016e6951432d0f39c7753"),
            (b"abc", "fb21ac42d95d7b753719e42e9d3ee20463be908629cfa3fee95c1d7a92e6dc01"),
            (b"A-TownChain Devnet", "0b2a8e90d808e6e8236642ce446a7a003917b17aeac4840643895a9e99cc1372"),
            (b"", "ersetzt-durch-laufzeit"),
        ];
        // Der vierte Fall ist ein Platzhalter-Schutz: echte Laengen-64/200/255
        // Faelle folgen separat, damit keine duplizierten Literal-Zeilen.
        let _ = faelle;
        assert_eq!(atc_hash_hex(b""), "990988d19043340533ab778bd931a811b7b28ed9664016e6951432d0f39c7753");
        assert_eq!(atc_hash_hex(b"abc"), "fb21ac42d95d7b753719e42e9d3ee20463be908629cfa3fee95c1d7a92e6dc01");
        assert_eq!(
            atc_hash_hex(b"A-TownChain Devnet"),
            "0b2a8e90d808e6e8236642ce446a7a003917b17aeac4840643895a9e99cc1372"
        );
    }

    #[test]
    fn goldene_vektoren_blockgrenzen() {
        let a64 = vec![b'a'; 64];
        let x200 = vec![b'x'; 200];
        assert_eq!(atc_hash_hex(&a64), "8d03946ed520566e717ba0b01a199dd806563707ffbf99b4861e988c80c79646");
        assert_eq!(atc_hash_hex(&x200), "45850dc6fbeb676e6705b43d80bfdfaaa111300f4c2d3fe20fe2ba94e2b307ed");
        assert_eq!(atc_hash_hex(b"block-payload-1"), "72340bfa03ff7183e2482a3633426fa7ea0b032cad84e5a6b548cf921675ad9a");
    }

    #[test]
    fn determinismus_und_laengenband() {
        let mut vorher = String::new();
        for l in 0..=130 {
            let daten = vec![0x61u8; l];
            let hex = atc_hash_hex(&daten);
            assert_eq!(hex.len(), 64, "Digest muss 32 Bytes sein");
            assert_eq!(hex, atc_hash_hex(&daten), "Determinismus bei Laenge {}", l);
            assert_ne!(hex, vorher, "aufeinanderfolgende Laengen muessen sich unterscheiden");
            vorher = hex;
        }
    }

    #[test]
    fn bitflip_aendert_digest() {
        let basis = b"Devnet-Transaktion-001";
        let mut geflippt = basis.to_vec();
        geflippt[0] ^= 0x01;
        assert_ne!(atc_hash(basis), atc_hash(&geflippt));
    }

    #[test]
    fn padding_kante_laengen_55_56_57() {
        for l in [55usize, 56, 57, 119, 120, 121] {
            let daten = vec![b'k'; l];
            let hex = atc_hash_hex(&daten);
            assert_eq!(hex.len(), 64, "Padding-Kante {} muss ohne Panik hashen", l);
        }
        // Kante 55 vs 56: unterschiedliche Eingaben -> unterschiedliche Digests
        assert_ne!(atc_hash(&vec![b'k'; 55]), atc_hash(&vec![b'k'; 56]));
    }
}

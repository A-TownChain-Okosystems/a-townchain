# SPEC-ATC-HASH-001 — "TownHash-256" (v1.0.0 DRAFT)

Copyright (c) 2026 Michael Wroblewski — Apache-2.0
Status: DRAFT, SCR-0119, Owner-Direktive 12.09.2026 ("atc-algorithm soll ein
eigenstaendiger Algorithmus wie SHA-256 sein").

## 1. Zweck

ATC-HASH-001 ist der EIGENSTAENDIGE Hash-Algorithmus des A-TownChain-
Oekosystems: komplette Eigenkonstruktion (Struktur, Konstanten, Rundenzahl,
Serialisierung) — keine Uebernahme einer Standard-Bibliothek. Langfristig
ersetzt er die dokumentierten FNV-1a-Platzhalter im Devnet-Pfad
(atc-node chain/gossip, PoH).

## 2. Konstruktion (normativ)

- Merkle-Damgard: 512-Bit-Bloecke (64 Bytes), Verkettung ueber 8x32-Bit-Zustand.
- Padding: 0x80, Nullen bis Blocklaenge ≡ 56 (mod 64), dann 8-Byte
  Little-Endian-Bitlaenge (unterscheidet sich bewusst von SHA-2: LE statt BE).
- Blockoerffnung: 16 Worte Little-Endian (u32::from_le_bytes).
- Message Schedule (24 Worte): W[t] = W[t-16] + s0(W[t-15]) + W[t-7] +
  s1(W[t-2]) mit s0 = rotr(5,19,27), s1 = rotr(9,21,29).
- Kompression: 24 Runden; runde t:
  S1 = rotr(e,10) ^ rotr(e,26) ^ rotr(e,5); ch = (e&f) ^ (!e&g);
  t1 = h + S1 + ch + K[t] + W[t]; S0 = rotr(a,3) ^ rotr(a,14) ^ rotr(a,24);
  maj = (a&b) ^ (a&c) ^ (b&c); t2 = S0 + maj; klassischer 8-Wort-Roll.
  Feedforward: H'[i] = H[i] + Arbeitswort[i].
- Serialisierung: 8 Zustandswoerter Little-Endian -> 32-Byte-Digest.

## 3. Konstantenableitung (normativ)

Alle Konstanten GANZZAHLIG aus splitmix32, Seed 0xA7C0DE01 ("ATC-Code"),
in dieser Reihenfolge: zuerst 8 Ausgabeworte = IV, danach 24 weitere = K[0..23].
Keine Floats, keine externen Tabellen, in Rust als const fn reproduzierbar
(src/hash.rs) und in der Python-Referenz identisch nachvollzogen.

## 4. Testvektoren (differenzialgesichert: Python-Referenz vs. Rust)

| Eingabe                  | Digest (hex)                                                     |
|--------------------------|------------------------------------------------------------------|
| ""                       | 990988d19043340533ab778bd931a811b7b28ed9664016e6951432d0f39c7753 |
| "abc"                    | fb21ac42d95d7b753719e42e9d3ee20463be908629cfa3fee95c1d7a92e6dc01 |
| "A-TownChain Devnet"     | 0b2a8e90d808e6e8236642ce446a7a003917b17aeac4840643895a9e99cc1372 |
| "a" x 64                 | 8d03946ed520566e717ba0b01a199dd806563707ffbf99b4861e988c80c79646 |
| "x" x 200                | 45850dc6fbeb676e6705b43d80bfdfaaa111300f4c2d3fe20fe2ba94e2b307ed |
| "block-payload-1"        | 72340bfa03ff7183e2482a3633426fa7ea0b032cad84e5a6b548cf921675ad9a |

## 5. Sicherheitsstatus (ehrlich)

UNANALYSIERT. ATC-HASH-001 ist eine Eigenkonstruktion ohne jegliche
Kryptoanalyse — keine Widerstandsbehauptung gegen Kollisionen, Preimage oder
Length-Extension. Mainnet-Einsatz erfordert zwingend eine externe
kryptographische Pruefung (F-067-Gate). Devnet-Grade: Ja — ersetzt FNV-1a als
dokumentierter Upgrade-Pfad, sobald die Adoption in atc-node als eigene Welle
gepinnt wird (Status: offen).

## 6. Referenzimplementierungen

- Rust (kanonisch): src/hash.rs — const-fn-Konstantenableitung, Tests mit
  den Vektoren aus Abschnitt 4.
- Python (Referenzmodell fuer die Differenzialpruefung): SCR-0119-Wellen-
  Notiz; Vektoren wurden aus ihr erzeugt und sind hier eingefroren.

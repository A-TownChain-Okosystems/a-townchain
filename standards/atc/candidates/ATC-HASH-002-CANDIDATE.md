# ATC-HASH-002 — SHA-256 Cryptographic Hash Standard (KANDIDAT Rev. 3)

> **Status:** `CANDIDATE`
> **Normative Registrierung:** `BLOCKED` — Branchschutz auf `atc-standards` verlangt Pull Request und 14 Statusprüfungen; die Prüfungen können wegen der unbestätigten Committer-Adresse nicht starten (siehe 0.1)
> **Unabhängige ATC-Testvektoren:** ERHOBEN (Abschnitt 8), erzeugt mit OpenSSL/Hashlib, nicht mit dem geprüften Code
> **Hashinventar:** PENDING
> **Code-Migration:** BLOCKED
> **Ersetzt:** ATC-HASH-001 (TownHash-256) für alle Konsensflächen
> **Datum:** 11.11.2026 · Rev. 3.1 (Rev. 2: Kodierung; Rev. 3: Merkle-Eindeutigkeit, Signaturbindung, Längengrenzen, Vektoren, Registry; **Rev. 3.1: Tiefenformel, Prüfreihenfolge vor Allokation, Signatur-Prüfordnung, ATC-HASH-001 als nicht registrierte Referenz statt als abgelöster Standard**)
> **Grundlage:** Owner-Entscheidung 11.11.2026 (SHA-256 als Konsenshashgrundlage)

---

## 0. Registry-Abgleich (verifiziert 11.11.2026)

Gelesen: `registry/standards.yaml` in `atc-standards`, main, 536 Einträge.

| Prüfung | Ergebnis |
|---|---|
| Einträge gesamt | 536 (506 `approved`, 27 `candidate`, Rest Sonderstatus) |
| Familie `ATC-HASH-*` | **keine vorhanden** |
| `ATC-HASH-001` registriert? | **nein** — die Kennung, die der Code als Standard zitiert, ist nie registriert worden |
| `ATC-HASH-002` frei? | **ja**, der gesamte Namensraum ist unbesetzt |
| Nachbarstandards | `ATC-STD-041` (Reproducible & Verifiable Builds, Hash-Verifikation), `ATC-STD-043` (Artifact Integrity, 9-Feld-Record) |

Konsequenz: Die Regel des Registers „Kein Eintrag = kein Standard" galt für TownHash-256
schon immer. Der Code hat eine Kennung erfunden, die nie ein Standard war. Dieses Dokument
schließt die Lücke, statt eine Behauptung zu erben.

Registrierung ist vorbereitet, aber nicht wirksam. Der Versuch eines direkten Commits auf
`atc-standards` wurde abgewiesen.

### 0.1 Warum die Registrierung nicht wirksam ist (verifiziert 11.11.2026)

```
PUT /repos/.../atc-standards/contents/standards/atc/candidates/ATC-HASH-002-CANDIDATE.md
HTTP 409: "Could not create file: Changes must be made through a pull request.
          14 of 14 required status checks are expected."
```

Der Branchschutz ist korrekt und wird nicht umgangen. Der Pull-Request-Weg ist ebenfalls
versperrt: Alle push-getriggerten Workflows enden wegen der unbestätigten
Committer-Adresse in `startup_failure`, damit kann keine der 14 erwarteten Prüfungen grün
werden. Ergebnis: `ATC-HASH-002` bleibt **nicht registriert**, bis die Adresse bestätigt ist.

Die vorbereitete Registerzeile (einzufügen nach `ATC-STD-NET-009`), **eine** Zeile:

```yaml
  - {id: ATC-HASH-002, title: "SHA-256 Cryptographic Hash Standard (Konsenshashgrundlage: Blockhash, Merkle, State, Receipt, Tx-ID)", version: "1.0.0", status: candidate, category: crypto, authority: "A-TownChain-Okosystems", owner: "Michael (Owner-Entwurf) / Standards Governance", normative: true, file: standards/atc/candidates/ATC-HASH-002-CANDIDATE.md, mandate: "Owner-Entscheidung 11.11.2026 (SHA-256 als Konsenshashgrundlage); Vektoren unabhaengig erhoben; Hinweis: der Code referenziert ATC-HASH-001, das nie registriert war und keine normative Autoritaet besitzt"}
```

---

## 1. Verhältnis zu ATC-HASH-001 (nie registriert, keine Ablösung)

| Feld | ATC-HASH-001 | ATC-HASH-002 |
|---|---|---|
| Verfahren | TownHash-256, eigene Merkle-Damgård-Konstruktion (512-Bit-Blöcke, 8×32-Bit-Zustand, 24 Runden, IV/K aus splitmix32, Seed `0xA7C0DE01`) | SHA-256 nach FIPS 180-4 |
| Kryptoanalyse | keine, ausdrücklich nicht kryptoanalysiert | umfangreich, jahrzehntelang öffentlich geprüft |
| Ausgabe | 32 Byte | 32 Byte |
| Konsensflächen | **verboten** | **vorgeschrieben** |
| Status | **nie registriert, kein Registereintrag, ungültige Referenz** | `candidate` |

**H-1.1 (MUSS)** ATC-HASH-001 darf nicht als normative Grundlage für Blockhashes,
Transaktions-IDs, Merkle-Wurzeln, Zustandswurzeln oder Belegwurzeln referenziert werden.

**H-1.1a (MUSS)** ATC-HASH-001 wird **nicht** als Standard geführt, auch nicht als
zurückgezogener. Es gibt keinen gültigen alten Registereintrag, der formal ersetzt werden
könnte; ein nachgetragener `retired`-Eintrag würde die Behauptung erzeugen, es habe ihn
gegeben. Die Kennung ist stattdessen an jeder Stelle, die sie nennt, als **nicht registrierte
normative Referenz** zu kennzeichnen, und die Referenz ist zu entfernen.

Grundlage: Register mit 536 Einträgen, keine `ATC-HASH`-Kennung, Regel „kein Eintrag = kein
Standard". Damit ist die Governance-Lücke benannt: Der Code hat eine normative Autorität
zitiert, die nie erteilt wurde. Das ist kein Dokumentationsfehler, sondern ein
Autoritätsverlust, der nur durch eine echte Registrierung geheilt wird.

**H-1.2 (MUSS)** Die Implementierung bleibt erhalten, gekennzeichnet als „nicht
kryptografisch validierte Eigenentwicklung". Historische Testergebnisse gelten nur für
diesen Status.

**H-1.3 (MUSS)** Devnet-Nutzung von TownHash-256 ist erlaubt, aber nur mit eigenen
Domain-Präfixen (Suffix `-D1`). Ein Devnet-Digest darf nie byte-identisch mit einem
Mainnet-Digest derselben Nutzlast sein.

---

## 2. Hashfunktion

**H-2.1 (MUSS)** SHA-256 nach FIPS 180-4: 512-Bit-Block, 32-Byte-Ausgabe, Standard-Padding.

**H-2.2 (MUSS)** Die Ausgabe ist der rohe 32-Byte-Digest. Hex ist Anzeigeformat, nie Eingabe.

**H-2.3 (MUSS) — Verbot der Texthashing-Antipattern.** Unzulässig: Hex-Darstellungen von
Hashes als Eingabe, Dezimaldarstellungen, Trennzeichen wie `|` oder `;` als Struktur,
Formatierungsregeln ohne bytegenaue Festlegung. Jede Eingabe ist eine festgelegte Bytefolge.

> Hintergrund: `components/node/src/chain.rs` bildet heute
> `format!("ATC-BLOCK-V1|{}|{}|{}", height, hex(prev_hash), payload)` und verbietet danach
> `|` und `;` in Payloads. Das ist ein Zeichenkettenformat mit nachträglichem Verbot. H-2.3
> beendet dieses Muster.

**H-2.4 (MUSS)** Keine Kürzung, keine Doppel-Hashung, kein zusätzliches Salz außerhalb der
Domain-Konvention.

---

## 3. Kanonische Eingabekodierung und Domain Separation

**H-3.1 (MUSS)**

```
input = DOMAIN_BYTES ‖ 0x00 ‖ PAYLOAD_BYTES
```

`DOMAIN_BYTES` ist ASCII ohne Längenfeld und ohne Padding, danach genau ein Byte `0x00`,
danach rohe Payload-Bytes.

**H-3.2 (MUSS)** Alle Ganzzahlen big-endian, vorzeichenlos, feste Breite (`u8`, `u16`,
`u32`, `u64`). Keine Varints, kein ZigZag.

**H-3.3 (MUSS)** Längenfelder `u32` big-endian, zählen Bytes. Siehe Abschnitt 6 für die
Grenzprüfung.

**H-3.4 (MUSS)** Domain-Strings sind unveränderlich. Strukturänderung = neue Version im
String (`-V1` → `-V2`), nicht Umdeutung des alten.

**H-3.5 (MUSS) — Register der Domain-Strings.**

| Fläche | Domain-String | ASCII | mit Terminator |
|---|---|---|---|
| Blockhash | `ATC-BLOCK-V2` | 12 | 13 |
| Transaktions-ID | `ATC-TXID-V1` | 11 | 12 |
| Merkle-Blatt | `ATC-MERKLE-L-V1` | 15 | 16 |
| Merkle-Knoten | `ATC-MERKLE-N-V1` | 15 | 16 |
| Merkle leer | `ATC-MERKLE-EMPTY-V1` | 19 | 20 |
| Zustands-Blatt | `ATC-STATE-V1` | 12 | 13 |
| Zustands-Knoten | `ATC-STATE-N-V1` | 14 | 15 |
| Zustand leer | `ATC-STATE-EMPTY-V1` | 17 | 18 |
| Beleg | `ATC-RECEIPT-V1` | 14 | 15 |

**H-3.6 (MUSS)** Ein Domain-String gehört genau einer Fläche und wird nie wiederverwendet.

**H-3.7 (MUSS)** Devnet mit TownHash-256 nutzt dieselben Strukturen mit Suffix `-D1`
(`ATC-BLOCK-D1`, `ATC-MERKLE-L-D1`, …).

---

## 4. Blockhash

**H-4.1 (MUSS) — Byte-Layout.** Die Signatur ist **nicht** Teil der Hash-Eingabe.

| Offset | Feld | Breite | Kodierung |
|---|---|---|---|
| 0 | `version` | 2 | u16 BE |
| 2 | `chain_id` | 8 | u64 BE |
| 10 | `height` | 8 | u64 BE, Genesis = 0 |
| 18 | `prev_hash` | 32 | rohe Bytes |
| 50 | `tx_root` | 32 | rohe Bytes |
| 82 | `state_root` | 32 | rohe Bytes |
| 114 | `receipt_root` | 32 | rohe Bytes |
| 146 | `gas_used` | 8 | u64 BE |
| 154 | `timestamp` | 8 | u64 BE, Zeitbasis nach ATC-10 |
| 162 | `proposer` | 33 | SEC1 komprimiert, secp256k1 |
| 195 | **Summe** | **195** | ohne Signatur |

**H-4.2 (MUSS)** `block_hash = SHA256("ATC-BLOCK-V2" ‖ 0x00 ‖ header[0..195])`, Eingabe
208 Byte.

**H-4.3 (MUSS)** Genesis hat `height = 0` und dasselbe Layout. Kein Sonderweg.

**H-4.4 (MUSS)** `tx_root`, `state_root`, `receipt_root` sind vor dem Hashen berechnet. Der
Blockhash bindet damit die Ausführung.

### Arithmetische Nachrechnung (unabhängig, 11.11.2026)

| Feld | Breite | Zwischensumme |
|---|---|---|
| version | 2 | 2 |
| chain_id | 8 | 10 |
| height | 8 | 18 |
| prev_hash | 32 | 50 |
| tx_root | 32 | 82 |
| state_root | 32 | 114 |
| receipt_root | 32 | 146 |
| gas_used | 8 | 154 |
| timestamp | 8 | 162 |
| proposer | 33 | **195** |

Domain-Präfix mit Terminator: 13 Byte. Blockhash-Eingabe: 13 + 195 = **208 Byte**. Beides
durch ein Skript nachgerechnet, das die Feldbreiten addiert und die erzeugte Bytefolge auf
Länge prüft (`assert len(header) == 195`).

---

## 5. Signaturbindung (neu in Rev. 3)

Die Signatur liegt außerhalb der Hash-Eingabe. Damit sie dennoch eindeutig an den
Blockinhalt gebunden ist, gilt:

**H-5.1 (MUSS) — Algorithmus.** ECDSA über secp256k1, SHA-256, wie im Krypto-Vertrag des
Projekts (Transaction/Account-Protokoll). Kein Ed25519 für Block-Proposals.

**H-5.2 (MUSS) — Signierte Bytefolge.** Signiert wird **genau** `block_hash`, die 32 Byte
aus H-4.2. Kein zusätzliches Präfix, kein zweiter Hash, kein Umschließen in eine
Nachrichtenstruktur. Ein Wrapper wäre eine zweite gültige Signaturform für denselben Block
und damit eine Ambiguität.

**H-5.3 (MUSS)** Die Signatur ist `r ‖ s`, je 32 Byte, 64 Byte gesamt, big-endian.

**H-5.4 (MUSS) — Kanonizität.** `s` muss in der unteren Hälfte der Gruppenordnung liegen
(Low-S, `s ≤ n/2`). High-S-Signaturen werden **abgelehnt**, nicht normalisiert. `r = 0` und
`s = 0` werden abgelehnt. DER-Kodierung wird abgelehnt. Die Nonce-Erzeugung ist
deterministisch nach RFC 6979; abweichende Nonce-Verfahren sind für Proposals unzulässig.

**H-5.5 (MUSS) — Proposer-Identität.** `proposer` im Header ist der 33-Byte SEC1-
komprimierte Konsensschlüssel. Gültig ist ein Proposal nur, wenn dieser Schlüssel im
aktiven Validator-Set steht und die Signatur gegen ihn verifiziert. Der Konsensschlüssel ist
**nicht** der Transaktionsschlüssel: Purpose Separation ist Pflicht (Transaction Key ≠
Consensus Key ≠ Node Identity ≠ Service Identity).

**H-5.6 (MUSS)** Die Signatur wird gegen den **neu berechneten** Blockhash geprüft, nicht
gegen einen mitgelieferten Wert. Weicht der mitgelieferte Hash ab, wird der Block abgelehnt.

**H-5.7 (MUSS)** Ein Block mit ungültiger Signatur wird verworfen und gezählt, nicht
repariert. Eine Signatur, die nur nach Normalisierung gültig wäre, gilt als ungültig.

**H-5.8 (MUSS) — Prüfordnung, fail-closed.** Ein eingehender Block wird in genau dieser
Reihenfolge geprüft; jeder Schritt darf ablehnen, keiner darf einen späteren vorwegnehmen
oder einen früheren überspringen:

| # | Prüfung | Ablehnung wenn |
|---|---|---|
| 1 | Längen | Header nicht exakt 195 Byte, Signatur nicht exakt 64 Byte |
| 2 | `version`, `chain_id` | unbekannt oder fremdes Netz |
| 3 | Höhe und `prev_hash` | nicht `best.height + 1` oder Vorgänger unbekannt |
| 4 | Wurzeln nachrechnen | `tx_root`, `receipt_root`, `state_root` weichen ab |
| 5 | Blockhash neu berechnen | mitgelieferter Hash ≠ neu berechnet |
| 6 | Proposer im Validator-Set | Schlüssel unbekannt, inaktiv oder nicht eindeutig zugeordnet |
| 7 | Kanonizität der Signatur | High-S, `r = 0`, `s = 0`, DER oder abweichende Kodierung |
| 8 | Signatur gegen den **neu berechneten** Hash | Verifikation schlägt fehl |

**H-5.9 (MUSS)** Der Blockhash ist die **einzige** Größe, die signiert wird. Es gibt keinen
zweiten signierten Gegenstand, der denselben Block autorisiert, und keinen Weg, die Signatur
auf einen anderen Header zu übertragen: jede Änderung eines Headerfeldes ändert den Hash und
damit die Signaturbasis.

**H-5.10 (MUSS)** Die Proposer-Zuordnung ist eindeutig: ein Konsensschlüssel gehört genau
einem Validator, und ein Validator hat genau einen aktiven Konsensschlüssel. Doppelte
Zuordnungen machen das Validator-Set ungültig, nicht den Block reparierbar.

---

## 6. Längen, Grenzen, Überläufe (neu in Rev. 3)

**H-6.1 (MUSS)** Jede Länge wird **vor** der Serialisierung geprüft. Überschreitet ein
Feld `u32::MAX`, wird die Operation abgelehnt, nicht gekürzt.

**H-6.2 (MUSS)** Es gibt keine Wraparound-Arithmetik in Kodierung oder Längenrechnung. Alle
Additionen von Längen und Offsets sind geprüft (`checked_add`, `checked_mul`), Fehler führen
zur Ablehnung.

**H-6.3 (MUSS)** Kein impliziter Cast zwischen `usize` und `u32`/`u64`. Die Kodierung
verwendet ausschließlich feste Breiten, damit das Ergebnis nicht von der Wortbreite der
Plattform abhängt. `usize`-Werte werden mit expliziter Bereichsprüfung übernommen.

**H-6.4 (MUSS) — Strukturgrenzen.**

| Größe | Grenze | Ablehnung bei Überschreitung |
|---|---|---|
| Transaktion (kanonisch) | 1 048 576 Byte | vor dem Hashen |
| Payload einer Transaktion | 1 048 576 Byte | vor dem Hashen |
| Transaktionen pro Block | 1 048 576 (2²⁰) | vor dem Bau des Merkle-Baums |
| Block-Payload gesamt | 4 194 304 Byte | vor dem Hashen |
| Zustandseinträge je Zustandswurzel | 2³² − 1 | durch `u32`-Zählung begrenzt |

**H-6.5 (MUSS)** `gas_used` wird mit geprüfter Arithmetik summiert. Ein Überlauf macht den
Block ungültig; er wird nicht abgeschnitten.

**H-6.6 (MUSS)** Grenzfälle sind zu testen: Länge 0, 1, `u32::MAX − 1`, `u32::MAX`,
`u32::MAX + 1` (muss abgelehnt werden). Dieselben Fälle auf 32- und 64-Bit-Zielen, mit
identischem Ergebnis.

---

## 7. Merkle-Baum: Eindeutigkeit (überarbeitet in Rev. 3)

**H-7.1 (MUSS)** Blatt: `H("ATC-MERKLE-L-V1" ‖ 0x00 ‖ tx_id)`.
Knoten: `H("ATC-MERKLE-N-V1" ‖ 0x00 ‖ left ‖ right)`.

**H-7.2 (MUSS)** Getrennte Domains für Blatt und Knoten. Eine Implementierung mit einer
gemeinsamen Domain ist nicht konform.

**H-7.3 (MUSS) — Leere Menge.** `tx_root = H("ATC-MERKLE-EMPTY-V1" ‖ 0x00)`.

> **Änderung gegenüber Rev. 2.** Rev. 2 legte 32 Nullbytes fest. Das ist ein Wert ohne
> Domain und nicht strukturell von einem gültigen Hash unterschieden; die Trennung hing an
> der unbelegten Annahme, dass SHA-256 nie null wird. Mit eigener Domain ist die leere Menge
> ein eigener Fall im Namensraum, und kein Ein-Knoten-Baum kann sie erzeugen.

**H-7.4 (MUSS) — Einzelnes Blatt.** Bei genau einem Blatt ist die Wurzel das Blatt selbst
(`root = leaf_0`). Es wird **nicht** erneut gehasht und nicht mit einem leeren Partner
verknüpft.

**H-7.5 (MUSS) — Ungerade Knotenzahl.** Der letzte Knoten einer Ebene wird unverändert in
die nächste Ebene übernommen, nicht dupliziert. Beleg: `(a,b,c)` ergibt eine andere Wurzel
als `(a,b,c,c)` (Abschnitt 8, Vektoren M3 und M4).

**H-7.6 (MUSS) — Reihenfolge.** Die Blattreihenfolge ist die **Blockreihenfolge** der
Transaktionen, wie sie in der Blockauswahl festgelegt wurde. Die Wurzel ist
reihenfolgeabhängig; eine Sortierung nach ID ist unzulässig.

**H-7.7 (MUSS) — Maximale Blattzahl und Tiefe, exakt bezogen.** Die Grenze ist die
**Blattzahl**, nicht die Tiefe:

| Blätter `n` | Tiefe (= Ebenen über den Blättern) |
|---|---|
| 0 | keine (Leerwurzel nach H-7.3) |
| 1 | 0 (Wurzel ist das Blatt) |
| 2 | 1 |
| 3 | 2 |
| 4 | 2 |
| 5 … 8 | 3 |
| `n` | `ceil(log2(n))` für `n ≥ 2` |
| 2²⁰ (1 048 576) | **20 (Obergrenze)** |

**H-7.7a (MUSS)** Die Tiefe ist eine **Folge** der Blattzahl, kein eigenständiger Parameter.
Wer die Tiefe begrenzt, ohne die Blattzahl zu begrenzen, hat nichts begrenzt. Umgekehrt gilt:
`n ≤ 2²⁰` ist die einzige normative Grenze, Tiefe 20 ist ihre Rechnung.

**H-7.7b (MUSS) — Prüfung vor der Allokation.** Die Reihenfolge ist verbindlich:

1. Blattzahl gegen `2²⁰` prüfen (Ablehnung, bevor irgendein Puffer wächst).
2. Summe der Payload-Längen gegen 4 MiB prüfen.
3. Erst danach Puffer für die nächste Ebene allokieren, größenbeschränkt auf `ceil(n/2)`.
4. Bei jeder Ebene erneut prüfen, dass die Zahl der Knoten nicht größer wird als die der
   vorigen (Invariante gegen Endlosschleifen und gegen Aufblähung durch fehlerhafte
   Hochzieh-Logik).

Eine Implementierung, die zuerst allokiert und danach prüft, ist nicht konform, auch wenn sie
das richtige Ergebnis liefert.

**H-7.8 (MUSS) — Darstellung der Wurzel.** 32 rohe Bytes. Hex nur zur Anzeige. Die Wurzel
ist eine Bytefolge, keine Zahl; es gibt keine Big-Endian-Interpretation der Wurzel selbst.

**H-7.9 (MUSS)** Für den Zustand gilt dieselbe Konstruktion mit `ATC-STATE-V1` (Blatt),
`ATC-STATE-N-V1` (Knoten) und `ATC-STATE-EMPTY-V1` (leer), zusätzlich sortiert nach
`key_bytes`.

---

## 8. Testvektoren (unabhängig erzeugt, 11.11.2026)

**Herkunft:** `python3` mit `hashlib.sha256` (OpenSSL-Backend), also **nicht** die geprüfte
Rust-Implementierung. Die Referenz wurde zuerst gegen alle vier FIPS-180-4-Vektoren
selbst geprüft; alle vier trafen. Erzeugendes Skript: `ATC-HASH-002-VECTORS.py` neben diesem
Dokument. Reproduzierbar mit `python3 ATC-HASH-002-VECTORS.py`.

### 8.1 FIPS-Grundvektoren (Selbsttest der Referenz)

| Eingabe | Digest |
|---|---|
| `""` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `"abc"` | `ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad` |
| 56-Byte-Nachricht `abcdbcde…nopq` | `248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1` |
| `"a"` × 1 000 000 | `cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0` |

### 8.2 Leere Payload je Domain

Eingabe jeweils `DOMAIN ‖ 0x00`.

| Domain | Erwarteter Digest |
|---|---|
| `ATC-BLOCK-V2` | `42cdfa6b8b91f73ca3a06dd1abbb62de85043c21ee5d9d93b44cce883117a816` |
| `ATC-TXID-V1` | `94cc7102a4bbc4d45a9201410c31bb22991cf121b4982cd54af47ec4b8d474ef` |
| `ATC-MERKLE-L-V1` | `d879922658c0a9d5d9474704e0825d8f08e2a8f9fd85a7a45cab34345b69dd2c` |
| `ATC-MERKLE-N-V1` | `7da650d4c8485e6fd19e04c920465710ab1be293cc8548d268be47f87daf4083` |
| `ATC-STATE-V1` | `bd18bf6cfbe3b5b172e3b134a2bc638312dda1c675fa60e95fe94eeb823edd15` |
| `ATC-STATE-N-V1` | `e64f4929918c8e7dd7aceef28973d38cb63405bbad6b1eca717bb05d9ad2e967` |
| `ATC-RECEIPT-V1` | `f33c851e58ace00cb2cafa70164d69fff8f92031ade3100e8628180bb2d6edcd` |
| `ATC-MERKLE-EMPTY-V1` | `6fd0aec3f13b64fb51cefa23755f766a86f519a8dd63f85fd88247b41d0827f3` |
| `ATC-STATE-EMPTY-V1` | `51f8d7e3912a7e7c7627d5a684181aab813f5bac17f382e9df29a1371ea37502` |

Gegenprobe: alle neun Digests sind verschieden. Damit ist belegt, dass ein Domain-String
allein schon die Ausgabe trennt.

### 8.3 Blockhash (Vollheader)

Proposer in beiden Fällen: komprimierter secp256k1-Generator
`0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798`.

**B1 (Höhe 1):** `version=1`, `chain_id=658467`, `height=1`,
`prev_hash=0x11`×32, `tx_root=0x22`×32, `state_root=0x33`×32, `receipt_root=0x44`×32,
`gas_used=21000`, `timestamp=1762900000`.

Digest: `b4269ae01991e6fde8c59e82a578fdcd00faf16f53dfa908ba7d196d700473e8`

**B0 (Genesis):** `version=1`, `chain_id=658467`, `height=0`, alle drei Wurzeln und
`prev_hash` = 32 Nullbytes, `gas_used=0`, `timestamp=0`.

Digest: `037a009df8c0ef114a9c68de1e9e41052078ce994eb814c2ca96da1cc32d09dc`

### 8.4 Transaktions-ID und Beleg

**T1:** Payload = `deadbeef` ‖ `u64 BE 658467` ‖ `u64 BE 1` (Testeingabe, keine reale
Transaktion).
Digest: `357f651c3f616d5395119acdd12053dc7f1c6885b7453362be865c4350e2f1e5`

**R1:** `height=1`, `index=0`, `status=0`, `gas_used=21000`, `tx_id=0xAA`×32.
Digest: `7e14e3851a4b1872e3087563574ca7e2462ca2313a1a23971fb9d76678670a59`

### 8.5 Merkle

Transaktions-IDs: `A=0xAA`×32, `B=0xBB`×32, `C=0xCC`×32.

| Fall | Konstruktion | Wurzel |
|---|---|---|
| M0 leer | `H(ATC-MERKLE-EMPTY-V1 ‖ 0x00)` | `6fd0aec3f13b64fb51cefa23755f766a86f519a8dd63f85fd88247b41d0827f3` |
| M1 ein Blatt | `leaf(A)` | `890a527ef31339b598709afb059c263821b852a81574dee6520455c984916c52` |
| M2 zwei Blätter | `node(leaf A, leaf B)` | `ffc66c9248c7729d368d4b1b4612cc2d5b43aa6eef2346a87c8a03faca709393` |
| M3 drei, hochziehen | `node(node(A,B), leaf C)` | `004253529731a9720cb7f2a1a933fe0acf165b0a8bd8fbd20fc4be21a9cc161d` |
| M4 vier, dupliziert | `node(node(A,B), node(C,C))` | `ae9e60e0d691b1ec47c6718118d963b32a5cf0cf63964f52638c67d12063c2d0` |

M3 ≠ M4: die Regel „hochziehen" ist damit nicht nur behauptet, sondern belegt. M1 ≠ M0:
ein Ein-Knoten-Baum erzeugt nie die leere Wurzel.

### 8.6 Zustand

Einträge `alice → "50"`, `bob → "100"`.

| Fall | Wurzel |
|---|---|
| zwei Einträge, sortiert nach `key_bytes` | `02af7debe4171f10d83046f7517fa7f341233fde1f37c2b6eebc3ba13b84539e` |
| gleiche Einträge in anderer Einfügereihenfolge | `02af7debe4171f10d83046f7517fa7f341233fde1f37c2b6eebc3ba13b84539e` |
| leer | `51f8d7e3912a7e7c7627d5a684181aab813f5bac17f382e9df29a1371ea37502` |

### 8.8 Tiefe und Grenzprüfung (Rev. 3.1, unabhängig gerechnet)

Ausgabe des Skripts, Abschnitt `checks_rev31()`:

| `n` Blätter | Tiefe |
|---|---|
| 0 | Leerwurzel (eigener Fall) |
| 1 | 0 |
| 2 | 1 |
| 3, 4 | 2 |
| 5 … 8 | 3 |
| 9 … 12 | 4 |
| 1 048 576 | **20** |

Die Hochzieh-Konstruktion wurde über alle Blattzahlen 1 bis 12 durchgerechnet; jede Ebene
schrumpft, jede Wurzel ist 32 Byte, kein Fall läuft über. Die Grenzprüfung vor der Allokation
verhält sich an der Kante wie festgelegt:

| Blätter | Payload | Ergebnis |
|---|---|---|
| 1 048 576 | 0 | akzeptiert |
| 1 048 577 | 0 | abgelehnt, `ADM-LEN-01` |
| 1 | 4 194 304 | akzeptiert |
| 1 | 4 194 305 | abgelehnt, `ADM-LEN-02` |

Damit ist die Tiefengrenze nicht nur behauptet, sondern als Folge der Blattzahl belegt: die
Prüfung sitzt **vor** der Allokation und lehnt genau an der Kante ab.

### 8.7 Grenzlängen

Für die Padding-Kanten sind zusätzlich die Längen 0, 1, 55, 56, 57, 63, 64, 65, 119, 120,
121 zu prüfen, je Domain. Die Werte werden mit demselben Skript erzeugt; sie sind im Skript
ausgerechnet und im Test als Literale abzulegen.

---

## 9. Versionierung und Migration

**H-9.1 (MUSS)** Ein Wechsel von Hashfunktion oder Eingabestruktur ist eine
Protokollaktivierung mit Höhe und Zeitpunkt, kein Softwareupdate.

**H-9.2 (MUSS)** Neue Versionsnummer im Domain-String, alte Strings bleiben für historische
Blöcke lesbar.

**H-9.3 (MUSS)** TownHash- und SHA-256-Zustandswurzeln sind nicht ineinander überführbar. Ein
Wechsel bedeutet neue Genesis. Devnet-Zustand wird nicht Mainnet-Zustand.

**H-9.4 (MUSS)** Kein Parallelbetrieb beider Flächen für dieselbe Rolle.

**H-9.5 (SOLL)** Migration in eigenem Dokument mit Aktivierungshöhe, Kompatibilitätsmatrix,
Rückfallpfad.

---

## 10. Verbote (Kurzliste)

- Kein TownHash-256 auf einer Konsensfläche, auch nicht im Release-Kandidaten.
- Kein Hashen von Hex-Strings, Dezimalzahlen oder formatierter Ausgabe.
- Keine Merkle-Konstruktion außerhalb von Abschnitt 7.
- Keine Wiederverwendung eines Domain-Strings.
- Keine Signatur in der Hash-Eingabe; keine zweite Signaturform über Wrapper.
- Keine Normalisierung nicht-kanonischer Signaturen.
- Keine ungeprüfte Längenrechnung, keine `usize`-abhängige Kodierung.
- Keine Sollwerte, die aus dem geprüften Code stammen.

---

## 11. Abnahmeliste

| Punkt | Nachweis |
|---|---|
| SHA-256 gegen FIPS | 4 Grundvektoren plus Grenzlängen, extern erzeugt |
| Domain-Register vollständig | je Fläche ein Vektor, Abschnitt 8.2 |
| Blatt/Knoten-Trennung | 8.2 und 8.5, Rollenwechsel ergibt anderen Digest |
| Merkle ungerade Zahl | M3 gegen M4 |
| Leere Menge ≠ Ein-Knoten-Baum | M0 gegen M1 |
| Wurzeldarstellung | 32 Byte, keine Zahlinterpretation |
| Header-Länge | 195 Byte, Eingabe 208 Byte, per `assert` geprüft |
| Signaturbindung | Verifikation gegen neu berechneten Hash, High-S abgelehnt |
| Längengrenzen | 0, 1, u32::MAX−1, u32::MAX, u32::MAX+1 |
| Kein TownHash im Pfad | Suche im Artefakt plus Test, der bei TownHash-Nutzung fehlschlägt |

---

## 12. Governancestatus

```
ATC-HASH-002
  CANDIDATE
  Normative Registrierung: BLOCKED (Branchschutz + 14 Checks, CI tot)
  Unabhängige ATC-Testvektoren: ERHOBEN (Abschnitt 8)
  Grenzlängen-Vektoren: PENDING
  Hashinventar: PENDING
  Code-Migration: BLOCKED
  Implementierungsfreigabe: BLOCKED

ATC-HASH-001
  NICHT REGISTRIERT, keine normative Autorität
  Referenz im Code = ungültig, zu entfernen
  Für Konsensflächen ausgeschlossen
  Implementierung unter Bestandsschutz, gekennzeichnet
```

---

## 13. Offene Punkte

1. Grenzlängen-Vektoren als Literale in den Test aufnehmen (Skript vorhanden).
2. Hashinventar über das Ökosystem (Blockhash, Transaktions-ID, Merkle, State, Receipt, Wallet, VM, PoH, Build-Kette).
3. Beleg-Merkle-Ebene: eigene Domains registrieren, falls Belege nicht mit den Tx-Domains gebaut werden.
4. `version`-Feld: Startwert und Verhalten bei unbekannter Version (Vorschlag: Ablehnung).
5. `timestamp`-Zeitbasis nach ATC-10.
6. Devnet-Präfixliste (`-D1`) endgültig festhalten.
7. Aktivierungshöhe und Datum, nach Implementierung.

---

## 14. Was dieses Dokument nicht ist

Kein ratifizierter Standard, keine Implementierung, kein Nachweis über Code. Es legt Bytes
fest, damit vier Komponenten später nicht drei Meinungen über dieselben Bytes haben.

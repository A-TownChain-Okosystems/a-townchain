# D-0 entschieden und umgesetzt: `ATCE`/`0x02` als aeussere Artefakt-Huelle

> **Status:** Entscheidung `DECIDED` (Owner 12.11.2026, Option 1).
> Schicht implementiert und lokal geprueft. `ATCE/0x02` bleibt **`OPEN`**, weil
> die Integration in den System-Deploypfad und die unabhaengige Pruefung fehlen.
> Differentialtest bleibt **`BLOCKED`** (Owner-Vorgabe bis zur vollstaendigen
> Integration und Pruefung), obwohl ein lokaler Lauf 40 von 40 Eingaben deckt.
> `CI_VERIFIED` ist nicht vergeben. **Datum:** 2026-11-12.

## 1 Die Entscheidung

Es bleiben zwei Formate mit getrennten Verantwortlichkeiten. Kandidat und Korpus
bleiben maszgeblich; sie werden **nicht** nachtraeglich auf `ATCB`/`0x01`
umgeschrieben.

| Bestandteil | Magic | Fassung | Verantwortung |
|---|---|---|---|
| ATC-VM-Bytecode | `ATCB` | `0x01` | VM-Ausfuehrungsformat |
| ATC-Artifact-Huelle | `ATCE` | `0x02` | aeusseres Artefaktformat |

Die Huelle veraendert weder Opcodes noch Bytecode-Semantik. Der Bytecode-Hash
bleibt unveraendert; die Huelle hat einen eigenen Digest. Kein Parser akzeptiert
das Magic des anderen (Abschnitt 4).

## 2 Umgesetzt

| Datei | Blob | Inhalt |
|---|---|---|
| `components/vm/src/envelope.rs` | `b253ffc3` | Huelle: Kopf, Fassung, Generation, Strom, Tail; Stufen 1 bis 10; `EnvelopeError` als `(Code, Offset)`; `from_artifact`, `into_artifact`, `to_bytes`, `artifact_hash`, `parse_with_limit` |
| `components/vm/src/artifact.rs` | `122bf399` | `StromFehler` und `dekodiere_strom` als **gemeinsame** Stromprozedur; `ATCB` bildet die Fehler auf seinen Typ ab |
| `components/vm/src/lib.rs` | `cb1d135d` | `pub mod envelope` |
| `components/vm/tests/atce_corpus.rs` | `b8d9edea` | Differentialtest der Huelle gegen den Korpus |
| `components/vm/tests/stage1_parser_corpus.rs` | `5dc33bbd` | erzeugtes Fixture (Bytes und Sollwerte, kein JSON-Parser) |
| `tools/build_parser_corpus.py` | `244809ee` | erzeugt **und** prueft das Fixture gegen den Korpus |

Commits (je Datei einer, Contents-API): `1309b622` (`artifact.rs`),
`a0ea77f3` (`envelope.rs`), `e1711c83` (`lib.rs`), `b7d0bcce` (Korpus-Fixture),
`5e0165a3` (Differentialtest), `926c59d4` (`build_parser_corpus.py`).

## 3 Keine doppelte VM-Logik

Der Strom wird genau einmal dekodiert. `dekodiere_strom` in `artifact.rs` enthaelt
Opcode-Tabelle, Operandenbreiten, die Anzahlpruefung, die Pruefung auf
ueberzaehlige Bytes und den leeren Strom; beide Hüllen rufen sie auf. Die Huelle
ergaenzt ausschliesslich ihr eigenes Magic, ihre Fassung, die Generation, den
Tail und die Stufenreihenfolge.

Nachweis der Verhaltensgleichheit: der Umbau hat die `ATCB`-Tests nicht veraendert,
die Suite blieb gruen (Abschnitt 6), und der fixierte Bytecode-Hash ist unveraendert.

## 4 Beide Richtungen sind getrennt

| Eingabe | `Envelope::parse` | `Artifact::parse` |
|---|---|---|
| `ATCE`/`0x02` | akzeptiert | `E-007` bei 0 |
| `ATCB`/`0x01` | `E-007` bei 0 | akzeptiert |

Es gibt keinen Fallback und kein spekulatives Parsen. Die Ausfuehrung laeuft
weiterhin ausschliesslich ueber `Artifact`/`Vm`; die Huelle fuehrt nichts selbst
aus, sie uebergibt den Strom mit `into_artifact()`.

## 5 Die verpflichtenden Tests

| Test (Owner-Vorgabe) | Erwartung | Ergebnis |
|---|---|---|
| `ATCE/0x02` mit `ATCB/0x01` | akzeptiert | `atce_mit_atcb_wird_akzeptiert`: Huelle gelesen, 4 Instruktionen, `into_artifact()` liefert dasselbe Programm und denselben Bytecode-Hash |
| `ATCE` mit falscher Fassung | abgelehnt | `falsche_fassung_abgelehnt`: `0x01`, `0x03`, `0xFF` → `E-003` bei 4 |
| `ATCB` ohne gueltige Huelle | gemaess Contract abgelehnt | `atcb_ohne_huelle_wird_abgelehnt`: `E-007` bei 0 in **beiden** Richtungen |
| manipulierte Integritaet | abgelehnt | `manipulierter_digest_abgelehnt`: `E-006` bei `tail_start`, auch bei manipuliertem Strom |
| ungueltige Laenge | abgelehnt | `ungueltige_laenge_abgelehnt`: `0`, `1`, `41` Byte → `E-001`; 42 Byte mit Anzahl 0 → `E-004` |
| trunkierter Strom | abgelehnt | `trunkierter_strom_abgelehnt`: `E-002` bzw. `E-005`, je bei `tail_start` |
| unbekannte Opcodes | deterministischer Fehler | `unbekannter_opcode_ist_deterministisch`: `E-010` bei 10, zwei Laeufe identisch |
| bestehende Bytecode-Hashes | unveraendert | `bytecode_hash_bleibt_unveraendert`: Fixwert `0dee1036…` bleibt, Huelle und Bytecode haben verschiedene Digests, `into_artifact()` erzeugt bytegleiche `ATCB`-Bytes |

Zusaetzlich: `domaene_geht_in_den_digest_ein` (Domaene `ATC-ARTIFACT-V1\0`,
16 Byte, geht in den Hash ein), `groesstwert_nur_mit_grenze` (Stufe 10 an der
Kante: Stromlaenge akzeptiert, ein Byte darunter `E-012` bei 10),
`generation_wird_aus_der_huelle_uebernommen`, `unbekannte_generation_abgelehnt`.

## 6 Differentialtest: zwei unabhaengige Implementierungen

`components/vm/tests/atce_corpus.rs` fuehrt alle 40 Eingaben des Korpus
(`C-1` bis `C-8`) durch die Rust-Huelle und vergleicht `(Code, Offset)` mit den
Sollwerten, die der unabhaengige Python-Referenzdekoder erzeugt hat.

| Pruefung | Ergebnis |
|---|---|
| `cargo test -p atc-vm --test atce_corpus` | **3 passed, 0 failed**: 40 von 40 Eingaben gleich, Klassen C-1..C-8 vollstaendig, mindestens 30 Eingaben |
| `python3 tools/check_parser_corpus.py` | PASS, alle Zeilen vom Referenzdekoder bestaetigt |
| `python3 tools/build_parser_corpus.py --check` | PASS, Korpus und Rust-Fixture unveraendert |

Damit liegen erstmals **zwei** Implementierungen desselben Korpus mit identischem
Ergebnis vor. Das ist der Sachstand, den REQ-SCE-084 verlangt; die *Statusfreigabe*
bleibt trotzdem `BLOCKED`, weil die vollstaendige Integration fehlt und eine
unabhaengige Pruefung nicht stattgefunden hat.

## 7 Testevidenz

Ausgefuehrt im Arbeitsbaum auf `926c59d4` mit den Blobs aus Abschnitt 2,
`cargo 1.99.1`, offline.

| Pruefung | Ergebnis |
|---|---|
| `cargo test --workspace --offline` | **699 passed, 0 failed**, exit 0 (vorher 684) |
| `cargo test -p atc-vm --offline` | **169 passed, 0 failed** (vorher 153): 56 lib, 45 bin, 26 `atcb_v2`, 3 `atce_corpus`, 12 `exec_chain_e2e`, 27 `gas_table_v1` |
| `cargo fmt --check` (geaenderte Dateien) | ohne Abweichung |
| `cargo clippy -p atc-vm --lib` | keine Warnung |
| `python3 -m pytest tools/tests` | 28 passed |
| `python3 tools/decision_register.py --check` | PASS, 8 Entscheidungen in 2 Dokumenten |

## 8 Statusgrenzen (Owner-Vorgabe 12.11.2026)

| Gegenstand | Status |
|---|---|
| D-0 Formatentscheidung | `DECIDED` |
| D1-B, D1-C | `IMPLEMENTED` |
| `ATCE/0x02` | `OPEN` — Schicht vorhanden, Integration fehlt |
| Differentialtest | `BLOCKED` — lokaler Lauf 40/40, Integration und Pruefung offen |
| `CI_VERIFIED` | nicht vergeben, kein CI-Lauf am HEAD |
| Konformitaet des Kandidaten | unberuehrt; keine Ratifikation, keine Releasefreigabe |
| P1a | `OPEN` |
| P2 | `BLOCKED` durch B-1 |

## 9 Offene Punkte

1. **Integration:** der System-Deploypfad nimmt heute `ATCB`-Bytes
   (`System::…` → `Artifact::parse`). Die Huelle ist noch nicht vorgeschaltet;
   solange das so ist, ist `ATCE/0x02` nicht der Artefaktpfad des Systems.
2. **Unabhaengige Pruefung** der Huelle durch eine zweite Person oder ein zweites
   Werkzeug ausserhalb dieses Repositories; der Differentialtest belegt nur
   Uebereinstimmung zweier Implementierungen, nicht Korrektheit der Spezifikation.
3. **`E-012`-Betriebswert** (Stromlaenge in Byte) bleibt offen, P1b.
4. **D1-A** Container-Bytegrenze vor dem Hashing bleibt offen, P1b.

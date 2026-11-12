# D1-B und D1-C — Umsetzung und Nachweis am VM-Parser

> **Status:** `TESTED` (lokale Suite am genannten Commit). `CI_VERIFIED` offen:
> es gibt keinen CI-Lauf am exakten Commit. `VERIFIED` nicht erreicht.
> **Datum:** 2026-11-12 · **Freigabe:** Owner, Umfang D1-B und D1-C
> **Nicht enthalten:** D1-A (Bytegrenze), P1b, B-1, P2, Ratifikation des
> Kandidaten, Gaswerte, Huelle (siehe Abschnitt 8).

## 1 Auftrag

Freigegeben war ein begrenztes Paket am VM-Parser:

1. `E-004` und `E-005` in den Fehlervertrag aufnehmen.
2. Parserstruktur vor der Digest-Pruefung ausfuehren.
3. Die deklarierte Anzahl `c` nicht mehr als Ersatz fuer die dekodierte
   Programmlaenge verwenden.
4. Die bekannten Abweichungen einzeln pruefen.
5. Bestehende Parser- und VM-Tests regressionssicher erhalten.
6. Keine Aenderung an Container-Bytegrenzen, Konsensparametern, B-1, P2 oder der
   Ratifikation.

Die zusaetzlichen Bedingungen des Owners vom 12.11.2026 (Magic bleibt `ATCB`,
Format `0x01` normativ, keine Aenderung ausserhalb D1-B/D1-C, Formatierung und
Builds gruen, nachvollziehbare Commits) sind eingehalten; Abschnitt 4 belegt das
Format unveraendert.

## 2 Die vier Abweichungen: vorher und nachher

| # | Abweichung vorher | Stand nach diesem Paket |
|---|---|---|
| 1 | Container-Magic `ATCB` / Format `0x01` galt als Abweichung zur Huelle `ATCE` / `0x02` | **Bleibt `ATCB` / `0x01`.** Owner-Bedingung; die Huelle ist nicht Teil dieses Pakets (Abschnitt 8, D-0) |
| 2 | `ArtifactError` kannte `E-004` und `E-005` nicht (10 von 12 Codes) | vollstaendig: `EmptyStream` (E-004), `CountMismatch` (E-005) |
| 3 | Digest wurde vor der Struktur geprueft (`artifact.rs:311-319` alt) | Struktur Stufen 5-8 **vor** Stufe 9; bei zwei Defekten gewinnt der Strukturcode |
| 4 | `c > MAX_PROGRAM_OPS` direkt nach dem Digest (deklarierte Anzahl) | Groessenpruefung am Ende auf der **dekodierten** Laenge; `c` ist nur Obergrenze |

## 3 Geaenderte Dateien

| Datei | Blob | Aenderung |
|---|---|---|
| `components/vm/src/artifact.rs` | `75e881e4` | Fehlervertrag mit Codes und Offsets, neue Pruefreihenfolge, Stromgrenze `tail_start`, Groessenpruefung auf dekodierter Laenge, 11 neue Tests |
| `components/system/src/system.rs` | `c9f1da42` | `ArtifactError::BadHash` traegt den Offset (`HASH_LEN`), zwei Zeilen |

Commits (Reihenfolge, je Datei ein Commit ueber die Contents-API):

| Commit | Datei |
|---|---|
| `c9ba30c2` | `components/vm/src/artifact.rs` — `fix(vm): D1-B und D1-C …` |
| `6a164080` | `components/system/src/system.rs` — `fix(system): ArtifactError::BadHash traegt den Offset` |

`Cargo.lock` wurde nach jedem Testlauf zurueckgesetzt; der Lock ist unveraendert
im Repository. Kein weiteres Paket wurde angefasst.

## 4 Format unveraendert (Nachweis, nicht Behauptung)

- `MAGIC = *b"ATCB"`, `CONTAINER_VERSION = 1`, `GENERATION_V1/V2 = 1/2` wie vorher.
- Kopf 10 Byte, Strom, Tail 32 Byte, `hash = SHA-256(ohne_hash)` wie vorher; **kein**
  Domaenenpraefix (das gehoert zu S1-1b und ist hier nicht entschieden).
- Die Abschnitte `// --- SHA-256`, `fn opcode(`, `fn schreibe_op(` sind Zeichen fuer
  Zeichen aus dem vorherigen Stand uebernommen (maschinell verglichen, „IDENTISCH").
- Der fixierte Hash-Test `gastabellen_version_aendert_den_bytecode_hash_nicht`
  traegt unveraendert `0dee103620709f6113425f657a4e48197ba5f69757dca7a20e4d4a116c13a2b4`.
  Haette sich das Format geaendert, waere dieser Test rot. Er ist gruen.

Damit sind Zustandswurzeln, Freigabeschluessel und Artefaktidentitaet von diesem
Paket **nicht** betroffen.

## 5 Pruefreihenfolge jetzt

| Stufe | Bedingung | Code | Offset |
|---|---|---|---|
| 1 | `len < 42` | `E-001` | Eingabelaenge |
| 2 | `bytes[0..4] != "ATCB"` | `E-007` | 0 |
| 3 | `bytes[4] != 0x01` | `E-003` | 4 |
| 4 | Generation nicht 1 oder 2 | `E-009` | 5 |
| 5-7 | Opcode unbekannt / v2-Opcode in Generation 1 / Operand reicht nicht bis `tail_start` | `E-010` / `E-011` / `E-002` | Opcode-Position / Opcode-Position / `tail_start` |
| 8a | Strom endet an einer Grenze vor `c` | `E-005` | `tail_start` |
| 8b | nach `c` Instruktionen bleiben Bytes | `E-008` | erstes Restbyte |
| 8c | `c = 0` und Strom leer | `E-004` | 10 |
| 9 | Tail passt nicht | `E-006` | `tail_start` |
| 10 | mehr dekodierte Ops als `MAX_PROGRAM_OPS` | `E-012` | 10 |

`tail_start = len - 32` ist die einzige Stromgrenze; `c` ist Obergrenze ohne
Endmarker. `E-004` gilt nur bei `c = 0` **und** leerem Strom: `c = 0` mit Bytes
ergibt `E-008`, `c = 2` bei einer Instruktion ergibt `E-005`, eine um ein Byte zu
kurze Instruktion an derselben Grenze ergibt `E-002`. Die vier Stromcodes sind
paarweise disjunkt.

## 6 Testevidenz

Ausgefuehrt im Arbeitsbaum auf `6a164080` mit den Blobs `75e881e4` und
`c9f1da42`, Toolchain `cargo 1.99.1`, offline.

| Pruefung | Ergebnis |
|---|---|
| `cargo test --workspace --offline` | **684 passed, 0 failed**, exit 0 (vorher 672) |
| `cargo test -p atc-vm --offline` | **153 passed, 0 failed** (vorher 132): 44 lib, 44 bin, 26 `atcb_v2`, 12 `exec_chain_e2e`, 27 `gas_table_v1` |
| `cargo test -p atc-system --offline` | 13 + 11 + 11 + 11 passed, 0 failed |
| `cargo fmt --check` (beide Dateien) | ohne Abweichung |
| `cargo clippy -p atc-vm --lib` | keine Warnung |
| `cargo build --workspace` | erfolgreich; die 24 Dead-Code-Warnungen des Bin `atc-vm-runner` sind unveraendert gegenueber dem Stand davor |

Neue Tests im VM-Crate und die Abweichung, die sie pruefen:

| Test | Abweichung |
|---|---|
| `leerer_strom_ergibt_e_004` | 2 (`E-004` fehlte) |
| `zu_grosse_anzahl_an_der_grenze_ergibt_count_mismatch` | 2 + 4 (`E-005` an `tail_start`, nicht `E-002`) |
| `abgeschnittener_operand_ergibt_e_002` | Abgrenzung `E-005` gegen `E-002` |
| `anzahl_null_mit_bytes_ergibt_e_008` | Abgrenzung `E-004` gegen `E-008` |
| `zu_kleine_anzahl_ergibt_trailing_bytes` | Offset `E-008` |
| `struktur_vor_digest` | 3 (Strukturcode gewinnt gegen veralteten Tail; reine Integritaet bleibt `E-006`) |
| `deklarierte_anzahl_wird_nicht_als_groesse_benutzt` | 4 (`c = 0xFFFFFFFF` → `E-005`) |
| `groesstwert_misst_die_dekodierte_laenge` | 4 (`E-012` erst bei mehr dekodierten Ops) |
| `huelle_bleibt_atcb_fassung_1` | Bedingung „Format `0x01`" |
| `fremdes_containerformat_abgelehnt` | `0x02`/`0x03`/`0xFF` → `E-003` |
| `fehlercodes_sind_die_geschlossene_liste` | `code()` deckt E-001 bis E-012 ab |

## 7 Grenze dieser Evidenz (wichtig)

Der Parser-Korpus `standards/atc/conformance/stage1-parser-corpus.json` besteht
aus **ATCE**-Containern mit Format `0x02` (40 Eingaben, C-1 bis C-8). Die VM
verarbeitet nach der Owner-Bedingung **ATCB** mit Format `0x01`. Ein Lauf des
Korpus gegen die VM ist deshalb heute nicht moeglich: jede C-1-Eingabe wuerde an
Stufe 2 mit `E-007` scheitern. Der Differentialtest (REQ-SCE-084, S1-T-54) bleibt
damit **unveraendert offen**, und `E-012` bleibt in der VM an die feste
Protokollkonstante `MAX_PROGRAM_OPS` gebunden, nicht an den in P1b zu messenden
Betreiberwert.

Die Pruefungen oben sind Code-Tests mit denselben Konstruktionen wie der Korpus,
nicht der Korpus selbst. Sie belegen die Semantik, nicht die Konformitaet.

## 8 Offene Punkte

**D-0 Huelle: zwei Aussagen, eine muss fallen.** Kandidat §18/§19 und der Korpus
beschreiben die Huelle `ATCE` mit Format `0x02` (Entscheidung vom 12.11.2026).
Die Freigabe zu D1-B/D1-C legt fest, dass `ATCB` das kanonische VM-Magic bleibt
und `0x01` dem normativen Contract entspricht. Beides zugleich ist eine zweite
Wahrheit ueber dasselbe Format. Entscheidung noetig, eine von zwei:

1. Huelle `ATCE`/`0x02` umsetzen (eigenes Paket, nicht freigegeben). Dann wird der
   Korpus gegen die VM lauffaehig und S1-T-54 ueberhaupt pruefbar.
2. Kandidat und Korpus auf `ATCB`/`0x01` umstellen. Dann entfaellt die Huelle als
   eigenes Format, und §18.1/§18.3 sowie der Korpus sind fortzuschreiben.

Bis zur Entscheidung bleibt der Code bei `ATCB`/`0x01` (ausdrueckliche
Owner-Bedingung) und der Kandidat unveraendert. Es wird hier **keine** der beiden
Varianten stillschweigend in die Spezifikation geschrieben.

**D1-A** (Bytegrenze vor dem Hashing) bleibt offen, Wert aus P1b.
**`E-012`-Betriebswert** bleibt offen (P1b); die VM prueft weiterhin die
Protokollkonstante 65 536 dekodierte Operationen, jetzt auf der richtigen Groesse.
**CI:** `CI_VERIFIED` erst nach einem gruenen CI-Lauf am exakten Commit. Die
Warnung `CI_VERIFIED` gilt ausdruecklich **nicht** fuer den lokalen Lauf oben.

## 9 Statusgrenzen

| Gegenstand | Status |
|---|---|
| D1-B Fehlercodes `E-004`, `E-005` | `TESTED` (lokal, `6a164080`) |
| D1-B Pruefreihenfolge (Struktur vor Integritaet) | `TESTED` (lokal) |
| D1-C Groessenpruefung auf dekodierter Laenge | `TESTED` (lokal) |
| Format `ATCB`/`0x01` unveraendert | `TESTED` (fixierter Hash gruen) |
| Differentialtest gegen den Korpus | nicht ausgefuehrt, blockiert durch D-0 |
| CI-Evidenz am Commit | nicht vorhanden |
| `CI_VERIFIED`, `VERIFIED` | nicht vergeben |
| Matrix S1-T-49 bis S1-T-52 | `TESTED` (Code-Test; Korpusanteil offen) |
| P1a | `OPEN` |
| P2 | `BLOCKED` durch B-1 |
| Normstatus des Kandidaten | unveraendert, nicht ratifiziert |

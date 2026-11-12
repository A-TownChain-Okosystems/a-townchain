# Stufe 1 — Nachweis des Parser-Korpus und des Referenzdekoders

> **Status:** Nachweis. Kein Standard, keine Konformitaetsaussage, keine
> Implementierungs- oder Releasefreigabe.
> **Datum:** 2026-11-12 · **Gebunden an:** die sechs Blobs in Abschnitt 5,
> erhoben bei `a-townchain` main `57fce3e3`
> **Gegenstand:** §19.2a und §19.5 des Kandidaten
> `ATC-SMART-CONTRACT-EXECUTION-001` (v1.10.0-draft), Korpus
> `standards/atc/conformance/stage1-parser-corpus.json`

## 1. Zweck und Grenze

Diese Nachweise beantworten die Frage, ob die Parserprozedur aus §19.2a
**ausfuehrbar und eindeutig** ist. Sie beantworten **nicht**, ob ein Dekoder
konform ist: REQ-SCE-084 verlangt zwei unabhaengige Dekoder, die auf demselben
Korpus dasselbe Paar `(Code, Offset)` liefern. Hier laeuft genau einer, der
Referenzdekoder; die VM ist kein konformer zweiter Dekoder (Abschnitt 4.2 und 4.4).

Daraus folgt: kein Test der Matrix wechselt durch dieses Dokument den Status,
ausser S1-T-18, der den L1-Status fuehrt (`EMPTY` ohne Dekoder, `OPEN` mit
ausfuehrbarem Dekoder). Die elf gruenen Werkzeugtests sind **keine** elf bestandenen
Matrixtests, und der Abgleich Dekoder gegen Korpus ist **kein** Differentialtest.

## 2. Werkzeuge und Aufrufe

| Werkzeug | Aufgabe |
|---|---|
| `tools/atc_reference_decoder.py` | reiner Dekoder nach §19.2a, ohne Projektcode, ohne Rust-Bindung |
| `tools/build_parser_corpus.py` | erzeugt C-1 bis C-8; Sollwerte **aus der Konstruktion**, nicht vom Dekoder |
| `tools/check_parser_corpus.py` | vergleicht Dekoder gegen Korpus, prueft die Datei gegen die Erzeugung |
| `tools/tests/test_parser_corpus.py` | Werkzeugtests, 11 Faelle |

```
python3 tools/check_stage1_vectors.py           # PASS: 9 Gold-, 2 Negativvektoren
python3 tools/build_parser_corpus.py --check    # PASS: 40 Eingaben in 8 Klassen
python3 tools/check_parser_corpus.py            # PASS: 40 Eingaben, 0 Abweichungen
python3 tools/atc_reference_decoder.py --selftest
python3 -m pytest tools/tests/test_parser_corpus.py -q   # 11 passed
```

Alle fuenf Aufrufe wurden am Stand `57fce3e3` mit den in Abschnitt 5 genannten
Blobs ausgefuehrt.

## 3. Korpus

| Klasse | Eingaben | Gegenstand | erwartete Codes |
|---|---|---|---|
| C-1 | 3 | GV-1, GV-3, GV-7 als vollstaendige Container | angenommen |
| C-2 | 3 | Magic falsch, Generation `0x00`, Generation `0x03` | `E-007`, `E-009` |
| C-3 | 2 | Altcontainer `ATCB` + `0x01` (GV-5 und Minimalform) | `E-007` bei 0 |
| C-4 | 3 | `ATCE` + `0x01`, `0x03`, `0xFF` | `E-003` bei 4 |
| C-5 | 17 | gekuerzte Stroeme, unbekannter Opcode, v2-Opcode bei Generation 1, leerer Strom, fehlender Operand | `E-002`, `E-005`, `E-010`, `E-011`, `E-004` |
| C-6 | 4 | ein bis drei eingefuegte Bytes vor dem Tail | `E-008` |
| C-7 | 2 | Tail rotiert, Tailbyte invertiert, Laenge unveraendert | `E-006` |
| C-8 | 6 | Anzahl zu klein, zu gross, `0xFFFFFFFF` | `E-008`, `E-005` |

Jede Zeile traegt ihre Konstruktion im Feld `hinweis` und ihren Sollwert in
`entscheidung`/`code`/`offset`. Die drei C-1-Container werden beim Erzeugen gegen
`stage1-vectors.json` geprueft, damit der Korpus nicht unbemerkt von §18.2 abweicht.

**Kein `E-012`-Korpusfall.** `E-012` misst die dekodierte Stromlaenge gegen einen
Betreiberwert; ein Wert ist nicht festgelegt (T-21 bis T-24). Ein Korpusfall wuerde
eine Zahl festschreiben, die nicht entschieden ist. Der Dekoder kennt den Parameter
`max_stream_bytes`; `test_e012_nur_mit_konfiguriertem_grenzwert` zeigt, dass `E-012`
ohne Wert nicht erreichbar und mit Wert wirksam ist. Siehe auch Abschnitt 6: `E-012`
begrenzt den Aufwand **nicht**, weil er am Ende der Pruefreihenfolge steht.

## 4. Befunde aus diesem Lauf

### 4.1 `E-005` und `E-008` waren nicht trennbar — jetzt sind sie es

Der Strom hat keinen Endmarker. Frueher standen `E-008` und `E-005` beide fuer
"es bleiben Bytes uebrig" beziehungsweise "es fehlen Bytes", ohne dass festgelegt war,
woran die Grenze gemessen wird. Jetzt gilt `tail_start = L - 32` (REQ-SCE-085), `c` ist
nur eine Obergrenze, und die vier Stromcodes sind paarweise disjunkt (REQ-SCE-087).

Der Korpus enthaelt den entscheidenden Nachbarschaftsfall bei **demselben Offset 19**:

| Eingabe | Konstruktion | Code |
|---|---|---|
| `C-8-GV1-ZU-KLEIN` | Anzahl `2` -> `1`, beide Instruktionen im Strom vorhanden | `E-008` bei 19 |
| `C-5-GV1-K1` | Anzahl `2` bleibt, das `Halt` fehlt | `E-005` bei 19 |

Ein Byte Unterschied in der Konstruktion, zwei verschiedene Codes, gleicher Offset. Die
Nachbarschaft `C-5-GV1-K1` gegen `C-5-GV1-K2` trennt zusaetzlich die saubere
Instruktionsgrenze vom abgeschnittenen Operanden: `E-005` bei 19 gegen `E-002` bei 18.

Der Fehlercode haengt damit **nicht an der Position allein**, sondern an der Position
zusammen mit dem Dekodierzustand und der deklarierten Obergrenze. Genau das war vorher
nicht festgelegt.

### 4.2 Die VM prueft den Digest vor dem Strom

`components/vm/src/artifact.rs:311-319` vergleicht den Hash, bevor dekodiert wird. Der
Vertrag legt die Struktur **vor** die Integritaet (Stufen 5 bis 8 vor Stufe 9,
REQ-SCE-088). Bei einem Container mit zwei Defekten meldet die VM daher `BadHash`, der
Vertrag den Strukturcode. Der Korpus fuehrt jede Strukturmutation doppelt, mit
belassenem und mit neu berechnetem Tail, und zeigt damit, dass der Strukturcode in
beiden Faellen gleich bleibt.

Das ist die **dritte** Abweichung der VM vom Vertrag, nach den zwei fehlenden
Fehlervarianten (`EMPTY_STREAM` = `E-004`, `COUNT_MISMATCH` = `E-005`) und der Annahme
von `ATCB`/`0x01`.

### 4.3 Der dokumentierte P2-Strom war nicht dekodierbar

`Store(u32)` verlangt nach ATC-BC-002 §16 vier Operandenbytes. Der in §18.2
dokumentierte P2-Strom kodierte `Store` mit einem Byte und war 21 Byte lang. Als
Container geprueft ergab er `E-002` bei Offset 31, **obwohl der Digest stimmte**:

```
GV-7 alt: 63 Byte, artifact_hash korrekt, Dekodierung -> E-002 bei 31
```

Damit war GV-7 als C-1-Eintrag nicht geeignet, und der Fehler waere ohne den Korpus
unentdeckt geblieben: eine Hashprobe belegt keine Dekodierbarkeit. P2 ist auf die
kanonische Kodierung `0b 00000000` berichtigt (25 Byte); GV-6 und GV-7 sind nach
S1-T-27 neu berechnet, alle anderen Vektoren bleiben unveraendert. Der Regressionstest
`test_p2_korrektur_ist_wirklich_noetig` haelt beide Fassungen fest.

### 4.4 Die VM begrenzt die deklarierte Anzahl, der Vertrag die dekodierte Laenge

`artifact.rs:322` prueft direkt nach dem Digest `count > MAX_PROGRAM_OPS`
(65 536, `vm.rs:7`), also die **deklarierte Anzahl** und damit vor dem Strom.
REQ-SCE-089 legt das Gegenteil fest: gemessen wird die dekodierte Stromlaenge, die
Anzahl ist ausdruecklich nur eine Obergrenze und **kein** Ausloeser fuer `E-012`.

Fuer dieselbe Eingabe ergibt das zwei verschiedene Ergebnisse:

| Eingabe | Vertrag | VM |
|---|---|---|
| `C-8-GV1-MAX` (Anzahl `0xFFFFFFFF`, 25-Byte-Strom, gueltiger Tail) | `E-005` bei 20 nach einem Schritt | `ProgramTooLarge` |

Das ist die **vierte** Abweichung. Beide Regeln sind vereinbar, wenn die
Anzahlpruefung dort bleibt, wo sie hingehoert — beim Erzeugen und beim Ausfuehren — und
der Dekodierpfad der Praezedenz des Vertrags folgt. Sie sind es heute nicht.

## 5. Bindung an den Repositorystand

Jede Aussage dieses Vermerks gilt fuer genau die folgenden Blobs. Aendert sich einer
davon, ist die Aussage **nicht** mehr belegt und muss neu erhoben werden (dasselbe
Verfahren wie S1-T-27 bei Wertwechseln); eine Fortschreibung ohne erneuten Lauf ist
unzulaessig.

Der Kopf-HEAD ist der **Erhebungszeitpunkt**, nicht die Bindung: er bewegt sich mit
jedem weiteren Commit, die Blobs nicht. Dieser Vermerk wurde mit einem spaeteren
Commit veroeffentlicht, der nur ihn, die Registerdatei und die Matrix aendert; die
sechs Blobs sind dabei unveraendert geblieben und wurden danach erneut geprueft.

| Gegenstand | Pfad | Blob bei `57fce3e3` |
|---|---|---|
| Kandidat v1.10.0-draft | `standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md` | `337bc21b` |
| Parser-Korpus | `standards/atc/conformance/stage1-parser-corpus.json` | `3563869b` |
| Referenzdekoder | `tools/atc_reference_decoder.py` | `10e504d1` |
| Korpus-Pruefwerkzeug | `tools/check_parser_corpus.py` | `b13d6d3f` |
| Werkzeugtests | `tools/tests/test_parser_corpus.py` | `8fa2de49` |
| VM-Dekodierpfad | `components/vm/src/artifact.rs` | `9a1c3bf9` |

Der Vermerk nennt seinen eigenen Blob nicht; das waere ein Zirkel. Er ist ueber Pfad
und Kopf-HEAD adressiert, die Nachverfolgungsmatrix verweist auf ihn.

## 6. Ressourcenobergrenze vor dem Dekodieren (offen)

**Frage.** Gibt es eine verbindliche Obergrenze fuer die Groesse einer
Transaktion beziehungsweise eines Containers, die **vor** dem Dekodieren wirkt?

**Befund am Stand `57fce3e3`: nein, fuer Container nicht.** Es gibt Grenzen, aber keine
davon begrenzt die Eingabe des Dekoders:

| Grenze | Wert | Ort | Gegenstand |
|---|---|---|---|
| `MAX_PROGRAM_OPS` | 65 536 | `vm.rs:7`, `artifact.rs:245`/`:322`, `vm.rs:221`, `backend.rs:199` | Anzahl Operationen, nicht Bytes |
| `MAX_STACK_ITEMS` | 4 096 | `vm.rs:9` | Ausfuehrung, nicht Deklaration |
| `MAX_STORAGE_SLOTS` | 65 536 | `vm.rs:11` | Zustand, nicht Eingabe |
| `MAX_SOURCE_BYTES` | 1 MiB | `assembler.rs:12` | Assemblerquelle, nicht Container |
| `MAX_RPC_REQUEST_BYTES` | 8 KiB | `node/src/rpc.rs:11` | HTTP-Rumpf einer RPC-Anfrage |
| `MAX_ARTIFACT_BYTES` | 512 MiB | `update-authority/src/lib.rs:37` | Release-Pakete, anderes Objekt |
| Block-Payload | keine | `node/src/chain.rs:17` (`String`) | beliebig lang |
| Mempool/Konfiguration | keine | `node/src/config.rs` (nur `max_peers`) | keine Groessengrenze |

`Artifact::parse` prueft als erstes nur `bytes.len() < 42` und hasht danach die
**gesamte** Eingabe (`artifact.rs:311-319`). Jede Grenzpruefung kommt spaeter. Eine
uebergrosse Eingabe kostet daher vollstaendige SHA-256-Arbeit, bevor irgendetwas
abgelehnt wird; die Kosten wachsen linear mit der Eingabe und sind pro Aufruf
wiederholbar. `E-012` hilft dabei nicht, weil es nach Vertrag Stufe 10 ist und die
dekodierte Laenge misst.

**Vorschlag (keine Entscheidung).** Die Laengengrenze gehoert zum Aufrufer, nicht in
den Dekoder: der Dekoder bleibt eine reine Funktion, und der Admission-Layer ruft ihn
nicht mit Eingaben oberhalb eines konfigurierten Hoechstwertes auf. `E-012` bleibt
dann die semantische Programmgrenze, und es entsteht kein neuer Fehlercode und keine
Aenderung der Praezedenz. Der Zahlenwert ist aus den Messungen in P1b abzuleiten und
nicht aus bestehenden VM-Grenzen; solange er fehlt, bleibt das Ressourcenrisiko offen.
Eingetragen als offener Punkt an D1, nicht als Anforderung.

## 7. Was daraus nicht folgt

| Aussage | gilt nicht, weil |
|---|---|
| "D1 ist implementiert" | `implementation_status` bleibt `OPEN`; die Projektimplementierung ist unveraendert |
| "der Parser ist konform" | ein Dekoder ist kein Differentialtest (REQ-SCE-084) |
| "die VM ist der zweite Dekoder" | vier Abweichungen: `ATCB`/`0x01`, zwei fehlende Fehlervarianten, Digest vor Struktur, Anzahl statt Stromlaenge |
| "L2 oder L3 sind pruefbar" | B-1 ist unentschieden; nur L1 zaehlt fuer P1a |
| "E-012 ist geklaert" | der Groesstwert ist offen (T-21 bis T-24) |
| "uebergrosse Eingaben sind abgewehrt" | keine Laengengrenze vor dem Dekodieren (Abschnitt 6) |
| "die Grenzwerte gelten" | 8.192/4.096 Byte und Tiefe 8 bleiben Vorschlag |

P1a bleibt offen, P2 bleibt an B-1 gesperrt, der Standard bleibt Kandidat.

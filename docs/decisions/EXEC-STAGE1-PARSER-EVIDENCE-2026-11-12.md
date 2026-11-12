# Stufe 1 — Nachweis des Parser-Korpus und des Referenzdekoders

> **Status:** Nachweis. Kein Standard, keine Konformitaetsaussage, keine
> Implementierungs- oder Releasefreigabe.
> **Datum:** 2026-11-12 · **Gebunden an:** die sechs Blobs in Abschnitt 5
> **Gegenstand:** §19.2a, §19.5 und §19.7 des Kandidaten
> `ATC-SMART-CONTRACT-EXECUTION-001` (v1.10.0-draft), Korpus
> `standards/atc/conformance/stage1-parser-corpus.json`

## 1. Zweck und Grenze

Diese Nachweise beantworten die Frage, ob die Parserprozedur aus §19.2a
**ausfuehrbar und eindeutig** ist. Sie beantworten **nicht**, ob ein Dekoder
konform ist: REQ-SCE-084 verlangt zwei unabhaengige Dekoder, die auf demselben
Korpus dasselbe Paar `(Code, Offset)` liefern. Hier laeuft genau einer, der
Referenzdekoder; die VM ist kein konformer zweiter Dekoder (Abschnitt 4.2 bis 4.4).

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
ohne Wert nicht erreichbar und mit Wert wirksam ist. Siehe Abschnitt 6: `E-012`
begrenzt den Aufwand **nicht**, weil es am Ende der Pruefreihenfolge steht und eine
andere Groesse misst.

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

Zusammen mit der Annahme von `ATCB`/`0x01` und den zwei fehlenden Fehlervarianten
(`EMPTY_STREAM` = `E-004`, `COUNT_MISMATCH` = `E-005`) sind das drei Korrekturen am
Dekodierpfad, bevor die VM als zweiter Dekoder gelten kann.

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

### 4.4 Vier Groessen, die nicht dasselbe sind

Der vierte Unterschied ist keine Namensfrage, sondern eine andere Pruefbasis mit anderer
Fehlerpraezedenz:

| Groesse | Bedeutung |
|---|---|
| `c` | im Container deklarierte Instruktionsanzahl (4 Byte im Kopf) |
| `p - 10` | tatsaechlich dekodierte Stromlaenge in Bytes, bis `tail_start` |
| `i` | tatsaechlich dekodierte Instruktionsanzahl |
| `MAX_PROGRAM_OPS` | bisherige VM-Grenze, 65 536, heute gegen `c` geprueft |

`artifact.rs:322` prueft `c > MAX_PROGRAM_OPS` direkt nach dem Digest, also die
**deklarierte Anzahl** und damit vor dem Strom. REQ-SCE-089 legt das Gegenteil fest:
gemessen wird die dekodierte Stromlaenge, die Anzahl ist ausdruecklich nur eine
Obergrenze und **kein** Ausloeser fuer `E-012`.

| Eingabe | Vertrag | VM |
|---|---|---|
| `C-8-GV1-MAX` (`c = 0xFFFFFFFF`, 25-Byte-Strom, gueltiger Tail) | `E-005` bei 20 nach einem Schritt | `ProgramTooLarge` |

Die VM begrenzt damit eine andere Groesse an einer anderen Stelle der Reihenfolge. Beide
Regeln sind vereinbar, wenn die Anzahlpruefung dort bleibt, wo sie hingehoert — beim
Erzeugen und beim Ausfuehren — und der Dekodierpfad der Praezedenz des Vertrags folgt.
Sie sind es heute nicht. Owner-Auflage: **D1-C**.

## 5. Bindung an den Repositorystand

Die Bindung sind die Blob-SHAs, nicht der Kopf-HEAD. Der HEAD ist nur der
Erhebungszeitpunkt; er bewegt sich mit jedem Commit, die Blobs nicht. Drei Zeitpunkte
und ein Inhaltsvergleich:

| Zeitpunkt | HEAD | Vorgang |
|---|---|---|
| Erhebung | `57fce3e3` | Korpus, Dekoder, Werkzeuge angelegt und geprueft |
| Nachtrag | `2c345975` | Vermerk veroeffentlicht, Blobs unveraendert |
| Gegenpruefung | `0cf3b9df` | Blobs erneut verglichen, unveraendert |
| Nachtrag §19.7 | `7a6fc634` | Kandidat geaendert (§19.7, Teilauflagen), fuenf Blobs unveraendert; alle Pruefungen erneut ausgefuehrt |

| Gegenstand | Pfad | Blob bei `57fce3e3` | Blob bei `7a6fc634` |
|---|---|---|---|
| Kandidat v1.10.0-draft | `standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md` | `337bc21b` | `b1789921` |
| Parser-Korpus | `standards/atc/conformance/stage1-parser-corpus.json` | `3563869b` | `3563869b` |
| Referenzdekoder | `tools/atc_reference_decoder.py` | `10e504d1` | `10e504d1` |
| Korpus-Pruefwerkzeug | `tools/check_parser_corpus.py` | `b13d6d3f` | `b13d6d3f` |
| Werkzeugtests | `tools/tests/test_parser_corpus.py` | `8fa2de49` | `8fa2de49` |
| VM-Dekodierpfad | `components/vm/src/artifact.rs` | `9a1c3bf9` | `9a1c3bf9` |

Der Kandidat hat sich geaendert, und deshalb wurde die Regel angewandt, statt sie zu
umgehen: alle Pruefungen wurden am neuen Blob `b1789921` erneut ausgefuehrt
(Vektoren, Korpus, Register, Werkzeugtests), alle `PASS`. Die Aenderung ist additiv
(§19.7 und die Teilauflagentabelle), die Aussagen zu §19.1 bis §19.5 sind unberuehrt.
Aendert sich kuenftig einer der fuenf uebrigen Blobs, gilt dasselbe: neu erheben, nicht
fortschreiben. Dieser Vermerk nennt seinen eigenen Blob nicht; das waere ein Zirkel.

## 6. Ressourcenobergrenze vor dem Dekodieren (offen, D1-A)

**Frage.** Gibt es eine verbindliche Obergrenze fuer die Groesse einer Transaktion
beziehungsweise eines Containers, die **vor** dem Hashing wirkt?

**Befund am Stand `7a6fc634`: nein, fuer Container nicht.** Es gibt Grenzen, aber keine
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

Der Ablauf im Transaktionspfad ist heute: Container annehmen, Mindestlaenge pruefen,
SHA-256 ueber die gesamte Eingabe, danach erst Struktur- und Groessenpruefungen. Die
Hash-Arbeit skaliert mit der Eingabe und ist pro Aufruf wiederholbar; eine uebergrosse
Eingabe verbraucht also Ressourcen, bevor sie als unzulaessig erkannt wird.

**Zielarchitektur (Owner, 12.11.2026).** Der Admission-Layer prueft die
Container-Bytegrenze **vor** dem Hashing und moeglichst vor jeder Allokation oder
Dekodierung; der Dekoder bleibt eine reine, deterministische Funktion ohne eigene
Groessenpolitik. Drei Bedingungen sind dabei ausdruecklich festgehalten (Kandidat
§19.7, REQ-SCE-090 bis 093, Vorschlag):

1. Der Wert ist **konsensweit identisch**. Ein bloss lokal konfigurierbarer Grenzwert
   waere auf dem Konsenspfad gefaehrlich: zwei Validatoren koennten dieselbe Transaktion
   unterschiedlich annehmen oder ablehnen.
2. Die Grenze wirkt auf die **tatsaechlichen Container-Bytes**, vor dem Hashing.
3. **Getrennte Schutzschichten bleiben getrennt:** RPC-Rumpf und Block-Payload begrenzen
   Transport und Block, nicht das Artefakt. Und die Bytegrenze darf nicht mit `E-012`
   verschmolzen werden: die Grenze bewertet die gesamte Eingabe (Kopf, Strom, Tail) vor
   dem Hashing, `E-012` die dekodierte Stromlaenge ohne Kopf und Tail. Zwei Messgroessen,
   zwei Zustaendigkeiten; ein gemeinsamer Fehlercode wuerde sie verdecken.

Der Zahlenwert ist aus P1b abzuleiten und nicht aus bestehenden VM-Grenzen; solange er
fehlt, bleibt **D1-A** offen und `S1-T-55` `OPEN`.

## 7. Status

| Bereich | Status |
|---|---|
| Blob-basierte Evidenzbindung | stabil (Abschnitt 5) |
| Parser-Fehlersemantik §19.1 bis 19.2a | spezifiziert |
| Referenzdekoder und Korpus | vorhanden, 0 Abweichungen |
| Container-Byteobergrenze | offen, D1-A, Wert aus P1b |
| VM-Fehlercodes `E-004`, `E-005` | nicht konform, D1-B |
| Digest-Pruefreihenfolge | nicht konform, D1 (S1-T-52) |
| Groessenpruefung `c` gegen `p - 10` | nicht konform, D1-C |
| Differentialtest | nicht bestanden |
| S1-T-18 (L1-Status) | `TESTED` als Statustest |
| Matrix | 55 Tests: 53 `OPEN`, 1 `BLOCKED`, 1 `TESTED` |
| P1a | `OPEN` |
| P2 | `BLOCKED` durch B-1 |
| Normstatus | Kandidat, nicht normativ |

Nicht abgeleitet ist daraus: keine Implementierungsfreigabe, keine Konformitaet, keine
Aussage ueber L2 und L3 (B-1 unentschieden), und die Grenzwerte 8.192/4.096 Byte sowie
Tiefe 8 bleiben Vorschlag.

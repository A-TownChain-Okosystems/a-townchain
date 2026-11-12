# Stufe 1 — Nachweis des Parser-Korpus und des Referenzdekoders

> **Status:** Nachweis. Kein Standard, keine Konformitaetsaussage, keine
> Implementierungs- oder Releasefreigabe.
> **Datum:** 2026-11-12 · **Grundlage:** `a-townchain` main `3c82a5df3`
> **Gegenstand:** §19.2a, §19.5 des Kandidaten
> `ATC-SMART-CONTRACT-EXECUTION-001` (v1.10.0-draft), Korpus
> `standards/atc/conformance/stage1-parser-corpus.json`

## 1. Zweck und Grenze

Diese Nachweise beantworten die Frage, ob die Parserprozedur aus §19.2a
**ausfuehrbar und eindeutig** ist. Sie beantworten **nicht**, ob ein Dekoder
konform ist: REQ-SCE-084 verlangt zwei unabhaengige Dekoder, die auf demselben
Korpus dasselbe Paar `(Code, Offset)` liefern. Hier laeuft genau einer, der
Referenzdekoder; die VM ist heute kein konformer zweiter Dekoder.

Daraus folgt: kein Test der Matrix wechselt durch dieses Dokument den Status,
ausser S1-T-18, der den L1-Status fuehrt (`EMPTY` ohne Dekoder, `OPEN` mit
ausfuehrbarem Dekoder).

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

**Kein `E-012`-Fall.** `E-012` misst die dekodierte Stromlaenge gegen einen
Betreiberwert; ein Wert ist nicht festgelegt (T-21 bis T-24). Ein Korpusfall wuerde
eine Zahl festschreiben, die nicht entschieden ist. Der Dekoder kennt den Parameter
`max_stream_bytes`; der Test `test_e012_nur_mit_konfiguriertem_grenzwert` zeigt, dass
`E-012` ohne Wert nicht erreichbar und mit Wert wirksam ist.

## 4. Drei Befunde aus diesem Lauf

### 4.1 `E-005` und `E-008` waren nicht trennbar — jetzt sind sie es

Der Strom hat keinen Endmarker. Frueher standen `E-008` (Stufe 6) und `E-005`
(Stufe 7) beide fuer "es bleiben Bytes uebrig" beziehungsweise "es fehlen Bytes", ohne
dass festgelegt war, woran die Grenze gemessen wird. Jetzt gilt `tail_start = L - 32`
(REQ-SCE-085), `c` ist nur eine Obergrenze, und die vier Stromcodes sind paarweise
disjunkt (REQ-SCE-087).

Der Korpus enthaelt den entscheidenden Nachbarschaftsfall bei **demselben Offset 19**:

| Eingabe | Konstruktion | Code |
|---|---|---|
| `C-8-GV1-ZU-KLEIN` | Anzahl `2` -> `1`, beide Instruktionen im Strom vorhanden | `E-008` bei 19 |
| `C-5-GV1-K1` | Anzahl `2` bleibt, das `Halt` fehlt | `E-005` bei 19 |

Ein Byte Unterschied in der Konstruktion, zwei verschiedene Codes, gleicher Offset.
Genau diese Stelle war vorher mehrdeutig. Die Nachbarschaft `C-5-GV1-K1` gegen
`C-5-GV1-K2` zeigt zusaetzlich die Grenze zwischen Grenze und Operand: `E-005` bei
19 gegen `E-002` bei 18.

### 4.2 Die VM prueft den Digest vor dem Strom

`components/vm/src/artifact.rs:316-318` vergleicht den Hash, bevor dekodiert wird. Der
Vertrag legt die Struktur **vor** die Integritaet (Stufen 5 bis 8 vor Stufe 9,
REQ-SCE-088). Bei einem Container mit zwei Defekten meldet die VM daher `BadHash`, der
Vertrag den Strukturcode. Der Korpus fuehrt jede Strukturmutation doppelt, mit
belassenem und mit neu berechnetem Tail, und zeigt damit, dass der Strukturcode in
beiden Faellen gleich bleibt.

Das ist die dritte Abweichung der VM vom Vertrag, nach den zwei fehlenden
Fehlervarianten (`E-004`, `E-005`) und der Annahme von `ATCB`/`0x01`.

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

## 5. Was daraus nicht folgt

| Aussage | gilt nicht, weil |
|---|---|
| "D1 ist implementiert" | `implementation_status` bleibt `OPEN`; die Projektimplementierung ist unveraendert |
| "der Parser ist konform" | ein Dekoder ist kein Differentialtest (REQ-SCE-084) |
| "L2 oder L3 sind pruefbar" | B-1 ist unentschieden; nur L1 zaehlt fuer P1a |
| "E-012 ist geklaert" | der Groesstwert ist offen (T-21 bis T-24) |
| "die Grenzwerte gelten" | 8.192/4.096 Byte und Tiefe 8 bleiben Vorschlag |

P1a bleibt offen, P2 bleibt an B-1 gesperrt, der Standard bleibt Kandidat.

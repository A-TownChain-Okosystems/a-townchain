# Stufe 1 — Nachverfolgung der Akzeptanztests

**Status:** Nachverfolgung. Ratifikation erfolgt am 12.11.2026, keine
Implementierungs- oder Releasefreigabe.
**Datum:** 2026-11-12 · **Basis der Ratifikation:** `a-townchain` main `a63e41a93`
**Vorlage:** `docs/decisions/EXEC-STAGE1-DECISIONS-2026-11-12.md`
**Register:** unveraendert (`docs/decisions/exec-decision-register.yaml`, Blob
`9b76813663`)

## 1. Zweck und Grenze

Der Owner hat am 12.11.2026 die vier Optionen ratifiziert (Vermerk:
`docs/decisions/EXEC-STAGE1-RATIFICATION-2026-11-12.md`) und die Nachverfolgung
ihrer Akzeptanzbedingungen bestaetigt. Dieses Dokument fuehrt nur den Nachweisstand.
Es aendert keine Anforderung, erteilt keine Freigabe und ersetzt keinen Test.

Eine periodische Ueberwachung ist nicht eingerichtet. Dieses Dokument wird
aktualisiert, wenn eine der drei Voraussetzungen eintritt:

1. der Owner bestaetigt eine Option oder eine Domaenenvariante,
2. ein reiner Dekoder entsteht (dann kann L1 von `EMPTY` nach `OPEN` wechseln),
3. B-1 wird entschieden (dann werden L2 und L3 pruefbar).

## 2. Gesamtstand

| Status | Anzahl |
|---|---|
| `TESTED` | 1 |
| `FAILED` | 0 |
| `BLOCKED` | 1 |
| `EMPTY` | 0 |
| `OPEN` | 53 |
| **Summe** | **55** |

Ein Test ist `TESTED`: S1-T-18, und zwar als **Statustest**, nicht als
Konformitaetsnachweis. Grund ist die in Abschnitt 1 festgelegte Bedingung "ein reiner
Dekoder entsteht, dann kann L1 von `EMPTY` nach `OPEN` wechseln": der
Referenzdekoder existiert seit dem 12.11.2026. P1a bleibt offen, P2 bleibt an B-1
gesperrt, eine Produktionsfreigabe ist nicht erteilt. Der Status `OPEN` heisst: die
Entscheidung liegt vor, die Umsetzung nicht.

Siebzehn Tests (S1-T-39 bis S1-T-55) sind am 12.11.2026 mit §19, §19.2a und §19.7 des
Kandidaten hinzugekommen: Parser-Fehlersemantik, Stromgrenze, Tail-Integritaet,
Versionsisolation und Differentialtest. Sie erweitern die Matrix, ohne einen
Konformitaetsbefund zu erzeugen.

## 3. Nachweisstand je Test

| ID | Pruefung | haengt an | Status | Nachweisart |
|---|---|---|---|---|
| S1-T-01 | gleicher Strom, anderes Magic | S1-1 | `OPEN` | Code-Test |
| S1-T-02 | Strom-Hash unabhaengig von Generation und Opzahl | S1-1 | `OPEN` | Code-Test |
| S1-T-03 | Zustandswurzel nennt genau eine Identitaet und eine versionierte Domaine | S1-1 | `OPEN` | Code-Test |
| S1-T-04 | Goldvektor-Datei mit beiden Hashes | S1-1 | `OPEN` | Datei + Gate |
| S1-T-05 | Grep-Gate: kein `code_hash = artifact.hash()` | S1-1 | `OPEN` | CI-Gate |
| S1-T-06 | Reproduzierbarkeit der Baseline-Werte aus 1.2 | S1-1 | `OPEN` | Messlauf |
| S1-T-07 | Altbelegung ohne Identitaet wird beim Start abgelehnt | S1-2 | `OPEN` | Code-Test |
| S1-T-08 | Widerruf trifft genau die gewaehlte Identitaet | S1-2 | `OPEN` | Code-Test |
| S1-T-09 | Mindestversion und Widerruf nennen ihre Identitaet | S1-2 | `OPEN` | Code-Test |
| S1-T-10 | Magic-Aenderung kann keine Freigabe umgehen oder vortaeuschen | S1-2 | `OPEN` | Code-Test |
| S1-T-11 | Revert nach 1 000 von 10 000 Gas (T-28) | S1-3 | `OPEN` | Code-Test |
| S1-T-12 | Erschoepfung (T-29): `gas_used` 10 000, `gas_refund` 0 | S1-3 | `OPEN` | Code-Test |
| S1-T-13 | Feldschnitt: `gas_refund` in Gas, Geldbetrag nicht vorhanden oder getrennt | S1-3 | `OPEN` | Code-Test |
| S1-T-14 | ungenutztes Budget verfaellt und ist im Beleg sichtbar | S1-3 | `OPEN` | Code-Test |
| S1-T-15 | jeder Fall T-21 bis T-24 genau einer Schicht zugeordnet | S1-4 | `OPEN` | Matrix + Gate |
| S1-T-16 | L2 laeuft ohne Produktionsfreigabe und ohne Netz | S1-4 | `BLOCKED` | Harness (B-1) |
| S1-T-17 | kein L3-Fall als `TESTED` solange B-1 offen ist | S1-4 | `OPEN` | Gate |
| S1-T-18 | L1-Status folgt der Definition: `EMPTY` ohne Dekoder, `OPEN` mit ausfuehrbarem Dekoder | S1-4 | `TESTED` | Statusbericht + Lauf 12.11.2026 |
| S1-T-19 | getrennte Domaenen, Generation aendert nur `artifact_hash` | S1-1 | `OPEN` | Code-Test |
| S1-T-20 | Determinismus ueber zwei Prozesse, keine Host-Zeit, kein Zufall | S1-1 | `OPEN` | Code-Test |
| S1-T-21 | gleicher Strom, andere Metadaten: `code_hash` gleich | S1-1 | `OPEN` | Code-Test |
| S1-T-22 | V1-Wurzel wird nie als V2 gelesen, Domaine sichtbar | S1-1 | `OPEN` | Code-Test |
| S1-T-23 | keine Uebernahme alter Schluessel, Zaehlung belegbar | S1-2 | `OPEN` | Code-Test + Record |
| S1-T-24 | Start bricht bei gemischten Schluesseln ab, ohne Rueckfall | S1-2 | `OPEN` | Code-Test |
| S1-T-25 | erste Bytes: Strom `0x01..0x13`, Container `0x41` | S1-1a | `OPEN` | Code-Test |
| S1-T-26 | Gate weist Opcode `0x41` oder Fassung ohne Domaene ab | S1-1a/1b | `OPEN` | CI-Gate |
| S1-T-27 | Wertwechsel dokumentiert, Goldvektor erneuert | S1-1b | `OPEN` | Datei + Record |
| S1-T-28 | kein unqualifiziertes `hash` in Code und Doku | S1-1 | `OPEN` | Grep-Gate |
| S1-T-29 | Schluesselraeume getypt, Eintrag nennt seine Identitaet | S1-2 | `OPEN` | Code-Test |
| S1-T-30 | beide Sperren werden geprueft, ein Treffer blockiert | S1-2 | `OPEN` | Code-Test |
| S1-T-31 | Wirkung der Sperren getrennt, wie entschieden | S1-2 | `OPEN` | Code-Test |
| S1-T-32 | Ablehnungsgrund nennt die getroffene Sperre | S1-2 | `OPEN` | Code-Test |
| S1-T-33 | Domaenen-Strings fest und byte-exakt im Standard | S1-1b | `OPEN` | Standard + Gate |
| S1-T-34 | Goldvektoren fuer beide Hashes (Eingabe, Digest) | S1-1b | `OPEN` | Datei + Test |
| S1-T-35 | Containerfassung an Domaenenfassung gebunden | S1-1b | `OPEN` | Code-Test |
| S1-T-36 | Registry und Wurzel koennen die Hashes nicht verwechseln | S1-1/1b | `OPEN` | Code-Test |
| S1-T-37 | flacher 32-Byte-Alt Schluessel wird nicht gedeutet, Start verweigert | S1-2 | `OPEN` | Code-Test |
| S1-T-38 | S1-2 nur als Ganzes: Teilumsetzung besteht das Gate nicht | S1-2 | `OPEN` | Gate |
| S1-T-39 | Tail beginnt exakt bei `10+n`; ein um ein Byte verschobener Digest ergibt `E-006` | S1-1 | `OPEN` | Code-Test |
| S1-T-40 | Gesamtlaenge unter 42 Byte ergibt `E-001`; Container mit 42 Byte und Anzahl 0 ergibt `E-004` | S1-1 | `OPEN` | Grenzwerttest |
| S1-T-41 | Zusaetzliche Bytes zwischen Stromende und Tail ergeben `E-008`, nicht `E-006` | S1-1 | `OPEN` | Code-Test |
| S1-T-42 | Praezedenz: acht kombinierte Fehlerfaelle liefern je den Code der fruehesten Stufe | S1-1 | `OPEN` | Code-Test |
| S1-T-43 | Differentialtest: mindestens 30 Eingaben durch zwei unabhaengige Dekoder, identische `(Code, Offset)` je Zeile | S1-1 | `OPEN` | Vergleichstest |
| S1-T-44 | Versionsisolation: `ATCE` + `0x01` ergibt `E-003`, kein Fallback-Parsing | S1-1 | `OPEN` | Code-Test |
| S1-T-45 | Altcontainer `ATCB` + `0x01` ergibt `E-007` (Magic vor Version), nicht `E-003` | S1-1 | `OPEN` | Code-Test |
| S1-T-46 | Unbekannte Versionen `0x03` und `0xFF` bei Magic `ATCE` ergeben `E-003`, ohne spekulatives Parsen | S1-1 | `OPEN` | Code-Test |
| S1-T-47 | Opcodes `0x11` bis `0x13` bei Generation `0x01` ergeben `E-011`; Generation `0x03` ergibt `E-009` | S1-1 | `OPEN` | Code-Test |
| S1-T-48 | Korpusauswahl: nur vollstaendige Container sind Parser-Eingaben; GV-5 erscheint ausschliesslich als C-3-Ablehnungsfall (`E-007`, Offset `0`) | S1-1 | `OPEN` | Reviewtest |
| S1-T-49 | `E-005` gilt genau an der Instruktionsgrenze: Strom endet vor `c`, `p = tail_start`; ein um ein Byte gekuerzter Strom an derselben Stelle ergibt `E-002` | S1-1 | `OPEN` | Code-Test + Korpus |
| S1-T-50 | `E-008` gilt genau nach `c` Instruktionen, Offset ist das erste Restbyte; eine zu kleine Anzahl ergibt an derselben Grenze `E-008`, nicht `E-005` | S1-1 | `OPEN` | Code-Test + Korpus |
| S1-T-51 | `E-004` gilt nur bei `c = 0` und leerem Strom; `c = 0` mit Bytes ergibt `E-008` | S1-1 | `OPEN` | Code-Test |
| S1-T-52 | Struktur vor Integritaet: beschaedigter Strom mit veraltetem Tail ergibt den Strukturcode, nicht `E-006` | S1-1 | `OPEN` | Code-Test + Korpus |
| S1-T-53 | Terminierung und Groesstwert: `c = 0xFFFFFFFF` ergibt `E-005` in einem Schritt, `E-012` nur bei konfiguriertem Wert | S1-1 | `OPEN` | Code-Test |
| S1-T-54 | Korpus C-1 bis C-8 mit mindestens 30 Eingaben, Sollwerte aus der Konstruktion, `tools/check_parser_corpus.py` ohne Abweichung; ein Dekoder allein begruendet keine Konformitaet | S1-1 | `OPEN` | Datei + Gate |
| S1-T-55 | Container-Bytegrenze wirkt vor dem Hashing: eine Eingabe oberhalb des Werts wird abgelehnt, ohne dass Hash- oder Dekodierarbeit anfaellt; derselbe Wert auf allen Validatoren; `E-012` bleibt davon getrennt | S1-1 | `OPEN` | Code-Test + Grenzwerttest |

## 3.1 Wirkung der Ratifikation auf den Nachweisstand

Die Ratifikation beseitigt die **Entscheidungs**blockade, nicht die Umsetzungs- oder
Freigabeblockade. Deshalb aendert sich in der Matrix kein Status:

| Frage | vor der Ratifikation | nach der Ratifikation |
|---|---|---|
| Fehlt eine Option? | ja, bei S1-1 bis S1-4 | nein, alle vier entschieden |
| Ist ein Test dadurch bestanden? | nein | nein |
| Ist Implementierung freigegeben? | nein | nein |
| Ist P1a abschliessbar? | nein | nein, L1 bleibt `EMPTY` bis der Dekoder existiert |
| Ist P2 freigegeben? | nein | nein, B-1 unentschieden |

Der Status `OPEN` bedeutet ab jetzt: die Entscheidung liegt vor, die Spezifikation
und die Umsetzung nicht. Kein Test ist `TESTED`, und keiner darf als bestanden
gemeldet werden, solange er nicht ausgefuehrt wurde.

## 3.2 Stand der Spezifikation (12.11.2026)

Die fuenf Bereiche sind im Kandidaten byte-genau spezifiziert, jeweils mit
Fehlerreaktion und Testbezug:

| Bereich | Abschnitt | Anforderungen | Vektornachweis |
|---|---|---|---|
| Hash-Domaenen | §18.1 | REQ-SCE-061 bis 064 | Labels 16/14/13 Byte, Nullbyte-Pflicht |
| Goldvektoren | §18.2 | REQ-SCE-065 | 9 Gold- und 6 Negativvektoren, maschinenlesbar in `standards/atc/conformance/stage1-vectors.json`; am 12.11.2026 auf Magic `ATCE` neu berechnet (GV-1, GV-3, GV-5, GV-7, GV-9, NV-2) |
| Containerfassung | §18.3 | REQ-SCE-066 bis 069 | Magic `ATCE`, Fassung `0x02`, Tail = `artifact_hash`, sieben Fehlercodes (neu: `E-007 UNKNOWN_MAGIC`) |
| Policy-Schluesselraeume | §18.4 | REQ-SCE-070 bis 074 | achtstufige Pruefsequenz, Startverweigerung |
| Gasrueckgabe | §18.5 | REQ-SCE-075 bis 077 | vier Faelle, Wortlaut fuer kuenftige Kontobelastung |
| Parser-Fehlersemantik | §19.1 bis 19.2 | REQ-SCE-078 bis 079 | Codeliste `E-001` bis `E-012`, zehnstufige Praezedenz |
| Stromgrenze und Prozedur | §19.2a | REQ-SCE-085 bis 089 | `tail_start = L - 32`, vier paarweise disjunkte Stromcodes, `E-005` an der Grenze |
| Tail-Integritaet | §19.3 | REQ-SCE-080 | exakter Offset `10+n`, 32 Byte, keine Bytes danach |
| Versionsisolation | §19.4 | REQ-SCE-081 bis 083 | keine Vermischung, reine Funktion, Generation vor Opcode |
| Differentialtest | §19.5 | REQ-SCE-084 | acht Korpusklassen, 40 Eingaben, Vergleich von `(Code, Offset)` |
| Parser-Korpus | §19.5 | REQ-SCE-084 | `standards/atc/conformance/stage1-parser-corpus.json`, erzeugt aus `tools/build_parser_corpus.py` |

Der Sollwert ist ohne Projektcode pruefbar: `python3 tools/check_stage1_vectors.py`
liest die Vektoren aus dem Dokument und rechnet sie mit reinem SHA-256 nach. Damit
ist die Spezifikation gegen sich selbst pruefbar. Ein ausgefuehrter Dekoder ist
etwas anderes: er zeigt, dass die Prozedur ausfuehrbar und eindeutig ist, nicht dass
zwei unabhaengige Dekoder uebereinstimmen. Deshalb aendert der Lauf aus 3.3 keinen
Teststatus ausser dem Statustest S1-T-18.

**Korpusgrenze (12.11.2026).** Nur GV-1, GV-3 und GV-7 beschreiben vollstaendige
Container und taugen als Parser-Eingaben; GV-5 ist Altformat und gehoert als
Ablehnungsfall nach C-3; GV-2, GV-4, GV-6, GV-8 und GV-9 sind Hash-Eingaben ohne
Container. Test S1-T-48 haelt das fest. GV-7 war dafuer erst nach der Korrektur des
P2-Stroms geeignet (Abschnitt 3.3).

Was weiterhin fehlt, ist ausdruecklich kein Spezifikationsrest: L1 ist mit dem
Referenzdekoder `OPEN`, aber die Konformitaet braucht einen zweiten Dekoder, und die
Aufrufarchitektur B-1 ist nicht entschieden.

## 3.3 Lauf vom 12.11.2026: Stromgrenze, Korpus und Referenzdekoder

| Schritt | Ergebnis |
|---|---|
| Spezifikation §19.2a | Stromgrenze `tail_start = L - 32`, Dekodierprozedur, vier disjunkte Stromcodes (REQ-SCE-085 bis 089) |
| Korpus | 40 Eingaben in C-1 bis C-8, Sollwerte aus der Konstruktion |
| Referenzdekoder | `tools/atc_reference_decoder.py`, reine Funktion, unabhaengig von der VM |
| Abgleich | `tools/check_parser_corpus.py`: 40 Eingaben, 0 Abweichungen |
| Werkzeugtests | `pytest tools/tests/test_parser_corpus.py`: 11 Tests gruen |
| Vektoren | `check_stage1_vectors.py`: 9 Gold- und 2 Negativvektoren nach der P2-Korrektur neu berechnet |
| Bindung | Nachweis an HEAD `57fce3e3` gebunden, sechs Blob-SHAs im Vermerk |
| L1 | `EMPTY` -> `OPEN` (S1-T-18 `TESTED`) |
| Konformitaet | **nicht** erreicht: ein Dekoder ist kein Differentialtest |

Drei Befunde aus diesem Lauf, alle am Byte nachpruefbar:

1. `E-005` und `E-008` waren ohne Stromgrenze nicht trennbar. Mit `tail_start = L - 32`
   sind sie paarweise disjunkt; der Korpus enthaelt beide Faelle bei **demselben
   Offset 19** (`C-8-GV1-ZU-KLEIN` -> `E-008`, `C-5-GV1-K1` -> `E-005`). Genau diese
   Nachbarschaft war vorher mehrdeutig.
2. Die VM prueft den Digest **vor** dem Strom (`artifact.rs:316-318`) und meldet bei
   zwei Defekten `BadHash` statt des Strukturcodes. Das ist die dritte Abweichung vom
   Vertrag und der Grund, warum sie im Differentialtest heute durchfaellt.
3. Der dokumentierte P2-Strom war nicht dekodierbar: `Store(u32)` braucht vier
   Operandenbytes, dokumentiert war eines. GV-7 ergab `E-002` bei Offset 31, obwohl
   der Digest stimmte. Ein Hash-Nachweis belegt also keine Dekodierbarkeit. P2 ist
   berichtigt, GV-6 und GV-7 sind nach S1-T-27 neu berechnet.
4. Vierte Abweichung der VM: sie begrenzt die deklarierte Anzahl
   (`MAX_PROGRAM_OPS` 65 536, `artifact.rs:322`), der Vertrag die dekodierte
   Stromlaenge. `C-8-GV1-MAX` ergibt nach Vertrag `E-005`, in der VM `ProgramTooLarge`.
5. Kein Aufwandsschutz vor dem Dekodieren: es gibt keine verbindliche Obergrenze fuer
   Container-Bytes, und `Artifact::parse` hasht die gesamte Eingabe vor jeder
   Grenzpruefung (`artifact.rs:311-319`). Der Owner hat D1 deshalb in drei getrennte
   Teilauflagen gegliedert (Register: D1-A Admission-Grenze, D1-B fehlende
   Fehlervarianten, D1-C Groessenpruefung); Kandidat §19.7 haelt den Rahmen als
   Vorschlag fest, der Wert bleibt offen bis P1b (S1-T-55, Abschnitt 6 des Vermerks).

Die Einzelheiten, Aufrufe und Ausgaben stehen in
`docs/decisions/EXEC-STAGE1-PARSER-EVIDENCE-2026-11-12.md`.

Offen bleibt bewusst: kein `E-012`-Korpusfall (kein Groesstwert festgelegt), kein
zweiter Dekoder (VM nicht konform), keine Aussage ueber P1a und P2.

## 4. Bereits belegte Befunde

Diese Punkte sind keine Tests, sondern am Code oder im Lauf erhobene Befunde. Sie
bleiben gueltig, auch wenn alle Tests `OPEN` sind.

| Befund | Beleg |
|---|---|
| Es gibt nur eine Hashfunktion | `components/vm/src/artifact.rs:297-298`; `grep` ueber `components/` ohne zweiten Kandidaten |
| Code-Hash und Artefakt-Hash sind heute derselbe Wert | `components/system/src/system.rs:223`; Demo-Lauf 12.11.2026: beide `5f2c38c8…`, Artefakt 138 Byte, 20 ATCB-Ops |
| Die Zustandswurzel nimmt den Code-Hash als Eingabe | `components/system/src/state.rs:12`, `43-52`; Demo: `552e5a0b…` -> `0c02e59d…` |
| Ein Container beginnt mit `0x41`, ein Strom mit `0x01..0x13` | `artifact.rs:273-284` und `:130-151`, 19 Opcodes |
| Alle Sperrab Fragen pruefen `policy.code_hash`, nie den beobachteten Hash | `exec-admission/src/lib.rs:191`, `334-335`, `486`, `500` |
| Der Widerruf ist eine flache Menge ohne Typangabe | `:335` `revoked: BTreeSet<[u8; 32]>` |

## 5. Offene Entscheidungsfragen

| Frage | Gegenstand | Empfehlung des Owners |
|---|---|---|
| S1-1 | drei Identitaeten | A, mit Domaenenvariante 1b |
| S1-1a/1b/1c | wie die Domaenen getrennt werden | 1b (beide mit Praefix) |
| S1-2 | Policy-Bindung mit getypten Schluesselraeumen | C, asymmetrisch |
| S1-3 | Erstattung | A, nur Gas-Rueckfluss |
| S1-4 | Testschichtung | B, nur L1 zaehlt fuer P1a |

Alle fuenf Punkte sind am 12.11.2026 entschieden und im Register als `stage1`
eingetragen (Vermerk: `EXEC-STAGE1-RATIFICATION-2026-11-12.md`). Die Spalte
`Empfehlung des Owners` ist damit der Beschluss; die Umsetzung ist es nicht.

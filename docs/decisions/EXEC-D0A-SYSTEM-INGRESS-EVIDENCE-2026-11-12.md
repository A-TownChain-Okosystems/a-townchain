# D-0 Phase 1 und 2: die Huelle `ATCE`/`0x02` ist der System-Eingang

> **Status:** Systemintegration **umgesetzt und lokal geprueft** (`TESTED`).
> `ATCE/0x02` bleibt `OPEN`: Abschluss setzt die vier Architekturfragen voraus
> ([Vorlage](EXEC-D0-ARCH-QUESTIONS-TEMPLATE-2026-11-12.md)) und einen CI-Lauf.
> `CI_VERIFIED` ist nicht vergeben. **Datum:** 2026-11-12.

## 1 Was sich geaendert hat

Vorher nahm das System innere `ATCB`-Bytes entgegen und las sie mit
`Artifact::parse`. Jetzt ist die Huelle der externe Eingang, und die Ausfuehrung
kommt an ihr nicht vorbei.

| Datei | Blob | Aenderung |
|---|---|---|
| `components/system/src/system.rs` | `14436437` | `deploy_container`, `check_container`, Feld `container_bytes`, Stufe `container`, drei Pruefungen in `execute` |
| `components/system/tests/container_admission.rs` | `be6a9c3e` | sieben Tests: Eingang, Gleichheit zum Quellweg, Rueckfuehrung, Negativmatrix, kein Rueckfall, Identitaetsbindung, Anzahl-Ueberlauf |
| `components/system/tests/e2e.rs` | `38af833a` | manipulierte Huelle (Stufe `container`, `E-006`), abweichende innere Bytes (Stufe `admission`), Deployment ohne Huelle (`E-001`) |

Commits: `d5b25f3b` (`system.rs`), `b1d45565` (Negativmatrix), `05fc49c1` (`e2e.rs`).

## 2 Reihenfolge im Eingang

`System::deploy_container` ist bindend geordnet:

1. **Huelle pruefen.** `Envelope::parse` → Laenge, Magic, Fassung, Generation,
   Strom, Tail, Groesstwert. Fehleridentitaet ist `(Code, Offset)`.
2. **Uebergabe.** `into_artifact()` erzeugt das bestehende `ATCB`-Artefakt. Kein
   zweiter Parser, keine Uebersetzung: die Huelle ist zu diesem Zeitpunkt bereits
   vollstaendig geprueft, also faellt keine Hash- oder Dekodierarbeit fuer eine
   spaeter abgelehnte Eingabe an.
3. **Freigabe.** Policy mit dem Bytecode-Hash, Signatur, Registry.

`System::execute` prueft in dieser Reihenfolge:

| Schritt | Pruefung | Fehler bei Verstoss |
|---|---|---|
| 0 | Huelle lesen, Strom ueber `into_artifact()` | Stufe `container` mit `(Code, Offset)`; leere Huelle `E-001`, `ATCB` `E-007` |
| 0b | Bytecode-Hash gegen `deployment.code_hash` | Stufe `admission`: Huelle und Freigabe nennen verschiedene Programme |
| 0c | innere Bytes gegen die Huelle (`artifact.to_bytes() == artifact_bytes`) | Stufe `admission`: zwei Aussagen ueber dasselbe Artefakt |
| 1 | Freigabe (Signatur, Registry, Ablauf) | Stufe `admission` |

Schritt 0c ist die Umsetzung der Architekturregel "eine Wahrheit je
Funktionalitaet": die Huelle ist die Quelle, die inneren Bytes sind eine Ableitung
und duerfen nicht auseinanderlaufen.

## 3 Die Negativmatrix (Phase 2)

Sollwerte mit dem unabhaengigen Referenzdekoder gegengeprueft
(`python3 tools/atc_reference_decoder.py <hex>`). Jeder Fall wird an **drei**
Eingaengen geprueft: `deploy_container`, `check_container` und `execute` (dort
durch Ersetzen der Huelle in einem gueltigen Deployment).

| Fall | Erwartet | Ergebnis |
|---|---|---|
| Digest manipuliert (letztes Byte) | `E-006` bei `len-32` | wie erwartet |
| falsche Generation (`0x07`) | `E-009` bei 5 | wie erwartet |
| ungueltige Laenge (41 Byte) | `E-001` bei 41 | wie erwartet |
| Ueberlauf der Anzahl (`0xFFFFFFFF`) | `E-005` bei `len-32` | wie erwartet, Laufzeit unabhaengig von der Anzahl (< 1 s geprueft) |
| abgeschnittener Tail (1 Byte) | `E-005` bei `len-33` | wie erwartet |
| falsches Magic (`ATC1`) | `E-007` bei 0 | wie erwartet |
| falsche Fassung (`0x03`) | `E-003` bei 4 | wie erwartet |
| leere Eingabe | `E-001` bei 0 | wie erwartet |
| gueltiges `ATCB` statt Huelle | `E-007` bei 0 | wie erwartet, an beiden Eingaengen |

Jede Ablehnung laesst Zustand und Kette unveraendert (kein Zustand, kein Block);
das ist in jedem Fall mitgeprueft, nicht nur der Fehlercode.

## 4 Bytegleiche Rueckfuehrung

Der Umlauf `ATCB` → `ATCE` → `ATCB` ist bytegleich:

| Pruefung | Ergebnis |
|---|---|
| `Envelope::parse(container).into_artifact().to_bytes() == deployment.artifact_bytes` | gleich |
| Bytecode-Hash aus Quelle und aus Huelle | gleich |
| `artifact_hash` der Huelle gegen Bytecode-Hash | verschieden, wie vorgesehen |
| Ausfuehrung Quelle gegen Ausfuehrung Huelle | gleiches Ergebnis (6), gleiche Zustandswurzel, gleicher Zustand |

## 5 Befund F-1: die Registry fuehrt eine Freigabe je `code_hash`

Beim Test ist ein bestehendes Verhalten sichtbar geworden, das **nicht** durch
diese Arbeit entstanden ist: `deploy` aus der Quelle und `deploy_container` aus
der Huelle erzeugen fuer dasselbe Programm **verschiedene** Policies
(`source_provenance` unterscheidet sich: `atclang:nops` gegen `atce:nops`). Die
Registry ist auf `code_hash` geschluesselt, also ueberschreibt die zweite Freigabe
die erste; eine danach ausgefuehrte erste Freigabe scheitert mit `ADM-009`
("Policy-Bytes weichen von der kanonischen Codierung ab").

Bewertung: kein Sicherheitsverlust (beide Freigaben nennen dasselbe Programm und
sind gueltig signiert), aber eine zweite Aussage ueber denselben `code_hash`.
Nicht in diesem Paket geaendert: die Frage, welche Identitaet Freigabe und
Widerruf bindet, ist S1-2 und dort bereits entschieden; eine Zusammenfuehrung
waere eine eigene Aenderung an `atc_exec_admission`. Der Test bildet den Ist-Stand
ab (zwei getrennte Systeme) und nennt die Ursache im Kommentar.

## 6 Testevidenz

Ausgefuehrt im Arbeitsbaum auf `b1d45565`, `cargo 1.99.1`, offline.

| Pruefung | Ergebnis |
|---|---|
| `cargo test --workspace --offline` | **708 passed, 0 failed**, exit 0 (vorher 699) |
| `cargo test -p atc-system --offline` | **45 passed** (13 backend, 7 container_admission, 11 differential, 13 e2e, 11 signature) |
| `cargo fmt -p atc-system -- --check` | ohne Abweichung |
| `cargo clippy -p atc-system` | keine neue Warnung (die zwei Warnungen liegen in `atc-node` und sind alt) |
| `python3 -m pytest tools/tests` | 28 passed |
| `python3 tools/decision_register.py --check` | PASS |

## 7 Was ausdruecklich nicht geaendert wurde

* Keine neue VM-Logik, kein Opcode, keine Formatvariante.
* Kein Bytecode-Hash geaendert (Fixwerttest unveraendert gruen).
* Keine Regel des Kandidaten umgeschrieben.
* Keine Aussage zu `VERIFIED` oder `CI_VERIFIED`.
* Der Ketten-/Payload-Pfad ist unberuehrt (siehe F-1 der Vorlage: dort liegt die
  verbleibende Luecke, wenn die Huelle der einzige externe Eingang sein soll).

## 8 Statusgrenzen

| Gegenstand | Status |
|---|---|
| D-0 Formatentscheidung | `DECIDED` |
| Systemeingang ueber die Huelle | umgesetzt, lokal geprueft (`TESTED`) |
| `ATCE/0x02` | `OPEN` — Abschluss setzt die vier Architekturfragen und CI voraus |
| Differentialtest Huelle gegen Referenzdekoder | `BLOCKED` (Owner-Vorgabe), lokal 40/40 |
| `CI_VERIFIED` | nicht vergeben |
| P1a | `OPEN` |
| P2 | `BLOCKED` durch B-1 |

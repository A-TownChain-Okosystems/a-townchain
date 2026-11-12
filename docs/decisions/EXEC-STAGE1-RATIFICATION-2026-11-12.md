# Ratifikationsvermerk Stufe 1 — ENTWURF, NICHT IN KRAFT

**Status:** Entwurf. Dieses Dokument ist **keine** Ratifikation. Es beschreibt, was
eine Ratifikation aendern wuerde, und wird erst gueltig, wenn der Owner sie
ausdruecklich ausspricht und der Wortlaut unten mit dem Datum bestaetigt ist.
**Datum:** 2026-11-12 · **Basis:** `a-townchain` main `45f9d9093`
**Register:** unveraendert, Blob `9b76813663`

## 1. Warum dieser Entwurf existiert

Der Owner hat am 12.11.2026 eine Kombination als **technische Empfehlung**
bestaetigt und ausdruecklich festgehalten, dass damit **keine**
Eigentuemer-Ratifikation erfolgt ist. Eine Ratifikation ist ein Akt des Owners, den
der Agent weder vorwegnehmen noch ersetzen kann. Dieser Entwurf haelt fest, was auf
ein Wort des Owners hin eingetragen wird, damit die Uebernahme selbst nachpruefbar
bleibt und nicht aus einer Empfehlung abgeleitet wird.

## 2. Zwei Ebenen, die nicht dasselbe sind

| Ebene | Gegenstand | wer | Stand |
|---|---|---|---|
| 1 | Owner-Entscheidung S1-1 bis S1-4 (Optionen, Domaenenvariante) | Owner | offen |
| 2 | Ratifikation des Standards `ATC-SMART-CONTRACT-EXECUTION-001` nach §9 | Owner | offen, davon unabhaengig |

Ebene 1 legt fest, **welche** Werte gelten. Ebene 2 entscheidet, ob der Standard
als Ganzes in Kraft tritt. Eine Entscheidung auf Ebene 1 macht den Kandidaten nicht
zum ratifizierten Standard, und eine spaetere Ratifikation auf Ebene 2 kann die
Werte aus Ebene 1 nicht ersetzen, ohne sie ausdruecklich zu aendern.

## 3. Was eingetragen wird, wenn der Owner bestaetigt

| Gegenstand | Option | Wirkung im Register | Wirkung in der Spezifikation |
|---|---|---|---|
| S1-1 | A, drei Identitaeten | `artifact_hash`, `code_hash`, `state_root` getrennt gefuehrt | REQ-SCE-041 erhaelt eine zweite Identitaet, SCX-003 wird praezisiert |
| S1-1 Variante | 1b, beide Hashes mit Praefix | Domaenen-Strings und Containerfassung werden Pflicht | neue Fassung des Containers, Goldvektoren fuer beide Hashes |
| S1-2 | C, asymmetrisch | D2 traegt Widerruf auf zwei Identitaeten | getypte Schluesselraeume, zwei Abfragen, zwei Statuscodes |
| S1-3 | A, nur Gasrueckgabe | D4 bleibt bei Gas | `gas_refund` in Gas, kein Geldbetrag |
| S1-4 | B, nur L1 fuer P1a | keine Registeraenderung | L2 und L3 bleiben ausserhalb von P1a |

Zusaetzlich wird im Register von `pending_globally` entfernt, was durch die
Entscheidung entfaellt: **B-2** (Artefaktidentitaet). **B-1** und **B-3** bleiben
offen; B-3 nur, solange die Testschichtung nicht beschlossen ist.

## 4. Was auch nach einer Ratifikation nicht gilt

1. **P1a bleibt offen.** L1 ist `EMPTY`, weil der reine Dekoder fehlt. Eine
   Entscheidung ueber die Testschichtung erzeugt keinen Test.
2. **P2 bleibt gesperrt.** Die Aufrufarchitektur B-1 ist nicht entschieden; ohne
   sie gibt es keine `CALL`-Instruktion und keine Frame-Semantik.
3. **Keine Produktionsfreigabe.** Die Ratifikation von Entscheidungen ist keine
   Freigabe von Code.
4. **Kein Containerformat in Kraft.** 1b verlangt eine neue Containerfassung; sie
   muss spezifiziert, mit Goldvektoren belegt und erst dann umgesetzt werden.
5. **Der Standard bleibt Kandidat**, solange Ebene 2 offen ist.

## 5. Bedingungen, die mit der Ratifikation verbunden sind

1. `ATC-STATE-V2` ist unabhaengig von der Domaenenvariante erforderlich, weil die
   Wurzel kuenftig den Strom-Hash bindet und damit eine andere Commitment-Semantik
   hat. V1 wird nie stillschweigend als V2 gelesen.
2. S1-2 gilt nur als Ganzes: getypte Schluesselraeume, zwei Abfragen, zwei
   Statuscodes, Ablehnung bei Treffer auf einer der Sperren, keine Deutung flacher
   Altschluessel (S1-T-29 bis S1-T-32, S1-T-37, S1-T-38).
3. 1b aendert jeden Artefakt-Hash. Weil kein Artefakt-Hash gespeichert ist, ist die
   Neuverifikation vollstaendig; sobald einer gespeichert wird, gilt das nicht mehr.
4. Bei S1-3 ist "ungenutztes Budget verfaellt" heute folgenlos, weil kein Konto
   belastet wird. Der Wortlaut muss sagen, dass die Regel greift, sobald eine
   Belastung existiert.

## 6. Wortlaut, der zur Ratifikation fuehrt

Eine Ratifikation liegt vor, wenn der Owner diese vier Entscheidungen und die
Variante ausdruecklich als Entscheidung (nicht als Empfehlung) bestaetigt und dabei
Datum und Fassung nennt, zum Beispiel:

> Ich ratifiziere am 12.11.2026 auf Basis von main `45f9d9093`: S1-1 Option A mit
> Domaenenvariante 1b, S1-2 Option C, S1-3 Option A, S1-4 Option B.

Ohne einen solchen Satz bleibt dieses Dokument Entwurf, das Register unveraendert
und Aufgabe 910 offen.

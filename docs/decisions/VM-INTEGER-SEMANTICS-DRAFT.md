---
document_id: ATC-DOC-DECISION-DRAFT-004
title: "Ganzzahl-Semantik für Vertragsausführung — Entwurf der verbindlichen Festlegung"
version: 0.1.0
status: draft
owner: A-TownChain-Okosystems
created: 2026-11-11
updated: 2026-11-11
standard: ATC-STD-MD-001
---

# Ganzzahl-Semantik — Entwurf

**Status: ENTWURF. Dieses Dokument entscheidet nichts.** Es legt die Fragen offen,
benennt die Folgen jeder Antwort und schlaegt eine Fassung zur Ratifizierung vor.
Bis zur Ratifizierung gilt: es gibt **keine** stille Normalisierung zwischen Compiler
und VM, und die Divergenz bleibt sichtbar (`atc_system::DIVERGENZEN`, zwei Tests in
`components/system/tests/differential.rs`).

## 1 — Der Befund, der den Entwurf erzwingt

| Stelle | heute | Folge |
|---|---|---|
| `atc-core` (Referenz) | `i64`, gepruefte Arithmetik | Ueberlauf ist ein Fehler (`ArithmeticOverflow`) |
| `atc-vm` (Kette) | `u64`, umlaufend | Ueberlauf ist ein stiller Wert |
| `Lt` | Referenz vorzeichenbehaftet, VM ohne Vorzeichen | Vergleich kehrt sich bei negativen Werten um |

Belegte Faelle: `2^62 * 4` → Referenz lehnt ab, VM liefert 0. `0 - 1 < 1` → Referenz 1,
VM 0. Ein Vertrag, der im Referenzweg abgelehnt wird, kann auf der Kette einen anderen
Zustand erzeugen. Damit sind Konsens-Ergebnisse nicht mehr durch die Referenz gedeckt.

## 2 — Die fuenf Fragen, die beantwortet werden muessen

### F1 — Bitbreite
**Vorschlag: 64 Bit, keine Ausnahme.** Kein 128-Bit, keine Zwischenbreite. Begruendung:
der Vertragsraum 0 und 0x0100..0x01FF sowie die Gasrechnung sind auf 64 Bit ausgelegt.

### F2 — Signedness
**Vorschlag: Zweierkomplement, Arithmetik und alle Vergleiche vorzeichenbehaftet.**
Es gibt genau eine Interpretation derselben 64 Bit, nicht zwei. Ein "unsigned Modus"
waere eine zweite Semantik und damit genau das, was diese Entscheidung verhindern soll.

### F3 — Ueberlaufverhalten
**Vorschlag: Abbruch (Trap), nie Umlauf, nie Saettigung.**
Betroffen: `Add`, `Sub`, `Mul` (und `Neg`, sobald es einen Befehl gibt). Begruendung:
Umlauf macht aus einem abgelehnten Programm ein laufendes mit anderem Zustand;
Saettigung verschleiert den Fehler als Ergebnis. Ein Trap ist fail-closed und
deterministisch.

### F4 — Division und Rest
**Vorschlag: ganzzahlige Division mit Abbruch zur Null, Rest nach der Regel
`r = a - (a / b) * b`, also mit dem Vorzeichen des Dividenden.**
Grenzfaelle: `b == 0` → Trap. `a == i64::MIN` und `b == -1` → Trap (das Ergebnis ist
nicht darstellbar). Kein Float, kein Rundungsmodus, keine Sonderregel je Vorzeichen.

### F5 — Fehlerverhalten an Grenzwerten
**Vorschlag: ein eigener, numerisch gekennzeichneter Fehlercode je Ursache**
(Ueberlauf, Division durch Null, nicht darstellbares Ergebnis), mit Rollback wie jeder
andere Fehler. Der Code darf nicht mit dem Stapel- oder Speichergrenzencode kollidieren;
die Kennung wird in derselben Inventur vergeben wie die uebrigen Fehlercodes.

## 3 — Was daraus folgt (Arbeitsumfang, nicht Teil dieses Entwurfs)

1. **Container-Generation.** Neue Befehle sind eine Aenderung am ATCB-Vertrag, also eine
   neue Generation. `PushI64` (negative Konstante) und die geprueften Varianten von
   `Add`/`Sub`/`Mul`/`Div` sowie ein vorzeichenbehaftetes `Lt` existieren heute nicht.
   Aeltere Artefakte behalten Generation und Semantik.
2. **Gastabelle** als neue Tabellenversion; die Kosten der neuen Befehle werden
   eingetragen, nicht geschaetzt.
3. **Backend** in `components/system`: `Neg`, `Pop`, negative Konstanten und `call`
   werden nicht mehr abgelehnt, sondern abgebildet (sobald es Befehle dafuer gibt).
4. **Referenz und VM** muessen in einem Differentialtest deckungsgleich sein, inklusive
   der Trap-Faelle: gleicher Fehler, gleiche Stelle, gleicher Zustand nach Rollback.
5. **Divergenzmarker** `DIVERGENZEN` wird erst entfernt, wenn Punkt 4 fuer alle Faelle
   belegt ist. Vorher bleibt er stehen, auch wenn die neuen Befehle existieren.

## 4 — Testmatrix fuer die Umsetzung

| Fall | Erwartung |
|---|---|
| `i64::MAX + 1` | Trap, kein Umlauf, Zustand unveraendert |
| `i64::MIN - 1` | Trap |
| `i64::MAX * 2` | Trap |
| `5 / 0` | Trap |
| `i64::MIN / -1` | Trap |
| `5 / 2`, `-5 / 2`, `5 / -2`, `-5 / -2` | 2, -2, -2, 2 (Abbruch zur Null) |
| `5 % 2`, `-5 % 2` | 1, -1 (Vorzeichen des Dividenden) |
| `0 - 1 < 1` | 1 |
| `-1 < 0` | 1 |
| gleiche Quelle in beiden Wegen | identisches Ergebnis **und** identischer Zustand |
| gleicher Trap in beiden Wegen | identischer Code, identischer Zustand nach Rollback |

## 5 — Empfehlung

F1 bis F5 wie vorgeschlagen ratifizieren, dann in der Reihenfolge Container-Generation,
Gastabelle, VM-Befehle, Backend, Differentialtest umsetzen. Die Alternative (die VM
behaelt Umlauf und der Referenzweg wird angepasst) ist billiger, macht aber die
Referenz zum Nachlaeufer der Implementierung; das widerspricht der bestehenden
Beweisrichtung.

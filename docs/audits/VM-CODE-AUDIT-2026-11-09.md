# VM-CODE-AUDIT — ATC-VM (ATC-BC-001 / ATC-BC-002 Kandidat)

**Datum:** 2026-11-09 · **Art:** read-only Pruefung, keine Codeaenderung
**Repo:** A-TownChain-Okosystems/a-townchain · **Branch:** main
**HEAD:** 6504d45 (Klon frisch, `cargo test -p atc-vm` 70 Tests gruen)
**Umfang:** components/vm (vm.rs, ops.rs, context.rs, assembler.rs, main.rs)

Ein gruener Testlauf ist kein Nachweis. Jeder Befund unten hat einen
Reproduktionspfad; die Reproduktion lief in einer temporaeren Testdatei, die
nach dem Lauf geloescht wurde (Baum unveraendert).

## Befunde

### VM-AUD-003-A — Roher `run()`-Pfad umgeht fuenf Sicherheitsgrenzen
**Prioritaet:** P0 · **Status:** RESIDUAL
**Stelle:** `components/vm/src/vm.rs`, `Vm::run` (Zeile ~362)
**Beschreibung:** `run()` ist `pub`, ruft `verify()` nicht auf, kennt kein Gas,
prueft weder `MAX_STACK_ITEMS` (Push/Dup/Load schreiben direkt in den Stapel)
noch `MAX_STORAGE_SLOTS`, und sichert keinen Snapshot.
**Reproduktion (ausgefuehrt):**
- 5000x `Push(1)` + `Halt` -> `run()` liefert 5000 Elemente, Grenze ist 4096.
- `Push(7), Store(0), Add` -> `Err(StackUnderflow)`, Zustand danach `[7]`.
- `Push(1), Store(70000), Halt` -> `run()` ok, 70001 Speicherplaetze (Grenze 65536).
**Wirkung:** Wer den direkten Pfad nutzt, erhaelt unbegrenzten Speicherwachstum
und Teilzustaende nach Fehlern. Ein Kommentar im Code sagt "for local/unit use",
aber nichts erzwingt das.
**Korrektur:** `run()` entweder entfernen, `pub(crate)` setzen oder auf
`run_with_gas(u64::MAX)` abbilden, damit eine Grenze gilt.

### VM-AUD-001-A — Generation ist Aufruferangabe, nicht Artefakteigenschaft
**Prioritaet:** P0 · **Status:** RESIDUAL
**Stelle:** `Vm::with_generation` (Zeile ~75), `Vm::verify` (v2-Gate)
**Beschreibung:** Die Generation kommt aus dem Aufrufer, nicht aus dem Artefakt.
`run()` prueft sie gar nicht: ein v1-Programm mit `Assert` liefert dort `Ok([])`,
waehrend derselbe Baum ueber `run_with_gas` korrekt `Err(V2OpInV1)` gibt.
**Reproduktion (ausgefuehrt):** `run()` = `Ok([])`, `run_with_gas(100)` =
`Err(V2OpInV1)` fuer identisches Programm.
**Wirkung:** Die Versionsgrenze aus ATC-BC-002 Abschnitt 2 ist auf dem rohen
Pfad nicht durchgesetzt.
**Korrektur:** Generation aus dem Artefaktkopf ableiten und im Konstruktor
setzen; `with_generation` nur fuer Tests, nicht `pub`.

### VM-AUD-001-B — Generation ohne Wertebereich
**Prioritaet:** P1 · **Status:** RESIDUAL
**Stelle:** `Vm::verify`, Bedingung `self.generation < 2`
**Beschreibung:** Jeder Wert ungleich 1 gilt als v2. `with_generation(9)` ist
erlaubt und schaltet die v2-Befehle frei.
**Reproduktion (ausgefuehrt):** Generation 9 mit `Assert` -> `Ok([])`.
**Korrektur:** Nur 1 und 2 zulassen, sonst `InvalidProgram`.

### VM-AUD-002-A — Verifier akzeptiert nicht ausfuehrbare Speicherplaetze
**Prioritaet:** P1 · **Status:** RESIDUAL
**Stelle:** `Vm::verify` prueft `self.storage.len()`, nicht die Slots im Programm
**Beschreibung:** `Store(70000)` uebersteht `verify()` (`Ok(())`), scheitert erst
in der Ausfuehrung mit `StorageLimitExceeded`.
**Reproduktion (ausgefuehrt):** `verify() = Ok(())`, `run_with_gas` =
`Err(StorageLimitExceeded { requested: 70001, limit: 65536 })`.
**Wirkung:** Ein Artefakt ist gueltig und trotzdem nie ausfuehrbar; Deployment
und Ausfuehrung urteilen verschieden.
**Korrektur:** Maximalen Slot aus dem Programm lesen und im Verifier begrenzen.

### VM-AUD-002-B — Stapelpruefung im Verifier ist wirkungslos
**Prioritaet:** P2 · **Status:** RESIDUAL
**Stelle:** `Vm::verify`, `if self.stack.len() > MAX_STACK_ITEMS`
**Beschreibung:** Der Verifier laeuft vor der Ausfuehrung, der Stapel ist dort
leer. Die Pruefung kann nie ausloesen; eine Pfadanalyse der Stapeltiefe fehlt.
**Korrektur:** Tiefe je Sprungziel rechnen (Fixpunkt) oder Feld leeren und im
Bericht als nicht implementiert fuehren.

### VM-AUD-002-C — Code nach HALT: v1 toleriert, v2 nicht
**Prioritaet:** P2 · **Status:** VERIFIED (als Kompatibilitaetsverhalten)
**Stelle:** `Vm::verify`, Block "nur v2"
**Beschreibung:** Die Unerreichbarkeitspruefung greift nur ab Generation 2.
v1-Programme mit Code nach `HALT` laufen weiter (`Ok([])`, Test
`v1_ignoriert_v2_strenge_nach_halt`). Das ist gewollt und getestet.

### VM-AUD-003-B — Zwei kopierte Interpreter-Schleifen
**Prioritaet:** P2 · **Status:** RESIDUAL
**Stelle:** `run_with_gas_inner` und `run`
**Beschreibung:** Beide Schleifen enthalten dieselben 20+ Arme, doppelt gepflegt.
Die v2-Pruefungen wurden zweimal eingebaut; jede kuenftige Aenderung muss beide
treffen. Die Divergenz aus VM-AUD-003-A ist genau so entstanden.
**Korrektur:** Eine Schleife mit Parameter `Option<&mut u64>` fuer Gas.

### VM-AUD-004-A — Doppelte Verifikation
**Prioritaet:** P3 · **Status:** VERIFIED
**Stelle:** `execute_state_transition_with_gas` -> `verify` + `run_with_gas` -> `verify`
**Wirkung:** Nur Kosten, keine Sicherheitsluecke. Reihenfolge ist korrekt
(Verifikation vor jeder Zustandsaenderung).

### VM-AUD-004-B — Gastabelle ohne Bindung
**Prioritaet:** P1 · **Status:** RESIDUAL
**Stelle:** `gas_cost` (Zeile 69)
**Beschreibung:** Kosten sind im Code hart verdrahtet, es gibt keine Tabelle mit
Versionsbindung. ATC-BC-002 fordert deterministische, versionierte Preise. Ein
Test belegt nur `Assert = 2`, nicht die Gleichheit zwischen Verifier- und
Interpreter-Sicht.
**Korrektur:** Kostentabelle als Daten mit Generationskennung, Test ueber alle
Ops, Abweichung = Fehler.

### VM-AUD-005-A — Snapshot deckt nur Stapel und Speicher
**Prioritaet:** P0 · **Status:** RESIDUAL
**Stelle:** `run_with_gas` (Snapshot), `run` (kein Snapshot)
**Beschreibung:** Gesichert werden `stack` und `storage`. Nicht gesichert:
`sender`/Salden, Ereignisse, verschachtelte Frames, Rueckgabewerte (existieren
im Crate nicht), und im rohen Pfad gar nichts.
**Reproduktion (ausgefuehrt):** `run()` hinterlaesst nach Fehler `[7]` im Zustand;
ueber `run_with_gas` bleibt der Zustand leer (Test
`revert_bricht_unbedingt_ab`, `revertif_wahr_nach_store_kein_zustand`).
**Wirkung:** Frame-Rollback ist fuer Stapel und Speicher belegt, fuer Salden und
Ereignisse nicht, weil es sie nicht gibt. Transaktions-Rollback ist nicht belegt.
**Korrektur:** Erst nach Einfuehrung von Salden und Ereignissen erweitern; bis
dahin im Standard nicht als vollstaendig ausweisen.

### VM-AUD-006-A — v1-Regressionsbasis zu schmal
**Prioritaet:** P1 · **Status:** RESIDUAL
**Stelle:** `tests/atcb_v2.rs` (3 v1-Tests), `tests/exec_chain_e2e.rs`
**Beschreibung:** Belegt sind Arithmetik, Gasuntergrenze und HALT-Toleranz. Es
fehlen: Golden-Gaswerte je v1-Op, byte-identische Artefakte vor/nach der
Aenderung, Fehlerpfadvergleich.
**Korrektur:** Golden-Vektor mit Erwartungswerten je Op, im CI gegen main gehalten.

### VM-AUD-007-A — Fehler tragen keine Codes
**Prioritaet:** P1 · **Status:** RESIDUAL
**Stelle:** `enum VmError`
**Beschreibung:** ATC-BC-002 Abschnitt 5 definiert E01-E06 als Teil des
Ergebnisses. Im Code existieren Varianten ohne Nummer; `AssertFailed` hat gar
keinen Code, `Reverted(0)` ist der allgemeine Fall.
**Wirkung:** Zwei Implementierungen koennen gleiches Verhalten, aber
unterschiedliche Berichte liefern; Differentialtests greifen zu kurz.
**Korrektur:** `fn code(&self) -> u16` auf `VmError`, Mapping E01-E06, Test.

### VM-AUD-007-B — REVERTIF zieht vor der Code-Pruefung
**Prioritaet:** P2 · **Status:** RESIDUAL
**Stelle:** `Op::RevertIf`-Arm, beide Schleifen
**Beschreibung:** Beide Werte werden gezogen, danach wird der Code geprueft. Bei
ungueltigem Code ist der Stapel bereits veraendert; ueber `run_with_gas` stellt
der Snapshot ihn wieder her, ueber `run()` nicht.
**Korrektur:** Code vor dem Ziehen der Bedingung pruefen (Spitze lesen, dann
ziehen), dann ist der Fehlerpfad ohne Zustandsaenderung.

## Verifier-Umgehung (ausdruecklicher Pruefpunkt)

Ein Artefakt **kann** ohne `verify()` ausgefuehrt werden: `Vm::run()` ist
oeffentlich. Dann gelten **keine** Garantien: keine Generationstrenge, kein Gas,
keine Stapel- oder Speichergrenze, kein Rollback. Der einzige Pfad mit
Garantien ist `execute_state_transition_with_gas` (Gate + verify + Snapshot).

## Gas-Konsistenz (ausdruecklicher Pruefpunkt)

Eine Kostentabelle existiert nur als Funktion im Code. Verifier und Interpreter
lesen dieselbe Funktion, eine Abweichung ist heute also nicht moeglich, aber
auch nicht abgesichert: kein Test vergleicht die Kosten aller Ops, keine
Versionsbindung, keine Tabelle als Artefakt. Bei der geplanten Nested-Execution
mit Teilbudgets wird das zur Fehlerquelle.

## Snapshot-Vollstaendigkeit (ausdruecklicher Pruefpunkt)

| Zustand | im Snapshot | Anmerkung |
| --- | --- | --- |
| Stapel | ja | `run_with_gas` |
| Speicherplaetze | ja | `run_with_gas` |
| Aufrufer | nein | unveraenderlich, derzeit unkritisch |
| Salden | nein | existieren im Crate nicht |
| Ereignisse | nein | existieren im Crate nicht |
| verschachtelte Frames | nein | nicht implementiert |
| Gasverbrauch | nein | gewollt: keine Erstattung |
| roher `run()`-Pfad | nein | VM-AUD-003-A |

## Zusammenfassung

| ID | Bereich | Prio | Status |
| --- | --- | --- | --- |
| VM-AUD-001-A | Generationsumgehung ueber run() | P0 | RESIDUAL |
| VM-AUD-003-A | run() ohne Grenzen und Snapshot | P0 | RESIDUAL |
| VM-AUD-005-A | Snapshot nur Stapel/Speicher | P0 | RESIDUAL |
| VM-AUD-001-B | Generation ohne Wertebereich | P1 | RESIDUAL |
| VM-AUD-002-A | Verifier akzeptiert tote Slots | P1 | RESIDUAL |
| VM-AUD-004-B | Gastabelle ohne Bindung | P1 | RESIDUAL |
| VM-AUD-006-A | v1-Regression zu schmal | P1 | RESIDUAL |
| VM-AUD-007-A | Fehler ohne Codes | P1 | RESIDUAL |
| VM-AUD-002-B | Stapelpruefung wirkungslos | P2 | RESIDUAL |
| VM-AUD-003-B | doppelte Schleifen | P2 | RESIDUAL |
| VM-AUD-007-B | REVERTIF zieht vor Pruefung | P2 | RESIDUAL |
| VM-AUD-002-C | HALT-Toleranz in v1 | P2 | VERIFIED |
| VM-AUD-004-A | doppelte Verifikation | P3 | VERIFIED |

**Urteil:** Keine der drei v2-Instruktionen hat einen Semantikfehler; ASSERT,
REVERT und REVERTIF verhalten sich wie in Abschnitt 15 festgelegt, und der
Ruecklauf stellt Stapel und Speicher wieder her. Die Luecken liegen an den
Raendern: der rohe Pfad ohne Grenzen, die Generation als Aufruferangabe, der
Verifier, der tote Slots durchlaesst, und Fehler ohne Codes.

**Reihenfolge der Korrekturen:** 1. VM-AUD-003-A (Grenzen auf allen Pfaden),
2. VM-AUD-001-A/B (Generation aus Artefakt, Wertebereich), 3. VM-AUD-005-A
(Snapshot-Abgrenzung dokumentieren), 4. VM-AUD-002-A, 5. VM-AUD-007-A,
6. VM-AUD-006-A, danach Nested Execution.

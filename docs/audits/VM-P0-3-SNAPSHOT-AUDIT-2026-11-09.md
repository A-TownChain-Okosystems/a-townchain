# P0-3 — Snapshot- und Rollback-Audit (read-only)

**Datum:** 2026-11-09 · **Art:** read-only, keine Codeaenderung
**Repo:** A-TownChain-Okosystems/a-townchain · **Branch:** main · **HEAD:** 8684065
**Gegenstand:** Geltungsbereich des Snapshots, Fehlerpfade, Zustandserhaltung

Methode: Quelltextpruefung von `components/vm/src/vm.rs` am HEAD, Abgleich mit
den vorhandenen Tests. Keine Aussage ohne Codefundstelle oder Test.

## 1. Was der Snapshot tatsaechlich tut

Fundstelle: `Vm::execute_transactional`.

1. `verify()?` — findet VOR jeder Zustandsaenderung statt. Ein Verifikations-
   fehler beruehrt den Zustand nicht, ein Rollback ist dort nicht noetig.
2. `original_stack = self.stack.clone()`, `original_storage = self.storage.clone()`.
3. `run_loop(...)`.
4. `Ok` -> Zustand bleibt bestehen, Stapel wird entnommen und zurueckgegeben.
5. `Err` -> `self.stack` und `self.storage` werden durch die Klone ersetzt.

Damit ist die Garantie exakt: **Stack und Storage werden bei jedem `Err` aus der
Ausfuehrung vollstaendig zurueckgenommen.** Mehr nicht.

## 2. Geltungsbereich, Zustand fuer Zustand

| Zustand | Stand | Begruendung (Codefund) |
| --- | --- | --- |
| Operand Stack | SNAPSHOT | `original_stack`, ersetzt bei `Err` |
| Storage (Speicherplaetze) | SNAPSHOT | `original_storage`, ersetzt bei `Err` |
| Aufrufer (`caller`) | EXCLUDED, unkritisch | nur gelesen (`Op::Caller`, Zeile 338), im gesamten Crate keine Schreibstelle |
| Gasverbrauch | EXCLUDED, gewollt | keine Erstattung; Budget wird als Wert uebergeben |
| Kontostaende | EXCLUDED | existieren im Crate nicht (0 Treffer) |
| Events | EXCLUDED | existieren im Crate nicht (0 Treffer) |
| Verschachtelte Frames | EXCLUDED | nicht implementiert, keine Frame-Verwaltung |
| Externe Seiteneffekte | EXCLUDED | das Crate hat keine Ein-/Ausgabe nach aussen |
| Rueckgabewerte | EXCLUDED | entstehen erst mit Rueckgabewert-Unterstuetzung |

## 3. Frame-Rollback und Transaction-Rollback sind nicht dasselbe

Heute gibt es **genau einen** Ausfuehrungsrahmen. Der Snapshot ist damit
transaktionsweit und deckungsgleich mit dem einzigen Rahmen.

- **Transaction-Rollback (heute belegt):** alle Aenderungen an Stack und Storage
  eines Aufrufs werden bei Fehler zurueckgenommen.
- **Frame-Rollback (heute EXCLUDED):** setzt mehrere Rahmen mit eigenem Zustand
  voraus. Ohne verschachtelte Aufrufe existiert kein innerer Rahmen, dessen
  Aenderungen einzeln zurueckzunehmen waeren.

Die beiden Garantien werden hier ausdruecklich NICHT gleichgesetzt. Wer
verschachtelte Aufrufe einfuehrt, muss Frame-Rollback neu bauen und belegen;
der bestehende Snapshot traegt ihn nicht.

## 4. Fehlerpfade

| Pfad | Rollback | Nachweis |
| --- | --- | --- |
| Verifikationsfehler | nicht noetig, keine Aenderung | `p0_1_ungueltiges_sprungziel_wird_vor_ausfuehrung_abgelehnt` |
| StackUnderflow nach `Store` | ja | `p0_1_kein_teilzustand_nach_fehler_ohne_gasbudget` |
| Gas erschoepft nach `Store` | ja | `gas_exhaustion_after_mutation_rolls_back_state` |
| `REVERT` nach `Store` | ja | `revert_bricht_unbedingt_ab` |
| `REVERTIF` nach `Store` | ja | `revertif_wahr_nach_store_kein_zustand` |
| `ASSERT` nach `Store` | **kein Test** | Luecke, siehe Befund P0-3-F3 |
| Speicherwachstum dann Fehler | nur indirekt | Luecke, siehe P0-3-F3 |
| Panik oder Abbruch | nein | siehe P0-3-F2 |
| Erfolg | Zustand bleibt, gewollt | `commit`-Pfad, `Ok` |

## 5. Befunde

### P0-3-F1 — Gasverbrauch wird nicht berichtet
**Prioritaet:** P2 · **Status:** RESIDUAL
`run_with_gas` nimmt das Budget als Wert und gibt es nicht zurueck. Nach einem
`OutOfGas` ist von aussen nicht erkennbar, wie viel verbraucht wurde. Fuer
Konsensabrechnung und Fehlercode E06 ist das zu wenig.
**Korrektur:** Rueckgabe von verbrauchtem Gas oder ein Ergebnisobjekt mit
Restbudget. Nicht Teil von P0-3.

### P0-3-F2 — Keine Panik-Sicherheit
**Prioritaet:** P2 · **Status:** RESIDUAL (dokumentierte Grenze)
Der Rollback greift nur bei `Err`. Eine Panik oder ein Abbruch (z.B.
Speichermangel beim Klonen) laesst den Zustand stehen. Im Ausfuehrungsloop gibt
es dafuer aktuell keinen Ausloeser: die einzige Indexschreibung ist durch
`resize` und die Slot-Grenze gedeckt, Arithmetik ist umhuellend. Ein
`catch_unwind` waere teuer und wuerde eine Panik nicht zu einem definierten
Ergebnis machen.
**Korrektur:** keine. Als Grenze im Standard festhalten.

### P0-3-F3 — Zwei Rollback-Faelle ohne eigenen Test
**Prioritaet:** P2 · **Status:** RESIDUAL
`ASSERT` nach `Store` und Speicherwachstum mit anschliessendem Fehler sind durch
den gemeinsamen Pfad gedeckt, aber nicht einzeln belegt. Zwei Tests schliessen
die Luecke ohne Codeaenderung.

### P0-3-F4 — Snapshot ist eine Vollkopie
**Prioritaet:** P3 · **Status:** VERIFIED (Kosten, keine Sicherheitsluecke)
`clone()` kopiert Stack und Storage vollstaendig. Bei Volllast: Storage
65536 x 8 B = 512 KiB, Stack 4096 x 8 B = 32 KiB, zusammen 544 KiB je Aufruf
(Korrektur nach P0-Gesamtaudit: die frueheren "rund 512 KB" waren nur der
Storage-Anteil). Korrekt, aber teuer; fuer Nested Execution mit
Teilbudgets wird das zum Thema.

## 6. Urteil

Der Snapshot deckt genau zwei Zustandsbereiche, und das ist belegt. Alles
andere ist entweder nicht vorhanden (Kontostaende, Events, Rahmen) oder
ausdruecklich ausgenommen (Gas, Panik, externe Effekte). Es wurde nichts
kuenstlich unterstuetzt, was es nicht gibt.

**Status P0-3:** dokumentiert und abgegrenzt; zwei Testluecken offen (F3).
Die Garantie lautet: Transaktions-Rollback fuer Stack und Storage. Sie lautet
nicht: vollstaendiger Rollback einer Transaktion im Sinne der Blockchain.

# Paket B — Bestandsaufnahme Gas und Fehlercodes (read-only)

**Datum:** 2026-11-09 · **Art:** read-only. Keine Code-, Test- oder
Spezifikationsaenderung, keine neue Gaslogik, keine Umnummerierung.
**Repo:** A-TownChain-Okosystems/a-townchain
**main HEAD:** `21adb7919d52550f1550c6722444ac9f6a5fa2fb` (Kurzform 21adb79)
**Geprueft:** `components/vm/src/{vm.rs,ops.rs,artifact.rs,context.rs,main.rs}`,
`components/vm/tests/*`, `standards/atc/candidates/ATC-BC-002-CANDIDATE.md`

## 1. Opcode-Liste und Kosten (Ist-Zustand)

Quelle: `fn gas_cost(&Op) -> u64` in `src/vm.rs`, einzige Definition.

| Opcode | Kosten | Sonderfall |
| --- | --- | --- |
| Push | 1 | operandentragend |
| Dup | 1 | |
| Swap | 1 | |
| Caller | 1 | |
| Add | 2 | |
| Sub | 2 | |
| Mul | 2 | |
| Div | 2 | Division durch 0 ist Laufzeitfehler nach Belastung |
| Eq | 2 | |
| Lt | 2 | |
| Load | 3 | |
| Store | 3 | **flach, unabhaengig vom Speicherwachstum** |
| Jump | 2 | |
| JumpIfZero | 2 | |
| JumpIfNotZero | 2 | |
| Halt | 0 | kostenlos, beendet |
| Assert | 2 | v2 |
| Revert | 1 | v2, codeabhaengig keine Mehrkosten |
| RevertIf | 3 | v2 |

19 von 19 Varianten sind abgedeckt. Das `match` hat keinen Default-Arm; ein
neuer Befehl ohne Kosten bricht den Build. Die Tabelle ist damit durch den
Compiler vollstaendig, nicht durch einen Test.

## 2. Aufrufstellen

`gas_cost` wird an genau einer Stelle benutzt: `Vm::run_loop`, Zeile 284, vor
der Ausfuehrung des Befehls. `verify()` und das Artefakt kennen keine Kosten.
Damit gibt es keine statische Kostenschranke fuer ein Deployment.

## 3. Befunde Gas

### GAS-AUD-001 — Tabelle vollstaendig, aber ungetestet je Opcode
**Fundstelle:** `vm::gas_cost` · **Ist:** exhaustive match, 19 Ops
**Spezifikationsbezug:** BC2 fordert "Gas-Tabelle versioniert und vollstaendig
getestet" · **Testabdeckung:** keine per-Op-Zusicherung, nur generische
OutOfGas-Faelle · **Abweichung:** Vollstaendigkeit ist compiler-, nicht
testgestuetzt · **Empfehlung:** je Opcode ein Kostentest plus Summenprobe
· **Status:** IMPLEMENTED

### GAS-AUD-002 — Keine Versionierung der Tabelle
**Fundstelle:** `vm::gas_cost` (keine Konstante, keine Version) · **Ist:** Werte
stehen im Code, ohne Kennung · **Spezifikationsbezug:** fehlend
· **Testabdeckung:** nein · **Abweichung:** Eine Kette kann nicht feststellen,
mit welcher Kostentabelle sie einen Block validiert hat
· **Empfehlung:** Kostentabelle als benannte, versionierte Konstante
(GAS_TABLE_V1) und Version im Artefakt- oder Kontextbezug
· **Status:** RESIDUAL

### GAS-AUD-003 — Ohne Budget existiert keine Schrittobergrenze
**Fundstelle:** `Vm::run_loop` mit `gas_limit: None` · **Ist:** `pc` springt
unbegrenzt; `Jump(0)` laeuft endlos, weil kein Zaehler und keine Grenze greift
· **Spezifikationsbezug:** fehlend · **Testabdeckung:** nein
· **Abweichung:** `run()` ist ein DoS-Vektor: ein Programm ohne Verzweigung nach
vorn blockiert den Aufrufer dauerhaft · **Empfehlung:** Schrittobergrenze oder
Pflichtbudget auch fuer `run()` · **Status:** RESIDUAL (P1)

### GAS-AUD-004 — Speicherwachstum wird nicht bepreist
**Fundstelle:** `Op::Store`, Kosten 3 · **Ist:** `Store(1000)` waechst den
Speicher auf 1001 Plaetze, kostet aber wie `Store(0)` · **Spezifikationsbezug:**
fehlend · **Testabdeckung:** nein · **Abweichung:** Speicherkosten sind nicht an
das Wachstum gebunden, nur an die Obergrenze · **Empfehlung:** Wachstumskosten
je neuem Slot, als eigene Tabellenzeile
· **Status:** RESIDUAL

### GAS-AUD-005 — Verbrauch bei Erfolg wird nicht berichtet
**Fundstelle:** `execute_transactional` · **Ist:** `Ok` liefert nur den Stapel;
`OutOfGas` nennt `required` und `remaining`, sonst nichts
· **Spezifikationsbezug:** E03 fordert "Gas vollstaendig verbraucht"
· **Testabdeckung:** nein · **Abweichung:** Ein Block kann die Abrechnung nicht
nachpruefen, weil der Verbrauch nach Erfolg unbekannt ist
· **Empfehlung:** Ergebnisobjekt mit verbrauchtem Gas
· **Status:** RESIDUAL

### GAS-AUD-006 — Belastung erfolgt vor der Wirkung (spec-konform)
**Fundstelle:** `run_loop` Zeilen 284–292 · **Ist:** Kosten werden geprueft und
abgezogen, bevor der Befehl wirkt · **Spezifikationsbezug:** BC2 fordert genau
das · **Testabdeckung:** indirekt ueber `gas_exhaustion_after_mutation_rolls_back_state`
· **Abweichung:** keine · **Empfehlung:** keine · **Status:** TESTED

### GAS-AUD-007 — HALT kostet 0 (spec-konform)
**Fundstelle:** `gas_cost`, `Op::Halt => 0` · **Ist:** Abschluss kostenlos
· **Spezifikationsbezug:** BC2 fuehrt HALT ohne Kosten · **Testabdeckung:** ja
(`explicit_gas_budget_executes_deterministically`) · **Abweichung:** keine
· **Empfehlung:** keine · **Status:** TESTED

## 4. Fehlercodes — Inventar

**Interne VM-Fehler** (`VmError`, 14 Varianten, keine numerischen Codes):
StackUnderflow, StackLimitExceeded, StorageLimitExceeded{requested,limit},
ProgramTooLarge{size,limit}, InvalidJump(usize), DivisionByZero,
OutOfGas{required,remaining}, InvalidProgram(&'static str),
Context(ContextError), AssertFailed, Reverted(u64), InvalidRevertCode(u64),
V2OpInV1, UnreachableCode(usize).

**Verifier-Fehler:** InvalidProgram, InvalidJump, V2OpInV1, UnreachableCode,
ProgramTooLarge, StorageLimitExceeded, InvalidRevertCode.

**Interpreter-Fehler:** StackUnderflow, StackLimitExceeded, DivisionByZero,
OutOfGas, AssertFailed, Reverted, StorageLimitExceeded, InvalidRevertCode.

**Kontext** (`ContextError`, 6): InvalidChainId, InvalidNetworkId,
InvalidGenesisId, ProtocolMismatch, VmMismatch, GenesisMismatch.

**Artefakt** (`ArtifactError`, 10): TooShort, BadMagic, UnsupportedContainer,
UnknownGeneration, BadHash, UnknownOpcode, GenerationMismatch,
TruncatedOperand, TrailingBytes, ProgramTooLarge.

**Vertrags-Ruecklaufcodes:** 0 = allgemein, 0x0100–0x01FF = Vertragsraum,
geprueft in `gueltiger_ruecklauf_code`. 0x00FF und 0x0200 werden von Verifier
und Interpreter abgelehnt, damit kein Pfad einen Code vorbeischmuggelt.

**E01–E06 in der Spezifikation:** E01 Pruefung fehlgeschlagen, E02 Ruecklauf,
E03 Gas erschoepft, E04 Stapelgrenze, E05 Speichergrenze, E06 unbekannter
Befehl beziehungsweise Aufruftiefe. Keine dieser Kennungen existiert im Code.

## 5. Befunde Fehler

### ERR-AUD-001 — Keine numerischen Codes im Code
**Fundstelle:** `enum VmError` · **Ist:** 14 Varianten, nur Rust-Namen
· **Spezifikationsbezug:** E01–E06 vorhanden, Abbildung fehlt
· **Testabdeckung:** nein · **Abweichung:** Ein Beobachter kann Fehler nicht
stabil klassifizieren; Debug-Strings sind keine Schnittstelle
· **Empfehlung:** Abbildung Variante auf Code, ohne Umnummerierung bestehender
Bedeutungen · **Status:** RESIDUAL

### ERR-AUD-002 — Spezifikation widerspricht sich bei Gas
**Fundstelle:** `ATC-BC-002` Zeile 47 gegen Zeile 124 · **Ist:** Zeile 47 nennt
E03 fuer Gas erschoepft, Zeile 124 fordert bei unzureichendem Gas einen
Ruecklauf mit E04 · **Spezifikationsbezug:** selbstwiderspruechlich
· **Testabdeckung:** nicht anwendbar · **Abweichung:** zwei Kennungen fuer
denselben Sachverhalt, E04 ist zugleich Stapelgrenze
· **Empfehlung:** eine Kennung fuer Gas, normativ entscheiden
· **Status:** BLOCKED (Entscheidung des Owners)

### ERR-AUD-003 — E06 doppelt belegt
**Fundstelle:** `ATC-BC-002` Zeile 50 gegen Zeilen 129 und 193 · **Ist:** Zeile
50 nennt E06 "unbekannter Befehl", Zeilen 129 und 193 reservieren E06 fuer
Aufruftiefe 64 und erklaeren sie fuer unveraenderlich
· **Spezifikationsbezug:** selbstwiderspruechlich · **Testabdeckung:** nicht
anwendbar · **Abweichung:** der reservierte Code ist zugleich einem anderen
Sachverhalt zugeordnet · **Empfehlung:** E06 ausschliesslich fuer Tiefe
reservieren, unbekannter Befehl braucht eine andere Kennung
· **Status:** BLOCKED (Entscheidung des Owners)

### ERR-AUD-004 — Vertragsraum korrekt getrennt
**Fundstelle:** `gueltiger_ruecklauf_code` · **Ist:** 0 und 0x0100–0x01FF
zulaessig, 0x00FF und 0x0200 abgelehnt, in Verifier und Interpreter gleich
· **Spezifikationsbezug:** deckungsgleich · **Testabdeckung:** ja, `tests/atcb_v2.rs` Zeilen 87 und 108 · **Abweichung:** keine
· **Empfehlung:** keine · **Status:** TESTED

### ERR-AUD-005 — Namensraeume strukturell getrennt, nicht dokumentiert
**Fundstelle:** `VmError::Reverted(u64)` gegen die uebrigen Varianten
· **Ist:** Vertragscodes stecken in einer Variante, interne Fehler in anderen;
das ist die Trennung, aber sie ist nirgends als Schnittstelle festgeschrieben
· **Spezifikationsbezug:** fehlend · **Testabdeckung:** teilweise
· **Abweichung:** ohne Festlegung ist die Trennung nicht gegen Umbauten
geschuetzt · **Empfehlung:** Namensraeume explizit dokumentieren, bevor Codes
vergeben werden · **Status:** RESIDUAL

### ERR-AUD-006 — Fehlerdarstellung im Runner nicht unterscheidbar
**Fundstelle:** `src/main.rs`, `RunOutcome::Failed(format!("ERROR: ATVM: {e:?}"))`
· **Ist:** alle VM-Fehler landen als Debug-String in einem gemeinsamen Pfad;
Erfolg 0, Nutzungsfehler 2, kein eigener Exit-Code je Fehlerklasse
· **Spezifikationsbezug:** fehlend · **Testabdeckung:** nein
· **Abweichung:** Abbruch und Ruecklauf sind fuer ein Werkzeug nicht
unterscheidbar · **Empfehlung:** Klassencode im Exit-Status, sobald Codes
feststehen · **Status:** RESIDUAL

### ERR-AUD-007 — Kontextfehler verdecken Verifikationsfehler
**Fundstelle:** `execute_state_transition_with_gas` · **Ist:** Gate laeuft vor
`verify()`, ein Kontextfehler gewinnt · **Spezifikationsbezug:** offen
· **Testabdeckung:** `state_transition_requires_identity_gate`
· **Abweichung:** fail-closed und damit unkritisch, aber die Reihenfolge ist
nicht festgeschrieben · **Empfehlung:** Reihenfolge dokumentieren
· **Status:** RESIDUAL

### ERR-AUD-008 — Drei getrennte Fehlerfamilien ohne gemeinsamen Namensraum
**Fundstelle:** `VmError`, `ContextError`, `ArtifactError` · **Ist:** 14 + 6 + 10
Varianten, keine gemeinsame Kennung · **Spezifikationsbezug:** fehlend
· **Testabdeckung:** je Familie vorhanden · **Abweichung:** ein Werkzeug muss
drei Familien einzeln kennen · **Empfehlung:** gemeinsame Klassifikation auf
Kennungsebene, Familien bleiben getrennt · **Status:** RESIDUAL

## 6. Zusammenfassung

Gas: Tabelle vollstaendig und fuer den Compiler verbindlich, Belastung vor
Wirkung korrekt, aber unversioniert, je Opcode ungetestet, ohne Schrittgrenze
bei fehlendem Budget und ohne Bepreisung von Speicherwachstum.

Fehler: Der Vertragsraum ist sauber getrennt und getestet. Was fehlt, sind
numerische Kennungen im Code und eine widerspruchsfreie Spezifikation. Zwei
Widersprueche (E03/E04, E06) sind BLOCKED und brauchen deine Entscheidung,
bevor irgendetwas implementiert wird.

**Vorschlag fuer die spaetere Umsetzung, in dieser Reihenfolge:**
1. Entscheidung E03/E04 und E06 (normativ, Owner).
2. Gastabelle als versionierte Konstante, danach per-Op-Tests.
3. Codes auf bestehende Varianten abbilden, ohne Umnummerierung.
4. Schrittobergrenze fuer budgetfreie Ausfuehrung.
5. Speicherwachstum bepreisen (eigene Tabellenzeile, neue Tabellenversion).

Nested Execution bleibt BLOCKED.

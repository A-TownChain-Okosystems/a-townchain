# E1 — Vertrags-Lowering fail-closed (Umsetzung)

**Entscheidung des Owners (12.11.2026):** E1 wird als **Sicherheitsmaßnahme** umgesetzt, nicht als
Fehlerbehebung. Kein neues Sprachfeature, keine Erweiterung des Compilers, solange die falsche
Erfolgsmeldung existiert.
**Grundlage:** `docs/audits/EXEC-ATCLANG-COMPILER-AUDIT-2026-11-12.md`, Befund F1.
**Status:** IMPLEMENTED + TESTED (lokal). `CI_VERIFIED` **nicht** vergeben (kein CI-Lauf für
`atc-core`, Audit F9).

---

## 1. Was geändert wurde

`lower_program` prüft jetzt **vor** jedem Aufbau des Kompilats, ob die Deklarationsliste einen
`Stmt::Contract` enthält, und lehnt in diesem Fall ab. Die Prüfung ist rekursiv angelegt
(`find_contract`), damit ein Vertrag nicht dadurch durchrutscht, dass er einmal verschachtelt
auftritt.

```
Vertrag 'FranchiseToken' ist nicht ausfuehrbar: ATCLang senkt Vertragsruempfe nicht zu
Bytecode (11 Funktionen, 8 Zustandsfelder, 5 Events, 0 weitere Deklarationen). Der Vertrag
wird abgelehnt, statt ihn mit einem Ersatzprogramm anzunehmen
(Compiler-Erfolg ist kein ausfuehrbarer Vertrag)
```

Die Ablehnung nennt Vertragsnamen und die tatsächlich betroffenen Bestandteile. Sie entsteht
**bevor** `__main__` gebaut wird, deshalb existiert kein Kompilat, kein `ConstI64(0)`-Rückfall und
kein `Push(0); Halt`-Ersatzprogramm.

Datei: `ai/atclang/crates/atc-core/src/lower.rs` (`find_contract`, `contract_surface`, Guard am
Anfang von `lower_program`). Der `Stmt::Contract`-Zweig in der Entry-Schleife bleibt als
Kommentar-Marker stehen, ist aber durch den Guard unerreichbar.

## 2. Abnahmekriterien des Owners und ihr Nachweis

| # | Kriterium | Nachweis | Ergebnis |
|---|---|---|---|
| 1 | `Stmt::Contract` wird nicht mehr stillschweigend übersprungen | `tests/contract_lowering.rs::vertrag_mit_logik_wird_abgelehnt_nicht_uebersprungen` | erfüllt |
| 2 | Nicht unterstützter Vertragskörper erzeugt expliziten Compilerfehler | dito, Fehlertext enthält Vertragsnamen und Grund | erfüllt |
| 3 | `atc run` liefert keinen falschen Exit 0 | `tests/cli_contract_rejection.rs::atc_run_lehnt_vertrag_mit_fehlerstatus_ab` (Exit 1, stdout leer) | erfüllt |
| 4 | `System::deploy` lehnt den fehlerhaften Vertrag ab | `components/system/tests/contract_rejection.rs::deploy_lehnt_vertrag_ab_und_schreibt_nichts` | erfüllt |
| 5 | Kein `Push(0); Halt` als Ersatzprogramm | `kein_ersatzprogramm_fuer_vertragsquellen` (Fehler entsteht im Lowering, nicht im Backend) | erfüllt |
| 6 | Regressionstest für den Vertrag mit `main = 42` | `vertrag_mit_logik_wird_abgelehnt_nicht_uebersprungen`; Gegenprobe `programm_ohne_vertrag_bleibt_unveraendert` (Programm ohne Vertrag liefert weiterhin 42 über Artefakt und VM) | erfüllt |
| 7 | Regressionstest für die 11 Funktionen und die übrigen Bestandteile | `token_vertrag_verliert_keine_bestandteile_still` zählt 11 Funktionen, 8 Zustandsfelder, 5 Events in der Quelle und verlangt dieselben Zahlen im Fehler | erfüllt |
| 8 | Alle betroffenen Tests laufen reproduzierbar | `cargo test --workspace`: 759 grün, 0 Fehler | erfüllt |

Zusätzlich, über die Kriterien hinaus: `keine_vertragsquelle_des_monorepos_ergibt_bytecode` prüft
**jede** der 50 Vertragsquellen unter `apps/atc-contracts` und verlangt, dass keine ein Kompilat
ergibt. Damit ist auch ein künftig hinzugefügter Vertrag abgedeckt, ohne dass jemand an den Test
denken muss.

## 3. Verhalten vorher und nachher

| Situation | vorher | nachher |
|---|---|---|
| `atc run token.atc` | `0`, Exit 0 | Compilerfehler, Exit 1, kein stdout |
| Vertrag mit `fn main() -> i64 { return 42; }` | `0`, Exit 0 | Compilerfehler, Exit 1 |
| leere Vertragsdeklaration `contract Leer { }` | Kompilat, `0` | Compilerfehler (leerer Vertrag ist nicht spezifiziert) |
| `System::deploy(token.atc)` | `Ok`, Registry-Eintrag, Audit-Eintrag | `SystemError::Compile(CompileError::Lower)`, Registry/Audit/Kette unverändert |
| `System::execute` auf dem Deployment | `Ok(Some(0))` | nicht erreichbar |
| Programm ohne Vertrag | läuft | läuft unverändert |
| `atc check`, `atc abi`, `atc storage` auf Verträgen | Frontend-Prüfung | unverändert (Frontend, keine Ausführungszusage) |
| `atc compile` (AST-JSON) | AST | unverändert (AST-Ausgabe, kein Bytecode) |

## 4. Bewusste Folge

**Bis ein Vertrags-Lowering existiert, lässt sich kein Vertrag deployen.** Das ist die richtige
Richtung des Fehlers: vorher wurde jeder Vertrag angenommen und keiner ausgeführt. Eine Kette, die
nichts ausführt, aber Erfolg meldet, ist gefährlicher als eine, die ablehnt.

Was diese Änderung **nicht** leistet, und was ausdrücklich offen bleibt: sie macht ATCLang nicht
fähiger. Vertragslogik (`state`, `require`, `emit`, `self.feld`, Maps, `u128`, Aufrufe) bleibt
außerhalb des ausführbaren Subsets. E1 entfernt die falsche Zusage, nicht die Lücke.

## 5. Was nicht angefasst wurde

Kein Standard, keine Registry, keine ID, kein Bytecodeformat, keine Opcode-Tabelle, keine
Grenzwerte. `atc-standards` und ATC-BC-001/BC-002 sind unverändert; E2 (Opcode-SSOT) ist eine
eigene, unabhängige Entscheidung. Der Bytecode bestehender, vertragsfreier Programme ändert sich
nicht, weil der Guard nur bei vorhandenem `Stmt::Contract` greift.

## 6. Nachweise

| Feld | Wert |
|---|---|
| Neue Tests | 11 (6 Compiler, 2 CLI, 3 System) |
| Workspace | 759 grün, 0 Fehler (vorher 748) |
| Formatierung | `rustfmt --check` auf `lower.rs` ohne Diff |
| Clippy | `cargo clippy -p atc-core -p atc-system --all-targets` ohne Befund in den geänderten Dateien |
| `CI_VERIFIED` | nicht vergeben |
| Offen | E2 (Opcode-SSOT), E3 (VM-Decoder), E4 (E2E Deploy/Execute/Call), E5 (CI und Typkonsistenz) |

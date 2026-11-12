# ATCLang-Compiler-Audit — Ist-Stand der sieben Stufen (12.11.2026)

**Gegenstand:** Wie funktioniert ATCLang heute tatsächlich, Stufe für Stufe, gemessen am Code.
**Methode:** Nur lesend. Jede Aussage ist an Datei:Zeile und Blob-SHA gebunden oder durch einen
ausgeführten Befehl belegt. Keine geplante Funktion wird als vorhanden ausgewiesen.
**Erhebungsstand:** Arbeitsstand `82ae098d`; die geprueften Compiler- und VM-Dateien sind
blobgleich mit `origin/main` (`0373087c`), am 12.11.2026 doppelt verglichen. Blob-SHAs in §9.
**Ergebnis vorweg:** Frontend vollständig und streng, Backend nur ein Ausdrucks-Subset. Der
Flaggschiff-Vertrag `apps/atc-contracts/contracts/token.atc` wird beim Deployment **erfolgreich
angenommen und führt nichts aus**. Das ist der zentrale Befund, nicht eine Nebenbemerkung.

---

## 1. Verortung: wo der Compiler wirklich liegt

| Ort | Was dort liegt | Status |
|---|---|---|
| `ai/atclang/crates/atc-core` | Kanonischer Compiler (17 Module, 10.136 Zeilen, 197 Tests) | **aktiv, gebaut** (Workspace-Mitglied) |
| `components/system` | Komponiert den Compiler in-process (`src/compile.rs`) und übersetzt in ATCB (`src/backend.rs`) | **aktiv** |
| `atclang` (eigenes Repo, Stand `b76c42b`) | Zweite Kopie desselben Crates, älterer Stand | divergent, kein Mirror-Pin |
| `tooling/atc-engineering/crates/atc-core` | **Anderes Crate** gleichen Namens ("Fundamentale Domain Types: IDs, States, Errors") | Namenskollision |

Die Kollision ist nicht kosmetisch: `grep -r "atc-core" --include=Cargo.toml` liefert zwei
verschiedene Pakete unter einem Namen. Wer nach Crate-Namen statt nach Pfad sucht, landet im
falschen Baum (gleiche Fehlerklasse wie `components/node` vs `apps/atc-node`).

Zweite Kopie, nachweisbar auseinandergelaufen (`diff -rq` gegen `ai/atclang/crates/atc-core/src`):
`project.rs`, `semantics.rs`, `storage.rs`, `abi.rs` fehlen im Standalone-Repo vollständig;
`ast.rs`, `lexer.rs`, `parser.rs`, `lower.rs`, `main.rs`, `lib.rs` weichen ab. Damit gibt es
zwei Wahrheiten über dieselbe Sprache, ohne Kennzeichnung, welche gilt.

Konsumenten des Compilers im Workspace: `components/system` (produktiv) und
`tooling/atc-engineering` (anderes Crate, siehe oben). Sonst niemand.

---

## 2. Die sieben Stufen im Ist-Stand

| # | Stufe | Pfad, Symbol | Umfang | Tests | Status |
|---|---|---|---|---|---|
| 1 | Lexer | `src/lexer.rs`, `tokenize` | 529 Z. | 15 | **IMPLEMENTED** |
| 2 | Parser / AST | `src/parser.rs` `parse_program`, `src/ast.rs` | 2.267 + 696 Z. | 49 | **IMPLEMENTED** |
| 3 | Type Checker | `src/types.rs`, `check_program` | 638 Z. | 8 | **IMPLEMENTED**, konservativ |
| 4 | Semantic Validator | `src/semantics.rs`, `check_program` | 563 Z. | 11 | **IMPLEMENTED** |
| 5 | Intermediate Representation | `src/ir.rs` | 74 Z. | 3 | **NICHT VORHANDEN** (Typ-/Wertgrenze, kein Instruktions-IR) |
| 6 | Bytecode-Compiler | `src/lower.rs` (`lower_program`) + `src/bytecode.rs` | 779 + 667 Z. | 31 | **PARTIAL** (nur Ausdrucks-Subset) |
| 7 | Verification Gate | `bytecode.rs::verify_with_signatures` (Worklist-Fixpunkt) | in Stufe 6 | 15 | **IMPLEMENTED** (compilerseitig) |

Ergänzend, ohne Gegenstück in der Sieben-Stufen-Vorlage:

| Baustein | Pfad | Status |
|---|---|---|
| Artefakt-Hülle `ATCA` | `src/artifact.rs` (524 Z.) | IMPLEMENTED, aber von keiner Kette gelesen |
| ABI-Erzeugung | `src/abi.rs` (375 Z.), `abi_sha256` je Vertrag | IMPLEMENTED |
| Storage-Layout S1 | `src/storage.rs` (358 Z.), `layout_hash` | IMPLEMENTED, nicht ausführbar (siehe F5) |
| Referenz-Interpreter | `src/vm.rs` (301 Z.) | IMPLEMENTED, dritte Ausführungssemantik |
| Native-Contract-Parser | `src/native.rs` (561 Z.) | IMPLEMENTED, eigener Dialekt (`@standard`, `@symbol`) |
| Projektprüfung | `src/project.rs`, `atc check <dir>` | IMPLEMENTED |

Was Stufe 1 und 2 wirklich abdecken: `Token` kennt 54 Varianten inklusive `Contract`, `State`,
`Event`, `Require`, `Use`, `Module`, `Import`, `Struct`, `Enum`, `Capability`, `Policy`;
`Stmt` kennt 21 Formen inklusive `For`, `Emit`, `Assign`, `StateBlock`. Die Grammatik ist also
deutlich reicher als das, was ausgeführt werden kann. Genau daraus entsteht F1.

---

## 3. Live-Belege (ausgeführt am 12.11.2026)

Werkzeug: `cargo test -p atc-core --offline` und `./target/debug/atc`.

```
cargo test -p atc-core            -> 170 lib + 4 cli + 3 contract_sources + 1 differential
                                     + 8 run + 11 stdlib = 197 Tests, 0 failed
atc check apps/atc-contracts      -> "50 Dateien, 33 Vertraege, 0 Fehler, 18 Warnungen", Exit 0
atc compile .../token.atc         -> 15.263 Byte kanonische AST-JSON, kein Bytecode
atc abi     .../token.atc         -> abi_sha256 65bd5eb9..., 8 state, 5 events, 11 fn
atc storage .../token.atc         -> 5acf7973... 8 Slots 256 Byte s1=no
atc run     .../token.atc         -> "0", Exit 0
atc run     "fn main() -> i64 { return 42; }"                  -> "42", Exit 0
atc run     "contract C { fn main() -> i64 { return 42; } }"   -> "0", Exit 0
atc run     "let a: u64 = 6;"                                  -> "0", Exit 0
atc run     "let a: u64 = 6; require(a > 0, \"x\");"           -> Parse-Fehler, Exit 1
atc run     "fn main() -> i64 { return 100000000000000000000000000; }"
                                             -> "Integerliteral liegt ausserhalb des
                                                aktuellen i64-VM-Ausfuehrungsbereichs"
atc check   "fn main() -> i64 { return bogus_fn(1); }"         -> "OK: validiert", Exit 0
atc run     "fn main() -> i64 { return bogus_fn(1); }"         -> Fehler im Lowering
```

Kettenpfad, als temporärer Test in `components/system` ausgeführt und danach wieder entfernt:

```
compile(token.atc)  -> FUNKTIONEN: 1   fn __main__ params=0 instrs=2
translate(...)      -> ATCB-OPS: 3     Push(0) Halt Halt   (source_len=2)
System::devnet().deploy(token.atc)     -> Ok
System::execute(..., gas=200.000)      -> Ok(Some(0))
```

Zum Vergleich der Umfang der Quelle: `token.atc` hat 78 Zeilen, **11 Funktionen, 8 Zustandsfelder,
5 Events, 7 `require`**. Über alle Quellen in `apps/atc-contracts` (50 Dateien, 33 Verträge):
**326 `fn`, 92 `require`, 146 `emit`**. Davon kommen **0** im Bytecode an.

---

## 4. Befunde

### F1 (kritisch) — Der Vertrags-Rumpf wird beim Lowering stillschweigend übersprungen

`lower_program` sammelt Funktionen nur aus `Stmt::Fn` auf Programmebene und überspringt in der
Entry-Schleife `Stmt::Fn | Stmt::NativeFunction | Stmt::Use | Stmt::Module | Stmt::Import |
Stmt::UseGroup | Stmt::Struct | Stmt::Enum | Stmt::Contract` (`src/lower.rs:543-557`, die
`Contract`-Zeile ist `:554`). Der leere Rumpf wird danach mit `ConstI64(0)`, `Return` aufgefüllt
(`finish_function`, `src/lower.rs:452-456`).

Folge, dreifach belegt: `atc run token.atc` gibt `0` mit Exit 0 aus; ein Vertrag, der `main`
enthält, wird nicht aufgerufen (`0` statt `42`); `System::deploy(token.atc)` **gelingt** und die
Kette führt `Push(0); Halt; Halt` aus.

Das ist die schlechteste Kombination: innerhalb eines Rumpfes ist das Lowering fail-closed
(`require`, `emit`, `Assign` -> "Vertragsstatement ausserhalb des ausfuehrbaren Subsets",
`src/lower.rs:317-321`), aber der Container "Vertrag" selbst wird ohne Meldung verworfen. Ein
Compiler, der nichts tut und Erfolg meldet, ist von einem Compiler, der funktioniert, nicht zu
unterscheiden. Dieselbe Signallage entsteht auf der Kette: Deployment erfolgreich, Ausführung
erfolgreich, Ergebnis 0. Es gibt keinen Punkt im Pfad, der widerspricht.

**Behoben am 12.11.2026 durch E1 (fail-closed).** `lower_program` lehnt jeden `Stmt::Contract`
jetzt vor dem Aufbau des Kompilats ab, mit Vertragsnamen und Bestandteilen im Fehler. `atc run`
endet mit Exit 1 ohne stdout, `System::deploy` mit `SystemError::Compile(CompileError::Lower)`,
Registry, Audit und Kette bleiben unveraendert. 11 neue Tests, Workspace 759 gruen. Nachweis und
Abnahmekriterien: `docs/decisions/EXEC-E1-CONTRACT-LOWERING-FAILCLOSED-2026-11-12.md`. Die Befunde
F2 bis F9 bleiben offen; E1 entfernt die falsche Zusage, nicht die Luecke.

### F2 (kritisch) — Zwei Opcode-Tabellen unter einem Magic, und die falsche ist eingefroren

| Byte | Compiler, `ATCB` (`src/bytecode.rs:172-260`) | ATC-VM (`components/vm/src/artifact.rs:203-221`) |
|---|---|---|
| `0x01` | ConstI64 (i64, BE) | Push(u64, LE) |
| `0x02` | LoadLocal (u16) | **Add** |
| `0x03` | StoreLocal (u16) | **Sub** |
| `0x10` | Add | **Halt** |
| `0x12` | **Mul** | **Revert(u32)** |
| `0x30` / `0x40` | Jump / Pop | (unbekannt) |

Beide Ströme tragen `ATCB`. Normativ eingefroren ist die **Compiler**-Tabelle:
`atclang/specs/bytecode/SPEC.md` §Format (Freeze-Anker `ee265313...`, "format_version u16 BE = 1",
`0x01` ConstI64, `0x02` LoadLocal, `0x10` Add). Implementiert ist die **Kandidaten**-Tabelle aus
`standards/atc/candidates/ATC-BC-002-CANDIDATE.md:256-259` ("0x01 Push(u64), 0x02 Add ... 0x10
Halt", Containerformat-Byte `0x01`, zusätzliche Generation). ATC-BC-001 steht **nicht** in
`atc-standards/registry/standards.yaml`; dort existiert nur ein Zeiger
(`references/standards/ATCB-v1-POINTER.md`, Status "FROZEN / NOT VERIFIED, Gate 0 ausstehend,
SCR-0129 PENDING_OWNER_APPROVAL").

Die VM liest Compiler-Bytes nie. Die Verbindung ist ein In-Memory-Übersetzer
`components/system/src/backend.rs` (`Instruction` -> `atc_vm::vm::Op`), aufgerufen in
`components/system/src/system.rs:251`. Er arbeitet auf der Instruktionsliste, nicht auf dem
Wire-Format; deshalb fällt die Divergenz heute nicht auf. Sobald irgendjemand ein ATCA/ATCB-Artefakt
aus dem Compiler an die VM gibt, ist das Ergebnis entweder `E-003` (Version) oder, bei passend
gemachtem Kopf, **stille Fehldeutung**: `0x02` würde als `Add` ausgeführt, wo `LoadLocal` gemeint
war. Fail-closed ist das nicht.

### F3 (hoch) — Der Übersetzer lehnt ganze Klassen ab, und zwei Semantiken bleiben offen

`backend.rs:32-36` führt die offenen Divergenzen selbst auf:

1. Zahlenbereich: atc-core rechnet `i64` **geprüft** (Überlauf ist Fehler), atc-vm `u64`
   **umlaufend**.
2. Vorzeichen: `Lt` vergleicht in atc-core vorzeichenbehaftet, in atc-vm ohne Vorzeichen.

Dazu `BackendError::UnsupportedCall` (`backend.rs:41`): **jeder** `call` wird abgelehnt, weil die
VM keine Aufrufkonvention hat; ebenso negative Konstanten (`NegativeConstant`). Ein Programm mit
Funktionsaufrufen kann also nicht auf die Kette, obwohl der Compiler `Call` erzeugt und der
Verifizierer ihn prüft. Dass `token.atc` trotzdem deployt, liegt nur an F1: es bleibt nichts übrig,
was abgelehnt werden könnte.

### F4 (hoch) — Es gibt keine Zwischensprache

`src/ir.rs` (74 Zeilen) definiert `ProtocolType`, `IrType`, `IrValue`: eine Grenze zwischen
ABI-Werten und Protokollzählern, **kein** Instruktions-IR. `grep -rn "IrValue\|IrType\|crate::ir"`
findet außer `ir.rs` selbst **keinen** Treffer, das Modul ist also von nichts benutzt. Das Lowering
geht direkt vom AST auf `Instruction`.

Damit ist ATC-STD-623 **P2 / REQ-STD-012** ("Zwischencode-/Optimierungsregeln deterministisch")
ohne Nachweis. Das ist keine Formalie: ohne IR gibt es keine Stufe, auf der man Optimierungen
deterministisch prüfen könnte, und keine gemeinsame Ebene für einen zweiten Codegenerator.

### F5 (hoch) — Drei Wortbreiten, und die Ökonomie passt in keine davon

Lexer: `Token::Int(u128)`. ABI: `AbiType::{U64, U128, U256, Bool, Address, Bytes, ContractRef}`
(`src/value.rs:14-22`). Bytecode: `ConstI64(i64)`. VM: `Push(u64)`.

Nachgewiesen: `return 100000000000000000000000000;` (1e26, die Größenordnung von `MAX_SUPPLY` im
Token-Vertrag) wird mit "Integerliteral liegt ausserhalb des aktuellen i64-VM-Ausfuehrungsbereichs"
abgelehnt. Ein Supply-Wert, ein Saldo in kleinster Einheit oder ein u128-Betrag ist im
ausführbaren Subset nicht darstellbar. `u128`/`u256` existieren heute als Typen in ABI und
Standardbibliotheks-Referenz (11 Konformanztests in `tests/stdlib_reference_conformance.rs`), aber
nicht als VM-Operation.

### F6 (mittel) — `atc check` validiert weniger als der Compiler, und Verträge sehen nur `check`

`atc check` meldet für `fn main() -> i64 { return bogus_fn(1); }` **"OK: validiert", Exit 0**;
erst das Lowering erkennt "Aufruf unbekannter Funktion 'bogus_fn'". Ein Vertrag, der `safe_add`
oder `is_valid_address` aufruft, wird also von der Prüfinstanz akzeptiert, die als Torwächter
verwendet wird, obwohl im Compiler keine dieser Funktionen definiert ist.

Weil Vertragsrümpfe nie gelowerd werden (F1), ist `check` die **einzige** Stufe, die einen Vertrag
jemals sieht. Die Prüfaussage "0 Fehler" über 33 Verträge bedeutet daher: syntaktisch, semantisch
Stufe 1 und typseitig plausibel, nicht: compilierbar.

### F7 (mittel) — `atc compile` kompiliert nicht

`atc compile <datei>` gibt kanonische AST-JSON auf stdout aus (15.263 Byte für `token.atc`), keinen
Bytecode. Der Modulkopf behauptet zugleich weiterhin "Noch NICHT Bestandteil: AST->Bytecode-Lowering"
(`src/main.rs:5`), obwohl `run` und die Bibliothek Lowering längst tun. Beides ist irreführend, in
entgegengesetzte Richtungen.

### F8 (mittel) — Capability Security ist ein Etikett

`ArtifactMetadata.capabilities` ist eine sortierte, duplikatfreie `Vec<String>`
(`validate_metadata`, `src/artifact.rs:261-280`): geprüft werden nur Länge und Sortierung. Es gibt
keine Zuordnung Fähigkeit -> Aufrufstelle, keine Prüfung zur Laufzeit, keine Ableitung aus dem
Quelltext. `target_profile` ist ein freier String ohne Wertebereich ("l1-deterministic-v1" im Test).
Die vier in der Zielarchitektur genannten Profile (Contract, System, Application, Security)
existieren im Code nicht als Typ und nicht als Prüfung. Für ATC-STD-620 §2 und die
Sicherheitszusagen der Dokumentation heißt das: die Eigenschaft ist derzeit nicht messbar.

### F9 (mittel) — Kein CI-Nachweis für den Compiler

`grep -rn "cargo" .github/workflows/*.yml` zeigt `cargo check`/`test` ausschließlich für
`components/wallet`, `components/sdk`, `modules/atc-zkp` und (Matrix) `components/node`,
`components/mining`. **`ai/atclang/crates/atc-core` wird von keinem Workflow gebaut oder getestet.**
Die 197 grünen Tests sind lokal, nicht CI-verankert; `CI_VERIFIED` ist damit nicht vergeben.

Zusätzlich: der Rust-Zweig des Determinism-Gate ruft `tools/determinism_check.py --lang rust
--test-cmd "true"` auf (`.github/workflows/determinism-gate.yml:25`). Der Testbefehl ist `true`,
also ein No-op; geprüft werden nur Quellmuster (`SystemTime::now`, `rand::`, ...), und `SKIP_DIRS`
schließt `tests` aus (`tools/determinism_check.py:20-41`).

---

## 5. Abgleich mit den normativen Standards

| Standard | Anforderung | Ist-Stand | Urteil |
|---|---|---|---|
| ATC-STD-620 KR-2 | Compiler deterministisch | Verifizierer deterministisch, Sprungmodell fest, keine Umgebungsabhängigkeit | erfüllt im Subset |
| ATC-STD-620 KR-3 | Dialektmodi explizit deklariert, kein stiller v1.0-Dialekt | Zwei `ATCB`-Tabellen, keine Dialektkennung im Strom | **nicht erfüllt** (F2) |
| ATC-STD-620 KR-4 | Fehlermeldungen katalogisiert und versionierbar | Meldungen sind freie Strings, je Stufe anders formuliert; `atc check` vs Lowering uneinheitlich | **nicht erfüllt** (F6) |
| ATC-STD-620 KR-6 | VM-Semantik gegen Spezifikation getestet | VM prüft gegen ATC-BC-002-Kandidat, nicht gegen die eingefrorene ATC-BC-001 | **nicht erfüllt** (F2) |
| ATC-STD-623 P1 | Grammatik-/Frontend-Konformanz | Lexer/Parser/Projektprüfung vorhanden, 49+15+15 Tests | Nachweis führbar |
| ATC-STD-623 P2 | Zwischencode deterministisch | kein IR | **nicht erfüllt** (F4) |
| ATC-STD-623 P3 | Fehlerdiagnose-Katalog | kein Katalog, siehe KR-4 | **nicht erfüllt** |
| ATC-STD-623 P5 | Roundtrip-/Regressionstests | 18-Korpus-Differential, `contract_sources` über 50 Dateien, Ketten-Differential | teilweise |
| ATC-STD-623 P6 | Kernregeln KR-1..KR-6 | KR-3, KR-4, KR-6 offen | **nicht erfüllt** |

---

## 6. Was gut ist (und bleiben muss)

Damit die Empfehlungen nicht das Falsche treffen: die tragenden Teile sind solide.

- Der **Verifizierer** ist echt: Worklist-Fixpunkt über alle erreichbaren Pfade, Stack-Höhen an
  Join-Stellen, Sprungziele, Div/0-Fang, `MissingReturn` (`src/bytecode.rs:240-436`). Er prüft vor
  Vertrauen, in `finish_function` und erneut in `CompiledProgram::verify`.
- Das **Fail-closed-Prinzip** ist pro Statement konsequent umgesetzt, inklusive ausdrücklicher
  Ablehnung statt Näherung (`src/lower.rs:317-321`).
- Die **Projektprüfung** (`atc check <dir>`) findet Importe, Duplikate, unbekannte Typnamen und
  verdeckende Importe über 50 Dateien mit 0 Fehlern.
- **ABI und Storage-Layout** sind deterministisch gehasht (`abi_sha256`, `layout_hash`) und damit
  anschlussfähig; Storage erkennt selbst, dass `token.atc` nicht S1-fähig ist (`s1=no`, 8 Slots,
  256 Byte, wegen `Map`-Feldern dynamisch).
- Der **Ketten-Differentialtest** (`components/system/tests/differential.rs`) vergleicht zwei Wege
  mit von Hand gerechneten Erwartungswerten und benennt seine eigene Grenze ehrlich (implizite
  Rückgabe 0, Speicher nicht vergleichbar).

Diese Teile sind die Grundlage, auf der ein Vertrags-Lowering aufsetzen kann. Sie sind nicht das
Problem.

---

## 7. Empfehlungen, in dieser Reihenfolge

**E1 (sofort, klein, ohne Entscheidung): F1 fail-closed machen.** Wenn `lower_program` einen
`Stmt::Contract` mit ausführbarem Rumpf sieht, muss es **ablehnen**, nicht überspringen. Ein
Vertrag, dessen Rümpfe nicht gesenkt werden, ist kein Programm. Das ändert kein Format, keine
ID, keinen Standard, und es beseitigt die einzige Stelle, an der heute ein No-op als Erfolg
durchgeht. Erwartete Nebenwirkung: `System::deploy` auf `token.atc` schlägt fehl, was korrekt ist.
Vor dem Umbau zu prüfen: welche Testsuiten `deploy` mit echten Vertragsquellen aufrufen.

**E2 (Owner-Entscheidung): eine Formatwahrheit.** Entweder die VM wird auf die eingefrorene
ATC-BC-001 konform gemacht, oder ATC-BC-001 wird per SCR durch ATC-BC-002 ersetzt (mit neuem
Freeze-Anker). Der heutige Zustand, eingefrorene Spezifikation ohne Implementierung und
implementierter Kandidat ohne Freigabe, ist der schlechteste von drei möglichen. Solange beide
existieren, darf kein Artefakt das Crate-Wechseln erlauben, und der Übersetzer in `atc-system`
bleibt die einzige Brücke, also ein dauerhafter zweiter Wahrheitsort.

**E3 (nach E2): Aufrufe und Wortbreite als Vertragspunkte entscheiden.** Ohne `call` in der VM
bleibt ATCLang eine Ausdruckssprache; ohne u128/u256 bleibt sie ohne Ökonomie. Beides sind
Formatfragen, keine Implementierungsdetails.

**E4: P2 ehrlich behandeln.** Entweder IR einführen (dann ist P2 prüfbar) oder im Standard
festhalten, dass P2 für den aktuellen Stand nicht anwendbar ist. Ein Standard, der eine Stufe
verlangt, die es nicht gibt, erzeugt Dauerbefunde.

**E5: `atc compile` umbenennen oder erweitern.** Heute `atc ast`. Ein Befehl namens `compile`, der
keinen Bytecode erzeugt, ist eine Falle für jedes Skript und jeden Agenten.

**E6: CI verankern.** `atc-core` in die Matrix von `ssot-source-validation.yml`; den Rust-Zweig des
Determinism-Gate auf einen echten Testbefehl umstellen. Beides ist eine Zeile, beides ist
Owner-Aktion am Workflow (Agent-Token ohne `workflow`-Scope, GH013).

**E7: Compiler-Kopien kennzeichnen.** Standalone `atclang` als Archiv markieren oder mit
Commit-Pin als Mirror führen; Namenskollision `atc-core` in `tooling/atc-engineering` auflösen.

---

## 8. Antwort auf die Ausgangsfrage, in einem Absatz

ATCLang ist heute eine **vollständige Frontend-Sprache mit einem sehr kleinen ausführbaren Kern**.
Ein Entwickler schreibt Verträge mit `contract`, `state`, `event`, `require`, `self.feld`, Maps und
`u128`; Lexer, Parser, Semantik, Typen und Projektprüfung nehmen das an und melden 0 Fehler. Davon
ausführbar sind nur Ausdrücke, `let`, `if`, `while`, Funktionsaufrufe und `return` über `i64`.
Vertragslogik wird nicht übersetzt, sondern verworfen, und zwar ohne Meldung: der Vertrag wird auf
der Kette angenommen und tut nichts. Die ATC-VM ist dabei nicht das Ziel des Compilers; sie spricht
eine andere Bytecode-Tabelle als die eingefrorene Spezifikation, und die Brücke ist ein Übersetzer
in `atc-system`, der Aufrufe grundsätzlich ablehnt.

---

## 9. Evidenz-Bindung

Blob-SHAs bei `82ae098d` (blobgleich mit `origin/main` `0373087c`) (Erhebungszeitpunkt, nicht Beweis: ein Blob-Wechsel entwertet die
zugehörigen Aussagen und verlangt eine neue Messung).

| Datei | Blob |
|---|---|
| `ai/atclang/crates/atc-core/src/lexer.rs` | `2e946cb5` |
| `ai/atclang/crates/atc-core/src/parser.rs` | `c7c3fc92` |
| `ai/atclang/crates/atc-core/src/lower.rs` | `624576b9` |
| `ai/atclang/crates/atc-core/src/bytecode.rs` | `8c4419ed` |
| `ai/atclang/crates/atc-core/src/ir.rs` | `081cf8df` |
| `ai/atclang/crates/atc-core/src/artifact.rs` | `8da375da` |
| `ai/atclang/crates/atc-core/src/vm.rs` | `009bdec1` |
| `ai/atclang/crates/atc-core/src/storage.rs` | `e2462937` |
| `ai/atclang/crates/atc-core/src/abi.rs` | `46760ebf` |
| `ai/atclang/crates/atc-core/src/types.rs` | `2818e4d0` |
| `ai/atclang/crates/atc-core/src/semantics.rs` | `006ba9e4` |
| `ai/atclang/crates/atc-core/src/project.rs` | `3939e388` |
| `components/vm/src/artifact.rs` | `122bf399` |
| `components/system/src/backend.rs` | `dc7d0841` |
| `components/system/src/compile.rs` | `baa8c82d` |

## 10. Status dieses Dokuments

| Feld | Wert |
|---|---|
| Art | Audit, rein lesend |
| Code geändert | keiner (temporärer Test in `components/system` angelegt und wieder entfernt) |
| Tests ausgeführt | `cargo test -p atc-core`: 197 grün, lokal |
| CI_VERIFIED | **nicht vergeben** (kein CI-Lauf für `atc-core`, siehe F9) |
| E2E_VERIFIED | **nicht vergeben** |
| Offene Entscheidungen | E2 (Formatwahrheit), E3 (Aufrufe/Wortbreite), E4 (P2 im Standard) |
| E1 (F1) | **IMPLEMENTED + TESTED** lokal (11 Tests, Workspace 759 grün); `CI_VERIFIED` offen |
| Nächster Schritt | E2 (Opcode-SSOT entscheiden), danach E3 (VM-Decoder) und E4 (E2E Deploy/Execute/Call) |

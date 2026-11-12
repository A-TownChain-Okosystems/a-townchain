# ATCLang-Zustandsleiter — Compiler-Erfolg ist kein ausführbarer Vertrag

**Vorschlag des Owners (12.11.2026), aufgenommen als Entscheidungsvorlage.** Die Regel selbst ist
noch nicht normativ: eine Aufnahme in den ATCLang-Standard braucht einen SCR nach ATC-STD-000 §9.
Dieses Dokument hält die Regel, die sieben Zustände und den heutigen Nachweis je Zustand fest,
damit die Aufnahme vorbereitet ist und niemand einen Zustand ohne Beleg behauptet.

**Regel:** `Compiler Success ≠ Executable Contract`. Ein erfolgreicher Syntax-, Typ- oder
Semantikcheck ist niemals ein Nachweis für einen ausführbaren Vertrag. Jeder Zustand braucht einen
eigenen, prüfbaren Nachweis; ein Zustand darf nur behauptet werden, wenn sein Nachweis vorliegt.

---

## 1. Die sieben Zustände und ihr Nachweis

| Zustand | Bedeutung | Nachweis (heute) | Stand für Verträge (12.11.2026) |
|---|---|---|---|
| `PARSED` | Lexer und Parser haben die Quelle angenommen | `parse_program` ohne Fehler, AST vorhanden | **JA** (50 Dateien, 33 Verträge) |
| `TYPE_CHECKED` | Typprüfung ohne Fehler | `types::check_program`, 0 Fehler | **JA** |
| `SEMANTICALLY_VALIDATED` | Semantikregeln ohne Fehler | `semantics::check_program` plus `project::check` | **JA** (0 Fehler, 18 Warnungen) |
| `BYTECODE_GENERATED` | Es existiert ein Kompilat mit Instruktionen | `lower_program` liefert `CompiledProgram` | **NEIN** (seit E1 ausdrückliche Ablehnung) |
| `BYTECODE_VERIFIED` | Verifizierer hat das Kompilat angenommen | `bytecode::verify_with_signatures` | **NEIN** (kein Kompilat) |
| `DEPLOYABLE` | `System::deploy` akzeptiert das Artefakt | `atc-system` Deploypfad | **NEIN** (`SystemError::Compile`) |
| `EXECUTABLE` | Ausführung liefert ein definiertes Ergebnis | `System::execute` / `vm::execute_verified` | **NEIN** |

Für Programme ohne Vertrag (Ausdrucks-Subset) sind alle sieben Zustände erreichbar; `EXECUTABLE`
ist dort belegt (`fn main() -> i64 { return 42; }` bis `System::execute` = 42, siehe
`components/system/tests/contract_rejection.rs`).

## 2. Warum die Trennung nötig ist

Vor E1 war der Zustand `SEMANTICALLY_VALIDATED` erreichbar, `BYTECODE_GENERATED` wurde behauptet,
ohne dass es ein Kompilat gab, und `EXECUTABLE` lieferte `0`. Genau diese Vermischung hat den
Befund F1 möglich gemacht: drei verschiedene Aussagen ("prüft", "kompiliert", "führt aus") wurden
aus einer einzigen Fehlermeldung "OK" abgeleitet.

## 3. Konsequenzen, wenn die Regel normativ wird

1. Ein Gate darf nur den Zustand behaupten, dessen Nachweis im selben Lauf erzeugt wurde.
2. `atc check` darf nicht mehr als "validiert" erscheinen, ohne den erreichten Zustand zu nennen
   (Audit F6: `check` ist heute schwächer als das Lowering).
3. Jede Statusanzeige in Dokumentation, Wiki und Roadmap muss den Zustand nennen, nicht nur
   "implementiert".
4. Ein Zustandswechsel ist ohne Nachweis ein Befund, kein Fortschritt. Dieselbe Systematik wie die
   Evidence-Klassifikation (NORMATIVE / IMPLEMENTED / RESEARCH / LEGACY).

## 4. Offen

- SCR für die Aufnahme in den ATCLang-Standard (Owner-Aktion, `atc-standards` ist PR-geschützt).
- Werkzeugseitige Abbildung der sieben Zustände (eine einzige Quelle, aus der Statusberichte
  gerendert werden) ist **nicht** umgesetzt und wird hier nicht als vorhanden behauptet.

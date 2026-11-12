# P1d-B — Build-Abdeckung (G-01) und Autorität über den Blockbestand (F3)

> **Status:** G-01 `IMPLEMENTED`/`TESTED`; F3 `ANALYSIERT`, Umsetzung offen.
> **Datum:** 2026-11-12. **CI_VERIFIED:** nein.

## 1 Governance-Regel G-01 (umgesetzt)

Der eigentliche P1d-A-Befund war nicht `sync_pull`, sondern dass
sicherheitsrelevanter Code **außerhalb jeder Build- und Testabdeckung** lag.
Daraus die Regel: *Kein Cargo-Manifest darf unbemerkt aus der Prüfung fallen.*

| Baustein | Inhalt |
|---|---|
| `registry/build_coverage.yaml` | Jedes Manifest mit `status` und `reason` |
| `tools/check_build_coverage.py` | R1 Vollständigkeit, R2 Grund, R3 Workspace-Liste in beide Richtungen, R4 verschachtelte Wurzelbäume, R5 offene Befunde zählen |
| `tools/tests/test_build_coverage.py` | 4 Tests, u. a. dass `apps/atc-node` stillgelegt bleibt und dort kein `sync_pull` zurückkehrt |

Ergebnis der Inventur (77 Manifeste):

| Status | Anzahl | Bedeutung |
|---|---|---|
| `workspace_member` | 24 | wird gebaut und getestet |
| `nested_workspace` | 4 | eigener Build-Wurzelbaum (`ai/genesis-engine`, `apps/atc-marketplace`, `components/zkp`, `tooling/atc-engineering`) |
| `nested_workspace_member` | 41 | Crates in diesen Wurzelbäumen |
| `duplicate_retired` | 1 | `apps/atc-node` (Dublette, F4) |
| `unverified_build` | **7** | **kein Build-Nachweis — offener Befund** |

Die sieben offenen Manifeste:
`apps/atc-interop/modules/atc-bridge`, `apps/atc-mining`,
`apps/atc-sdk/modules/atc-cli`, `apps/atc-sdk/modules/atc-sdk/rust`,
`apps/atc-storage`, `ecosystem/integration`, `modules/atc-blockchain/kernel`.

Gate jetzt: **PASS** (alles klassifiziert), mit sichtbarer Liste der offenen
sieben. `--strict` macht sie zum Fehler; das ist der Zielzustand, sobald für
jedes eine Prüfstrategie entschieden ist. **Owner-Aktion:** den Prüfer als
CI-Schritt einhängen (`check_build_coverage.py --strict`, sobald die sieben
entschieden sind) — Workflow-Dateien kann der Agent mit seinem Token nicht
ändern (GH013, bereits in F-009/F-010 dokumentiert).

## 2 F3 — Analyse am Code (kein Umbau in diesem Schritt)

Ist-Zustand: `System` hält `chain: Chain` und ruft in `execute` die Kette
**selbst**:

```text
VM ausführen (auf vm.state(), Klon)
  -> after = ContractState::from_words(vm.state())
  -> payload = "exec:<code_hash[:8]>:<state_root_after>"
  -> self.chain.produce(&payload)          // Block entsteht IN System
  -> *state = after                        // Zustand wird geschrieben
```

Reihenfolge ist heute günstig (Block vor Zustand), aber es bleiben zwei
Drift-Fenster:

1. **Kein gemeinsamer Commit-Punkt.** Block und Zustand sind zwei getrennte
   Schreibvorgänge auf zwei Objekten. Stirbt der Prozess dazwischen oder gibt der
   Aufrufer den Zustand nicht weiter, behauptet ein Block einen Zustand, den der
   Aufrufer nicht hält.
2. **Keine Vorbedingung.** Es wird nicht geprüft, ob der übergebene
   `ContractState` dem im letzten Block zugesicherten `state_root` entspricht.
   Zwei Aufrufer können mit verschiedenen Zuständen weiterrechnen, und beide
   Blöcke landen in derselben Kette.

Zielbild (deine Vorgabe, hier nur festgehalten): `System` erzeugt
Ausführungsergebnis und Payload; der **eine** autoritative Bestand liegt in der
Kompositionswurzel; Block-Commit und Zustandsübernahme sind eine Transaktion
(„alles oder nichts"), mit Vorbedingung auf `state_root` und Nachbedingung auf
den neuen Wurzelwert.

Umsetzungsschnitt für P1d-B: `System::execute` liefert `(payload, after)` statt
selbst zu produzieren; `DevnetNode` committet Block und Zustand gemeinsam; die
Kette verlässt `System` (Zugriff nur noch lesend über die Komposition). Das
berührt `deploy`, `execute`, `state`, die Demo-/CLI-Pfade und alle bestehenden
Tests — deshalb eigener Schritt mit eigener Evidenz, nicht als Nebenänderung.

## 3 Reihenfolge (Owner-Festlegung)

| # | Arbeit | Abnahmekriterium |
|---|---|---|
| 1 | F3 Chain-Vereinheitlichung | keine zweite kanonische Chain |
| 2 | Standalone Node in der Kompositionswurzel | eine Wurzel |
| 3 | `import_tx_block` | kein Port-Bypass |
| 4 | Atomare Zustandsänderungen | kein State/Chain-Drift |
| 5 | E2E-Sicherheitsaudit | manipulierte Peers vollständig abgewiesen |

## 4 Nachweis

```
python3 tools/check_build_coverage.py     GATE PASS, 77 Manifeste klassifiziert, 7 offen
python3 -m pytest tools/tests -q          40 passed (vorher 36, +4 G-01)
cargo test --workspace                    748 grün, 0 rot
```

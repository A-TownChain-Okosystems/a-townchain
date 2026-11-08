# Konsolidierung: ein Repository für das gesamte Ökosystem

**Ziel:** Alle Org-Repos physisch in `a-townchain` zusammenführen (Owner-Entscheidung
08.11.2026: Monorepo ist SSOT). Ergebnis: 1 Repo statt 32, 26 Quell-Repos aufgelöst.

## Ziel-Layout

| Zielpfad | Herkunft | Dateien |
|---|---|---|
| `components/` | bestehend (Bibliotheken: algorithm, wallet, oracle, zkp, vm, node, …) | 1.132 |
| `apps/` | atc-node, atc-ide, atc-launchpad, atc-marketplace, atc-explorer, atc-indexer, atc-compute, atc-storage, atc-sdk, atc-interop, atc-contracts, atc-mining | 854 |
| `kernel/` | a-townchain-os (kai-os), atc-shivacore, globus-os | 719 |
| `ai/` | aurora-ai, atclang, genesis-engine, genesis-chronicles, genesis-franchise-factory | 971 |
| `standards/` | atc-standards | 2.160 |
| `tooling/` | atc-toolchain, atc-engineering | 164 |
| `docs/` | a-townchain-os-docs, demo-repository | 5.092 |
| `ecosystem/` | catalog, control-plane, schemas, integration, tools, docs, tests | 1.263 |

Summe Zuwachs: **11.220 Dateien**, Zielbestand ~12.350. Ausgeschlossen: `.git`,
`target/`, `node_modules/`, `__pycache__`, `.venv`, `dist/`, `build/` sowie die 26
Spiegel unter `ecosystem/components/` (die Quelle ist der jeweilige Code selbst).

## Phasen

1. **P1 — Physische Zusammenführung** (`tools/consolidate_monorepo.py`): kopiert Inhalte,
   schreibt Herkunft + Quell-Commit je Repo nach `MIGRATION_MANIFEST_MONOREPO.json`.
   Idempotent, `--dry-run` validiert die Zielstruktur.
2. **P2 — Bau-Vereinheitlichung:** ein Root-`[workspace]` mit allen Rust-Crates,
   einheitliche CI, gemeinsame Lizenz-/Security-Dateien, Dubletten auflösen
   (`modules/atc-zkp` vs `components/zkp`, Spiegel entfernen).
3. **P3 — Repos stilllegen:** die 26 Quell-Repos archivieren (read-only), Issues in
   `a-townchain` migrieren, READMEs auf den Monorepo-Pfad verweisen.

## Abnahmekriterien

- `cargo test` für alle Crates im Root-Workspace grün (P2).
- Kein Zielpfad doppelt belegt; jede Datei hat genau eine Herkunft im Manifest.
- `git log` des Monorepos enthält für jeden importierten Repo-Stand den Quell-SHA.

## Stand 08.11.2026 — blockiert

Der Import ist **vorbereitet und validiert** (Dry-Run: 11.220 Dateien, 26 Repos), kann
aber nicht ausgeliefert werden:

- `git push` → HTTP 403 „You must verify your email address" (Account des Connectors).
- `POST /git/blobs` und `POST /git/trees` → ebenfalls 403.
- Nur `PUT /contents` funktioniert (eine Datei = ein Commit). Für 12.350 Dateien wären
  das ~24.700 Aufrufe bei 5.000 erlaubten pro Stunde; zudem 12.350 Einzel-Commits.

**Freigabe:** Sobald die E-Mail des GitHub-Accounts verifiziert ist, läuft
`python3 tools/consolidate_monorepo.py --sources <repos> --target .` und danach ein
einzelner `git push` — wenige Minuten, ein Commit-Baum, vollständige Provenienz.

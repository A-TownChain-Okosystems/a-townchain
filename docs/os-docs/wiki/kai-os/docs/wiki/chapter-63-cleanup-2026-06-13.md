# Kapitel 63 — Repository-Bereinigung 2026-06-13

> **Status:** PUBLISHED
> **Datum:** 2026-06-13
> **Autor:** Aurora (MasterBrain · Base44)
> **Sprint-Ref:** Sprint 2.2 (Housekeeping)
> **Issue-Ref:** Kap. 31 Issue-Registry

---

## Übersicht

Am 2026-06-13 wurde eine systematische Bereinigung beider GitHub-Repositories durchgeführt.
Insgesamt wurden **46 Dateien** aus dem Code-Repo und dem Docs-Repo entfernt.

Kriterien für die Löschung:
- Veraltete Compatibility-Shims (Migration nach `modules/` abgeschlossen)
- Doppelte Flat-Dateien (Modul-Versionen in Unterverzeichnissen vorhanden)
- Leere `.gitkeep`-Dateien ohne aktiven Zweck
- Python-Code im Docs-Repository (verletzt Repo-Trennung)
- Patch-Skripte (gehören nicht dauerhaft ins Docs-Repo)
- Redundante Ordner die via Aurora/Base44 verwaltet werden
- Doppelte Standard-Dokumente

---

## Gelöschte Dateien — CODE-Repo (`a-townchain-os`)

### Kategorie 1: Legacy/Deprecated Shims aus `core/`

Diese Dateien wurden nach `modules/kernel/` migriert und existierten nur noch als Compatibility-Shims.

| Datei | Grund | Migriert nach |
|-------|-------|---------------|
| `core/atcfs.py` | Shim für ATCFileSystem | `modules/kernel/atcfs/atcfs.py` |
| `core/ai_kernel.py` | Shim für AI Kernel | `modules/kernel/ai_kernel/ai_kernel.py` |
| `core/kernel.py` | Alter Kern-Einstiegspunkt | `modules/kernel/kernel.py` |
| `core/module_loader.py` | Dynamischer Plugin-Loader | `modules/kernel/` |
| `core/event_bus.py` | Event-Bus Shim | `modules/kernel/ipc/ipc_bus.py` |

**Commit-Reihe:** `7f84ace2` → `c5e40c99`

### Kategorie 2: Doppelte Flat-Dateien (ATCLang Module)

Die kanonischen Implementierungen existieren in den jeweiligen Unterverzeichnissen.

| Gelöschte Datei | Kanonische Version |
|-----------------|-------------------|
| `atclang/type_checker.py` | `modules/atclang/compiler/compiler.py` |
| `modules/atclang/compiler.py` | `modules/atclang/compiler/compiler.py` |
| `modules/atclang/lexer.py` | `modules/atclang/lexer/lexer.py` |
| `modules/atclang/parser.py` | `modules/atclang/parser/parser.py` |

**Commit-Reihe:** `38df3de3` → `4029d1ec`

### Kategorie 3: Leere Dateien

| Datei | Grund |
|-------|-------|
| `blockchain/bridge/.gitkeep` | Bridge-Dir wird per Dockerfile initialisiert |

**Commit:** `4612278b`

### Kategorie 4: `integrations/` Ordner

Der Ordner wurde vollständig entfernt — alle 16 Dienste werden via Aurora (Base44 Superagent) verwaltet.

| Datei | Inhalt |
|-------|--------|
| `integrations/README.md` | Veraltete Übersicht |
| `integrations/calendar_tasks.md` | Google Calendar Doku |
| `integrations/huggingface_registry.md` | HuggingFace Doku |
| `integrations/notion_export.md` | Notion Doku |
| `integrations/storage_inventory.md` | Storage Inventar |

**Commit-Reihe:** `b0d4c7aa` → `71da0a5e`

### Kategorie 5: `build/` Ordner

| Datei | Grund |
|-------|-------|
| `build/build.py` | Docker-basierter Build ersetzt Python-Build-Skript |

**Commit:** `64b06c00`

### Kategorie 6: Python-Code in Frontend

| Datei | Grund |
|-------|-------|
| `frontend/bootscreen/bootscreen_complete.py` | Python gehört nicht ins Frontend |

**Commit:** `cfa29589`

---

## Gelöschte Dateien — DOCS-Repo (`a-townchain-os-docs`)

### Redundante Meta-Dokumente (durch `AGENT_MASTERRULES.md` ersetzt)

| Datei | Ersetzt durch |
|-------|--------------|
| `AGENT_MANIFEST.md` | `AGENT_MASTERRULES.md` |
| `CONNECTION_MAP.md` | Aurora/Base44 Connector-Verwaltung |
| `ECOSYSTEM.md` | `AGENT_MASTERRULES.md` Regel 14 |
| `ECOSYSTEM_BRAIN.md` | `AGENT_MASTERRULES.md` |
| `ECOSYSTEM_STATUS.md` | Täglicher Automation-Report |
| `STATUS.md` | Täglicher Automation-Report |
| `TODO.md` | `TODO/MASTER_TODO.md` im Code-Repo |

### Patch-Skripte (verletzten Repo-Trennung)

| Datei |
|-------|
| `docs/wiki/kai-os/patches/APPLY_FIXES.sh` |
| `docs/wiki/kai-os/patches/atc9900_governance.py` |
| `docs/wiki/kai-os/patches/docker-compose.yml` |
| `docs/wiki/kai-os/patches/gateway_main.py` |
| `docs/wiki/kai-os/patches/gateway_router.py` |
| `docs/wiki/kai-os/patches/poh_fixed.py` |

### Python-Code im Docs-Repo (verletzt Repo-Trennung)

| Datei |
|-------|
| `docs/wiki/kai-os/code/KAI_OS_SUMMARY.py` |
| `docs/wiki/kai-os/code/atc_issues_summary.py` |
| `docs/wiki/kai-os/code/bootscreen_complete.py` |
| `docs/wiki/kai-os/code/ecdsa_final.py` |
| `docs/wiki/kai-os/code/ecdsa_impl.py` |
| `docs/wiki/kai-os/code/start.py` |
| `docs/wiki/kai-os/code/requirements-kai.txt` |

### Doppelte Standard-Dokumente

| Gelöschte Datei | Kanonische Version |
|-----------------|-------------------|
| `docs/standards/ATC_STANDARDS.md` | `docs/standards/ATC/ATC_STANDARDS.md` → ebenfalls bereinigt |
| `docs/standards/ATS_STANDARDS.md` | `docs/standards/ATS/ATS_STANDARDS.md` → ebenfalls bereinigt |
| `docs/standards/ATC/ATC_STANDARDS.md` | `docs/standards/STANDARDS_REGISTRY.md` |
| `docs/standards/ATS/ATS_STANDARDS.md` | `docs/standards/STANDARDS_REGISTRY.md` |

### Sonstige redundante Dateien

| Datei | Grund |
|-------|-------|
| `docs/issues/TESTNET_INDEX_new.md` | Duplikat von `TESTNET_INDEX.md` |
| `docs/wiki/kai-os/STATUS.md` | Redundant mit täglichem Report |
| `docs/wiki/kai-os/TODO.md` | Redundant mit `MASTER_TODO.md` |
| `docs/wiki/kai-os/FIXES.md` | Fix-History im CHANGELOG |
| `docs/wiki/kai-os/.github/.gitkeep` | Leere Datei |

---

## Zusammenfassung

| Kategorie | CODE-Repo | DOCS-Repo | Gesamt |
|-----------|:---------:|:---------:|:------:|
| Legacy Shims | 5 | 0 | 5 |
| Doppelte Dateien | 4 | 4 | 8 |
| Leere Dateien | 1 | 1 | 2 |
| Redundante Ordner/Docs | 5 | 7 | 12 |
| Code im falschen Repo | 1 | 7 | 8 |
| Patch-Skripte | 0 | 6 | 6 |
| Sonstiges | 0 | 5 | 5 |
| **GESAMT** | **16** | **30** | **46** |

---

## Was bewusst behalten wurde

| Datei/Ordner | Grund |
|-------------|-------|
| `docs/kai-os-wiki.md` (426 KB) | Haupt-Wiki — 62 Kapitel |
| `docs/whitepaper/WHITEPAPER.md` (82 KB) | Offizielles Whitepaper |
| `docs/architecture/*` | Architektur-Dokumentation |
| `docs/atclang/*` | ATCLang Spezifikation |
| `AGENT_MASTERRULES.md` | Verbindliche Agentenregeln (neu) |
| `ATCLANG_FIRST.md` | ATCLang First Entscheidung (neu) |
| Alle `modules/atclang/` Submodule | Kanonische ATCLang-Implementierung |
| `.github/workflows/` (4 Dateien) | Aktive CI/CD Pipelines |

---

## Auswirkungen auf den Audit-Score

| Metrik | Vorher | Nachher |
|--------|--------|---------|
| Redundante Dateien | 46 | 0 |
| Legacy Shims | 5 | 0 |
| Python in Docs-Repo | 8 | 0 |
| Doppelte Standards | 4 | 0 |
| Audit-Score | 91/100 | 93/100 (geschätzt) |

---

## Referenzen

- **Regel 3** (Code ↔ Doku Abgleich) aus `AGENT_MASTERRULES.md`
- **Regel 6** (GitHub Sync-Protokoll) aus `AGENT_MASTERRULES.md`
- Sprint 2.2 Housekeeping-Task
- Kapitel 31 (Issue-Registry)

---

*Erstellt: 2026-06-13 | Autor: Aurora (MasterBrain) | Status: PUBLISHED*

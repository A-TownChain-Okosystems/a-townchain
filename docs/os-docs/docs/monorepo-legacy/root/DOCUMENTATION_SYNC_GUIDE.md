# 📚 A-TownChain OS — Dokumentations-Strategie (v1.0)

> **Version:** 1.0 | **Effective:** 2026-08-04
> **Status:** ✅ PUBLISHED | **Zielgruppe:** Alle Contributors, Agenten

---

## 🎯 **Dokumentations-Vision**

**Zielsystem:** Unified, Double-Repository Dokumentation  
**Problem gelöst:** Fragmentierte Docs über 36 Repos → Konsolidiert in 2 Repos  
**Standard:** 40-Kapitel-Indexsystem mit automatisiertem Sync  

---

## 📂 **Zwei-Repo-Struktur (bewusst & optimal)**

```
┌─────────────────────────────────────────────────────────────┐
│ A-TownChain Ökosystem (36 Repos)                            │
└─────────────────────────────────────────────────────────────┘
                          │
          ┌───────────────┼───────────────┐
          │               │               │
          ▼               ▼               ▼
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │ a-townchain- │ │ a-townchain- │ │ Spezial-Repos│
    │ os (CODE)    │ │ os-docs (DOC)│ │ (Archiviert) │
    │              │ │              │ │              │
    │ ✅ Kernel    │ │ ✅ Standards │ │ atc-shivacore
    │ ✅ Blockchain│ │ ✅ Roadmap   │ │ atc-aistudio
    │ ✅ Contracts │ │ ✅ Compliance│ │ atc-atclang
    │ ✅ Gateway   │ │ ✅ Decisions │ │ ...
    │ ✅ UI        │ │ ✅ Wiki      │ │
    │ ✅ Tests     │ │ ✅ Tutorials │ │
    └──────────────┘ └──────────────┘ └──────────────┘
          │                 │
          │    ← sync → ← sync →
          │    (manual + auto)
          │                 │
    ┌─────┴─────────────────┴────────┐
    │ Developer / User Experience    │
    │ (Unified Wiki + Code in one    │
    │  logical structure)            │
    └────────────────────────────────┘
```

---

## 🔄 **Synchronisierungs-Strategie**

### **Modell: Docs-First mit Code-Copy**

```
1. SINGLE SOURCE OF TRUTH
   ├─ a-townchain-os-docs (Primary)
   └─ a-townchain-os (Mirror/Reference)

2. SYNC DIRECTION
   a-townchain-os-docs → a-townchain-os/docs/
   
3. TRIGGER
   ├─ Manual: ./scripts/sync-docs.sh [--commit] [--push]
   ├─ Pre-Release: Automatisch vor Tagging
   └─ Weekly: (Optional) GitHub Actions Cronjob
```

### **Sync-Prozess (Punkt 3)**

#### **Manual Sync (für Entwickler)**

```bash
# Lokal (wenn beide Repos auf Maschine)
./scripts/sync-docs.sh --commit --push

# Output:
# === A-TownChain OS Docs Sync ===
# Source: ../a-townchain-os-docs
# Target: docs/
# 
# Synchronisiere Dateien...
#   → Main Documentation (.md Dateien)...
#     ✓ docs/ synchronisiert
#   → Root-Dateien...
#     ✓ KONSOLIDIERUNGS_ROADMAP.md
#     ✓ NAMING_CONVENTIONS.md
#     ✓ DECISIONS_REGISTER.md
#   → CI-Templates...
#     ✓ ci-templates/ synchronisiert
# 
# ✓ Synchronisierung abgeschlossen
# 
# Erstelle Commit...
#   [main 7a3f2c9] docs: Sync from a-townchain-os-docs
#
# Pushe zu main...
#   ✓ Push erfolgreich
```

#### **Pre-Release Sync (vor v1.0.0)**

```bash
# Im Release-Prozess (Schritt 1 vor Tag):
./scripts/sync-docs.sh --commit

# Verifiziere kein Diff vor Release:
git diff --exit-code docs/ || echo "Docs nicht aktuell!"
```

#### **Automated Sync (GitHub Actions)**

```yaml
# .github/workflows/docs-sync.yml (neu, optional)
name: Sync Docs

on:
  schedule:
    - cron: '0 0 * * 0'  # Sonntags 00:00 UTC
  workflow_dispatch:

jobs:
  sync:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0
      
      - name: Sync docs from a-townchain-os-docs
        run: |
          git clone https://github.com/A-TownChain-Okosystems/a-townchain-os-docs.git /tmp/docs-repo
          ./scripts/sync-docs.sh --source /tmp/docs-repo --commit --push
```

---

## 📖 **Kapitel-Standard (40 Kapitel)**

### **Struktur**

```
Kap. 1-10   Core Architektur (Kernel, Blockchain, Gateway)
Kap. 11-20  Datenebenen & Services (Wallet, Persistence, P2P)
Kap. 21-30  Sicherheit & Compliance (Crypto, Audit, Legal)
Kap. 31-40  Standards & Entscheidungen (Registry, Roadmap, Policy)
```

### **Datei-Konvention**

```markdown
# Kap. X — [Titel]

> **Quelle:** WIKI_INDEX.md
> **Standard:** ATC-XX
> **Status:** PUBLISHED (DRAFT / TODO)

[Content]
```

### **Registrierung**

Alle Kapitel müssen in `docs/WIKI_INDEX.md` gelistet sein:

```markdown
| Kap. | Titel | Datei | Scope | Status |
|------|-------|-------|-------|--------|
| 5 | ShivaOS Kernel | [KERNEL_FROM_SCRATCH_PLAN.md](./KERNEL_FROM_SCRATCH_PLAN.md) | Bare-Metal Rust | ✅ PUBLISHED |
```

---

## ✅ **Dokumentations-Checkliste (vor Release)**

### **Code-Seite (a-townchain-os)**

- [ ] Alle `.py`/`.ts`/`.rs` Dateien haben Docstrings/Comments
- [ ] Neue Klassen/Funktionen haben entsprechende Kap. im Wiki
- [ ] Tests decken neue Features ab (80%+ Coverage)
- [ ] CHANGELOG.md ist aktualisiert
- [ ] README.md hat aktuellen Status

### **Dokumentations-Seite (a-townchain-os-docs)**

- [ ] Neue `.md` Dateien in `/docs` organisiert
- [ ] WIKI_INDEX.md ist aktualisiert (+ Kap.-Nummer)
- [ ] Links zu Standards (ATC-XX) vorhanden
- [ ] Status = PUBLISHED (nicht DRAFT)
- [ ] DECISIONS_REGISTER.md referenziert (wenn nötig)

### **Sync-Seite**

- [ ] `./scripts/sync-docs.sh --commit --push` ausgeführt
- [ ] Keine Datei-Konflikte in `a-townchain-os/docs/`
- [ ] Git-Status sauber (`git status` zeigt nichts)
- [ ] Push erfolgreich (kein 403/404)

---

## 🛠️ **Wie man neue Dokumentation hinzufügt**

### **Szenario 1: Neuer Standard (ATC-XX)**

```
1. Erstelle: a-townchain-os-docs/docs/standards/ATC-XX_NAME.md
2. Update: a-townchain-os-docs/docs/standards/STANDARDS_REGISTRY.md
3. Update: docs/WIKI_INDEX.md (+ Kap.-Nummer falls Architektur)
4. Sync: ./scripts/sync-docs.sh --commit --push
5. Link in Code: Referenziere ATC-XX in Docstring
6. Commit in a-townchain-os mit "Implements ATC-XX"
```

### **Szenario 2: Neue Kap. (Kap. 41+)**

```
1. Erstelle: a-townchain-os-docs/docs/KAP_41_TITLE.md
2. Starten mit:
   # Kap. 41 — [Titel]
   > **Status:** DRAFT
   > **Standard:** (ATC-XX falls applicable)
3. Update: docs/WIKI_INDEX.md (+ Status: DRAFT)
4. Review: Siehe ✅ Checkliste oben
5. Publish: Status → PUBLISHED, Sync
```

### **Szenario 3: Bug in Dokumentation**

```
1. Identifiziere Fehler in a-townchain-os-docs
2. Fix dort (Single Source of Truth)
3. Sync: ./scripts/sync-docs.sh --commit --push
4. PR mit Beschreibung "docs: Fix [Kap. X]"
```

---

## 🚨 **Häufige Fehler & Lösungen**

### **Fehler 1: Doku nur in a-townchain-os, nicht in a-townchain-os-docs**

**Problem:** Änderung wird überschrieben beim nächsten Sync.

**Lösung:**
```bash
# Kehre Änderung rückgängig
git checkout docs/FILE.md

# Mache Änderung in a-townchain-os-docs statt
cd ../a-townchain-os-docs
# [edit]
git commit -m "docs: ..."
git push

# Zurück zu a-townchain-os
cd ../a-townchain-os
./scripts/sync-docs.sh --commit --push
```

### **Fehler 2: Sync fehlgeschlagen ("Source not found")**

**Lösung:**
```bash
# Überprüfe Source-Pfad
SOURCE_REPO=/full/path/to/a-townchain-os-docs ./scripts/sync-docs.sh

# Oder clone die Repo temporär
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os-docs.git /tmp/docs
SOURCE_REPO=/tmp/docs ./scripts/sync-docs.sh --commit
```

### **Fehler 3: WIKI_INDEX.md nicht synchronisiert**

**Lösung:**
```bash
# Die WIKI_INDEX.md wird in a-townchain-os erstellt (nicht synced)
# Wenn Änderungen nötig:
# - Editiere docs/WIKI_INDEX.md direkt in a-townchain-os
# - ODER editiere Quelle + sync

# Prüfe Inhalt nach Sync:
git diff docs/WIKI_INDEX.md
```

---

## 📊 **Metriken & Monitoring**

### **Sync-Health**

```bash
# Prüfe ob docs/ aktuell ist
./scripts/sync-docs.sh (ohne --commit) 
# Sollte: "✓ Synchronisierung abgeschlossen" ohne Änderungen

# Wenn Änderungen gezeigt werden:
git diff --stat docs/
# → Zeigt wie viele Dateien veraltet sind
```

### **Release-Checkliste Integration**

```
Pre-Release-Prozess:
├─ Schritt 1: Docs-Sync → ./scripts/sync-docs.sh --commit
├─ Schritt 2: Tests → make test
├─ Schritt 3: Workflow-Check → CI passes
├─ Schritt 4: Release Notes → CHANGELOG.md
└─ Schritt 5: Tag → git tag v1.0.0
```

---

## 🔗 **Verwandte Dokumente**

- [`docs/WIKI_INDEX.md`](../../project/WIKI_INDEX.md) — Alle 40 Kapitel + Status
- [`AGENT_POLICY.md`](../../AGENT_POLICY.md) — Sync-Reihenfolge (Schritt 2)
- ``scripts/sync-docs.sh`` (Skript im Code-Monorepo archiviert) — Automatisches Sync-Script
- [a-townchain-os-docs](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs) — Primary Docs Repo

---

## ✅ **Status: v1.0 LIVE**

**Rollout:** 2026-08-04  
**Agent:** Copilot (@copilot)  
**Nächster Review:** Nach v1.0.0 Release

---

## 📝 **Changelog dieser Strategie**

### v1.0 (2026-08-04)
- Zwei-Repo-Modell etabliert
- 40-Kapitel-Standard definiert
- Sync-Script (`sync-docs.sh`) implementiert
- WIKI_INDEX.md erstellt
- Checklisten + Error-Handling dokumentiert

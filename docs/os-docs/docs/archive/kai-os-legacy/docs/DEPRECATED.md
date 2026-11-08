# 📚 Deprecated Components — A-TownChain Ökosystem

> Diese Datei dokumentiert alle veralteten Komponenten.
> Gemäß Richtlinie: **Code wird nicht gelöscht — er bleibt im Repository
> und ist hier vollständig dokumentiert.**

---

## Veraltete Dateien (Stand: 2026-06-09)

| Datei | Repo | Ersetzt durch | Grund |
|-------|------|---------------|-------|
| `KAI_OS_SUMMARY.py` | a-townchain-os | `docs/kai-os-wiki.md` + `ECOSYSTEM.md` | Manuelles Zusammenfassungs-Script, nicht mehr aktuell |
| `atc_issues_summary.py` | a-townchain-os | KaiOsTodo-Datenbank (Base44 Aurora) | Ersetzt durch automatisiertes Issue-Tracking |
| `atc-kernel/net/atcnet.py` | atc-kernel | [`atcnet` Repo](https://github.com/A-TownChain-Okosystems/atcnet) | Duplikat entfernt — eigenständiges Repo |
| `build_pdf.py` | kai-os-wiki | Markdown direkt / Docusaurus | PDF-Export nicht mehr primäres Format |

---

## Veraltete Konzepte (dokumentiert, nicht mehr aktiv)

### Legacy Python Blockchain (v1.x)
- **Was:** Erste Python-Implementierung von Blockchain/Consensus
- **Wo war es:** `blockchain/consensus/*.py` (legacy, v1.x)
- **Warum veraltet:** Ersetzt durch Substrate/Rust-basierte Implementierung (Sprint 2.1+)
- **Dokumentation:** KAI-OS Wiki Kapitel 28 (Migration Path), docs/MIGRATION_MAP.md
- **Status:** Code bleibt erhalten als Referenzimplementierung

### Einfaches PoW-only Consensus
- **Was:** Reines SHA-256 PoW ohne PoS/PoH
- **Ersetzt durch:** `blockchain/consensus/hybrid_consensus.py` (PoH→PoS→PoW)
- **Dokumentation:** KAI-OS Wiki Kapitel 5 (Consensus-Architektur)

### Zentrales API ohne Gateway
- **Was:** Direkte Verbindung Frontend → Backend
- **Ersetzt durch:** Frontend → Gateway (Port 4000) → Backend (Port 5000)
- **Dokumentation:** KAI-OS Wiki Kapitel 14 (API-Architektur)

---

## Migration Guide

Für alle deprecated Komponenten gilt:
1. Neuen Import/Pfad in der Tabelle oben nachschlagen
2. Bei Fragen: KAI-OS Wiki Kapitel 30 (Migration Map)
3. Alte Tests bleiben erhalten und werden als `test_legacy_*` markiert

---

*Automatisch verwaltet von Aurora AI | A-TownChain-Okosystems*

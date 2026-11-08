# Issue #60 — v3.2.1 — Integration: AIKernel in IPC Bus registrieren

> **Status:** CLOSED | **Erstellt:** 2026-06-12 | **Labels:** enhancement, ai, priority:medium, kernel
> **GitHub:** https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/60

---

## Beschreibung

## Beschreibung
`ai_kernel.py` und `ipc_bus.py` sind beide implementiert (v3.2.0), aber noch nicht verbunden.
Der IPC Bus hat keinen `ai.query` Topic-Handler.

## Aufgaben
- [ ] In `ipc_bus.py`: Topic `ai.query` registrieren
- [ ] In `ipc_bus.py`: Topic `ai.review` für Code-Reviews registrieren
- [ ] In `ai_kernel.py`: IPC Bus Subscriber einbauen
- [ ] `kernel.py` (Haupt-Kernel): beide beim Start 

---

## Aufgaben

- [ ] Implementierung
- [ ] Tests
- [ ] Dokumentation
- [ ] Code-Review

---

## Notion-Querverweis

- **Master Roadmap:** [Notion Roadmap](https://app.notion.com/p/Master-Roadmap-Synced-67-Issues-All-Closed-373b826db85c8125ba83f04995191bf0)

---

*Auto-generiert von Aurora v3.2 · 05.07.2026*

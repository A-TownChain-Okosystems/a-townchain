# Issue #18 — 🐳 [Testnet] Docker Compose — 5-Node lokales Netzwerk

> **Status:** CLOSED | **Erstellt:** 2026-05-19 | **Labels:** enhancement, blockchain, priority:medium, testnet, devops, in-progress
> **GitHub:** https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/18

---

## Beschreibung

## 🎯 Ziel
Mit einem einzigen `docker-compose up` startet ein vollständiges lokales Testnetzwerk mit 5 Nodes (1 Bootstrap, 1 Validator, 1 Miner, 2 Full Nodes).

## 📋 Aufgaben
- [ ] `Dockerfile` für A-TownChain OS Backend
- [ ] `docker-compose.testnet.yml` — 5 Service-Definitionen
- [ ] Umgebungsvariablen pro Node (NODE_ID, NODE_TYPE, STAKE, BOOTSTRAP)
- [ ] Shared Volume für Block-Daten (optional)
- [ ] Healthcheck pro Service
- [ ] `scripts/testnet-start.sh` — Convenience-Script
- [ ] `scripts/t

---

## Aufgaben

- [ ] Implementierung
- [ ] Tests
- [ ] Dokumentation
- [ ] Code-Review

---

## Abhängigkeiten

_(Keine expliziten Abhängigkeiten dokumentiert.)_

---

## Notion-Querverweis

- **Master Roadmap:** [Notion](https://app.notion.com/p/Master-Roadmap-Synced-67-Issues-All-Closed-373b826db85c8125ba83f04995191bf0)
- **Live-Status:** [Notion Kap. 31](https://app.notion.com/p/Live-Status-Kap-31)

---

*Auto-generiert von Aurora v3.2 · 05.07.2026*

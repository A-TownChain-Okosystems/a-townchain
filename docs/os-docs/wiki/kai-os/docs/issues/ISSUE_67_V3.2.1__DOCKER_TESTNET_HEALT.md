# Issue #67 — v3.2.1 — Docker: Testnet Health-Checks + CI/CD Pipeline

> **Status:** CLOSED | **Erstellt:** 2026-06-12 | **Labels:** priority:medium, build, testnet, devops
> **GitHub:** https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/67

---

## Beschreibung

## Beschreibung
`docker/docker-compose.testnet.yml` (v3.2.0) ist fertig, aber die CI/CD Pipeline fehlt noch.

## Aufgaben
- [ ] `.github/workflows/testnet_ci.yml` erstellen
  - Trigger: Push auf `main`, PR auf `main`
  - Job 1: `docker compose up -d` — Testnet starten
  - Job 2: Warte auf Health-Check aller 3 Nodes
  - Job 3: Smoke-Test — API `/health` auf allen Nodes
  - Job 4: `docker compose do

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

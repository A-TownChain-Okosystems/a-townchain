# Issue #17 — ⛓ [Testnet] Longest-Chain-Rule — Fork-Auflösung

> **Status:** CLOSED | **Erstellt:** 2026-05-19 | **Labels:** enhancement, blockchain, priority:high, testnet, in-progress
> **GitHub:** https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/17

---

## Beschreibung

## 🎯 Ziel
Bei konkurrierenden Chains (Forks) gewinnt immer die längste gültige Chain — genau wie bei Bitcoin. Implementierung der Fork-Auflösungslogik.

## 📋 Aufgaben
- [ ] `HybridConsensus.resolve_fork(chain_a, chain_b)` implementieren
- [ ] Fork-Erkennung: zwei Blöcke mit gleichem `prev_hash`
- [ ] Orphan-Blöcke verwalten (temporärer Pool)
- [ ] Chain-Reorg: lokale Chain auf längere umstellen
- [ ] TX-Revert: TXs aus verworfenen Blöcken zurück in Mempool
- [ ] Reorg-Event im EventBus publizier

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

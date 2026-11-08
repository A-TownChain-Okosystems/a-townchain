# Issue #61 — v3.2.1 — Integration: BlockGossip in P2PNode einbinden

> **Status:** CLOSED | **Erstellt:** 2026-06-12 | **Labels:** enhancement, blockchain, priority:medium, networking
> **GitHub:** https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/61

---

## Beschreibung

## Beschreibung
`block_gossip.py` (v3.2.0) ist standalone implementiert, aber noch nicht mit `p2p_node.py` verbunden.

## Aufgaben
- [ ] `P2PNode` erhält ein `BlockGossip`-Attribut
- [ ] `P2PNode.on("INV", ...)` Handler → `gossip.handle_inv()`
- [ ] `P2PNode.on("block", ...)` Handler → `gossip.handle_block()`
- [ ] `P2PNode.on("tx", ...)` Handler → `gossip.handle_tx()`
- [ ] `P2PNode.announce_bloc

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

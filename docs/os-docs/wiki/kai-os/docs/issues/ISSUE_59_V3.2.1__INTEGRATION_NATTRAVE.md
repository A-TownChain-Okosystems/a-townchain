# Issue #59 — v3.2.1 — Integration: NATTraversal in P2PNode einbinden

> **Status:** CLOSED | **Erstellt:** 2026-06-12 | **Labels:** enhancement, priority:medium, networking
> **GitHub:** https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/59

---

## Beschreibung

## Beschreibung
`nat_traversal.py` (v3.2.0) existiert als eigenständiges Modul, ist aber noch nicht in `p2p_node.py` integriert.

## Aufgaben
- [ ] `P2PNode.__init__()` — `NATTraversal` instanziieren
- [ ] `P2PNode.start()` — `nat.discover()` aufrufen, externe IP/Port loggen
- [ ] Wenn externe IP bekannt: in Peer-Announcements (`INV`) einbetten
- [ ] `P2PNode.hole_punch(peer_id)` — Methode hinzufü

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

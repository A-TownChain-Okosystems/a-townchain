# Issue #62 — v3.2.1 — Integration: ServiceDiscovery im API Gateway aktivieren

> **Status:** CLOSED | **Erstellt:** 2026-06-12 | **Labels:** enhancement, priority:medium, gateway
> **GitHub:** https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/62

---

## Beschreibung

## Beschreibung
`service_discovery.py` (v3.2.0) ist implementiert, aber das API Gateway (`gateway/main.py` / `gateway/app.py`)
nutzt es noch nicht für dynamisches Routing.

## Aufgaben
- [ ] Gateway-Start: `ServiceDiscovery.start_health_monitor()` aufrufen
- [ ] Route-Handler: `discovery.get_url("chain", "/blocks")` statt hardcodierter Ports
- [ ] Fallback wenn Service unhealthy → 503 zurückgeben


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

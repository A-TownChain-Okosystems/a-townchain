# 📡 A-TownChain OS — API Reference (Index)

> **Status:** Index/Übersichts-Dokument — verweist auf die vollständigen, modul-spezifischen API-Dokumentationen.
> Diese Datei existierte bisher nicht, wurde aber von `devnet/README.md` referenziert — als Teil des
> Reality-Checks vom 06.07.2026 nachgetragen (siehe `docs/REALITY_CHECK_2026-07-06.md`).

## Übersicht

A-TownChain OS exponiert seine Dienste über den zentralen **API Gateway** (Port 4000), der Anfragen
an die dahinterliegenden Module (Core, Chain, Wallet, AI, Game — Ports 5000-5004) weiterleitet.
Vollständige, modul-spezifische Endpunkt-Dokumentation findet sich in den jeweiligen Wiki-Kapiteln:

| Modul | API-Dokumentation |
|-------|--------------------|
| Franchise Factory | [docs/wiki/franchise/docs/API.md](./wiki/franchise/docs/API.md) |
| Overview / Core | [docs/wiki/overview/docs/API.md](./wiki/overview/docs/API.md) |
| ShivaOS UI | [docs/wiki/ui/docs/API.md](./wiki/ui/docs/API.md) |
| Gateway-Architektur | [docs/architecture/GATEWAY.md](./architecture/GATEWAY.md) |

## Gateway-Grundlagen

- **Basis-URL (lokal/Devnet):** `http://localhost:4000`
- **Service-Discovery:** Der Gateway registriert Backend-Services dynamisch (siehe `gateway/service_discovery.py`)
- **Health-Check:** `GET /health` — Standard-Endpunkt aller Services

## Verwandte Dokumente

- [ATCLang Guide](./atclang-guide.md) — Sprachreferenz für Smart Contracts
- [ATCLang Spezifikation](./atclang/ATCLANG_SPEC_FULL.md) — vollständige Sprach-Spec
- [Architektur-Übersicht](./architecture/) — alle System-Komponenten

---
*Nachgetragen von Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5) im Rahmen des 24-Repo Reality-Checks, 06.07.2026.*

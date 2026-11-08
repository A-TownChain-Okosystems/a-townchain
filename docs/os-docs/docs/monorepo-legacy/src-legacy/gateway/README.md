# ATC Gateway

## Status: Re-aktiviert & sauber aus dem Monorepo ausgegliedert (09.07.2026)

API-Gateway des A-TownChain-Oekosystems -- Routing, Auth-Middleware,
Rate-Limiting, Signature-Verification, Service Discovery.

Dieses Repo war zuvor als "archiviert/migriert nach a-townchain-os" markiert
(Beschreibung war stale) -- der Gateway-Code lag tatsaechlich weiterhin im
Monorepo. Er wurde jetzt sauber hierher migriert und das Repo ist wieder die
kanonische Quelle fuer Gateway-Code.

## Struktur -- zwei parallele Implementierungen

- **`python/`** -- die stabile, produktive Implementierung (FastAPI-Stil,
  `main.py`, `router.py`, `middleware/*.py`). Dies ist die aktuell lauffaehige
  Version.
- **`atclang/`** -- experimenteller Port derselben Gateway-Logik nach ATCLang
  (`main.atc`, `router.atc`, `middleware/*.atc`). Laut `REALITY_STATUS.md` im
  Hauptrepo hat der ATCLang-Parser aktuell noch Probleme mit Generics/
  Modul-Bloecken (96/176 Dateien parsen fehlerfrei) -- der Status dieses Ports
  ist entsprechend als experimentell/nicht garantiert lauffaehig einzustufen.
- **`docs/ARCHITECTURE.md`** -- Architektur-Dokumentation des Gateways.

## Wichtig

- **Kanonische Quelle fuer Projektstatus:** `REALITY_STATUS.md` im Root von
  `a-townchain-os` -- nicht dieses README bei Widersprüchen.
- Copyright-Header (Michael Wroblewski) unveraendert aus dem Monorepo
  uebernommen.

## Naechste Schritte (offen)

- Verifizieren, ob `python/`- und `atclang/`-Version funktional aequivalent
  sind oder der ATCLang-Port veraltet/unvollstaendig ist.
- CI fuer dieses Repo aufsetzen (Tests aus `tests/test_gateway_full.py` /
  `tests/unit/test_gateway.py` lagen im Monorepo, noch nicht hierher migriert
  -- ggf. in Folgeschritt nachziehen).

---
*Migriert: 09.07.2026.*

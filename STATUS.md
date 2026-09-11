# STATUS — atc-sdk

**Stand:** 2026-08-05
**Version:** v1.0.0
**Status:** SKELETON → INITIALIZING

## Übersicht
Dieses Repo ist Teil des A-TownChain OS Ökosystems (26 Repos).
Org: [A-TownChain-Okosystems](https://github.com/A-TownChain-Okosystems)

## Module
- [ ] Initial setup
- [ ] Core implementation
- [ ] Tests
- [ ] Documentation

## Abhängigkeiten
Siehe DEPENDENCIES.md

## Letzte Änderungen
- 2026-08-05: Repo initialisiert durch Aurora Sync Agent

- 11.09.2026 (SCR-0110): Erster Chain-Access-Consumer — modules/atc-cli/src/rpc_client.rs: minimaler JSON-RPC-2.0-Client (chain_id/boot_hash/peers/ping) gegen den atc-node Devnet-RPC (SCR-0109-Protokoll), std-only, 3 Unit-Tests inkl. Mock-Node-Roundtrip und ehrlichem Verbindungsfehler-Fall (CI-verifiziert). Ehrlich: kein TLS, keine Verbindungs-Wiederverwendung, keine Retry-Logik; echte Node-Anbindung im Integrationstest folgt.

---
document_id: ATC-DOC-ATCNOD-002
title: "Project Status"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
updated: 2026-09-10
standard: ATC-STD-MD-001
scr: SCR-0074
---

# Project Status — ATC Node

| Property | Value |
|---|---|
| Repository | ATC Node |
| Version | 0.1.0 |
| Status | development |
| Build | PASS (cargo, MVP-Kern Config+PeerTable, CI-gruen SCR-0083) |
| Tests | NOT RUN — Testplan definiert, Suite entsteht mit Implementierung |
| Security | NOT AUDITED — SECURITY.md-Prozess aktiv, Audit ausstehend |
| Documentation | compliant |
| Last Audit | 2026-09-10 (SCR-0074 Org-Compliance-Scan) |

## Status Summary

`ATC Node` befindet sich im Status `development` (ATC-STD-201). Diese Datei wurde
im Org-Compliance-Scan SCR-0074 nachgezogen, weil das Pflichtartefakt STATUS.md
fehlte. Nach dem Prinzip **No status without evidence** werden keine PASS-Zustände
behauptet; Build/Test-Evidence entsteht erst mit der Implementierung und wird
dann über CI-Records referenziert.

- 11.09.2026 (SCR-0106): Devnet-Bootstrap Stufe 1 — Genesis (config/devnet/genesis.json + src/bootstrap.rs) mit Validierung, deterministischem FNV-1a-Boot-Hash (MVP-Platzhalter, nicht kryptographisch) und 2-Node-Peer-Join-Smoke (6 Unit-Tests, CI-verifiziert). KEIN echtes Netzwerk, KEIN RPC, KEINE Blockproduktion — Devnet-Gate Stufe 2 offen (F-139).

- 11.09.2026 (SCR-0108): Devnet-RPC Stufe 2 — src/rpc.rs mit DevnetRpc (Chain-ID/Boot-Hash/Peer-Count-Schnappschuss) und Zeilenprotokoll ueber echtes TCP (CHAIN_ID/BOOT_HASH/PEERS/PING, ein Request pro Verbindung); 2 Unit-Tests inkl. echtem Socket-Roundtrip (CI-verifiziert). Ehrlich: KEIN JSON-RPC, KEINE Auth/TLS (Devnet-only), Gossip/Blockproduktion Stufe 3 offen (F-140).

- 11.09.2026 (SCR-0109): Devnet-RPC Stufe 3 — JSON-RPC-2.0-Subset (chain_id/boot_hash/peers/ping, -32601-Fehlercode) ueber denselben TCP-Socket mit Auto-Erkennung (Zeile vs. JSON-Objekt); ehrlich minimale Feldextraktion, kein voller JSON-Parser, keine Batch/Notifications; 3 neue Unit-Tests inkl. JSON-TCP-Roundtrip (CI-verifiziert). Auth/TLS, Wallet/Explorer/SDK-Consumer und Gossip bleiben offen (F-140).

- 11.09.2026 (SCR-0112): ATC-Node als Prozess startbar — src/main.rs: Devnet-Bootstrap (Genesis, Peer-Join) beim Start, danach Dauerdienst des Chain-Access auf TCP (Standard-Adresse 127.0.0.1:39471, per Argument ueberschreibbar), stderr-Log mit Chain-ID/Boot-Hash/Peers. Ehrlich: Devnet-only, kein TLS/Auth, kein Gossip, keine Blockproduktion (Stufe offen). Docker/Compose und echte Mehr-Prozess-Devnets bauen darauf auf.

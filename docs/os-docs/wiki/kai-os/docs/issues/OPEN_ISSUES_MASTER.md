# 📋 Offene Issues & To-Dos — A-TownChain OS

> Stand: 2026-06-09 | Quelle: GitHub API  
> Gesamt: **18 offen** | **201 offene Sub-Tasks** | **4 geschlossen**

---

## 🔴 Priority HIGH

---

### #22 🚀 KAI-OS v1.3.2-beta — Substrate-Integration + DevOps
**Sprint:** 2.1 | **Milestone:** M1 | **Sub-Tasks:** 2 offen, 4 erledigt

- [x] Substrate Node Template eingebunden
- [x] pallet-poh implementiert (276 Zeilen, 8 Tests)
- [x] pallet-ai-registry implementiert
- [x] pallet-agent-registry implementiert
- [ ] PR `feature/kai-os-integration` → `main` mergen
- [ ] Wiki Auto-Sync Workflow aktivieren (wiki-sync.yml)

---

### #20 🧪 API-Gateway-Tests — Unit & Integrationstests Port 4000
**Dateien:** `gateway/main.py`, `gateway/router.py`, `gateway/middleware/`

- [ ] `tests/test_gateway.py` — Unit-Tests für `create_app()`
- [ ] `tests/test_gateway.py` — Integrationstests für alle Routen
- [ ] Test: Auth-Middleware (gültige/ungültige API-Keys)
- [ ] Test: Rate-Limiter (Burst-Schutz, 100 req/min)
- [ ] Test: Service-Routing zu Core:5000 / Chain:5001 / Wallet:5002 / AI:5003 / Game:5004
- [ ] Test: Request-Logger Ausgabe
- [ ] Test: Signature-Verify Middleware
- [ ] CI: Tests in GitHub Actions einbinden (Coverage-Ziel: >80%)

---

### #17 ⛓ [Testnet] Longest-Chain-Rule — Fork-Auflösung
**Datei:** `shivaos/consensus/shiva_consensus.py`

- [ ] `HybridConsensus.resolve_fork(chain_a, chain_b)` implementieren
- [ ] Fork-Erkennung: zwei Blöcke mit gleichem `prev_hash`
- [ ] Längste gültige Chain gewinnt (kumulative Höhe / Arbeit)
- [ ] Orphan-Block Pool: verworfene Blöcke zwischenspeichern
- [ ] Reorganisation (Reorg): Chain-Swap bei längerer Konkurrenz-Chain
- [ ] Reorg-Limit: max. 100 Blöcke tief
- [ ] Unit-Tests: `test_fork_resolution.py` (min. 5 Szenarien)
- [ ] Dokumentation: Fork-Auflösungs-Diagramm in Wiki
- [ ] Integration mit Block-Propagation (#15)
- [ ] Monitoring: Fork-Events im Dashboard anzeigen

---

### #16 🔄 [Testnet] Initial Sync — Neue Nodes synchronisieren
**Dateien:** `shivaos/net/atcnet.py`, `blockchain/nodes/node.py`

- [ ] `sync_from_peer(peer_address)` — Chain-Download in 50-Block-Batches
- [ ] Sync-Fortschritt im Dashboard anzeigen (%)
- [ ] Checkpoint-System: vertrauenswürdige Block-Hashes hardcoden
- [ ] Parallelisierung: von mehreren Peers gleichzeitig syncen
- [ ] Sync-Abbruch & Resume bei Verbindungsabbruch
- [ ] Validierung jedes heruntergeladenen Blocks (Hash + Signatur)
- [ ] Sync-Timeout: 30s pro Peer
- [ ] Unit-Tests: `test_initial_sync.py`
- [ ] Dokumentation in `docs/architecture/TESTNET.md` aktualisieren

---

### #15 📡 [Testnet] Block Propagation — P2P Block Broadcasting
**Datei:** `blockchain/nodes/p2p_propagation.py` (bereits vorhanden)

- [ ] `broadcast_block(block)` — Block an alle Peers senden
- [ ] `broadcast_tx(tx)` — Transaktion an alle Peers senden
- [ ] Deduplication: bereits gesehene Blöcke ignorieren (Bloom Filter / Set)
- [ ] Retry-Logik bei fehlgeschlagener Übertragung (3 Versuche)
- [ ] Gossip-Protokoll: exponentielles Backoff
- [ ] Peer-Score: unzuverlässige Peers downranken
- [ ] Integration mit Bootstrap Node (#14)
- [ ] Unit-Tests: `test_p2p_propagation.py` erweitern
- [ ] Monitoring: Propagationszeit messen

---

### #14 🌐 [Testnet] Bootstrap Node — P2P Discovery Service
**Datei:** `blockchain/nodes/discovery.py` (bereits vorhanden — vervollständigen)

- [ ] UDP-basierter Node-Announcer finalisieren
- [ ] Bootstrap-Node Config in `config/settings.json` eintragen
- [ ] `NodeRegistry` — aktive Peers verwalten (max. 200)
- [ ] Heartbeat: inaktive Nodes nach 60s entfernen
- [ ] REST-Endpoint: `GET /peers` — aktive Node-Liste zurückgeben
- [ ] Persistenz: bekannte Peers in `data/peers.json` speichern
- [ ] Docker-ready: Bootstrap-Node als separater Service in docker-compose
- [ ] TLS-Verschlüsselung für Peer-Verbindungen
- [ ] Geografische Verteilung: Region-Awareness
- [ ] Unit-Tests: `test_discovery.py` erweitern

---

### #8 🌐 Multi-Node Testnet — P2P Netzwerk live
**Abhängig von:** #14, #15, #16, #17, #18

- [ ] Phase 1: Node-Discovery via UDP Broadcast
- [ ] Phase 2: Peer-Verbindungen aufbauen (TCP, max. 25 Peers pro Node)
- [ ] Phase 3: Block-Sync zwischen Nodes
- [ ] Phase 4: Konsens dezentral erreichen (PoI+PoS)
- [ ] Mindestens 3 Nodes synchronisiert (lokal)
- [ ] End-to-End-Test: TX auf Node A → Bestätigung auf Node C
- [ ] Monitoring: Netzwerk-Topologie visualisieren
- [ ] Stress-Test: 100 TX/s unter Last
- [ ] Dokumentation: Testnet-Setup-Guide schreiben
- [ ] Video-Demo: Testnet live
- [ ] Public Testnet: Bootstrap-Node auf VPS deployen
- [ ] Faucet: Test-ATC automatisch ausgeben
- [ ] Explorer: Testnet-Blöcke live anzeigen (#5)
- [ ] Announcement: Testnet-Launch auf GitHub ankündigen

---

### #3 ⚔️ Shivamon Battle UI — Animierte Kämpfe im Browser
**Dateien:** `frontend/index.html`, `frontend/assets/js/battle.js` (neu)

- [ ] Battle-Seite in `frontend/index.html` (Sidebar: `⚔️ Battle`)
- [ ] Shivamon-Auswahl: eigene Collection vs. Gegner-Token-ID
- [ ] Animierter Kampfablauf — Runde für Runde mit HP-Bars
- [ ] Angriffs-Animationen per CSS/Canvas
- [ ] Battle-Log: Aktionen als Text-Stream
- [ ] Sieg/Niederlage-Screen mit XP-Gewinn
- [ ] Backend: `POST /api/game/battle` Endpoint

---

### #2 🤖 Gemini AI Integration ⚠️ ISSUE SCHLIESSEN
**Status:** ✅ Vollständig implementiert am 2026-06-08

- [x] `backend/api/routes/ai.py` — Gemini API Endpoint
- [x] API-Key Management via `.env`
- [x] Gemini AI Orchestrator v2.1.0
- [x] ATCPromptContext + Cache
- [x] REST Routes vollständig
- [x] Settings UI mit localStorage
- [x] Live-Chat im Dashboard
- [ ] ⚠️ **Issue #2 auf GitHub schließen** (Label `done` vorhanden, Issue noch offen)

---

## 🟡 Priority MEDIUM

---

### #19 📊 [Testnet] Node-Monitoring Dashboard
**Datei:** `frontend/index.html`

- [ ] Node-Monitor Seite (Sidebar: `🌐 Nodes`)
- [ ] Node-Karten: ID · Typ · Online/Offline · Peers · Chain-Height · Mempool
- [ ] Live-Refresh alle 5 Sekunden (WebSocket oder Polling)
- [ ] Sync-Fortschrittsbalken pro Node
- [ ] Verbindungs-Graph: welche Nodes sind verbunden
- [ ] Alert bei Node-Ausfall (rot markieren)
- [ ] Backend: `GET /api/nodes/status` Endpoint
- [ ] Historische Daten: Node-Uptime-Chart (letzte 24h)
- [ ] Export: Node-Status als JSON-Report
- [ ] Responsive Design (Mobile-Kompatibilität)
- [ ] Dark-Mode Unterstützung

---

### #18 🐳 [Testnet] Docker Compose — 5-Node lokales Netzwerk
**Datei:** `docker-compose.testnet.yml`, `Dockerfile`

- [ ] `Dockerfile` für A-TownChain OS Backend
- [ ] `docker-compose.testnet.yml` mit 5 Services (Bootstrap, Validator, Miner, 2x Full Node)
- [ ] Umgebungsvariablen pro Node-Typ (.env.testnet)
- [ ] Internes Docker-Netzwerk `atc-testnet`
- [ ] Volume für Chain-Daten (Persistenz zwischen Restarts)
- [ ] Health-Checks für alle Services
- [ ] `make testnet-up` / `make testnet-down` Shortcut (Makefile)
- [ ] Log-Aggregation: alle Node-Logs in einer Ausgabe
- [ ] Port-Mapping Dokumentation
- [ ] README: Testnet-Start-Guide
- [ ] GitHub Actions: Testnet-Smoke-Test nach jedem Push
- [ ] docker-compose.override.yml für lokale Entwicklung

---

### #13 🛒 ATC Marketplace — Shivamon kaufen & verkaufen
**Datei:** `blockchain/contracts/marketplace/marketplace_contract.py`

- [ ] `list_for_sale(token_id, seller, price_atc)`
- [ ] `buy(token_id, buyer)`
- [ ] `cancel_listing(token_id, seller)`
- [ ] `get_listings()` — alle aktiven Listings
- [ ] `get_listing(token_id)` — einzelnes Listing
- [ ] Auktions-Mechanismus (Highest Bidder nach X Blöcken)
- [ ] Escrow: ATC wird beim Kauf gesperrt
- [ ] Royalties: 2.5% an Original-Ersteller
- [ ] Frontend: Marketplace-Seite (Sidebar: `🛒 Market`)
- [ ] Filter: Preis / Rarity / Element / Level
- [ ] Sortierung: Preis aufsteigend/absteigend
- [ ] Warenkorb / Watchlist
- [ ] Transaktions-Bestätigungs-Dialog
- [ ] Backend: `GET/POST /api/game/marketplace`
- [ ] On-Chain: Sale-Event loggen
- [ ] Tests: `test_marketplace.py`
- [ ] Dokumentation: Marketplace-Guide
- [ ] Gas-Fee Kalkulator
- [ ] Batch-Listing: mehrere Shivamon auf einmal listen
- [ ] Rabatt-System für ATC-Staker
- [ ] Notification: Kauf-Bestätigung per Event

---

### #12 ⛓ Solidity Smart Contracts — On-Chain ATC Token
**Datei:** `blockchain/contracts/solidity/`

- [ ] `ATC Token.sol` — ERC20 (ATC-8300)
- [ ] `Shivamon.sol` — ERC721 NFT (ATC-9000)
- [ ] `ATCGovernance.sol` — DAO Voting (ATC-9900)
- [ ] `GenesisToken.sol` — ATC-001 Genesis
- [ ] `Marketplace.sol` — ERC721 Marketplace
- [ ] Hardhat-Konfiguration (`hardhat.config.js`)
- [ ] Unit-Tests für alle Contracts (`test/*.js`)
- [ ] Deployment-Script (Sepolia / Polygon Mumbai)
- [ ] ABI-Export für Frontend-Integration
- [ ] Etherscan-Verifikation
- [ ] Slither Security-Audit

---

### #11 🥚 Shivamon Breeding — Gen 2 NFT Züchtung
**Datei:** `blockchain/contracts/shivamon/shivamon_contract.py`

- [ ] `breed(parent1_id, parent2_id, owner)` implementieren
- [ ] DNA-Mixing: Stats 50/50 + zufällige Mutation (5% Chance)
- [ ] Breed-Cooldown: 24h zwischen Züchtungen
- [ ] Breeding-Fee in ATC Token
- [ ] Rarity-System: Common(60%) / Rare(25%) / Epic(10%) / Legendary(5%)
- [ ] Gen-2 NFT minten mit neuer Token-ID
- [ ] Max-Generation: Gen-5 Limit
- [ ] Frontend: Breeding-UI im Dashboard
- [ ] Animations: Ei-Schlüpf-Animation
- [ ] Backend: `POST /api/game/breed`
- [ ] Tests: `test_breeding.py`
- [ ] On-Chain: Breeding-Event loggen
- [ ] Genealogie-Baum: Eltern-Kind-Beziehung anzeigen
- [ ] Seltenheits-Garantie: 1 Epic pro 10 Breeds
- [ ] Breeding-Statistiken im Dashboard
- [ ] Gemeinsames Breeding mit anderem User (PvP Breed)
- [ ] Seasonal Breeding Events (limitierte Rassen)

---

### #9 🏛 Governance Contract (ATC-9900) — DAO Voting
**Datei:** `blockchain/contracts/governance/governance_contract.py`

- [ ] `Proposal` Datenstruktur
- [ ] `create_proposal(title, description, options, voting_period)`
- [ ] `vote(proposal_id, voter, option)` — ATC-gewichtetes Voting
- [ ] `execute_proposal(proposal_id)` — nach Ablauf ausführen
- [ ] Quorum-Prüfung (min. 10% ATC-Supply)
- [ ] Veto-Mechanismus (33% Nein = abgelehnt)
- [ ] Delegation: Voting-Power übertragen
- [ ] Frontend: Governance-Seite (Sidebar: `🏛 DAO`)
- [ ] Proposal-Timeline: Abstimmungsfrist anzeigen
- [ ] Results-Chart: Balkendiagramm der Stimmen
- [ ] On-Chain: Voting-Ergebnis unveränderlich
- [ ] Tests: `test_governance.py`
- [ ] Dokumentation: DAO-Guide
- [ ] Emergency-Shutdown: Multi-Sig Override
- [ ] Time-Lock: 24h zwischen Proposal-Annahme und Ausführung
- [ ] Snapshot: Voting-Power bei Proposal-Erstellung einfrieren
- [ ] Off-Chain Signaling: Meinungsumfrage vor Proposal

---

### #7 📦 Build System — EXE / AppImage Installer
**Datei:** `build/build.py`

- [ ] PyInstaller-Config ausbauen (build.spec)
- [ ] Single-File EXE: Gateway + Backend + Frontend
- [ ] Auto-Start: Gateway bootet beim EXE-Start
- [ ] Embedded Browser (PyWebView) für Dashboard
- [ ] Windows: `.exe` + NSIS-Installer
- [ ] Linux: `.AppImage` Build
- [ ] GitHub Actions: Auto-Build bei Release-Tag

---

### #5 🌐 ATC Blockchain Explorer — Block & TX Browser
**Datei:** `frontend/index.html` (Sidebar: `🔍 Explorer`)

- [ ] Explorer-Seite im Dashboard
- [ ] Block-Liste: Height, Hash, Timestamp, TX-Anzahl
- [ ] TX-Detail: von/an/Betrag/Status
- [ ] Adress-Suche: Balance + TX-History
- [ ] Live-Update: neuer Block → Explorer aktualisiert sich
- [ ] Backend: `GET /api/blockchain/explorer/blocks`
- [ ] Pagination für lange Block-Listen

---

## 🟢 Priority LOW

---

### #10 🌉 Cross-Chain Bridge — ATC ↔ EVM Interoperabilität
**Datei:** `blockchain/bridge/bridge_contract.py` (neu)

- [ ] Lock/Mint Mechanismus implementieren
- [ ] Lock-Contract auf A-TownChain
- [ ] Mint-Contract auf EVM-Seite (Wrapped-ATC)
- [ ] Bridge-Relayer-Service
- [ ] Unterstützte Chains: Sepolia, Polygon Mumbai, BSC Testnet
- [ ] Multi-Sig für Bridge-Operationen (3-of-5)
- [ ] Frontend: Bridge-UI (Sidebar: `🌉 Bridge`)
- [ ] Security-Audit vor Mainnet
- [ ] Rate-Limiting: max. 10.000 ATC pro Stunde
- [ ] Emergency-Pause-Mechanismus
- [ ] Oracle: Kurs-Feed ATC/ETH
- [ ] Dokumentation: Bridge-Guide
- [ ] Tests: `test_bridge.py`
- [ ] Monitoring: Bridge-Volume Dashboard
- [ ] Bug-Bounty für Bridge-Schwachstellen
- [ ] Versicherungsfonds: 5% aller Bridge-Fees
- [ ] Cross-Chain Messaging (nicht nur Token)
- [ ] Layer-2 Integration (Arbitrum/Optimism)

---

## ✅ Geschlossene Issues

| # | Titel | Geschlossen |
|---|-------|-------------|
| #21 | build(deps): flask-cors 4.0.1→6.0.0 | 2026-06-08 |
| #6  | 🔐 ECDSA Signatur — TX-Autorisierung | 2026-05-20 |
| #4  | 💾 NFT Persistenz — SQLite | 2026-05-20 |
| #1  | 🔗 Smart Contract — ATC Token Standards | 2026-05-22 |

---

## 📊 Fortschritt-Übersicht

| Kategorie | Issues offen | Sub-Tasks offen | Sub-Tasks erledigt |
|-----------|-------------|-----------------|-------------------|
| 🌐 P2P / Networking | 5 | 51 | 0 |
| ⛓ Blockchain / Contracts | 4 | 57 | 4 |
| 🎮 Gaming / NFT | 3 | 44 | 0 |
| 🖥 Frontend / UI | 3 | 25 | 0 |
| 🛠 DevOps / Build | 2 | 19 | 0 |
| 🤖 AI / KI | 1 | 1 | 6 |
| **Gesamt** | **18** | **197** | **10** |

*Auto-generiert von Superagent — 2026-06-09*

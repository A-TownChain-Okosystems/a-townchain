# Sprint 4.0 — Mainnet Launch

> **Issues:** #70, #71 | **Priorität:** MEDIUM | **Meilenstein:** MK9
> **Status:** ⬜ GEPLANT | **Ziel:** Q4 2026

## Overview

Öffentlicher Mainnet-Start von A-TownChain OS (Chain ID 9001). Umfasst Genesis Block Signierung, Bootstrap Node Deployment, Validator Onboarding und öffentlichen Launch.

## Todos

### Phase 1: Genesis Block Konfiguration (#71)
- [ ] Genesis Block Struktur finalisieren (Chain ID 9001)
- [ ] Genesis-Wallet `ATCf9327118a7dfb30f72ba6aa82e1186078c42232884` verifizieren
- [ ] 5 Validator Public Keys in Genesis Block eintragen
- [ ] Token Distribution Schedule finalisieren (15/10/50/10/15%)
- [ ] Genesis Block Signierung (offline, air-gapped machine)
- [ ] Genesis Block Hash verifizieren und dokumentieren
- [ ] `config/mainnet_genesis.json` finaler Review
- [ ] Genesis Block im Wiki dokumentieren (Kap. 55)

### Phase 2: Bootstrap Node Setup (extern)
- [ ] VPS erwerben (Hetzner CX21+: 4GB RAM, 2 vCPU, 40GB SSD)
- [ ] Domain registrieren (z.B. `bootstrap.a-townchain-os.io` oder `.io`)
- [ ] DNS A-Record auf VPS public IP konfigurieren
- [ ] Bootstrap Node Software deployen (`scripts/bootstrap_node.py`)
- [ ] Port 9000 öffnen (Firewall-Regel, ufw/iptables)
- [ ] SSL-Zertifikat (Let's Encrypt + auto-renew)
- [ ] Systemd Service für Bootstrap Node (auto-restart)
- [ ] Bootstrap Node Health Check (Uptime Monitoring, 24/7)
- [ ] DNS Seed List hardcodieren (ähnlich Bitcoin seed nodes)
- [ ] Backup Strategy (Daily snapshot, 7-day retention)

### Phase 3: Validator Setup (#70)
- [ ] 10+ Validator Nodes akquirieren (Community + Partner)
- [ ] Validator Onboarding Guide erstellen (Setup → Bond → Activate)
- [ ] Validator Bond Distribution (10.000 ATC pro Validator)
- [ ] Validator Configuration Template (`config/validator.yaml`)
- [ ] Validator Monitoring Dashboard (Grafana)
- [ ] Slashing Conditions testen (DevNet Simulation)
- [ ] Validator Key Management Guide (HSM / Ledger / Cold Storage)
- [ ] Validator Performance Benchmarking

### Phase 4: Mainnet Launch
- [ ] **Go/No-Go Decision** (MK7 AI Protocol + MK8 Security mindestens teilweise erfüllt)
- [ ] Genesis Block Broadcast (Bootstrap Node → Network)
- [ ] Block #0 verifizieren (alle Validatoren bestätigen)
- [ ] First Transaction Test (Genesis → Validator Bond)
- [ ] Network Stability Check (24h Observation Window)
- [ ] Public Announcement (Twitter, Discord, Blog Post, Press Release)
- [ ] Block Explorer Live-Schaltung (a-townchain-os.io/explorer)
- [ ] Mainnet Status Dashboard (uptime, block height, TX count)
- [ ] Community Onboarding (Validator recruitment, Developer docs)

## Voraussetzungen
- 🔵 MK7 (AI Agent Protocol) — mindestens Spezifikation abgeschlossen
- 🔵 MK8 (Security Audit) — mindestens interne Phase abgeschlossen
- ✅ MK1-MK6 (alle Core-Meilensteine erfüllt)

## Deliverables
1. Signierter Genesis Block (Chain ID 9001)
2. Bootstrap Node (24/7, public IP, DNS seed)
3. 10+ aktive Validator Nodes
4. Block Explorer (public)
5. Mainnet Status Dashboard
6. Public Launch Announcement

## Blocker
- 🔴 **VPS + Domain** für Bootstrap-Node (Michael — Hetzner CX21 empfohlen)
- 🔴 **Validator Bond Distribution** (10.000 ATC × 10+ Validators)
- 🔴 **SSL-Zertifikate** (Let's Encrypt, kostenlos aber Setup erforderlich)

## Querverweise
- [Issue #70](../issues/ISSUE_70_SPRINT_4.0_VALIDATOR-NODES_.md)
- [Issue #71](../issues/ISSUE_71_SPRINT_4.0_GENESIS_BLOCK__K.md)
- [Roadmap MK9](../ROADMAP.md#mk9--mainnet-launch--geplant)
- [Sprint Roadmap](../../SPRINT_ROADMAP.md#-sprint-40--mainnet-launch-geplant)
- [Genesis Config](../../config/mainnet_genesis.json)

---

*Aurora Agent · 05.07.2026*

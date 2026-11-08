# MASTER TODO — A-TownChain OS
> Aktualisiert: 2026-06-12 06:55 UTC

## 🔴 Priorität MEDIUM (sofort angehen)

### #51 — IPC Bus: Vollständige Kernel-Integration
- [ ] `modules/kernel/ipc/ipc_bus.py` vollständig implementieren
- [ ] Standard-Topics registrieren: block.new, tx.broadcast, peer.connect, ai.query, fs.write
- [ ] Integration mit ai_kernel.py, atcfs.py, consensus
- [ ] Unit-Tests für alle Topics
- **Sprint:** 2.4 | **Wiki:** Kap. 58

### #50 — Hugging Face Code-Review Pipeline
- [ ] Pipeline-Script erstellen (hf_review_pipeline.py)
- [ ] GitHub Actions Workflow einrichten
- [ ] Automatisches Code-Review bei PRs
- [ ] Report-Format definieren
- **Sprint:** 2.x | **Wiki:** Kap. 61

### #49 — BigQuery Analytics Pipeline
- [ ] GCP-Projekt erstellen (console.cloud.google.com)
- [ ] BigQuery Dataset: atcchain_metrics anlegen
- [ ] Tabellen: commits, issues, traffic, sync_log
- [ ] Export-Pipeline aus Google Sheets
- **Sprint:** 4.x | **Wiki:** Kap. 60

## 🟡 Priorität LOW (nächste Sprints)

### #48 — ATCLang v0.4.0 Type System
- [ ] Type-Checker implementieren
- [ ] Structs im Compiler ergänzen
- [ ] ATC:: Standardbibliothek (Wallet, Math, Chain, String)
- **Sprint:** 3.x | **Wiki:** Kap. 36

### #47 — ZKP Zero-Knowledge Proofs
- [ ] ZKP-Bibliothek evaluieren (zk-SNARKs / Bulletproofs)
- [ ] L0 Security Layer definieren
- [ ] Proof-Generierung in Consensus integrieren
- **Sprint:** 3.x | **Wiki:** Kap. 25

### #46 — Mobile Wallet Biometrie
- [ ] Biometric Auth (Face ID / Fingerprint) einbauen
- [ ] FCM Push-Notifications
- [ ] iOS + Android Tests
- **Sprint:** 3.x | **Wiki:** Kap. 38

### #45 — ATCoin DeFi AMM+
- [ ] LP-Farming / Yield-Aggregator
- [ ] Flash Loans
- [ ] Impermanent-Loss-Schutz
- **Sprint:** 3.x | **Wiki:** Kap. 26

### #44 — Mainnet Monitoring Grafana
- [ ] Grafana einrichten
- [ ] Prometheus-Metriken exportieren
- [ ] Alerts definieren (Block-Time, TPS, Validator-Health)
- **Sprint:** 4.x | **Wiki:** Kap. 49

### #52 — Mainnet Launch Manager
- [ ] MainnetLaunchManager vollständig implementieren
- [ ] Genesis-Konfiguration (Chain-ID 9000) finalisieren
- [ ] 5 Genesis-Validators koordinieren
- [ ] Mainnet-Checkliste abarbeiten
- **Sprint:** 4.2 | **Wiki:** Kap. 55

## ✅ Abgeschlossen (41 Issues)
Kritischer Pfad: #14 #15 #16 #17 #18 #8 ✅
v3.0 Release: #34 Solana Bridge, #35 ATCLang v0.3.0, #36 Mainnet Config, #37 DEX/AMM, #38 Mobile Wallet, #39 DAO Governance ✅

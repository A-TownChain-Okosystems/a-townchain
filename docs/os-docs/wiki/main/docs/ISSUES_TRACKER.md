# Issue-Tracker — Vollständige Übersicht

## Tags & Labels

| Tag | Farbe | Beschreibung |
|-----|-------|-------------|
| `critical` | 🔴 | Mainnet-Blocker |
| `high` | 🟠 | Wichtig für v3.0 |
| `medium` | 🟡 | Soll in v3.0 |
| `low` | 🟢 | Nice-to-have |
| `security` | 🔒 | Sicherheits-relevant |
| `performance` | ⚡ | Performance-Verbesserung |
| `atclang` | 🔵 | ATCLang-Compiler |
| `shivaos` | 🟣 | ShivaOS Kernel |
| `blockchain` | ⛓️ | Blockchain Core |
| `gaming` | 🎮 | Shivamon/Gaming |
| `ai` | 🤖 | KI/Gemini |
| `bridge` | 🌉 | Cross-Chain |
| `docs` | 📄 | Dokumentation |
| `bug` | 🐛 | Fehler |
| `enhancement` | ✨ | Verbesserung |
| `feature` | 🚀 | Neues Feature |
| `in-progress` | 🔄 | In Bearbeitung |
| `done` | ✅ | Abgeschlossen |

## v2.1.0 — Alle Issues ✅

| # | Titel | Tags | Status |
|---|-------|------|--------|
| 1 | Smart Contracts ATC-8300/9000/9900 | `blockchain` `feature` | ✅ |
| 2 | Gemini AI BYOK | `ai` `feature` | ✅ |
| 3 | Battle UI | `gaming` `feature` | ✅ |
| 4 | Persistenz | `blockchain` `feature` | ✅ |
| 5 | Block Explorer | `blockchain` `feature` | ✅ |
| 6 | ECDSA Wallet | `security` `blockchain` | ✅ |
| 9 | Governance DAO | `blockchain` `feature` | ✅ |
| 12 | Solidity Hardhat | `blockchain` `feature` | ✅ |
| 14 | Bootstrap Node | `shivaos` `feature` | ✅ |

## v2.2.0 — Alle Issues ✅

| # | Titel | Tags | Commits |
|---|-------|------|---------|
| 7 | Build System | `feature` | `Fix #7` |
| 8 | Testnet | `shivaos` `feature` | `Fix #8+#18` |
| 10 | Bridge ETH+POL+BSC | `bridge` `critical` | `Fix #10` |
| 11 | Breeding Engine | `gaming` `feature` | `Fix #11` |
| 13 | Marketplace v2 | `gaming` `feature` | `Fix #13` |
| 18 | Testnet-Launcher | `shivaos` | `Fix #18` |
| 19 | Node-Monitoring | `performance` | `Fix #19` |
| 23 | ATCFS Syscalls | `shivaos` `feature` | `Fix #23` |
| 24 | MultiSig Wallet | `security` `blockchain` | `Fix #24` |
| 25 | Gateway v1.0.0 | `feature` `performance` | `Fix #25` |
| 26 | Integration Tests | `feature` | `Fix #26` |
| 27 | atcpkg Manager | `atclang` `feature` | `Fix #27` |
| 28 | ShivaOS UI TUI | `shivaos` `feature` | `Fix #28` |
| 29 | Federated Learning | `ai` `feature` | `Fix #29` |
| 30 | atcpkg Registry | `atclang` `feature` | `Fix #30` |
| 32 | Syscall-Tabelle | `shivaos` `feature` | `Fix #32` |
| 33 | Gas-Fee EIP-1559 | `blockchain` `feature` | `Fix #33` |

## v3.0.0 — Offen 🔄

| # | Titel | Tags | Prio | Sub-Features |
|---|-------|------|------|-------------|
| 15 | Solana Bridge | `bridge` `critical` | 🔴 | SPL-Token, Wormhole, Relayer |
| 16 | ATCLang v0.3.0 | `atclang` `critical` | 🔴 | async/await, Generics, Closures, Module |
| 17 | Mainnet Launch | `blockchain` `critical` | 🔴 | Genesis, Validator-Set, Chain-ID 9000 |
| 20 | DEX / AMM | `blockchain` `feature` | 🟡 | Swap-Router, Liquidity Pools, LP-Token |
| 21 | Mobile Wallet | `feature` `medium` | 🟡 | iOS, Android, React Native, QR-Code |
| 22 | DAO Live | `blockchain` `medium` | 🟡 | FFT Voting, Quorum, Execute on-chain |
| 31 | ZK-Proofs | `security` `low` | 🟢 | zk-SNARK, Privacy TX, Verifier |
| 34 | Layer-2 Rollup | `blockchain` `low` | 🟢 | Shiva-L2, Batch TX, Fraud Proofs |
| 35 | Marketplace v2 | `gaming` `low` | 🟢 | Cross-Chain NFT, Auktion v2 |

## Sub-Features Details

### #15 Solana Bridge
- [ ] SPL-Token Standard Mapping
- [ ] Wormhole-Protokoll Integration
- [ ] Solana Relayer Node
- [ ] Cross-Chain TX-Nachweis
- [ ] Bridge UI (Einzahlen/Auszahlen)

### #16 ATCLang v0.3.0
- [ ] `async fn` / `await` Keywords
- [ ] Generics: `fn foo<T>(x: T) -> T`
- [ ] Closures: `let f = |x| x * 2`
- [ ] Module-System: `import ATC::Gaming::Shivamon`
- [ ] Besserer Error-Reporter (Zeile+Spalte+Context)
- [ ] ATCLang → WASM Kompilierung (experimentell)
- [ ] Type Inference: `let x = 42` (kein expliziter Typ nötig)

### #17 Mainnet Launch
- [ ] Genesis Block (finaler Hash)
- [ ] Initialer Validator-Set (≥ 10 Nodes)
- [ ] ATC Token Distribution (Whitepaper-Tokenomics)
- [ ] Audit (externes Security-Audit)
- [ ] Chain-ID 9000 finalisieren
- [ ] Mainnet-Dokumentation + Launch-Kommunikation

### #20 DEX / AMM
- [ ] Constant-Product AMM: `x * y = k`
- [ ] Swap-Router (Multi-Hop: ATC→FFT→andere)
- [ ] Liquidity Pool Token (ATC-8300)
- [ ] Fee-Distribution (0.3% Swap-Fee an LPs)
- [ ] Impermanent-Loss-Warnung im UI

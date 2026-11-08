# Verbesserungen & Enhancement-Backlog

## Kurzfristig (v3.0.0-Zyklus)

### ATCLang
- [ ] Error-Reporter: Zeile + Spalte + Code-Kontext ausgeben
  ```
  Zeile 12, Spalte 5: UndeclaredVariable 'amoutn'
    fn transfer(amoutn: u128) {
        ^^^^^
  Meintest du: 'amount'?
  ```
- [ ] Type Inference: `let x = 42` ohne expliziten Typ
- [ ] String-Interpolation: `f"Balance: {self.balance} ATC"`
- [ ] Import-Aliases: `use ATC::Crypto::sha256 as hash`
- [ ] Test-Framework: `atctest` für Contract-Unit-Tests

### Gateway
- [ ] JWT-Auth statt statischer API-Keys
- [ ] TLS/HTTPS via nginx Reverse Proxy
- [ ] GraphQL-Endpoint (neben REST)
- [ ] WebSocket-Subscriptions für Events

### Smart Contracts
- [ ] Upgrade-Proxy Pattern (UUPS)
- [ ] Time-Lock für Owner-Transfers
- [ ] Snapshot-Funktion für Governance (Token-Balance zu Voting-Start)
- [ ] Contract-Templates: 1-Klick Token/NFT Deploy

### Shivamon
- [ ] Battle-Replay-System (TX-basiert, verifizierbar)
- [ ] NFT-Metadata auf IPFS spiegeln
- [ ] Shivamon-Leihsystem (Lending für Battles)
- [ ] Saison-Pass (ATC-8300, zeitlich begrenzt)

## Mittelfristig (v4.0.0)

### Performance
- [ ] ATCLang → WASM (×100 VM-Speedup)
- [ ] MessagePack statt JSON für P2P
- [ ] Lazy-Loading Blockchain-History
- [ ] Block-Pruning (Archive vs. Light Node)

### UX / DX
- [ ] `atc` CLI-Tool: `atc deploy contract.atc`
- [ ] VS Code Extension: ATCLang Syntax-Highlighting
- [ ] Playground: Browser-basierter ATCLang Editor + VM
- [ ] SDK: Python + JavaScript + Rust

### Skalierung
- [ ] Layer-2 Rollup (Shiva-L2): Batch-TX, Fraud Proofs
- [ ] State Channels für schnelle Micro-Payments
- [ ] Sharding (Zulässig nach Mainnet-Launch)

## Code-Qualität

- [ ] 100% Test-Coverage für Kern-Module (aktuell ~70%)
- [ ] Property-Based Testing (Hypothesis) für Consensus
- [ ] Formale Verifikation (TLA+ für Consensus-Logik)
- [ ] Automatische Dependency-Updates (Dependabot)
- [ ] CI/CD Pipeline (GitHub Actions → Auto-Deploy Testnet)

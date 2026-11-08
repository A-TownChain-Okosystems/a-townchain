# Commit-Geschichte & Versionierung

## Commit-Konventionen

```
<typ>(<scope>): <kurze Beschreibung>

Typen:
  feat     Neues Feature
  fix      Bugfix
  sec      Security-Fix
  docs     Dokumentation
  perf     Performance
  test     Tests
  refactor Code-Refactoring
  chore    Build/Config
  ci       CI/CD

Scopes:
  atclang  ATCLang Compiler/VM
  shivaos  Kernel/ATCFS
  blockchain  Core Chain
  contracts   Smart Contracts
  gateway     API Gateway
  gaming      Shivamon
  bridge      Cross-Chain Bridge
  ai          Gemini/Federated Learning
  pkg         atcpkg
  docs        Dokumentation
  test        Tests
```

## Wichtige Commits (10.06.2026)

```
e03c4374  feat(pkg):    Fix #30 atcpkg Registry API
4fb1c12a  feat(ai):     Fix #29 Federated Learning Coordinator FedAvg
570b7473  feat(shivaos):Fix #28 ShivaOS UI Renderer TUI
8678f803  feat(pkg):    Fix #27 atcpkg Package Manager
26e694c8  feat(build):  Fix #7  Build System Docker/AppImage/EXE/.deb
4f3fcab5  feat(gaming): Fix #13 ATC Marketplace Festpreis+Auktion
d0b9ba95  feat(bridge): Fix #10 Cross-Chain Bridge ETH+POL+BSC
37cad2a2  feat(gaming): Fix #11 Shivamon Breeding Engine
636aa98c  feat(shivaos):Fix #32 ShivaOS Syscall-Tabelle 20 Syscalls
f0b9ce29  feat(blockchain):Fix #33 Gas-Fee Engine EIP-1559
64515fb8  test:         Fix #26 Integration Tests 9/9 bestanden
6e2d7c86  feat(gateway):Fix #25 Gateway v1.0.0 alle Middlewares
b2604804  feat(security):Fix #24 MultiSig Wallet M-of-N
109ce465  feat(shivaos):Fix #23 ATCFS Syscall-Interface
445e4386  feat(shivaos):Fix #19 Node-Monitoring Prometheus
f4d5f449  feat(network):Fix #8+#18 Testnet Launcher + Docker Compose
793f61b3  docs:         AGENT_MANIFEST.md — KI-Datenkarte
37dc2dfe  chore:        MONOREPO v3.0.0 Restrukturierung
```

## Semantic Versioning

```
v2.1.0 → v2.2.0: MINOR (neue Features, keine Breaking Changes)
v2.2.0 → v3.0.0: MAJOR (Breaking: Monorepo-Struktur, neue API-Endpoints)

Geplant:
v3.0.0 → v3.1.0: ATCLang v0.3.0 + Solana Bridge
v3.1.0 → v4.0.0: Mainnet Launch (MAJOR)
```

## Git-Tags (empfohlen)

```bash
git tag -a v2.1.0 -m "v2.1.0 Feature-Complete"
git tag -a v2.2.0 -m "v2.2.0 Feature-Complete (Monorepo v3.0.0)"
git tag -a v3.0.0-alpha -m "v3.0.0-alpha — Solana Bridge WIP"
```

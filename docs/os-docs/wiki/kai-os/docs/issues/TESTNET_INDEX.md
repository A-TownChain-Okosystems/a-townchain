# 🌐 Testnet Issues — Index

> Sub-Issues von [Issue #8 — Multi-Node Testnet](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/8)
> **Milestone:** v2.2.0

| Issue | Titel | Prio | Doku |
|-------|-------|------|------|
| [#14](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/14) | 🌐 Bootstrap Node — P2P Discovery | 🔴 High | [Testnet.md#3](../architecture/TESTNET.md#3-node-discovery-issue-14) |
| [#15](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/15) | 📡 Block Propagation | 🔴 High | [Testnet.md#4](../architecture/TESTNET.md#4-block-propagation-issue-15) |
| [#16](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/16) | 🔄 Initial Sync | 🔴 High | [Testnet.md#5](../architecture/TESTNET.md#5-initial-sync-issue-16) |
| [#17](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/17) | ⛓ Longest-Chain-Rule | 🔴 High | [Testnet.md#6](../architecture/TESTNET.md#6-longest-chain-rule-issue-17) |
| [#18](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/18) | 🐳 Docker Compose Testnet | 🟡 Medium | [Testnet.md#7](../architecture/TESTNET.md#7-docker-compose-issue-18) |
| [#19](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/19) | 📊 Node-Monitoring Dashboard | 🟡 Medium | [Testnet.md#8](../architecture/TESTNET.md#8-node-monitoring-dashboard-issue-19) |

## Implementierungsreihenfolge
```
#14 Bootstrap Node
  └─→ #15 Block Propagation
        └─→ #16 Initial Sync
              └─→ #17 Longest-Chain-Rule
                    └─→ #18 Docker Compose
                          └─→ #19 Monitoring Dashboard
```

→ [Vollständige Testnet-Dokumentation](../architecture/TESTNET.md)

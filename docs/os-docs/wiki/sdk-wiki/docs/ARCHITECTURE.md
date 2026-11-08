# 🏗️ SDK Architektur — atc-sdk

---

## 1. Cross-Language Model

Das `atc-sdk` stellt einheitliche Schnittstellen über vier Programmiersprachen bereit:

- **TypeScript (`@atc/sdk`)**: Browser, React, Node.js.
- **Python (`atc-sdk`)**: AI Engine, Data Pipelines.
- **Rust (`atc-sdk`)**: Native Crate für Kernel Modules & High-Speed Trading.
- **Go (`atc-sdk-go`)**: Microservices & Backend Integrationen.

```
[ Application Code ] ──> [ SDK Client Wrapper ] ──> [ Ed25519 Signer ] ──> [ Gateway RPC ]
```

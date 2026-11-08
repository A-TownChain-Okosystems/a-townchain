# Kernel & Shell — Architektur

> **Stand:** 05.07.2026 | **Sprint:** 2.4 | **Standard:** ATC-96

## Übersicht

Der KAI-OS Kernel bietet Prozessverwaltung, IPC, Dateisystem (ATCFS), Shell und Package Management — alles in ATCLang implementiert.

## Module (11 .atc Dateien)

| Modul | Datei | Zeilen | Beschreibung |
|-------|-------|--------|--------------|
| Kernel | modules/kernel/kernel/kernel.atc | 147 | Micro-Kern, Scheduler, Memory |
| Shell | modules/kernel/shell/shell.atc | 295 | Interactive Command Processor |
| Package Manager | modules/kernel/pkg/manager.atc | 207 | On-chain Package Registry |
| IPC Bus | modules/kernel/ipc/ipc_bus.atc | 101 | Inter-Process Communication |
| Process Manager | modules/kernel/process/process_mgr.atc | 160 | Process Lifecycle, Spawn/Kill |
| ATCFS | modules/kernel/fs/atcfs.atc | 141 | A-TownChain File System |
| ATCNet | modules/kernel/net/atcnet.atc | 134 | Network Stack |
| ECDSA | modules/contracts/wallet/ecdsa.atc | 59 | secp256k1 Sign/Verify |
| Keygen | modules/contracts/wallet/keygen.atc | 74 | BIP39 Key Generation |
| Wordlist | blockchain/wallet/wordlist.atc | 111 | BIP39 Wordlist (256 words) |
| ECDSA Impl | tools/ecdsa_impl.atc | 118 | ECDSA Implementation Details |

## Shell Features

- 13 Built-in Commands (help, status, ls, cd, echo, env, export, alias, history, whoami, clear, exit, pwd)
- Command History (256 entries max)
- Environment Variables
- Aliases
- Background Process Support (&)

## Package Manager Features

- On-chain Package Registry
- publish, install, uninstall, deprecate, ban, verify
- Dependency Resolution
- Version Tracking
- Download Counter

## AD-002: EventBus vs IPCBus

- **Status:** VALIDATE — Michael muss entscheiden
- **Option A:** EventBus (Publish/Subscribe) — lose Kopplung
- **Option B:** IPCBus (direkte Messages) — straffe Kontrolle
- **Aktuell:** IPCBus implementiert (ipc_bus.atc)

---

*Kernel & Shell Architecture · Sprint 2.4 · 05.07.2026 · Aurora*

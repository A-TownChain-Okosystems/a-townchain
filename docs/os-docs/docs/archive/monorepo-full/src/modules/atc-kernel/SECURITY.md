# Security — ShivaOS Kernel

## Fixes v2.1.0
- ✅ VM Call-Depth-Limit = 128
- ✅ Reentrancy-Guard in BaseContract
- ✅ P2P Rate-Limiting (100 Msgs/60s, 64KB)
- ✅ ATCNet: Silent Exceptions geloggt

## ATS-1002 Security-Standard
- Alle Kernel-Calls authentifiziert
- Memory-Isolation pro Prozess
- IPC-Kanäle typisiert und validiert

Details: [Security Audit](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/SECURITY.md)

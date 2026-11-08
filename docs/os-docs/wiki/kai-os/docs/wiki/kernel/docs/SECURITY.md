# ShivaOS Security v2.1.0

## Fixes (09.06.2026)

### ✅ ATCNet Rate-Limiting
- Max 100 Nachrichten/60s pro Peer
- 64 KB Message-Size-Limit
- Silent Exceptions → geloggt

### ✅ P2P Signatur-Validierung
- ECDSA-Signatur auf allen Block/TX-Messages
- Replay-Schutz via Nonce + Chain-ID 9000

### ✅ Kernel Reentrancy-Schutz
- Alle Contract-Aufrufe durch `_nonreentrant_enter/exit` geschützt

## ATS-1002 Security Standard
- Alle Kernel-Calls authentifiziert
- Memory Isolation per Prozess
- IPC-Kanäle typisiert und validiert

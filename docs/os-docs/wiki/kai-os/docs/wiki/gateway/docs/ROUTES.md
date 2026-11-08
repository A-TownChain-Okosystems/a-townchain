# Gateway Routen-Referenz

## Blockchain
| Method | Path | Beschreibung |
|--------|------|-------------|
| GET | /api/blockchain/status | Chain-Status |
| GET | /api/blockchain/blocks | Block-Liste |
| POST | /api/blockchain/tx | TX einreichen |

## Wallet
| Method | Path | Beschreibung |
|--------|------|-------------|
| POST | /api/wallet/generate | Neue Wallet |
| GET | /api/wallet/balance/:addr | Balance |
| POST | /api/wallet/send | ATC senden (Auth + Signatur) |

## AI / Gemini
| Method | Path | Beschreibung |
|--------|------|-------------|
| GET | /api/ai/status | Orchestrator-Status |
| POST | /api/ai/chat | Chat mit Gemini |
| POST | /api/ai/generate-atclang | ATCLang generieren |
| POST | /api/ai/explain-contract | Contract erklären |

## NFT / Shivamon
| Method | Path | Beschreibung |
|--------|------|-------------|
| POST | /api/game/mint | Shivamon minten |
| GET | /api/game/tokens/:addr | Tokens nach Adresse |

## Auth
Header: `X-API-Key: <key>` (min. 32 Zeichen)

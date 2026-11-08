# 🔌 API Spezifikation — atc-explorer

## Endpunkte

- `GET /api/blocks?page=1&limit=20` - Block-Liste abrufen
- `GET /api/block/:hash_or_height` - Block-Details
- `GET /api/tx/:hash` - Transaktions-Details
- `GET /api/address/:address` - Kontostand und Historie
- `GET /api/stats` - Netzwerkstatistiken (TPS, Blockzeit, Gas)

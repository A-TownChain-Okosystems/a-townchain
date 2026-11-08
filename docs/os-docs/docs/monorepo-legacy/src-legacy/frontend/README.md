# 🖥 A-TownChain OS — Unified Frontend

Konsolidierung K4 — 3 Frontend-Repos → 1 Monorepo-Verzeichnis.

## Struktur
```
frontend/
├── index.html           # Haupt-Dashboard (ShivaOS v2.0) ← atc-frontend
├── admin/
│   ├── index.html       # Admin Code Center ← atc-ui
│   ├── api.js           # Admin API Client
│   ├── DESIGN.md        # Design-System Docs
│   └── CHANGELOG.md     # Changelog
├── battle/
│   └── index.html       # Shivamon Battle UI
├── bootscreen/
│   └── README.md        # Boot-Screen Spec
├── mobile/
│   ├── wallet_api.atc   # Wallet API (ATCLang) ← atc-mobile
│   ├── wallet/
│   │   └── biometric_auth.atc  # Biometrische Auth (ATCLang)
│   └── README.md
├── assets/
│   ├── css/
│   │   └── variables.css
│   └── js/
│       └── api.js
├── src/                 # TypeScript Source (future)
├── package.json         # npm package definition
└── tsconfig.json        # TypeScript config
```

## Zusammengeführte Repos
| Repo | → Ziel | Dateien |
|------|--------|---------|
| atc-frontend | frontend/ | index.html, battle/, bootscreen/, assets/ |
| atc-ui | frontend/admin/ | index.html (2306 lines), api.js, DESIGN.md |
| atc-mobile | frontend/mobile/ | wallet_api.atc, biometric_auth.atc |

## API Connection
- REST: `http://localhost:5000/api/...`
- WebSocket (geplant): `ws://localhost:5001`

## Run
```bash
cd frontend && python -m http.server 3000
# oder
npx http-server . -p 3000
```

## Nächste Schritte
- K5: Build-System & Docker
- K6: CI/CD Pipeline
- K7: Tests & QA (≥80% Coverage)
- K8: Release v1.0

*Konsolidiert von Aurora #2 am 04.08.2026*

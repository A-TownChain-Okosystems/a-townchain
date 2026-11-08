# 🖥 A-TownChain OS — Frontend

Futuristisches Neon-Dashboard für das A-TownChain Ökosystem.

## Structure
```
frontend/
├── index.html        # Haupt-Dashboard (ShivaOS v2.0)
├── assets/
│   ├── css/          # Stylesheets
│   └── js/           # JavaScript Module
└── pages/            # Sub-Pages (Code Center, Factory...)
```

## API Connection
Das Frontend verbindet sich mit dem Backend via:
- REST: `http://localhost:5000/api/...`
- WebSocket (geplant): `ws://localhost:5001`

## Run
Einfach `index.html` im Browser öffnen oder:
```bash
python -m http.server 3000
```

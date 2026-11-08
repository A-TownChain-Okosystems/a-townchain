# ATC Dashboard Deployment

## Entwicklung (lokal)
```bash
# Backend starten
python start.py

# Frontend öffnen
open atc-ui/index.html
# oder:
python -m http.server 3000 --directory atc-ui
```

## Produktions-Setup
```nginx
server {
    listen 80;
    server_name dashboard.atownchain.io;

    location / {
        root /var/www/atc-ui;
        index index.html;
        try_files $uri $uri/ /index.html;
    }

    location /api/ {
        proxy_pass http://localhost:4000;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## Konfiguration (config.js)
```javascript
window.ATC_CONFIG = {
    apiBase:    'https://api.atownchain.io',
    wsBase:     'wss://ws.atownchain.io',
    chainId:    9000,
    version:    '2.1.0',
    theme:      'neon-dark'
};
```

## Environment-Variablen
| Variable | Beschreibung |
|----------|-------------|
| `ATC_API_URL` | API Gateway URL |
| `ATC_WS_URL` | WebSocket URL |
| `ATC_CHAIN_ID` | Chain-ID (Standard: 9000) |

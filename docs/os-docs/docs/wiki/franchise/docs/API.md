# 🔌 API Referenz — Franchise Factory

## Endpunkte (via API Gateway Port 4000)

### Lizenzen
```
GET  /api/franchise/licenses           Alle Lizenzen
GET  /api/franchise/license/:id        Lizenz by ID
POST /api/franchise/register           Neue Lizenz registrieren
POST /api/franchise/transfer           Lizenz übertragen
GET  /api/franchise/licenses/:addr     Lizenzen nach Adresse
```

### Revenue
```
POST /api/franchise/revenue/record     Einnahme erfassen
POST /api/franchise/revenue/payout     Auszahlung auslösen
GET  /api/franchise/revenue/stats/:id  Statistiken
```

### Token
```
GET  /api/franchise/token/balance/:addr  FFT-Balance
POST /api/franchise/token/transfer       FFT übertragen
GET  /api/franchise/token/supply         Total Supply
```

## Authentifizierung
Header: `X-API-Key: <your-key>`

## Beispiel
```bash
curl -X POST http://localhost:4000/api/franchise/register \
  -H 'X-API-Key: your-key' \
  -H 'Content-Type: application/json' \
  -d '{"brand_name":"MyBrand","region":"DACH","revenue_share":10,"duration_days":365}'
```

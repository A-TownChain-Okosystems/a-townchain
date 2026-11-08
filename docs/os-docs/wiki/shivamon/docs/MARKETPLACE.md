# Marketplace API

## Endpunkte
```
POST /api/marketplace/list    Shivamon zum Verkauf stellen
POST /api/marketplace/buy     Shivamon kaufen
GET  /api/marketplace/listings  Aktive Listings
GET  /api/marketplace/history   Kaufhistorie
```

## Listing
```json
{
  "token_id": 42,
  "price": "1000000000000000000000",
  "seller": "ATC...",
  "signature": "..."
}
```

## Fee: 2.5% des Kaufpreises (an Treasury)

# 📄 Issue #13 — ATC Marketplace (Shivamon kaufen & verkaufen)

> **Labels:** enhancement · game · marketplace · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #13](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/13)

---

## Ziel

Dezentraler NFT-Marktplatz im ShivaOS Dashboard — Shivamon listen, kaufen, verkaufen und Angebote machen. Alle Trades werden in ATC abgewickelt.

---

## Marketplace-Mechanismus

```
Seller listet Shivamon für 500 ATC
  └─→ NFT wird im Contract gesperrt (escrow)
  └─→ Listing erscheint im Marketplace

Buyer klickt "Kaufen"
  └─→ 500 ATC Transfer: Buyer → Seller (abzgl. 2.5% Fee → Treasury)
  └─→ NFT Transfer: Escrow → Buyer
  └─→ Listing wird entfernt

Seller bricht ab
  └─→ NFT aus Escrow zurück an Seller
  └─→ Listing wird entfernt
```

---

## Contract-Methoden

```python
class MarketplaceContract:

    def list_for_sale(self, token_id: str, seller: str,
                      price_atc: float) -> dict: ...
    # Sperrt NFT in Escrow, erstellt Listing

    def buy(self, token_id: str, buyer: str) -> dict: ...
    # Prüft Balance, führt ATC + NFT Transfer durch, zieht 2.5% Fee

    def cancel_listing(self, token_id: str, seller: str) -> dict: ...
    # Gibt NFT aus Escrow zurück

    def make_offer(self, token_id: str, buyer: str,
                   offer_atc: float) -> dict: ...
    # Legt ein Angebot ab (Seller kann annehmen)

    def get_listings(self, element=None, rarity=None,
                     min_price=None, max_price=None) -> list: ...

    def get_stats(self) -> dict: ...
    # Floor Prices, Volumen, Top Sales
```

### Listing-Datenmodell

```python
@dataclass
class Listing:
    listing_id:  str        # "LST-" + SHA-256[:12]
    token_id:    str        # SHV-...
    seller:      str        # ATC-Adresse
    price_atc:   float
    shivamon:    dict       # Snapshot des NFT zum Listing-Zeitpunkt
    listed_at:   int
    expires_at:  int        # Optional: 30 Tage
    status:      str        # "active" | "sold" | "cancelled"
```

---

## Frontend-Layout

```
🛒 MARKETPLACE

Filter: [Element ▼] [Rarity ▼] [Preis: 0 — 9999] [Sortierung ▼]

Stats: Floor: 45 ATC · 24h Volumen: 3.420 ATC · Listings: 127

┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│ 🔥       │ │ ⚡       │ │ 🌀       │ │ 💧       │
│Ignarex   │ │Voltrix   │ │Quantrix  │ │Aquarix   │
│Epic      │ │Legendary │ │Rare      │ │Genesis✨ │
│ATK: 180  │ │ATK: 290  │ │ATK: 140  │ │ATK: 450  │
│          │ │          │ │          │ │          │
│ 350 ATC  │ │ 1200 ATC │ │  89 ATC  │ │4500 ATC  │
│[KAUFEN]  │ │[KAUFEN]  │ │[KAUFEN]  │ │[KAUFEN]  │
└──────────┘ └──────────┘ └──────────┘ └──────────┘
```

---

## Aufgaben

- [ ] `blockchain/contracts/marketplace/marketplace_contract.py`
- [ ] Escrow-Mechanismus (NFT im Contract sperren)
- [ ] 2.5% Marketplace-Fee → ATC Treasury
- [ ] `backend/api/routes/marketplace_routes.py`
- [ ] Filter: Element · Rarity · Preis-Range · Level
- [ ] Frontend Marketplace-Seite (Sidebar: `🛒 Market`)
- [ ] Eigene Listings verwalten (Preis ändern, abbrechen)
- [ ] Floor Price + Handelsvolumen Stats
- [ ] Offer-System (Gegenangebote)
- [ ] Tests: `tests/test_marketplace.py`

---

## Akzeptanzkriterien

- [ ] Listing, Kauf und Abbrechen funktionieren korrekt
- [ ] ATC wird korrekt transferiert (inkl. 2.5% Fee)
- [ ] NFT geht an Käufer, ATC an Verkäufer
- [ ] Filter funktionieren
- [ ] Floor Price und Volumen werden korrekt berechnet

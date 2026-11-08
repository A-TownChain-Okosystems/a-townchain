# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
blockchain/contracts/marketplace/marketplace_contract.py
ATC Marketplace — Shivamon NFT kaufen & verkaufen
Issue #13: ATC Marketplace

Features:
  - list_for_sale: NFT zum Kauf anbieten (ATC-Preis)
  - buy: NFT kaufen + ATC transferieren (2.5% Royalty)
  - cancel_listing: Listing entfernen
  - get_listings: Aktive Listings filtern + sortieren
  - Preis-Orakel: ATC/USD Referenz
"""
import time, hashlib
from dataclasses import dataclass, field
from enum import Enum
from typing import Optional
from blockchain.contracts.base.base_contract import BaseContract


class ListingStatus(Enum):
    ACTIVE    = "ACTIVE"
    SOLD      = "SOLD"
    CANCELLED = "CANCELLED"
    EXPIRED   = "EXPIRED"


@dataclass
class Listing:
    listing_id:   str
    token_id:     str           # Shivamon Token-ID
    seller:       str           # ATC-Adresse
    price_atc:    float         # Verkaufspreis in ATC
    rarity:       str           # Common/Rare/Epic/Legendary/Genesis
    element:      str           # Fire/Water/Earth/Air/Shadow/Neon/Quantum
    level:        int
    name:         str
    created_at:   int
    expires_at:   int           # Auto-Expire nach 30 Tagen
    status:       str = "ACTIVE"
    buyer:        str = ""
    sold_at:      int = 0
    sold_price:   float = 0.0

    def to_dict(self):
        return {
            "listing_id": self.listing_id,
            "token_id":   self.token_id,
            "seller":     self.seller,
            "price_atc":  self.price_atc,
            "rarity":     self.rarity,
            "element":    self.element,
            "level":      self.level,
            "name":       self.name,
            "created_at": self.created_at,
            "expires_at": self.expires_at,
            "status":     self.status,
            "buyer":      self.buyer,
            "sold_at":    self.sold_at,
        }


class MarketplaceContract(BaseContract):
    """
    ATC Marketplace — dezentrales NFT-Handelssystem.
    Royalty: 2.5% des Verkaufspreises → Creator
    Platform-Fee: 1% → Treasury
    """

    ROYALTY_PERCENT  = 2.5    # 2.5% → Shivamon Creator
    PLATFORM_PERCENT = 1.0    # 1.0% → Treasury/Owner
    LISTING_TTL      = 30 * 24 * 3600   # 30 Tage Auto-Expire

    def __init__(self, owner: str):
        super().__init__(owner, contract_id="ATC_MARKETPLACE")
        self.listings:      dict[str, Listing] = {}   # listing_id → Listing
        self.token_listing: dict[str, str]     = {}   # token_id → listing_id
        self.sales_history: list[dict]         = []
        self._atc_balances: dict[str, float]   = {}
        self._nft_contract  = None   # ShivamonContract-Referenz

    # ── Integration ────────────────────────────────────
    def set_token_contract(self, nft_contract):
        """Verbindet mit dem ShivamonContract."""
        self._nft_contract = nft_contract

    def set_balance_oracle(self, balances: dict):
        """ATC-Balances vom Token-Contract."""
        self._atc_balances = balances

    # ── Listing erstellen ──────────────────────────────
    def list_for_sale(
        self, seller: str, token_id: str,
        price_atc: float,
        nft_meta: dict = None
    ) -> dict:
        """Listet ein Shivamon-NFT zum Verkauf auf."""
        self.when_not_paused()
        if price_atc <= 0:
            raise ValueError("Preis muss > 0 ATC sein")
        if token_id in self.token_listing:
            existing_lid = self.token_listing[token_id]
            if self.listings[existing_lid].status == "ACTIVE":
                raise ValueError(f"Token {token_id} ist bereits gelistet")

        # NFT-Ownership prüfen (wenn Contract verbunden)
        if self._nft_contract:
            nft = self._nft_contract.tokens.get(token_id)
            if not nft:
                return {"success": False, "error": f"Token {token_id} nicht gefunden"}
            if nft.owner != seller:
                return {"success": False, "error": "Nicht der Eigentümer"}
            meta = nft.to_dict()
        else:
            meta = nft_meta or {}

        now = int(time.time())
        lid = "LIST-" + hashlib.sha256(
            f"{seller}{token_id}{now}".encode()
        ).hexdigest()[:10].upper()

        listing = Listing(
            listing_id = lid,
            token_id   = token_id,
            seller     = seller,
            price_atc  = price_atc,
            rarity     = meta.get("rarity", "Unknown"),
            element    = meta.get("element", "Unknown"),
            level      = meta.get("level", 1),
            name       = meta.get("name", token_id),
            created_at = now,
            expires_at = now + self.LISTING_TTL,
        )

        self.listings[lid]           = listing
        self.token_listing[token_id] = lid
        self._emit("Listed", {
            "listing_id": lid, "token_id": token_id,
            "seller": seller, "price_atc": price_atc
        })
        return {"success": True, "listing": listing.to_dict()}

    # ── Kaufen ─────────────────────────────────────────
    def buy(
        self, buyer: str, listing_id: str,
        atc_balances: dict = None
    ) -> dict:
        """Kauft ein gelistetes NFT. ATC wird transferiert."""
        self.when_not_paused()
        listing = self._get_active_listing(listing_id)

        if listing.seller == buyer:
            raise ValueError("Seller kann nicht selbst kaufen")

        balances = atc_balances or self._atc_balances
        buyer_balance = balances.get(buyer, 0.0)
        if buyer_balance < listing.price_atc:
            raise ValueError(
                f"Unzureichendes Guthaben: {buyer_balance:.4f} < {listing.price_atc:.4f} ATC"
            )

        # Gebühren berechnen
        royalty      = listing.price_atc * (self.ROYALTY_PERCENT / 100)
        platform_fee = listing.price_atc * (self.PLATFORM_PERCENT / 100)
        seller_gets  = listing.price_atc - royalty - platform_fee

        # Balances updaten
        balances[buyer]               = buyer_balance - listing.price_atc
        balances[listing.seller]      = balances.get(listing.seller, 0.0) + seller_gets
        balances[self.owner]          = balances.get(self.owner, 0.0) + platform_fee
        # Royalty → Contract-Owner (= ursprünglicher Shivamon-Creator)
        balances["ROYALTY_TREASURY"]  = balances.get("ROYALTY_TREASURY", 0.0) + royalty

        # NFT transferieren
        if self._nft_contract:
            self._nft_contract.transfer(listing.token_id, listing.seller, buyer)

        # Listing abschließen
        now = int(time.time())
        listing.status     = ListingStatus.SOLD.value
        listing.buyer      = buyer
        listing.sold_at    = now
        listing.sold_price = listing.price_atc

        sale = {
            "listing_id":   listing_id,
            "token_id":     listing.token_id,
            "seller":       listing.seller,
            "buyer":        buyer,
            "price_atc":    listing.price_atc,
            "seller_gets":  seller_gets,
            "royalty":      royalty,
            "platform_fee": platform_fee,
            "sold_at":      now,
        }
        self.sales_history.append(sale)
        self._emit("Sold", sale)
        return {"success": True, "sale": sale}

    # ── Listing entfernen ──────────────────────────────
    def cancel_listing(self, seller: str, listing_id: str) -> dict:
        """Entfernt ein Listing (nur Seller oder Owner)."""
        listing = self.listings.get(listing_id)
        if not listing:
            raise KeyError(f"Listing {listing_id} nicht gefunden")
        if listing.status != ListingStatus.ACTIVE.value:
            raise ValueError(f"Listing ist nicht aktiv: {listing.status}")
        if listing.seller != seller and seller != self.owner:
            raise PermissionError("Nur Seller oder Contract-Owner können canceln")

        listing.status = ListingStatus.CANCELLED.value
        self._emit("ListingCancelled", {
            "listing_id": listing_id, "token_id": listing.token_id,
            "cancelled_by": seller
        })
        return {"success": True, "listing_id": listing_id}

    # ── Queries ────────────────────────────────────────
    def get_listings(
        self,
        rarity: str = None,
        element: str = None,
        min_price: float = None,
        max_price: float = None,
        sort_by: str = "price_asc",  # price_asc|price_desc|newest|rarity
        limit: int = 50
    ) -> list:
        """Gibt aktive Listings zurück — gefiltert + sortiert."""
        now = int(time.time())
        results = []

        for l in self.listings.values():
            if l.status != ListingStatus.ACTIVE.value:
                continue
            if l.expires_at < now:
                l.status = ListingStatus.EXPIRED.value
                continue
            if rarity  and l.rarity != rarity:
                continue
            if element and l.element.split(" ")[-1] != element:
                continue
            if min_price and l.price_atc < min_price:
                continue
            if max_price and l.price_atc > max_price:
                continue
            results.append(l)

        # Sortierung
        RARITY_RANK = {"Common":1,"Uncommon":2,"Rare":3,"Epic":4,"Legendary":5,"Genesis":6}
        if sort_by == "price_asc":
            results.sort(key=lambda x: x.price_atc)
        elif sort_by == "price_desc":
            results.sort(key=lambda x: x.price_atc, reverse=True)
        elif sort_by == "newest":
            results.sort(key=lambda x: x.created_at, reverse=True)
        elif sort_by == "rarity":
            results.sort(key=lambda x: RARITY_RANK.get(x.rarity, 0), reverse=True)

        return [l.to_dict() for l in results[:limit]]

    def get_listing(self, listing_id: str) -> dict:
        l = self.listings.get(listing_id)
        return l.to_dict() if l else {"error": "Nicht gefunden"}

    def get_token_listing(self, token_id: str) -> dict:
        lid = self.token_listing.get(token_id)
        return self.get_listing(lid) if lid else {"error": "Kein aktives Listing"}

    def get_sales_history(self, seller: str = None, buyer: str = None, limit: int = 20) -> list:
        history = self.sales_history
        if seller:
            history = [s for s in history if s["seller"] == seller]
        if buyer:
            history = [s for s in history if s["buyer"] == buyer]
        return sorted(history, key=lambda x: x["sold_at"], reverse=True)[:limit]

    def get_stats(self) -> dict:
        active = sum(1 for l in self.listings.values() if l.status == "ACTIVE")
        sold   = sum(1 for l in self.listings.values() if l.status == "SOLD")
        total_volume = sum(s["price_atc"] for s in self.sales_history)
        return {
            "active_listings": active,
            "total_sold":      sold,
            "total_listings":  len(self.listings),
            "total_volume_atc": total_volume,
            "royalty_percent": self.ROYALTY_PERCENT,
            "platform_fee_percent": self.PLATFORM_PERCENT,
            "standard": "ATC-9000 Marketplace"
        }

    # ── Helper ─────────────────────────────────────────
    def _get_active_listing(self, listing_id: str) -> Listing:
        l = self.listings.get(listing_id)
        if not l:
            raise KeyError(f"Listing {listing_id} nicht gefunden")
        if l.status != ListingStatus.ACTIVE.value:
            raise ValueError(f"Listing nicht aktiv: {l.status}")
        if int(time.time()) > l.expires_at:
            l.status = ListingStatus.EXPIRED.value
            raise ValueError("Listing abgelaufen")
        return l

# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/contracts/shivamon/shivamon_contract.py
# Shivamon NFT Contract — ATC-9000 Standard
#
# Shivamon sind NFT-Battle-Kreaturen im A-TownChain Ökosystem.
# Jedes Shivamon ist einzigartig (NFT) mit:
#   - Element (Feuer/Wasser/Erde/Luft/Shadow/Neon/Quantum)
#   - Stats (HP, Attack, Defense, Speed, Special)
#   - Rarity (Common → Legendary → Genesis)
#   - DNA Hash (einzigartiger genetischer Fingerabdruck)
#   - Level & Erfahrungspunkte

import hashlib, time, os, json
from enum import Enum
from dataclasses import dataclass, asdict

class Element(Enum):
    FIRE     = "🔥 Fire"
    WATER    = "💧 Water"
    EARTH    = "🌍 Earth"
    AIR      = "💨 Air"
    SHADOW   = "🌑 Shadow"
    NEON     = "⚡ Neon"
    QUANTUM  = "🌀 Quantum"

class Rarity(Enum):
    COMMON    = "Common"
    UNCOMMON  = "Uncommon"
    RARE      = "Rare"
    EPIC      = "Epic"
    LEGENDARY = "Legendary"
    GENESIS   = "Genesis"

RARITY_MULTIPLIER = {
    "Common": 1.0, "Uncommon": 1.2, "Rare": 1.5,
    "Epic": 2.0, "Legendary": 3.0, "Genesis": 5.0
}

@dataclass
class ShivamonStats:
    hp:      int
    attack:  int
    defense: int
    speed:   int
    special: int

    def total(self):
        return self.hp + self.attack + self.defense + self.speed + self.special

class ShivamonNFT:
    """Ein einzelnes Shivamon NFT."""

    def __init__(self, token_id: str, name: str, element: Element,
                 rarity: Rarity, owner: str, generation: int = 1):
        self.token_id   = token_id
        self.name       = name
        self.element    = element
        self.rarity     = rarity
        self.owner      = owner
        self.generation = generation
        self.level      = 1
        self.xp         = 0
        self.wins       = 0
        self.losses     = 0
        self.minted_at  = int(time.time())
        self.dna_hash   = self._generate_dna()
        self.stats      = self._generate_stats()
        self.moves      = self._assign_moves()

    def _generate_dna(self) -> str:
        seed = f"{self.token_id}{self.name}{self.element.value}{time.time()}"
        return hashlib.sha256(seed.encode()).hexdigest()

    def _generate_stats(self) -> ShivamonStats:
        # Deterministische Stats aus DNA
        dna_int = int(self.dna_hash, 16)
        mult    = RARITY_MULTIPLIER[self.rarity.value]
        base    = 50 + (self.generation * 5)
        return ShivamonStats(
            hp      = int(base * mult * ((dna_int >> 0  & 0xFF) / 128 + 0.5)),
            attack  = int(base * mult * ((dna_int >> 8  & 0xFF) / 128 + 0.5)),
            defense = int(base * mult * ((dna_int >> 16 & 0xFF) / 128 + 0.5)),
            speed   = int(base * mult * ((dna_int >> 24 & 0xFF) / 128 + 0.5)),
            special = int(base * mult * ((dna_int >> 32 & 0xFF) / 128 + 0.5)),
        )

    def _assign_moves(self) -> list:
        move_pool = {
            "Fire":    ["Flame Burst","Inferno","Ember Strike","Solar Beam"],
            "Water":   ["Aqua Jet","Hydro Pump","Tidal Wave","Ice Shard"],
            "Earth":   ["Rock Smash","Quake","Boulder Crush","Mudslide"],
            "Air":     ["Wind Slash","Tornado","Gust","Sky Dive"],
            "Shadow":  ["Dark Pulse","Void Strike","Soul Drain","Eclipse"],
            "Neon":    ["Volt Crash","Neon Surge","Thunder Arc","Plasma Beam"],
            "Quantum": ["Phase Shift","Quantum Leap","Reality Tear","Entangle"],
        }
        el_name = self.element.value.split(" ")[1]
        moves   = move_pool.get(el_name, ["Tackle","Scratch"])
        return moves[:4]

    def gain_xp(self, amount: int):
        self.xp += amount
        while self.xp >= self.level * 100:
            self.xp    -= self.level * 100
            self.level += 1
            # Level-Up: Stats steigen
            self.stats.hp      += 5
            self.stats.attack  += 3
            self.stats.defense += 3
            self.stats.speed   += 2
            self.stats.special += 4

    def to_dict(self) -> dict:
        return {
            "token_id":   self.token_id,
            "name":       self.name,
            "element":    self.element.value,
            "rarity":     self.rarity.value,
            "owner":      self.owner,
            "generation": self.generation,
            "level":      self.level,
            "xp":         self.xp,
            "wins":       self.wins,
            "losses":     self.losses,
            "dna_hash":   self.dna_hash[:16] + "...",
            "stats":      asdict(self.stats),
            "total_stats":self.stats.total(),
            "moves":      self.moves,
            "minted_at":  self.minted_at,
            "standard":   "ATC-9000"
        }


class ShivamonContract:
    """
    ATC-9000 Smart Contract — Shivamon NFT Registry.
    Verwaltet Mint, Transfer, Battle, Breeding.
    """

    # Bekannte Shivamon-Namen nach Element
    NAMES = {
        "Fire":    ["Ignarex","Pyrodon","Flamecor","Embrix","Volcanix"],
        "Water":   ["Aquarix","Tideon","Glacivex","Hydrox","Torrento"],
        "Earth":   ["Terranox","Stonex","Mudrix","Quarzon","Geovex"],
        "Air":     ["Windrix","Stormax","Gazeron","Cyclonix","Breezex"],
        "Shadow":  ["Shadowx","Voidrix","Darknex","Eclipsion","Umbravex"],
        "Neon":    ["Voltrix","Neonex","Arcvex","Plasmon","Thunderix"],
        "Quantum": ["Quantrix","Phasex","Entanglex","Superion","Wavrix"],
    }
    MAX_SUPPLY = 9900  # ATC-9900 kompatibel

    def __init__(self):
        self.tokens         = {}   # token_id → ShivamonNFT
        self.owner_tokens   = {}   # owner → [token_ids]
        self.total_minted   = 0
        self.battle_log     = []

    # ── Minting ────────────────────────────────────────
    def mint(self, owner: str, element: str = None, rarity: str = None,
             generation: int = 1) -> dict:
        if self.total_minted >= self.MAX_SUPPLY:
            return {"success": False, "error": "Max supply reached (9900)"}

        import random
        # Element wählen
        el_name = element or random.choice([e.value.split(" ")[1] for e in Element])
        el      = next((e for e in Element if el_name in e.value), Element.NEON)

        # Rarity wählen (gewichtet)
        if not rarity:
            rarity = random.choices(
                ["Common","Uncommon","Rare","Epic","Legendary","Genesis"],
                weights=[50, 25, 15, 7, 2.5, 0.5]
            )[0]
        rar = next((r for r in Rarity if r.value == rarity), Rarity.COMMON)

        # Name
        el_key = el_name
        names  = self.NAMES.get(el_key, ["Shivamon"])
        name   = random.choice(names) + f"-{self.total_minted+1:04d}"

        # Token ID
        token_id = "SHV-" + hashlib.sha256(
            f"{owner}{name}{time.time()}".encode()
        ).hexdigest()[:12].upper()

        nft = ShivamonNFT(token_id, name, el, rar, owner, generation)
        self.tokens[token_id]  = nft
        if owner not in self.owner_tokens:
            self.owner_tokens[owner] = []
        self.owner_tokens[owner].append(token_id)
        self.total_minted += 1

        return {"success": True, "shivamon": nft.to_dict()}

    # ── Transfer ───────────────────────────────────────
    def transfer(self, token_id: str, from_addr: str, to_addr: str) -> dict:
        nft = self.tokens.get(token_id)
        if not nft:
            return {"success": False, "error": "Token not found"}
        if nft.owner != from_addr:
            return {"success": False, "error": "Not the owner"}
        self.owner_tokens[from_addr].remove(token_id)
        nft.owner = to_addr
        if to_addr not in self.owner_tokens:
            self.owner_tokens[to_addr] = []
        self.owner_tokens[to_addr].append(token_id)
        return {"success": True, "token_id": token_id, "new_owner": to_addr}

    # ── Battle ─────────────────────────────────────────
    def battle(self, attacker_id: str, defender_id: str) -> dict:
        a = self.tokens.get(attacker_id)
        d = self.tokens.get(defender_id)
        if not a or not d:
            return {"success": False, "error": "Token not found"}

        import random
        rounds = []
        a_hp   = a.stats.hp
        d_hp   = d.stats.hp

        for rnd in range(1, 6):
            # Attacker schlägt
            dmg_a = max(1, a.stats.attack - d.stats.defense // 2 + random.randint(-5, 5))
            d_hp -= dmg_a
            rounds.append({"round": rnd, "attacker": a.name, "damage": dmg_a, "defender_hp": max(0, d_hp)})
            if d_hp <= 0:
                break
            # Defender schlägt zurück
            dmg_d = max(1, d.stats.attack - a.stats.defense // 2 + random.randint(-5, 5))
            a_hp -= dmg_d
            rounds.append({"round": rnd, "attacker": d.name, "damage": dmg_d, "defender_hp": max(0, a_hp)})
            if a_hp <= 0:
                break

        winner = a if a_hp > d_hp else d
        loser  = d if winner == a else a
        winner.wins   += 1
        loser.losses  += 1
        winner.gain_xp(50 + loser.level * 10)

        result = {
            "success":  True,
            "winner":   winner.to_dict(),
            "loser":    loser.to_dict(),
            "rounds":   rounds,
            "xp_gained": 50 + loser.level * 10
        }
        self.battle_log.append({"attacker": attacker_id, "defender": defender_id,
                                 "winner": winner.token_id, "ts": int(time.time())})
        return result

    # ── Queries ────────────────────────────────────────
    def get_token(self, token_id: str) -> dict:
        nft = self.tokens.get(token_id)
        return nft.to_dict() if nft else {"error": "Not found"}

    def get_owner_tokens(self, owner: str) -> list:
        ids = self.owner_tokens.get(owner, [])
        return [self.tokens[i].to_dict() for i in ids if i in self.tokens]

    def get_stats(self) -> dict:
        return {
            "total_minted": self.total_minted,
            "max_supply":   self.MAX_SUPPLY,
            "remaining":    self.MAX_SUPPLY - self.total_minted,
            "owners":       len(self.owner_tokens),
            "battles":      len(self.battle_log),
            "standard":     "ATC-9000"
        }

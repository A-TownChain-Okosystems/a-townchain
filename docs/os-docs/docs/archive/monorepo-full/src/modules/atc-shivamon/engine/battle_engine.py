# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Shivamon Battle Engine v2.0.0 — ATC-9000 Standard
Vollständiges Kampfsystem: Rundenbasiert, Typ-Schwächen, Moves, XP.
"""
import hashlib, random, time
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Tuple
from enum import Enum

class Element(Enum):
    FIRE     = "fire"
    WATER    = "water"
    EARTH    = "earth"
    AIR      = "air"
    SHADOW   = "shadow"
    NEON     = "neon"
    QUANTUM  = "quantum"

# Typ-Schwächen Matrix
TYPE_CHART: Dict[Element, Dict[Element, float]] = {
    Element.FIRE:    {Element.WATER: 0.5, Element.EARTH: 2.0, Element.AIR: 1.5},
    Element.WATER:   {Element.FIRE: 2.0,  Element.EARTH: 0.5, Element.NEON: 1.5},
    Element.EARTH:   {Element.WATER: 2.0, Element.AIR: 0.5},
    Element.AIR:     {Element.EARTH: 2.0, Element.FIRE: 0.5},
    Element.SHADOW:  {Element.NEON: 0.5,  Element.QUANTUM: 2.0},
    Element.NEON:    {Element.SHADOW: 2.0, Element.QUANTUM: 0.5},
    Element.QUANTUM: {Element.SHADOW: 0.5, Element.NEON: 2.0},
}

@dataclass
class Move:
    name:     str
    element:  Element
    power:    int         # 0-150
    accuracy: float       # 0.0-1.0
    pp:       int         # Power Points
    effect:   str = ""    # "burn", "stun", "heal", etc.

@dataclass
class BattleShivamon:
    name:     str
    element:  Element
    hp:       int
    max_hp:   int
    attack:   int
    defense:  int
    speed:    int
    moves:    List[Move]
    level:    int = 1

    def is_alive(self) -> bool: return self.hp > 0
    def hp_percent(self) -> float: return self.hp / self.max_hp if self.max_hp > 0 else 0

@dataclass
class BattleRound:
    round_num:    int
    attacker:     str
    defender:     str
    move:         str
    damage:       int
    effectiveness: float
    critical:     bool
    hp_after:     int
    event:        str = ""

class BattleEngine:
    """
    Rundenbasiertes Kampfsystem.
    Fairer RNG via SHA3-Seed (verifizierbarer Zufall).
    """

    def __init__(self, seed: Optional[str] = None):
        self._seed    = seed or hashlib.sha3_256(str(time.time()).encode()).hexdigest()
        self._counter = 0

    def _rng(self) -> float:
        self._counter += 1
        raw = f"{self._seed}{self._counter}".encode()
        h   = hashlib.sha3_256(raw).hexdigest()
        return int(h[:8], 16) / 0xFFFFFFFF

    def _damage(self, attacker: BattleShivamon, defender: BattleShivamon,
                move: Move) -> Tuple[int, float, bool]:
        # Typ-Effektivität
        chart    = TYPE_CHART.get(move.element, {})
        eff      = chart.get(defender.element, 1.0)
        # Critical Hit (6.25%)
        crit     = self._rng() < 0.0625
        crit_mul = 1.5 if crit else 1.0
        # Accuracy
        if self._rng() > move.accuracy:
            return 0, 0.0, False
        # Damage Formula (vereinfacht Game Freak)
        base  = ((2 * attacker.level / 5 + 2) * move.power * attacker.attack / defender.defense) / 50 + 2
        rand  = 0.85 + self._rng() * 0.15
        dmg   = int(base * eff * crit_mul * rand)
        return max(1, dmg), eff, crit

    def _priority(self, a: BattleShivamon, b: BattleShivamon) -> Tuple[BattleShivamon, BattleShivamon]:
        """Schnelleres Shivamon greift zuerst an."""
        if a.speed >= b.speed:
            return a, b
        return b, a

    def battle(self, s1: BattleShivamon, s2: BattleShivamon,
               max_rounds: int = 50) -> dict:
        rounds: List[BattleRound] = []
        round_num = 0

        while s1.is_alive() and s2.is_alive() and round_num < max_rounds:
            round_num += 1
            first, second = self._priority(s1, s2)

            for att, dfn in [(first, second), (second, first)]:
                if not att.is_alive() or not dfn.is_alive():
                    break
                # Besten verfügbaren Move wählen
                available = [m for m in att.moves if m.pp > 0]
                if not available:
                    rounds.append(BattleRound(round_num, att.name, dfn.name,
                        "Struggle", 5, 1.0, False, dfn.hp, "no PP left"))
                    dfn.hp = max(0, dfn.hp - 5)
                    continue
                move = max(available, key=lambda m: m.power)
                move.pp -= 1
                dmg, eff, crit = self._damage(att, dfn, move)
                dfn.hp = max(0, dfn.hp - dmg)
                event = ""
                if eff > 1: event = "super effective!"
                elif eff < 1 and eff > 0: event = "not very effective..."
                if crit: event += " Critical hit!"
                rounds.append(BattleRound(
                    round_num, att.name, dfn.name, move.name,
                    dmg, eff, crit, dfn.hp, event.strip()
                ))

        winner = s1.name if s1.is_alive() else s2.name if s2.is_alive() else "draw"
        xp_gain = int(sum(r.damage for r in rounds) * 0.5) + round_num * 2
        return {
            "winner":   winner,
            "rounds":   len(rounds),
            "xp_gain":  xp_gain,
            "log":      [vars(r) for r in rounds],
            "final_hp": {s1.name: s1.hp, s2.name: s2.hp},
            "seed":     self._seed,
        }

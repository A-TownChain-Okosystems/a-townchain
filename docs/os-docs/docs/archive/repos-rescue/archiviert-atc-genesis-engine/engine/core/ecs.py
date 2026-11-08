"""
Genesis Engine — Minimal ECS (Entity-Component-System) Core
MVP Milestone 1 — ECHTER CODE, keine Vision-Doku.

Design: einfach, dependency-frei, testbar. Entities sind IDs, Components
sind reine Datenklassen, Systems operieren auf (Entity, Components)-Tupeln.
"""
from dataclasses import dataclass, field
from typing import Dict, Type, List, Iterator, Tuple, Any
import itertools


class World:
    """Zentrale Verwaltung aller Entities und Components."""

    def __init__(self):
        self._next_id = itertools.count(1)
        self.entities: set[int] = set()
        # component_type -> {entity_id: component_instance}
        self.components: Dict[Type, Dict[int, Any]] = {}
        self.systems: List["System"] = []

    def create_entity(self) -> int:
        eid = next(self._next_id)
        self.entities.add(eid)
        return eid

    def destroy_entity(self, entity_id: int) -> None:
        self.entities.discard(entity_id)
        for store in self.components.values():
            store.pop(entity_id, None)

    def add_component(self, entity_id: int, component: Any) -> None:
        ctype = type(component)
        self.components.setdefault(ctype, {})[entity_id] = component

    def get_component(self, entity_id: int, ctype: Type):
        return self.components.get(ctype, {}).get(entity_id)

    def has_component(self, entity_id: int, ctype: Type) -> bool:
        return entity_id in self.components.get(ctype, {})

    def query(self, *ctypes: Type) -> Iterator[Tuple[int, tuple]]:
        """Liefert (entity_id, (comp1, comp2, ...)) fuer alle Entities,
        die ALLE angegebenen Component-Typen besitzen."""
        if not ctypes:
            return
        stores = [self.components.get(ct, {}) for ct in ctypes]
        base = stores[0]
        for eid in base:
            if all(eid in s for s in stores[1:]):
                yield eid, tuple(s[eid] for s in stores)

    def add_system(self, system: "System") -> None:
        system.world = self
        self.systems.append(system)

    def update(self, dt: float) -> None:
        for system in self.systems:
            system.update(dt)


class System:
    """Basisklasse fuer Systems. Konkrete Systems ueberschreiben update()."""
    world: World = None

    def update(self, dt: float) -> None:
        raise NotImplementedError


# --- Kern-Components (minimal, erweiterbar) ---

@dataclass
class Position:
    x: float = 0.0
    y: float = 0.0


@dataclass
class Velocity:
    dx: float = 0.0
    dy: float = 0.0


@dataclass
class Sprite:
    color: tuple = (255, 255, 255)
    width: int = 16
    height: int = 16


class MovementSystem(System):
    """Bewegt alle Entities mit Position + Velocity."""

    def update(self, dt: float) -> None:
        for eid, (pos, vel) in self.world.query(Position, Velocity):
            pos.x += vel.dx * dt
            pos.y += vel.dy * dt

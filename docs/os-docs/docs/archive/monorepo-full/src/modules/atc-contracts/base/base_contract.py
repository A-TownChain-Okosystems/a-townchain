# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
BaseContract — Basisklasse fuer alle A-TownChain Smart Contracts
Issue #1: Smart Contract Implementation
"""
import time, hashlib, json
from abc import ABC, abstractmethod
from typing import Optional


class ContractEvent:
    """Event-Objekt fuer Contract-Emissionen."""
    def __init__(self, name: str, data: dict, contract_address: str):
        self.name             = name
        self.data             = data
        self.contract_address = contract_address
        self.timestamp        = int(time.time())

    def to_dict(self):
        return {
            "event":    self.name,
            "data":     self.data,
            "contract": self.contract_address,
            "ts":       self.timestamp,
        }


class BaseContract(ABC):
    """
    Abstrakte Basisklasse fuer alle ATC Smart Contracts.
    Stellt Pause/Unpause, Owner-Check, Event-Log bereit.
    """

    def __init__(self, owner: str, contract_id: Optional[str] = None):
        self.owner    = owner
        self.address  = contract_id or self._gen_address(owner)
        self.paused   = False
        self.events: list[ContractEvent] = []
        self.created_at = int(time.time())

    def _gen_address(self, owner: str) -> str:
        seed = f"{owner}{time.time_ns()}"
        return "ATC_CONTRACT_" + hashlib.sha256(seed.encode()).hexdigest()[:16].upper()

    # ── Access Control ──────────────────────────────────
    def only_owner(self, caller: str) -> bool:
        if caller != self.owner:
            raise PermissionError(f"Only owner {self.owner} can call this")
        return True

    def when_not_paused(self):
        if self.paused:
            raise RuntimeError("Contract is paused")

    # ── Pause System ─────────────────────────────────────
    def pause(self, caller: str):
        self.only_owner(caller)
        self.paused = True
        self._emit("Paused", {"by": caller})

    def unpause(self, caller: str):
        self.only_owner(caller)
        self.paused = False
        self._emit("Unpaused", {"by": caller})

    # ── Event System ─────────────────────────────────────
    def _emit(self, name: str, data: dict):
        event = ContractEvent(name, data, self.address)
        self.events.append(event)
        return event

    def get_events(self, name: Optional[str] = None) -> list:
        if name:
            return [e.to_dict() for e in self.events if e.name == name]
        return [e.to_dict() for e in self.events]

    def info(self) -> dict:
        return {
            "address":    self.address,
            "owner":      self.owner,
            "paused":     self.paused,
            "created_at": self.created_at,
            "events":     len(self.events),
        }

    @abstractmethod
    def name(self) -> str: ...

# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ShivaCore Kernel — Capability-basierte Zugriffskontrolle
Milestone 1 fuer die Kernel-Weiterentwicklung (08.07.2026) — ECHTER CODE.

Baut direkt auf dem bestehenden KernelProcess.owner/.stake-Feld auf,
ohne die Kernarchitektur zu brechen (siehe AD-008: komplementaere Schichten).

Design (bewusst minimal, Layer-1-Konzept aus der Kernel-Architektur-Skizze,
aber ohne Hardware-/TEE-Abhaengigkeiten):
  - Capability: unveraenderliches Zugriffs-Ticket auf EINE Ressource
    (resource_type + resource_id) mit einer Menge von Rechten (Rights).
  - CapabilityManager: einzige Instanz, die Capabilities ausstellt,
    prueft, delegiert und entzieht. Kein Prozess kommt an eine Ressource
    vorbei am Manager.
  - Delegation kann Rechte nur EINSCHRAENKEN, nie erweitern.
"""
from dataclasses import dataclass, field
from enum import Flag, auto
from typing import Dict, Set, Optional
import uuid


class Right(Flag):
    NONE = 0
    READ = auto()
    WRITE = auto()
    EXECUTE = auto()
    DELEGATE = auto()   # darf diese Capability (eingeschraenkt) weitergeben
    ALL = READ | WRITE | EXECUTE | DELEGATE


class ResourceType:
    MEMORY = "memory"
    IPC_CHANNEL = "ipc_channel"
    DEVICE = "device"


class CapabilityError(Exception):
    pass


@dataclass(frozen=True)
class Capability:
    """Unveraenderliches Zugriffs-Ticket. frozen=True verhindert nachtraegliche
    Manipulation der Rechte durch Userspace-Code."""
    cap_id: str
    resource_type: str
    resource_id: int
    rights: Right
    owner_pid: int
    issued_by: int          # pid des Ausstellers (0 = Kernel selbst)
    parent_cap_id: Optional[str] = None   # gesetzt bei Delegation

    def has(self, right: Right) -> bool:
        return bool(self.rights & right)


class CapabilityManager:
    """Zentrale Instanz: alle Ressourcenzugriffe im Kernel muessen hier
    geprueft werden. Kein direkter Zugriff am Manager vorbei."""

    def __init__(self):
        # cap_id -> Capability
        self._caps: Dict[str, Capability] = {}
        # (resource_type, resource_id) -> set(cap_id) fuer schnellen Widerruf
        self._by_resource: Dict[tuple, Set[str]] = {}

    def grant(self, owner_pid: int, resource_type: str, resource_id: int,
              rights: Right, issued_by: int = 0) -> Capability:
        """Kernel (issued_by=0) oder ein Prozess mit DELEGATE-Recht stellt
        eine neue Capability aus."""
        cap = Capability(
            cap_id=str(uuid.uuid4()),
            resource_type=resource_type,
            resource_id=resource_id,
            rights=rights,
            owner_pid=owner_pid,
            issued_by=issued_by,
        )
        self._caps[cap.cap_id] = cap
        key = (resource_type, resource_id)
        self._by_resource.setdefault(key, set()).add(cap.cap_id)
        return cap

    def check(self, cap_id: str, resource_type: str, resource_id: int,
              required: Right) -> bool:
        """Prueft, ob eine Capability existiert, zur Ressource passt und
        das benoetigte Recht enthaelt."""
        cap = self._caps.get(cap_id)
        if cap is None:
            return False
        if cap.resource_type != resource_type or cap.resource_id != resource_id:
            return False
        return cap.has(required)

    def require(self, cap_id: str, resource_type: str, resource_id: int,
                required: Right) -> None:
        """Wie check(), wirft aber CapabilityError statt bool zurueckzugeben —
        fuer Kernel-interne Durchsetzung an Zugriffspunkten (alloc/read/write/...)."""
        if not self.check(cap_id, resource_type, resource_id, required):
            raise CapabilityError(
                f"Zugriff verweigert: cap={cap_id} resource={resource_type}:{resource_id} "
                f"benoetigt={required}"
            )

    def delegate(self, parent_cap_id: str, to_pid: int,
                 subset_rights: Right) -> Capability:
        """Erzeugt eine abgeleitete Capability mit EINGESCHRAENKTEN Rechten.
        Kann niemals mehr Rechte gewaehren als die Eltern-Capability."""
        parent = self._caps.get(parent_cap_id)
        if parent is None:
            raise CapabilityError(f"Eltern-Capability nicht gefunden: {parent_cap_id}")
        if not parent.has(Right.DELEGATE):
            raise CapabilityError("Eltern-Capability erlaubt keine Delegation")
        if subset_rights & ~parent.rights:
            raise CapabilityError("Delegation darf Rechte nur einschraenken, nie erweitern")

        child = Capability(
            cap_id=str(uuid.uuid4()),
            resource_type=parent.resource_type,
            resource_id=parent.resource_id,
            rights=subset_rights,
            owner_pid=to_pid,
            issued_by=parent.owner_pid,
            parent_cap_id=parent.cap_id,
        )
        self._caps[child.cap_id] = child
        key = (child.resource_type, child.resource_id)
        self._by_resource.setdefault(key, set()).add(child.cap_id)
        return child

    def revoke(self, cap_id: str, cascade: bool = True) -> int:
        """Entzieht eine Capability. Bei cascade=True werden auch alle
        davon abgeleiteten (delegierten) Capabilities entzogen.
        Gibt die Anzahl entzogener Capabilities zurueck."""
        if cap_id not in self._caps:
            return 0
        revoked = 0
        to_revoke = [cap_id]
        if cascade:
            to_revoke.extend(self._find_children(cap_id))

        for cid in to_revoke:
            cap = self._caps.pop(cid, None)
            if cap:
                key = (cap.resource_type, cap.resource_id)
                self._by_resource.get(key, set()).discard(cid)
                revoked += 1
        return revoked

    def _find_children(self, parent_cap_id: str) -> list:
        children = [c.cap_id for c in self._caps.values() if c.parent_cap_id == parent_cap_id]
        result = list(children)
        for child_id in children:
            result.extend(self._find_children(child_id))
        return result

    def capabilities_for_pid(self, pid: int) -> list:
        return [c for c in self._caps.values() if c.owner_pid == pid]

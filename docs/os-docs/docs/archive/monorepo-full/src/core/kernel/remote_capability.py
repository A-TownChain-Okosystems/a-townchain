# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ShivaCore Kernel — Remote-Capability-Tickets (RCT). Milestone 08.07.2026.

Erlaubt es, eine Capability kryptographisch signiert an einen fremden
Knoten (DID) zu delegieren -- ohne zentrale Vermittlungsinstanz. Baut auf
did.py (echte Ed25519-Signaturen) und capabilities.py (bestehendes
Rights/Delegation-Modell) auf.

EHRLICHE EINORDNUNG: Signaturpruefung, Replay-Schutz (Nonce-Speicher) und
Delegationsketten-Validierung sind ECHTE, funktionierende Logik -- keine
Simulation. NICHT enthalten: Hardware-Enklaven, formale TLA+/Coq-
Verifikation, DHT-Netzwerk-Transport (das Ticket muss "irgendwie" zum
Zielknoten kommen -- wie, ist hier nicht Teil des Kernels, wie in der
Spezifikation auch richtig beschrieben).
"""
from dataclasses import dataclass, field
from typing import Optional, List
import time
import secrets
import json

from src.core.kernel.did import Did, NodeIdentity
from src.core.kernel.capabilities import Right, ResourceType


class TicketError(Exception):
    pass


class ResolutionError(Exception):
    """Analog zu ResolutionError aus der Spezifikation (enum-artig ueber
    Subklassen realisiert, pythonischer als ein Rust-enum)."""


class InvalidSignatureError(ResolutionError): pass
class WrongSubjectError(ResolutionError): pass
class ReplayError(ResolutionError): pass
class ConstraintsTooStrictError(ResolutionError): pass
class ExpiredError(ResolutionError): pass


@dataclass(frozen=True)
class ResourceDescriptor:
    resource_type: str          # z.B. ResourceType.MEMORY / IPC_CHANNEL
    resource_id: int
    rights: Right


@dataclass(frozen=True)
class Constraints:
    max_operations: int
    deadline_unix: float                  # Unix-Timestamp
    energy_budget_uj: Optional[int] = None


@dataclass(frozen=True)
class RemoteCapabilityTicket:
    issuer_did: Did
    subject_did: Did
    resource: ResourceDescriptor
    constraints: Constraints
    nonce: str
    issuer_signature: bytes
    parent_ticket_nonce: Optional[str] = None   # fuer Delegationsketten

    def signing_payload(self) -> bytes:
        """Deterministische Byte-Repraesentation fuer Signatur/Verifikation."""
        payload = {
            "issuer_did": self.issuer_did.value,
            "subject_did": self.subject_did.value,
            "resource_type": self.resource.resource_type,
            "resource_id": self.resource.resource_id,
            "rights": self.resource.rights.value,
            "max_operations": self.constraints.max_operations,
            "deadline_unix": self.constraints.deadline_unix,
            "energy_budget_uj": self.constraints.energy_budget_uj,
            "nonce": self.nonce,
            "parent_ticket_nonce": self.parent_ticket_nonce,
        }
        return json.dumps(payload, sort_keys=True).encode()


def issue_ticket(issuer: NodeIdentity, subject_did: Did,
                  resource: ResourceDescriptor, constraints: Constraints,
                  parent_ticket_nonce: Optional[str] = None) -> RemoteCapabilityTicket:
    """Bob erstellt ein RCT fuer Alice (oder einen Vermittler Charlie bei
    Delegationsketten)."""
    nonce = secrets.token_hex(16)
    unsigned = RemoteCapabilityTicket(
        issuer_did=issuer.did, subject_did=subject_did, resource=resource,
        constraints=constraints, nonce=nonce, issuer_signature=b"",
        parent_ticket_nonce=parent_ticket_nonce,
    )
    signature = issuer.sign(unsigned.signing_payload())
    return RemoteCapabilityTicket(
        issuer_did=issuer.did, subject_did=subject_did, resource=resource,
        constraints=constraints, nonce=nonce, issuer_signature=signature,
        parent_ticket_nonce=parent_ticket_nonce,
    )


@dataclass
class LocalCap:
    """Ergebnis einer erfolgreichen Ticket-Einloesung: eine lokale
    Capability mit Constraints-basiertem Verbrauchszaehler."""
    cap_id: str
    resource: ResourceDescriptor
    constraints: Constraints
    issuer_did: Did
    operations_used: int = 0
    revoked: bool = False

    def consume_operation(self, energy_uj: int = 0) -> None:
        if self.revoked:
            raise TicketError("Capability bereits widerrufen")
        if self.operations_used >= self.constraints.max_operations:
            self.revoked = True
            raise TicketError("max_operations ueberschritten -- Capability widerrufen")
        if time.time() > self.constraints.deadline_unix:
            self.revoked = True
            raise TicketError("Deadline ueberschritten -- Capability widerrufen")
        self.operations_used += 1


class NonceStore:
    """Replay-Schutz. In der Spezifikation als Bloom-Filter beschrieben --
    hier als exaktes Set realisiert (korrekt, nur ohne Bloom-Filters
    Speichervorteil; fuer echte Knotenzahlen spaeter austauschbar,
    Interface bleibt gleich)."""

    def __init__(self):
        self._seen = set()

    def check_and_record(self, nonce: str) -> bool:
        """Gibt True zurueck wenn der Nonce NEU ist (und merkt ihn sich),
        False wenn er bereits gesehen wurde (Replay-Versuch)."""
        if nonce in self._seen:
            return False
        self._seen.add(nonce)
        return True


class RemoteCapabilityResolver:
    """Laeuft (konzeptuell) als isolierter Userspace-Dienst mit Zugriff auf
    DID-Speicher und Nonce-Speicher -- hier als Klasse mit eigenem State."""

    def __init__(self, own_did: Did):
        self.own_did = own_did
        self.nonces = NonceStore()

    def resolve(self, ticket: RemoteCapabilityTicket) -> LocalCap:
        """Validiert ein eingehendes Ticket vollstaendig und erzeugt eine
        lokale Capability. Wirft ResolutionError-Subklassen bei Ablehnung."""
        if not NodeIdentity.verify(ticket.issuer_did, ticket.signing_payload(),
                                   ticket.issuer_signature):
            raise InvalidSignatureError(f"Ungueltige Signatur von {ticket.issuer_did}")

        if ticket.subject_did.value != self.own_did.value:
            raise WrongSubjectError(
                f"Ticket ist fuer {ticket.subject_did}, nicht fuer {self.own_did}")

        if not self.nonces.check_and_record(ticket.nonce):
            raise ReplayError(f"Nonce {ticket.nonce} wurde bereits verwendet")

        if time.time() > ticket.constraints.deadline_unix:
            raise ExpiredError("Ticket-Deadline bereits verstrichen")

        if ticket.constraints.max_operations <= 0:
            raise ConstraintsTooStrictError("max_operations muss > 0 sein")

        return LocalCap(
            cap_id=f"rct-{ticket.nonce[:8]}",
            resource=ticket.resource,
            constraints=ticket.constraints,
            issuer_did=ticket.issuer_did,
        )

    def resolve_chain(self, chain: List[RemoteCapabilityTicket]) -> LocalCap:
        """Mehrstufige Delegation (Bob -> Charlie -> Alice). Jedes Ticket in
        der Kette muss gueltig sein UND darf Rechte nur einschraenken
        (Attenuation), nie erweitern, relativ zu seinem Elternticket."""
        if not chain:
            raise TicketError("Leere Delegationskette")

        for i in range(1, len(chain)):
            parent, child = chain[i - 1], chain[i]
            if child.parent_ticket_nonce != parent.nonce:
                raise TicketError(
                    f"Kettenbruch: Ticket {i} referenziert nicht Ticket {i-1}")
            if child.resource.resource_type != parent.resource.resource_type or \
               child.resource.resource_id != parent.resource.resource_id:
                raise TicketError("Kettenglied betrifft andere Ressource als Elternticket")
            if child.resource.rights & ~parent.resource.rights:
                raise TicketError(
                    "Kettenglied erweitert Rechte -- verstoesst gegen Attenuation")
            if child.constraints.max_operations > parent.constraints.max_operations:
                raise TicketError("Kettenglied erweitert max_operations")
            if child.constraints.deadline_unix > parent.constraints.deadline_unix:
                raise TicketError("Kettenglied erweitert Deadline")

        # Nur das letzte Ticket der Kette wird tatsaechlich fuer diesen Knoten eingeloest;
        # alle vorherigen muessen aber ebenfalls signaturgueltig sein.
        for t in chain[:-1]:
            if not NodeIdentity.verify(t.issuer_did, t.signing_payload(), t.issuer_signature):
                raise InvalidSignatureError(f"Ungueltige Signatur in Kettenglied von {t.issuer_did}")

        return self.resolve(chain[-1])

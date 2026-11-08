# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
DID-Resolver (ATAUTH-1000)
Deterministische did:kai:<hash> Identitaeten fuer Agenten/Nutzer,
mit Registrierung, Aufloesung, Verifikation, Update und Widerruf.
"""
import hashlib
import time
from dataclasses import dataclass, field
from typing import Dict, List, Optional


@dataclass
class DIDDocument:
    did: str
    public_key: str
    capabilities: List[str] = field(default_factory=list)
    created_at: float = field(default_factory=time.time)
    updated_at: float = field(default_factory=time.time)
    revoked: bool = False


class DIDResolver:
    """Registry + Resolver fuer did:kai:<hash> Identitaeten."""

    def __init__(self):
        self._registry: Dict[str, DIDDocument] = {}
        self._by_key: Dict[str, str] = {}  # public_key -> did (fuer verify)

    @staticmethod
    def create_did(public_key: str) -> str:
        """Erzeugt deterministisch dieselbe DID fuer denselben Public Key."""
        digest = hashlib.sha256(public_key.encode()).hexdigest()
        return f"did:kai:{digest}"

    def register(self, public_key: str, capabilities: Optional[List[str]] = None) -> DIDDocument:
        did = self.create_did(public_key)
        doc = self._registry.get(did)
        if doc is None:
            doc = DIDDocument(did=did, public_key=public_key, capabilities=capabilities or [])
            self._registry[did] = doc
            self._by_key[public_key] = did
        return doc

    def resolve(self, did: str) -> Optional[DIDDocument]:
        return self._registry.get(did)

    def verify(self, did: str, public_key: str) -> bool:
        doc = self._registry.get(did)
        if doc is None or doc.revoked:
            return False
        return doc.public_key == public_key

    def update(self, did: str, capabilities: Optional[List[str]] = None) -> Optional[DIDDocument]:
        doc = self._registry.get(did)
        if doc is None:
            return None
        if capabilities is not None:
            doc.capabilities = capabilities
        doc.updated_at = time.time()
        return doc

    def revoke(self, did: str) -> bool:
        doc = self._registry.get(did)
        if doc is None:
            return False
        doc.revoked = True
        doc.updated_at = time.time()
        return True

    def stats(self) -> Dict[str, int]:
        total = len(self._registry)
        revoked = sum(1 for d in self._registry.values() if d.revoked)
        return {"total": total, "active": total - revoked, "revoked": revoked}

# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ShivaCore Kernel — Dezentrale Knoten-Identitaet (DID) mit echten
Ed25519-Signaturen. Milestone 08.07.2026.

EHRLICHE EINORDNUNG: Der private Schluessel liegt hier im Prozessspeicher
(Python), NICHT in einer Hardware-Enklave (TrustZone/SGX/Secure Element) --
das gibt es in dieser Sandbox nicht und wird hier nicht vorgetaeuscht.
Die Kryptografie selbst (Ed25519-Signieren/Verifizieren) ist echt und
funktioniert genauso wie eine spaetere Hardware-Enklave-Implementierung
nutzen wuerde -- nur der Schluessel-Speicherort ist (noch) nicht
hardware-gesichert. Das ist der ehrliche Zwischenschritt.
"""
from dataclasses import dataclass
from cryptography.hazmat.primitives.asymmetric import ed25519
from cryptography.hazmat.primitives import serialization
from cryptography.exceptions import InvalidSignature
import base64


class DIDError(Exception):
    pass


@dataclass(frozen=True)
class Did:
    """Dezentrale Identitaet: did:shivacore:<base64-oeffentlicher-schluessel>"""
    value: str

    def __str__(self):
        return self.value


class NodeIdentity:
    """Eine Knoten-Identitaet mit Ed25519-Schluesselpaar. In einer spaeteren
    Hardware-Implementierung wuerde `_private_key` durch einen Handle auf
    eine Secure-Enclave-API ersetzt -- die oeffentliche Schnittstelle
    (did, sign, verify) bliebe identisch."""

    def __init__(self, private_key: ed25519.Ed25519PrivateKey = None):
        self._private_key = private_key or ed25519.Ed25519PrivateKey.generate()
        pub_bytes = self._private_key.public_key().public_bytes(
            encoding=serialization.Encoding.Raw,
            format=serialization.PublicFormat.Raw,
        )
        self.did = Did(f"did:shivacore:{base64.urlsafe_b64encode(pub_bytes).decode().rstrip('=')}")
        self._public_key = self._private_key.public_key()

    @classmethod
    def generate(cls) -> "NodeIdentity":
        return cls()

    def sign(self, message: bytes) -> bytes:
        """Syscall-Aequivalent zu sign_challenge() aus der Spezifikation."""
        return self._private_key.sign(message)

    def public_key_bytes(self) -> bytes:
        return self._public_key.public_bytes(
            encoding=serialization.Encoding.Raw,
            format=serialization.PublicFormat.Raw,
        )

    @staticmethod
    def verify(did: Did, message: bytes, signature: bytes) -> bool:
        """Verifiziert eine Signatur gegen die im DID kodierte
        Oeffentliche-Schluessel. Echte Ed25519-Kryptografie."""
        try:
            b64 = did.value.split("did:shivacore:")[1]
            padded = b64 + "=" * (-len(b64) % 4)
            pub_bytes = base64.urlsafe_b64decode(padded)
            pub_key = ed25519.Ed25519PublicKey.from_public_bytes(pub_bytes)
            pub_key.verify(signature, message)
            return True
        except (InvalidSignature, IndexError, ValueError):
            return False

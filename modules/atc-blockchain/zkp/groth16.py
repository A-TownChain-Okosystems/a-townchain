# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Canonical ZKP boundary for the A-TownChain blockchain.

The production implementation lives in the Rust ``atc_zkp`` module.
This Python module is compatibility-only: it never implements or silently
emulates proof generation/verification. Callers must explicitly select the
canonical Rust implementation.
"""

from __future__ import annotations


class ZKPBackendUnavailable(RuntimeError):
    """Raised when the canonical Rust ZKP backend is not available."""


class ZKPLayer:
    """Compatibility boundary; proof operations are delegated to Rust."""

    def __init__(self, *args, **kwargs):
        raise ZKPBackendUnavailable(
            "The canonical ZKP backend is Rust module 'atc_zkp'. "
            "Python proof generation/verification is not supported."
        )


def get_zkp_layer() -> ZKPLayer:
    """Fail closed instead of silently selecting a non-canonical backend."""
    raise ZKPBackendUnavailable(
        "No Python ZKP backend is registered. Use the canonical Rust 'atc_zkp' backend."
    )


class ShieldedTransaction:
    """Compatibility type; construction requires the canonical Rust backend."""

    __slots__ = ()

    def __new__(cls, *args, **kwargs):
        raise ZKPBackendUnavailable(
            "ShieldedTransaction is implemented by the canonical Rust ZKP backend."
        )


class Groth16Proof:
    """Compatibility type; construction requires the canonical Rust backend."""

    __slots__ = ()

    def __new__(cls, *args, **kwargs):
        raise ZKPBackendUnavailable(
            "Groth16Proof is implemented by the canonical Rust ZKP backend."
        )

# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Supply-Cap-Invariante des ATCoin (Owner-Entscheidung 08.11.2026).

Der Owner hat verbindlich festgelegt: maximale Gesamtmenge 360.000.000 ATC,
18 Dezimalstellen, Chain-ID 658467, begrenztes Supply-Modell. Der Wert
21.000.000 ist ein nicht autorisierter Spiegelwert und darf nicht als
alternative Protokollregel bestehen bleiben.

Diese Tests binden die Entscheidung an die Quellen, damit sie nicht still
zurueckdriftet. Sie pruefen Quellen, nicht Laufzeitverhalten.
"""

from __future__ import annotations

import hashlib
import re
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[3]
REGISTRY = ROOT / "standards/contracts/registry/contracts.yaml"

# Owner-Entscheidung 08.11.2026
CAP_TOKENS = 360_000_000
DECIMALS = 18
CHAIN_ID = 658467
CAP_WEI = CAP_TOKENS * 10**DECIMALS
UNAUTHORIZED_CAP_WEI = 21_000_000 * 10**DECIMALS

CONTRACT_RE = re.compile(r"^\s*contract\s+ATCoin\b", re.MULTILINE)
CAP_RE = re.compile(r"(?:MAX_SUPPLY|ATC_MAX_SUPPLY)\s*:\s*u(?:64|128|256)\s*=\s*([0-9_]+)")


def atc_sources() -> list[Path]:
    return [
        p
        for p in sorted(ROOT.rglob("*.atc"))
        if p.is_file()
        and p.relative_to(ROOT).parts[0] not in {"docs", "target", "node_modules", ".git"}
    ]


def atcoin_sources() -> list[Path]:
    out = []
    for path in atc_sources():
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        if CONTRACT_RE.search(text):
            out.append(path)
    return out


def caps_in(path: Path) -> list[int]:
    text = path.read_text(encoding="utf-8")
    return [int(m.group(1).replace("_", "")) for m in CAP_RE.finditer(text)]


def test_atcoin_sources_exist():
    sources = atcoin_sources()
    assert sources, "kein Vertrag ATCoin gefunden"


@pytest.mark.parametrize("path", atcoin_sources(), ids=lambda p: str(p.relative_to(ROOT)))
def test_every_atcoin_source_declares_the_owner_cap(path: Path):
    caps = caps_in(path)
    assert caps, f"{path.relative_to(ROOT)} deklariert keinen Supply-Cap"
    for cap in caps:
        assert cap == CAP_WEI, (
            f"{path.relative_to(ROOT)}: Cap {cap} != {CAP_WEI} "
            f"({CAP_TOKENS:,} ATC * 10^{DECIMALS})"
        )


def test_no_unauthorized_21_million_cap_anywhere():
    offenders = []
    for path in atc_sources():
        if UNAUTHORIZED_CAP_WEI in caps_in(path):
            offenders.append(str(path.relative_to(ROOT)))
    assert not offenders, f"nicht autorisierter 21-Mio-Cap in: {offenders}"


def test_canonical_atcoin_source_declares_decimals():
    """Kanonisch ist atcoin.atc (ATC-89); dort steht die Dezimalzahl als Konstante."""
    canonical = ROOT / "apps/atc-contracts/atcoin/atcoin.atc"
    text = canonical.read_text(encoding="utf-8")
    assert re.search(rf"ATC_DECIMALS\s*:\s*u8\s*=\s*{DECIMALS}\b", text), "Dezimalstellen != 18"


CHAIN_ID_RE = re.compile(r"CHAIN_ID\s*:\s*u64\s*=\s*([0-9_]+)")


@pytest.mark.parametrize("path", atcoin_sources(), ids=lambda p: str(p.relative_to(ROOT)))
def test_chain_id_is_the_atc_chain_inside_atc_chain_sources(path: Path):
    """Die Chain-ID gilt innerhalb der ATC-Blockchain (Owner 08.11.2026).

    Geprueft wird nur, was die ATC-Chain selbst deklariert. Chain-IDs fremder
    Ketten (Bridge-/Interop-Kontexte) sind ausdruecklich nicht Gegenstand dieser
    Invariante und werden hier weder gelesen noch festgeschrieben.
    """
    text = path.read_text(encoding="utf-8")
    for found in CHAIN_ID_RE.finditer(text):
        value = int(found.group(1).replace("_", ""))
        assert value == CHAIN_ID, (
            f"{path.relative_to(ROOT)}: CHAIN_ID {value} != {CHAIN_ID} "
            "(ATC-Chain-Konstante)"
        )


def test_no_token_source_declares_a_foreign_chain_id():
    """Nach der Rollentrennung traegt kein Token den Chain-ID-Zustand.

    Die Chain-ID gilt innerhalb der ATC-Blockchain und gehoert zur
    Chain-Konfiguration, nicht zum Token-Zustand.
    """
    for path in atcoin_sources():
        values = [int(m.group(1).replace("_", "")) for m in CHAIN_ID_RE.finditer(path.read_text(encoding="utf-8"))]
        assert all(v == CHAIN_ID for v in values), f"{path.relative_to(ROOT)}: fremde Chain-ID {values}"


def test_registry_binds_cap_to_the_canonical_bytes():
    registry = yaml.safe_load(REGISTRY.read_text(encoding="utf-8"))
    entry = next(e for e in registry["contracts"] if e["contract_id"] == "ATC-SC-TOKEN-001")
    canonical = ROOT / entry["contract_path"]
    actual = hashlib.sha256(canonical.read_bytes()).hexdigest()
    assert entry["source_sha256"] == actual, "source_sha256 bindet nicht an die kanonischen Bytes"
    assert caps_in(canonical) == [CAP_WEI], "kanonische Quelle traegt nicht genau den Owner-Cap"

# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Regressionstests fuer das Contract-Registry-Gate (ATC-STD-SC-019).

Die Tests bauen jeweils einen kleinen Monorepo-Ausschnitt im tmp_path und
pruefen, dass jedes Gate (A1-A7) genau seinen Fehler findet.
"""

from __future__ import annotations

import hashlib
import importlib.util
import sys
from pathlib import Path

import pytest

TOOL = Path(__file__).resolve().parents[1] / "check_contract_registry.py"


def load_tool():
    spec = importlib.util.spec_from_file_location("sc_gate", TOOL)
    mod = importlib.util.module_from_spec(spec)
    sys.modules["sc_gate"] = mod
    spec.loader.exec_module(mod)
    return mod


GATE = load_tool()


def make_tree(tmp_path: Path, *, status: str = "development", network=None, extra_entry: str = "") -> Path:
    base = tmp_path / "apps/atc-contracts"
    base.mkdir(parents=True)
    src = base / "token.atc"
    src.write_text("// Token-Contract\ncontract Token {}\n", encoding="utf-8")
    digest = hashlib.sha256(src.read_bytes()).hexdigest()
    registry = tmp_path / "standards/contracts/registry/contracts.yaml"
    registry.parent.mkdir(parents=True)
    registry.write_text(
        "contracts:\n"
        "  - contract_id: ATC-SC-TOKEN-001\n"
        '    name: "Token"\n'
        "    category: SC-TOKEN\n"
        '    version: "0.1.0"\n'
        f"    status: {status}\n"
        "    language: atclang\n"
        "    runtime: atc-vm\n"
        "    owner: ATC-Governance\n"
        "    upgradeable: false\n"
        "    verified: false\n"
        "    audited: false\n"
        "    source_repository: A-TownChain-Okosystems/a-townchain\n"
        "    contract_path: apps/atc-contracts/token.atc\n"
        f"    source_sha256: {digest}\n"
        f"{extra_entry}"
        + (f"    network: {network}\n" if network else "")
        + "exclusions:\n"
        '  - "apps/atc-contracts/modules/**"\n',
        encoding="utf-8",
    )
    return registry


def deployments(tmp_path: Path, body: str = "deployments: []\n") -> Path:
    path = tmp_path / "standards/contracts/registry/deployments.yaml"
    path.write_text(body, encoding="utf-8")
    return path


def run(tmp_path: Path, registry: Path, deps: Path):
    return GATE.check_registry(tmp_path, registry, deps)


def gates(findings):
    return {f["gate"] for f in findings}


def test_sauberer_bestand_besteht():
    findings, stats = GATE.check_registry(GATE.ROOT, GATE.REGISTRY, GATE.DEPLOYMENTS)
    assert findings == [], f"unerwartete Befunde: {findings}"
    assert stats["contracts"] == stats["sources"]
    assert stats["contracts"] >= 16


def test_a1_pflichtfeld_fehlt(tmp_path):
    reg = make_tree(tmp_path)
    text = reg.read_text().replace('    owner: ATC-Governance\n', "")
    reg.write_text(text)
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    assert "A1" in gates(findings)
    assert any("owner" in f["detail"] for f in findings)


def test_a2_pfad_existiert_nicht(tmp_path):
    reg = make_tree(tmp_path)
    reg.write_text(reg.read_text().replace("token.atc", "fehlt.atc"))
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    assert "A2" in gates(findings)


def test_a3_drift_wird_erkannt(tmp_path):
    reg = make_tree(tmp_path)
    (tmp_path / "apps/atc-contracts/token.atc").write_text("// geaendert\n", encoding="utf-8")
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    assert "A3" in gates(findings)
    assert any("driftet" in f["detail"] for f in findings)


def test_a4_waisenquelle_wird_gemeldet(tmp_path):
    reg = make_tree(tmp_path)
    (tmp_path / "apps/atc-contracts/orphan.atc").write_text("// orphan\n", encoding="utf-8")
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    assert "A4" in gates(findings)
    assert any("orphan.atc" in f["detail"] for f in findings)


def test_a4_ausschluss_wird_respektiert(tmp_path):
    reg = make_tree(tmp_path)
    mirror = tmp_path / "apps/atc-contracts/modules/token.atc"
    mirror.parent.mkdir(parents=True)
    mirror.write_text("// Spiegel\n", encoding="utf-8")
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    assert "A4" not in gates(findings), f"Spiegel darf keine Waise sein: {findings}"


def test_a5_deployed_ohne_netz_und_record(tmp_path):
    reg = make_tree(tmp_path, status="deployed")
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    details = [f["detail"] for f in findings if f["gate"] == "A5"]
    assert any("network" in d for d in details)
    assert any("Deploy-Record" in d for d in details)


def test_a5_deployed_mit_record_besteht(tmp_path):
    reg = make_tree(tmp_path, status="deployed", network="devnet")
    deps = deployments(tmp_path, "deployments:\n  - contract_id: ATC-SC-TOKEN-001\n    network: devnet\n")
    findings, _ = run(tmp_path, reg, deps)
    assert findings == [], f"unerwartet: {findings}"


def test_a5_verified_ohne_auditstatus(tmp_path):
    reg = make_tree(tmp_path)
    reg.write_text(reg.read_text().replace("    verified: false", "    verified: true"))
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    assert "A5" in gates(findings)


def test_a5_archived_braucht_marker(tmp_path):
    reg = make_tree(tmp_path, status="archived")
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    assert any("Archivmarker" in f["detail"] for f in findings)


def test_a6_id_format_und_dopplung(tmp_path):
    reg = make_tree(tmp_path)
    reg.write_text(reg.read_text().replace("ATC-SC-TOKEN-001", "TOKEN-1"))
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    assert "A6" in gates(findings)


def test_a6_kategorie_passt_nicht_zur_id(tmp_path):
    reg = make_tree(tmp_path)
    reg.write_text(reg.read_text().replace("category: SC-TOKEN", "category: SC-DEFI"))
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    assert any("passt nicht zur ID" in f["detail"] for f in findings)


def test_a7_unbekannte_kategorie(tmp_path):
    reg = make_tree(tmp_path)
    reg.write_text(reg.read_text().replace("category: SC-TOKEN", "category: SC-UNSINN"))
    findings, _ = run(tmp_path, reg, deployments(tmp_path))
    assert "A7" in gates(findings)

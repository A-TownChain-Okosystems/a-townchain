# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Regressionstests fuer den Vertragskopien-Audit (ATC-STD-SC-019).

Der Audit friert ein, welche Vertragsnamen mehrfach vorkommen und ob die Kopien
byte-gleich (``identical``) oder getrennte Implementierungen (``divergent``)
sind. Diese Tests pruefen die Vergleichslogik und dass der eingecheckte Stand
zum Baum passt.
"""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path

TOOL = Path(__file__).resolve().parents[1] / "audit_contract_copies.py"


def load_tool():
    spec = importlib.util.spec_from_file_location("copy_audit", TOOL)
    mod = importlib.util.module_from_spec(spec)
    sys.modules["copy_audit"] = mod
    spec.loader.exec_module(mod)
    return mod


def entry(kind: str, copies: int, variants: int, files: list[str]) -> dict:
    return {"kind": kind, "copies": copies, "variants": variants, "files": files}


def test_identical_becoming_divergent_is_reported():
    tool = load_tool()
    frozen = {"X": entry("identical", 3, 1, ["a.atc", "b.atc", "c.atc"])}
    actual = {"X": entry("divergent", 3, 2, ["a.atc", "b.atc", "c.atc"])}
    findings = tool.compare(frozen, actual)
    assert any("driften jetzt" in f for f in findings)


def test_new_duplicate_name_is_reported():
    tool = load_tool()
    findings = tool.compare({}, {"Y": entry("identical", 2, 1, ["a.atc", "b.atc"])})
    assert any("neue Mehrfachdeklaration" in f for f in findings)


def test_vanished_duplicate_name_is_reported():
    tool = load_tool()
    frozen = {"Z": entry("identical", 2, 1, ["a.atc", "b.atc"])}
    findings = tool.compare(frozen, {})
    assert any("verschwunden" in f for f in findings)


def test_extra_variant_in_divergent_group_is_reported():
    tool = load_tool()
    frozen = {"D": entry("divergent", 4, 2, ["a.atc", "b.atc", "c.atc", "d.atc"])}
    actual = {"D": entry("divergent", 4, 3, ["a.atc", "b.atc", "c.atc", "d.atc"])}
    findings = tool.compare(frozen, actual)
    assert any("Variantenzahl" in f for f in findings)


def test_unchanged_state_is_clean():
    tool = load_tool()
    frozen = {"S": entry("identical", 2, 1, ["a.atc", "b.atc"])}
    assert tool.compare(frozen, dict(frozen)) == []


def test_checked_in_state_matches_the_tree():
    tool = load_tool()
    assert tool.main([]) == 0, "eingefrorene Einordnung passt nicht zum Baum (--write pruefen)"

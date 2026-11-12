"""Tests fuer die Bindung der Admission-Codeliste (Spezifikation, SSOT, Rust)."""
from __future__ import annotations

import json
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import check_admission_codes as cac  # noqa: E402


def test_liste_ist_konsistent() -> None:
    assert cac.pruefe() == []


def test_elf_aktive_stufen_und_reihenfolge() -> None:
    tabelle = cac.spec_tabelle()
    assert [schritt for schritt, _ in tabelle] == list(range(1, 12))
    assert [code for _, code in tabelle] == [
        "ADM-010",
        "ADM-009",
        "ADM-001",
        "ADM-002",
        "ADM-003",
        "ADM-004",
        "ADM-005",
        "ADM-006",
        "ADM-007",
        "ADM-008",
        "ADM-011",
    ]


def test_adm012_ist_vorgeschlagen_und_nicht_implementiert() -> None:
    ssot = cac.lade_ssot()
    eintrag = next(c for c in ssot["codes"] if c["code"] == "ADM-012")
    assert eintrag["status"] == "proposed"
    assert eintrag["stage"] == "registration"
    assert "ADM-012" not in cac.rust_code_map().values(), "Sperre muss sichtbar bleiben"
    assert "ADM-012" in cac.SPEC.read_text(encoding="utf-8")


def test_implementierung_kennt_genau_die_aktiven_codes() -> None:
    assert set(cac.rust_code_map().values()) == {
        c["code"] for c in cac.lade_ssot()["codes"] if c["status"] == "active"
    }


def test_alte_kennung_ist_nur_ein_zeiger() -> None:
    """ATC-SEC-EXEC-001 darf keine Code-Tabelle mehr fuehren (eine Wahrheit)."""
    zeiger = cac.POINTER.read_text(encoding="utf-8")
    assert "ATC-STD-410-CANDIDATE.md" in zeiger
    assert "| `ADM-" not in zeiger, "keine Code-Tabelle im Zeiger"


def test_sperre_wird_erkannt(tmp_path) -> None:
    """Wird ADM-012 vorzeitig implementiert, muss der Pruefer das melden."""
    ssot = cac.lade_ssot()
    for eintrag in ssot["codes"]:
        if eintrag["code"] == "ADM-012":
            eintrag["status"] = "active"
            eintrag["order"] = 12
            eintrag["rust_variant"] = "PolicyConflict"  # absichtlich verfrueht
    pfad = tmp_path / "admission-codes.json"
    pfad.write_text(json.dumps(ssot), encoding="utf-8")
    original = cac.SSOT
    try:
        cac.SSOT = pfad
        fehler = cac.pruefe()
    finally:
        cac.SSOT = original
    assert any("ADM-012" in f for f in fehler), fehler


def test_doppelte_nummer_wird_erkannt(tmp_path) -> None:
    ssot = cac.lade_ssot()
    ssot["codes"][1]["code"] = ssot["codes"][0]["code"]
    pfad = tmp_path / "admission-codes.json"
    pfad.write_text(json.dumps(ssot), encoding="utf-8")
    original = cac.SSOT
    try:
        cac.SSOT = pfad
        fehler = cac.pruefe()
    finally:
        cac.SSOT = original
    assert any("doppelte" in f or "luecken" in f for f in fehler), fehler

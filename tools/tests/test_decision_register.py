"""Tests fuer das Entscheidungsregister (D1 bis D7) und seine Teilauflagen."""

from __future__ import annotations

import copy
import subprocess
import sys
from pathlib import Path

import pytest

TOOLS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(TOOLS))

import decision_register as dr  # noqa: E402


@pytest.fixture(scope="module")
def data() -> dict:
    return dr.load()


STATUSE = {"DECIDED", "SPECIFIED", "IMPLEMENTED", "OPEN"}


def d1(data: dict) -> dict:
    return next(d for d in data["decisions"] if d["id"] == "D1")


def sub_gate(entry: dict, sid: str) -> dict:
    for dec in entry["decisions"]:
        for sub in dec.get("sub_gates") or []:
            if sub["id"] == sid:
                return sub
    raise AssertionError(f"{sid} fehlt im Register")


def test_register_ist_gueltig(data: dict) -> None:
    assert dr.validate(data) == []


def test_check_laeuft_ueber_die_echte_datei() -> None:
    result = subprocess.run(
        [sys.executable, str(TOOLS / "decision_register.py"), "--check"],
        capture_output=True,
        text=True,
        check=False,
    )
    assert result.returncode == 0, result.stdout


def test_d1_hat_drei_getrennte_teilauflagen(data: dict) -> None:
    ids = [sub["id"] for dec in data["decisions"] for sub in (dec.get("sub_gates") or [])]
    assert ids == ["D1-A", "D1-B", "D1-C"]


def test_teilauflagen_haben_eigene_zustaendigkeit(data: dict) -> None:
    assert sub_gate(data, "D1-A")["owner"] == "Admission-Layer"
    assert sub_gate(data, "D1-B")["owner"] == "VM-Parser"
    assert sub_gate(data, "D1-C")["owner"] == "VM-Parser / Admission-Vertrag"


def test_teilauflagen_ohne_nachweis_bleiben_offen(data: dict) -> None:
    """OPEN braucht einen Offen-Grund, IMPLEMENTED einen Nachweis.

    D1-A haengt an der Messung in P1b und bleibt offen. D1-B und D1-C sind
    umgesetzt; ihr Eintrag muss den Nachweis nennen, sonst ist der Status eine
    Behauptung ohne Beleg.
    """
    a = sub_gate(data, "D1-A")
    assert a["status"] == "OPEN"
    assert "P1b" in a["open_because"]

    for sid in ("D1-B", "D1-C"):
        sub = sub_gate(data, sid)
        assert sub["status"] in STATUSE
        assert "EXEC-D1BC-VM-PARSER-EVIDENCE-2026-11-12.md" in sub["open_because"]
        assert "CI_VERIFIED offen" in sub["open_because"] or "P1b" in sub["open_because"]


def test_teilauflagen_werden_gerendert(data: dict) -> None:
    block = dr.render(data)
    assert "**Teilauflagen**" in block
    for sid in ("D1-A", "D1-B", "D1-C"):
        assert f"| {sid} |" in block


def test_d0_ist_eigene_entscheidung(data: dict) -> None:
    """D-0 (Formate) steht als eigene Entscheidung im Register und ist entschieden."""
    d0 = next(d for d in data["decisions"] if d["id"] == "D-0")
    assert d0["decision_status"] == "DECIDED"
    assert d0["implementation_status"] == "OPEN", "ATCE/0x02 bleibt OPEN bis Integration"
    assert any("ATCE" in c for c in d0["conditions"])


def test_doppelte_teilauflagen_id_wird_erkannt(data: dict) -> None:
    broken = copy.deepcopy(data)
    d1(broken)["sub_gates"].append(copy.deepcopy(d1(broken)["sub_gates"][0]))
    assert any("doppelte ID in sub_gates" in e for e in dr.validate(broken))


def test_ungueltiger_teilauflagen_status_wird_erkannt(data: dict) -> None:
    broken = copy.deepcopy(data)
    d1(broken)["sub_gates"][0]["status"] = "DONE"
    assert any("status 'DONE' ungueltig" in e for e in dr.validate(broken))


def test_fehlende_zustaendigkeit_wird_erkannt(data: dict) -> None:
    broken = copy.deepcopy(data)
    del d1(broken)["sub_gates"][0]["owner"]
    assert any("Feld 'owner' fehlt" in e for e in dr.validate(broken))

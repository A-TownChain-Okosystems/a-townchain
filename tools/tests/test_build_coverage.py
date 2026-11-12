"""Tests fuer die Governance-Regel G-01 (Build-Abdeckung)."""
import importlib.util
from pathlib import Path

WURZEL = Path(__file__).resolve().parent.parent.parent
spec = importlib.util.spec_from_file_location(
    "check_build_coverage", WURZEL / "tools" / "check_build_coverage.py"
)
modul = importlib.util.module_from_spec(spec)
spec.loader.exec_module(modul)


def test_registry_deckt_alle_manifeste():
    """R1: kein Manifest ohne Eintrag — die Luecke aus F4 darf nicht zurueckkommen."""
    gefunden = set(modul.manifeste())
    eingetragen = {e["path"] for e in modul.lies_registry()}
    assert gefunden - eingetragen == set()


def test_jeder_eintrag_hat_grund_und_status():
    """R2: eine Klassifikation ohne Begruendung ist keine Klassifikation."""
    for e in modul.lies_registry():
        assert e.get("status") in modul.STATUS, e
        assert e.get("reason"), e


def test_duplikat_knoten_ist_stillgelegt():
    """F4: apps/atc-node ist als Dublette gefuehrt und wird nicht gebaut."""
    eintraege = {e["path"]: e for e in modul.lies_registry()}
    eintrag = eintraege["apps/atc-node"]
    assert eintrag["status"] == "duplicate_retired"
    quelle = (WURZEL / "apps/atc-node/src/gossip.rs").read_text(encoding="utf-8")
    assert "pub fn sync_pull" not in quelle


def test_gate_laeuft_und_meldet_offene_befunde():
    """R5: offene Manifeste werden gezaehlt, blockieren aber den Gate-Lauf nicht."""
    assert modul.main([]) == 0

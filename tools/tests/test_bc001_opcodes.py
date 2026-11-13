"""Tests fuer die Bindung der ATC-BC-001-Opcode-Tabelle (E2-C-1).

Die Tests pruefen nicht nur den Ist-Zustand, sondern auch, dass der Drift-Test
tatsaechlich anschlaegt: jede Verfaelschung einer Kopie muss Fehler erzeugen.
"""

from __future__ import annotations

import shutil
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import check_bc001_opcodes as bc  # noqa: E402

# ---------------------------------------------------------------- Hilfsmittel


def _kopie(tmp_path: Path, quelle: Path, name: str) -> Path:
    ziel = tmp_path / name
    shutil.copyfile(quelle, ziel)
    return ziel


@pytest.fixture()
def umgebung(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> dict[str, Path]:
    """Kopiert Spezifikation, SSOT und Implementierungen in ein temporaeres
    Verzeichnis und zeigt die Modulpfade darauf. Die echten Dateien bleiben
    unberuehrt."""
    pfade = {
        "spec": _kopie(tmp_path, bc.SPEC, "SPEC.md"),
        "bytecode": _kopie(tmp_path, bc.ATC_CORE_BYTECODE, "bytecode_core.rs"),
        "artifact": _kopie(tmp_path, bc.ATC_CORE_ARTIFACT, "artifact_core.rs"),
        "vm": _kopie(tmp_path, bc.VM_ARTIFACT, "artifact_vm.rs"),
    }
    ssot = tmp_path / "bc001-opcodes.json"
    ssot.write_text(bc.SSOT.read_text(encoding="utf-8"), encoding="utf-8")
    pfade["ssot"] = ssot

    monkeypatch.setattr(bc, "SPEC", pfade["spec"])
    monkeypatch.setattr(bc, "SSOT", pfade["ssot"])
    monkeypatch.setattr(bc, "ATC_CORE_BYTECODE", pfade["bytecode"])
    monkeypatch.setattr(bc, "ATC_CORE_ARTIFACT", pfade["artifact"])
    monkeypatch.setattr(bc, "VM_ARTIFACT", pfade["vm"])
    return pfade


# ------------------------------------------------------------------ Abnahme


def test_19_opcodes_vollstaendig() -> None:
    tabelle = bc.spec_tabelle()
    assert len(tabelle) == 19
    assert bc.spec_metadaten()["declared_opcode_count"] == 19
    assert [z["name"] for z in tabelle][:4] == ["ConstI64", "LoadLocal", "StoreLocal", "Add"]
    assert [z["name"] for z in tabelle][-3:] == ["Jump", "JumpIfFalse", "Pop"]
    assert [z["opcode"] for z in tabelle] == [
        "0x01",
        "0x02",
        "0x03",
        "0x10",
        "0x11",
        "0x12",
        "0x13",
        "0x14",
        "0x15",
        "0x16",
        "0x17",
        "0x18",
        "0x19",
        "0x1A",
        "0x20",
        "0x21",
        "0x30",
        "0x31",
        "0x40",
    ]


def test_bytewerte_eindeutig() -> None:
    codes = [z["opcode"] for z in bc.spec_tabelle()]
    assert len(codes) == len(set(codes))
    namen = [z["name"] for z in bc.spec_tabelle()]
    assert len(namen) == len(set(namen))


def test_ssot_entspricht_spezifikation() -> None:
    assert bc.SSOT.read_text(encoding="utf-8") == bc.erzeuge()


def test_keine_zusaetzlichen_oder_fehlenden_opcodes() -> None:
    spez = {z["opcode"] for z in bc.spec_tabelle()}
    ssot = {z["opcode"] for z in bc.lade_ssot()["opcodes"]}
    assert spez == ssot
    assert len(spez) == 19


def test_determinismus() -> None:
    erste = bc.erzeuge()
    zweite = bc.erzeuge()
    dritte = bc.erzeuge()
    assert erste == zweite == dritte
    assert bc.render(bc.spec_tabelle(), bc.spec_metadaten(), bc.spec_sha256()) == erste


def test_endianness_container_und_version_gebunden() -> None:
    quelle = bc.lade_ssot()["source"]
    assert quelle["standard"] == "ATC-BC-001"
    assert quelle["endianness"] == "big"
    assert quelle["container"] == "ATCB"
    assert quelle["format_version"] == "1"
    assert quelle["spec_sha256"] == bc.spec_sha256()


def test_operandenformen_bekannt() -> None:
    for zeile in bc.spec_tabelle():
        assert zeile["operand"] in bc.ERLAUBTE_OPERANDEN


def test_implementierungen_stimmen_ueberein() -> None:
    spez = {z["name"]: int(z["opcode"], 16) for z in bc.spec_tabelle()}
    assert bc.rust_kodierer() == spez
    assert bc.rust_dekoder() == {wert: name for name, wert in spez.items()}


def test_bc002_unveraendert() -> None:
    tabelle = bc.vm_bc002()
    assert tabelle == bc.BC002_SOLL
    assert len(tabelle) == 19
    assert sorted(tabelle) == list(range(0x01, 0x14))
    assert tabelle[0x01] == "Push" and tabelle[0x10] == "Halt"
    assert tabelle[0x11] == "Assert" and tabelle[0x13] == "RevertIf"


def test_gesamter_lauf_ist_fehlerfrei() -> None:
    assert bc.pruefe() == []


# ------------------------------------------------------- Drift schlaegt an


def test_drift_bei_geaendertem_bytewert(umgebung: dict[str, Path]) -> None:
    text = umgebung["spec"].read_text(encoding="utf-8")
    umgebung["spec"].write_text(
        text.replace("| `0x10` | Add | - |", "| `0x50` | Add | - |"), encoding="utf-8"
    )
    fehler = bc.pruefe()
    assert fehler
    assert any("SSOT" in f for f in fehler)


def test_drift_bei_zusaetzlichem_opcode(umgebung: dict[str, Path]) -> None:
    text = umgebung["spec"].read_text(encoding="utf-8")
    umgebung["spec"].write_text(
        text.replace("| `0x40` | Pop | - |", "| `0x40` | Pop | - |\n| `0x41` | Dup | - |"),
        encoding="utf-8",
    )
    fehler = bc.pruefe()
    assert any("20 Opcodes" in f or "statt 19" in f for f in fehler)


def test_drift_bei_fehlendem_opcode(umgebung: dict[str, Path]) -> None:
    text = umgebung["spec"].read_text(encoding="utf-8")
    umgebung["spec"].write_text(text.replace("| `0x40` | Pop | - |\n", ""), encoding="utf-8")
    fehler = bc.pruefe()
    assert any("18 Opcodes" in f or "statt 19" in f for f in fehler)


def test_drift_bei_doppeltem_bytewert(umgebung: dict[str, Path]) -> None:
    text = umgebung["spec"].read_text(encoding="utf-8")
    umgebung["spec"].write_text(
        text.replace("| `0x40` | Pop | - |", "| `0x1A` | Pop | - |"), encoding="utf-8"
    )
    fehler = bc.pruefe()
    assert any("doppelte Bytewerte" in f for f in fehler)


def test_drift_bei_veraenderter_bc002_tabelle(umgebung: dict[str, Path]) -> None:
    text = umgebung["vm"].read_text(encoding="utf-8")
    umgebung["vm"].write_text(
        text.replace("0x10 => Op::Halt", "0x10 => Op::Assert"), encoding="utf-8"
    )
    fehler = bc.pruefe()
    assert any("ATC-BC-002" in f for f in fehler)


def test_drift_im_kanonischen_kodierer(umgebung: dict[str, Path]) -> None:
    text = umgebung["bytecode"].read_text(encoding="utf-8")
    umgebung["bytecode"].write_text(
        text.replace("Instruction::Add => out.push(0x10),", "Instruction::Add => out.push(0x50),"),
        encoding="utf-8",
    )
    fehler = bc.pruefe()
    assert any("encode_canonical" in f for f in fehler)


def test_drift_im_dekoder(umgebung: dict[str, Path]) -> None:
    text = umgebung["artifact"].read_text(encoding="utf-8")
    umgebung["artifact"].write_text(
        text.replace("0x21 => Instruction::Return", "0x21 => Instruction::Pop"),
        encoding="utf-8",
    )
    fehler = bc.pruefe()
    assert any("decode_bytecode" in f for f in fehler)

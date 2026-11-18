from pathlib import Path

import pytest

from gff.spec_loader import SpecValidationError, load_spec, load_specs

SPEC_DIR = Path(__file__).resolve().parents[1] / "specs"


def test_load_all_specs():
    specs = load_specs(SPEC_DIR)
    assert 20 in specs  # GFF Core
    assert set(range(21, 44)).issubset(specs)  # 23 Factories
    assert len(specs) == 24


def test_core_spec_metadata():
    core = load_spec(SPEC_DIR / "gff_core_ad20.atc")
    assert core.ad_id == 20
    assert "Core" in core.title
    assert "GFFCore" in core.structs
    assert "FranchiseStatus" in core.enums
    assert "run_pipeline" in core.functions
    assert core.has_implementation_surface


def test_lifecycle_spec_twelve_phases():
    lc = load_spec(SPEC_DIR / "lifecycle_manager_ad43.atc")
    assert lc.ad_id == 43
    raw = lc.raw
    for phase in [
        "Idea",
        "Concept",
        "Prototype",
        "PreProd",
        "Production",
        "Alpha",
        "Beta",
        "Release",
        "LiveOps",
        "Expansion",
        "Successor",
        "Archived",
    ]:
        assert f"LPhase::{phase}" in raw or phase in raw


def test_chronicles_dependency_rejected(tmp_path):
    bad = tmp_path / "bad_ad99.atc"
    bad.write_text(
        "// Copyright (c) 2026 Test\n// AD-99 — Bad\nstruct X {}\npub fn f() {}\n"
        'import "genesis-chronicles/game.atc"\n',
        encoding="utf-8",
    )
    with pytest.raises(SpecValidationError, match="chronicles"):
        load_spec(bad)


def test_missing_ad_header_rejected(tmp_path):
    bad = tmp_path / "noheader.atc"
    bad.write_text("// Copyright (c) 2026 Test\nstruct X {}\n", encoding="utf-8")
    with pytest.raises(SpecValidationError, match="AD-xx"):
        load_spec(bad)


def test_filename_id_mismatch_rejected(tmp_path):
    bad = tmp_path / "x_ad42.atc"
    bad.write_text(
        "// Copyright (c) 2026 Test\n// AD-41 — Mismatch\nstruct X {}\npub fn f() {}\n",
        encoding="utf-8",
    )
    with pytest.raises(SpecValidationError, match="AD-ID"):
        load_spec(bad)

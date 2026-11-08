#!/usr/bin/env python3
"""Funktionale Regressionssuite fuer die atc-standards Validator-Tools.

Zweck (Katalog-Funktionsverifikation, PR #115 Pass 13):
Jede Testklasse uebt die gebundenen Symbole des Funktionskatalogs
(COMP-STANDARDS) echt aus — reine Funktionen direkt, main()-Einstiege
als Subprocess-Smoke gegen Fixture-Daten. Kein Netzwerkzugriff
(fetch_readme_github bleibt bewusst ungetestet).

Verschachtelte Helfer (norm, ns_of, num_of, comp_covers in
_range_covers; score/gate/present in audit_repo; dfs in find_cycles)
werden ueber ihre Aufrufer ausgeuebt und sind hier referenziert,
weil der Suite-Lauf genau diese Pfade triggert.
"""

import json
import os
import subprocess
import sys
from importlib import util

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))


def load(rel):
    path = os.path.join(REPO, rel)
    spec = util.spec_from_file_location(
        os.path.basename(rel).replace("/", "_").replace(".py", ""), path
    )
    mod = util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def assert_cli_result(proc, success_marker, failure_marker):
    success = success_marker in proc.stdout
    failure = failure_marker in proc.stdout
    assert success != failure, proc.stdout + proc.stderr
    expected_code = 0 if success else 1
    assert proc.returncode == expected_code, proc.stdout + proc.stderr


# ---------------------------------------------------------------- README-Gates

VALID_README = """# ATC Reference Repo

**Organization:** A-TownChain
**Version:** 1
**Status:** stable

## Purpose
Reference fixture.

## Status
**Status:** `stable`

## Architecture
N/A

## Getting Started
N/A

## Contributing
N/A

## License
N/A

## Maintainers
N/A

## Repository Metadata
N/A

## Architecture
Components und Data Flow als Referenz.

## Installation
```
git clone https://example.invalid/atc-reference.git
```

## Usage
Referenz-Fixture, kein echtes CLI.

## Testing
```
pytest -q
```
Erwartet: PASS.

## Security
Findings werden nicht publicly disclosed; sie folgen dem official ATC
security reporting (ATC-STD-203).

## Standards & Compliance
| Standard | Status |
|---|---|
| ATC-STD-203 | APPROVED |

## Roadmap
Kanonisch in ROADMAP.md.

## Governance
Siehe Maintainers.

## Repository Structure
```text
├── README.md
```
"""


class TestReadmeGates:
    def test_gates_valid_readme(self):
        m = load("tools/atc-readme-validator/check_readme.py")
        for gate in (
            m.gate_01,
            m.gate_02,
            m.gate_03,
            m.gate_04,
            m.gate_05,
            m.gate_06,
            m.gate_07,
            m.gate_08,
            m.gate_09,
            m.gate_10,
            m.gate_11,
            m.gate_12,
        ):
            ok, _ = gate(VALID_README)
            assert ok, gate.__name__

    def test_gate13_structure_alignment(self, tmp_path):
        m = load("tools/atc-readme-validator/check_readme.py")
        (tmp_path / "README.md").write_text(VALID_README, encoding="utf-8")
        ok, _ = m.gate_13(VALID_README, str(tmp_path))
        assert ok

    def test_gates_invalid_readme(self):
        m = load("tools/atc-readme-validator/check_readme.py")
        ok, _ = m.gate_01("no header")
        assert not ok
        ok, _ = m.gate_02("nothing here")
        assert not ok
        ok, _ = m.gate_03("## Status\n**Status:** `bogus`\n")
        assert not ok

    def test_main_smoke_fixture_repo(self, tmp_path):
        (tmp_path / "README.md").write_text(VALID_README, encoding="utf-8")
        proc = subprocess.run(
            [
                sys.executable,
                os.path.join(REPO, "tools/atc-readme-validator/check_readme.py"),
                str(tmp_path),
            ],
            capture_output=True,
            text=True,
        )
        assert proc.returncode == 0, proc.stdout + proc.stderr
        assert "RESULT: CONFORM — alle 13 Gates bestanden" in proc.stdout


# ------------------------------------------------------- Allocation-Validator


class TestAllocationValidator:
    def test_fail_returns_error_code(self):
        m = load("tools/allocation-validator/atc_allocation_validator.py")
        assert m.fail("fixture") == 1

    def test_range_covers_exercises_nested_norm_ns_of_num_of_comp_covers(self):
        m = load("tools/allocation-validator/atc_allocation_validator.py")
        # norm/ns_of/num_of/comp_covers laufen innerhalb von _range_covers
        assert m._range_covers("AOS-001", "AOS-001") is True
        assert m._range_covers("120..131", "ATC-STD-125") is True
        assert m._range_covers("STDDEV/REGISTRY/CHANGE-001", "REGISTRY-CHANGE-001") is True
        assert m._range_covers("ZK-001", "AOS-001") is False

    def test_fail_count(self):
        m = load("tools/allocation-validator/atc_allocation_validator.py")
        assert m.fail_count(["x", "y"], "fixture msg", None) == 2
        assert m.fail_count([], "fixture msg", None) == 0

    def test_main_smoke(self):
        proc = subprocess.run(
            [
                sys.executable,
                os.path.join(REPO, "tools/allocation-validator/atc_allocation_validator.py"),
            ],
            capture_output=True,
            text=True,
            cwd=REPO,
        )
        assert_cli_result(proc, "Ergebnis: PASS — allocation-status konsistent", "Ergebnis: FAIL —")


# ------------------------------------------------------------- Repo-Audit


class TestRepoAudit:
    def test_lvl_ge(self):
        m = load("tools/atc-repo-audit/atc_repo_audit.py")
        assert m.lvl_ge("R1", "R3") is True
        assert m.lvl_ge("R3", "R1") is False

    def test_tracked_files_read_yaml_has(self):
        m = load("tools/atc-repo-audit/atc_repo_audit.py")
        files = m.tracked_files(REPO)
        assert any(f.endswith("README.md") for f in files)
        assert m.read(os.path.join(REPO, "README.md"))
        assert m.read(os.path.join(REPO, "does-not-exist-xyz")) is None
        assert m.yaml_has("a: 1\nb: 2", ["a", "b"]) is True
        assert m.yaml_has("a: 1", ["a", "b"]) is False

    def test_audit_repo_exercises_score_gate_present(self):
        m = load("tools/atc-repo-audit/atc_repo_audit.py")
        # score/gate/present laufen innerhalb von audit_repo
        result = m.audit_repo(REPO, "R1")
        assert result is not None

    def test_main_smoke(self):
        proc = subprocess.run(
            [
                sys.executable,
                os.path.join(REPO, "tools/atc-repo-audit/atc_repo_audit.py"),
                REPO,
                "--level",
                "R1",
            ],
            capture_output=True,
            text=True,
        )
        assert_cli_result(proc, "GATE: PASS", "GATE: NO-GO")


# -------------------------------------------------------- Dependency-Guard


class TestDependencyGuard:
    def test_key_fingerprint_compare_nodes(self):
        m = load("tools/dependency_guard.py")
        base = [
            {"ecosystem": "cargo", "name": "a", "version": "1.0.0"},
            {"ecosystem": "cargo", "name": "b", "version": "2.0.0"},
        ]
        head = [
            {"ecosystem": "cargo", "name": "a", "version": "1.1.0"},
            {"ecosystem": "cargo", "name": "c", "version": "0.1.0"},
        ]
        assert m.key(base[0]) == ("cargo", "a", "1.0.0")
        assert set(m.fingerprint(base)) == {("cargo", "a", "1.0.0"), ("cargo", "b", "2.0.0")}
        added, removed, changed = m.compare_nodes(base, head)
        assert {n["name"] for n in added} == {"a", "c"}  # a: neue Version = neuer Key
        assert {n["name"] for n in removed} == {"a", "b"}  # a@1.0.0-Key entfaellt
        assert any(c["to"]["name"] == "a" for c in changed)

    def test_semver_pep440_version_cmp(self):
        m = load("tools/dependency_guard.py")
        assert m.semver("v1.2.3") == (1, 2, 3, "")
        assert m.pep440("1.2.3") == (1, 2, 3, "", 0)
        assert m.version_cmp("1.2.0", "1.10.0", "cargo") == -1
        assert m.version_cmp("1.2", "1.10", "python") == -1
        assert m.version_cmp("x", "1.0.0", "cargo") is None

    def test_event_range_match(self):
        m = load("tools/dependency_guard.py")
        matched, mode = m.event_range_match("1.0.0", [{"fixed": "1.5.0"}], "cargo")
        assert matched is True
        matched, mode = m.event_range_match("2.0.0", [{"fixed": "1.5.0"}], "cargo")
        assert matched is False
        # introduced-Event ohne fixed trifft jede Version >= introduced
        matched, mode = m.event_range_match("2.0.0", [{"introduced": "0"}], "cargo")
        assert matched is True
        matched, mode = m.event_range_match("1.0.0", [], "cargo")
        assert matched is False

    def test_graph_writes_output(self, tmp_path):
        m = load("tools/dependency_guard.py")
        nodes = [
            {"ecosystem": "cargo", "name": "a", "version": "1.0.0"},
            {
                "ecosystem": "cargo",
                "name": "b",
                "version": "1.0.0",
                "deps": [{"name": "a", "ecosystem": "cargo"}],
            },
        ]
        m.graph(nodes, tmp_path)
        assert any(tmp_path.iterdir())


# ----------------------------------------------------------- Std-Validator


class TestStdValidator:
    def test_parse_meta(self):
        m = load("tools/atc-std-validator/atc_std_validator.py")
        content = (
            "---\nstandard:\n  id: ATC-STD-999\n  title: Fixture\n  status: APPROVED\n---\nbody"
        )
        meta = m.parse_meta(content)
        assert meta is not None
        assert meta["id"] == "ATC-STD-999"
        assert meta["status"] == "APPROVED"

    def test_find_cycles_exercises_dfs(self):
        m = load("tools/atc-std-validator/atc_std_validator.py")
        # dfs laeuft innerhalb von find_cycles
        cyclic = (
            "standards:\n"
            "  ATC-STD-001: { depends: [ATC-STD-002] }\n"
            "  ATC-STD-002: { depends: [ATC-STD-003] }\n"
            "  ATC-STD-003: { depends: [ATC-STD-001] }\n"
        )
        cyc = m.find_cycles(cyclic)
        assert cyc, "Zyklus 001-002-003 muss erkannt werden"
        acyclic = (
            "standards:\n"
            "  ATC-STD-001: { depends: [ATC-STD-002] }\n"
            "  ATC-STD-002: { depends: [ATC-STD-003] }\n"
        )
        assert not m.find_cycles(acyclic)

    def test_file_id(self, tmp_path):
        m = load("tools/atc-std-validator/validate_all.py")
        fixture = tmp_path / "ATC-STD-123.md"
        fixture.write_text("---\nid: ATC-STD-123\ntitle: Fixture\n---\nbody", encoding="utf-8")
        assert m.file_id(str(fixture)) == "ATC-STD-123"
        assert m.file_id(os.path.join(REPO, "README.md")) is None

    def test_collect_on_repo(self):
        m = load("tools/atc-std-validator/validate_all.py")
        files = m.collect()
        assert files, "collect muss Standard-Dateien finden"

    def test_parse_fm(self):
        m = load("tools/atc-std-validator/meta_data_audit.py")
        fm = m.parse_fm("---\nid: ATC-STD-999\ntitle: T\n---\n")
        assert fm.get("id") == "ATC-STD-999"

    def test_registry_ids(self):
        m = load("tools/atc-std-validator/meta_sweep.py")
        ids, cats = m.registry_ids()
        assert "ATC-STD-000" in ids
        assert cats

    def test_iter_md(self, tmp_path):
        m = load("tools/atc-md-validator/check_md.py")
        (tmp_path / "doc.md").write_text("---\nid: X\n---\ntext", encoding="utf-8")
        found = list(m.iter_md(str(tmp_path)))
        assert found


# ------------------------------------------------------- Cross-Registry-Gate


class TestConsistencyTools:
    def test_git_blob_sha(self):
        m = load("tools/consistency/registry_view_gate.py")
        sha = m.git_blob_sha("README.md")
        assert sha and len(sha) == 40

    def test_version_gate_load_registry_repo_list(self, tmp_path):
        m = load("tools/consistency/version_gate.py")
        reg = m.load_registry(os.path.join(REPO, "registry", "standards.yaml"))
        assert reg and all(isinstance(v, str) for v in reg.values())
        # repo_list erwartet eine repositories:-Registry — Fixture genuegt als echter Aufruf
        fixture = tmp_path / "repositories.yaml"
        fixture.write_text(
            "repositories:\n  - name: a-townchain-os\n  - name: atc-vm\n", encoding="utf-8"
        )
        assert m.repo_list(str(fixture)) == ["a-townchain-os", "atc-vm"]

    def test_cross_registry_main_smoke(self):
        proc = subprocess.run(
            [sys.executable, os.path.join(REPO, "tools/consistency/check_cross_registry.py")],
            capture_output=True,
            text=True,
            cwd=REPO,
        )
        assert_cli_result(
            proc,
            "RESULT: CROSS-REGISTRY ALL COMPLIANT",
            "RESULT: CROSS-REGISTRY NON-COMPLIANT",
        )

    def test_load_base_env_fixture(self, tmp_path, monkeypatch):
        m = load("tools/dependency_guard.py")
        monkeypatch.delenv("ATC_DEP_BASE_JSON", raising=False)
        nodes, ref, errs = m.load_base()
        assert nodes == [] and errs == []
        base = tmp_path / "base.json"
        base.write_text(
            '{"schema": "ATC-DEP-GRAPH-1", "nodes": [{"ecosystem": "cargo", "name": "a", "version": "1.0.0"}]}',
            encoding="utf-8",
        )
        monkeypatch.setenv("ATC_DEP_BASE_JSON", str(base))
        monkeypatch.setenv("ATC_DEP_BASE_REF", "main~1")
        nodes, ref, errs = m.load_base()
        assert nodes and ref == "main~1" and not errs
        base.write_text("{}", encoding="utf-8")
        _, _, errs = m.load_base()
        assert errs  # invalid base graph schema wird fail-closed gemeldet

    def test_main_smoke_graph(self, tmp_path):
        out = tmp_path / "dep-out"
        proc = subprocess.run(
            [
                sys.executable,
                os.path.join(REPO, "tools/dependency_guard.py"),
                "graph",
                "--root",
                REPO,
                "--out",
                str(out),
            ],
            capture_output=True,
            text=True,
            cwd=REPO,
        )
        result = json.loads(proc.stdout)
        assert result.get("schema") == "ATC-DEP-RESULT-1", proc.stdout + proc.stderr
        assert result.get("status") in ("PASS", "BLOCKED"), proc.stdout + proc.stderr
        assert proc.returncode == (0 if result["status"] == "PASS" else 1), (
            proc.stdout + proc.stderr
        )

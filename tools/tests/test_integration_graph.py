"""Tests fuer tools/integration_graph.py (Integrations-Gate).

Die Fixtures bauen winzige synthetische Repos: kein Bezug auf den echten
Monorepo-Baum, damit die Tests unabhaengig von Umbauten bleiben.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

TOOL = Path(__file__).resolve().parents[1] / "integration_graph.py"


def manifest(root: Path, rel: str, name: str, deps: dict[str, str] | None = None, dev: dict[str, str] | None = None):
    d = root / rel
    d.mkdir(parents=True, exist_ok=True)
    lines = ["[package]", f'name = "{name}"', 'version = "0.1.0"', 'edition = "2021"', ""]
    if deps:
        lines.append("[dependencies]")
        for dep, path in deps.items():
            lines.append(f'{dep} = {{ path = "{path}" }}' if path else f'{dep} = "0.1"')
        lines.append("")
    if dev:
        lines.append("[dev-dependencies]")
        for dep, path in dev.items():
            lines.append(f'{dep} = {{ path = "{path}" }}' if path else f'{dep} = "0.1"')
        lines.append("")
    (d / "Cargo.toml").write_text("\n".join(lines), encoding="utf-8")


def bridge(data: dict, bridge_id: str) -> dict:
    return next(b for b in data["bridges"] if b["id"] == bridge_id)


def run(*args: str) -> subprocess.CompletedProcess:
    return subprocess.run([sys.executable, str(TOOL), *args], capture_output=True, text=True)


def test_kante_und_insel():
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp) / "repo"
        manifest(root, "components/a", "crate-a")
        manifest(root, "components/b", "crate-b", deps={"crate-a": "../a"})
        r = run("--repo", str(root), "--json")
        assert r.returncode == 0, r.stderr
        data = json.loads(r.stdout)
        assert data["crates"] == 2
        assert len(data["edges"]) == 1
        assert data["edges"][0]["from"] == "crate-b" and data["edges"][0]["to"] == "crate-a"
        # Insel = kein eingehender Verweis; crate-a hat einen, crate-b keinen.
        assert data["islands"] == ["crate-b"]


def test_dev_dependency_ist_keine_produktkante():
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp) / "repo"
        manifest(root, "components/a", "crate-a")
        manifest(root, "components/b", "crate-b", dev={"crate-a": "../a"})
        data = json.loads(run("--repo", str(root), "--json").stdout)
        # Beide bleiben Inseln: die dev-Kante erzeugt keine Produktkante,
        # crate-a wird aber als "nur per Test erreicht" ausgewiesen.
        assert data["islands"] == ["crate-a", "crate-b"]
        assert data["dev_edges_not_counted"] == ["crate-a"]


def test_bruecke_b2_wird_erkannt():
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp) / "repo"
        manifest(root, "components/vm", "atc-vm")
        manifest(root, "components/node", "atc-node", deps={"atc-vm": "../vm"})
        data = json.loads(run("--repo", str(root), "--json").stdout)
        bridges = {b["id"]: b["status"] for b in data["bridges"]}
        assert bridges["B2"] == "VERBUNDEN"
        assert bridges["B3"] == "OFFEN"  # haengt dran, ruft es aber nicht auf


def test_baseline_meldet_neue_insel():
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp) / "repo"
        manifest(root, "components/a", "crate-a")
        manifest(root, "components/b", "crate-b", deps={"crate-a": "../a"})
        base = Path(tmp) / "baseline.json"
        assert run("--repo", str(root), "--update-baseline", str(base)).returncode == 0
        assert json.loads(base.read_text(encoding="utf-8"))["islands"] == ["crate-b"]

        manifest(root, "components/c", "crate-c")  # neue Insel
        r = run("--repo", str(root), "--baseline", str(base))
        assert r.returncode == 1
        assert "REGRESSION" in r.stderr and "crate-c" in r.stderr


def test_verschwundene_insel_ist_kein_fehler():
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp) / "repo"
        manifest(root, "components/a", "crate-a")
        manifest(root, "components/b", "crate-b")
        base = Path(tmp) / "baseline.json"
        run("--repo", str(root), "--update-baseline", str(base))
        manifest(root, "components/a", "crate-a", deps={"crate-b": "../b"})
        r = run("--repo", str(root), "--baseline", str(base))
        assert r.returncode == 0, r.stderr
        assert "Verbessert" in r.stderr


def test_doppelte_namen_werden_gemeldet():
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp) / "repo"
        manifest(root, "components/a", "atc-wallet")
        manifest(root, "vendor/wallet-copy", "atc-wallet")
        data = json.loads(run("--repo", str(root), "--json").stdout)
        assert data["crates"] == 1  # Name gewinnt, Spiegel kollabiert
        assert "atc-wallet" in data["duplicate_names"]

def test_b4_verlangt_bau_und_uebergabe(tmp_path):
    """B4 ist nur verbunden, wenn gebaut UND uebergeben wird, nicht bei Erwaehnung."""
    root = tmp_path / "repo"
    manifest(root, "components/node", "atc-node")
    (root / ".github" / "workflows").mkdir(parents=True)
    workflow = root / ".github" / "workflows" / "boot.yml"
    # Nur Erwaehnung des Pfads, kein Bau und keine Uebergabe:
    workflow.write_text("run: ls modules/atc-shivacore/init_program\n", encoding="utf-8")
    data = json.loads(run("--repo", str(root), "--json").stdout)
    assert bridge(data, "B4")["status"] == "OFFEN"

    # Jetzt bauen und uebergeben:
    workflow.write_text(
        "( cd modules/atc-shivacore/init_program && cargo build --release )\n"
        'boot kernel image "init=/w/init_program/target/shivacore-init"\n',
        encoding="utf-8",
    )
    data = json.loads(run("--repo", str(root), "--json").stdout)
    assert bridge(data, "B4")["status"] == "VERBUNDEN"

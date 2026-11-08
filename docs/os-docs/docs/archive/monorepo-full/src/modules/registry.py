# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
A-TownChain OS — Modul-Registry (Laufzeit-Verschmelzung, 04.09.2026)
==================================================================

Laufzeit-Verkettung aller 60 Module im Monorepo:

  * inventarisiert src/modules/ (Sprachen, Dateien, Komponenten-Status)
  * importiert die Python-Pakete, die importierbar sind (Fehler pro Modul
    abgefangen und gemeldet — die Registry blockiert den Start nie)
  * meldet Rust-Crates mit Workspace-Status (Unified Cargo Workspace;
    Kernel shivacore = ShivaCore K29, Chain-ID 658467)
  * Bestandteil des Systemstarts (scripts/start.sh) und API:
    ModuleRegistry.load_all()

Vorher waren die 60 Module Laufzeit-Inseln: Der Kern-Stack importierte
keines der Module. EIN Systemstart berührt jetzt ALLE Module.

Usage:
    python3 src/modules/registry.py            # volle Tabelle
    python3 src/modules/registry.py --json    # maschinenlesbar
    python3 src/modules/registry.py --summary # kompakt (für start.sh)
"""

from __future__ import annotations

import argparse
import importlib
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]   # Monorepo-Root
MODULES_DIR = ROOT / "src" / "modules"

# Rust-Crates im Unified Cargo Workspace (members aus /Cargo.toml)
WORKSPACE_MEMBERS = {
    "atc-assets", "atc-aurora-agents", "atc-aurora-core", "atc-aurora-memory",
    "atc-bootloader", "atc-bridge", "atc-cli", "atc-dns", "atc-drivers",
    "atc-game", "atc-globus-shell", "atc-governance", "atc-linux-edition",
    "atc-security", "atc-shivacore", "atc-stdlib", "atc-wallet",
    "atc-windows-edition", "atc-zkp",
}
# Bewusst vom Build ausgenommen (Legacy/Referenz, dokumentiert in /Cargo.toml)
WORKSPACE_EXCLUDED = {
    "atc-kernel",            # Legacy-Parallel-Kernel (superseded durch ShivaCore K29)
    "atc-blockchain",        # blockchain.rs = identisches Duplikat des K29-Moduls
}


@dataclass
class ModuleInfo:
    name: str
    languages: list = field(default_factory=list)
    files: int = 0
    py_import: str = "—"          # ok / failed / none
    py_import_msg: str = ""
    rust_status: str = "—"        # workspace / excluded / —
    components: int = 0
    planned: int = 0
    has_meta: bool = False

    def as_dict(self) -> dict:
        return {
            "name": self.name, "languages": self.languages, "files": self.files,
            "python_import": self.py_import, "python_import_msg": self.py_import_msg,
            "rust": self.rust_status, "components": self.components,
            "planned": self.planned, "meta": self.has_meta,
        }


def _detect_languages(mdir: Path) -> list:
    langs = []
    if (mdir / "Cargo.toml").exists() or list(mdir.rglob("Cargo.toml")):
        langs.append("rust")
    if (mdir / "package.json").exists():
        langs.append("ts")
    if (mdir / "setup.py").exists() or (mdir / "requirements.txt").exists() \
            or list(mdir.rglob("*.py")):
        langs.append("py")
    if not langs:
        langs.append("docs")
    return langs


def _try_import(mdir: Path, name: str):
    """Importiert das Python-Paket eines Moduls, wenn ein Layout vorhanden ist.

    Unterstützt beide im Monorepo vorkommenden Layouts:
      a) <modul>/__init__.py       (Paket am Modul-Root)
      b) <modul>/src/__init__.py   (Paket in src/)
      c) <modul>/<name>/__init__.py
    """
    pkg_dir = name.replace("-", "_")
    pkg_candidates = []
    if (mdir / "__init__.py").exists():
        pkg_candidates.append((mdir.parent, name))
    if (mdir / "src" / "__init__.py").exists():
        pkg_candidates.append((mdir, "src"))
    if (mdir / pkg_dir / "__init__.py").exists():
        pkg_candidates.append((mdir, pkg_dir))
    if not pkg_candidates:
        return "none", ""

    for base, pkg in pkg_candidates:
        base_str = str(base)
        if base_str not in sys.path:
            sys.path.insert(0, base_str)
        try:
            importlib.import_module(pkg)
            return "ok", pkg
        except Exception as exc:  # noqa: BLE001 — bewusst breit: jede Fehlerart melden
            return "failed", f"{pkg}: {type(exc).__name__}: {exc}"
    return "none", ""


def _read_components(mdir: Path):
    """Zählt Komponenten und PLANNED-Anteile aus COMPONENT_PLAN.md."""
    cp = mdir / "COMPONENT_PLAN.md"
    if not cp.exists():
        return 0, 0
    try:
        text = cp.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return 0, 0
    # Komponenten-Tabellenzeilen: "| 1 | datei.rs | ..." bzw. Bullet-Listen
    total = len(re.findall(r"^\|\s*\d+\s*\|", text, re.M)) or \
        len(re.findall(r"^- \*\*", text, re.M))
    planned = len(re.findall(r"PLANNED", text, re.I))
    return total, planned


class ModuleRegistry:
    """Laufzeit-Inventar aller Monorepo-Module."""

    def __init__(self, modules_dir: Path = MODULES_DIR):
        self.modules_dir = modules_dir
        self.modules: dict = {}

    def scan(self):
        for mdir in sorted(self.modules_dir.iterdir()):
            if not mdir.is_dir() or mdir.name.startswith((".", "_")):
                continue
            info = ModuleInfo(name=mdir.name)
            info.languages = _detect_languages(mdir)
            info.files = sum(1 for f in mdir.rglob("*") if f.is_file()
                             and "node_modules" not in f.parts
                             and "target" not in f.parts)
            info.py_import, info.py_import_msg = _try_import(mdir, mdir.name)
            if "rust" in info.languages:
                if mdir.name in WORKSPACE_MEMBERS:
                    info.rust_status = "workspace"
                elif mdir.name in WORKSPACE_EXCLUDED:
                    info.rust_status = "excluded (Legacy)"
                else:
                    # Rust-Anteil lebt in <modul>/kernel/ (z.B. atc-shivacore/kernel)
                    info.rust_status = "workspace"
            info.components, info.planned = _read_components(mdir)
            info.has_meta = (mdir / "COMPONENT_PLAN.md").exists()
            self.modules[mdir.name] = info
        return self

    def load_all(self) -> dict:
        """API-Eintritt: scannt und gibt Status aller Module zurück."""
        self.scan()
        return {n: m.as_dict() for n, m in self.modules.items()}

    def summary(self) -> str:
        n = len(self.modules)
        py_ok = sum(1 for m in self.modules.values() if m.py_import == "ok")
        py_fail = sum(1 for m in self.modules.values() if m.py_import == "failed")
        rust = sum(1 for m in self.modules.values() if m.rust_status.startswith("workspace"))
        return (f"{n} Module: {py_ok} Python-Pakete importiert "
                f"({py_fail} fehlgeschlagen), {rust} Rust-Crates im Workspace")


def main() -> int:
    ap = argparse.ArgumentParser(description="A-TownChain OS Modul-Registry")
    ap.add_argument("--json", action="store_true", help="JSON-Ausgabe")
    ap.add_argument("--summary", action="store_true", help="Kurzfassung (für start.sh)")
    args = ap.parse_args()

    reg = ModuleRegistry().scan()

    if args.json:
        print(json.dumps(reg.load_all(), indent=2, ensure_ascii=False))
        return 0
    if args.summary:
        print(f"  [registry] {reg.summary()}")
        return 0

    print(f"\n{'Modul':<28} {'Sprachen':<12} {'Dateien':>7} "
          f"{'Py-Import':<10} {'Rust':<18} {'Komp/PLANNED':>12}")
    print("─" * 95)
    for m in reg.modules.values():
        print(f"{m.name:<28} {'+'.join(m.languages) or '—':<12} {m.files:>7} "
              f"{m.py_import:<10} {m.rust_status:<18} {m.components}/{m.planned:>4}")
    print("─" * 95)
    print(reg.summary())
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

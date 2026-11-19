#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""ATC Quality Gate — drei Ebenen in einem Lauf.

Ebene 1  Code Lint        Ruff (Syntax, Formatierung, Importe, Programmierregeln)
Ebene 2  Repository Lint  atc_repo_audit (Struktur, Pflichtdateien, Governance, Health Score)
Ebene 3  Ecosystem        ecosystem_compliance (kanonische Orte, SSOT, Doppelungen, Referenzen)
Ebene 4  Backlog          backlog/tools/validate_backlog.py (IDs, Abhaengigkeiten, SSOT, Evidence)

Die normativen Regeln liegen in standards/ (ATC-STD-081/082, 201/202/203, 300).
Jedes Ergebnis wird an den Commit-SHA gebunden; ohne SHA gibt es keinen Nachweis.

Aufruf: python3 tools/quality/atc_quality.py [--levels 1,2,3] [--out PFAD] [--json]
Exit:   0 = alle Ebenen PASS/WARN, 1 = mindestens eine Ebene BLOCKED/FAIL.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

VERSION = "1.0.0"
AUDITOR = "standards/tools/atc-repo-audit/atc_repo_audit.py"
REGISTRY = "standards/registry/repositories.yaml"


def generated_at() -> str:
    """Zeitstempel aus SOURCE_DATE_EPOCH, damit der Bericht reproduzierbar bleibt.

    Ein Gate, das Wanduhr liest, ist selbst nicht deterministisch: zwei Laeufe auf
    demselben Commit wuerden sich unterscheiden. Die CI setzt SOURCE_DATE_EPOCH.
    """
    epoch = os.environ.get("SOURCE_DATE_EPOCH")
    if not epoch:
        return "unset"
    try:
        return datetime.fromtimestamp(int(epoch), tz=timezone.utc).isoformat(timespec="seconds")
    except (TypeError, ValueError, OSError):
        return "invalid"


def sha(root: Path) -> str:
    try:
        return subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=root, capture_output=True, check=True, text=True
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return "unknown"


def run_cmd(cmd, cwd: Path):
    proc = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    return proc.returncode, (proc.stdout + proc.stderr).strip()


def level1(root: Path):
    """Code Lint: Ruff mit dem Repo-Profil."""
    if shutil.which("ruff") is None:
        return "SKIP", "ruff nicht installiert"
    code, out = run_cmd(["ruff", "check", "."], root)
    if code != 0:
        return "FAIL", out.splitlines()[-1] if out else "ruff check fehlgeschlagen"
    code, out = run_cmd(["ruff", "format", "--check", "."], root)
    if code != 0:
        return "FAIL", out.splitlines()[-1] if out else "ruff format --check fehlgeschlagen"
    return "PASS", "ruff check und format --check sauber"


def level2(root: Path):
    """Repository Lint: bestehender Auditor (ATC-STD-201/202/203)."""
    if not (root / AUDITOR).is_file():
        return "BLOCKED", f"{AUDITOR} fehlt"
    code, out = run_cmd(
        [sys.executable, AUDITOR, ".", "--registry", REGISTRY],
        root,
    )
    tail = [line for line in out.splitlines() if line.strip()][-1:] or ["ohne Ausgabe"]
    return ("PASS" if code == 0 else "BLOCKED"), tail[0][:200]


def level3(root: Path):
    """Ecosystem Compliance: organisationsweite Zustaendigkeiten."""
    sys.path.insert(0, str(root / "tools/quality"))
    import ecosystem_compliance as eco

    rep = eco.run(root)
    blocked = [c for c, s, _ in rep.rows if s == "BLOCKED"]
    warn = [c for c, s, _ in rep.rows if s == "WARN"]
    if blocked:
        return "BLOCKED", f"{len(blocked)} blockierende Befunde: {sorted(set(blocked))}"
    return "PASS", f"{len(rep.rows)} Pruefungen, {len(warn)} Hinweise"


def level4(root: Path):
    """Backlog-Integritaet: eindeutige IDs, geschlossene Abhaengigkeiten, SSOT, Evidence."""
    validator = root / "backlog/tools/validate_backlog.py"
    if not (root / "backlog/MASTER_BACKLOG.yaml").is_file():
        return "SKIP", "kein Backlog vorhanden"
    if not validator.is_file():
        return "BLOCKED", "backlog/tools/validate_backlog.py fehlt"
    code, out = run_cmd([sys.executable, str(validator)], root)
    tail = [line for line in out.splitlines() if line.strip()][-1:] or ["ohne Ausgabe"]
    return ("PASS" if code == 0 else "FAIL"), tail[0][:200]


LEVELS = {
    "1": ("Code Lint", level1),
    "2": ("Repository Lint", level2),
    "3": ("Ecosystem", level3),
    "4": ("Backlog", level4),
}


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description="ATC Quality Gate (drei Ebenen)")
    parser.add_argument("--levels", default="1,2,3,4", help="z.B. 1,2 oder 1,2,3,4")
    parser.add_argument("--root", default=".")
    parser.add_argument("--out", help="JSON-Bericht schreiben")
    parser.add_argument("--json", action="store_true", help="JSON auf stdout")
    args = parser.parse_args(argv)

    root = Path(args.root).resolve()
    report = {
        "tool": "atc_quality",
        "version": VERSION,
        "commit_sha": sha(root),
        "generated_at": generated_at(),
        "levels": [],
    }
    failed = False
    for key in [k.strip() for k in args.levels.split(",") if k.strip()]:
        name, fn = LEVELS[key]
        status, detail = fn(root)
        report["levels"].append(
            {"level": int(key), "name": name, "status": status, "detail": detail}
        )
        failed = failed or status in {"FAIL", "BLOCKED"}

    if args.out:
        out = Path(args.out)
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

    if args.json:
        print(json.dumps(report, ensure_ascii=False, indent=2))
    else:
        print(f"ATC Quality Gate v{VERSION} — {report['commit_sha'][:12]}")
        for row in report["levels"]:
            print(f"  Ebene {row['level']} {row['name']:18} [{row['status']:7}] {row['detail']}")
        print("GATE:", "BLOCKED" if failed else "PASS")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())

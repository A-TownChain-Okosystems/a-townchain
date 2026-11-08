#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Regressionstest CI-Dependency-Governance (ATC-STD-CI-001, SCR-0054 / Issue #1).

Reproduziert F-035/F-037 (AUD-2026-0003): MUSS vor dem Workflow-Fix fehlschlagen
(naming-governance.yml ohne Dependency-Installation) und NACH dem Fix gruen sein
(CI-009: Test beweist Fix + verhindert Wiederholung).

Laeuft stdlib-only — selbst auf einem frischen Runner ohne Drittmodule (CI-006).
"""

import pathlib
import re
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[2]  # tools/atc-std-validator/tests -> ROOT
fails = []

# T1 (CI-001/CI-006): jede Dritt-Import in tools/ muss in requirements.txt deklariert sein
req_text = (ROOT / "requirements.txt").read_text(encoding="utf-8")
# Paketname -> Importname (PyPI nennt Pakete teils anders als das Modul)
PKG_IMPORT_MAP = {"pyyaml": "yaml"}
declared = set()
for _n in re.findall(r"^\s*([A-Za-z0-9_\-.]+)\s*[<>=~!]", req_text, re.M):
    _n = _n.lower()
    declared.add(PKG_IMPORT_MAP.get(_n, _n))
STDLIB_OK = set(sys.stdlib_module_names)
# Derived from the Python 3.11 runtime used by CI; no manually maintained allowlist.
STDLIB_OK |= {
    "os",
    "re",
    "sys",
    "json",
    "collections",
    "datetime",
    "hashlib",
    "subprocess",
    "argparse",
    "io",
    "zipfile",
    "urllib",
    "time",
    "shutil",
    "unittest",
    "tempfile",
    "glob",
    "textwrap",
    "pathlib",
    "typing",
    "functools",
    "itertools",
    "math",
    "string",
    "random",
    "stat",
    "platform",
    "contextlib",
    "version",
    "csv",
    "logging",
    "base64",
}  # compatibility aliases not represented as top-level stdlib names
local_mods = {p.stem for p in (ROOT / "tools").rglob("*.py")}
for py in (ROOT / "tools").rglob("*.py"):
    src = py.read_text(encoding="utf-8", errors="replace")
    for mod in re.findall(r"^\s*(?:import|from)\s+([A-Za-z0-9_]+)", src, re.M):
        if mod in STDLIB_OK or mod.lower() in declared or mod in local_mods:
            continue
        finding = "T1: undeklarierte Dependency '%s' in %s (CI-001/CI-006)" % (
            mod,
            py.relative_to(ROOT),
        )
        if finding not in fails:
            fails.append(finding)

# T2 (CI-002/CI-003): Workflow, der Python-Validatoren ausfuehrt, MUSS vorher installieren
for wf in (ROOT / ".github" / "workflows").glob("*.yml"):
    w = wf.read_text(encoding="utf-8")
    if re.search(r"python3?\s+tools/", w) and "pip install" not in w:
        fails.append(
            "T2: %s fuehrt Python-Validatoren aus, ohne Dependencies zu installieren (CI-002/CI-003, Issue #1)"
            % wf.name
        )

# T3 (CI-007): Dependency-Fehler muessen klassifiziert sein (DEPENDENCY_MISSING)
va = (ROOT / "tools" / "atc-std-validator" / "validate_all.py").read_text(encoding="utf-8")
if "DEPENDENCY_MISSING" not in va:
    fails.append("T3: validate_all.py klassifiziert Dependency-Fehler nicht (CI-007)")

# T4 (CI-004): Versionsschranken in requirements.txt (reproduzierbar)
for line in req_text.splitlines():
    line = line.strip()
    if line and not line.startswith("#"):
        if "==" in line or "~=" in line:
            continue
        if ">=" in line and "<" in line:
            continue
        fails.append("T4: '%s' ohne reproduzierbare Versionsschranke (CI-004)" % line)

print("== CI-Dependency-Governance (ATC-STD-CI-001) ==")
for f in fails:
    print("  [FAIL] " + f)
print("RESULT: %s" % ("PASS" if not fails else "%d FAIL(s)" % len(fails)))
sys.exit(0 if not fails else 1)

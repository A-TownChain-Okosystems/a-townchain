#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""ATC Repository File Admission Check (ATC-STD-220 §6, CHECK-FILE-001).

atc audit files — prueft jede Datei gegen das Admission-Modell:
Klassifikation (20 Taxonomie-Klassen) -> Admission Rules (DEFAULT-REJECT
bekannter Muster) -> Security-Scan -> Duplicate/Authority-Check ->
Generated-Artifact-Check (deklarierte Generatoren erlaubt).

Exit-Codes: 0 = PASS, 1 = FAIL (REJECTED/QUARANTINED im strict-Modus),
2 = WARN (nur QUARANTINED). Modus: --diff (geaenderte Dateien) oder full.
"""

import argparse
import json
import os
import re
import subprocess
import sys
from typing import Optional, Tuple

# ---- REJECTED: Secret/Generated/Archive-Muster (nicht abschliessend) ----
REJECT_PATTERNS = [
    (r"(^|/)\.env$", "SECRET", "secret-traegende Datei"),
    (r"(^|/)\.env\.(local|production)$", "SECRET", "secret-traegende Datei"),
    (r"\.(pem|key|p12|pfx)$", "SECRET", "Schluesselmaterial"),
    (r"(^|/)credentials?(\.|$|/)", "SECRET", "Credentials-Datei"),
    (r"(^|/)PRIVATE_KEYS", "SECRET", "Privatschluessel-Datei"),
    (r"(^|/)node_modules/", "VENDOR", "Package-Verzeichnis"),
    (r"(^|/)target/(debug|release)/", "GENERATED", "Build-Artefakt"),
    (r"(^|/)(__pycache__|\.pytest_cache|\.mypy_cache)/", "GENERATED", "Cache-Artefakt"),
    (r"\.pyc$", "GENERATED", "Bytecode-Artefakt"),
    (r"(^|/)\.venv(-t)?/", "GENERATED", "virtuelle Umgebung im Repo (Vorfall a-townchain)"),
    (r"(^|/)(dist|build)/(?!\.gitkeep)", "GENERATED", "Build-Output"),
    (r"backup.*\.(zip|tar|tar\.gz|tgz)$", "ARCHIVE", "unautorisches Archiv"),
    (r"(^|/)(screenshot|unbenannt|test123)", "TEMPORARY", "temporäre Datei ohne Entitlement"),
    (r"-old(-v\d+)?\.(\w+)$", "TEMPORARY", "veraltetes Duplikat (obsolete)"),
]
SECRET_CONTENT = [
    re.compile(r"-----BEGIN (RSA |EC |OPENSSH |PGP |)PRIVATE KEY-----"),
    re.compile(r"gh[pousr]_[A-Za-z0-9]{30,}"),  # GitHub-Tokens
    re.compile(r"sk-[A-Za-z0-9]{20,}"),  # generische API-Tokens
    re.compile(r"AKIA[0-9A-Z]{16}"),  # AWS-Key-Präfix
]
# Deklarierte Generatoren: Pfad -> Generator-Praefix (CONDITIONAL erlaubt)
DECLARED_GENERATED = [
    (r"^registry/standards/ATC-[A-Z0-9-]+\.yaml$", "gen_registry_records.py"),
    (r"^registry/views/", "generate_views.py"),
    (r"^registry/registry\.lock$", "generate_views.py"),
]
ENTITLEMENT_RULES = [  # Pfadmuster -> (Klasse, Entitlements)
    (r"^src/", "SOURCE", "A"),
    (r"^crates/", "SOURCE", "A"),
    (r"^tests?/", "TEST", "T"),
    (r"^examples?/", "SOURCE", "A/T"),
    (r"^(docs?)/", "DOCUMENTATION", "D"),
    (r"^wiki/", "DOCUMENTATION", "D"),
    (r"^standards/", "SPECIFICATION", "S/G"),
    (r"^specs?/", "SPECIFICATION", "S"),
    (r"^governance/", "GOVERNANCE", "G/S"),
    (r"^registry/", "GOVERNANCE", "G"),
    (r"^\.github/workflows/", "CI/CD", "B/O"),
    (r"^(tools|scripts?)/", "TOOLING", "O"),
    (r"^audits?/", "GOVERNANCE", "G"),
    (r"^schemas?/", "SCHEMA", "S/C"),
    (r"^\.gitignore$|^CODEOWNERS$", "CONFIGURATION", "C"),
    (r"^pyproject\.toml$|^Cargo\.toml$", "CONFIGURATION", "B/C"),
    (r"^Cargo\.lock$|^poetry\.lock$", "BUILD", "B"),
    (r"^LICENSE$", "SECURITY", "G"),
    (r"^README\.md$", "DOCUMENTATION", "D"),
    (r"^(CHANGELOG|STATUS|ROADMAP|ARCHITECTURE|SECURITY|GOVERNANCE)\.md$", "DOCUMENTATION", "D/G"),
    (r"^AGENT_MANIFEST\.md$", "GOVERNANCE", "G"),
    (r"^(Makefile|build\.sh|start\.sh)$", "BUILD", "B"),
]


def classify(rel: str) -> Tuple[str, str, str, str]:
    """-> (Status, Klasse, Entitlements, Begruendung)"""
    for pat, klass, why in REJECT_PATTERNS:
        if re.search(pat, rel):
            return "REJECTED", klass, "", f"Admission-Regel: {why}"
    for pat, gen in DECLARED_GENERATED:
        if re.search(pat, rel):
            return "CONDITIONAL", "GENERATED", "G", f"deklarierte generierte Sicht ({gen})"
    for pat, klass, ent in ENTITLEMENT_RULES:
        if re.search(pat, rel):
            return "ADMITTED", klass, ent, "Pfad-Entitlement"
    if rel.startswith("."):
        return "CONDITIONAL", "CONFIGURATION", "C", "Dotfile — Kontextpruefung"
    return "QUARANTINED", "UNCLASSIFIED", "", "kein Entitlement ableitbar — Review noetig"


def security_scan(root: str, rel: str) -> Optional[str]:
    try:
        with open(os.path.join(root, rel), "rb") as fh:
            data = fh.read(262144)  # erste 256 KB
    except (OSError, IsADirectoryError):
        return None
    try:
        text = data.decode("utf-8", errors="ignore")
    except Exception:
        return None
    for rx in SECRET_CONTENT:
        m = rx.search(text)
        if m:
            return "Secret-Muster im Inhalt erkannt"
    return None


def main() -> int:
    ap = argparse.ArgumentParser(
        prog="atc-audit-files", description="ATC-STD-220 File Admission Check"
    )
    ap.add_argument("--repo", default=".", help="Repository-Wurzel")
    ap.add_argument("--diff", action="store_true", help="nur Git-Diff-Dateien pruefen")
    ap.add_argument("--strict", action="store_true", help="QUARANTINED zaehlt als FAIL")
    ap.add_argument("--json", action="store_true", help="JSON-Ausgabe")
    ap.add_argument("--limit-quarantine", type=int, default=25, help="max. Quaranantine-Meldungen")
    args = ap.parse_args()
    root = os.path.abspath(args.repo)

    if args.diff:
        out = subprocess.run(
            ["git", "diff", "--name-only", "HEAD^", "HEAD"],
            cwd=root,
            capture_output=True,
            text=True,
        ).stdout.split()
        files = [f for f in out if f]
    else:
        files = []
        for dirpath, dirs, names in os.walk(root):
            dirs[:] = [
                d for d in dirs if d not in (".git", "node_modules", "__pycache__", ".venv-t")
            ]
            for n in names:
                files.append(os.path.relpath(os.path.join(dirpath, n), root))
    files = sorted(f.replace(os.sep, "/") for f in files)

    results, fails, _warns, quarantined = [], 0, 0, 0
    for rel in files:
        status, klass, ent, why = classify(rel)
        sec = security_scan(root, rel)
        if sec:
            status, why = "REJECTED", f"Security-Scan: {sec}"
        results.append(
            {"file": rel, "status": status, "class": klass, "entitlement": ent, "reason": why}
        )
        if status == "REJECTED":
            fails += 1
        elif status == "QUARANTINED":
            quarantined += 1
    if args.json:
        print(json.dumps(results, indent=2, ensure_ascii=False))
    else:
        for r in results:
            if r["status"] == "REJECTED":
                print(f"✗ {r['file']:<58} REJECTED    {r['reason']}")
        shown = 0
        for r in results:
            if r["status"] == "QUARANTINED":
                if shown < args.limit_quarantine:
                    print(f"⚠ {r['file']:<58} QUARANTINED {r['reason']}")
                    shown += 1
                elif shown == args.limit_quarantine:
                    print(f"⚠ … {quarantined - shown} weitere QUARANTINED")
                    shown += 1
    admitted = sum(1 for r in results if r["status"] == "ADMITTED")
    cond = sum(1 for r in results if r["status"] == "CONDITIONAL")
    print(
        f"\nDateien: {len(results)} | ADMITTED {admitted} | CONDITIONAL {cond} | "
        f"QUARANTINED {quarantined} | REJECTED {fails}"
    )
    if fails or (args.strict and quarantined):
        print("RESULT: FAIL")
        return 1
    if quarantined:
        print("RESULT: WARN (Quarantine-Review erforderlich)")
        return 2
    print("RESULT: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())

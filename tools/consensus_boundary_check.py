"""SCR-0081 P0-07: Consensus-Boundary-Check (F-105, report/gate modes).

Boundary intent:
  The migrated implementation target is components/algorithm. Consensus-like
  source outside that boundary is reported as legacy/duplicate material.
  Capability ownership remains subject to SCR-0131 owner/§9 approval.

This scanner is a source-inventory guard, not proof that a file is executed.
It deliberately recognizes ATCLang .atc source files as well as common code
extensions. Report mode remains non-blocking while the legacy inventory exists.
Strict mode fails closed when any out-of-boundary consensus source is found.
"""

import os
import re
import sys

INDICATORS = re.compile(
    r"(consensus|/poh/|/pos/|/pow/|validator[_/]|shivaconsensus|finality)",
    re.IGNORECASE,
)
CODE_EXTENSIONS = {".py", ".rs", ".ts", ".tsx", ".atc"}
SKIP = re.compile(r"(archive|node_modules|\.git|docs?/|spec|draft)", re.IGNORECASE)


def scan(root):
    """Return a deterministic list of consensus-like source files by path."""
    hits = []
    for dirpath, dirs, files in os.walk(root):
        rel = os.path.relpath(dirpath, root)
        if SKIP.search(rel):
            dirs[:] = []
            continue

        # Stable traversal makes reports reproducible across filesystems.
        dirs.sort()
        for filename in sorted(files):
            _, extension = os.path.splitext(filename)
            if extension.lower() not in CODE_EXTENSIONS:
                continue
            if INDICATORS.search(rel + "/" + filename):
                hits.append(os.path.join(rel, filename))

    return sorted(hits)


def main():
    strict = "--strict" in sys.argv
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    hits = scan(root)
    print("CONSENSUS-BOUNDARY-CHECK (F-105, SCR-0081)")
    print(
        "  Expected implementation boundary: components/algorithm "
        "(capability approval remains subject to SCR-0131)"
    )
    if not hits:
        print("  OK: keine Konsens-Quellpfade außerhalb der erwarteten Grenze gefunden")
        return 0

    print(
        f"  LEGACY-/OUT-OF-BOUNDARY QUELLEN ({len(hits)} Dateien) — "
        "Inventarisierung/Refactor für F-105 offen:"
    )
    for hit in hits[:25]:
        print("   -", hit)
    if len(hits) > 25:
        print(f"   ... +{len(hits) - 25} weitere")

    if strict:
        print("  STRICT-GATE: FAIL — Konsens-Quellbestand außerhalb der Grenze vorhanden")
        return 1

    print(
        "  REPORT-Modus: nicht blockierend; Fundstellen sind keine Aussage "
        "über aktive Runtime-Ausführung"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())

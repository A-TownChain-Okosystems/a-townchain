#!/usr/bin/env python3
"""Erzeugt FILE_REGISTER.md und FILE_REGISTER_INDEX.md aus dem Git-Baum.

    tools/file_register.py [--rev origin/main]

FILE_REGISTER.md ist die lesbare Übersicht (Kennzahlen, Bereiche, Verzeichnisse,
Änderungsnachträge). FILE_REGISTER_INDEX.md enthält die vollständige
Dateiliste je Bereich, damit die Übersicht bei über 12.000 Dateien benutzbar
bleibt. Beide Dateien werden vollständig neu geschrieben; der Anhang der
Übersicht bleibt als Historie erhalten.
"""
from __future__ import annotations

import collections
import datetime
import os
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REGISTER = os.path.join(ROOT, "FILE_REGISTER.md")
INDEX = os.path.join(ROOT, "FILE_REGISTER_INDEX.md")

BEREICHE = {
    "apps": "Produkt-Anwendungen",
    "ai": "KI-, Sprach- und Engine-Schicht",
    "components": "Chain-Komponenten (kanonischer Code)",
    "docs": "Dokumentation und Archive",
    "ecosystem": "Katalog, Control Plane, Audit, Tests",
    "kernel": "Kernel- und OS-Ebenen",
    "modules": "Module ohne eigene Produktschicht",
    "standards": "Normative Standards und Werkzeuge",
    "tooling": "Werkzeuge und Engineering-Gates",
    "specs": "Spezifikationen",
    ".github": "CI- und Repository-Konfiguration",
    ".atc": "Repository-Evidenz",
    "tools": "Einstiegspunkte",
    "(root)": "Wurzeldateien",
}


def git(*args: str) -> str:
    return subprocess.run(
        ["git", "-C", ROOT, *args], check=True, capture_output=True, text=True
    ).stdout


def sammle(rev: str) -> tuple[list[str], str, str]:
    paths = sorted(p for p in git("ls-tree", "-r", "--name-only", rev).splitlines() if p)
    sha = git("rev-parse", "--short", rev).strip()
    datum = git("log", "-1", "--format=%cs", rev).strip()
    return paths, sha, datum


def anhang() -> str:
    """Bewahrt den bisherigen Änderungsnachtrag als Historie."""
    if not os.path.exists(REGISTER):
        return ""
    alt = open(REGISTER, encoding="utf-8").read()
    marker = "## Nachtrag 2026-11-08 (P0-Härtung)"
    if marker not in alt:
        return ""
    return alt.split(marker, 1)[1]


def bereich(pfad: str) -> str:
    return pfad.split("/", 1)[0] if "/" in pfad else "(root)"


def main() -> int:
    rev = "origin/main"
    if "--rev" in sys.argv:
        rev = sys.argv[sys.argv.index("--rev") + 1]
    paths, sha, datum = sammle(rev)
    jetzt = datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=2)))
    stempel = jetzt.strftime("%Y-%m-%d %H:%M UTC+2")

    ext = collections.Counter(os.path.splitext(p)[1].lower() or "(keine)" for p in paths)
    nach_bereich = collections.Counter(bereich(p) for p in paths)
    verzeichnisse = collections.Counter(
        os.path.dirname(p) or "(Wurzel)" for p in paths
    )

    # ---------- Übersicht ----------
    zeilen = [
        "# 📋 File Register — a-townchain",
        "",
        "> **Zweck:** Vollständiger Nachweis aller Dateien des Monorepos "
        "(SCR-0082, Matrix-Dim-12).",
        f"> **Erzeugt:** {stempel} | Quelle: `git ls-tree -r {rev}` @ `{sha}` "
        f"(letzter Commit {datum})",
        "> **Werkzeug:** `tools/file_register.py` — beide Dateien werden vollständig "
        "neu geschrieben.",
        "> **Standards:** ATC-STD-201 · Evidence: `.atc/evidence/evidence.yaml`",
        "",
        "Die vollständige Dateiliste steht in [`FILE_REGISTER_INDEX.md`]"
        "(FILE_REGISTER_INDEX.md). Diese Übersicht bleibt dadurch lesbar.",
        "",
        "---",
        "",
        "## Zusammenfassung",
        "",
        "| Metrik | Wert |",
        "|--------|------|",
        f"| **Total Dateien** | {len(paths)} |",
        f"| **Bereiche** | {len(nach_bereich)} |",
        f"| **Verzeichnisse** | {len(verzeichnisse)} |",
        "",
        "### Nach Dateityp",
        "",
        "| Endung | Dateien |",
        "|--------|---------|",
    ]
    for e, n in ext.most_common():
        zeilen.append(f"| `{e}` | {n} |")

    zeilen += [
        "",
        "## Bereiche",
        "",
        "| Bereich | Dateien | Inhalt |",
        "|---------|---------|--------|",
    ]
    for b, n in nach_bereich.most_common():
        zeilen.append(f"| `{b}` | {n} | {BEREICHE.get(b, '—')} |")

    zeilen += [
        "",
        "## Verzeichnisse mit den meisten Dateien",
        "",
        "| Verzeichnis | Dateien |",
        "|-------------|---------|",
    ]
    for d, n in verzeichnisse.most_common(25):
        zeilen.append(f"| `{d}` | {n} |")
    zeilen += ["", f"… insgesamt {len(verzeichnisse)} Verzeichnisse.", ""]

    hist = anhang()
    if hist:
        zeilen += ["---", "", "## Anhang: Änderungsnachtrag 2026-11-08 (P0-Härtung)", hist.rstrip(), ""]

    # ---------- Index ----------
    idx = [
        "# 📑 File Register Index — a-townchain",
        "",
        f"> Vollständige Dateiliste. Erzeugt {stempel} aus `{rev}` @ `{sha}`.",
        "> Lesbare Kennzahlen: [`FILE_REGISTER.md`](FILE_REGISTER.md).",
        "",
        f"**Total: {len(paths)} Dateien**",
        "",
    ]
    gruppen: dict[str, list[str]] = collections.defaultdict(list)
    for p in paths:
        gruppen[bereich(p)].append(p)
    for b, _ in nach_bereich.most_common():
        eintraege = gruppen[b]
        idx += [f"## {b} ({len(eintraege)})", ""]
        idx += [f"- `{p}`" for p in eintraege]
        idx.append("")

    open(REGISTER, "w", encoding="utf-8").write("\n".join(zeilen))
    open(INDEX, "w", encoding="utf-8").write("\n".join(idx))
    print(f"FILE_REGISTER.md: {len(zeilen)} Zeilen")
    print(f"FILE_REGISTER_INDEX.md: {len(idx)} Zeilen, {len(paths)} Dateien @ {sha}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Prueft relative Markdown-Links im Baum.

Ein relativer Link, dessen Ziel nicht existiert, ist ein toter Verweis: der
Leser landet auf einer 404-Seite, und niemand merkt es, weil Markdown-Links
nirgends geprueft werden. Dieses Werkzeug macht den Zustand messbar.

Regeln:
- externe Ziele (http/https/mailto/tel) und reine Anker (#...) werden nicht geprueft
- Inline-Code (`...`) wird ignoriert, damit Code-Fragmente wie H[x](:32) nicht
  als Link gelten
- archivierte Baeume (/archive/, /legacy/) sind ausgenommen: dort sind tote
  Verweise Teil des historischen Materials
- Ziele ausserhalb des Baums (andere Repositories) werden als Fehler gemeldet,
  weil ein solcher Link im Browser immer 404 ergibt; der Verweis gehoert auf die
  GitHub-URL der vorhandenen Datei
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

SKIP_DIRS = ("/archive/", "/legacy/", "node_modules", "/.git/")
LINK = re.compile(r"\[([^\]]*)\]\(([^)\s]+)(\s+\"[^\"]*\")?\)")
CODE = re.compile(r"`[^`\n]*`")


def md_files(root: Path) -> list[Path]:
    return [
        p
        for p in sorted(root.rglob("*.md"))
        if not any(k in "/" + str(p.relative_to(root)) for k in SKIP_DIRS)
    ]


def broken_links(root: Path) -> list[dict]:
    findings: list[dict] = []
    for path in md_files(root):
        text = path.read_text(encoding="utf-8", errors="replace")
        masked = CODE.sub(lambda m: " " * len(m.group(0)), text)
        for match in LINK.finditer(masked):
            target = match.group(2)
            if target.startswith(("http://", "https://", "mailto:", "tel:", "#")):
                continue
            clean = target.split("#")[0]
            if not clean:
                continue
            if (path.parent / clean).resolve().exists():
                continue
            findings.append(
                {
                    "file": str(path.relative_to(root)),
                    "line": masked[: match.start()].count("\n") + 1,
                    "text": match.group(1),
                    "target": target,
                }
            )
    return findings


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Prueft relative Markdown-Links.")
    parser.add_argument("--root", default=None, help="Wurzel (Standard: zwei Ebenen ueber diesem Skript)")
    parser.add_argument("--quiet", action="store_true", help="nur Zusammenfassung")
    args = parser.parse_args(argv)

    root = Path(args.root).resolve() if args.root else Path(__file__).resolve().parents[1]
    findings = broken_links(root)

    if not args.quiet:
        for f in findings:
            print(f"{f['file']}:{f['line']}  [{f['text']}]({f['target']})")

    print(f"Markdown-Link-Pruefung ({root.name}): {len(md_files(root))} Dateien, {len(findings)} tote relative Links")
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())

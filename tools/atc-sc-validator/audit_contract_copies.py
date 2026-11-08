#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Audit der Vertragskopien im Monorepo (ATC-STD-SC-019, Befund 08.11.2026).

Ein Vertragsname darf mehrfach vorkommen, aber dann muss die Sache entschieden
sein: entweder byte-identische Spiegelkopie (``identical``) oder bewusst
getrennte Implementierung (``divergent``). Der Audit friert diese Einordnung in
``standards/contracts/registry/copy_audit.yaml`` ein und meldet jede Abweichung
davon — neuer Drift faellt damit auf, ohne dass die bekannten Faelle den Build
rot machen.

Aufruf:
    tools/atc-sc-validator/audit_contract_copies.py            # pruefen
    tools/atc-sc-validator/audit_contract_copies.py --write    # Einordnung neu schreiben
    tools/atc-sc-validator/audit_contract_copies.py --json     # Maschinenlesbar

Exit-Codes: 0 unveraendert, 1 Abweichung, 2 Aufruffehler.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
AUDIT = ROOT / "standards/contracts/registry/copy_audit.yaml"
# docs/ enthaelt Historie und Archive: dort sind Kopien erwuenscht.
SKIP_DIRS = {"docs", "target", "node_modules", ".git"}

CONTRACT_RE = re.compile(r"^\s*contract\s+([A-Za-z_][A-Za-z0-9_]*)", re.MULTILINE)


def _iter_sources() -> list[Path]:
    out = []
    for p in ROOT.rglob("*.atc"):
        rel = p.relative_to(ROOT)
        if rel.parts and rel.parts[0] in SKIP_DIRS:
            continue
        out.append(p)
    return sorted(out)


def collect() -> dict[str, dict]:
    """Gruppiert alle Vertraege nach Name und Inhalt."""
    groups: dict[str, list[Path]] = {}
    for p in _iter_sources():
        try:
            text = p.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        for match in CONTRACT_RE.finditer(text):
            groups.setdefault(match.group(1), []).append(p)

    result: dict[str, dict] = {}
    for name, paths in sorted(groups.items()):
        if len(paths) < 2:
            continue
        by_hash: dict[str, list[str]] = {}
        for p in paths:
            digest = hashlib.sha256(p.read_bytes()).hexdigest()
            by_hash.setdefault(digest, []).append(str(p.relative_to(ROOT)))
        kind = "identical" if len(by_hash) == 1 else "divergent"
        result[name] = {
            "kind": kind,
            "copies": len(paths),
            "variants": len(by_hash),
            "files": sorted(str(p.relative_to(ROOT)) for p in paths),
        }
    return result


def _simple_yaml(data: dict[str, dict]) -> str:
    lines = [
        "# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0",
        "# Erzeugt von tools/atc-sc-validator/audit_contract_copies.py --write",
        "# kind: identical = byte-gleiche Spiegelkopie, divergent = getrennte Implementierung",
        "contracts:",
    ]
    for name in sorted(data):
        entry = data[name]
        lines.append(f"  {name}:")
        lines.append(f"    kind: {entry['kind']}")
        lines.append(f"    copies: {entry['copies']}")
        lines.append(f"    variants: {entry['variants']}")
        lines.append("    files:")
        for f in entry["files"]:
            lines.append(f"      - {f}")
    return "\n".join(lines) + "\n"


def _parse_frozen(text: str) -> dict[str, dict]:
    """Liest die eingefrorene Einordnung. Bewusst tolerant, aber ohne Duplikate."""
    frozen: dict[str, dict] = {}
    current: str | None = None
    in_files = False
    for raw in text.splitlines():
        line = raw.split("#", 1)[0].rstrip()
        if not line.strip():
            continue
        if line.startswith("contracts:"):
            continue
        if line.startswith("  ") and not line.startswith("    "):
            current = line.strip().rstrip(":")
            frozen[current] = {"kind": "", "copies": 0, "variants": 0, "files": []}
            in_files = False
            continue
        if current is None:
            continue
        body = line.strip()
        if body.startswith("- "):
            if in_files:
                frozen[current]["files"].append(body[2:].strip())
            continue
        in_files = False
        if body.startswith("kind:"):
            frozen[current]["kind"] = body.split(":", 1)[1].strip()
        elif body.startswith("copies:"):
            frozen[current]["copies"] = int(body.split(":", 1)[1])
        elif body.startswith("variants:"):
            frozen[current]["variants"] = int(body.split(":", 1)[1])
        elif body.startswith("files:"):
            in_files = True
    return frozen


def compare(frozen: dict[str, dict], actual: dict[str, dict]) -> list[str]:
    findings: list[str] = []
    for name in sorted(set(frozen) | set(actual)):
        was, now = frozen.get(name), actual.get(name)
        if was is None:
            findings.append(f"neue Mehrfachdeklaration: {name} ({now['copies']}x, {now['kind']})")
            continue
        if now is None:
            findings.append(f"Mehrfachdeklaration verschwunden: {name} (war {was['copies']}x)")
            continue
        if was["kind"] == "identical" and now["kind"] == "divergent":
            findings.append(
                f"{name}: Kopien driften jetzt ({now['variants']} Varianten) — sie waren byte-gleich"
            )
        elif was["kind"] != now["kind"]:
            findings.append(f"{name}: Einordnung {was['kind']} -> {now['kind']}")
        if was["copies"] != now["copies"]:
            findings.append(f"{name}: Kopienzahl {was['copies']} -> {now['copies']}")
        if was["kind"] == "divergent" and was["variants"] != now["variants"]:
            findings.append(f"{name}: Variantenzahl {was['variants']} -> {now['variants']}")
        if was["kind"] == "divergent" and set(was["files"]) != set(now["files"]):
            findings.append(f"{name}: Dateiliste der Kopien geaendert")
    return findings


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Audit der Vertragskopien (SC-019)")
    parser.add_argument("--write", action="store_true", help="Einordnung neu schreiben")
    parser.add_argument("--json", action="store_true", help="Ergebnis als JSON ausgeben")
    args = parser.parse_args(argv)

    actual = collect()
    if args.write:
        AUDIT.parent.mkdir(parents=True, exist_ok=True)
        AUDIT.write_text(_simple_yaml(actual), encoding="utf-8")
        print(f"{AUDIT.relative_to(ROOT)}: {len(actual)} Mehrfachdeklarationen eingefroren")
        return 0

    if not AUDIT.exists():
        print(f"FEHLER: {AUDIT.relative_to(ROOT)} fehlt (mit --write erzeugen)", file=sys.stderr)
        return 2

    findings = compare(_parse_frozen(AUDIT.read_text(encoding="utf-8")), actual)
    if args.json:
        print(json.dumps({"findings": findings, "actual": actual}, indent=2, ensure_ascii=False))
        return 1 if findings else 0

    identical = sum(1 for v in actual.values() if v["kind"] == "identical")
    divergent = len(actual) - identical
    print(f"Vertragskopien-Audit (SC-019): {len(actual)} Mehrfachdeklarationen "
          f"({identical} byte-gleich, {divergent} divergent)")
    for f in findings:
        print(f"  ✗ {f}")
    print(f"  Befunde: {len(findings)}")
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())

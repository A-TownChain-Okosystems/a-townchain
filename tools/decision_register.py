#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Entscheidungsregister D1 bis D7 pruefen und in die Dokumente schreiben.

SSOT ist docs/decisions/exec-decision-register.yaml. Dieses Werkzeug rendert die
Registertabelle und schreibt sie zwischen die Marker in

    docs/decisions/AD-EXEC-D1-D7.md
    standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md

Damit koennen die beiden Dokumente nicht auseinanderlaufen: die Tabelle wird an
beiden Stellen aus derselben Quelle erzeugt.

Aufruf:
    python3 tools/decision_register.py --check   # nur pruefen, Exit 1 bei Abweichung
    python3 tools/decision_register.py --write   # Tabelle neu schreiben
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
REGISTER = ROOT / "docs/decisions/exec-decision-register.yaml"
DOCS = [
    ROOT / "docs/decisions/AD-EXEC-D1-D7.md",
    ROOT / "standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md",
]
BEGIN = "<!-- BEGIN DECISION-REGISTER (erzeugt aus docs/decisions/exec-decision-register.yaml, nicht von Hand aendern) -->"
END = "<!-- END DECISION-REGISTER -->"

STATUS_VALUES = {"DECIDED", "SPECIFIED", "IMPLEMENTED", "OPEN"}
APPROVAL_VALUES = {"FREIGEGEBEN", "BEDINGT_FREIGEGEBEN", "OFFEN"}


def load() -> dict:
    with REGISTER.open(encoding="utf-8") as fh:
        data = yaml.safe_load(fh)
    if not isinstance(data, dict) or "decisions" not in data:
        raise SystemExit(f"{REGISTER}: 'decisions' fehlt")
    return data


def validate(data: dict) -> list[str]:
    errors: list[str] = []
    ids = set()
    for dec in data["decisions"]:
        did = dec.get("id", "?")
        if did in ids:
            errors.append(f"{did}: doppelte ID")
        ids.add(did)
        for field in ("title", "decision_status", "decision", "gate"):
            if not dec.get(field):
                errors.append(f"{did}: Feld '{field}' fehlt")
        if dec.get("decision_status") not in STATUS_VALUES:
            errors.append(f"{did}: decision_status '{dec.get('decision_status')}' ungueltig")
        if dec.get("specification_status") not in STATUS_VALUES:
            errors.append(f"{did}: specification_status '{dec.get('specification_status')}' ungueltig")
        if dec.get("implementation_status") not in STATUS_VALUES:
            errors.append(f"{did}: implementation_status '{dec.get('implementation_status')}' ungueltig")
        if dec.get("decision") not in APPROVAL_VALUES:
            errors.append(f"{did}: decision '{dec.get('decision')}' ungueltig")
        if dec.get("decision") == "BEDINGT_FREIGEGEBEN" and not dec.get("conditions"):
            errors.append(f"{did}: bedingt freigegeben ohne Auflagen")
    expected = [f"D{i}" for i in range(1, 8)]
    if sorted(ids) != sorted(expected):
        errors.append(f"erwartete IDs D1..D7, gefunden {sorted(ids)}")

    sub_ids = set()
    for dec in data["decisions"]:
        for sub in dec.get("sub_gates") or []:
            sid = sub.get("id", "?")
            if sid in sub_ids:
                errors.append(f"{sid}: doppelte ID in sub_gates")
            sub_ids.add(sid)
            for field in ("title", "owner", "goal", "status"):
                if not sub.get(field):
                    errors.append(f"{sid}: Feld '{field}' fehlt")
            if sub.get("status") not in STATUS_VALUES:
                errors.append(f"{sid}: status '{sub.get('status')}' ungueltig")

    seen = set()
    for st in data.get("stage1") or []:
        sid = st.get("id", "?")
        if sid in seen:
            errors.append(f"{sid}: doppelte ID in stage1")
        seen.add(sid)
        for field in ("title", "option", "ratified_at", "basis_head", "effect"):
            if not st.get(field):
                errors.append(f"{sid}: Feld '{field}' fehlt")
    return errors


def approval_text(value: str) -> str:
    return {
        "FREIGEGEBEN": "freigegeben",
        "BEDINGT_FREIGEGEBEN": "bedingt freigegeben",
        "OFFEN": "offen",
    }[value]


def render(data: dict) -> str:
    lines = [
        BEGIN,
        "",
        f"**Entscheidungsregister** (Owner, {data['decided_on']}; "
        f"Quelle: `docs/decisions/exec-decision-register.yaml`)",
        "",
        "| ID | Entscheidung (Titel) | Owner-Entscheidung | Ratifikation | Spezifikation | spezifiziert in | Umsetzung | Gate |",
        "|---|---|---|---|---|---|---|---|",
    ]
    for dec in data["decisions"]:
        lines.append(
            "| {id} | {title} | {own} | {rat} | {spec} | {where} | {impl} | {gate} |".format(
                id=dec["id"],
                title=dec["title"],
                own=approval_text(dec["decision"]),
                rat=dec.get("ratification", "nicht ratifiziert"),
                spec=dec["specification_status"],
                where=dec.get("specified_in", "-"),
                impl=dec["implementation_status"],
                gate=dec["gate"],
            )
        )
    sub_gates = [(dec["id"], sub) for dec in data["decisions"] for sub in (dec.get("sub_gates") or [])]
    if sub_gates:
        lines += [
            "",
            "**Teilauflagen** (getrennt implementierbar und testbar, je eigene Zustaendigkeit; "
            "eine erfuellte Teilauflage hebt die naechste nicht auf)",
            "",
            "| ID | Teilauflage | Zustaendigkeit | Ziel | Status | offen, weil |",
            "|---|---|---|---|---|---|",
        ]
        for parent, sub in sub_gates:
            lines.append(
                "| {id} | {title} | {owner} | {goal} | {status} | {because} |".format(
                    id=sub["id"],
                    title=sub["title"],
                    owner=sub["owner"],
                    goal=sub["goal"],
                    status=sub["status"],
                    because=sub.get("open_because", "-"),
                )
            )
        lines.append("")

    lines += [
        "",
        "Zustandsleiter: **DECIDED** (Entscheidung getroffen) → **SPECIFIED** "
        "(vollstaendig normativ spezifiziert) → **IMPLEMENTED** (im bestehenden "
        "System nachgewiesen). Kein Zustand ersetzt den naechsten automatisch; die "
        "Gates bleiben bis zum jeweiligen Nachweis offen.",
        "",
        "Drei Ebenen: **Owner-Entscheidung** ist intern verbindlich, auch wenn der "
        "zugehoerige Standard nicht ratifiziert ist. **Ratifikation** sagt, ob der "
        "Standard in Kraft ist; \"nicht ratifiziert (Kandidat)\" bindet niemanden. "
        "Ein Wert im Kandidaten bleibt **Vorschlag**, solange der Owner ihn nicht "
        "ausdruecklich beschliesst.",
        "",
        "Offen und nicht Teil dieser Freigabe: " + "; ".join(data["pending_globally"]) + ".",
        "",
    ]

    stage1 = data.get("stage1") or []
    if stage1:
        lines += [
            "**Stufe-1-Ratifikation** (Owner, " + str(data.get("decided_on", ""))
            + "; Quelle: `docs/decisions/exec-decision-register.yaml`)",
            "",
            "| ID | Gegenstand | Option | ratifiziert | Basis | Wirkung |",
            "|---|---|---|---|---|---|",
        ]
        for st in stage1:
            lines.append(
                "| {id} | {title} | {option} | {ratified_at} | `{basis_head}` | {effect} |".format(
                    id=st["id"],
                    title=st["title"],
                    option=st["option"],
                    ratified_at=st["ratified_at"],
                    basis_head=st["basis_head"],
                    effect=st["effect"],
                )
            )
        lines.append("")
        resolved = data.get("resolved_by_ratification") or []
        if resolved:
            lines += [
                "Durch die Stufe-1-Ratifikation entfallen: " + "; ".join(resolved) + ".",
                "",
            ]
        lines += [
            "Eine Stufe-1-Ratifikation entscheidet die Option, nicht den Standard: der "
            "Kandidat bleibt Kandidat, P1a bleibt offen und P2 bleibt gesperrt.",
            "",
            "Nicht freigegeben sind weiterhin Implementierung und Release.",
            "",
        ]

    lines += ["", END]
    return "\n".join(lines)


def apply(path: Path, block: str, write: bool) -> bool:
    text = path.read_text(encoding="utf-8")
    if BEGIN not in text or END not in text:
        raise SystemExit(f"{path}: Marker fehlen")
    start = text.index(BEGIN)
    stop = text.index(END) + len(END)
    new = text[:start] + block + text[stop:]
    if new == text:
        return True
    if write:
        path.write_text(new, encoding="utf-8")
    return False


def main() -> int:
    ap = argparse.ArgumentParser(description="Entscheidungsregister pruefen oder schreiben")
    ap.add_argument("--write", action="store_true", help="Tabelle in die Dokumente schreiben")
    ap.add_argument("--check", action="store_true", help="nur pruefen")
    args = ap.parse_args()
    if not (args.write or args.check):
        ap.error("--write oder --check angeben")

    data = load()
    errors = validate(data)
    if errors:
        for e in errors:
            print(f"FEHLER {e}")
        return 1

    block = render(data)
    stale = []
    for path in DOCS:
        identical = apply(path, block, args.write)
        if not identical:
            stale.append(path.relative_to(ROOT))

    if args.write:
        print(f"Register geschrieben: {len(data['decisions'])} Entscheidungen, {len(DOCS)} Dokumente")
        return 0

    if stale:
        for s in stale:
            print(f"ABWEICHUNG {s}: Registerblock veraltet, --write ausfuehren")
        return 1
    print(f"PASS: Register konsistent in {len(DOCS)} Dokumenten, {len(data['decisions'])} Entscheidungen")
    return 0


if __name__ == "__main__":
    sys.exit(main())

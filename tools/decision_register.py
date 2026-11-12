#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Entscheidungsregister D1 bis D7 pruefen und in die Dokumente schreiben.

SSOT ist docs/decisions/exec-decision-register.yaml. Dieses Werkzeug rendert die
Registertabelle und schreibt sie zwischen die Marker in

    docs/decisions/AD-EXEC-D1-D7.md
    standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md

Der Registerblock `chain_state` (P1d-B2) wird getrennt geprueft und in ein
eigenes Dokument geschrieben:

    docs/decisions/AD-P1D-B2-FOUR-DECISIONS-2026-11-12.md

Damit gilt fuer beide Bloecke dieselbe Regel: erzeugt wird nur aus dieser Datei.

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

CHAIN_STATE_DOC = ROOT / "docs/decisions/AD-P1D-B2-FOUR-DECISIONS-2026-11-12.md"
CS_BEGIN = "<!-- BEGIN CHAIN-STATE-DECISIONS (erzeugt aus docs/decisions/exec-decision-register.yaml, nicht von Hand aendern) -->"
CS_END = "<!-- END CHAIN-STATE-DECISIONS -->"

GATES_DOC = ROOT / "docs/decisions/EXEC-P1D-B2-S1-S5-ENTRY-EVIDENCE-2026-11-12.md"
G_BEGIN = "<!-- BEGIN CHAIN-STATE-GATES (erzeugt aus docs/decisions/exec-decision-register.yaml, nicht von Hand aendern) -->"
G_END = "<!-- END CHAIN-STATE-GATES -->"
# Die vier Entscheidungen sind benannt und abgeschlossen. Ein neuer Eintrag ist
# eine Owner-Entscheidung, keine stille Erweiterung des Registers.
CS_IDS = ["AD-B1-01", "AD-B1-02", "AD-B1-03", "AD-B1-04"]
CS_PHASES = {"P1d-C", "P1d-D", "P1d-E"}
# Die P1d-C-Abnahmetests stehen hier, nicht in C selbst: ein Abnahmetest, der erst
# beim Umbau entsteht, kann nicht mehr als Eintrittsbedingung dienen.
C_ENTRY_IDS = [f"C-T-{i:02d}" for i in range(1, 9)]

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
    # D1 bis D7 sind Pflicht. Zusaetzlich zulaessig sind ausdruecklich benannte
    # Entscheidungen mit eigener Kennung: D-0 (Formatentscheidung) und S1-2-F1
    # (Admission Registry, F-1 aus der Stufe-1-Arbeit). Alles andere nicht: das
    # Register soll nicht unbemerkt wachsen.
    pflicht = [f"D{i}" for i in range(1, 8)]
    zusaetzlich = {"D-0", "S1-2-F1"}
    zulaessig = set(pflicht) | zusaetzlich
    fehlend = sorted(set(pflicht) - ids)
    unbekannt = sorted(ids - zulaessig)
    if fehlend:
        errors.append(f"erwartete IDs D1..D7, fehlend: {fehlend}")
    if unbekannt:
        errors.append(
            f"unbekannte IDs: {unbekannt} (zulaessig sind D-0, D1..D7 und "
            f"{sorted(zusaetzlich - {'D-0'})})"
        )

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


def validate_chain_state(data: dict) -> list[str]:
    """Den Block `chain_state` pruefen (P1d-B2, Chain- und Zustandsautoritaet)."""
    errors: list[str] = []
    cs = data.get("chain_state")
    if not cs:
        return ["chain_state fehlt (P1d-B2-Entscheidungen AD-B1-01 bis AD-B1-04)"]

    for field in ("register", "decided_on", "basis_head", "source"):
        if not cs.get(field):
            errors.append(f"chain_state: Feld '{field}' fehlt")

    rule = cs.get("rule") or {}
    for field in ("id", "title", "text"):
        if not rule.get(field):
            errors.append(f"chain_state.rule: Feld '{field}' fehlt")
    if not rule.get("classes"):
        errors.append("chain_state.rule: 'classes' fehlt (Phasen der Regel)")
    cond = rule.get("commit_precondition") or {}
    fields = cond.get("fields") or []
    if len(fields) != 4:
        errors.append(
            "chain_state.rule.commit_precondition: genau vier erwartete Ausgangswerte "
            f"(Hoehe, Hash, state_root, state_version), gefunden {len(fields)}"
        )
    if not cond.get("rule"):
        errors.append("chain_state.rule.commit_precondition: 'rule' fehlt")

    ids = [dec.get("id", "?") for dec in cs.get("decisions") or []]
    if ids != CS_IDS:
        errors.append(f"chain_state: erwartete IDs {CS_IDS}, gefunden {ids}")
    for dec in cs.get("decisions") or []:
        did = dec.get("id", "?")
        for field in ("title", "choice", "decision", "status", "phase"):
            if not dec.get(field):
                errors.append(f"{did}: Feld '{field}' fehlt")
        if dec.get("status") not in STATUS_VALUES:
            errors.append(f"{did}: status '{dec.get('status')}' ungueltig")
        if dec.get("decision") not in APPROVAL_VALUES:
            errors.append(f"{did}: decision '{dec.get('decision')}' ungueltig")
        if dec.get("phase") not in CS_PHASES:
            errors.append(f"{did}: phase '{dec.get('phase')}' ungueltig")
        if not dec.get("consequences"):
            errors.append(f"{did}: 'consequences' fehlt (Folgen gehoeren zur Entscheidung)")
        if not dec.get("rejected"):
            errors.append(f"{did}: 'rejected' fehlt (die verworfene Alternative muss benannt sein)")

    gate_ids = set()
    test_ids: set[str] = set()
    for gate in cs.get("specification_gates") or []:
        gid = gate.get("id", "?")
        if gid in gate_ids:
            errors.append(f"{gid}: doppelte ID in specification_gates")
        gate_ids.add(gid)
        for field in ("title", "blocks", "acceptance", "decides", "specification"):
            if not gate.get(field):
                errors.append(f"{gid}: Feld '{field}' fehlt")
        if gate.get("status") not in STATUS_VALUES:
            errors.append(f"{gid}: status '{gate.get('status')}' ungueltig")
        # Ein Gate ist geschlossen, wenn seine Abnahme belegt ist, nicht wenn sie
        # beschlossen ist: ohne Nachweispfad bleibt es OPEN.
        if gate.get("status") in {"SPECIFIED", "IMPLEMENTED"} and not gate.get("evidence"):
            errors.append(
                f"{gid}: status '{gate.get('status')}' ohne 'evidence' "
                "(ein Gate ist geschlossen, wenn seine Abnahme belegt ist)"
            )
        for field in ("required_content", "artifacts", "owner_points", "entry_tests"):
            if not gate.get(field):
                errors.append(f"{gid}: '{field}' fehlt")
        for test in gate.get("entry_tests") or []:
            tid = test.get("id", "?")
            if tid in test_ids:
                errors.append(f"{tid}: doppelte ID in entry_tests")
            test_ids.add(tid)
            if not tid.startswith(f"{gid}-T-"):
                errors.append(f"{tid}: Eintrittstest gehoert nicht zum Gate {gid}")
            for field in ("test", "proves"):
                if not test.get(field):
                    errors.append(f"{tid}: Feld '{field}' fehlt")
            if not test.get("proves"):
                errors.append(f"{tid}: 'proves' fehlt (was der Test belegt)")
        if len(gate.get("entry_tests") or []) < 3:
            errors.append(f"{gid}: weniger als drei Eintrittstests")
    if not gate_ids:
        errors.append("chain_state: keine specification_gates (P1d-C waere ohne Spezifikation freigegeben)")

    entry_ids = [item.get("id", "?") for item in cs.get("p1d_c_entry") or []]
    if entry_ids != C_ENTRY_IDS:
        errors.append(f"chain_state: erwartete P1d-C-Abnahmetests {C_ENTRY_IDS}, gefunden {entry_ids}")
    for item in cs.get("p1d_c_entry") or []:
        cid = item.get("id", "?")
        for field in ("title", "requires"):
            if not item.get(field):
                errors.append(f"{cid}: Feld '{field}' fehlt")
        if item.get("status") not in STATUS_VALUES:
            errors.append(f"{cid}: status '{item.get('status')}' ungueltig")

    if not cs.get("pending"):
        errors.append("chain_state: 'pending' fehlt (offene Punkte muessen sichtbar bleiben)")
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


def render_chain_state(data: dict) -> str:
    """Den Block `chain_state` als Markdown rendern (Ziel: P1d-B2-Dokument)."""
    cs = data["chain_state"]
    rule = cs["rule"]
    lines = [
        CS_BEGIN,
        "",
        f"**Registerstand P1d-B2** (Owner, {cs['decided_on']}; Basis `{cs['basis_head']}`; "
        "Quelle: `docs/decisions/exec-decision-register.yaml`)",
        "",
        f"**{rule['id']} — {rule['title']}**",
        "",
        f"> {rule['text']}",
        "",
    ]
    if rule.get("classes"):
        lines += [
            "Damit sind fuenf Vorgaenge getrennt: " + ", ".join(rule["classes"]) + ".",
            "",
        ]

    lines += [
        "| ID | Gegenstand | Entscheidung | Owner-Entscheidung | Status | Phase |",
        "|---|---|---|---|---|---|",
    ]
    for dec in cs["decisions"]:
        lines.append(
            "| {id} | {title} | {choice} | {own} | {status} | {phase} |".format(
                id=dec["id"],
                title=dec["title"],
                choice=dec["choice"],
                own=approval_text(dec["decision"]),
                status=dec["status"],
                phase=dec["phase"],
            )
        )

    cond = rule.get("commit_precondition") or {}
    if cond:
        lines += [
            "",
            "**Commit-Vorbedingung** (Owner-Vorgabe, keine fuenfte Entscheidung, sondern "
            "die Bedingung, ohne die AD-B1-01 bis AD-B1-04 nicht pruefbar waeren): "
            + ", ".join(f"`{f}`" for f in cond["fields"])
            + ".",
            "",
            f"> {cond['rule']}",
            "",
        ]

    gates = cs.get("specification_gates") or []
    if gates:
        lines += [
            "**Spezifikationsgates** (muessen vor P1d-C geschlossen sein; ein Gate ist "
            "geschlossen, wenn seine Abnahme belegt ist, nicht wenn sie beschlossen ist)",
            "",
            "| ID | Spezifikation | sperrt | Abnahme |",
            "|---|---|---|---|",
        ]
        for gate in gates:
            lines.append(
                "| {id} | {title} | {blocks} | {acc} |".format(
                    id=gate["id"],
                    title=gate["title"],
                    blocks=gate["blocks"],
                    acc=gate["acceptance"],
                )
            )
        lines.append("")

    pending = cs.get("pending") or []
    if pending:
        lines += ["**Offen und hier nicht entschieden:**", ""]
        lines += [f"- {item}" for item in pending]
        lines.append("")

    lines += [
        "Die vier Entscheidungen sind **getroffen**, nicht umgesetzt: sie aendern keinen "
        "Code, vergeben kein `CI_VERIFIED` und oeffnen P1d-C erst, wenn die "
        "Spezifikationsgates belegt sind.",
        "",
        CS_END,
    ]
    return "\n".join(lines)


def render_gates(data: dict) -> str:
    """Die Spezifikationsgates B2-S1 bis B2-S5 und die P1d-C-Abnahmetests."""
    cs = data["chain_state"]
    gates = cs["specification_gates"]
    entry = cs.get("p1d_c_entry") or []
    lines = [
        G_BEGIN,
        "",
        f"**Spezifikationsgates P1d-B2** (Owner, {cs['decided_on']}; "
        f"Erhebung `{cs.get('specification_basis_head', cs['basis_head'])}`; "
        "Quelle: `docs/decisions/exec-decision-register.yaml`)",
        "",
        "> Ein Gate ist geschlossen, wenn seine Abnahme belegt ist, nicht wenn sie "
        "beschlossen ist. `SPECIFIED` verlangt fertige Artefakte und einen gruenen "
        "Pruefer; `IMPLEMENTED` ist erst nach P1d-C vergebbar.",
        "",
        "| ID | Gegenstand | entscheidet | Status | Eintrittstests |",
        "|---|---|---|---|---|",
    ]
    for gate in gates:
        lines.append(
            "| {id} | {title} | {dec} | {status} | {n} |".format(
                id=gate["id"],
                title=gate["title"],
                dec=gate["decides"],
                status=gate["status"],
                n=len(gate["entry_tests"]),
            )
        )

    for gate in gates:
        lines += [
            "",
            f"### {gate['id']} {gate['title']}",
            "",
            f"**Status:** `{gate['status']}` | **entscheidet:** {gate['decides']} | "
            f"**sperrt:** {gate['blocks']}",
            "",
            f"**Abnahme:** {gate['acceptance']}",
            "",
            f"**Worum es geht:** {gate['specification']}",
            "",
            "**Was festgelegt werden muss:**",
            "",
        ]
        lines += [f"- {item}" for item in gate["required_content"]]
        lines += ["", "**Artefakte, ohne die das Gate nicht schliessbar ist:**", ""]
        lines += [f"- `{item}`" for item in gate["artifacts"]]
        if gate.get("owner_points"):
            lines += ["", "**Offene Owner-Punkte:**", ""]
            lines += [f"- {item}" for item in gate["owner_points"]]
        lines += [
            "",
            "**Eintrittstests:**",
            "",
            "| ID | Pruefung | was sie belegt |",
            "|---|---|---|",
        ]
        for test in gate["entry_tests"]:
            lines.append(
                "| {id} | {test} | {proves} |".format(
                    id=test["id"], test=test["test"], proves=test["proves"]
                )
            )
        if gate.get("evidence"):
            lines += ["", f"**Nachweis:** `{gate['evidence']}`"]

    if entry:
        lines += [
            "",
            "## P1d-C-Abnahmetests (Eintrittsbedingung, nicht Teil von C)",
            "",
            "| ID | Abnahme | setzt voraus | Status |",
            "|---|---|---|---|",
        ]
        for item in entry:
            lines.append(
                "| {id} | {title} | {req} | {status} |".format(
                    id=item["id"], title=item["title"], req=item["requires"], status=item["status"]
                )
            )
        lines += [
            "",
            "Die acht Abnahmen stehen hier, weil ein Abnahmetest, der erst beim Umbau "
            "entsteht, nicht mehr als Eintrittsbedingung wirken kann. `C-T-08` ist die "
            "Verschaerfung aus der Owner-Bewertung vom 12.11.2026: `state_root_before` "
            "wird in C Pruefgroesse und bleibt nicht Belegfeld.",
        ]

    lines += ["", G_END]
    return "\n".join(lines)


def apply(path: Path, block: str, write: bool, begin: str = BEGIN, end: str = END) -> bool:
    text = path.read_text(encoding="utf-8")
    if begin not in text or end not in text:
        raise SystemExit(f"{path}: Marker fehlen")
    start = text.index(begin)
    stop = text.index(end) + len(end)
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
    errors = validate(data) + validate_chain_state(data)
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

    cs_block = render_chain_state(data)
    if not apply(CHAIN_STATE_DOC, cs_block, args.write, CS_BEGIN, CS_END):
        stale.append(CHAIN_STATE_DOC.relative_to(ROOT))

    cs = data["chain_state"]
    g_block = render_gates(data)
    if not apply(GATES_DOC, g_block, args.write, G_BEGIN, G_END):
        stale.append(GATES_DOC.relative_to(ROOT))

    if args.write:
        print(
            f"Register geschrieben: {len(data['decisions'])} Entscheidungen, "
            f"{len(DOCS)} Dokumente; chain_state: {len(cs['decisions'])} Entscheidungen, "
            f"1 Dokument; Gates: {len(cs['specification_gates'])}, "
            f"P1d-C-Abnahmen: {len(cs.get('p1d_c_entry') or [])}, 1 Dokument"
        )
        return 0

    if stale:
        for s in stale:
            print(f"ABWEICHUNG {s}: Registerblock veraltet, --write ausfuehren")
        return 1
    print(
        f"PASS: Register konsistent in {len(DOCS)} Dokumenten, {len(data['decisions'])} "
        f"Entscheidungen; chain_state konsistent in 1 Dokument, {len(cs['decisions'])} Entscheidungen; "
        f"Gates konsistent in 1 Dokument, {len(cs['specification_gates'])} Gates, "
        f"{len(cs.get('p1d_c_entry') or [])} P1d-C-Abnahmen"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())

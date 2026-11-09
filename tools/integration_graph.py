#!/usr/bin/env python3
"""Integrations-Gate: zaehlt Crates, Inseln und Ausfuehrungsbruecken.

Baut aus den Cargo-Manifesten der angegebenen Repos einen internen
Crate-Graphen (nur Arbeitsabhaengigkeiten, dev-dependencies zaehlen NICHT als
Produktkante), klassifiziert die Crates nach Bereich und prueft die sechs
Bruecken, ohne die das Oekosystem keine Kette bildet.

Ausgabe: Tabelle oder JSON. Exit 1 nur bei neuer Insel gegenueber der Baseline
(Regression) oder wenn eine Bruecke als beansprucht markiert ist, aber keinen
Beleg hat. Fehlende Bruecken sind Befund, kein Absturz: dieses Werkzeug
beschreibt den Ist-Stand, es erzwingt ihn nicht.

Kein Netzwerk, keine Schreibvorgaenge ausser --update-baseline.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

# --- Manifest-Lesen -------------------------------------------------------

SECTION_RE = re.compile(r"^\s*\[([^\]]+)\]\s*$")
NAME_RE = re.compile(r"^\s*name\s*=\s*\"([^\"]+)\"")
DEP_RE = re.compile(r"^\s*([A-Za-z0-9_.\-]+)\s*=\s*(.+)$")

# Abschnitte, die eine Arbeitsabhaengigkeit beschreiben. dev-dependencies
# fehlen hier bewusst: ein Test, der eine Komponente importiert, verbindet
# nichts im Betrieb.
WORK_SECTIONS = ("dependencies", "build-dependencies")


def parse_manifest(text: str) -> tuple[str | None, dict[str, str], dict[str, str]]:
    """Liefert (Paketname, Arbeitsabhaengigkeiten, dev-Abhaengigkeiten)."""
    name = None
    work: dict[str, str] = {}
    dev: dict[str, str] = {}
    section = ""
    in_package = False
    for line in text.splitlines():
        m = SECTION_RE.match(line)
        if m:
            section = m.group(1).strip()
            in_package = section == "package"
            continue
        if in_package and name is None:
            m = NAME_RE.match(line)
            if m:
                name = m.group(1)
                continue
        is_work = section in WORK_SECTIONS or section.endswith(".dependencies") and "dev" not in section
        is_dev = "dev-dependencies" in section
        if not (is_work or is_dev):
            continue
        m = DEP_RE.match(line)
        if not m:
            continue
        dep, tail = m.group(1), m.group(2)
        path = re.search(r"path\s*=\s*\"([^\"]+)\"", tail)
        target = work if is_work else dev
        target[dep] = path.group(1) if path else ""
    return name, work, dev


def find_crates(root: Path) -> dict[str, dict]:
    crates: dict[str, dict] = {}
    for manifest in sorted(root.rglob("Cargo.toml")):
        parts = set(manifest.parts)
        if ".git" in parts or "target" in parts:
            continue
        try:
            name, work, dev = parse_manifest(manifest.read_text(encoding="utf-8", errors="replace"))
        except OSError:
            continue
        if not name:
            continue
        rel = manifest.parent.relative_to(root).as_posix()
        # Ein Name darf mehrfach vorkommen (Spiegel); der erste gewinnt, die
        # weiteren werden als Duplikat gemeldet statt still verschluckt.
        crates.setdefault(name, {"path": rel, "root": root.name, "work": work, "dev": dev, "duplicates": []})
        if crates[name]["path"] != rel:
            crates[name]["duplicates"].append(rel)
    return crates


# --- Bereiche -------------------------------------------------------------

def area_of(path: str) -> str:
    parts = path.split("/") if path and path != "." else ["."]
    if len(parts) >= 3 and parts[0] == "modules" and parts[1] == "atc-shivacore":
        return f"shiva/{parts[2]}"
    top = {
        "components": lambda p: f"components/{p[1]}" if len(p) > 1 else "components",
        "modules": lambda p: f"modules/{p[1]}" if len(p) > 1 else "modules",
        "system": lambda p: f"system/{p[1]}" if len(p) > 1 else "system",
        "apps": lambda p: f"apps/{p[1]}" if len(p) > 1 else "apps",
        "ai": lambda p: f"ai/{p[1]}" if len(p) > 1 else "ai",
        "tooling": lambda p: f"tooling/{p[1]}" if len(p) > 1 else "tooling",
        "standards": lambda p: "standards",
        "ecosystem": lambda p: "ecosystem",
        "kernel": lambda p: "kernel-mirror",
    }
    return top.get(parts[0], lambda p: p[0])(parts)


# --- Bruecken -------------------------------------------------------------

BRIDGES = [
    ("B1", "Compiler -> Artefakt", "ein Produkt-Crate haengt am Compiler (atc-core), nicht nur die Werkzeuge"),
    ("B2", "Artefakt -> VM", "ein Crate haengt an atc-vm"),
    ("B3", "Vertrag -> Kette", "der Node haengt an atc-vm und ruft execute_state_transition"),
    ("B4", "OS -> Userland", "das CI-Image bekommt ein Ramdisk-Programm und baut den Init"),
    ("B5", "KI -> Tool-Grenze", "ein KI-Crate haengt an einer Tool-Ausfuehrung mit Allowlist"),
    ("B6", "Admission -> Durchsetzung", "Lader, VM oder Prozessstart haengen an atc-exec-admission"),
]


def source_text(root: Path, rel: str) -> str:
    base = root / rel
    if not base.is_dir():
        return ""
    out = []
    for f in sorted(base.rglob("*.rs")):
        try:
            out.append(f.read_text(encoding="utf-8", errors="replace"))
        except OSError:
            continue
    return "\n".join(out)


def evaluate_bridges(crates: dict[str, dict], roots: dict[str, Path]) -> list[dict]:
    def consumers(name: str) -> list[str]:
        return sorted(c for c, d in crates.items() if name in d["work"])

    findings = []

    b1 = [c for c in consumers("atc-core") if not area_of(crates[c]["path"]).startswith(("tooling/", "ai/atclang"))]
    findings.append({"id": "B1", "status": "VERBUNDEN" if b1 else "OFFEN", "evidence": b1 or ["nur Werkzeug-Crates haengen an atc-core"]})

    b2 = consumers("atc-vm")
    findings.append({"id": "B2", "status": "VERBUNDEN" if b2 else "OFFEN", "evidence": b2 or ["kein Crate haengt an atc-vm"]})

    node_ok = False
    node_evidence = []
    if "atc-node" in crates and "atc-vm" in crates["atc-node"]["work"]:
        text = source_text(roots[crates["atc-node"]["root"]], crates["atc-node"]["path"])
        if "execute_state_transition" in text:
            node_ok = True
            node_evidence.append("atc-node haengt an atc-vm und ruft execute_state_transition")
        else:
            node_evidence.append("atc-node haengt an atc-vm, ruft es aber nicht auf")
    else:
        node_evidence.append("atc-node haengt nicht an atc-vm")
    findings.append({"id": "B3", "status": "VERBUNDEN" if node_ok else "OFFEN", "evidence": node_evidence})

    # B4 verlangt BEIDES: das Init wird gebaut UND als Ramdisk-Eintrag an den
    # Boot-Builder uebergeben. Eine blosse Erwaehnung von "init_program" genuegt
    # nicht, sonst faerbt ein Pfadnamen-Fund das Gate gruen.
    ci_evidence = []
    ci_ok = False
    for root in roots.values():
        wf = root / ".github" / "workflows"
        if wf.is_dir():
            for yml in sorted(wf.glob("*.yml")):
                text = yml.read_text(encoding="utf-8", errors="replace")
                builds_init = bool(re.search(r"init_program[\s\S]{0,200}?cargo", text))
                passes_init = bool(re.search(r'"init=[^"]*shivacore-init"', text))
                if builds_init:
                    ci_evidence.append(f"{root.name}/{yml.name}: baut das Init-Programm")
                if passes_init:
                    ci_evidence.append(f"{root.name}/{yml.name}: uebergibt init=... an den Boot-Builder")
                if builds_init and passes_init:
                    ci_ok = True
        script = root / "scripts" / "qemu_boot.sh"
        if script.is_file():
            stext = script.read_text(encoding="utf-8", errors="replace")
            if 'init=$INIT_ELF' in stext:
                ci_evidence.append(
                    f"{root.name}/scripts/qemu_boot.sh: baut den Init und legt ihn in die Ramdisk"
                )
    findings.append({"id": "B4", "status": "VERBUNDEN" if ci_ok else "OFFEN", "evidence": ci_evidence or ["kein Workflow baut den Init und uebergibt ihn dem Image"]})

    ai_crates = [c for c, d in crates.items() if area_of(d["path"]).startswith(("ai/aurora", "ai/genesis"))]
    tool_targets = {"atc-exec-admission", "atc-tool-runtime", "globus-tool-runtime"}
    b5 = [c for c in ai_crates if tool_targets & set(crates[c]["work"])]
    findings.append({"id": "B5", "status": "VERBUNDEN" if b5 else "OFFEN", "evidence": b5 or [f"{len(ai_crates)} KI-Crates, keines haengt an einer Tool-Ausfuehrung"]})

    b6 = [c for c in consumers("atc-exec-admission") if crates[c]["path"] != "components/exec-admission"]
    findings.append({"id": "B6", "status": "VERBUNDEN" if b6 else "OFFEN", "evidence": b6 or ["niemand haengt an atc-exec-admission"]})
    return findings


# --- Hauptlauf ------------------------------------------------------------

def analyse(roots: dict[str, Path]) -> dict:
    crates: dict[str, dict] = {}
    for root in roots.values():
        for name, info in find_crates(root).items():
            if name in crates:
                crates[name]["duplicates"].append(f"{root.name}:{info['path']}")
                continue
            crates[name] = info

    edges = []
    incoming = {name: 0 for name in crates}
    dev_only = {name: 0 for name in crates}
    for name, info in crates.items():
        for dep in info["work"]:
            if dep in crates and dep != name:
                edges.append({"from": name, "to": dep, "area_from": area_of(info["path"]), "area_to": area_of(crates[dep]["path"])})
                incoming[dep] += 1
        for dep in info["dev"]:
            if dep in crates and dep != name and dep not in info["work"]:
                dev_only[dep] += 1

    islands = sorted(n for n in crates if incoming[n] == 0)
    areas: dict[str, int] = {}
    for info in crates.values():
        a = area_of(info["path"])
        areas[a] = areas.get(a, 0) + 1

    return {
        "crates": len(crates),
        "areas": dict(sorted(areas.items())),
        "edges": edges,
        "islands": islands,
        "dev_edges_not_counted": sorted(n for n, c in dev_only.items() if incoming[n] == 0 and c > 0),
        "bridges": evaluate_bridges(crates, roots),
        "duplicate_names": {n: d["duplicates"] for n, d in crates.items() if d["duplicates"]},
    }


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--repo", action="append", required=True, help="Repowurzel (mehrfach erlaubt)")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--baseline", help="Baseline-JSON; neue Inseln => Exit 1")
    ap.add_argument("--update-baseline", help="Baseline aus dem aktuellen Lauf schreiben")
    args = ap.parse_args()

    roots = {Path(r).name: Path(r).resolve() for r in args.repo}
    result = analyse(roots)

    if args.update_baseline:
        Path(args.update_baseline).write_text(
            json.dumps(
                {"crates": result["crates"], "islands": result["islands"]},
                indent=2,
                ensure_ascii=False,
            )
            + "\n",
            encoding="utf-8",
        )

    new_islands, healed = [], []
    if args.baseline:
        base = json.loads(Path(args.baseline).read_text(encoding="utf-8"))
        # Die Baseline haengt am Baumzustand, nicht am Verzeichnisnamen: eine
        # abweichende Crate-Zahl heisst "anderer Ausschnitt", nicht "Regression".
        recorded_crates = base.get("crates")
        if recorded_crates is not None and recorded_crates != result["crates"]:
            print(
                f"Hinweis: Baseline kennt {recorded_crates} Crates, Lauf sieht {result['crates']}",
                file=sys.stderr,
            )
        known = set(base.get("islands", []))
        new_islands = sorted(set(result["islands"]) - known)
        healed = sorted(known - set(result["islands"]))

    if args.json:
        out = dict(result)
        out["new_islands"] = new_islands
        out["healed_islands"] = healed
        print(json.dumps(out, indent=2, ensure_ascii=False))
    else:
        print(f"Crates: {result['crates']}  Kanten: {len(result['edges'])}  Inseln: {len(result['islands'])}")
        print("\nBereiche:")
        for a, n in result["areas"].items():
            print(f"  {a:28s} {n}")
        print("\nBruecken:")
        for b in result["bridges"]:
            print(f"  {b['id']} {b['status']:10s} {b['evidence'][0][:80]}")
        if result["duplicate_names"]:
            print(f"\nDoppelte Paketnamen: {len(result['duplicate_names'])}")
        if result["dev_edges_not_counted"]:
            print(f"Nur per dev-dependency erreicht (zaehlt nicht als Kante): {', '.join(result['dev_edges_not_counted'])}")

    status = 0
    if new_islands:
        print("\nREGRESSION: neue Inseln gegenueber Baseline:", ", ".join(new_islands), file=sys.stderr)
        status = 1
    if healed:
        print("Verbessert (nicht mehr Insel):", ", ".join(healed), file=sys.stderr)
    return status


if __name__ == "__main__":
    sys.exit(main())

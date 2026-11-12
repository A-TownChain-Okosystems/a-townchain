#!/usr/bin/env python3
"""Governance-Regel G-01: jedes Cargo-Manifest ist klassifiziert.

Ein Repository kann gruene Tests haben, waehrend sicherheitsrelevanter Rust-Code
ausserhalb der Build-Abdeckung liegt. Genau das war Befund F4
(`apps/atc-node`, kein Workspace-Mitglied, nie gebaut). Dieser Pruefer macht die
Luecke sichtbar und verhindert, dass ein neues Manifest unbemerkt hinzukommt.

Regeln:
  R1 Jedes Cargo.toml ausserhalb von `target/` steht in registry/build_coverage.yaml.
  R2 Jeder Eintrag hat einen Grund (nicht leer).
  R3 `workspace_member` muss der Workspace-Liste in Cargo.toml entsprechen — in
     beide Richtungen (kein Eintrag ohne Manifest, kein Mitglied ohne Eintrag).
  R4 `nested_workspace` verlangt einen eigenen `[workspace]` im genannten Baum.
  R5 `unverified_build` wird als offener Befund gemeldet (Exitcode bleibt 0,
     solange die Regeln R1-R4 halten), damit die Regel heute durchsetzbar ist und
     die Luecke sichtbar bleibt statt rot zu rauschen.

Aufruf:
    python3 tools/check_build_coverage.py            # pruefen
    python3 tools/check_build_coverage.py --write     # neue Manifeste ergaenzen
    python3 tools/check_build_coverage.py --strict    # offene Befunde = Fehler
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REGISTRY = ROOT / "registry" / "build_coverage.yaml"
STATUS = (
    "workspace_member",
    "nested_workspace",
    "nested_workspace_member",
    "duplicate_retired",
    "unverified_build",
)


def manifeste() -> list[str]:
    out = []
    for pfad in sorted(ROOT.rglob("Cargo.toml")):
        if "target" in pfad.parts or ".git" in pfad.parts:
            continue
        out.append(str(pfad.parent.relative_to(ROOT)))
    return sorted(p for p in out if p != ".")


def workspace_mitglieder() -> list[str]:
    text = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    return re.findall(r'"([^"]+)"', text.split("members = [")[1].split("]")[0])


def lies_registry() -> list[dict]:
    if not REGISTRY.exists():
        return []
    eintraege: list[dict] = []
    aktuell: dict | None = None
    for zeile in REGISTRY.read_text(encoding="utf-8").splitlines():
        s = zeile.strip()
        if s.startswith("- path:"):
            aktuell = {"path": s.split(":", 1)[1].strip().strip('"')}
            eintraege.append(aktuell)
        elif aktuell is not None and s.startswith("status:"):
            aktuell["status"] = s.split(":", 1)[1].strip()
        elif aktuell is not None and s.startswith("reason:"):
            aktuell["reason"] = s.split(":", 1)[1].strip().strip('"')
    return eintraege


def schreibe_registry(eintraege: list[dict]) -> None:
    kopf = REGISTRY.read_text(encoding="utf-8").split("manifests:")[0] if REGISTRY.exists() else ""
    with REGISTRY.open("w", encoding="utf-8") as f:
        f.write(kopf + "manifests:\n")
        for e in sorted(eintraege, key=lambda x: x["path"]):
            f.write(f'  - path: "{e["path"]}"\n    status: {e["status"]}\n    reason: "{e["reason"]}"\n')


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--write", action="store_true", help="fehlende Manifeste ergaenzen")
    ap.add_argument("--strict", action="store_true", help="offene Befunde sind Fehler")
    args = ap.parse_args(argv)

    gefunden = manifeste()
    eintraege = lies_registry()
    bekannt = {e["path"] for e in eintraege}
    fehler: list[str] = []

    fehlend = [m for m in gefunden if m not in bekannt]
    if args.write and fehlend:
        for pfad in fehlend:
            eltern = Path(pfad)
            wurzel = "."
            for kandidat in [eltern, *eltern.parents]:
                c = ROOT / kandidat / "Cargo.toml"
                if c.exists() and "[workspace]" in c.read_text(encoding="utf-8"):
                    wurzel = str(kandidat)
                    break
            if wurzel != ".":
                status = (
                    "nested_workspace" if wurzel == pfad else "nested_workspace_member"
                )
                grund = f"Build-Wurzelbaum: {wurzel}"
            else:
                status, grund = "unverified_build", "neu; Pruefstrategie festzulegen (G-01)"
            eintraege.append({"path": pfad, "status": status, "reason": grund})
        schreibe_registry(eintraege)
        bekannt = {e["path"] for e in eintraege}
        print(f"ergaenzt: {len(fehlend)} Manifest(e)")
        fehlend = []

    if fehlend:  # R1
        fehler.append(f"R1 nicht klassifiziert: {', '.join(fehlend)}")

    mitglieder = workspace_mitglieder()
    for e in eintraege:  # R2, R3, R4
        if e.get("status") not in STATUS:
            fehler.append(f"R2 unbekannter Status fuer {e['path']}: {e.get('status')!r}")
        if not e.get("reason"):
            fehler.append(f"R2 Grund fehlt: {e['path']}")
        ist_mitglied = any(e["path"] == m or e["path"].startswith(m + "/") for m in mitglieder)
        if e.get("status") == "workspace_member" and not ist_mitglied:
            fehler.append(f"R3 {e['path']} als Mitglied gefuehrt, steht aber nicht in Cargo.toml")
        if ist_mitglied and e.get("status") not in ("workspace_member",):
            fehler.append(f"R3 {e['path']} ist Workspace-Mitglied, aber als {e.get('status')} gefuehrt")
        if e.get("status") == "nested_workspace":
            c = ROOT / e["path"] / "Cargo.toml"
            if not c.exists() or "[workspace]" not in c.read_text(encoding="utf-8"):
                fehler.append(f"R4 {e['path']} hat keinen eigenen [workspace]")
        if e.get("status") == "nested_workspace_member":
            if not any(
                (ROOT / a / "Cargo.toml").exists()
                and "[workspace]" in (ROOT / a / "Cargo.toml").read_text(encoding="utf-8")
                for a in [*Path(e["path"]).parents]
                if str(a) != "."
            ):
                fehler.append(f"R4 {e['path']} liegt in keinem verschachtelten Workspace")

    for m in mitglieder:  # R3 rueckwaerts
        if m not in bekannt:
            fehler.append(f"R3 Mitglied ohne Eintrag: {m}")

    offen = [e["path"] for e in eintraege if e.get("status") == "unverified_build"]
    print(f"Manifeste: {len(gefunden)} | Eintraege: {len(eintraege)}")
    for status in STATUS:
        n = sum(1 for e in eintraege if e.get("status") == status)
        print(f"  {status:18s} {n:3d}")
    if offen:
        print(f"OFFEN (kein Build-Nachweis): {len(offen)}")
        for p in offen:
            print(f"  - {p}")
    if args.strict and offen:
        fehler.append(f"R5 {len(offen)} Manifest(e) ohne Build-Nachweis")

    if fehler:
        print("\nGATE FAIL")
        for f in fehler:
            print("  " + f)
        return 1
    print("\nGATE PASS — jedes Manifest ist klassifiziert")
    return 0


if __name__ == "__main__":
    sys.exit(main())

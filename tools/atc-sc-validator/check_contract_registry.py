#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Contract-Registry-Gate (ATC-STD-SC-019).

Haelt die Registry (SSOT) und die tatsaechlichen Contract-Quellen deckungsgleich.

Gates:
  A1 Pflichtfelder je Eintrag
  A2 contract_path existiert im Monorepo
  A3 source_sha256 stimmt mit der Datei (Drift-Erkennung)
  A4 keine Waisen: jede Contract-Quelle ist registriert
  A5 Status-Konsistenz (deployed braucht Netz + Deploy-Record, verified/audited
     brauchen Evidence, archived braucht Archivmarker)
  A6 contract_id eindeutig im Format ATC-SC-<KATEGORIE>-<NNN>
  A7 Kategorie im Enum nach SC-001
  A8 jede registrierte Quelle besteht `atc check` (ATCLang-Parser/Typpruefung)
  A9 die Vertrags-ABI der Quelle stimmt mit abi_sha256 ueberein (abgeleitete
     Schnittstelle driftet sonst unbemerkt; archivierte Eintraege ohne Wert)

Aufruf:
  python3 tools/atc-sc-validator/check_contract_registry.py
  python3 tools/atc-sc-validator/check_contract_registry.py --write-hashes
  python3 tools/atc-sc-validator/check_contract_registry.py --json
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REGISTRY = ROOT / "standards/contracts/registry/contracts.yaml"
DEPLOYMENTS = ROOT / "standards/contracts/registry/deployments.yaml"
CONTRACT_BASE = "apps/atc-contracts"

REQUIRED_FIELDS = (
    "contract_id",
    "name",
    "category",
    "version",
    "status",
    "language",
    "runtime",
    "owner",
    "upgradeable",
    "verified",
    "audited",
    "source_repository",
    "contract_path",
)
CATEGORIES = {
    "SC-CORE",
    "SC-IDENTITY",
    "SC-TOKEN",
    "SC-NFT",
    "SC-DEFI",
    "SC-GOVERNANCE",
    "SC-BRIDGE",
    "SC-ORACLE",
    "SC-GAMEFI",
    "SC-MINING",
    "SC-REGISTRY",
    "SC-SYSTEM",
}
STATUSES = {"development", "review", "audited", "deployed", "deprecated", "archived", "example"}
ID_PATTERN = re.compile(r"^ATC-SC-([A-Z]+)-(\d{3})$")


# --------------------------------------------------------------------------- IO


def parse_value(value: str):
    value = value.split("  #")[0].strip()
    if value in ("", "null", "~"):
        return None
    if value in ("true", "false"):
        return value == "true"
    if value.startswith('"') and value.endswith('"'):
        return value[1:-1]
    return value


def load_registry(path: Path) -> dict:
    """Minimaler YAML-Leser fuer diese Registry-Struktur (ohne PyYAML)."""
    data: dict = {"contracts": [], "exclusions": []}
    if not path.is_file():
        return data
    section = None
    current: dict | None = None
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.rstrip()
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        if re.match(r"^[a-z_]+:\s*$", line):
            section = line.split(":")[0]
            continue
        if section == "contracts":
            if line.startswith("  - "):
                current = {}
                data["contracts"].append(current)
                key, _, value = line[4:].partition(":")
            elif line.startswith("    ") and current is not None:
                key, _, value = line.strip().partition(":")
            else:
                continue
            current[key.strip()] = parse_value(value.strip())
        elif section == "exclusions" and line.startswith("  - "):
            data["exclusions"].append(parse_value(line[4:].strip()))
    return data


def sha256_of(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def deployment_ids(path: Path) -> set[str]:
    if not path.is_file():
        return set()
    return set(re.findall(r"contract_id:\s*\"?([A-Z0-9-]+)\"?", path.read_text(encoding="utf-8")))


def discover_sources(base: Path, exclusions: list[str], root: Path) -> dict[str, Path]:
    found: dict[str, Path] = {}
    if not base.is_dir():
        return found
    for path in sorted(base.rglob("*.atc")):
        if not path.is_file():
            continue
        rel = path.relative_to(root).as_posix()
        if any(re.fullmatch(pat.replace("**", ".*"), rel) for pat in exclusions):
            continue
        if "/tests/" in rel:
            continue
        found[rel] = path
    return found


# ------------------------------------------------------------------------ GATE


def compile_contract(atc_bin: Path, target: Path, timeout: int = 60) -> tuple[bool, str]:
    """ATCLang-Pruefung einer Quelle. Rueckgabe: (ok, Meldung)."""
    import subprocess

    try:
        proc = subprocess.run(
            [str(atc_bin), "check", str(target)],
            capture_output=True,
            text=True,
            timeout=timeout,
        )
    except (OSError, subprocess.SubprocessError) as exc:  # Binary fehlt/startet nicht
        return False, f"atc check nicht ausfuehrbar: {exc}"
    if proc.returncode == 0:
        return True, ""
    meldung = (proc.stdout + proc.stderr).strip().splitlines()
    return False, meldung[0][:120] if meldung else f"Exit {proc.returncode}"


def abi_digest(atc_bin: Path, target: Path, timeout: int = 60) -> str | None:
    """Sammel-Digest der Vertrags-ABI einer Quelle: (Hash, Vertragszahl)."""
    import subprocess

    try:
        proc = subprocess.run(
            [str(atc_bin), "abi", "--digest", str(target)],
            capture_output=True,
            text=True,
            timeout=timeout,
        )
    except (OSError, subprocess.SubprocessError):
        return None
    if proc.returncode != 0:
        return None
    line = proc.stdout.strip().splitlines()
    if not line:
        return None
    parts = line[0].split()
    return parts[0] if parts else None


def check_registry(
    root: Path,
    registry_path: Path,
    deployments_path: Path,
    atc_bin: Path | None = None,
    reg: dict | None = None,
) -> tuple[list[dict], dict]:
    # `reg` erlaubt dem Aufrufer, die nachgezogenen Hashes/ABIs anschliessend
    # zu schreiben; ohne ihn wird die Registry frisch geladen.
    if reg is None:
        reg = load_registry(registry_path)
    entries = reg["contracts"]
    findings: list[dict] = []
    ids_seen: dict[str, int] = {}
    registered_paths: set[str] = set()
    deployed_ids = deployment_ids(deployments_path)

    for idx, e in enumerate(entries, start=1):
        cid = str(e.get("contract_id") or f"<Eintrag {idx}>")

        # A1 Pflichtfelder
        missing = [f for f in REQUIRED_FIELDS if e.get(f) in (None, "")]
        if missing:
            findings.append({"gate": "A1", "id": cid, "detail": f"Pflichtfelder fehlen: {', '.join(missing)}"})

        # A6 Format + Eindeutigkeit
        m = ID_PATTERN.match(cid)
        if not m:
            findings.append({"gate": "A6", "id": cid, "detail": "ID nicht im Format ATC-SC-<KATEGORIE>-<NNN>"})
        else:
            num = int(m.group(2))
            if not 1 <= num <= 999:
                findings.append({"gate": "A6", "id": cid, "detail": "Nummer ausserhalb 001..999"})
            elif e.get("category") and str(e.get("category")) != f"SC-{m.group(1)}":
                findings.append(
                    {"gate": "A6", "id": cid, "detail": f"Kategorie {e.get('category')} passt nicht zur ID"}
                )
        ids_seen[cid] = ids_seen.get(cid, 0) + 1

        # A7 Kategorie
        if e.get("category") not in CATEGORIES:
            findings.append({"gate": "A7", "id": cid, "detail": f"unbekannte Kategorie: {e.get('category')}"})

        if e.get("status") not in STATUSES:
            findings.append({"gate": "A5", "id": cid, "detail": f"unbekannter Status: {e.get('status')}"})

        # A2 Pfad
        cpath = e.get("contract_path")
        if not cpath:
            continue
        registered_paths.add(str(cpath))
        target = root / str(cpath)
        if not target.is_file():
            findings.append({"gate": "A2", "id": cid, "detail": f"contract_path existiert nicht: {cpath}"})
            continue

        # A8 Kompilierbarkeit (ATCLang-Parser + Typpruefung)
        if atc_bin is not None:
            ok, meldung = compile_contract(atc_bin, target)
            if not ok:
                findings.append({"gate": "A8", "id": cid, "detail": f"atc check fehlgeschlagen: {meldung}"})

        # A9 ABI-Drift
        if atc_bin is not None:
            status = e.get("status")
            declared_abi = e.get("abi_sha256")
            actual_abi = abi_digest(atc_bin, target)
            if status == "archived":
                if declared_abi:
                    findings.append(
                        {"gate": "A9", "id": cid, "detail": "archivierter Eintrag traegt abi_sha256"}
                    )
                    e.pop("abi_sha256", None)
            elif actual_abi is not None:
                if declared_abi in (None, ""):
                    findings.append({"gate": "A9", "id": cid, "detail": "abi_sha256 fehlt"})
                    e["abi_sha256"] = actual_abi
                elif declared_abi != actual_abi:
                    findings.append(
                        {
                            "gate": "A9",
                            "id": cid,
                            "detail": f"ABI driftet: {declared_abi[:12]}… != {actual_abi[:12]}…",
                        }
                    )
                    e["abi_sha256"] = actual_abi

        # A3 Drift
        actual = sha256_of(target)
        declared = e.get("source_sha256")
        if declared in (None, ""):
            findings.append({"gate": "A3", "id": cid, "detail": "source_sha256 fehlt"})
            e["source_sha256"] = actual
        elif declared != actual:
            findings.append({"gate": "A3", "id": cid, "detail": f"Quelle driftet: {declared[:12]}… != {actual[:12]}…"})
            e["source_sha256"] = actual

        # A5 Status-Konsistenz
        status = e.get("status")
        if status == "deployed":
            if not e.get("network"):
                findings.append({"gate": "A5", "id": cid, "detail": "deployed ohne network"})
            if cid not in deployed_ids:
                findings.append({"gate": "A5", "id": cid, "detail": "deployed ohne Deploy-Record (ATC-DEP-NNNN)"})
        if status in ("development", "review", "example", "archived") and e.get("network"):
            findings.append({"gate": "A5", "id": cid, "detail": f"network gesetzt, obwohl Status {status}"})
        if e.get("verified") is True and status not in ("audited", "deployed"):
            findings.append({"gate": "A5", "id": cid, "detail": "verified=true ohne audited/deployed"})
        if e.get("audited") is True and not e.get("audit_ref"):
            findings.append({"gate": "A5", "id": cid, "detail": "audited=true ohne audit_ref"})
        if status == "archived":
            head = target.read_text(encoding="utf-8", errors="ignore")[:400].upper()
            if "ARCHIVED" not in head and "archive" not in str(cpath).lower():
                findings.append({"gate": "A5", "id": cid, "detail": "Status archived ohne Archivmarker im Kopf"})

    for cid, count in ids_seen.items():
        if count > 1:
            findings.append({"gate": "A6", "id": cid, "detail": f"ID {count}x vergeben"})

    # A4 Waisen
    sources = discover_sources(root / CONTRACT_BASE, reg["exclusions"], root)
    for rel in sources:
        if rel not in registered_paths:
            findings.append({"gate": "A4", "id": "-", "detail": f"nicht registrierte Contract-Quelle: {rel}"})

    stats = {"contracts": len(entries), "sources": len(sources), "exclusions": len(reg["exclusions"])}
    return findings, stats


# ----------------------------------------------------------------------- AUSGABE


def fmt(value) -> str:
    if value is True:
        return "true"
    if value is False:
        return "false"
    if value is None:
        return "null"
    text = str(value)
    if re.fullmatch(r"[A-Za-z0-9._/-]+", text):
        return text
    return '"' + text.replace('"', '\\"') + '"'


def write_registry(reg: dict, path: Path) -> None:
    lines = [
        "# Contract Registry — ATC-STD-SC-002/019 · SSOT aller Smart Contracts",
        "# Geprueft durch tools/atc-sc-validator/check_contract_registry.py (Gate A1-A9).",
        "# source_sha256 bindet jeden Eintrag an die Quelle, abi_sha256 an die ABI;",
        "# Drift in Quelle oder Schnittstelle laesst das Gate fehlschlagen.",
        "contracts:",
    ]
    for e in sorted(reg["contracts"], key=lambda x: str(x.get("contract_id"))):
        lines.append(f"  - contract_id: {e.get('contract_id')}")
        for key in REQUIRED_FIELDS[1:]:
            lines.append(f"    {key}: {fmt(e.get(key))}")
        for key in ("source_sha256", "abi_sha256", "network", "audit_ref", "note"):
            if e.get(key) not in (None, ""):
                lines.append(f"    {key}: {fmt(e.get(key))}")
        lines.append("")
    lines.append("exclusions:")
    for pat in reg["exclusions"]:
        lines.append(f"  - {fmt(pat)}")
    lines.append("")
    path.write_text("\n".join(lines), encoding="utf-8")


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--write-hashes", action="store_true", help="source_sha256 in der Registry aktualisieren")
    ap.add_argument("--write-abis", action="store_true", help="abi_sha256 in der Registry aktualisieren")
    ap.add_argument("--json", action="store_true", help="Befunde als JSON")
    ap.add_argument(
        "--atc",
        default=str(ROOT / "target/debug/atc"),
        help="Pfad zum atc-Binary fuer Gate A8 (Default: target/debug/atc)",
    )
    ap.add_argument("--skip-compile", action="store_true", help="Gate A8 (atc check) aussetzen")
    args = ap.parse_args(argv)

    if not REGISTRY.exists():
        print(f"FEHLER: Registry fehlt: {REGISTRY}")
        return 2

    atc_bin = None if args.skip_compile else Path(args.atc)
    if atc_bin is not None and not atc_bin.exists():
        print(f"FEHLER: atc-Binary nicht gefunden: {atc_bin} (mit --skip-compile aussetzen)")
        return 2

    findings, stats = check_registry(ROOT, REGISTRY, DEPLOYMENTS, atc_bin)

    drift = [f for f in findings if f["gate"] == "A3" and "fehlt" not in f["detail"]]
    if args.write_hashes and drift:
        reg = load_registry(REGISTRY)
        check_registry(ROOT, REGISTRY, DEPLOYMENTS, atc_bin, reg=reg)  # Hashes in-place nachziehen
        write_registry(reg, REGISTRY)
        findings, stats = check_registry(ROOT, REGISTRY, DEPLOYMENTS, atc_bin)

    abi_drift = [f for f in findings if f["gate"] == "A9"]
    if args.write_abis and abi_drift:
        reg = load_registry(REGISTRY)
        check_registry(ROOT, REGISTRY, DEPLOYMENTS, atc_bin, reg=reg)  # ABIs in-place nachziehen
        write_registry(reg, REGISTRY)
        findings, stats = check_registry(ROOT, REGISTRY, DEPLOYMENTS, atc_bin)

    if args.json:
        print(json.dumps({"findings": findings, **stats}, indent=2, ensure_ascii=False))
    else:
        print(f"Contract-Registry-Gate (SC-019): {stats['contracts']} Eintraege, {stats['sources']} Quellen")
        if not findings:
            print("  GATE PASS — Registry und Quellen sind deckungsgleich")
        for f in findings:
            print(f"  ✗ [{f['gate']}] {f['id']}: {f['detail']}")
        print(f"  Befunde: {len(findings)}")
    return 0 if not findings else 1


if __name__ == "__main__":
    sys.exit(main())

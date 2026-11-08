#!/usr/bin/env python3
import argparse
import hashlib
import json
import re
from pathlib import Path

SCHEMA = "ATC-DEP-ADVISORY-1"
SEVERITIES = {"critical", "high", "medium", "low", "unknown"}
ECO_MAP = {
    "crates.io": "cargo",
    "npm": "npm",
    "PyPI": "python",
    "Go": "go",
    "cargo": "cargo",
    "npmjs": "npm",
    "pypi": "python",
    "golang": "go",
}


def normalize_severity(item):
    raw = (item.get("database_specific") or {}).get("severity")
    return raw.lower() if isinstance(raw, str) and raw.lower() in SEVERITIES else "unknown"


def normalize(raw):
    out = []
    for item in raw if isinstance(raw, list) else raw.get("advisories", []):
        if not isinstance(item, dict):
            continue
        aliases = item.get("aliases") or []
        aid = item.get("id") or (aliases[0] if aliases else None)
        affected = []
        for a in item.get("affected") or []:
            package = a.get("package") or {}
            eco = ECO_MAP.get(package.get("ecosystem"), package.get("ecosystem"))
            pkg = package.get("name")
            if not eco or not pkg:
                continue
            ranges = []
            for r in a.get("ranges") or []:
                if not isinstance(r, dict):
                    continue
                events = []
                for ev in r.get("events") or []:
                    if isinstance(ev, dict):
                        events.append(
                            {
                                "introduced": ev.get("introduced"),
                                "fixed": ev.get("fixed"),
                                "last_affected": ev.get("last_affected"),
                            }
                        )
                if events:
                    ranges.append({"type": r.get("type"), "events": events})
            affected.append(
                {
                    "ecosystem": eco,
                    "package": pkg,
                    "ranges": ranges,
                    "versions": sorted(set(a.get("versions") or [])),
                }
            )
        if aid and affected:
            out.append(
                {
                    "id": aid,
                    "summary": item.get("summary", ""),
                    "severity": normalize_severity(item),
                    "affected": affected,
                    "aliases": sorted(set(aliases)),
                    "modified": item.get("modified"),
                    "published": item.get("published"),
                }
            )
    return sorted(out, key=lambda x: (x["id"], json.dumps(x, sort_keys=True)))


def validate(payload):
    errors = []
    if payload.get("schema") != SCHEMA:
        errors.append("invalid advisory schema")
    source = payload.get("source")
    if (
        not isinstance(source, dict)
        or source.get("format") != "OSV"
        or not re.fullmatch(r"[0-9a-f]{64}", str(source.get("input_sha256", "")))
    ):
        errors.append("missing or invalid OSV snapshot provenance")
    if (
        isinstance(source, dict)
        and "raw_response_sha256" in source
        and not re.fullmatch(r"[0-9a-f]{64}", str(source.get("raw_response_sha256", "")))
    ):
        errors.append("invalid raw OSV response provenance")
    advisories = payload.get("advisories")
    if not isinstance(advisories, list):
        return errors + ["advisories must be a list"]
    seen = set()
    for i, a in enumerate(advisories):
        aid = a.get("id")
        if not aid:
            errors.append(f"advisory[{i}] missing id")
        elif aid in seen:
            errors.append(f"duplicate advisory id: {aid}")
        else:
            seen.add(aid)
        if a.get("severity") not in SEVERITIES:
            errors.append(f"advisory[{i}] invalid severity")
        if not isinstance(a.get("affected"), list) or not a["affected"]:
            errors.append(f"advisory[{i}] missing affected")
        for j, x in enumerate(a.get("affected") or []):
            if x.get("ecosystem") not in {"cargo", "npm", "python", "go"}:
                errors.append(f"advisory[{i}].affected[{j}] unsupported ecosystem")
            if not x.get("package"):
                errors.append(f"advisory[{i}].affected[{j}] missing package")
            if not isinstance(x.get("ranges"), list) or not isinstance(x.get("versions"), list):
                errors.append(f"advisory[{i}].affected[{j}] invalid range/version lists")
            for k, r in enumerate(x.get("ranges") or []):
                if (
                    not isinstance(r, dict)
                    or not isinstance(r.get("type"), str)
                    or not r.get("type")
                    or not isinstance(r.get("events"), list)
                    or not r.get("events")
                ):
                    errors.append(f"advisory[{i}].affected[{j}].ranges[{k}] invalid type/events")
                    continue
                for e, ev in enumerate(r["events"]):
                    if not isinstance(ev, dict) or not any(
                        ev.get(name) is not None
                        for name in ("introduced", "fixed", "last_affected")
                    ):
                        errors.append(
                            f"advisory[{i}].affected[{j}].ranges[{k}].events[{e}] invalid event"
                        )
    return errors


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--input", required=True)
    ap.add_argument("--output", default="security/advisories.json")
    args = ap.parse_args()
    p = Path(args.input)
    raw = json.loads(p.read_text())
    advisories = normalize(raw)
    payload = {
        "schema": SCHEMA,
        "source": {"format": "OSV", "input_sha256": hashlib.sha256(p.read_bytes()).hexdigest()},
        "advisories": advisories,
    }
    errors = validate(payload)
    if errors:
        raise SystemExit("invalid normalized advisory snapshot: " + "; ".join(errors))
    Path(args.output).parent.mkdir(parents=True, exist_ok=True)
    Path(args.output).write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(
        json.dumps(
            {
                "schema": SCHEMA,
                "advisories": len(advisories),
                "input_sha256": payload["source"]["input_sha256"],
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()

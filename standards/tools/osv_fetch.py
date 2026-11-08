#!/usr/bin/env python3
import argparse
import hashlib
import json
import urllib.error
import urllib.request
from pathlib import Path

from osv_snapshot import normalize as normalize_snapshot
from osv_snapshot import validate

ECO = {"cargo": "crates.io", "npm": "npm", "python": "PyPI", "go": "Go"}


def cvss_severity(score):
    try:
        s = float(score)
    except (TypeError, ValueError):
        return None
    if 9.0 <= s <= 10.0:
        return "critical"
    if 7.0 <= s < 9.0:
        return "high"
    if 4.0 <= s < 7.0:
        return "medium"
    if 0.0 <= s < 4.0:
        return "low"
    return None


def normalize_severity(item):
    raw = (item.get("database_specific") or {}).get("severity")
    if isinstance(raw, str) and raw.lower() in {"critical", "high", "medium", "low", "unknown"}:
        return raw.lower()
    candidates = []
    for entry in item.get("severity") or []:
        if not isinstance(entry, dict):
            continue
        score = entry.get("score")
        if isinstance(score, (int, float)):
            candidates.append(float(score))
        elif isinstance(score, str):
            try:
                candidates.append(float(score))
            except ValueError:
                pass
    mapped = [cvss_severity(x) for x in candidates]
    order = {"critical": 4, "high": 3, "medium": 2, "low": 1}
    return max((x for x in mapped if x), key=lambda x: order[x], default="unknown")


def normalize(raw):
    items = (
        raw if isinstance(raw, list) else raw.get("advisories", []) if isinstance(raw, dict) else []
    )
    out = []
    for item in items:
        if not isinstance(item, dict):
            continue
        aliases = item.get("aliases") or []
        aid = item.get("id") or (aliases[0] if aliases else None)
        affected = []
        for a in item.get("affected") or []:
            if not isinstance(a, dict):
                continue
            package = a.get("package") or {}
            eco = ECO_MAP.get(package.get("ecosystem"), package.get("ecosystem"))
            pkg = package.get("name")
            if not eco or not pkg:
                continue
            ranges = []
            for r in a.get("ranges") or []:
                if not isinstance(r, dict):
                    continue
                events = [
                    {
                        "introduced": ev.get("introduced"),
                        "fixed": ev.get("fixed"),
                        "last_affected": ev.get("last_affected"),
                    }
                    for ev in r.get("events") or []
                    if isinstance(ev, dict)
                ]
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


def validate_raw_response(raw, query_count):
    errors = []
    if not isinstance(raw, dict):
        return ["OSV response must be a JSON object"]
    results = raw.get("results")
    if not isinstance(results, list):
        return ["OSV response missing results list"]
    if len(results) != query_count:
        errors.append(f"OSV results/query count mismatch: {len(results)} != {query_count}")
    for i, result in enumerate(results):
        if not isinstance(result, dict):
            errors.append(f"OSV results[{i}] must be an object")
            continue
        vulns = result.get("vulns", [])
        if not isinstance(vulns, list):
            errors.append(f"OSV results[{i}].vulns must be a list")
        elif any(not isinstance(v, dict) for v in vulns):
            errors.append(f"OSV results[{i}].vulns contains non-object")
    return errors


def fetch_json(endpoint, body):
    req = urllib.request.Request(
        endpoint,
        data=body,
        headers={"Content-Type": "application/json", "Accept": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            status = getattr(resp, "status", 200)
            if status < 200 or status >= 300:
                raise RuntimeError(f"OSV HTTP status {status}")
            return json.load(resp)
    except urllib.error.HTTPError as e:
        raise RuntimeError(f"OSV HTTP error {e.code}") from e
    except urllib.error.URLError as e:
        raise RuntimeError(f"OSV network error: {e.reason}") from e
    except json.JSONDecodeError as e:
        raise RuntimeError(f"OSV response is not valid JSON: {e}") from e


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--graph", default="artifacts/dependency/dependency-graph.json")
    ap.add_argument("--raw-output", default="artifacts/dependency/osv-response.json")
    ap.add_argument("--query-output", default="artifacts/dependency/osv-query.json")
    ap.add_argument("--advisory-output", default="security/advisories.json")
    ap.add_argument("--endpoint", default="https://api.osv.dev/v1/querybatch")
    args = ap.parse_args()
    try:
        graph = json.loads(Path(args.graph).read_text())
    except Exception as e:
        raise SystemExit(f"invalid dependency graph: {e}")
    if graph.get("schema") != "ATC-DEP-GRAPH-1":
        raise SystemExit("invalid dependency graph schema")
    queries = sorted(
        [
            {
                "package": {"name": n["name"], "ecosystem": ECO[n["ecosystem"]]},
                "version": n["version"],
            }
            for n in graph.get("nodes", [])
            if n.get("ecosystem") in ECO and n.get("name") and n.get("version")
        ],
        key=lambda q: (q["package"]["ecosystem"], q["package"]["name"], q["version"]),
    )
    if not queries:
        raise SystemExit(
            "OSV fetch blocked: dependency graph contains no supported versioned dependencies"
        )
    body = json.dumps({"queries": queries}, sort_keys=True, separators=(",", ":")).encode()
    Path(args.query_output).parent.mkdir(parents=True, exist_ok=True)
    Path(args.query_output).write_bytes(body)
    raw = fetch_json(args.endpoint, body)
    errors = validate_raw_response(raw, len(queries))
    if errors:
        raise SystemExit("invalid OSV response: " + "; ".join(errors))
    raw_bytes = (json.dumps(raw, sort_keys=True, separators=(",", ":")) + "\n").encode()
    raw_sha = hashlib.sha256(raw_bytes).hexdigest()
    Path(args.raw_output).parent.mkdir(parents=True, exist_ok=True)
    Path(args.raw_output).write_bytes(raw_bytes)
    advisories = []
    for result in raw["results"]:
        advisories.extend(result.get("vulns", []))
    query_sha256 = hashlib.sha256(body).hexdigest()
    payload = {
        "schema": "ATC-DEP-ADVISORY-1",
        "source": {
            "format": "OSV",
            "endpoint": args.endpoint,
            "input_sha256": query_sha256,
            "raw_response_sha256": raw_sha,
        },
        "advisories": normalize_snapshot(advisories),
    }
    errors = validate(payload)
    if errors:
        raise SystemExit("invalid OSV snapshot: " + "; ".join(errors))
    Path(args.advisory_output).parent.mkdir(parents=True, exist_ok=True)
    Path(args.advisory_output).write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(
        json.dumps(
            {
                "queries": len(queries),
                "advisories": len(payload["advisories"]),
                "raw_response_sha256": raw_sha,
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()

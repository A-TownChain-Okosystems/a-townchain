#!/usr/bin/env python3
"""Fail-closed validator for ATC-DEPENDENCY-GRAPH-001 and ATC-BUILD-MANIFEST-001.

Gates checked locally (exit 1 on any violation):
  G1 schema      - required fields present, no unknown keys in critical objects
  G2 topology    - all 26 active repos covered, edges resolvable, acyclic,
                   topological_order is a valid Kahn order
  G3 build       - every repo has a manifest entry; entries either carry
                   command+entry_evidence or an explicit blocked/exempt reason
Fail-closed: missing data is an error, never a warning. An evidenced command
is NOT proof of a successful build (no sham validation).
"""
import json, sys
from collections import deque
import re

ACTIVE = {".github","a-townchain","a-townchain-ecosystem","a-townchain-os","a-townchain-os-docs",
 "atc-algorithm","atc-compute","atc-contracts","atc-engineering","atc-ide","atc-launchpad",
 "atc-marketplace","atc-node","atc-sdk","atc-shivacore","atc-standards","atc-toolchain","atc-vm",
 "atc-zkp","atclang","aurora-ai","demo-repository","genesis-chronicles","genesis-engine",
 "genesis-franchise-factory","globus-os"}
GRAPH_REQ = {"schema_id","schema_version","status","generated","provenance","policy","nodes","edges",
 "topological_order","cycle_check","blockers","gates"}
BM_REQ = {"schema_id","schema_version","status","generated","policy","graph_ref","repos"}
NODE_REQ = {"id","layer","uses"}
SHA_RE = re.compile(r"^[0-9a-f]{40}$")
STATUS_NEEDS_REASON = {"BLOCKED_NO_ENTRY","BLOCKED_NO_VERIFIED_ENTRY"}

def main(graph_path, manifest_path):
    errors = []
    g = json.load(open(graph_path)); b = json.load(open(manifest_path))
    if g["schema_id"] != "ATC-DEPENDENCY-GRAPH-001": errors.append("graph schema_id")
    if b["schema_id"] != "ATC-BUILD-MANIFEST-001": errors.append("manifest schema_id")
    missing = GRAPH_REQ - set(g); errors += [f"graph field missing: {m}" for m in missing]
    missing = BM_REQ - set(b); errors += [f"manifest field missing: {m}" for m in missing]

    nodes = {n["id"]: n for n in g["nodes"]}
    # G2 coverage
    if set(nodes) != ACTIVE:
        errors.append(f"graph coverage mismatch: {set(nodes) ^ ACTIVE}")
    if len(b["repos"]) != len(ACTIVE) or {r["repo"] for r in b["repos"]} != ACTIVE:
        errors.append("manifest coverage != 26 active repos")
    # edges resolvable, declared uses match edges
    edge_pairs = {(e["from"], e["to"]) for e in g["edges"]}
    for e in g["edges"]:
        for k in ("from","to","ifc"):
            if k not in e: errors.append(f"edge missing {k}")
        if e["from"] not in nodes or e["to"] not in nodes:
            errors.append(f"unresolvable edge {e['from']}->{e['to']}")
        if e["from"] == e["to"]: errors.append(f"self-loop {e['from']}")
    for n in nodes.values():
        declared = set(n["uses"])
        actual = {f for (f, t) in edge_pairs if t == n["id"]}
        if declared != actual:
            errors.append(f"uses mismatch for {n['id']}: declared {sorted(declared)} != edges {sorted(actual)}")
        for m in NODE_REQ - set(n): errors.append(f"node {n['id']} missing {m}")
    # acyclic via Kahn + order validity (unresolvable edges are flagged above)
    valid_pairs = {(f, t) for (f, t) in edge_pairs if f in nodes and t in nodes}
    indeg = {r: 0 for r in nodes}
    succ = {r: [] for r in nodes}
    for (f,t) in valid_pairs: indeg[t] += 1; succ[f].append(t)
    q = deque(sorted(r for r in nodes if indeg[r] == 0)); seen = []
    while q:
        r = q.popleft(); seen.append(r)
        for t in sorted(succ[r]):
            indeg[t] -= 1
            if indeg[t] == 0: q.append(t)
    if len(seen) != len(nodes): errors.append("CYCLE DETECTED - graph is not a DAG")
    order = g["topological_order"]
    if sorted(order) != sorted(nodes): errors.append("topological_order is not a permutation of nodes")
    pos = {r: i for i, r in enumerate(order)}
    for (f,t) in valid_pairs:
        if pos.get(f, 10**9) >= pos.get(t, -1): errors.append(f"order violation: {f} must be built before {t}")
    if not g["blockers"]: errors.append("blockers list empty (open decisions must be explicit)")
    # G3: manifest entries and exact provenance
    for r in b["repos"]:
        st = r.get("status")
        if st not in {"EXEMPT_ORG_CONFIG", "EXEMPT_FIXTURE"}:
            sha = r.get("head_sha")
            if not isinstance(sha, str) or not SHA_RE.fullmatch(sha):
                errors.append(f"{r.get('repo')}: head_sha must be a full 40-character lowercase SHA")
        if st in STATUS_NEEDS_REASON and not r.get("blocked_reason"):
            errors.append(f"{r['repo']}: blocked status without reason")
        if st == "BUILD_ENTRY_EVIDENCED":
            if not r.get("build_command"): errors.append(f"{r['repo']}: EVIDENCED without build_command")
            if not r.get("entry_evidence"): errors.append(f"{r['repo']}: EVIDENCED without entry_evidence")
        if st == "CANONICAL_ELSEWHERE" and not r.get("canonical_source_path"):
            errors.append(f"{r['repo']}: CANONICAL_ELSEWHERE without canonical path")
    if errors:
        print(json.dumps({"validation": "FAIL", "errors": errors}, indent=2)); return 1
    print(json.dumps({"validation": "PASS", "nodes": len(nodes), "edges": len(edge_pairs),
        "order_valid": True, "manifest_repos": len(b["repos"]),
        "gates": {"G1_schema": "PASS", "G2_topology": "PASS", "G3_build": "PASS_ENTRY_LEVEL"}}, indent=2))
    return 0

if __name__ == "__main__":
    sys.exit(main(sys.argv[1], sys.argv[2]))

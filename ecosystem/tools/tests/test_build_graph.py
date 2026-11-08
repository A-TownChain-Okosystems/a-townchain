import json, subprocess, sys, os
HERE = os.path.dirname(__file__)
VALIDATOR = os.path.join(HERE, "..", "validate_build_graph.py")
GRAPH = os.path.join(HERE, "..", "..", "docs", "architecture", "ATC-DEPENDENCY-GRAPH-001.json")
BM = os.path.join(HERE, "..", "..", "docs", "build", "ATC-BUILD-MANIFEST-001.json")

def run(g=GRAPH, b=BM):
    return subprocess.run([sys.executable, VALIDATOR, g, b], capture_output=True, text=True)

def write(tmp, obj, name):
    p = os.path.join(tmp, name); json.dump(obj, open(p, "w")); return p

def test_live_artifacts_pass():
    r = run(); assert r.returncode == 0, r.stdout

def test_cycle_detected(tmp_path):
    g = json.load(open(GRAPH)); g["edges"].append({"from": "a-townchain-os", "to": "atclang", "ifc": "IFC-0001"})
    gp = write(tmp_path, g, "g.json"); r = run(gp, BM)
    assert r.returncode == 1 and "CYCLE" in r.stdout

def test_order_violation_detected(tmp_path):
    g = json.load(open(GRAPH)); g["topological_order"] = g["topological_order"][::-1]
    gp = write(tmp_path, g, "g.json"); r = run(gp, BM)
    assert r.returncode == 1 and "order violation" in r.stdout

def test_blocked_without_reason_fails(tmp_path):
    b = json.load(open(BM))
    for x in b["repos"]:
        if x["repo"] == "atc-toolchain": x.pop("blocked_reason", None)
    bp = write(tmp_path, b, "b.json"); r = run(GRAPH, bp)
    assert r.returncode == 1 and "without reason" in r.stdout

def test_coverage_gap_fails(tmp_path):
    g = json.load(open(GRAPH)); g["nodes"] = [n for n in g["nodes"] if n["id"] != "atclang"]
    gp = write(tmp_path, g, "g.json"); r = run(gp, BM)
    assert r.returncode == 1 and "coverage" in r.stdout


def test_short_head_sha_fails(tmp_path):
    b = json.load(open(BM))
    for x in b["repos"]:
        if x["repo"] == "a-townchain":
            x["head_sha"] = "a70b16f2d"
    bp = write(tmp_path, b, "b.json"); r = run(GRAPH, bp)
    assert r.returncode == 1 and "full 40-character" in r.stdout

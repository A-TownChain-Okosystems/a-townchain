#!/usr/bin/env python3
"""check_cross_registry.py — Cross-Registry-Konsistenztest (SCR-0091, Owner-Bereinigungsplan).

Erzwingt die SSOT-Kette registry/standards.yaml -> registry.lock -> Views
-> Cross-Registry-Validierung. Bei JEGLICHER Abweichung: Exit-Code 1.

Regeln:
  R1  jede Standard-ID ist eindeutig
  R2  jede Registry-ID besitzt genau eine kanonische Datei (Ausnahme: dokumentiertes EXEMPT)
  R3  jede kanonische Standarddatei besitzt einen Registry-Eintrag
  R4  Versionen stimmen ueberein (Registry <-> Standarddatei-Frontmatter)
  R5  Statuswerte stimmen ueberein (Registry <-> Standarddatei-Frontmatter)
  R6  Taxonomie und Registry sind synchron
  R7  keine View enthaelt widerspruechliche aktuelle Zahlen (State-Block == Ist)
  R8  APPROVED wird nicht mit IMPLEMENTED gleichgesetzt (KPI-Trennung vorhanden)
  R9  keine View enthaelt veraltete hartcodierte Governance-Daten
      (ATC-STD-000-Version/Status in Views == Registry)
  R10 generierte Views sind reproduzierbar (Generator-Idempotenz)
"""

import hashlib
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
REG = os.path.join(ROOT, "registry", "standards.yaml")
FAILS = []


def fail(rule, msg):
    FAILS.append(f"[{rule}] {msg}")


def _vge(a, b):
    return tuple(int(x) for x in a.split(".")) >= tuple(int(x) for x in b.split("."))


def main():
    # Registry laden
    reg_text = open(REG, encoding="utf-8").read()
    entries, ids = [], []
    for line in reg_text.splitlines():
        m = re.search(r"- \{id: (ATC-\S+?),.*?version: \"?([\d.]+)\"?, status: (\w+)", line)
        if m:
            entries.append(m.group(1))
            ids.append(
                {"id": m.group(1), "version": m.group(2), "status": m.group(3), "line": line}
            )
    reg_sha = hashlib.sha256(open(REG, "rb").read()).hexdigest()

    # R1: Eindeutigkeit
    dup = {i for i in entries if entries.count(i) > 1}
    if dup:
        fail("R1", "Doppelte Registry-IDs: " + ", ".join(sorted(dup)))
    print(f"R1 Eindeutigkeit: {len(entries)} Eintraege, {len(dup)} Duplikate")

    # R2/R3: Registry <-> Dateien
    std_dir = os.path.join(ROOT, "standards")
    reg_by_id = {e["id"]: e for e in ids}
    files = []
    for dp, _, fs in os.walk(std_dir):
        for f in fs:
            if (
                f.endswith(".md") and f != "INDEX.md"
            ):  # INDEX.md = generierter Familien-Navigationsindex (MAINT-000 §25), kein Standard
                files.append((os.path.join(dp, f), os.path.splitext(f)[0]))
    file_ids = {fid for _, fid in files}
    no_file, masters = [], []
    for e in ids:
        i = e["id"]
        if i in file_ids:
            continue
        mf = re.search(r"file: (\S+?),", e["line"])
        if mf and os.path.exists(os.path.join(ROOT, mf.group(1))):
            masters.append(i)
            continue
        hits = [
            p
            for p in ("governance", "governance/constitution")
            if os.path.exists(os.path.join(ROOT, p, i + ".md"))
        ]
        if os.path.exists(os.path.join(ROOT, "registry", "standards", i + ".yaml")):
            masters.append(i)
            continue
        if hits:
            masters.append(i)
            continue
        no_file.append(i)
    no_entry = [fid for _, fid in files if fid not in reg_by_id]
    if no_file:
        fail("R2", "Registry-IDs ohne kanonische Datei: " + ", ".join(no_file[:10]))
    if no_entry:
        fail("R3", "Standarddateien ohne Registry-Eintrag: " + ", ".join(no_entry[:10]))
    print(
        f"R2/R3 Datei-Relation: {len(no_file)} ohne Datei, {len(masters)} Master-Dokumente ausserhalb, {len(no_entry)} ohne Eintrag"
    )

    # R4/R5: Version/Status Registry <-> Frontmatter
    v_drift = s_drift = 0
    for path, fid in files:
        e = reg_by_id.get(fid)
        if not e:
            continue
        head = open(path, encoding="utf-8").read(4000)
        mv = re.search(r'version:\s*"?([\d.]+)"?', head)
        ms = re.search(r"status:\s*(\w+)", head)
        if mv and mv.group(1) != e["version"]:
            v_drift += 1
            if v_drift <= 5:
                fail("R4", f"{fid}: Registry v{e['version']} vs Datei v{mv.group(1)}")
        if ms and ms.group(1).lower() != e["status"].lower():
            s_drift += 1
            if s_drift <= 5:
                fail("R5", f"{fid}: Registry {e['status']} vs Datei {ms.group(1)}")
    print(f"R4/R5 Version/Status-Drift: {v_drift}/{s_drift}")

    # R6: Taxonomie-Sync
    import yaml

    tax = yaml.safe_load(open(os.path.join(ROOT, "registry", "taxonomy.yaml"), encoding="utf-8"))
    reg_yaml = yaml.safe_load(open(REG, encoding="utf-8"))
    std_list = reg_yaml.get("standards", [])
    fam_by_name = {}
    for d in tax["taxonomy"]["domains"]:
        for f_ in d["families"]:
            fam_by_name[f_["name"]] = f_
    reg_counts = {}
    unmatched = []
    for e in std_list:
        cat = e.get("category", "?")
        key = (
            cat + " (familie)"
            if (cat in fam_by_name and (cat + " (familie)") in fam_by_name)
            else cat
        )
        reg_counts[key] = reg_counts.get(key, 0) + 1
        if key not in fam_by_name:
            unmatched.append(e.get("id", "?"))
    if unmatched:
        fail(
            "R6",
            "Registry-Standards ohne Taxonomie-Familie: " + ", ".join(sorted(set(unmatched))[:10]),
        )
    for name, f_ in fam_by_name.items():
        if f_.get("standards_count", 0) != reg_counts.get(name, 0):
            fail(
                "R6",
                f"Taxonomie-Drift Familie '{name}': {f_.get('standards_count')} != Registry {reg_counts.get(name, 0)}",
            )
    print(f"R6 Taxonomie: {len(fam_by_name)} Familien gespiegelt, {len(unmatched)} ohne Zuordnung")

    # R7: State-Block in README == Ist
    rd = open(os.path.join(ROOT, "README.md"), encoding="utf-8").read()
    m = re.search(r"standards_total: (\d+)", rd)
    if not m:
        fail("R7", "README State-Block fehlt")
    elif int(m.group(1)) != len(entries):
        fail("R7", f"README standards_total {m.group(1)} != Registry {len(entries)}")
    m = re.search(r'registry_sha256: "([0-9a-f]{16,64})"', rd)
    if not m:
        fail("R7", "README registry_sha256 fehlt")
    elif m.group(1) not in reg_sha:
        fail("R7", "README registry_sha256 != aktueller Registry-SHA")
    m = re.search(r"standards_approved: (\d+)", rd)
    n_app = sum(1 for e in ids if e["status"] == "approved")
    if m and int(m.group(1)) != n_app:
        fail("R7", f"README approved {m.group(1)} != Registry {n_app}")
    print(f"R7 State-Block: total/approved geprueft gegen Registry ({len(entries)}/{n_app})")

    # R8: KPI-Trennung
    if "Standards implementiert" in rd and "unzulässig" not in rd and "unzulaessig" not in rd:
        fail("R8", "README identifiziert APPROVED-als-IMPLEMENTED-Aussage nicht als unzulaessig")
    if "specification-only" not in rd and "specification_only" not in rd:
        fail("R8", "Implementierungs-KPI ohne specification-only-Trennung")
    print("R8 KPI-Trennung: geprueft")

    # R9: keine veraltete hartcodierte Governance-Daten in Views
    e000 = reg_by_id.get("ATC-STD-000", {})
    for view in ["README.md", "AGENT_MANIFEST.md", "STATUS.md"]:
        vt = open(os.path.join(ROOT, view), encoding="utf-8").read()
        if view == "STATUS.md":
            cut = vt.find("\n## ")
            vt = vt[:cut] if cut > 0 else vt  # Audit-Trail = historische Doku, erlaubt (SCR-0091)
        for mm in re.finditer(r"ATC-STD-000[^\n]{0,80}?v(\d+\.\d+\.\d+)", vt):
            if e000 and mm.group(1) != e000["version"]:
                fail("R9", f"{view}: ATC-STD-000 v{mm.group(1)} != Registry v{e000['version']}")
    print("R9 Hartcode-Drift in Views: geprueft")

    # R10: Reproduzierbarkeit (Generator idempotent bis auf Zeitstempel)
    before = open(os.path.join(ROOT, "README.md"), encoding="utf-8").read()
    r = subprocess.run(
        [sys.executable, os.path.join(ROOT, "tools", "gen_views", "generate_views.py")],
        capture_output=True,
        text=True,
    )
    after = open(os.path.join(ROOT, "README.md"), encoding="utf-8").read()

    def norm(t):
        return re.sub(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}( UTC\+2)?", "TS", t)

    if r.returncode != 0 or norm(before) != norm(after):
        fail("R10", "generate_views.py nicht idempotent reproduzierbar")
    print("R10 Reproduzierbarkeit: geprueft")

    # R11: ATC-STD-000-Bootstrap-Sunset technisch erzwungen (Owner-Audit 11.09.2026 P0-2)
    std003 = open(
        os.path.join(ROOT, "standards", "governance", "ATC-STD-003.md"), encoding="utf-8"
    ).read()
    msun = re.search(r"Sunset mit ATC-STD-000 v([\d.]+)", std003)
    if not msun:
        fail(
            "R11",
            "ATC-STD-003 par.6: keine maschinenlesbare Sunset-Version (Sunset mit ATC-STD-000 vX.Y.Z)",
        )
    else:
        sunset = msun.group(1)
        e000 = reg_by_id.get("ATC-STD-000", {})
        cur = e000.get("version", "0")
        if _vge(cur, sunset):
            f000 = os.path.join(ROOT, "governance", "ATC-STD-000.md")
            if not os.path.exists(f000):
                fail("R11", "ATC-STD-000: Datei fehlt - EXEMPT endet mit v" + sunset)
            else:
                head = open(f000, encoding="utf-8").read(4000)
                mv = re.search(r'version:\s*"?([\d.]+)"?', head)
                mst = re.search(r"status:\s*(\w+)", head)
                if mv and mv.group(1) != cur:
                    fail("R11", "ATC-STD-000 Datei v" + mv.group(1) + " != Registry v" + cur)
                if mst and mst.group(1).lower() != e000.get("status", "").lower():
                    fail(
                        "R11",
                        "ATC-STD-000 Datei-Status "
                        + mst.group(1)
                        + " != Registry "
                        + str(e000.get("status")),
                    )
        if (
            _vge(cur, sunset)
            and tuple(int(x) for x in cur.split(".")) > tuple(int(x) for x in sunset.split("."))
            and e000.get("status") != "approved"
        ):
            fail(
                "R11",
                "ATC-STD-000 v"
                + cur
                + " ueberschreitet Sunset v"
                + sunset
                + " ohne APPROVED - par.9-Freigabe (Owner) fehlt",
            )
        print("R11 ATC-STD-000-Sunset: Sunset v" + sunset + ", Registry v" + cur + " - geprueft")

    # R12: Drei-Stufen-Compliance-State + Matrix-Abdeckung (SCR-0094, Owner-Audit P1-01)
    cs_path = os.path.join(ROOT, "registry", "compliance_state.yaml")
    if not os.path.exists(cs_path):
        fail("R12", "registry/compliance_state.yaml fehlt (SCR-0094)")
    else:
        cs = yaml.safe_load(open(cs_path, encoding="utf-8")) or {}
        if cs.get("formal_compliance") != "PASS":
            fail(
                "R12",
                "formal_compliance != PASS - formal PASS ist nur behauptbar, wenn dieser Test selbst gruen ist",
            )
        try:
            _impl = yaml.safe_load(
                open(
                    os.path.join(ROOT, "registry", "standard-implementation.yaml"), encoding="utf-8"
                )
            )
            _reg = yaml.safe_load(open(REG, encoding="utf-8"))
            _kk = {}
            for _s in _impl.get("standards", []):
                _st = _s.get("implementation", {}).get("status", "specification_only")
                _kk[_st] = _kk.get(_st, 0) + 1
            _tot = sum(_kk.values())
            _reg_n = len(_reg.get("standards", []))
            if _tot != _reg_n:
                fail(
                    "R12",
                    "Implementierungs-Matrix deckt "
                    + str(_tot)
                    + " Standards ab, Registry hat "
                    + str(_reg_n)
                    + " - Matrix ergaenzen",
                )
            exp = (
                "PASS"
                if _kk.get("specification_only", 0) == 0
                else (
                    "PARTIAL" if _kk.get("enforced", 0) + _kk.get("implemented", 0) > 0 else "NONE"
                )
            )
            if cs.get("implementation_compliance") != exp:
                fail(
                    "R12",
                    "implementation_compliance '"
                    + str(cs.get("implementation_compliance"))
                    + "' != abgeleitet '"
                    + exp
                    + "' (KPI "
                    + str(_kk)
                    + ")",
                )
        except Exception as _e:
            fail("R12", "standard-implementation.yaml nicht lesbar: " + str(_e))
        ready_ok = False
        for _rf in ("milestones.yaml", "releases.yaml"):
            try:
                _doc = yaml.safe_load(open(os.path.join(ROOT, "registry", _rf), encoding="utf-8"))
                _walk = (
                    [_doc] if isinstance(_doc, dict) else (_doc if isinstance(_doc, list) else [])
                )
                while _walk:
                    _d = _walk.pop(0)
                    if isinstance(_d, dict):
                        _idt = (
                            str(_d.get("id", ""))
                            + str(_d.get("title", ""))
                            + str(_d.get("name", ""))
                        )
                        if str(_d.get("status", "")).upper() == "ACCEPTED" and re.search(
                            r"(?i)mainnet|release", _idt
                        ):
                            ready_ok = True
                        _walk.extend(v for v in _d.values() if isinstance(v, (dict, list)))
                    elif isinstance(_d, list):
                        _walk.extend(_d)
            except FileNotFoundError:
                pass
        if cs.get("production_readiness") not in ("NOT_READY", "TESTNET_READY", "PRODUCTION_READY"):
            fail(
                "R12",
                "production_readiness ungueltiger Wert: " + str(cs.get("production_readiness")),
            )
        if cs.get("production_readiness") not in ("NOT_READY", None) and not ready_ok:
            fail(
                "R12",
                "production_readiness != NOT_READY ohne ACCEPTED Mainnet-/Release-Meilenstein - von Agenten nicht frei setzbar",
            )
        print(
            "R12 Drei-Stufen-Compliance: formal="
            + str(cs.get("formal_compliance"))
            + ", implementation="
            + str(cs.get("implementation_compliance"))
            + ", production="
            + str(cs.get("production_readiness"))
        )

    # R13: Per-Standard-Metadaten (ATC-STD-LIB-001 Phase 1, SCR-0100)
    reg_data = yaml.safe_load(open(REG, encoding="utf-8")).get("standards", [])
    try:
        _impl = {
            e.get("id"): e
            for e in yaml.safe_load(
                open(
                    os.path.join(ROOT, "registry", "standard-implementation.yaml"), encoding="utf-8"
                )
            ).get("standards", [])
        }
    except Exception:
        _impl = {}
    _expected = {}
    for _s in reg_data:
        _f = _s.get("file")
        if _f:
            _expected[_s["id"]] = (
                os.path.join(ROOT, os.path.dirname(_f), _s["id"] + ".metadata.yaml"),
                _s,
            )
    _found = 0
    for _sid, (_path, _s) in _expected.items():
        if not os.path.exists(_path):
            fail("R13", "Metadaten fehlen: " + _sid + " (" + os.path.basename(_path) + ")")
            continue
        _m = yaml.safe_load(open(_path, encoding="utf-8")) or {}
        if (
            str(_m.get("version", "")) != str(_s.get("version", ""))
            or _m.get("status") != _s.get("status")
            or _m.get("title") != _s.get("title")
        ):
            fail(
                "R13",
                "Metadaten-Drift: "
                + _sid
                + " (Registry version="
                + str(_s.get("version"))
                + " status="
                + str(_s.get("status"))
                + ")",
            )
        _im = (_m.get("implementation") or {}).get("status", "specification_only")
        _is = (_impl.get(_sid) or {}).get("implementation", {}).get("status", "specification_only")
        if _im != _is:
            fail(
                "R13",
                "Implementierungs-Drift in Metadaten: "
                + _sid
                + " ("
                + str(_im)
                + " != "
                + str(_is)
                + ")",
            )
        _found += 1
    import glob as _glob

    _orphans = [
        p
        for p in _glob.glob(
            os.path.join(ROOT, "standards", "**", "*.metadata.yaml"), recursive=True
        )
        + _glob.glob(os.path.join(ROOT, "governance", "**", "*.metadata.yaml"), recursive=True)
        if os.path.basename(p).replace(".metadata.yaml", "") not in _expected
    ]
    for _o in _orphans:
        fail(
            "R13", "Metadaten-Datei ohne Registry-Eintrag (Waiskind): " + os.path.relpath(_o, ROOT)
        )
    print(
        "R13 Per-Standard-Metadaten: "
        + str(_found)
        + " geprueft, "
        + str(len(_orphans))
        + " Waiskinder"
    )

    # R14: Standards-Profile (ATC-STD-LIB-001 sec.8, SCR-0101)
    _reg_all = yaml.safe_load(
        open(os.path.join(ROOT, "registry", "repositories.yaml"), encoding="utf-8")
    )
    _std_ids = {
        s["id"]: s for s in yaml.safe_load(open(REG, encoding="utf-8")).get("standards", [])
    }
    _profiles_dir = os.path.join(ROOT, "profiles")
    _pfiles = (
        sorted([f for f in os.listdir(_profiles_dir) if f.endswith(".yaml")])
        if os.path.isdir(_profiles_dir)
        else []
    )
    _regnames = {e["name"]: e for e in _reg_all.get("repositories", [])}
    for _pf in _pfiles:
        _p = yaml.safe_load(open(os.path.join(_profiles_dir, _pf), encoding="utf-8")) or {}
        _pn = _p.get("profile")
        if _pn not in _regnames:
            fail("R14", "Profile ohne Registry-Repo: " + str(_pn))
            continue
        _e = _regnames[_pn]
        if (
            _p.get("registry_id") != _e.get("id")
            or _p.get("domain") != _e.get("domain")
            or _p.get("criticality") != _e.get("criticality")
        ):
            fail("R14", "Profile-Drift gegen repositories.yaml: " + str(_pn))
        _req = _p.get("required_standards") or []
        if not _req:
            fail("R14", "required_standards leer: " + str(_pn))
        for _rs in _req:
            if _rs not in _std_ids:
                fail(
                    "R14", "Profile " + str(_pn) + " referenziert unbekannten Standard: " + str(_rs)
                )
            elif _std_ids[_rs].get("status") != "approved":
                fail("R14", "Profile " + str(_pn) + " bindet nicht-APPROVED Standard: " + str(_rs))
    for _rn in _regnames:
        if not os.path.exists(os.path.join(_profiles_dir, _rn + ".yaml")):
            fail("R14", "Profile fehlt fuer Repo: " + _rn)
    print("R14 Standards-Profile: " + str(len(_pfiles)) + " geprueft")
    # R15: Org-Engineering-Baseline (ATC-ORG-BASELINE-001, SCR-0103)
    import yaml as _y15

    _ry = _y15.safe_load(open("registry/repositories.yaml", encoding="utf-8"))
    _rl15 = _ry.get("repositories", _ry) if isinstance(_ry, dict) else []
    _T0 = {".github", "atc-standards"}
    _T1E = {"a-townchain", "atclang", "atc-shivacore", "atc-zkp", "atc-wallet", "atc-algorithm"}
    for _re in _rl15:
        _nm = str(_re.get("name"))
        _t = _re.get("tier")
        if _t not in ("T0", "T1", "T2", "T3", "T4"):
            fail("R15", "Tier fehlt/ungueltig: " + _nm)
        elif _nm in _T0 and _t != "T0":
            fail("R15", "T0-Mitgliedschaft verletzt: " + _nm)
        elif _nm in _T1E and _t != "T1":
            fail("R15", "T1-Owner-Liste verletzt: " + _nm)
        elif (
            str(_re.get("criticality")) == "C1"
            and str(_re.get("security_class")) == "S4"
            and _t not in ("T0", "T1")
        ):
            fail("R15", "C1+S4-Regel verletzt (muss T0/T1 sein): " + _nm)
    _st15 = _y15.safe_load(open("registry/standards.yaml", encoding="utf-8"))["standards"]
    _b15 = [x for x in _st15 if str(x.get("id")) == "ATC-ORG-BASELINE-001"]
    if len(_b15) != 1 or _b15[0].get("status") != "approved":
        fail("R15", "ATC-ORG-BASELINE-001 fehlt in Registry oder nicht APPROVED")
    print("R15 Org-Baseline-Tiers: 27 Repos geprueft")

    print()
    if FAILS:
        print("RESULT: CROSS-REGISTRY NON-COMPLIANT")
        for f_ in FAILS:
            print("  " + f_)
        sys.exit(1)
    print("RESULT: CROSS-REGISTRY ALL COMPLIANT")


if __name__ == "__main__":
    main()

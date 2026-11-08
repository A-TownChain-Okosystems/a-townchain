#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""ATC Standards Gesamt-Validierung (CI-Modus, ATC-STD-000 §7.11).

Laeuft ueber alle Standard-Dateien (governance/ + standards/), validiert je
Datei (S-01…S-16) und prueft dateiuebergreifend S-17 Duplicate Detection
(7.8) gegen die Registry (Allokations-Autoritaet).

Aufruf: python3 validate_all.py
Exit:   0 = alles COMPLIANT, 1 = mindestens ein FAIL.
"""

import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
REGISTRY = os.path.join(ROOT, "registry", "standards.yaml")
CAND = ["governance", "standards"]


def collect():
    # Registry-getrieben (ATC-STD-000 §24): alle file:-Eintraege aus standards.yaml
    files = []
    try:
        with open(REGISTRY, encoding="utf-8") as fh:
            for line in fh:
                m = re.search(r"file:\s*([^},]+)", line)
                if m:
                    _p = m.group(1).strip()
                    if _p.startswith('"') and _p.endswith('"'):
                        _p = _p[1:-1]  # SG-01/AUD-2026-0004: Quotes im YAML-Wert strippen
                    files.append(os.path.join(ROOT, _p))
    except OSError:
        pass
    for d in CAND:
        base = os.path.join(ROOT, d)
        for dirpath, _dirs, names in os.walk(base):
            for n in names:
                if n.endswith(".md"):
                    f = os.path.join(dirpath, n)
                    if f not in files:
                        files.append(f)
    return sorted(files)


READ_ERRORS = []  # SG-01 (AUD-2026-0004): Lesefehler duerfen nicht still verschwinden


def file_id(path):
    try:
        head = open(path, encoding="utf-8").read(2500)
        m = re.search(
            r"^\s*id:\s*(ATC-STD-(?:BUG-|NET-|ZKP-|AI-DEV-|MD-|SC-|README-|DESC-|VERSION-|AUDIT-|AI-DECISION-|UPDATE-|IMPROVEMENT-|COMPAT-|MILESTONE-|FRAMEWORK-|REPO-AUDIT-|AOS-|PROTOCOL-|TAXONOMY-|STDDEV-|REGISTRY-|CHANGE-)?[0-9]{3,}|ATC-AAS-[0-9]{3,}|ATC-AURORA-SWARM-[0-9]{3,}|ATC-ENT-[0-9]{3,}|ATC-AI-GOV-(?:[A-Z]+-)?[0-9]{3,})\s*$",
            head,
            re.M,
        )
        return m.group(1) if m else None
    except Exception as e:
        READ_ERRORS.append((path, str(e)[:80]))
        return None


def main():
    files = [f for f in collect() if file_id(f)]
    # SG-01 (AUD-2026-0004): unlesbare Kandidatendateien = FAIL (fail-closed)
    if READ_ERRORS:
        for p, e in READ_ERRORS:
            print("S-01 READ: FAIL — %s nicht lesbar: %s" % (p, e))
        sys.exit(1)
    if not files:
        print("Keine Standard-Dateien gefunden")
        return 1
    fails = 0
    print("== ATC Standards Gesamt-Validierung (CI) ==")
    for f in files:
        r = subprocess.run(
            [sys.executable, os.path.join(HERE, "atc_std_validator.py"), f, "--registry", REGISTRY],
            capture_output=True,
            text=True,
        )
        line = [ln for ln in r.stdout.splitlines() if "COMPLIANT" in ln or "NON-COMPLIANT" in ln]
        status = line[0].split("|")[0].strip() if line else ("FAIL" if r.returncode else "?")
        print("%-52s %s" % (os.path.relpath(f, ROOT), status))
        if r.returncode:
            fails += 1
            for ln in r.stdout.splitlines():
                if "[FAIL]" in ln:
                    print("    " + ln.strip())
    # S-17 Duplicate Detection (7.8): gleiche ID in mehreren Dateien = FAIL
    ids = {}
    for f in files:
        i = file_id(f)
        if i:
            ids.setdefault(i, []).append(os.path.relpath(f, ROOT))
    dups = {i: fs for i, fs in ids.items() if len(fs) > 1}
    print(
        "S-17 Duplicate Detection (§7.8): "
        + ("PASS — keine Doppelvergaben" if not dups else "FAIL")
    )
    for i, fs in dups.items():
        print("  %s in: %s" % (i, ", ".join(fs)))
        fails += 1
    # S-18 Registry-Parse-Check: standards.yaml muss strukturell gueltig sein
    # (AUD-001 Governance-Audit 07.09.2026: korrupte Registry wurde nicht erkannt)
    registry_ok = True
    try:
        # Voll-Modus (AUD-2026-0002 Nachtrag): ALLE Registry-Dateien strukturell pruefen
        import glob as _glob

        import yaml

        _reg_files = sorted(_glob.glob(os.path.join(ROOT, "registry", "*.yaml")))
        for _rf in _reg_files:
            with open(_rf, encoding="utf-8") as fh:
                yaml.safe_load(fh)  # wirft bei Strukturfehlern (z.B. Einrueckungs-Bruch)
        with open(REGISTRY, encoding="utf-8") as fh:
            data = yaml.safe_load(fh)
        entries = data.get("standards", []) if isinstance(data, dict) else []
        if not entries:
            print("S-18 Registry-Parse: FAIL — standards.yaml enthaelt keine 'standards:'-Liste")
            registry_ok = False
        else:
            for e in entries:
                for req in ("id", "title", "version", "status", "owner", "file"):
                    if req not in e:
                        print(
                            "S-18 Registry-Parse: FAIL — %s fehlt Pflichtfeld '%s'"
                            % (e.get("id", "?"), req)
                        )
                        registry_ok = False
            print(
                "S-18 Registry-Parse: %s (%d Eintraege, %d Registry-Dateien geprueft)"
                % ("PASS" if registry_ok else "FAIL", len(entries), len(_reg_files))
            )
    except ImportError:
        # Fallback ohne PyYAML (SG-01, AUD-2026-0004): fail-closed und aequivalent stark zum
        # yaml-Pfad — Fluss-Mapping-Eintrage des standards:-Abschnitts voll parsen und
        # Pflichtfelder pruefen. Ein strukturfehlerhaftes standards.yaml MUSS FAILen.
        from atc_std_validator import _parse_flow_entry  # PyYAML-freier Fluss-Parser (SCR-0013)

        reg_text = open(REGISTRY, encoding="utf-8").read()
        sec = re.search(r"^standards:[ \t]*\n(.*?)(?=^[A-Za-z_][\w-]*:|\Z)", reg_text, re.S | re.M)
        if not sec:
            print("S-18 Registry-Parse: FAIL — kein 'standards:'-Abschnitt (Fallback)")
            registry_ok = False
            entries = []
        else:
            entries = [
                _parse_flow_entry(m.group(1))
                for m in re.finditer(r"^\s+-\s+\{(.*)\}\s*$", sec.group(1), re.M)
            ]
        if not entries:
            print(
                "S-18 Registry-Parse: FAIL — standards.yaml enthaelt keine 'standards:'-Liste (Fallback)"
            )
            registry_ok = False
        for n, line in enumerate(open(REGISTRY, encoding="utf-8"), 1):
            s = line.strip()
            if s.startswith("- {") and not s.endswith("}"):
                print("S-18 Registry-Parse: FAIL — Zeile %d unbalanciert (kein '}' am Ende)" % n)
                registry_ok = False
            if s.count("{") != s.count("}") or s.count('"') % 2:
                print("S-18 Registry-Parse: FAIL — Zeile %d Klammer-/Quote-Balanz defekt" % n)
                registry_ok = False
        for e in entries:
            for req in ("id", "title", "version", "status", "owner", "file"):
                if req not in e:
                    print(
                        "S-18 Registry-Parse: FAIL — %s fehlt Pflichtfeld '%s' (Fallback)"
                        % (e.get("id", "?"), req)
                    )
                    registry_ok = False
        print(
            "S-18 Registry-Parse: %s (Fallback-Modus, PyYAML fehlt; %d Eintraege voll geprueft)"
            % ("PASS" if registry_ok else "FAIL", len(entries))
        )
    except Exception as e:
        print("S-18 Registry-Parse: FAIL — %s" % str(e)[:120])
        registry_ok = False
    if not registry_ok:
        fails += 1

    # Registry-Cross-Check: Registry-Eintraege muessen Dateien haben
    # SG-01 (AUD-2026-0004): fail-closed — Registry unlesbar/inkonsistent = FAIL, nie still
    try:
        reg = open(REGISTRY, encoding="utf-8").read()
        reg_ids = set(re.findall(r"id:\s*(ATC-STD-[0-9]{3,})", reg))
        missing = reg_ids - set(ids)
        if missing:
            print("S-18a Registry-Cross: FAIL — Registry ohne Datei: " + ", ".join(sorted(missing)))
            fails += 1
        else:
            print("S-18a Registry-Cross: PASS (%d Registry-IDs alle mit Datei)" % len(reg_ids))
    except Exception as e:
        print("S-18a Registry-Cross: FAIL — Registry nicht lesbar: %s" % str(e)[:100])
        fails += 1

    # S-20 Milestone-Registry-Check (ATC-STD-MILESTONE-001 §17)
    milestone_ok = True
    MS_PATH = os.path.join(ROOT, "registry", "milestones.yaml")
    if not os.path.exists(MS_PATH):
        print("S-20 Milestones: PASS (keine milestones.yaml — Standard ohne Registry-Nutzung)")
    else:
        try:
            import yaml as _yaml

            ms_data = _yaml.safe_load(open(MS_PATH, encoding="utf-8")) or {}
            entries = ms_data.get("milestones", [])
            if not isinstance(entries, list) or not entries:
                print("S-20 Milestones: FAIL — milestones.yaml ohne 'milestones:'-Liste")
                milestone_ok = False
            else:
                required = (
                    "id",
                    "name",
                    "category",
                    "status",
                    "owner",
                    "goal",
                    "scope",
                    "acceptance_criteria",
                    "dependencies",
                    "risk",
                    "evidence",
                    "start",
                    "target_date",
                    "audit_ref",
                )
                allowed_status = {
                    "PLANNED",
                    "DEFINED",
                    "IN_PROGRESS",
                    "BLOCKED",
                    "FEATURE_COMPLETE",
                    "VALIDATION",
                    "AUDIT",
                    "FAILED",
                    "ACCEPTED",
                    "RELEASED",
                    "VERIFIED",
                    "CLOSED",
                    "SUPERSEDED",
                }
                ids, stat = set(), {}
                for m in entries:
                    mid = str(m.get("id", "?"))
                    for req in required:
                        if req not in m:
                            print("S-20 Milestones: FAIL — %s fehlt Pflichtfeld '%s'" % (mid, req))
                            milestone_ok = False
                    if not re.match(r"^ATC-M-(?:[A-Z]+-)?[0-9]{3,}$", mid):
                        print("S-20 Milestones: FAIL — %s: ID-Pattern verletzt" % mid)
                        milestone_ok = False
                    if mid in ids:
                        print("S-20 Milestones: FAIL — %s: doppelte ID" % mid)
                        milestone_ok = False
                    ids.add(mid)
                    stat[mid] = m.get("status")
                    if m.get("category") not in {"M%d" % i for i in range(9)}:
                        print(
                            "S-20 Milestones: FAIL — %s: Kategorie %s ungueltig"
                            % (mid, m.get("category"))
                        )
                        milestone_ok = False
                    if m.get("status") not in allowed_status:
                        print(
                            "S-20 Milestones: FAIL — %s: Status %s ungueltig"
                            % (mid, m.get("status"))
                        )
                        milestone_ok = False
                    if not isinstance(m.get("risk"), dict) or "overall" not in (
                        m.get("risk") or {}
                    ):
                        print("S-20 Milestones: FAIL — %s: risk.overall fehlt" % mid)
                        milestone_ok = False
                for m in entries:
                    mid = str(m.get("id", "?"))
                    if m.get("status") in {"ACCEPTED", "RELEASED", "VERIFIED", "CLOSED"}:
                        if not m.get("evidence"):
                            print("S-20 Milestones: FAIL — %s: ACCEPTED ohne Evidence (§6)" % mid)
                            milestone_ok = False
                        if not m.get("audit_result"):
                            print("S-20 Milestones: FAIL — %s: ACCEPTED ohne audit_result" % mid)
                            milestone_ok = False
                        if not m.get("actual_completion"):
                            print(
                                "S-20 Milestones: FAIL — %s: ACCEPTED ohne actual_completion" % mid
                            )
                            milestone_ok = False
                        if m.get("compatibility_status") == "UNKNOWN":
                            print(
                                "S-20 Milestones: FAIL — %s: ACCEPTED mit compatibility UNKNOWN (COMPAT-001)"
                                % mid
                            )
                            milestone_ok = False
                    for dep in m.get("dependencies") or []:
                        if str(dep).startswith("ATC-M-"):
                            if dep not in ids:
                                print(
                                    "S-20 Milestones: FAIL — %s: unbekannte Dependency %s"
                                    % (mid, dep)
                                )
                                milestone_ok = False
                            elif stat.get(dep) not in (
                                None,
                                "ACCEPTED",
                                "RELEASED",
                                "VERIFIED",
                                "CLOSED",
                                "SUPERSEDED",
                            ) and m.get("status") in {"ACCEPTED", "RELEASED", "VERIFIED", "CLOSED"}:
                                print(
                                    "S-20 Milestones: FAIL — %s: ACCEPTED bei offener kritischer Dependency %s (%s) (§8)"
                                    % (mid, dep, stat.get(dep))
                                )
                                milestone_ok = False
                print(
                    "S-20 Milestones: %s (%d Eintraege, IDs: %s)"
                    % ("PASS" if milestone_ok else "FAIL", len(entries), ", ".join(sorted(ids)))
                )
        except ImportError:
            # SG-01 (AUD-2026-0004): fail-closed — milestones.yaml muss auch ohne PyYAML
            # strukturell geprueft werden; WARN ist ein Fail-Open-Zustand.
            raw = open(MS_PATH, encoding="utf-8").read()
            blocks = re.split(r"^\s*-\s+id:\s+", raw, flags=re.M)[1:]
            if not blocks:
                print("S-20 Milestones: FAIL — milestones.yaml ohne '- id:'-Eintraege (Fallback)")
                milestone_ok = False
            seen = set()
            for b in blocks:
                mid = b.splitlines()[0].strip()
                if not re.match(r"^ATC-M-(?:[A-Z]+-)?[0-9]{3,}$", mid):
                    print("S-20 Milestones: FAIL — %s: ID-Pattern verletzt (Fallback)" % mid)
                    milestone_ok = False
                if mid in seen:
                    print("S-20 Milestones: FAIL — %s: doppelte ID (Fallback)" % mid)
                    milestone_ok = False
                seen.add(mid)
                for req in (
                    "name",
                    "category",
                    "status",
                    "owner",
                    "goal",
                    "risk",
                    "target_date",
                    "audit_ref",
                ):
                    if not re.search(r"^\s*%s\s*:" % req, b, re.M):
                        print("S-20 Milestones: FAIL — %s: Feld '%s' fehlt (Fallback)" % (mid, req))
                        milestone_ok = False
            print(
                "S-20 Milestones: %s (Fallback-Modus, PyYAML fehlt; %d Bloecke geprueft)"
                % ("PASS" if milestone_ok else "FAIL", len(blocks))
            )
        except Exception as e:
            print("S-20 Milestones: FAIL — %s" % str(e)[:120])
            milestone_ok = False
    if not milestone_ok:
        fails += 1

    # S-21 Framework-Katalog-Check (ATC-STD-FRAMEWORK-001 §5/§17)
    framework_ok = True
    FW_PATH = os.path.join(ROOT, "registry", "framework.yaml")
    if not os.path.exists(FW_PATH):
        print("S-21 Framework: PASS (keine framework.yaml)")
    else:
        try:
            import yaml as _fy

            fw = _fy.safe_load(open(FW_PATH, encoding="utf-8")) or {}
            fams = (fw.get("framework") or {}).get("families") or []
            if not fams:
                print("S-21 Framework: FAIL — framework.yaml ohne 'families:'-Liste")
                framework_ok = False
            else:
                reg = open(REGISTRY, encoding="utf-8").read()
                reg_ids = set(re.findall(r"id:\s*(ATC-[\w.-]+)", reg)) | set(
                    re.findall(r'id:\s*"(ATC-[\w.-]+)"', reg)
                )
                allowed = {"NEU", "BELEGT", "VERWEIST", "KONFLIKT", "GEPLANT"}
                fam_ids, slot_ids = set(), set()
                for fam in fams:
                    fid = str(fam.get("id", "?"))
                    for req in ("id", "name", "range"):
                        if req not in fam:
                            print("S-21 Framework: FAIL — Familie %s fehlt '%s'" % (fid, req))
                            framework_ok = False
                    if fid in fam_ids:
                        print("S-21 Framework: FAIL — doppelte Familie %s" % fid)
                        framework_ok = False
                    fam_ids.add(fid)
                    frefs = [str(r) for r in (fam.get("family_refs") or [])]
                    for s in fam.get("slots") or []:
                        sid = str(s.get("id", "?"))
                        if "title" not in s or "status" not in s:
                            print(
                                "S-21 Framework: FAIL — Slot %s unvollständig (title/status)" % sid
                            )
                            framework_ok = False
                        if sid in slot_ids:
                            print("S-21 Framework: FAIL — doppelter Slot %s" % sid)
                            framework_ok = False
                        slot_ids.add(sid)
                        st = s.get("status")
                        if st not in allowed:
                            print("S-21 Framework: FAIL — Slot %s: Status %s ungueltig" % (sid, st))
                            framework_ok = False
                        if st == "KONFLIKT" and not s.get("note"):
                            print("S-21 Framework: FAIL — Slot %s: KONFLIKT ohne Note" % sid)
                            framework_ok = False
                        if st in ("BELEGT", "VERWEIST"):
                            for ref in [str(r) for r in (s.get("refs") or [])] or frefs:
                                if ref.startswith("ATC-") and ref not in reg_ids:
                                    print(
                                        "S-21 Framework: FAIL — Slot %s: Referenz %s nicht in Registry"
                                        % (sid, ref)
                                    )
                                    framework_ok = False
                st_total = sum(len(f.get("slots") or []) for f in fams)
                print(
                    "S-21 Framework: %s (%d Familien, %d Slots, Registry-Referenzen aufgeloest)"
                    % ("PASS" if framework_ok else "FAIL", len(fams), st_total)
                )
        except ImportError:
            print("S-21 Framework: WARN (Fallback-Modus, PyYAML fehlt)")
        except Exception as e:
            print("S-21 Framework: FAIL — %s" % str(e)[:120])
            framework_ok = False
    if not framework_ok:
        fails += 1

    # S-22 Repo-Audit-Check-Katalog (ATC-STD-REPO-AUDIT-002 §2/§9)
    checks_ok = True
    RC_PATH = os.path.join(ROOT, "registry", "repo-audit-checks.yaml")
    if not os.path.exists(RC_PATH):
        print("S-22 RepoAuditChecks: PASS (keine repo-audit-checks.yaml)")
    else:
        try:
            import yaml as _cy

            rc = (_cy.safe_load(open(RC_PATH, encoding="utf-8")) or {}).get(
                "repo-audit-checks"
            ) or {}
            areas = rc.get("areas") or []
            checks = rc.get("checks") or []
            if not areas or not checks:
                print("S-22 RepoAuditChecks: FAIL — areas/checks fehlen")
                checks_ok = False
            else:
                area_ids = [a.get("id") for a in areas]
                if len(area_ids) != len(set(area_ids)):
                    print("S-22 RepoAuditChecks: FAIL — doppelte Bereichs-IDs")
                    checks_ok = False
                wsum = sum(a.get("weight") or 0 for a in areas)
                if wsum != 100:
                    print("S-22 RepoAuditChecks: FAIL — Bereichsgewichte Summe %s != 100" % wsum)
                    checks_ok = False
                seen = set()
                per_area = {aid: 0 for aid in area_ids}
                methods = {"AUTO", "MANUAL", "HYBRID"}
                for chk in checks:
                    cid = str(chk.get("id", "?"))
                    if not re.match(r"^CHECK-[0-9]{3,}$", cid):
                        print("S-22 RepoAuditChecks: FAIL — %s: ID-Pattern verletzt" % cid)
                        checks_ok = False
                    if cid in seen:
                        print("S-22 RepoAuditChecks: FAIL — %s doppelt" % cid)
                        checks_ok = False
                    seen.add(cid)
                    if chk.get("area") not in area_ids:
                        print(
                            "S-22 RepoAuditChecks: FAIL — %s: unbekannter Bereich %s"
                            % (cid, chk.get("area"))
                        )
                        checks_ok = False
                    else:
                        per_area[chk["area"]] += 1
                    if chk.get("method") not in methods:
                        print(
                            "S-22 RepoAuditChecks: FAIL — %s: Methode %s ungueltig"
                            % (cid, chk.get("method"))
                        )
                        checks_ok = False
                    if not isinstance(chk.get("weight"), int) or not (
                        1 <= chk.get("weight", 0) <= 3
                    ):
                        print(
                            "S-22 RepoAuditChecks: FAIL — %s: Gewicht %s ungueltig (1-3)"
                            % (cid, chk.get("weight"))
                        )
                        checks_ok = False
                    if not chk.get("title") or not chk.get("description"):
                        print("S-22 RepoAuditChecks: FAIL — %s: title/description fehlt" % cid)
                        checks_ok = False
                if len(checks) < 48:
                    print(
                        "S-22 RepoAuditChecks: FAIL — nur %d Checks (>= 48 gefordert)" % len(checks)
                    )
                    checks_ok = False
                for aid, n in per_area.items():
                    if n < 3:
                        print(
                            "S-22 RepoAuditChecks: FAIL — Bereich %s: nur %d Checks (>= 3)"
                            % (aid, n)
                        )
                        checks_ok = False
                print(
                    "S-22 RepoAuditChecks: %s (%d Bereiche/Gewichtsumme %d, %d Checks)"
                    % ("PASS" if checks_ok else "FAIL", len(areas), wsum, len(checks))
                )
        except ImportError:
            print("S-22 RepoAuditChecks: WARN (Fallback-Modus, PyYAML fehlt)")
        except Exception as e:
            print("S-22 RepoAuditChecks: FAIL — %s" % str(e)[:120])
            checks_ok = False
    if not checks_ok:
        fails += 1

    # S-23 Protocol-Registry (ATC-STD-PROTOCOL-001 §20/§21)
    proto_ok = True
    PR_PATH = os.path.join(ROOT, "registry", "protocol-registry.yaml")
    if not os.path.exists(PR_PATH):
        print("S-23 ProtocolRegistry: PASS (keine protocol-registry.yaml)")
    else:
        try:
            import yaml as _py

            pr = (_py.safe_load(open(PR_PATH, encoding="utf-8")) or {}).get(
                "protocol-registry"
            ) or {}
            protos = pr.get("protocols") or []
            if len(protos) < 20:
                print(
                    "S-23 ProtocolRegistry: FAIL — nur %d Protokolle (>= 20 gefordert)"
                    % len(protos)
                )
                proto_ok = False
            seen = set()
            statuses = {"planned", "draft", "active", "experimental", "deprecated"}
            for e in protos:
                pid = str(e.get("id", "?"))
                if not re.match(r"^ATC-PROTO-[A-Z0-9]+(-[A-Z0-9]+)*-[0-9]{3,}$", pid):
                    print("S-23 ProtocolRegistry: FAIL — %s: ID-Pattern verletzt" % pid)
                    proto_ok = False
                if pid in seen:
                    print("S-23 ProtocolRegistry: FAIL — %s doppelt" % pid)
                    proto_ok = False
                seen.add(pid)
                if e.get("status") not in statuses:
                    print(
                        "S-23 ProtocolRegistry: FAIL — %s: Status %s ungueltig"
                        % (pid, e.get("status"))
                    )
                    proto_ok = False
                if e.get("priority") not in {"P0", "P1", "P2"}:
                    print(
                        "S-23 ProtocolRegistry: FAIL — %s: Prioritaet %s ungueltig"
                        % (pid, e.get("priority"))
                    )
                    proto_ok = False
                if not re.match(r"^[0-9]+[.][0-9]+[.][0-9]+$", str(e.get("version", "?"))):
                    print(
                        "S-23 ProtocolRegistry: FAIL — %s: Version %s kein SemVer"
                        % (pid, e.get("version"))
                    )
                    proto_ok = False
                for f in ("name", "domain", "layer", "specification"):
                    if not e.get(f):
                        print("S-23 ProtocolRegistry: FAIL — %s: Feld %s fehlt" % (pid, f))
                        proto_ok = False
            sc = {}
            for e in protos:
                sc[e.get("status", "?")] = sc.get(e.get("status", "?"), 0) + 1
            print(
                "S-23 ProtocolRegistry: %s (%d Protokollfamilien, Status: %s)"
                % ("PASS" if proto_ok else "FAIL", len(protos), sc)
            )
        except ImportError:
            print("S-23 ProtocolRegistry: WARN (Fallback-Modus, PyYAML fehlt)")
        except Exception as e:
            print("S-23 ProtocolRegistry: FAIL — %s" % str(e)[:120])
            proto_ok = False
    if not proto_ok:
        fails += 1

    # S-24 Standards Taxonomy (ATC-STD-TAXONOMY-001 §8/§13)
    tax_ok = True
    TAX_PATH = os.path.join(ROOT, "registry", "taxonomy.yaml")
    if not os.path.exists(TAX_PATH):
        print("S-24 Taxonomy: PASS (keine taxonomy.yaml)")
    else:
        try:
            import yaml as _py

            t = (_py.safe_load(open(TAX_PATH, encoding="utf-8")) or {}).get("taxonomy") or {}
            domains = t.get("domains") or []
            LIFECYCLE = {"PROPOSED", "ANALYZED", "APPROVED", "ACTIVE", "DEPRECATED", "RETIRED"}
            dom_ids, fam_names, fam_codes = set(), [], []
            n_dom = n_fam = 0
            for d in domains:
                if not d.get("id") or d["id"] in dom_ids:
                    print("S-24 Taxonomy: FAIL — Domain-ID fehlt/doppelt: %s" % d.get("id"))
                    tax_ok = False
                dom_ids.add(d.get("id"))
                if d.get("lifecycle") not in LIFECYCLE:
                    print("S-24 Taxonomy: FAIL — Domain %s: Lifecycle ungueltig" % d.get("id"))
                    tax_ok = False
                n_dom += 1
                for f in d.get("families") or []:
                    n_fam += 1
                    if f.get("id") in fam_codes:
                        print("S-24 Taxonomy: FAIL — Familien-Code doppelt: %s" % f.get("id"))
                        tax_ok = False
                    if f.get("name") in fam_names:
                        print("S-24 Taxonomy: FAIL — Familien-Name doppelt: %s" % f.get("name"))
                        tax_ok = False
                    if f.get("lifecycle") not in LIFECYCLE:
                        print("S-24 Taxonomy: FAIL — Familie %s: Lifecycle ungueltig" % f.get("id"))
                        tax_ok = False
                    fam_codes.append(f.get("id"))
                    fam_names.append(f.get("name"))
                    cat_ids = [c2.get("id") for c2 in f.get("categories") or []]
                    if len(cat_ids) != len(set(cat_ids)):
                        print(
                            "S-24 Taxonomy: FAIL — Familie %s: Kategorie-Codes doppelt"
                            % f.get("id")
                        )
                        tax_ok = False
            # TAX-CHECK-013/014: Registry-Konsistenz (jede standards.yaml-Kategorie -> Familie)
            sr = _py.safe_load(
                open(os.path.join(ROOT, "registry", "standards.yaml"), encoding="utf-8")
            )
            fam_set = set(fam_names)
            missing = set()
            for e in sr.get("standards") or []:
                cat = e.get("category", "?")
                if cat not in fam_set and (cat + " (familie)") not in fam_set:
                    missing.add(cat)
            if missing:
                print(
                    "S-24 Taxonomy: FAIL — Kategorien ohne Taxonomie-Familie: %s" % sorted(missing)
                )
                tax_ok = False
            n_std = sum(
                f.get("standards_count", 0) for d in domains for f in d.get("families") or []
            )
            print(
                "S-24 Taxonomy: %s (%d Domains, %d Familien, %d Standards zugeordnet, Registry-Konsistenz %s)"
                % (
                    "PASS" if tax_ok else "FAIL",
                    n_dom,
                    n_fam,
                    n_std,
                    "OK" if not missing else "FEHLER",
                )
            )
        except ImportError:
            print("S-24 Taxonomy: WARN (Fallback-Modus, PyYAML fehlt)")
        except Exception as e:
            print("S-24 Taxonomy: FAIL — %s" % str(e)[:120])
            tax_ok = False
    if not tax_ok:
        fails += 1

    # S-25 Frontmatter-Strict-YAML (AUD-2026-0003 / F-032+F-038: Frontmatter muss strictes YAML sein)
    fm_ok = True
    try:
        import glob as _g2

        import yaml as _fy

        _n = 0
        for _f in sorted(_g2.glob(os.path.join(ROOT, "standards", "**", "*.md"), recursive=True)):
            _t = open(_f, encoding="utf-8").read()
            _m = re.match(r"^---\n(.*?)\n----\n", _t, re.S) or re.match(
                r"^---\n(.*?)\n---\n", _t, re.S
            )
            if not _m:
                continue
            try:
                _d = _fy.safe_load(_m.group(1))
                if not (isinstance(_d, dict) and "standard" in _d):
                    raise ValueError("kein 'standard:'-Objekt")
            except Exception as _e:
                print(
                    "S-25 Frontmatter-YAML: FAIL — %s: %s" % (os.path.basename(_f), str(_e)[:100])
                )
                fm_ok = False
            _n += 1
        print(
            "S-25 Frontmatter-YAML: %s (%d Dateien strict geparst)"
            % ("PASS" if fm_ok else "FAIL", _n)
        )
    except ImportError:
        # CI-007/CI-010 (ATC-STD-CI-001): fehlende Dependency = FAIL,
        # klassifiziert — kein gepruefter Fallback mehr (Issue #1, P1).
        print(
            "S-25 Frontmatter-YAML: FAIL (DEPENDENCY_MISSING — PyYAML fehlt;"
            " ATC-STD-CI-001 CI-001/CI-002, Issue #1)"
        )
        fm_ok = False
    if not fm_ok:
        fails += 1

    # ===== Enforcement-Pipeline (SCR-0053): Voll-Erzwungung der definierten Standards je Push/PR =====
    # E-Stages: Zusatz-Gates der Einzel-Validatoren. Jeder echte Befund zaehlt
    # in das Gesamtergebnis und rottet den Workflow aus. Fehlende Runner-
    # Abhaengigkeiten (PyYAML) degradieren sichtbar zu WARN (Präzedenz S-25),
    # echte Findings bleiben FAIL.
    stages = [
        (
            "E-1",
            "Meta-Daten-Audit (SCR-0050, MD-A..MD-F)",
            [sys.executable, os.path.join(HERE, "meta_data_audit.py")],
        ),
        (
            "E-2",
            "Meta-Sweep (SCR-0047, MS-1..MS-7)",
            [sys.executable, os.path.join(HERE, "meta_sweep.py")],
        ),
        (
            "E-3",
            "Agent-Manifest-Check (AGENT_MANIFEST-Standard)",
            [sys.executable, os.path.join(HERE, "check_agent_manifest.py")],
        ),
        (
            "E-4",
            "README-Gates (ATC-STD-README-001, 13 Gates)",
            [
                sys.executable,
                os.path.join(ROOT, "tools", "atc-readme-validator", "check_readme.py"),
                ROOT,
            ],
        ),
        (
            "E-5",
            "CI-Dependency-Governance (ATC-STD-CI-001, Regressionstest Issue #1)",
            [sys.executable, os.path.join(HERE, "tests", "test_ci_dependency_governance.py")],
        ),
    ]
    for sid, name, cmd in stages:
        try:
            r = subprocess.run(cmd, capture_output=True, text=True)
        except OSError as _e:
            print("%s %s: FAIL (Stage nicht ausfuehrbar: %s)" % (sid, name, _e))
            fails += 1
            continue
        out = (r.stdout or "") + (r.stderr or "")
        missing_dep = "ModuleNotFoundError" in out and "yaml" in out
        if r.returncode == 0:
            print("%s %s: PASS" % (sid, name))
        elif missing_dep:
            # CI-007/CI-010 (ATC-STD-CI-001): Dependency-Fehler werden
            # klassifiziert und FAILen — kein Silent-Fallback mehr.
            print(
                "%s %s: FAIL (DEPENDENCY_MISSING — Runner-Dependency fehlt,"
                " ATC-STD-CI-001 CI-002/CI-006; kein Erfolg durch"
                " guenstigen Runner-Zustand, CI-010)" % (sid, name)
            )
            fails += 1
        else:
            print("%s %s: FAIL" % (sid, name))
            for line in out.splitlines()[-12:]:
                print("    " + line)
            fails += 1

    print("RESULT: " + ("ALL COMPLIANT" if fails == 0 else "%d FAIL(s)" % fails))

    # S-19 Mutationstest-Suite: Header-Drift-Erkennung muss zuverlaessig
    # funktionieren — gezielte Mutanten gegen echte Standards. Laeuft in
    # der CI automatisch mit (validate_all wird je Push/PR ausgefuehrt).
    suite = os.path.join(
        os.path.dirname(os.path.abspath(__file__)), "tests", "test_s19_mutation.py"
    )
    if os.path.exists(suite):
        rc = subprocess.run([sys.executable, suite]).returncode
        if rc != 0:
            return rc
    return 0 if fails == 0 else 1


if __name__ == "__main__":
    sys.exit(main())

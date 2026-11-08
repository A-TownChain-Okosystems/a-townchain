#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""ATC Standard Validator v0.2.0 — Validator fuer ATC-STD-000 (Governance).

Prueft einen Standard gegen die Verfassung: Metadaten-Header, ID-Format,
SemVer, Lifecycle-Status, Abstract/Scope, REQ-IDs, normative Sprache,
Compliance/Security-Sektionen, Changelog, References, Registry-Eintrag
und Abhaengigkeitszyklen (registry/dependencies.yaml).

Aufruf: python3 atc_std_validator.py <standard.md> [--registry <standards.yaml>]
Exit:   0 = COMPLIANT, 1 = NON-COMPLIANT. Nur Python-stdlib.
"""

import argparse
import os
import re
import sys


def _parse_flow_entry(body):
    """Parst den Rumpf eines Fluss-Mappings '{id: X, title: "a, b", ...}' ohne PyYAML.
    Kommas innerhalb gequoteter Werte bleiben erhalten."""
    out = {}
    buf = ""
    in_q = None
    parts = []
    for ch in body:
        if in_q:
            buf += ch
            if ch == in_q:
                in_q = None
            continue
        if ch in ('"', "'"):
            in_q = ch
            buf += ch
            continue
        if ch == ",":
            parts.append(buf.strip())
            buf = ""
        else:
            buf += ch
    parts.append(buf.strip())
    for p in parts:
        if ":" in p:
            k, _, v = p.partition(":")
            out[k.strip()] = v.strip().strip('"').strip("'")
    return out


def _find_registry_entry(reg_text, sid):
    """Findet den Eintrag ausschliesslich innerhalb des 'standards:'-Abschnitts
    (bis zum naechsten Top-Level-Key). Positionssensitiv, PyYAML-frei."""
    sec = re.search(r"^standards:[ \t]*\n(.*?)(?=^[A-Za-z_][\w-]*:|\Z)", reg_text, re.S | re.M)
    if not sec:
        return None
    for m in re.finditer(r"^\s+-\s+\{(.*)\}\s*$", sec.group(1), re.M):
        d = _parse_flow_entry(m.group(1))
        if d.get("id") == sid:
            return d
    return None


VERSION = "0.2.0"
STATES = [
    "idea",
    "proposed",
    "draft",
    "review",
    "candidate",
    "approved",
    "stable",
    "deprecated",
    "retired",
]
CATS = [
    "governance",
    "architecture",
    "repository",
    "development",
    "security",
    "protocol",
    "blockchain",
    "ai",
    "os",
    "infrastructure",
    "applications",
    "bug",
    "net",
    "zkp",
    "ai-dev",
    "aas",
    "enterprise",
    "readme",
    "md",
    "sc",
    "desc",
    "version",
    "audit",
    "ai-decision",
    "update",
    "improvement",
    "ai-gov",
    "compat",
    "milestone",
    "framework",
    "repo-audit",
    "agent-operating",
    "master-audit",
    "protocol",
    "taxonomy",
    "governance-core",
]
REQ_RE = re.compile(r"REQ-[A-Z]+(?:-[A-Z]+)?-[0-9]{3}")


def read(p):
    try:
        with open(p, encoding="utf-8", errors="replace") as f:
            return f.read()
    except OSError:
        return None


class V:
    def __init__(self):
        self.results = []

    def add(self, code, status, msg):
        self.results.append((code, status, msg))


def parse_meta(text):
    """Minimal-Parser fuer den YAML-Header (bis '---')."""
    m = re.match(r"^(?:---\s*\n)?standard:\s*\n(.*?)\n---", text, re.S)
    if not m:
        return None
    meta = {}
    for line in m.group(1).splitlines():
        km = re.match(
            r"^\s*(id|title|version|status|category|authority|owner|created|updated|normative|superseded_by):\s*(.+)$",
            line,
        )
        if km:
            val = km.group(2).strip().strip('"').strip("'")
            meta[km.group(1)] = val
    return meta


def find_cycles(dep_text):
    """Zyklenerkennung im standards:-Graph der registry/dependencies.yaml."""
    sec = re.search(r"^standards:\s*\n(.*?)(?=\Z|^\w)", dep_text, re.S | re.M)
    if not sec:
        return None
    graph = {}
    for line in sec.group(1).splitlines():
        m = re.match(r"^\s*(ATC-STD-[0-9]+):\s*\{\s*depends:\s*\[(.*?)\]\s*\}", line)
        if m:
            graph[m.group(1)] = [d.strip() for d in m.group(2).split(",") if d.strip()]
    cycles = []
    state = {}

    def dfs(node, path):
        if state.get(node) == "ok":
            return
        if node in path:
            cycles.append(path[path.index(node) :] + [node])
            return
        state[node] = "visiting"
        for dep in graph.get(node, []):
            dfs(dep, path + [node])
        state[node] = "ok"

    for n in graph:
        dfs(n, [])
    return cycles or ([], [])[0]


def validate(path, registry_path):
    v = V()
    text = read(path) or ""
    meta = parse_meta(text) or {}

    # S-01 Metadaten
    REQ_KEYS = [
        "id",
        "title",
        "version",
        "status",
        "category",
        "authority",
        "owner",
        "created",
        "updated",
        "normative",
    ]
    if not meta:
        v.add("S-01", "FAIL", "Kein maschinenlesbarer Metadaten-Header (ATC-STD-000 §8)")
    else:
        fehlt = [k for k in REQ_KEYS if k not in meta]
        v.add(
            "S-01",
            "FAIL" if fehlt else "PASS",
            "Metadaten: " + ("vollstaendig" if not fehlt else "fehlt: " + ", ".join(fehlt)),
        )

    # S-02 ID-Format (7.10/7.11: Muster aus naming-conventions.schema.json)
    sid = meta.get("id", "")
    _ok02 = (
        re.match(
            r"^ATC-STD-(?:BUG-|NET-|ZKP-|AI-DEV-|README-|MD-|SC-|DESC-|VERSION-|AUDIT-|AI-DECISION-|UPDATE-|IMPROVEMENT-|COMPAT-|MILESTONE-|FRAMEWORK-|REPO-AUDIT-|AOS-|PROTOCOL-|TAXONOMY-|STDDEV-|REGISTRY-|CHANGE-|LICENSE-|V2S-)?[0-9]{3,}$",
            sid,
        )
        or re.match(r"^ATC-(?:AAS|AURORA-SWARM|ENT)-[0-9]{3,}$", sid)
        or re.match(r"^ATC-AI-GOV-(?:[A-Z]+-)?[0-9]{3,}$", sid)
    )
    v.add(
        "S-02",
        "PASS" if _ok02 else "FAIL",
        "ID-Format: %s"
        % (sid if _ok02 else (sid or "FEHLT") + " (Schema: alle *StandardId-Muster)"),
    )

    # S-03 SemVer
    ver = meta.get("version", "")
    v.add(
        "S-03",
        "PASS" if re.match(r"^\d+\.\d+\.\d+$", ver) else "FAIL",
        "Version: %s" % (ver if ver else "FEHLT"),
    )

    # S-04 Lifecycle
    st = meta.get("status", "").lower()
    v.add(
        "S-04",
        "PASS" if st in STATES else "FAIL",
        "Status: %s" % (st if st in STATES else st + " (ungueltig)"),
    )

    # S-05 Kategorie
    kat = meta.get("category", "").lower()
    _cats = set(CATS)
    if registry_path:
        _cp = os.path.join(os.path.dirname(registry_path), "categories.yaml")
        if os.path.exists(_cp):
            _t = read(_cp)
            _cats |= set(re.findall(r"name:\s*([\w-]+)", _t)) | set(
                re.findall(r"^\s*([\w-]+):\s*\{", _t, re.M)
            )
    v.add(
        "S-05",
        "PASS" if kat in _cats else "FAIL",
        "Kategorie: %s" % (kat if kat in CATS else kat + " (nicht in categories.yaml)"),
    )

    # S-06 Abstract
    v.add(
        "S-06",
        "PASS" if re.search(r"^##+\s.*(?:Abstract|Purpose)", text, re.M | re.I) else "FAIL",
        "Abstract/Purpose-Sektion (§9-Struktur)",
    )

    # S-07 Scope
    hat_scope = re.search(r"^##+\s.*Scope", text, re.M | re.I) or re.search(r"Scope:", text)
    v.add("S-07", "PASS" if hat_scope else "FAIL", "Scope (Gilt/Nicht-Gilt)")

    # S-08 REQ-IDs (Duplikat-Check nur auf id:-Deklarationen, nicht Prosa-Beispiele)
    reqs = REQ_RE.findall(text)
    decl = re.findall(r"id:\s*(REQ-[A-Z]+(?:-[A-Z]+)?-[0-9]{3})", text)
    dup = [r for r in set(decl) if decl.count(r) > 1]
    if dup:
        v.add("S-08", "FAIL", "Doppelte REQ-ID-Deklarationen: " + ", ".join(dup))
    elif decl:
        v.add(
            "S-08",
            "PASS",
            "REQ-IDs: %d deklariert, %d Erwaehnungen, eindeutig" % (len(decl), len(reqs)),
        )
    else:
        v.add(
            "S-08", "WARN", "Keine REQ-ID-Deklarationen (ATC-STD-000 §11 empfiehlt REQ-<DOM>-NNN)"
        )

    # S-19 Header-Sync (Review-Befund 07.09., hardened 07.09.): Der Zitat-
    # Kopf nach dem Frontmatter DARF keine abweichende Version/Status
    # nennen. Nur der Kopf-Block wird geprueft — nie Prosa-/Beispielbloecke.
    # Kopf-Bereich robust bestimmen:
    #   a) klassisches Frontmatter (Datei beginnt mit '---'): Kopf liegt
    #      ZWISCHEN dem schliessenden '---' und dem naechsten '---'
    #   b) Frontmatter ohne oeffnendes '---' (projektueblich): Kopf liegt
    #      zwischen dem ersten und zweiten '---'
    #   c) kein zweites '---': Kopf auf max. 1500 Zeichen begrenzt
    #      (verhindert False Positives aus spaeteren Beispielbloecken)
    if text.startswith("---"):
        fm_close = text.find("\n---", 3)
        head_start = fm_close if fm_close > 0 else 0
    else:
        head_start = text.find("\n---")
        head_start = head_start if head_start > 0 else 0
    head_end = text.find("\n---", head_start + 4)
    if head_end > head_start:
        head = text[head_start:head_end]
    else:
        head = text[head_start : head_start + 1500]
    hv = re.search(r">\s*\*\*Version:?\*\*\s*v?([\d.]+)", head)
    hst = re.search(r">\s*\*\*Status:?\*\*\s*([A-Za-zÄÖÜäöü\-]+)", head)
    fails = []
    if hv and ver and hv.group(1) != ver:
        fails.append("Kopf-Version %s != Frontmatter %s (Registry ist SSOT)" % (hv.group(1), ver))
    if hst and st:
        hst_n = hst.group(1).strip().upper()
        if hst_n != st.upper():
            fails.append("Kopf-Status '%s' != Frontmatter '%s'" % (hst.group(1).strip(), st))
        # Eingebettete Versionsangabe in der Kopf-Statuszeile, z.B.
        # '> **Status:** PROPOSED (v1.0.1) — ...' ist ebenfalls eine Version-
        # Behauptung und muss zur Frontmatter-Version passen.
        hsv = re.search(r">\s*\*\*Status:?\*\*[^\n]*\(v([\d.]+)\b", head)
        if hsv and ver and hsv.group(1) != ver:
            fails.append("Kopf-Statuszeile Version %s != Frontmatter %s" % (hsv.group(1), ver))
    # H1-Titel: '(vX.Y.Z[, LIFECYCLE])' ist eine Versions-/Status-Behauptung.
    # Lifecycle-Vokabular wird erzwungen; Deskriptoren (z.B. FORMALE) werden
    # ignoriert, damit keine False Positives entstehen.
    LIFECYCLE = {
        "DRAFT",
        "PROPOSED",
        "CANDIDATE",
        "REVIEW",
        "APPROVED",
        "STABLE",
        "RETIRED",
        "NORMATIV",
    }
    tm = re.search(r"^# .*\(v?([\d.]+)(?:,\s*([A-ZÄÖÜ]+))?", head, re.M)
    if tm:
        tv = tm.group(1)
        if ver and tv != ver:
            fails.append("Titel-Version %s != Frontmatter %s" % (tv, ver))
        ts = tm.group(2)
        if ts and ts.upper() in LIFECYCLE and st and ts.upper() != st.upper():
            fails.append("Titel-Status '%s' != Frontmatter '%s'" % (ts, st))
    if fails:
        v.add("S-19", "FAIL", "; ".join(fails))
    elif hv or hst:
        if hst or not st:
            v.add("S-19", "PASS", "Kopf-/Frontmatter-Sync: Version+Status konsistent")
        else:
            v.add("S-19", "WARN", "Kopf-Statuszeile fehlt bei vorhandenem Frontmatter-Status")
    else:
        v.add(
            "S-19",
            "WARN",
            "Kein Versions-/Status-Kopf im Kopfbereich — Kopf geloescht oder ausserhalb des Erwartungsbereichs",
        )

    # S-09 Normative Sprache
    normativ = meta.get("normative", "")
    hat_rf = re.search(r"\b(MUST|SHOULD|MAY|MUSS|SOLLTE|DARF)\b", text)
    if normativ == "true":
        v.add(
            "S-09",
            "PASS" if hat_rf else "FAIL",
            "RFC-2119-Terminologie (MUST/SHOULD/MAY oder MUSS/SOLLTE/DARF)"
            if hat_rf
            else "normativ: true ohne MUST/SHOULD/MAY bzw. MUSS/SOLLTE/DARF",
        )
    else:
        v.add("S-09", "PASS", "nicht normativ — Terminologie optional")

    # S-10 Compliance-Definition
    v.add(
        "S-10",
        "PASS" if re.search(r"Compliance", text, re.I) else "WARN",
        "Compliance-Definition (Verfahren wie Einhaltung geprueft wird)",
    )

    # S-11 Security Considerations
    v.add(
        "S-11",
        "PASS" if re.search(r"^##+\s.*Security", text, re.M | re.I) else "WARN",
        "Security Considerations",
    )

    # S-12 Changelog
    v.add(
        "S-12",
        "PASS" if re.search(r"^##+\s.*Changelog", text, re.M | re.I) else "WARN",
        "Changelog-Sektion",
    )

    # S-13 References
    v.add(
        "S-13",
        "PASS" if re.search(r"^##+\s.*References", text, re.M | re.I) else "WARN",
        "References (kategorisiert NORMATIVE/INFORMATIVE/…)",
    )

    # S-14 Registry-Eintrag (SCR-0013-Fix: positionssensitiver Section-Parser statt
    # Rohtext-Suche — verhindert Fehl-PASS bei Eintraegen ausserhalb der standards-Liste,
    # z.B. in legacy_series; ohne PyYAML-Abhaengigkeit, CI-ubuntu hat kein yaml)
    if registry_path and os.path.exists(registry_path):
        reg_entry = _find_registry_entry(read(registry_path) or "", sid)
        if reg_entry:
            rv = reg_entry.get("version")
            rs = reg_entry.get("status")
            konsistent = (not rv or rv == ver) and (not rs or rs == st)
            v.add(
                "S-14",
                "PASS" if konsistent else "FAIL",
                "Registry-Eintrag: vorhanden, "
                + (
                    "Version/Status konsistent"
                    if konsistent
                    else "Divergenz Datei(%s/%s) vs Registry(%s/%s)" % (ver, st, rv, rs)
                ),
            )
        else:
            v.add(
                "S-14",
                "FAIL",
                "KEIN Registry-Eintrag in standards.yaml standards-Liste (ATC-STD-000 §19: Kein Eintrag = kein Standard)",
            )
    else:
        v.add("S-14", "WARN", "Registry-Check uebersprungen (keine standards.yaml)")

    # S-16 Naming Convention (ATC-STD-000 §7 / 7.10: Regeln NUR aus naming-conventions.schema.json)
    import json as _json

    fname = os.path.basename(path)
    _dflt_std = r"^ATC-STD-[0-9]{3,}$"
    _dflt_req = r"^REQ-(STD|REPO|SEC|PROTO)-[0-9]{3,}$"
    _dflt_fn = r"^ATC-STD-[0-9]{3,}[.]md$"
    std_p, req_p, fn_p, schema_ok = _dflt_std, _dflt_req, _dflt_fn, False
    if registry_path:
        _sp = os.path.join(
            os.path.dirname(registry_path), "..", "schemas", "naming-conventions.schema.json"
        )
        if os.path.exists(_sp):
            try:
                _s = _json.load(open(_sp, encoding="utf-8"))
                _ip = _s["properties"]["identifierPatterns"]["properties"]
                _fp = _s["properties"]["filenamePatterns"]["properties"]
                std_p = "|".join(
                    "(?:" + pat["pattern"] + ")"
                    for k, pat in _ip.items()
                    if k.lower().endswith("standardid")
                )
                req_p = (
                    "|".join(
                        "(?:" + pat["pattern"] + ")"
                        for k, pat in _ip.items()
                        if k.lower().endswith("requirementid")
                    )
                    or _dflt_req
                )
                fn_p = "|".join(
                    "(?:" + pat["pattern"] + ")" for k, pat in _fp.items() if k.endswith("Doc")
                )
                schema_ok = True
            except Exception:
                pass
    std_re, req_re, fn_re = re.compile(std_p), re.compile(req_p), re.compile(fn_p)
    msgs = []
    if not schema_ok:
        msgs.append("naming-conventions.schema.json nicht geladen (7.10)")
    if sid and not std_re.match(sid):
        msgs.append("Standard-ID %s nicht konform (7.2)" % sid)
    if not fn_re.match(fname):
        msgs.append("Dateiname %s nicht konform (7.6)" % fname)
    _badreq = sorted(
        set(
            r
            for r in re.findall(r"REQ-[A-Za-z]+(?:-[A-Za-z]+)?-[0-9]+", text)
            if not req_re.match(r)
        )
    )
    if _badreq:
        msgs.append("REQ-IDs nicht konform (7.2): " + ", ".join(_badreq))
    kurz = re.findall(
        r"\b(?:SCR|F|TC|TS|GATE|ATC-SA|ATC-SCHEMA|ATC-PROTO|ATC-SPEC|ATC-DOC)-[0-9]{1,2}\b", text
    )
    if kurz:
        msgs.append("Malforme IDs <3 Ziffern (7.2): " + ", ".join(sorted(set(kurz))))
    v.add(
        "S-16",
        "PASS" if not msgs else "FAIL",
        "Naming Convention §7: "
        + ("ID-/Datei-Formate konform, Regeln aus Schema geladen" if not msgs else "; ".join(msgs)),
    )

    # S-15 Abhaengigkeitszyklen
    dep_path = (
        os.path.join(os.path.dirname(registry_path or ""), "dependencies.yaml")
        if registry_path
        else None
    )
    dep = read(dep_path) if dep_path else None
    if dep and "standards:" in dep:
        cycles = find_cycles(dep)
        v.add(
            "S-15",
            "PASS" if not cycles else "FAIL",
            "Standard-Abhaengigkeitsgraph: "
            + ("azyklisch" if not cycles else "ZYKLUS: " + " → ".join(cycles)),
        )
    else:
        v.add("S-15", "WARN", "Zyklenerkennung uebersprungen (kein standards-Graph)")

    return v


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("standard")
    ap.add_argument("--registry", default=None)
    args = ap.parse_args()
    reg = args.registry or os.path.join(
        os.path.dirname(os.path.abspath(__file__)), "../../registry/standards.yaml"
    )
    v = validate(args.standard, reg)
    print("ATC STANDARD VALIDATION")
    print("=" * 60)
    print("Standard: %s | Validator v%s (ATC-STD-000)" % (os.path.basename(args.standard), VERSION))
    print()
    fails = warns = 0
    for code, status, msg in v.results:
        print("[%s] %s: %s" % (status, code, msg))
        fails += status == "FAIL"
        warns += status == "WARN"
    print()
    print("RESULT")
    print("=" * 60)
    print(
        "%s | Fails: %d | Warns: %d" % ("COMPLIANT" if not fails else "NON-COMPLIANT", fails, warns)
    )
    sys.exit(0 if not fails else 1)


if __name__ == "__main__":
    main()

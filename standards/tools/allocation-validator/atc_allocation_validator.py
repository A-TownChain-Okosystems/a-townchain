#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Standard-ID Allocation Gate (P1-A2, Allocation-SSOT-Drift-Check).

Prueft, dass registry/allocation/allocation-status.yaml mit den SSOTs
uebereinstimmt:
  - registry/standards.yaml  (Bestands-SSOT: ALLOCATED)
  - registry/framework.yaml (VERWEIST-Slots + FAM-Ranges: RESERVED)

Regeln:
  ALLOC-01  Jede Registry-3xx-ID muss als ALLOCATED gefuehrt werden.
  ALLOC-02  Jede als ALLOCATED gefuehrte ID muss in der Registry existieren.
  ALLOC-03  Jeder VERWEIST-/FAM-Range-Slot muss als RESERVED gefuehrt werden.
  ALLOC-04  Jede als RESERVED gefuehrte ID muss einen VERWEIST-/FAM-Beleg haben.
  ALLOC-05  FREE-Slots muessen numerisch unbeeobachtet sein (weder Registry noch Verweis).
  ALLOC-06  counts muessen stimmen (allocated/reserved/free = 100 je Familie).
  ALLOC-07  rule_free_is_not_allocatable muss auf true stehen.
  ALLOC-08  Jede ID in reservations.yaml muss in allocation-status als RESERVED stehen.
  ALLOC-11  Jede Kategorie mit fam:-Feld (categories.yaml) muss einen FAM-Eintrag dieser ID
            in framework.yaml haben, dessen Range den Namespace der Kategorie abdeckt
            (Owner-Direktive 14.09.: keine Registry-Integration ohne formale Familien-Allokation).
  ALLOC-12  Jeder Standard einer fam:-verkabelten Kategorie muss in der FAM-Range liegen
            (Standards-Registry-Eintrag nur innerhalb der allokierten Namespace-Range).

Exit 0 = PASS, 1 = FAIL (Drift). Neue ID-Vergabe ohne Aktualisierung beider
Seiten laesst das Gate anschlagen — 'ID nicht gefunden' wird nie 'ID ist frei'."""

import argparse
import os
import re
import sys

try:
    import yaml
except ImportError:
    sys.exit("PyYAML erforderlich: pip install pyyaml")


def fail(msg):
    print(f"  FAIL  {msg}")
    return 1


def _range_covers(fam_range, ident, namespace_only=False):
    """True, wenn die FAM-Range die Identitaet abdeckt.
    namespace_only=True: Namespace-Gleichheit reicht (ALLOC-11, Kategorie-Ranges);
    sonst numerische Deckung (ALLOC-12).
    Unterstuetzt: '+'-Komposite (FAM-Range wie Kategorie-Range; jede
    Ident-Komponente muss gedeckt sein), '/'-Gruppen mit gemeinsamer Nummer
    (STDDEV/REGISTRY/CHANGE-001), numerische Baender (120..131) und
    Einzel-Slots (AOS-001)."""

    def norm(x):
        x = x.replace("ATC-STD-", "").strip().strip('"')
        return re.sub(r"\s*\([^)]*\)\s*$", "", x)

    def ns_of(x):
        m = re.match(r"^(.*?)(?:-\d+)+$", x)
        return m.group(1).rstrip("-") if m else x

    def num_of(x):
        m = re.search(r"(\d+)$", x)
        return int(m.group(1)) if m else -1

    def comp_covers(sub, ident_ns, ident_num, ns_only):
        sub = sub.strip()
        # 1) Numerisches Band: 120..131
        mb = re.match(r"^(\d+)\s*\.\.\s*(\d+)$", sub)
        if mb and ident_ns.isdigit() and re.fullmatch(r"\d+", str(ident_num)):
            return int(mb.group(1)) <= ident_num <= int(mb.group(2))
        parts = [s.strip() for s in sub.split("/")]
        # 2) '/'-Gruppe: letzte Komponente traegt die Nummer, gilt fuer alle Namespaces
        mc_last = re.match(
            r"^(.*?)-(\d+)(?:\s*\.\.\s*(?:ATC-STD-)?(?:[A-Z0-9-]*?-)?(\d+))?$", parts[-1]
        )
        num_lo = int(mc_last.group(2)) if mc_last else None
        num_hi = int(mc_last.group(3)) if (mc_last and mc_last.group(3)) else None
        for idx, part in enumerate(parts):
            cns = mc_last.group(1) if idx == len(parts) - 1 and mc_last else part
            if cns != ident_ns and not ident_ns.startswith(cns + "-"):
                continue
            if ns_only or num_lo is None or num_hi is None:
                return True  # Einzel-Slot/Gruppe: Namespace-Deckung genuegt
            return num_lo <= ident_num <= num_hi
        return False

    fr = norm(fam_range)
    for ident_comp in norm(ident).split("+"):
        ic = ident_comp.strip()
        if not ic:
            continue
        if fr == ic:
            continue  # exakte Gleichheit (z.B. ATC-STD-999)
        ins, inum = ns_of(ic), num_of(ic)
        if not any(comp_covers(c, ins, inum, namespace_only) for c in fr.split("+")):
            return False
    return True


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--family", type=int, default=300)
    ap.add_argument("--registry", default="registry/standards.yaml")
    ap.add_argument("--framework", default="registry/framework.yaml")
    ap.add_argument("--status", default="registry/allocation/allocation-status.yaml")
    ap.add_argument("--reservations", default="registry/allocation/reservations.yaml")
    args = ap.parse_args()
    b = str(args.family)[0]

    reg = yaml.safe_load(open(args.registry, encoding="utf-8"))
    fw_raw = open(args.framework, encoding="utf-8").read()
    status = yaml.safe_load(open(args.status, encoding="utf-8"))

    lo, hi = args.family, args.family + 99
    allocated = {
        s["id"] for s in reg.get("standards", []) if re.fullmatch(rf"ATC-STD-{b}\d\d", s["id"])
    }
    reserved = set()
    for m in re.finditer(
        rf"- id: (ATC-STD-{b}\d\d)\s*\n\s*title: [^\n]*\n\s*status: VERWEIST", fw_raw
    ):
        reserved.add(m.group(1))
    for m in re.finditer(r"range: (ATC-STD-(\d+)\.\.ATC-STD-(\d+))", fw_raw):
        for n in range(int(m.group(2)), int(m.group(3)) + 1):
            if lo <= n <= hi:
                reserved.add(f"ATC-STD-{n}")

    alloc_sec = status["allocation"]
    st_alloc = {f"ATC-STD-{k}" for k in status.get("allocated", {})}
    st_res = {f"ATC-STD-{k}" for k in status.get("reserved", {})}
    st_free = {f"ATC-STD-{n}" for n in _free_ids(status)}
    counts = alloc_sec["counts"]

    errs = 0
    print(
        f"Allocation-Gate Familie {args.family}: Registry {len(allocated)} | "
        f"Verweise {len(reserved)} | Status-Datei {len(st_alloc)}/{len(st_res)}/{len(st_free)}"
    )
    errs += fail_count(allocated - st_alloc, "ALLOC-01 Registry-ID fehlt als ALLOCATED", errs)
    for x in st_alloc - allocated:
        errs += fail(f"ALLOC-02 als ALLOCATED gefuehrt, aber nicht in Registry: {x}")
    for x in reserved - st_res:
        errs += fail(f"ALLOC-03 VERWEIST-/FAM-Slot fehlt als RESERVED: {x}")
    for x in st_res - reserved:
        errs += fail(f"ALLOC-04 als RESERVED gefuehrt ohne Verweis-Beleg: {x}")
    for x in st_free & (allocated | reserved):
        errs += fail(f"ALLOC-05 als FREE gefuehrt, aber belegt: {x}")
    if counts.get("allocated") != len(st_alloc):
        errs += fail(
            f"ALLOC-06a counts.allocated falsch ({counts.get('allocated')} != {len(st_alloc)})"
        )
    if counts.get("reserved") != len(st_res):
        errs += fail(
            f"ALLOC-06b counts.reserved falsch ({counts.get('reserved')} != {len(st_res)})"
        )
    if counts.get("free_numerically_unobserved") != len(st_free):
        errs += fail(
            f"ALLOC-06c counts.free falsch ({counts.get('free_numerically_unobserved')} != {len(st_free)})"
        )
    if len(st_alloc) + len(st_res) + len(st_free) != 100:
        errs += fail(
            f"ALLOC-06d Familie nicht vollstaendig abgebildet "
            f"({len(st_alloc)}+{len(st_res)}+{len(st_free)} != 100)"
        )
    if alloc_sec.get("rule_free_is_not_allocatable") is not True:
        errs += fail("ALLOC-07 rule_free_is_not_allocatable != true")

    try:
        resv = yaml.safe_load(open(args.reservations, encoding="utf-8"))
        for r in resv.get("reservations", []):
            rid = r.get("id", "")
            if rid.startswith("ATC-STD-") and rid not in st_res:
                errs += fail(
                    f"ALLOC-08 {rid} in reservations.yaml, aber nicht RESERVED in allocation-status"
                )
    except FileNotFoundError:
        errs += fail("ALLOC-08 reservations.yaml nicht gefunden")

    # ALLOC-11/12: Familien-Allokation fuer fam:-verkabelte Kategorien (SCR-0120 v8)
    cat_path = os.path.join(os.path.dirname(os.path.abspath(args.framework)), "categories.yaml")
    if os.path.exists(cat_path):
        cats = yaml.safe_load(open(cat_path, encoding="utf-8"))
        fw_raw2 = open(args.framework, encoding="utf-8").read()
        fam_entries = {}  # FAM-ID -> (name, range)
        for m in re.finditer(r"- id: (FAM-\d+)\n\s+name: (.+?)\n\s+range: (.+?)(?:\n|$)", fw_raw2):
            fam_entries[m.group(1)] = (m.group(2).strip('"'), m.group(3).strip('"'))
        real = {k: v for k, v in cats.items() if k != "categories" and isinstance(v, dict)}
        wired = {k: v for k, v in real.items() if v.get("fam")}
        legacy = len(real) - len(wired)
        for cat, meta in sorted(wired.items()):
            fam = meta["fam"]
            if fam not in fam_entries:
                errs += fail(
                    f"ALLOC-11 Kategorie '{cat}' verweist auf {fam}, aber {fam} fehlt in framework.yaml"
                )
                continue
            frange = fam_entries[fam][1]
            if not _range_covers(frange, meta.get("range", ""), namespace_only=True):
                errs += fail(
                    f"ALLOC-11 {fam}-Range '{frange}' deckt Kategorie-Range '{meta.get('range', '')}' ({cat}) nicht ab"
                )

        # ALLOC-12: Jeder Registry-Standard MUSS von mindestens einer FAM-Range
        # gedeckt sein; liegt er im Namespace der Familien-Range seiner Kategorie,
        # zwingend durch genau diese Familie (Namespace-Konsistenz).
        uncovered, own_miss = [], []
        for s in reg.get("standards", []):
            sid = s.get("id", "")
            covered_by = [fid for fid, (_, rg) in fam_entries.items() if _range_covers(rg, sid)]
            if not covered_by:
                uncovered.append(sid)
                continue
            cat = s.get("category")
            if cat in wired and wired[cat]["fam"] in fam_entries:
                frange = fam_entries[wired[cat]["fam"]][1]
                if (
                    _range_covers(frange, sid, namespace_only=True)
                    and wired[cat]["fam"] not in covered_by
                ):
                    own_miss.append((sid, cat, wired[cat]["fam"]))
        for sid in sorted(uncovered):
            errs += fail(
                f"ALLOC-12 {sid} liegt in keiner FAM-Range — keine formale Familien-Allokation"
            )
        for sid, cat, fam in own_miss:
            errs += fail(
                f"ALLOC-12 {sid} (Namespace von Kategorie '{cat}') liegt ausserhalb der {fam}-Range"
            )
        print(
            f"ALLOC-12 Standard-Abdeckung: {len(reg.get('standards', [])) - len(uncovered) - len(own_miss)}/{len(reg.get('standards', []))} "
            f"Registry-Standards formal durch FAM-Ranges gedeckt"
        )
        if legacy:
            print(
                f"ALLOC-11/12 Familien-Allokation: {len(wired)} fam-verkabelte Kategorien geprueft "
                f"({legacy} Legacy-Kategorien ohne fam:-Feld, grandfathered)"
            )
        else:
            print(
                f"ALLOC-11/12 Familien-Allokation: ALLE {len(wired)} Kategorien fam-verkabelt — "
                f"Grandfathering vollstaendig entfallen (Owner-Direktive 14.09., SCR-0120 v9)"
            )

    if errs:
        print(
            f"Ergebnis: FAIL — {errs} Allocation-Drift-Funde. Registry-Update oder "
            f"allocation-status.yaml-Update erforderlich."
        )
        sys.exit(1)
    print("Ergebnis: PASS — allocation-status konsistent mit Registry + Framework.")


def fail_count(missing, msg, _):
    n = 0
    for x in sorted(missing):
        fail(f"{msg}: {x}")
        n += 1
    return n


def _free_ids(status):
    """Parst ids wie '305, 315-319, 336-339' in eine Liste von Slot-Nummern."""
    ids = []
    raw = status.get("free_numerically_unobserved", {}).get("ids", [])
    parts = (
        [str(p).strip() for p in raw]
        if isinstance(raw, list)
        else [p.strip() for p in str(raw).split(",")]
    )
    for part in parts:
        if not part:
            continue
        m = re.fullmatch(r"(\d+)-(\d+)", part)
        if m:
            ids.extend(range(int(m.group(1)), int(m.group(2)) + 1))
        elif part.isdigit():
            ids.append(int(part))
        else:
            raise ValueError(f"Ungueltiger FREE-Slot: {part!r}")
    return ids


if __name__ == "__main__":
    main()

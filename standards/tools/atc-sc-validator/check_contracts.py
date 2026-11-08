#!/usr/bin/env python3
"""Contract-Registry-Validator — ATC-STD-SC-019 (Gates SC-G12-Vorbereitung).

Prueft contracts/registry/contracts.yaml und deployments.yaml gegen
SC-002 (Identitaet) und SC-019 (Registry-Pflichten).
Aufruf: python3 tools/atc-sc-validator/check_contracts.py [atc-standards-pfad]
"""

import re
import sys

import yaml

REQ_FIELDS = [
    "contract_id",
    "name",
    "category",
    "repository",
    "version",
    "status",
    "language",
    "runtime",
    "owner",
    "upgradeable",
    "verified",
    "audited",
]
CID = re.compile(r"^ATC-SC-[A-Z]+-[0-9]{3,}$")
DEP = re.compile(r"^ATC-DEP-[0-9]{3,}$")
CATS = {
    "SC-CORE",
    "SC-TOKEN",
    "SC-NFT",
    "SC-DEFI",
    "SC-GOV",
    "SC-MARKET",
    "SC-GAME",
    "SC-MINING",
    "SC-BRIDGE",
    "SC-IDENTITY",
    "SC-ORACLE",
    "SC-SYSTEM",
}


def main():
    root = sys.argv[1] if len(sys.argv) > 1 else "."
    fails = []
    contracts = yaml.safe_load(open(root + "/contracts/registry/contracts.yaml"))["contracts"]
    deps = yaml.safe_load(open(root + "/contracts/registry/deployments.yaml"))["deployments"]
    ids = set()
    for c in contracts:
        cid = c.get("contract_id", "??")
        if not CID.match(str(cid)):
            fails.append("contract_id-Muster: " + str(cid))
        if cid in ids:
            fails.append("doppelte contract_id: " + str(cid))
        ids.add(cid)
        if c.get("category") not in CATS:
            fails.append("unzulaessige Kategorie: %s (%s)" % (c.get("category"), cid))
        for f in REQ_FIELDS:
            if f not in c:
                fails.append("Pflichtfeld fehlt: %s (%s)" % (f, cid))
        if not re.match(r"^\d+\.\d+\.\d+$", str(c.get("version", ""))):
            fails.append("Version nicht SemVer: " + str(cid))
    for d in deps:
        if not DEP.match(str(d.get("deployment_id", ""))):
            fails.append("deployment_id-Muster: " + str(d.get("deployment_id")))
        if d.get("contract_id") not in ids:
            fails.append("Deployment referenziert unbekannten Contract: %s" % d.get("contract_id"))
    print("== Contract-Registry (ATC-STD-SC-019) ==")
    print("  Contracts: %d · Deployments: %d" % (len(contracts), len(deps)))
    for f in fails:
        print("  [FAIL] " + f)
    print("RESULT: " + ("NON-COMPLIANT (%d)" % len(fails) if fails else "CONFORM"))
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())

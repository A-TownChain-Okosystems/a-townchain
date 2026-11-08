#!/usr/bin/env python3
import argparse
import hashlib
import json
from pathlib import Path

from dependency_guard import graph, scan


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=".")
    ap.add_argument("--out", default="artifacts/dependency")
    args = ap.parse_args()
    root = Path(args.root).resolve()
    out = Path(args.out)
    nodes, errors = scan(root)
    if errors:
        raise SystemExit(
            "SBOM blocked by dependency scan errors: " + json.dumps(errors, sort_keys=True)
        )
    g = graph(nodes, out)
    graph_sha = hashlib.sha256(json.dumps(g, sort_keys=True).encode()).hexdigest()
    packages = []
    for n in g["nodes"]:
        ref = f"pkg:{n['ecosystem']}/{n['name']}@{n['version']}"
        p = {
            "SPDXID": "SPDXRef-" + hashlib.sha256(ref.encode()).hexdigest()[:24],
            "name": n["name"],
            "versionInfo": n["version"],
            "downloadLocation": n.get("source") or "NOASSERTION",
            "filesAnalyzed": False,
            "licenseConcluded": "NOASSERTION",
            "licenseDeclared": "NOASSERTION",
            "copyrightText": "NOASSERTION",
            "externalRefs": [
                {
                    "referenceCategory": "PACKAGE-MANAGER",
                    "referenceType": n["ecosystem"],
                    "referenceLocator": ref,
                }
            ],
        }
        if n.get("checksum"):
            p["checksums"] = [{"algorithm": "SHA256", "checksumValue": n["checksum"]}]
        packages.append(p)
    doc = {
        "spdxVersion": "SPDX-2.3",
        "dataLicense": "CC0-1.0",
        "SPDXID": "SPDXRef-DOCUMENT",
        "name": "ATC Dependency SBOM",
        "documentNamespace": "urn:atc:sbom:" + graph_sha,
        "creationInfo": {
            "created": "1970-01-01T00:00:00Z",
            "creators": ["Tool: ATC Native Dependency SBOM"],
        },
        "packages": sorted(packages, key=lambda x: (x["name"], x["versionInfo"], x["SPDXID"])),
        "relationships": [],
    }
    byref = {}
    for p in doc["packages"]:
        for r in p["externalRefs"]:
            byref[r["referenceLocator"]] = p["SPDXID"]
    for e in g["edges"]:
        a = byref.get("pkg:" + e["from"].replace(":", "/", 1))
        b = byref.get("pkg:" + e["to"].replace(":", "/", 1))
        if a and b:
            doc["relationships"].append(
                {"spdxElementId": a, "relationshipType": "DEPENDS_ON", "relatedSpdxElement": b}
            )
    doc["relationships"].sort(key=lambda x: (x["spdxElementId"], x["relatedSpdxElement"]))
    out.mkdir(parents=True, exist_ok=True)
    (out / "sbom.spdx.json").write_text(json.dumps(doc, indent=2, sort_keys=True) + "\n")
    (out / "sbom-result.json").write_text(
        json.dumps(
            {
                "schema": "ATC-DEP-SBOM-1",
                "status": "PASS",
                "source_graph_sha256": graph_sha,
                "package_count": len(packages),
            },
            indent=2,
            sort_keys=True,
        )
        + "\n"
    )


if __name__ == "__main__":
    main()

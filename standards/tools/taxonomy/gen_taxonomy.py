#!/usr/bin/env python3
"""Generiert registry/taxonomy.yaml — die ATC Standards Taxonomy (ATC-STD-TAXONOMY-001 §8).
SSOT der vierstufigen Hierarchie (Domain→Familie→Kategorie→Standard); Bestands-Abbild
aus registry/categories.yaml + registry/standards.yaml. Regenerierung nur via TCR + SCR."""

import datetime as _dt
import re as _re

import yaml

# ── Domain-Zuordnung: JEDER Bestands-Familie MUSS ein Domain zugewiesen sein.
# Neue Kategorien ohne Mapping → AssertionError (erzwingt TAX-CHECK-013-Vollständigkeit).
DOMAIN_MAP = {
    # numerische ID-Bereiche (ATC-STD-000 §7)
    "governance": "GOV",
    "architecture": "SW",
    "repository": "SW",
    "development": "SW",
    "security": "TRUST",
    "protocol": "CHAIN",
    "blockchain": "CHAIN",
    "ai": "AI",
    "os": "SW",
    "infrastructure": "SW",
    "applications": "SW",
    # benannte Familien
    "bug": "GOV",
    "zkp": "TRUST",
    "ai-dev": "AI",
    "aas": "AI",
    "aurora-swarm": "AI",
    "enterprise": "GOV",
    "v2s": "GOV",
    "readme": "SW",
    "md": "SW",
    "sc": "CHAIN",
    "net": "CHAIN",
    "desc": "GOV",
    "version": "GOV",
    "audit": "GOV",
    "ai-decision": "AI",
    "update": "GOV",
    "repo-audit": "GOV",
    "repo-maint": "GOV",
    "err": "GOV",
    "cicd": "GOV",
    "implementation": "GOV",
    "repo-discovery": "GOV",
    "agent-operating": "AI",
    "master-audit": "GOV",
    "framework": "GOV",
    "milestone": "GOV",
    "compat": "GOV",
    "governance-core": "GOV",
    # neu via SCR-0024/SCR-0037
    "taxonomy": "GOV",
    "license": "GOV",
    # neu via SCR-0056
    "improvement": "GOV",
    "ai-gov": "AI",
    # neu via SCR-0092 (11.09., Drift-Fix 13.09.: fehlte beim ENG-001-Commit -> KeyError auf main)
    "eng": "SW",
    # neu via SCR-0117 (13.09.)
    "legal": "GOV",
    # neu via SCR-0120 (14.09.)
    "maint": "SW",
}
DOMAINS = {
    "GOV": "Governance & Meta-Standards",
    "SW": "Software Engineering",
    "CHAIN": "Blockchain, Smart Contracts & Protokolle",
    "AI": "Artificial Intelligence & Agenten",
    "TRUST": "Security & Trust",
}
# Familien-Codes (global eindeutig, TAX-CHECK-003)
FAMILY_CODES = {
    "bug": "BUG",
    "v2s": "V2S",
    "zkp": "ZKP",
    "ai-dev": "AID",
    "aas": "AAS",
    "enterprise": "ENT",
    "readme": "RDM",
    "md": "MD",
    "sc": "SC",
    "net": "NET",
    "desc": "DESC",
    "version": "VER",
    "audit": "AUD",
    "ai-decision": "AIDEC",
    "update": "UPD",
    "repo-audit": "RA",
    "repo-maint": "RM",
    "err": "ER",
    "cicd": "CI",
    "implementation": "IM",
    "repo-discovery": "RD",
    "protocol": "PROT",
    "agent-operating": "AOS",
    "master-audit": "MAUD",
    "framework": "FW",
    "milestone": "MIL",
    "compat": "CMP",
    "taxonomy": "TAX",
    "governance-core": "SGC",
    "license": "LIC",
    "improvement": "IMP",
    "ai-gov": "AIG",
    "eng": "ENG",
    "legal": "LEGAL",
    "maint": "MAINT",
    "aurora-swarm": "ASW",
}

cats = yaml.safe_load(open("registry/categories.yaml", encoding="utf-8"))
std = yaml.safe_load(open("registry/standards.yaml", encoding="utf-8"))

# Familien sammeln: numerische Bereiche + benannte
families = {}
for num, meta in cats.get("categories", {}).items():
    families[meta["name"]] = {
        "code": "N" + str(num),
        "range": meta["range"],
        "description": meta["description"],
        "source": "numeric-§7",
        "fam": meta.get("fam"),
    }
for key, meta in cats.items():
    if key == "categories" or not isinstance(meta, dict):
        continue
    if meta["name"] in families:  # Namenskollision numeric/named (protocol)
        families[meta["name"] + " (familie)"] = {
            "code": FAMILY_CODES[meta["name"]],
            "range": meta["range"],
            "description": meta["description"],
            "source": "named-family",
            "fam": meta.get("fam"),
        }
    else:
        families[meta["name"]] = {
            "code": FAMILY_CODES[meta["name"]],
            "range": meta["range"],
            "description": meta["description"],
            "source": "named-family",
            "fam": meta.get("fam"),
        }
families["taxonomy"] = {
    "code": "TAX",
    "range": "ATC-STD-TAXONOMY-001-999",
    "description": "Standards Taxonomy & Family Creation (Meta-Governance, SCR-0024)",
    "source": "named-family",
}

# Standards je Familie zählen (Registry-Konsistenz TAX-CHECK-013/014)
std_count = {}
for e in std["standards"]:
    cat = e.get("category", "?")
    # Namenskollision numeric/named ("protocol"): Standards gehoeren zur benannten Familie
    key = cat + " (familie)" if (cat in families and (cat + " (familie)") in families) else cat
    std_count[key] = std_count.get(key, 0) + 1

dom_fams = {d: [] for d in DOMAINS}
for name, meta in families.items():
    dom = DOMAIN_MAP[name.replace(" (familie)", "")]
    assert dom in DOMAINS, f"unbekannter Domain fuer {name}"
    fam = {
        "id": meta["code"],
        "name": name,
        "status": "ACTIVE",
        "lifecycle": "ACTIVE",
        "id_range": meta["range"],
        "source": meta["source"],
        "description": meta["description"],
        "standards_count": std_count.get(name, 0),
        "categories": [
            {
                "id": "GENERAL",
                "name": "Allgemein (Bestand, keine Unterteilung)",
                "status": "ACTIVE",
                "lifecycle": "ACTIVE",
            }
        ],
    }
    if meta.get("fam"):
        fam["fam"] = meta["fam"]  # FAM-Allokationsverweis (framework.yaml, ALLOC-11)
    dom_fams[dom].append(fam)

# Kategorie-Ebene: für neue Familien via ATC-CAT-REQ; Bestand führt GENERAL (Grandfathering)


def reg_lookup(mid):
    """SCR-0091: Status/Version aus Registry-SSOT abgeleitet — keine Hartcodes."""
    for line in open("registry/standards.yaml", encoding="utf-8"):
        m = _re.search(r"- \{id: " + mid + r',.*?version: "?([\d.]+)"?, status: (\w+)', line)
        if m:
            return {"id": mid, "version": m.group(1), "status": m.group(2).upper()}
    raise SystemExit(
        "ERROR: taxonomy entry '" + mid + "' has no corresponding registry entry (SCR-0091)"
    )


def reg_ver(mid):
    return reg_lookup(mid).get("version", "n/a")


data = {
    "taxonomy": {
        "standard": "ATC-STD-TAXONOMY-001",
        "version": reg_ver("ATC-STD-TAXONOMY-001"),
        "generated": _dt.date.today().isoformat(),
        "hierarchy": ["DOMAIN", "FAMILY", "CATEGORY", "STANDARD"],
        "id_scheme_new_families": "ATC-STD-<FAMCODE>[-<CATEGORY>]-NNN",
        "grandfathering": "Bestehende Standards behalten ihre IDs (§30); Bestands-Familien fuehren Kategorie GENERAL bis Unterteilung via ATC-CAT-REQ.",
        "lifecycle": ["PROPOSED", "ANALYZED", "APPROVED", "ACTIVE", "DEPRECATED", "RETIRED"],
        "requests": {
            "family": "ATC-FAM-REQ-NNN",
            "category": "ATC-CAT-REQ-NNN",
            "change": "ATC-TCR-NNN",
        },
        "tax_checks": "TAX-CHECK-001..018 (S-24 automatisiert 001-006, 013, 014, 015)",
        "governance_core": {
            "family": "FAM-43 Standards Governance Core",
            "members": [
                reg_lookup(mid)
                for mid in [
                    "ATC-STD-TAXONOMY-001",
                    "ATC-STD-STDDEV-001",
                    "ATC-STD-REGISTRY-001",
                    "ATC-STD-CHANGE-001",
                    "ATC-STD-AUDIT-001",
                ]
            ],
        },
        "documented_negativfall": "Owner-Beispielfamilie AIA (AI Agents) ueberlappt Bestands-Familie AAS (ATC-AAS-001..025) — TAX-CHECK-008/010 wuerden ATC-FAM-REQ zurueckweisen bzw. auf MERGE lenken.",
        "domains": [
            {
                "id": code,
                "name": name,
                "status": "ACTIVE",
                "lifecycle": "ACTIVE",
                "families": dom_fams[code],
            }
            for code, name in DOMAINS.items()
        ],
    }
}
# Validierung der Eindeutigkeit (TAX-CHECK-002..005)
codes = [f["id"] for d in data["taxonomy"]["domains"] for f in d["families"]]
assert len(codes) == len(set(codes)), "Familien-Codes nicht eindeutig"
names = [f["name"] for d in data["taxonomy"]["domains"] for f in d["families"]]
assert len(names) == len(set(names)), "Familien-Namen nicht eindeutig"

with open("registry/taxonomy.yaml", "w", encoding="utf-8") as fh:
    fh.write("# ATC Standards Taxonomy — ATC-STD-TAXONOMY-001 §8 (SSOT)\n")
    fh.write(
        "# Generiert von tools/taxonomy/gen_taxonomy.py — Aenderungen nur via ATC-TCR + SCR.\n"
    )
    yaml.dump(data, fh, allow_unicode=True, sort_keys=False, width=200)

total_std = sum(f["standards_count"] for d in data["taxonomy"]["domains"] for f in d["families"])
print(
    f"taxonomy.yaml: {len(DOMAINS)} Domains, {len(codes)} Familien "
    f"({sum(1 for c in codes if c.startswith('N'))} numerisch, {len(codes) - sum(1 for c in codes if c.startswith('N'))} benannt), "
    f"{total_std} Standards zugeordnet (Registry-Konsistenz)"
)

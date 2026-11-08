#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Generiert die machine-readable Maintenance-Conformance-Artefakte (SCR-0120 v7):
  1. registry/maintenance-conformance.yaml  (Conformance-SSOT, MAINT-000 §20.1)
  2. registry/maintenance-requirements.yaml (Requirements-Registry, MAINT-000 §6)
  3. standards/maintenance/INDEX.md          (Family Index, MAINT-000 §25)
Leitet alles aus den normativen Dokumenten (standards/maintenance/) und der Registry ab.
Regenerierung nach jeder Aenderung an der MAINT-Familie."""

import io
import os
import re
import sys

import yaml

BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MD_DIR = os.path.join(BASE, "standards", "maintenance")
OUT_CONF = os.path.join(BASE, "registry", "maintenance-conformance.yaml")
OUT_REQ = os.path.join(BASE, "registry", "maintenance-requirements.yaml")
OUT_IDX = os.path.join(BASE, "standards", "maintenance", "INDEX.md")

META = {
    "000": (
        "governance_classification",
        "P0",
        "governance",
        "Governance (this standard: contract layer only)",
    ),
    "001": ("governance_classification", "P0", "governance", "Classification"),
    "002": ("governance_classification", "P0", "governance", "Lifecycle"),
    "003": ("engineering", "P1", "technical", "Code Maintenance"),
    "004": ("engineering", "P0", "technical", "Dependency Maintenance"),
    "005": ("security", "P0", "technical", "Security Maintenance"),
    "006": ("engineering", "P1", "technical", "Infrastructure Maintenance"),
    "007": ("platform", "P0", "technical", "OS Maintenance"),
    "008": ("platform", "P0", "technical", "Blockchain Maintenance"),
    "009": ("platform", "P0", "technical", "VM & Runtime Maintenance"),
    "010": ("platform", "P1", "technical", "AI Maintenance"),
    "011": ("governance_assets", "P1", "operational", "Repository Maintenance"),
    "012": ("governance_assets", "P1", "operational", "Standards Maintenance"),
    "013": ("governance_assets", "P2", "operational", "Documentation Maintenance"),
    "014": ("engineering", "P1", "operational", "Performance Maintenance"),
    "015": ("assurance", "P0", "operational", "Reliability Maintenance"),
    "016": ("lifecycle", "P1", "operational", "Compatibility Maintenance"),
    "017": ("lifecycle", "P0", "operational", "Upgrade & Migration"),
    "018": ("security", "P0", "operational", "Rollback & Recovery"),
    "019": ("assurance", "P0", "evidence_control", "Maintenance Evidence"),
    "020": ("assurance", "P1", "evidence_control", "Maintenance Automation"),
    "021": ("security", "P1", "evidence_control", "Emergency Maintenance"),
    "022": ("lifecycle", "P2", "evidence_control", "End-of-Life / Retirement"),
    "023": ("supply_ecosystem", "P2", "evidence_control", "Vendor & Supply-Chain Maintenance"),
    "024": ("supply_ecosystem", "P2", "evidence_control", "Cross-Ecosystem Maintenance"),
}
GROUPS = {
    "governance_classification": "Governance & Classification",
    "engineering": "Engineering",
    "security": "Security",
    "platform": "Platform",
    "governance_assets": "Governance Assets",
    "lifecycle": "Lifecycle",
    "assurance": "Assurance",
    "supply_ecosystem": "Supply & Ecosystem",
}
DOMAINS = {
    "governance": "MAINT-000..002",
    "technical": "MAINT-003..010",
    "operational": "MAINT-011..018",
    "evidence_control": "MAINT-019..024",
}

GEN_NOTE = "GENERIERT aus standards/maintenance/ + registry/standards.yaml — nicht manuell pflegen (SCR-0120 v7)"


def main():
    reg = yaml.safe_load(
        io.open(os.path.join(BASE, "registry", "standards.yaml"), encoding="utf-8")
    )
    reg_ids = {s["id"]: s for s in reg["standards"] if s.get("category") == "maint"}
    standards = []
    for nr in sorted(META):
        sid = f"ATC-STD-MAINT-{nr}"
        md_path = os.path.join(MD_DIR, f"{sid}.md")
        if not os.path.exists(md_path):
            sys.exit(f"FEHLER: {md_path} fehlt")
        if sid not in reg_ids:
            sys.exit(f"FEHLER: {sid} nicht in Registry")
        md = io.open(md_path, encoding="utf-8").read()
        reqs = re.findall(r"\| (REQ-MAINT-\d{3}) \|", md)
        group, prio, domain, role = META[nr]
        standards.append(
            {
                "id": sid,
                "role": role,
                "group": GROUPS[group],
                "group_key": group,
                "domain": domain,
                "priority_kai_os": prio,
                "parent_standard": None if nr == "000" else "ATC-STD-MAINT-000",
                "inherits_mandatory_baseline": nr != "000",
                "req_count": len(reqs),
                "reqs": reqs,
                "version": str(reg_ids[sid].get("version", "")),
                "status": reg_ids[sid].get("status", ""),
            }
        )

    # 1) Conformance-SSOT
    doc = {
        "maintenance_conformance": {
            "standard": "ATC-STD-MAINT-000 \u00a720.1",
            "version": "1.0.0",
            "generated_note": GEN_NOTE,
            "role": "ATC-STD-MAINT-000 ist der normative Governance- und Contract-Layer der gesamten Maintenance-Familie",
            "parent_standard": "ATC-STD-MAINT-000",
            "family_range": "ATC-STD-MAINT-000..024 (ab 025 frei via SCR)",
            "normative_release_gate": "A system MUST NOT be released to a lifecycle state requiring operational support unless its required maintenance capabilities are implemented, validated, documented, and evidenced.",
            "domains": {k: {"range": v} for k, v in DOMAINS.items()},
            "schemas": {
                "record": "schemas/maintenance/maintenance-record.schema.json",
                "readiness": "schemas/maintenance/maintenance-readiness.schema.json",
                "evidence": "schemas/maintenance/maintenance-evidence.schema.json",
            },
            "requirements_registry": "registry/maintenance-requirements.yaml",
            "family_index": "standards/maintenance/INDEX.md",
            "conformance_workflow": ".github/workflows/maintenance-conformance.yml",
            "registry_records": "registry/standards (ATC-*.yaml, kanonisch via tools/gen_views/gen_registry_records.py; registry_version unabhaengig von Standard-Version, MAINT-000 \u00a722.1)",
            "standards": standards,
        }
    }
    with io.open(OUT_CONF, "w", encoding="utf-8") as f:
        yaml.safe_dump(
            doc, f, allow_unicode=True, sort_keys=False, default_flow_style=False, width=110
        )

    # 2) Requirements-Registry aus den YAML-Bloecken in MAINT-000 §6 parsen
    md0 = io.open(os.path.join(MD_DIR, "ATC-STD-MAINT-000.md"), encoding="utf-8").read()
    blocks = re.findall(r"```yaml\n(requirement:.*?)```", md0, re.S)
    reqdefs, seen = [], set()
    for b in blocks:
        y = yaml.safe_load(b)
        rid = y["requirement"]["id"]
        if rid in seen:
            sys.exit(f"FEHLER: REQ doppelt: {rid}")
        seen.add(rid)
        rid = y["requirement"]["id"]
        if not re.fullmatch(r"REQ-MAINT-\d{3}", rid):
            sys.exit(f"FEHLER: {rid} entspricht nicht der Klasse MAINT (REQ-MAINT-NNN, Variante A)")
        reqdefs.append(y["requirement"])
    table_ids = re.findall(r"\| (REQ-MAINT-\d{3}) \|", md0)
    if sorted(seen) != sorted(set(table_ids)):
        sys.exit(
            f"FEHLER: REQ-Drift YAML-Bloecke vs Tabelle: {sorted(seen)} vs {sorted(set(table_ids))}"
        )
    if len(reqdefs) != 15:
        sys.exit(f"FEHLER: REQ-Definitionen: {len(reqdefs)} (erwartet 15)")
    doc2 = {
        "maintenance_requirements": {
            "standard": "ATC-STD-MAINT-000 \u00a76",
            "version": "1.0.0",
            "generated_note": GEN_NOTE,
            "req_count": len(reqdefs),
            "verification_types": ["evidence", "gate", "ci", "audit"],
            "requirements": reqdefs,
        }
    }
    with io.open(OUT_REQ, "w", encoding="utf-8") as f:
        yaml.safe_dump(
            doc2, f, allow_unicode=True, sort_keys=False, default_flow_style=False, width=110
        )

    # 3) Family Index
    lines = [
        "# Maintenance Family Index (ATC-STD-MAINT-000 \u00a725)",
        "",
        "<!-- " + GEN_NOTE + " -->",
        "",
        "**Parent Standard:** ATC-STD-MAINT-000 \u2014 normative Governance & Contract Layer.  ",
        f"**Standards:** {len(standards)} (P0: {sum(1 for s in standards if s['priority_kai_os'] == 'P0')}, "
        f"P1: {sum(1 for s in standards if s['priority_kai_os'] == 'P1')}, P2: {sum(1 for s in standards if s['priority_kai_os'] == 'P2')})  ",
        f"**REQs gesamt:** {sum(s['req_count'] for s in standards)}  ",
        "**Conformance-SSOT:** registry/maintenance-conformance.yaml \u00b7 **Requirements-Registry:** registry/maintenance-requirements.yaml",
        "",
        "| ID | Rolle | Domain | Prio | Version | Status | REQs |",
        "|---|---|---|---|---|---|---|",
    ]
    for s in standards:
        lines.append(
            f"| [{s['id']}](ATC-STD-MAINT-{s['id'][-3:]}.md) | {s['role']} | {s['domain']} | {s['priority_kai_os']} | {s['version']} | {s['status']} | {s['req_count']} |"
        )
    lines += ["", "Domains: " + " \u00b7 ".join(f"`{k}` ({v})" for k, v in DOMAINS.items()), ""]
    with io.open(OUT_IDX, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))

    p0 = [s["id"] for s in standards if s["priority_kai_os"] == "P0"]
    print(
        f"OK: conformance.yaml ({len(standards)} Standards, {len(p0)}xP0, {sum(s['req_count'] for s in standards)} REQs) "
        f"+ requirements.yaml ({len(reqdefs)} REQ-Definitionen) + INDEX.md"
    )


if __name__ == "__main__":
    main()

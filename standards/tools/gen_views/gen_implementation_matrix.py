#!/usr/bin/env python3
"""gen_implementation_matrix.py — Voll-Klassifikation aller Standards (SCR-0048).

Implementiert ATC-STD-IMPLEMENTATION-001 fuer ALLE Standards: jedes Standard
erhaelt Applicability (§1) + Implementierungs-Status (§2) + Evidence auf Basis
echter Fakten des 26-Repo-Stands. Familienbezogene Klassifikation mit
dokumentierter Regel je Familie (SCR-0048); keine Fake-Status: Standards mit
nicht existierendem Zielsystem = specification_only/NOT-Applikations-Fall,
nie IMPLEMENTED ohne Evidence.
"""

import os
import re
from datetime import datetime

# SCR-0123: Per-Standard-Overrides - evidenzbasierte Einzel-Klassifikation,
# die Familien-Regeln uebersteuert (nie ohne Evidence setzen; Generator-SSOT).
OVERRIDES = {
    "ATC-STD-MAINT-000": (
        "MANDATORY",
        "implemented",
        "Readiness-Gate implementiert (Welle 1, SCR-0123): gen_maintenance_conformance.py + validate_maintenance_readiness.py CI-live (maintenance-conformance.yml); Readiness-Records fuer beide Released-Komponenten (atc-standards, a-townchain-os)",
        "Naechste Releases bis F-ORG-002 + Rollback-Runbook geschlossen: Readiness FAIL = BLOCK (Records ehrlich)",
    ),
}

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

RULES = {
    "repository": (
        "MANDATORY",
        "enforced",
        "Governance-CI 26/26 (fetch-depth:0, AUD-2026-0002 F-048); AGENT_MANIFEST v3.1.7 Rollout 26/26; Branch-Protection SCR-0003",
        "",
    ),
    "security": (
        "MANDATORY",
        "implemented",
        "Secret-Scanning+Push-Protection je Repo; Dependabot-Rollout AUD-2026-0002 F-025; G18-Regel AD-023",
        "CodeQL 0/26 = Coverage-Luecke dokumentiert (P2)",
    ),
    "license": (
        "MANDATORY",
        "enforced",
        "Apache-2.0-Lizensierung 26/26 abgeschlossen (Org-Audit 08.09., FAM-44)",
        "",
    ),
    "governance": (
        "MANDATORY",
        "enforced",
        "atc-standards Governance-CI + Validator-Suite S-01..S-25 ALL COMPLIANT; immutable Tag-Ruleset",
        "",
    ),
    "governance-core": (
        "MANDATORY",
        "enforced",
        "ATC-STD-000 v1.3.0 CANDIDATE; Registry revision is canonical; §9-Approval-Process approval/",
        "",
    ),
    "md": (
        "MANDATORY",
        "enforced",
        "tools/atc-md-validator CONFORM (Meta-Sweep 08.09., SCR-0047)",
        "",
    ),
    "readme": (
        "MANDATORY",
        "enforced",
        "tools/atc-readme-validator 13/13 Gates CONFORM (SCR-0047)",
        "",
    ),
    "bug": (
        "MANDATORY",
        "implemented",
        "Issue-Triage a-townchain-os; docs/error-records/ATC-ERR-0001.yaml; atc-contracts 61/61 Tests",
        "",
    ),
    "err": (
        "MANDATORY",
        "implemented",
        "ATC-ERR-0001 CLOSED; AST-Gate-Test atc-contracts; Dossier docs-hub",
        "",
    ),
    "repo-audit": (
        "MANDATORY",
        "enforced",
        "tools/atc-repo-audit R3 100/100; 64 Checks; AUD-2026-0001/0002",
        "",
    ),
    "repo-maint": (
        "MANDATORY",
        "implemented",
        "Maintenance-Report RUN-001 atc-shivacore (SCR-0043 Pilot)",
        "",
    ),
    "repo-discovery": (
        "MANDATORY",
        "implemented",
        "tools/discovery/scan.py DISC-2026-09-08-001/002; Ausbaustufen RD-004..009 dokumentiert",
        "",
    ),
    "implementation": (
        "MANDATORY",
        "implemented",
        "registry/standard-implementation.yaml + generate_views.py + registry.lock (SCR-0045/0048)",
        "",
    ),
    "audit": (
        "MANDATORY",
        "implemented",
        "AUD-2026-0001/0002; ATC-ORG-AUDIT-002; META-SWEEP-2026-09-08.md",
        "",
    ),
    "agent-operating": (
        "MANDATORY",
        "implemented",
        "AGENT_MANIFEST-Rollout 23/23; 5 Agenten registriert (incl. Aurora-Bot)",
        "",
    ),
    "framework": (
        "MANDATORY",
        "enforced",
        "framework.yaml FAM-01..FAM-49; S-21 Framework-PASS",
        "",
    ),
    "taxonomy": (
        "MANDATORY",
        "enforced",
        "gen_taxonomy.py 431 Standards zugeordnet, Registry-Konsistenz PASS (S-24)",
        "",
    ),
    "desc": ("MANDATORY", "enforced", "DESC-001-Kategorie im Validator geprueft (S-18/S-19)", ""),
    "compat": (
        "CONDITIONAL",
        "implemented",
        "COMPAT-Praefixe im Validator geprueft (S-18/S-25)",
        "",
    ),
    "version": (
        "MANDATORY",
        "enforced",
        "VERSION-001 im Validator; Version-Baseline-Sync 26 Repos = offene P2 (Issue #96)",
        "",
    ),
    "update": (
        "MANDATORY",
        "implemented",
        "Update-Standard; Dependabot als Update-Kanal (AUD-2026-0002 F-025)",
        "",
    ),
    "milestone": ("MANDATORY", "implemented", "ATC-M-001..008 im Validator (S-20 PASS)", ""),
    "master-audit": (
        "MANDATORY",
        "implemented",
        "AUD-Verfahren dokumentiert + angewendet (ATC-ORG-AUDIT-002)",
        "",
    ),
    "architecture": (
        "MANDATORY",
        "implemented",
        "Architektur-Layer AD-026; REPOSITORY_MAP; Rollen AD-046/SCR-0005",
        "",
    ),
    "ai-decision": ("MANDATORY", "implemented", "AID-Datensatz im Validator geprueft (S-22)", ""),
    "ai": (
        "CONDITIONAL",
        "implemented",
        "AI-DEV-Standards: Agent-Governance aktiv (AGENT_MANIFEST, Rollen, Mandate); AI-Kernel-Rebuild K3 offen — Detail-Nachweis je Standard im Coverage-Programm",
        "AI-Familie: vertiefte Einzelerfassung empfohlen",
    ),
    "ai-dev": (
        "MANDATORY",
        "implemented",
        "AI Development Governance 07.09. (12 Standards); Workflow in SCR/Commit-Praxis belegt",
        "",
    ),
    "development": (
        "MANDATORY",
        "specification_only",
        "DTC-Praxis in SCR/Commits belegt (STD-300); G0-G19-Gates: ATCLang-Rebuild-Pipeline AD-023 offen (G1/G2 bestanden)",
        "Rebuild qualitaetsgetrieben",
    ),
    "sc": (
        "CONDITIONAL",
        "implemented",
        "atc-contracts restauriert + Contract-Test-Suite 61/61 GRUEN (Issue #99)",
        "",
    ),
    "blockchain": (
        "CONDITIONAL",
        "specification_only",
        "Legacy-Restore a-townchain (217 Dateien); SC-Rebuild AD-023 qualitaetsgetrieben offen; 76 Chain-Standards warten auf Implementierung im Rebuild",
        "",
    ),
    "zkp": (
        "CONDITIONAL",
        "specification_only",
        "atc-zkp R1-Skeleton (7 Crates, AD-045); Implementierung G18-vor-Freeze im Rebuild",
        "",
    ),
    "v2s": (
        "CONDITIONAL",
        "specification_only",
        "V2S-000 Master integriert; 26 Phasen-Standards V2S-001..026 nicht gebaut (SCR-0042-DoD)",
        "",
    ),
    "net": (
        "CONDITIONAL",
        "specification_only",
        "atc-node R1-Skeleton (S4, AD-046); Bootstrap/Discovery-Standards warten auf Node-Rebuild",
        "",
    ),
    "maintenance": (
        "MANDATORY",
        "specification_only",
        "MAINT-Familie 000..024 (SCR-0120): normative Betriebs-Disziplin; Conformance-Struktur via maintenance-conformance-Gate maschinenpruefbar; operative Umsetzung je Anwendung ausstehend",
        "Readiness-Gate-Tooling Welle 1 SCR-0123: MAINT-000 implemented (Override)",
    ),
    "ai-gov": (
        "MANDATORY",
        "specification_only",
        "ACCESS-001 (SCR-0121): Rollenprofil-Matrix dokumentiert; PAT-Migration auf Fine-grained Stufe-2-Profil offen",
        "",
    ),
    "os": (
        "CONDITIONAL",
        "specification_only",
        "atc-shivacore Rebuild SC-001+ offen (AD-023); Kernel M2/L1 + 22 Security-Dateien vorhanden",
        "",
    ),
    "protocol": (
        "CONDITIONAL",
        "specification_only",
        "ATC-PROTO-P2P-001; Protokoll-Implementierung im Node-Rebuild",
        "",
    ),
    "infrastructure": (
        "CONDITIONAL",
        "specification_only",
        "atc-oracle/storage/compute/mining/launchpad R1-Skelette; Infra-Normen warten auf Implementierung",
        "",
    ),
    "applications": (
        "CONDITIONAL",
        "specification_only",
        "atc-explorer/-marketplace/-wallet/-sdk/-interop/-indexer restauriert (Vault M6/L5); Norm-Anwendung im Rebuild nachzuweisen",
        "",
    ),
    "enterprise": (
        "CONDITIONAL",
        "specification_only",
        "Enterprise-Module ohne aktive Ziel-Repos im 26er-Stand; AD-047-Disposition fuer verwaiste Module gilt",
        "",
    ),
    "aas": (
        "CONDITIONAL",
        "implemented",
        "Agent-Governance aktiv: AGENT_MANIFEST v3.1.7 als Identity/Scope/Discovery-Mechanismus 26/26; Permission via Rollenmodell; Detail-Nachweis je AAS-Standard im Coverage-Programm",
        "vertiefte Einzelerfassung empfohlen",
    ),
    "aas-meta": (
        "REFERENCE",
        "reference",
        "Meta-/Referenz-Eintraege; keine Implementierungspflicht",
        "",
    ),
}


def main():
    reg = open(os.path.join(ROOT, "registry", "standards.yaml"), encoding="utf-8").read()
    entries = []
    for line in reg.splitlines():
        if not line.strip().startswith("- {id: ATC"):
            continue

        def field(name, pat=r"([^,}]+)"):
            m = re.search(name + r":\s*" + pat, line)
            return m.group(1).strip().strip('"') if m else ""

        sid = field("id")
        fam = os.path.dirname(field("file")).replace("standards/", "")
        entries.append((sid, field("version", r'"([^"]*)"'), fam))

    out = []
    out.append("# ============================================================================")
    out.append("# ATC Standard Implementation Matrix (SSOT) — VOLL-KLASSIFIKATION (SCR-0048)")
    out.append("# Standard: ATC-STD-IMPLEMENTATION-001 — Matrix v1.1.0")
    out.append("# Applicability §1: MANDATORY/CONDITIONAL/REFERENCE/NOT_APPLICABLE")
    out.append("# Status §2: enforced/implemented/specification_only/reference")
    out.append("# Klassifikationsregel: familienbezogen mit Evidence (SCR-0048).")
    out.append("# Keine Fake-Status: Zielsystem fehlt => specification_only, nie IMPLEMENTED.")
    out.append("# ============================================================================")
    out.append('version: "1.2.0"')
    out.append("generated_from: registry/standards.yaml")
    out.append('updated: "%s"' % datetime.now().strftime("%Y-%m-%d"))
    out.append("coverage_kpi:")
    out.append("  registry_total: %d" % len(entries))
    out.append("  matrix_entries: %d" % len(entries))
    out.append('  matrix_coverage: "100%"')
    out.append('  enforced_target: "je Wartungszyklus steigend (REQ-IMP-007)"')
    out.append(
        '  hinweis: "Wertigkeit der 21 SCR-0045-Detail-Eintraege bleibt; Familien-Evidence konsolidiert"'
    )
    out.append("")
    out.append("standards:")
    stats = {}
    for sid, ver, fam in entries:
        r = OVERRIDES.get(sid) or RULES.get(fam)
        if r is None:
            applic, impl, ev, note = (
                "REFERENCE",
                "reference",
                "Legacy-Serie (atc/): durch ATC-STD-000ff superseded; historische Referenz konserviert",
                "",
            )
        else:
            applic, impl, ev, note = r
        stats[impl] = stats.get(impl, 0) + 1
        out.append("  - standard: %s" % sid)
        out.append('    version: "%s"' % (ver or "n/a"))
        out.append("    applicability: %s" % applic)
        out.append("    implementation:")
        out.append("      status: %s" % impl)
        out.append('      evidence: "%s"' % ev)
        if note:
            out.append('      note: "%s"' % note)
        out.append("    independence: L1")
        out.append("")
    open(
        os.path.join(ROOT, "registry", "standard-implementation.yaml"), "w", encoding="utf-8"
    ).write("\n".join(out))
    print("Klassifikation:", stats, "| Eintraege:", len(entries))


if __name__ == "__main__":
    main()

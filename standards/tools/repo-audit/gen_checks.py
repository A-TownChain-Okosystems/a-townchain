#!/usr/bin/env python3
"""Genriert registry/repo-audit-checks.yaml — den CHECK-Katalog (ATC-STD-REPO-AUDIT-002 §2).
SSOT der Check-Definitionen; Regenerierung nur via SCR (REQ-RB-011)."""

import yaml

# 16 Bereiche mit Gewicht (Summe MUSS 100 sein)
AREAS = [
    ("FEHLER", 5),
    ("VOLLSTAENDIGKEIT", 7),
    ("LUECKEN", 5),
    ("SICHERHEIT", 12),
    ("CODEQUALITAET", 6),
    ("ARCHITEKTUR", 6),
    ("DOKUMENTATION", 7),
    ("TESTS", 10),
    ("BUILD", 9),
    ("ABHAENGIGKEITEN", 6),
    ("STRUKTUR", 4),
    ("KOMPATIBILITAET", 6),
    ("VERSIONIERUNG", 4),
    ("CICD", 5),
    ("GOVERNANCE", 5),
    ("VERBESSERUNG", 3),
]

# Checks: (Bereich, Titel, Methode, Gewicht, Prüfanweisung)
CHECKS = [
    # FEHLER (REPO-AUDIT-001 §6)
    (
        "FEHLER",
        "TODO/FIXME-Scan",
        "AUTO",
        2,
        "grep-basierter Scan auf TODO/FIXME/XXX im Quellcode; Evidence: Fundliste mit Datei:Zeile",
    ),
    (
        "FEHLER",
        "Dummy-/Platzhalter-Code-Scan",
        "AUTO",
        3,
        "Scan auf unimplementierte Funktionen, Dummy-Rückgaben, 'not implemented' Muster; Evidence: Fundliste",
    ),
    (
        "FEHLER",
        "Dead-Code-/Unreachable-Scan",
        "AUTO",
        1,
        "Lint-Report (clippy dead_code, ruff vulture, tsc noUnusedLocals); Evidence: Lint-Output",
    ),
    (
        "FEHLER",
        "Error-Handling-Pfad-Review",
        "HYBRID",
        3,
        "Stichproben-Review von Fehlerpfaden (kein Silent Failure, keine unhandled exceptions); Evidence: Review-Notiz + Code-Stellen",
    ),
    # VOLLSTAENDIGKEIT (§10)
    (
        "VOLLSTAENDIGKEIT",
        "SOLL/IST-Abgleich gegen Spezifikation",
        "HYBRID",
        3,
        "13 Quellen (README/SPEC/ROADMAP/ISSUES …) gegen Implementierung; Evidence: Anforderungsliste mit Status",
    ),
    (
        "VOLLSTAENDIGKEIT",
        "8-Status-Klassifikation je Anforderung",
        "MANUAL",
        3,
        "Je Anforderung: IMPLEMENTED/PARTIAL/MISSING/BROKEN/UNTESTED/UNDOCUMENTED/DEPRECATED/BLOCKED; Evidence: Matrix",
    ),
    (
        "VOLLSTAENDIGKEIT",
        "Kernfunktionen laut Repo-Zweck vorhanden",
        "MANUAL",
        2,
        "Repo-Zweck vs. implementierter Funktionsumfang; Evidence: Abgleichsnotiz",
    ),
    (
        "VOLLSTAENDIGKEIT",
        "Implizite Anforderungen erkannt",
        "MANUAL",
        1,
        "Nicht dokumentierte, aber erwartete Verhalten identifiziert; Evidence: Liste",
    ),
    # LUECKEN (§11)
    (
        "LUECKEN",
        "GAP-Scan über 13 Kategorien",
        "HYBRID",
        3,
        "GAP-CODE/TEST/DOC/SECURITY/ARCH/API/DATA/DEP/CI/GOV/OPS/RECOVERY/COMPATIBILITY; Evidence: GAP-<KAT>-NNN-Liste",
    ),
    (
        "LUECKEN",
        "Fehlende Standard-Dokumente",
        "AUTO",
        2,
        "SECURITY.md, CONTRIBUTING.md, LICENSE vorhanden?; Evidence: Datei-Existenzliste",
    ),
    (
        "LUECKEN",
        "Test-Gaps vs. öffentliche APIs",
        "HYBRID",
        2,
        "Exportierte/public APIs gegen Testabdeckung; Evidence: Lückenliste",
    ),
    (
        "LUECKEN",
        "TODO-Backlog-Datenbasis aktuell",
        "MANUAL",
        1,
        "Offene Arbeiten erfasst (Issues/TODO-Standard)?; Evidence: Backlog-Auszug",
    ),
    # SICHERHEIT (§12)
    (
        "SICHERHEIT",
        "Secret-Scan",
        "AUTO",
        3,
        "gitleaks/trufflehog: Keys, Tokens, Passwörter, .env, private Keys; Evidence: Scan-Report (ohne Klartext-Secrets)",
    ),
    (
        "SICHERHEIT",
        "Dependency-Vulnerability-Scan",
        "AUTO",
        3,
        "Dependabot/CodeQL/cargo-audit Alerts; Evidence: Alert-Liste",
    ),
    (
        "SICHERHEIT",
        "Input-Validation-Review",
        "HYBRID",
        2,
        "Eingabepfade auf Unsafe Input/Injection geprüft; Evidence: Review-Notiz",
    ),
    (
        "SICHERHEIT",
        "Verschärfung kritischer Bereiche",
        "MANUAL",
        3,
        "Blockchain/Wallet/Contracts/Bridges/Mining/Admin-API/KI-Agenten: erweiterter Umfang (AuthN/AuthZ, Privilege Escalation, Deserialization); Evidence: Prüfprotokoll",
    ),
    # CODEQUALITAET (§6)
    (
        "CODEQUALITAET",
        "Lint clean (clippy/ruff/eslint/tsc)",
        "AUTO",
        2,
        "Linter ohne Fehler; Evidence: Lint-Output",
    ),
    (
        "CODEQUALITAET",
        "Duplikations-Messung",
        "AUTO",
        1,
        "copy-paste-Detektor (jscpd); Evidence: Duplikatsquote",
    ),
    (
        "CODEQUALITAET",
        "Komplexitätsmetrik in Toleranz",
        "HYBRID",
        2,
        "Zyklomatische Komplexität kritischer Funktionen; Evidence: Metrik-Report",
    ),
    (
        "CODEQUALITAET",
        "Naming-/Typisierungs-Review",
        "HYBRID",
        1,
        "Konsistenz von Naming und Typisierung; Evidence: Review-Notiz",
    ),
    # ARCHITEKTUR (§14)
    (
        "ARCHITEKTUR",
        "ARCHITECTURE.md vs. Implementierung",
        "MANUAL",
        3,
        "Soll/Ist-Abgleich inkl. undokumentierter/nicht existierender Komponenten; Evidence: Abgleichsliste",
    ),
    (
        "ARCHITEKTUR",
        "Unerlaubte Abhängigkeiten geprüft",
        "HYBRID",
        2,
        "Layer-/Modulgrenzen (z.B. Kernel ohne std, ATC-STD-204 Interfaces); Evidence: Verstoßliste",
    ),
    (
        "ARCHITEKTUR",
        "Zyklen-Freiheit",
        "AUTO",
        2,
        "Zyklen-Detektor (cargo-depgraph/madge); Evidence: Graph-Report",
    ),
    (
        "ARCHITEKTUR",
        "Separation of Concerns",
        "MANUAL",
        1,
        "Verantwortlichkeiten sauber getrennt; Evidence: Review-Notiz",
    ),
    # DOKUMENTATION (§15-16)
    (
        "DOKUMENTATION",
        "README vollständig & aktuell",
        "HYBRID",
        2,
        "README-001-Konformenz; Evidence: Checkliste",
    ),
    (
        "DOKUMENTATION",
        "CHANGELOG aktuell",
        "AUTO",
        2,
        "Letzte Version im CHANGELOG vs. letzter Release/Tag; Evidence: Diff-Notiz",
    ),
    (
        "DOKUMENTATION",
        "Doku-Code-Konsistenz",
        "HYBRID",
        2,
        "Beispiele/Anleitungen stimmen mit Code überein; Evidence: Stichprobe",
    ),
    (
        "DOKUMENTATION",
        "Codeänderung↔Doku-Prüfung erfolgt",
        "MANUAL",
        2,
        "Bei geänderten Dateien seit letztem Audit: zugehörige Doku geprüft (REQ-RA-016); Evidence: Mapping",
    ),
    # TESTS (§9)
    (
        "TESTS",
        "Test-Suite existiert & läuft",
        "AUTO",
        3,
        "Tests vorhanden, Suite grün; Evidence: Testrunner-Output",
    ),
    (
        "TESTS",
        "Coverage gemessen",
        "AUTO",
        2,
        "Coverage-Report (llvm-cov/coverage.py/c8); Evidence: Coverage-%",
    ),
    (
        "TESTS",
        "Fehlerfall-/Edge-Case-Tests",
        "HYBRID",
        2,
        "Negative Pfade und Grenzfälle getestet; Evidence: Testliste",
    ),
    (
        "TESTS",
        "Regression-Tests für bekannte Bugs",
        "HYBRID",
        3,
        "Geschlossene F-NNN haben Regression-Tests; Evidence: Mapping",
    ),
    # BUILD (§7-8)
    (
        "BUILD",
        "Clean Build reproduzierbar",
        "AUTO",
        3,
        "Clean-Build ohne Fehler; Evidence: Build-Log",
    ),
    ("BUILD", "Lint + Type Check", "AUTO", 2, "statischer Check ohne Fehler; Evidence: Output"),
    (
        "BUILD",
        "Runtime-Smoke-Test",
        "HYBRID",
        3,
        "Installieren/Starten/Initialisieren/Konfiguration/Datenzugriff/Core Functions/Shutdown (§8); Evidence: Smoke-Protokoll",
    ),
    (
        "BUILD",
        "Recovery-Verhalten geprüft",
        "HYBRID",
        1,
        "Neustart/Fehler-Recovery verhält sich definiert; Evidence: Notiz",
    ),
    # ABHAENGIGKEITEN (§13)
    (
        "ABHAENGIGKEITEN",
        "Manifest-Vollständigkeit & Lockdatei",
        "AUTO",
        2,
        "Manifest + Lock konsistent, reproduzierbare Builds; Evidence: Lock-Prüfung",
    ),
    ("ABHAENGIGKEITEN", "Veraltete Dependencies", "AUTO", 2, "outdated-Report; Evidence: Liste"),
    (
        "ABHAENGIGKEITEN",
        "Ungenutzte/doppelte Dependencies",
        "AUTO",
        1,
        "unused-Detektor; Evidence: Liste",
    ),
    (
        "ABHAENGIGKEITEN",
        "Lizenz-Kompatibilität",
        "HYBRID",
        1,
        "Lizenzen je Dep erfasst und kompatibel; Evidence: Lizenzliste",
    ),
    # STRUKTUR (§5)
    (
        "STRUKTUR",
        "Kern-Dateien vorhanden",
        "AUTO",
        2,
        "README/LICENSE/CHANGELOG vorhanden; Evidence: Existenzliste",
    ),
    (
        "STRUKTUR",
        "Struktur passt zum Repo-Zweck",
        "MANUAL",
        2,
        "Vorhandene Struktur vollständig & logisch (keine Schablone); Evidence: Baum-Bewertung",
    ),
    (
        "STRUKTUR",
        "Keine Duplikat-/Konkurrenz-Repos",
        "MANUAL",
        2,
        "Repo-Zweck eindeutig in der Org (Layer-Zuordnung L0-L7); Evidence: Org-Abgleich",
    ),
    (
        "STRUKTUR",
        "Ungenutzte Verzeichnisse/Dateien",
        "AUTO",
        1,
        "Leere/tote Pfade; Evidence: Fundliste",
    ),
    # KOMPATIBILITAET (§20)
    (
        "KOMPATIBILITAET",
        "API-Breaking-Change-Scan",
        "AUTO",
        3,
        "Semver-Diff/ABI-Check gegen letzte Version; Evidence: Diff-Report",
    ),
    (
        "KOMPATIBILITAET",
        "4-Status-Bewertung nach MAJOR",
        "MANUAL",
        3,
        "API/ABI/DB/Config/CLI/SDK/Deps/Protokoll/Formate/Contracts/Repos mit COMPATIBLE/PARTIALLY/INCOMPATIBLE/UNKNOWN; Evidence: Matrix",
    ),
    (
        "KOMPATIBILITAET",
        "UNKNOWN-Verbot bei Release",
        "AUTO",
        3,
        "Kein UNKNOWN in Release-Artefakten (COMPAT-001); Evidence: Status-Scan",
    ),
    (
        "KOMPATIBILITAET",
        "COMPAT-001-Wiederherstellung angewendet",
        "MANUAL",
        1,
        "Bei Inkompatibilität: Methode A-F dokumentiert; Evidence: COMP-NNN-Verweis",
    ),
    # VERSIONIERUNG (§19)
    (
        "VERSIONIERUNG",
        "Git-Tag vs. Datei-Version",
        "AUTO",
        3,
        "Tag == Cargo/package/npm-Version (VERSION-001); Evidence: Abgleich",
    ),
    (
        "VERSIONIERUNG",
        "CHANGELOG vs. Version konsistent",
        "AUTO",
        2,
        "Letzter CHANGELOG-Eintrag == aktuelle Version; Evidence: Abgleich",
    ),
    (
        "VERSIONIERUNG",
        "GitHub-Release/Tag konsistent",
        "AUTO",
        2,
        "Releases & Tags synchron (vgl. F-028 verwaister Tag); Evidence: API-Abgleich",
    ),
    (
        "VERSIONIERUNG",
        "Docker-/Package-Tags aktuell",
        "AUTO",
        1,
        "Published Artefakt-Tags == Quellversion; Evidence: Registry-Abgleich",
    ),
    # CICD (§17)
    (
        "CICD",
        "CI-Workflow vorhanden",
        "AUTO",
        2,
        ".github/workflows mit Build+Test; Evidence: Workflow-Liste",
    ),
    ("CICD", "CI grün auf main", "AUTO", 3, "Letzter Lauf erfolgreich; Evidence: Status-API"),
    (
        "CICD",
        "Security-Scan in Pipeline",
        "AUTO",
        2,
        "Secret-Scan/Vuln-Scan als CI-Schritt; Evidence: Workflow-Inspektion",
    ),
    (
        "CICD",
        "Deployment-/Release-Pipeline",
        "HYBRID",
        1,
        "Automatisierung bis Artifact/Release; Evidence: Pipeline-Beschreibung",
    ),
    # GOVERNANCE (§15/§18, ATC-STD-000)
    (
        "GOVERNANCE",
        "Kanonische Einordnung korrekt",
        "MANUAL",
        2,
        "Layer/Kategorie/Status stimmen mit REALITY_STATUS/a-townchain-os-docs überein; Evidence: Abgleich",
    ),
    (
        "GOVERNANCE",
        "Standards-Einhaltung (Registry-Gate)",
        "AUTO",
        2,
        "Commit-Trailers, Naming, Copyright-Header 'Michael Wroblewski'; Evidence: Scan",
    ),
    (
        "GOVERNANCE",
        "Verantwortlichkeit definiert (CODEOWNERS)",
        "AUTO",
        2,
        "CODEOWNERS/Maintainer hinterlegt; Evidence: Datei-Check",
    ),
    (
        "GOVERNANCE",
        "Registry-/AD-Einträge aktuell",
        "HYBRID",
        1,
        "Repo-/Architektur-Registry-Einträge synchron; Evidence: Abgleich",
    ),
    # VERBESSERUNG (§21)
    (
        "VERBESSERUNG",
        "IMP-Scan über 12 Kategorien",
        "MANUAL",
        2,
        "PERFORMANCE/SECURITY/ARCHITECTURE/MAINTAINABILITY/AUTOMATION/DOCUMENTATION/TESTING/UX/DX/COST/SCALABILITY/OBSERVABILITY; Evidence: IMP-NNN-Liste",
    ),
    (
        "VERBESSERUNG",
        "Performance-Engpass-Scan",
        "HYBRID",
        1,
        "Offensichtliche Engpässe (O(n²)-Loops, fehlende Caches); Evidence: Fundliste",
    ),
    (
        "VERBESSERUNG",
        "Observability-Abdeckung",
        "HYBRID",
        1,
        "Logs/Metrics/Tracing vorhanden; Evidence: Abdeckungsnotiz",
    ),
    (
        "VERBESSERUNG",
        "Automatisierungspotenzial",
        "MANUAL",
        1,
        "Manuelle Schritte, die CI/Agent übernehmen könnte; Evidence: Vorschlagsliste",
    ),
]

assert len(CHECKS) == 64, f"Erwartet 64 Checks, gefunden {len(CHECKS)}"
assert sum(w for _, w in AREAS) == 100, "Bereichsgewichte müssen 100 ergeben"
per_area = {}
for a, *_ in CHECKS:
    per_area[a] = per_area.get(a, 0) + 1
assert all(per_area[a] == 4 for a, _ in AREAS), f"Je Bereich 4 Checks erwartet: {per_area}"

data = {
    "repo-audit-checks": {
        "standard": "ATC-STD-REPO-AUDIT-002",
        "version": "1.0.0",
        "generated": "2026-09-07",
        "note": "CHECK-Katalog gem. ATC-STD-REPO-AUDIT-002 §2 — SSOT der Check-Definitionen. Ergebnis je Check: PASS/WARN/FAIL/SKIP (SKIP nur mit Begründung, REQ-RB-004). Health-Score-Formel und A-E-Mapping: Standard §4-§5. Regenerierung nur via SCR (REQ-RB-011).",
        "areas": [
            {
                "id": a,
                "weight": w,
                "description": {
                    "FEHLER": "Fehler (REPO-AUDIT-001 §6)",
                    "VOLLSTAENDIGKEIT": "Vollständigkeit (§10)",
                    "LUECKEN": "Lücken (§11)",
                    "SICHERHEIT": "Sicherheit (§12)",
                    "CODEQUALITAET": "Codequalität (§6)",
                    "ARCHITEKTUR": "Architektur (§14)",
                    "DOKUMENTATION": "Dokumentation (§15-16)",
                    "TESTS": "Tests (§9)",
                    "BUILD": "Build & Lauffähigkeit (§7-8)",
                    "ABHAENGIGKEITEN": "Abhängigkeiten (§13)",
                    "STRUKTUR": "Repository-Struktur (§5)",
                    "KOMPATIBILITAET": "Kompatibilität (§20)",
                    "VERSIONIERUNG": "Versionierung (§19)",
                    "CICD": "CI/CD (§17)",
                    "GOVERNANCE": "Governance (§4/§15/§18)",
                    "VERBESSERUNG": "Verbesserungspotenzial (§21)",
                }[a],
            }
            for a, w in AREAS
        ],
        "checks": [
            {
                "id": f"CHECK-{i:03d}",
                "area": a,
                "title": t,
                "method": m,
                "weight": w,
                "description": d,
            }
            for i, (a, t, m, w, d) in enumerate(CHECKS, 1)
        ],
    }
}
with open("registry/repo-audit-checks.yaml", "w", encoding="utf-8") as fh:
    fh.write("# ATC Repository Audit CHECK-Katalog — ATC-STD-REPO-AUDIT-002 §2 (SSOT)\n")
    fh.write(
        "# Generiert von tools/repo-audit/gen_checks.py — Änderungen nur via SCR (REQ-RB-011).\n"
    )
    yaml.dump(data, fh, allow_unicode=True, sort_keys=False, width=200)

print(
    f"repo-audit-checks.yaml: {len(AREAS)} Bereiche (Gewichtsumme {sum(w for _, w in AREAS)}), "
    f"{len(CHECKS)} Checks — je Bereich 4; Methoden: "
    f"{{'AUTO': %d, 'HYBRID': %d, 'MANUAL': %d}}"
    % (
        sum(1 for _, _, m, _, _ in CHECKS if m == "AUTO"),
        sum(1 for _, _, m, _, _ in CHECKS if m == "HYBRID"),
        sum(1 for _, _, m, _, _ in CHECKS if m == "MANUAL"),
    )
)

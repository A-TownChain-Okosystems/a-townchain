# atc-contracts [L5]

ATC Contracts — Smart Contracts & Registry (ATCLang-Zielplattform).

**Vault-Restauration (07.09.2026, AD-020/026/027):** Inhalt aus dem Wiki-Vault
(docs/archive/monorepo-full/) restauriert — vor der Repo-Leerung byte-identisch gesichert. Keine — Vault-Stand konsistent.

**Module:** atc-contracts

**Meile (AD-027):** M6 — Dienste laufen

**Hinweis:** Basis fuer den Rebuild; Gate-Kriterien laut LAUFFAEHIGKEITS_ROADMAP
(a-townchain-os-docs/docs/roadmap/).

---

## ATC Compliance & Governance (ATC-STD-201 / 202 / 203)

**ATC COMPLIANCE: R3** — auditiert am 2026-09-07 (atc-repo-audit; R-Level aus `.atc/repository.yaml`).
Architekturentscheidungen: zentral im [DECISIONS_REGISTER](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) (AD-Nummern verbindlich; lokale Entscheidungen in `docs/decisions/`).

- **Purpose:** Smart-Contract-Sammlung der Chain (L5).
- **Scope:** Layer L5, Domain contracts — atc-contracts als SPEC in der 23-Repo-Landschaft (AD-024/026).
- **Architecture:** ATC-LIC License Gate (ATVM blockt unlizentierten Code — Code is Law), ATVM-Verifizierung, .atc-Referenzvertraege (Registry-Standard).
- **Features:** Vertrags-Sammlung + Lizenz-Gate-Referenzen; Chain-ID 658467.
- **Installation:** Modul-Build je Sprache (rust); Integration via Monorepo-Workspace (a-townchain-os, sync_modules.py).
- **Development:** Conventional Commits; Governance-Regeln aus atc-standards; Naming gemaess ATC-STD-000 §7.
- **Testing:** Testplan bis M6 (tests/TESTPLAN.md); Governance-CI.
- **Security:** SECURITY.md; S-Klasse S3; ATC-STD-203 Release-Gates; Emergency-Prozess ATC-STD-000 §32.
- **Roadmap:** Einordnung in die Lauffaehigkeits-Roadmap M1-M8 (AD-027) und Bauhierarchie L0-L7 (AD-026).
- **Version:** CHANGELOG.md; SemVer; Releases als ATC-REL-X.Y.Z.
- **License:** Apache-2.0 — Apache-2.0, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC/ATS-LIC).


## M-Status-Klarstellung (SCR-0080, 10.09.2026)

**M6 = CLAIMED, nicht VERIFIED.** Der M6-Claim ("Dienste laufen") ist gemäß
ATC-STD-MILESTONE-001 (kein Abschluss ohne überprüfbaren Nachweis) ohne
M6-EVIDENCE-BUNDLE ein Claim, kein akzeptierter Meilenstein. Kanonischer,
maschinenlesbarer Stand: `.atc/evidence/evidence.yaml`. Statusleiter:
SPECIFIED → IMPLEMENTED → TESTED → VERIFIED → AUDITED → RELEASED.
CLAIMED ≠ PASS · DOCUMENTED ≠ IMPLEMENTED · IMPLEMENTED ≠ VERIFIED.

# ATC-STD-REPO-001 — Repository Structure Standard
> **Status:** 📐 PROPOSED (v1.0.0) | **Datum:** 07.09.2026 | **Autor:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Standard-ID:** ATC-STD-REPO-001 | **Scope:** Alle offiziellen A-TownChain-Okosystems-Repositories (22 aktiv)
> **Referenzen:** AD-017 (Produkt-Repos kanonisch), AD-025 (Vault-Restauration, modules/-Layout), AD-026 (Bauhierarchie L0-L7), AD-029 (Governance-Mandat), ATC-STD-REPO-002 (Naming/Classification), ATC-STD-REPO-003 (Security/Release)
> **Anwendungsregel:** VERBINDLICH fuer NEUE Repositories ab sofort. Bestands-Repositories dokumentieren ihr Struktur-Mapping in docs/REPOSITORY_STANDARD.md; R2+-Repos migrieren bis M8 (AD-027).

---

## Abstract

ATC-STD-REPO-001 definiert die normative Top-Level-Struktur, die
Verantwortlichkeits-Trennung und die Dokumentations-Mindestausstattung jedes
offiziellen Organisations-Repositories. Ziel: vorhersagbare Repository-Landschaft
— jeder Entwickler (und jeder KI-Agent) findet sich in jedem Repo auf Anhieb zurecht.

> **ATC-STD-REPO-001 = Wo gehoert etwas hin, wenn es benoetigt wird.**
> Nicht jedes Repository benoetigt jeden Ordner — aber wenn ein Ordner existiert,
> muss er der Standard-Struktur folgen.

## 1. Standardisierte Top-Level-Struktur

```
repository/
├── .github/            # workflows/, ISSUE_TEMPLATE/, PULL_REQUEST_TEMPLATE.md, CODEOWNERS, dependabot.yml
├── docs/               # architecture/, specifications/, security/, operations/, decisions/, README.md
├── src/                # Produktionscode (Domain-Struktur gemaess Abschnitt 3)
├── tests/              # unit/, integration/, e2e/, security/, fixtures/
├── examples/           # Nutzer-/Entwicklerbeispiele
├── scripts/            # Build-/Maintenance-Skripte
├── tools/              # interne Entwicklungswerkzeuge
├── configs/           # Konfiguration
├── deployments/        # docker/, kubernetes/, terraform/
├── benches/           # Benchmarks
├── assets/            # nicht-ausfuehrbare Projektressourcen
├── README.md LICENSE SECURITY.md CONTRIBUTING.md CHANGELOG.md
├── CODE_OF_CONDUCT.md VERSION .gitignore
```

## 2. Verantwortlichkeits-Trennung

| Bereich | Zweck |
|---|---|
| src/ | Produktionscode |
| tests/ | Tests |
| docs/ | technische Dokumentation |
| examples/ | Nutzer-/Entwicklerbeispiele |
| scripts/ | Build-/Maintenance-Skripte |
| tools/ | interne Entwicklungswerkzeuge |
| configs/ | Konfiguration |
| deployments/ | Deployment/Infrastructure-as-Code |
| benches/ | Benchmarks |
| .github/ | GitHub Automation |
| assets/ | nicht-ausfuehrbare Ressourcen |

**Grundregel:** Produktionscode gehoert nicht in scripts/, Spezifikationen gehoeren
nicht in src/, und temporaere Entwicklungsartefakte gehoeren ueberhaupt nicht ins
Repository.

## 3. ATC-Domain-Strukturen (Referenzmuster)

**Blockchain-Core:** src/{core, consensus, execution, state, storage, networking,
cryptography, transactions, accounts, tokens, governance, rpc, runtime}/

**ATCLang:** src/{lexer, parser, ast, semantic, compiler, bytecode, vm, runtime,
stdlib}/ — kanonische Auspraegung: AD-022-Struktur (specs/, crates/, python/, tests/
mit conformance/determinism/differential).

**Wallet:** src/{wallet, accounts, keys, transactions, signing, networks, storage, ui}/

**Bestands-Harmonisierung (AD-025):** Die restaurierten Produkt-Repos verwenden
das Vault-Layout modules/<modul>/. Solange aktiv entwickelt wird, dokumentiert
jedes Repo sein Mapping modules/ ↔ src/ in docs/REPOSITORY_STANDARD.md;
bei naechster grosser Restrukturierung ist das src/-Layout Ziel.

## 4. Dokumentations-Mindeststandard

Produktive Repositories benoetigen mindestens:

```
docs/
├── architecture/ARCHITECTURE.md
├── specifications/SPECIFICATION.md
├── security/SECURITY_ARCHITECTURE.md
├── decisions/ADR-NNNN-*.md
└── operations/OPERATIONS.md
```

Zusaetzlich Root: README.md, CHANGELOG.md, SECURITY.md, CONTRIBUTING.md.

**README-Mindestinhalte (12):** 1. Purpose, 2. Scope, 3. Architecture,
4. Features, 5. Repository Structure, 6. Installation, 7. Development, 8. Testing,
9. Security, 10. Roadmap, 11. Versioning, 12. License.

## 5. ADR-Standard (Architekturentscheidungen)

Architekturentscheidungen duerfen NICHT nur in Issues oder Chatverlaeufen
existieren. Repo-lokale Entscheidungen: `docs/decisions/ADR-NNNN-<slug>.md` nach
dem Schema Status/Date/Context/Decision/Alternatives/Consequences/
Security Impact/Compatibility Impact/Migration.

**Zwei-Ebenen-Modell (Harmonisierung AD-029):**
- **Organisationsweit** (querschnittsrelevant, API-/Protokoll-abhaengig):
  zentrales DECISIONS_REGISTER.md im Hub (AD-001…AD-029ff) bleibt AUTORITATIV.
- **Repo-lokal:** ADRs nach obigem Schema; verweisen auf uebergeordnete AD-Nummern.

## 6. Repository-Hygiene (Verbotene Inhalte)

Nicht ins Repository: .env, private keys, wallet seeds, API secrets, production
credentials, node_modules/, target/, dist/ (generiert), build/, *.log, tmp/,
debug/, personenbezogene Daten. Erlaubt: `.env.example` als Vorlage.

## 7. Zentrale Standard-Datei

Jedes Repository fuehrt docs/REPOSITORY_STANDARD.md mit: Standard-ID + Version,
Repository-Klassifizierung (ATC-STD-REPO-002), Required-/Optional-Bestandteile,
Struktur-Mapping und Compliance-Level.

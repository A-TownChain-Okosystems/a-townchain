---
standard:
  id: ATC-STD-202
  title: "ATC-STD-202 — Repository Naming & Classification Standard"
  version: "1.2.0"
  status: approved
  category: repository
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  supersedes: ["ATC-STD-REPO-002@1.0.x"]
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-202 — Repository Naming & Classification Standard (v1.2.0, APPROVED)
> **Status:** APPROVED (v1.2.0) — §9-Sammelfreigabe 11.09.2026 (Owner-Chat), approval/APPROVAL-DECISION-2026-09-11.md | **Datum:** 07.09.2026 | **Autor:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Standard-ID:** ATC-STD-202 | **Scope:** GitHub-Organisation A-TownChain-Okosystems
> **Referenzen:** AD-025 (Genesis Chronicles Umbenennung), AD-026 (Bauhierarchie L0-L7), AD-029 (Governance-Mandat), ATC-STD-201/-003
> **Anwendungsregel:** VERBINDLICH — die Klassifizierungstabelle ist die normative Einordnung aller 26 aktiven Repos.

---

## Abstract

ATC-STD-202 definiert Namensregeln, Repository-Typen, Compliance-Level
(R0-R4), die Monorepo-/Multi-Repo-Grenze und die Abhaengigkeitsrichtungen der
gesamten Organisation.

## 1. Namensstandard

**Empfohlen:** `atc-<domain>` (atc-core-Familie: atc-node, atc-wallet, atc-sdk,
atc-bridge, atc-consensus, atc-storage...).

**Eigenstaendige Produktlinien:** atclang, globus-os, aurora-ai, genesis-engine,
genesis-chronicles, a-townchain, atc-shivacore (Kernel-Marke), a-townchain-os
(Integration), a-townchain-os-docs (Hub).

**Verboten:** final-project, new-atc, atc-new, atc-test, atc-final-final.

## 2. Repository-Typen

| Typ | Bedeutung |
|---|---|
| CORE | Kritische Protokoll-/Systemkomponenten |
| SPEC | Standards und Protokollspezifikationen |
| SDK | Developer Libraries |
| APPLICATION | Anwendungen |
| INFRA | Deployment und Infrastruktur |
| AI | KI-Systeme |
| OS | Betriebssystem-/Kernel-Komponenten |
| GAME | GameFi/Gaming |

## 3. Klassifizierung der aktiven Landschaft (normativ, Stand 07.09.2026)

| Repository | Typ | Level | AD-026 | Bemerkung |
|---|---|---|---|---|
| atc-shivacore | CORE/OS | R4 | L1 | Kernel, AD-028 Service-Space-Migration verifiziert |
| atclang | CORE | R4 | L0 | Sprache + ATVM, G1 PASSED |
| a-townchain | CORE | R4 | L3 | Blockchain-Core (Funktion von atc-core), Chain-ID 658467 |
| a-townchain-os | INFRA | R3 | L7 | Integration/Launch-Stack (Monorepo per AD-017) |
| a-townchain-os-docs | SPEC | R3 | parallel | Governance-Hub: Standards, DECISIONS_REGISTER, Vault |
| atc-contracts | SPEC/CORE | R3 | L5 | Referenzvertraege + ATC-Standards-.atc (AD-028) |
| globus-os | OS | R3 | L4 | Userspace-OS |
| aurora-ai | AI | R3 | L2 | Rust Core + Python AI-Layer (AD-021) |
| atc-sdk | SDK | R2 | L5 | inkl. CLI + atcpkg |
| atc-wallet | APPLICATION | R2 | L5 | |
| atc-explorer | APPLICATION | R2 | L5 | |
| atc-marketplace | APPLICATION | R2 | L5 | DEX + Assets |
| atc-indexer | CORE | R2 | L5 | Analytics-Service |
| atc-interop | CORE | R2 | L5 | Bridge-Service |
| genesis-engine | GAME | R2 | L6 | |
| genesis-chronicles | GAME | R2 | L6 | ehem. Shivamon (AD-025) |
| atc-node | CORE | R1 | L5 | Skelett (M6-Neubau) |
| atc-storage | CORE | R1 | L5 | Skelett (Fundament im Kernel) |
| atc-compute | CORE | R1 | L5 | Skelett (Fundament im Kernel) |
| atc-oracle | CORE | R1 | L5 | Skelett (Neuland) |
| atc-mining | CORE | R1 | L5 | Skelett (PoW-Auslagerung) |
| atc-launchpad | APPLICATION | R1 | L5 | Skelett (Neuland) |

**Abweichungsnotiz zu frueheren Vorschlaegen:** Es existiert kein separates
atc-standards-Repo — Standards leben kanonisch im Hub (docs/standards/), die
.atc-Referenzimplementierungen in atc-contracts. shivamon heisst seit AD-025
genesis-chronicles. Die Chain-Core-Rolle von atc-core uebernimmt a-townchain.


## 3a. Rollenfestlegung Chain-Komponenten (SCR-0005 Option A, AD-046 — MUST)

Die Ziel-Rollen der Chain-Komponenten sind normativ (Overlap-Aufloesung,
Technology Profile Audit 07.09., Finding F-011):

| Repo | Rolle (normativ) |
|---|---|
| **a-townchain** | Chain-Protokoll & Bibliothek: State-Modell, Tx-Struktur, Chain-Orchestrierung, Konsens-Anbindung (atc-algorithm), atcnet-Propagation. Python-Bestand = Migrations-Kandidat (ATC-STD-100 REQ-STD-103, Rust-first). |
| **atc-vm** | Virtual Machine / Contract-Ausführung: ATCLang-VM-Runtime als eigenes CORE-Repo (AD-043). Führt Contract-Bytecode aus; Ziel: `cargo build`-fähige VM, auf der atc-node und a-townchain aufbauen. R1/S4/experimental (Registry). |
| **atc-algorithm** | Konsens-Algorithmik: Proof-of-History, Fork-Resolution, Konsens-Anbindung als eigenes CORE-Repo (AD-043). Liefert dem atc-node die Konsens-Bausteine; implementiert keine eigene Chain-Semantik. R1/S4/experimental. |
| **atc-zkp** | ZKP-Layer (AD-045): kryptografische Verifikationsschicht INNERHALB der Chain (pluggable Proof Systems Groth16/PLONK/Halo2/STARK, 7 Rust-Crates) — kein eigenes Netzwerk/Konsens/State. R1/S4/experimental (Standards ATC-STD-ZKP-001..010). |
| **atc-node** | Full-Node-Binary & Runtime: Distribution-Ziel (`git clone && cargo build` -> lauffaehiger Node) mit Bootstrap, Discovery, Validator-Betrieb. BAUT AUF a-townchain (Bibliothek), atc-algorithm (Konsens) und atc-vm (Ausfuehrung) auf — implementiert selbst KEINE Chain-Semantik. |

Abhaengigkeitsrichtung (verbindlich, keine Zyklen, §6):
atc-node -> a-townchain + atc-algorithm + atc-vm. Security: atc-node = S4
(Node-Binary ist Netzwerk-Angriffsflaeche: Bootstrap-Node, Validator-Keys,
§9-Katalog).

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-STD-211 | Die Rollen a-townchain (Chain-Protokoll/Bibliothek) und atc-node (Full-Node-Binary/Runtime) MUessen wie in §3a festgeschrieben eingehalten werden. | §3a |
| id: REQ-STD-212 | atc-node MUSS als Security-Klassifikation S4 fuehren (§9). | §3a |

## 4. Compliance-Level

| Level | Bedeutung | Mindest-Gates |
|---|---|---|
| R0 | Experimental | README + LICENSE |
| R1 | Development | + Struktur-Standard 001, Basis-Tests, CI laeuft |
| R2 | Standardized | + SECURITY.md, volle Test-Pyramide, Release-Gate, ADR-Pflicht |
| R3 | Production | + Security-Audit, Operations-Doku, Versions-Freeze-Disziplin |
| R4 | Critical Infrastructure | + Verifier-/Differential-Kriterien (AD-021/022), Change-Freeze-Zonen |

## 5. Monorepo vs. Multi-Repo

Kein Mega-Monorepo. Separate Repositories je Grosseinheit (Blockchain Core,
ATCLang, Globus OS, Aurora AI, Wallet, Explorer, SDK, Infrastructure, Game).
Monorepo nur INNERHALB enger Produktgrenzen (z.B. atc-wallet mit wallet-core/
-desktop/-mobile/-sdk als Unterordner). Die Integrations-Ausnahme a-townchain-os
(L7, AD-017) ist der einzige organisationsweite Workspace — als
INTEGRATIONSZIEL, nie als Quelle (sync_modules.py, Produkt gewinnt).

## 6. Abhaengigkeitsrichtungen (keine Zyklen)

ATC Standards → Protocol → Core → Node → SDK → Applications.

Konkrete Auspraegung = AD-026-Bauhierarchie:
L0 atclang → L1 Kernel → L2 Aurora → L3 Chain → L4 OS → L5 Dienste → L6 Game
→ L7 Integration. Hoehere Layer duerfen tiefere nutzen, NIEMALS umgekehrt
(Verifikation AD-028: 0 verkehrte Quer-Abhaengigkeiten).

ATCLang-Compilerkette: Specification → Lexer → Parser → AST → Semantic →
Compiler → Bytecode → VM (G1-Spec: specs/language/SPEC.md).

---

## 7. Ownership-Standard (Abschnitt 17, MUST ab R2)

Jedes Repository definiert `.atc/ownership.yaml`:

```yaml
repository: atc-standards
maintainers:
  - ShivaCoreDev
  - aurora-superagent
security:
  team: ShivaCoreDev
architecture:
  team: ShivaCoreDev
release:
  team: ShivaCoreDev
```

GitHub CODEOWNERS MUSS daraus abgeleitet bzw. synchron gehalten werden.

## 8. Lifecycle-Management (Abschnitt 18, MUST)

Zulaessige Stadien und Uebergaenge (kein Springen):

```
experimental → development → beta → production → deprecated → archived
```

Ein Repository DARF NICHT Stadien ueberspringen (z.B. experimental → production
ist UNZULAESSIG). Uebergaenge werden in `.atc/lifecycle.yaml` mit Datum
dokumentiert. `archived` entspricht dem physischen GitHub-Archiv (irreversible
Read-Only-Politik siehe AD-020-Erfahrung).

## 9. Security-Klassifizierung S0-S4 (Abschnitt 31, MUST)

| Klasse | Bedeutung | Zusatz-Anforderungen |
|---|---|---|
| S0 | Public/Non-Critical | Basis |
| S1 | Standard | + Hygiene-Scan |
| S2 | Important | + SECURITY.md, Dependency Policy |
| S3 | Critical | + Security-Audit, Adversarial-Tests |
| S4 | Protocol/Infrastructure Critical | + Reproducible Builds, Signierte Releases, Differential-Kriterien (AD-021/022) |

Die S-Klasse MUSS in `.atc/repository.yaml` (security.criticality) und in
registry/repositories.yaml gefuehrt werden. Hoehere S-Klasse = strengere Gates
(ATC-STD-203 §9). Aktuelle Zuordnung: siehe registry/repositories.yaml.

## 10. ATC Repository Dependency Graph (Abschnitt 32, MUST gepflegt)

Der zentrale Abhaengigkeits-Graph wird in registry/dependencies.yaml gepflegt
(konkrete Auspraegung = AD-026 L0-L7). Er beantwortet: Wer haengt von wem ab,
welche Aenderungen propagieren, wo liegen Single Points of Failure, welche
Komponenten sind vor einem Release erneut zu testen.

## 11. Repository-Registry (Abschnitt 33, MUST)

registry/repositories.yaml im atc-standards-Repository ist die ZENTRALE,
maschinenlesbare Registry aller Repositories (name, classification, maturity,
security, status, layer). Neue Repos MUSSEN bei Anlage registriert werden.
teams.yaml (Team-Zuordnung) und dependencies.yaml (Graph) ergaenzen sie.

## Security Considerations

Struktur-/Hygiene-Regeln sind Security-relevant (keine Secrets im Tree,
ATC-STD-203 zusaetzlich anwendbar). Vertiefte Security-Anforderungen:
ATC-STD-203.

## References

NORMATIVE: ATC-STD-000 (Governance), ATC-STD-202, ATC-STD-203 ·
INFORMATIVE: AD-025/026/028/029/031 (DECISIONS_REGISTER, Hub) ·
IMPLEMENTATION: tools/atc-repo-audit, schemas/repository.schema.yaml.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

- 1.1.0 (2026-09-07): SCR-0005 Option A (Owner-Freigabe) — Abschnitt 3a Rollenfestlegung Chain-Komponenten (a-townchain = Chain-Protokoll/Bibliothek, atc-node = Full-Node-Binary/Runtime, S4); REQ-STD-211/212.

- 1.0.1 (07.09.2026): Unter ATC-STD-000 Governance gestellt; ID von
  ATC-STD-REPO-001 auf ATC-STD-201 umgestellt (supersedes); Metadaten-Header
  ergaenzt.
- 1.0.0 (07.09.2026): Formale Spezifikation (AD-031): RFC-2119,
  Compliance-Matrix M-01…M-16, Validator-Regeln V-01…V-16.

| 1.2.0 | 2026-09-08 | Klassifizierungstabelle auf 26 Repos erweitert: atc-vm, atc-algorithm (AD-043), atc-zkp (AD-045) — schließt AUD-2026-0002/F-028 (Issue #98). |

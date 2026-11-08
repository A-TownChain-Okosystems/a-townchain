# ATC-STD-REPO-002 — Repository Naming & Classification Standard
> **Status:** 📐 PROPOSED (v1.0.0) | **Datum:** 07.09.2026 | **Autor:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Standard-ID:** ATC-STD-REPO-002 | **Scope:** GitHub-Organisation A-TownChain-Okosystems
> **Referenzen:** AD-025 (Genesis Chronicles Umbenennung), AD-026 (Bauhierarchie L0-L7), AD-029 (Governance-Mandat), ATC-STD-REPO-001/-003
> **Anwendungsregel:** VERBINDLICH — die Klassifizierungstabelle ist die normative Einordnung aller 22 aktiven Repos.

---

## Abstract

ATC-STD-REPO-002 definiert Namensregeln, Repository-Typen, Compliance-Level
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
| atc-standards | SPEC | R3 | parallel | KANONISCHES Standards-Repo (AD-030, Commit 0eea944) |
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

**Abweichungsnotiz zu frueheren Vorschlaegen:** Seit AD-030 (07.09.) existiert
doch ein separates atc-standards-Repo als KANONISCHE Standards-Heimat; die
Hub-Kopie docs/standards/ ist Archiv-Snapshot. shivamon heisst seit AD-025
genesis-chronicles. Die Chain-Core-Rolle von atc-core uebernimmt a-townchain. shivamon heisst seit AD-025
genesis-chronicles. Die Chain-Core-Rolle von atc-core uebernimmt a-townchain.

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

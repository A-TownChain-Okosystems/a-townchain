# 📦 Repository-Aufteilung — 5-Produkt-Struktur

> **Beschluss:** 06.09.2026 (AD-014), erweitert durch AD-015 (Kernel-Repo) | **Agent:** Aurora | **Status:** UMGESETZT (API-verifiziert)

Das A-TownChain-Ökosystem wird neben Monorepo und Docs-Hub in **5 eigenständige
Produkt-Repos** gegliedert. Die Module wurden aus `a-townchain-os/src/modules/`
in die Produkt-Repos überführt (Copy — der Monorepo bleibt Integrations- und
Deployment-Zentrale, inkl. Unified Cargo Workspace und Kernel-Build).

## Produkt-Repos

| Repo | Produkt | Module | Umfang |
|---|---|---|---|
| [`atclang`](https://github.com/A-TownChain-Okosystems/atclang) | ATCLang (Programmiersprache) | atclang, atc-atclang, atc-stdlib | 138 Dateien / 32.347 Zeilen |
| [`atc-vm`](https://github.com/A-TownChain-Okosystems/atc-vm) | **A-TownChain Virtual Machine** (ATVM, AD-043) | Bytecode-Verifikation + Ausfuehrung + Runtime (Rust-first; Python-Referenz-Modul folgt aus atclang) | R1-Skeleton |
| [`atc-algorithm`](https://github.com/A-TownChain-Okosystems/atc-algorithm) | **ATC-Algorithmus** — Hybrid Consensus (PoH+PoS+PoW, AD-044) | PoH-Zeitstempel-Kette, PoS/PoW-Hybrid-Auswahl, Finalitaet/Fork-Resolution (Rust-first; PoH-Referenz folgt aus a-townchain) | R1-Skeleton |
| [`atc-zkp`](https://github.com/A-TownChain-Okosystems/atc-zkp) | **ATC ZKP-Layer** — Zero-Knowledge Proof Layer (AD-045) | Verifikationsschicht L1↔Apps: Proof Generator, Circuit Registry, Verification Engine, Commitment-/Nullifier-Manager, ZKVM; pluggable Proof Systems (Groth16/PLONK/Halo2/STARK); 7 Crates (Rust) | R1-Skeleton |
| [`a-townchain`](https://github.com/A-TownChain-Okosystems/a-townchain) | Blockchain (Chain-ID 658467) | atc-blockchain, atcnet, atc-wallet, atc-contracts, atc-bridge, atc-dex, atc-explorer, atc-assets, atc-zkp, atc-governance, atc-dns, atc-testnet | 369 Dateien / 21.604 Zeilen |
| [`globus-os`](https://github.com/A-TownChain-Okosystems/globus-os) | Betriebssystem (Userspace) | atc-kernel, atc-globus-shell/desktop/fs/net/registry/os, atc-bootloader, atc-drivers, atc-linux/windows-edition, atc-mobile | 361 Dateien / 86.395 Zeilen |
| [`atc-shivacore`](https://github.com/A-TownChain-Okosystems/atc-shivacore) | **ShivaCore-Kernel** (Microkernel, AD-012/013) | atc-shivacore (Kernel-Crate, 674/674 Tests), atc-shivacore-tools | 61 .rs-Dateien |
| [`aurora-ai`](https://github.com/A-TownChain-Okosystems/aurora-ai) | KI-System | atc-aurora-core/agents/memory/runtime/ai, atc-aistudio | 377 Dateien / 80.170 Zeilen |
| [`genesis-engine`](https://github.com/A-TownChain-Okosystems/genesis-engine) | Game-Engine | atc-genesis-engine/ecs/creatures/world, atc-game, genesis-chronicles | 115 Dateien / 5.350 Zeilen |

## Verbleib im Monorepo (`a-townchain-os`)

Kern-Stack (`src/`: gateway, blockchain, core, contracts, franchise, game, frontend),
Plattform-Module (atc-backend, atc-gateway, atc-frontend, atc-ui, atc-cli, atc-sdk,
atc-ide, atc-devtools, atc-ci, atc-deploy, atc-monitoring, atc-security, atc-social,
atc-toolchain, atc-devtools, atc-ci, atc-deploy, atc-monitoring, atc-security, atc-social,
atc-analytics, atc-atcpkg, atc-franchise, atc-standards, atc-whitepaper),
Unified Cargo Workspace, Docker-Stack, Tests.

## Regeln (ab 06.09.2026)

1. **Produkt-Entwicklung** (neue Features eines Produkts) läuft im jeweiligen Produkt-Repo.
2. **Integration/Deployment** (Workspace-Build, Docker, Mainnet-Stack) bleibt im Monorepo.
3. **Dokumentation** kanonisch im Docs-Hub (Architektur AD-012/AD-013, Standards, Entscheidungen).
4. Produkt-Repos: Modulpfad `modules/<modulname>/`, README + LICENSE nach Ökosystem-Standard (Apache-2.0 lizenziert — siehe LICENSE).
5. `atclang` behält seine alte Git-Historie (91 Commits); alte Inhalte sind via Datenverlust-Audit (03.09., 0 Verluste) im Monorepo + Hub-Archiv gesichert.

*Live-Stand API-verifiziert am 06.09.2026: alle 5 Produkt-Repos aktiv, HEAD-Commits bestätigt.*

## Alt-Repos: GELÖSCHT (06.09.2026)

Alle 124 archivierten Alt-Repos wurden am 06.09.2026 endgültig gelöscht
(Owner-Entscheidung). Vor der Löschung wurde JEDE der 2.966 Dateien per
SHA-256 gegen Monorepo + Hub verifiziert: 527 Dateien mit abweichendem
Original-Stand (pristin vor Copyright-/Link-Fixes) wurden nach
`docs/archive/repos-rescue/` gerettet, der Rest war byte-identisch in
`a-townchain-os` (src/modules/) bzw. hier im Hub vorhanden — 0 Datenverlust.
Die 9 Issues des Alt-Repos atc-genesis-engine liegen als
`docs/archive/repos-rescue/_ISSUES_DUMP.json`. Details: AD-016.

## Sync-Regel (AD-017, 06.09.2026)

Produkt-Repos = kanonische Modul-Quelle; Monorepo = Integrationsziel.
Einspielen nur via `a-townchain-os/scripts/sync_modules.py` (`--check`/`--sync`).
Modul-Code nie direkt im Monorepo bearbeiten. Details: DECISIONS_REGISTER AD-017.

## Rebuild-Stand (AD-026/027, 07.09.2026)

| Repo | Zustand |
|---|---|
| atclang [L0] | Aufgebaut; G1 PASSED 07.09. (Language Spec + registry.json, Commit abb3262) — naechstes Gate G2 |
| atc-shivacore [L1] | RESTAURIERT + AD-028: Service-Space-Migration (Kernel 394/394 + Service 280/280 = 674/674, Boot gruen) + atc-security-Tooling |
| aurora-ai [L2] | AUS VAULT RESTAURIERT 07.09. (6 Module, M3-Basis) |
| a-townchain [L3] | AUS VAULT RESTAURIERT 07.09. (6 Module, Chain-ID 658467, M4-Basis) |
| globus-os [L4] | AUS VAULT RESTAURIERT 07.09. (10 Module, M5-Basis) |
| 7 L5-Dienste (contracts, wallet, sdk, explorer, indexer, interop, marketplace) | AUS VAULT RESTAURIERT 07.09. (M6-Basen) |
| 6 L5-Dienste (node, storage, compute, oracle, mining, launchpad) | Skelette — Neuentwicklung (kein Vault-Gegenstueck) |
| genesis-engine [L6] | AUS VAULT RESTAURIERT 07.09. (4 Module, M7-Basis) |
| genesis-chronicles [L6] | AUS VAULT RESTAURIERT 07.09. (AD-025-Korrektur Shivamon→Genesis Chronicles, M7-Basis) |
| a-townchain-os [L7] | Leer — Integration als LETZTER Schritt (M8): Kernstack, docker/, scripts/, config/, tests/ liegen bis dahin im Vault |
| a-townchain-os-docs [Hub] | Wiki-VAULT, parallel |

Im Vault verbleiben bis M8: Kernstack (gateway/backend/frontend/core), Module
atc-backend/-frontend/-gateway/-ui/-ide/-devtools/-ci/-deploy/-monitoring/-social/
-franchise/-security/-standards/-whitepaper, atc-kernel (Python-Legacy), sowie die
ATCLang-Legacy-Module (atc-atclang, atclang, atc-vm, atc-stdlib — bewusst NICHT
restauriert, da vom L0-Rebuild abgeloest).
## Zielarchitektur-Landkarte (AD-024 UMGESETZT, 06.09.2026 — alle 14 Repos erstellt)

Alle 14 Repos wurden am 06.09.2026 (21:30–21:45 UTC+2) erstellt und initialisiert.
Vault-Restore: atc-sdk, atc-contracts, atc-wallet, atc-explorer, genesis-chronicles (Bestand aus
monorepo-full). Neu angelegt (Grundstruktur): atc-node, atc-indexer, atc-mining,
atc-interop, atc-oracle, atc-storage, atc-launchpad, atc-marketplace, atc-compute.
Organisation jetzt: 22 aktive Repos.

| Ziel-Repo | Heutiger Ort (Modul im Produkt-Repo/Vault) | Priorität | Promotion-Kriterium |
|---|---|---|---|
| atc-sdk | Monorepo-Plattformmodul atc-sdk (Vault) | P0 | ATCLang Rust-ABI stabil (AD-022-Gates) |
| atc-node | NEU (kein Vault-Bestand) — aus a-townchain/Protocol-Core | P0 | **Rolle (SCR-0005 Option A, AD-046):** Full-Node-Binary & Runtime — Distribution-Ziel (git clone && cargo build -> Node); baut auf a-townchain + atc-algorithm + atc-vm auf; S4 |
| atc-contracts | a-townchain-Modul atc-contracts (Vault) | P0/P1 | ATVM-Contract-Kontext verifiziert |
| atc-wallet | a-townchain-Modul atc-wallet (Vault) | P0/P1 | Runtime-Signing/HD-Referenz stabil |
| atc-explorer + atc-indexer | a-townchain-Modul atc-explorer (Vault); Indexer NEU | P1 | Chain-RPC stabil |
| atc-mining | NEU (Core definiert PoW-Regeln; Mining-Stack separat) | P1 | Consensus-Rebuild fertig |
| atc-interop | a-townchain-Modul atc-bridge (Vault); IBC/Relayer NEU | P1 | Bridge-Security-Konzept (eigene Sicherheitsdomäne) |
| genesis-chronicles (ex-`genesis-chronicles`, umbenannt per AD-025) | Vault-Restore genesis-chronicles | P1 | ECS-/World-Kern stabil |
| atc-oracle | NEU (Lücke „Oracle & External Data Binding") | P2 | Bedarfsfall DeFi/AI-Feeds |
| atc-storage | NEU | P2 | Asset-/NFT-Metadaten-Bedarf |
| atc-launchpad / atc-marketplace | a-townchain-Module atc-dex/atc-assets (Vault) | P2 | TGE-/Produkt-Entscheidung |

> Regel: KEIN neues Repo ohne Content — Abspaltung immer als Restore/Ableitung aus dem
> Vault bzw. Produkt-Repo, nie als leeres Skeleton (Lehre aus der 70-Repo-Ära).

## Bau-Hierarchie: Repos bauen aufeinander auf (AD-026, 07.09.2026 — VERBINDLICH)

Die 26 Repos werden SEQUENZIELL aufeinander aufgebaut. Ein Layer startet erst,
wenn der vorherige sein Gate/Freeze erreicht hat (Ausnahme: Wiki-Hub, parallel).

    [L0] atclang — Sprache, ATC-IR, Bytecode, ATVM (Gates G0-G19; Phase 1 fertig, G1 offen)
      └─> [L1] atc-shivacore — Kernel (SC-001…SC-013, AD-012/013)
            └─> [L2] aurora-ai — Rust Core + Python AI-Layer (AD-021; Kernel-Event-Bridge)
                  └─> [L3] a-townchain — Blockchain L1, Chain-ID 658467 (ATCLang-Verträge + Kernel-Service)
                        └─> [L4] globus-os — Userspace-OS auf ShivaCore (Bootchain AD-013)
                              └─> [L5] Blockchain-Services: atc-node, atc-contracts, atc-wallet,
                                  atc-sdk, atc-storage, atc-compute, atc-oracle, atc-indexer,
                                  atc-explorer, atc-interop, atc-mining, atc-marketplace, atc-launchpad
                                    └─> [L6] genesis-engine → genesis-chronicles (Engine → Spiel)
                                          └─> [L7] a-townchain-os — Monorepo: INTEGRATION aller Layer (AD-017)
    [Hub] a-townchain-os-docs — Wiki/Vault, kontinuierlich parallel

| Layer | Repos | Baut auf | Rebuild-Gate |
|---|---|---|---|
| L0 | atclang | — | G1…G19 (AD-022) |
| L1 | atc-shivacore | L0 (Priorität) | SC-001…SC-013 (AD-013) |
| L2 | aurora-ai | L1 Kernel-Event-Bridge | Rust Core stabil (AD-021) |
| L3 | a-townchain | L0 Verträge + L1 System-Service | Chain-ID 658467, ATVM grün |
| L4 | globus-os | L1 Bootchain | globus-init + Services (AD-013) |
| L5 | 13 Blockchain-Services | L3, teils L4 | je Service-spezifisch |
| L6 | genesis-engine, genesis-chronicles | L4 + L3 Contracts | Engine-Core stabil |
| L7 | a-townchain-os (Monorepo) | ALLE Layer | Integration per AD-017 |
| Hub | a-townchain-os-docs | — | kontinuierlich |

**Hinweis:** AD-026 ersetzt die AD-020-Empfehlung „Launch-Stack zuerst"
(Launch per AD-023 aufgehoben — keine Deadline mehr; Monorepo-Integration
erfolgt als LETZTER Schritt statt zuerst).

## Governance-Klassifizierung (AD-029, 07.09.2026)

Alle Repos sind nach ATC-STD-REPO-002 klassifiziert (Typ + Level R0-R4):
atc-shivacore CORE/OS R4 · atclang CORE R4 · a-townchain CORE R4 · Hub SPEC R3 ·
Monorepo INFRA R3 · globus-os OS R3 · aurora-ai AI R3 · atc-contracts SPEC/CORE R3 ·
Dienste R2 · Skelette R1. Volltabelle: docs/standards/ATC-STD-REPO-002.md.

## Tiefenanalyse (07.09.2026)

Vollstaendige Zuordnungs-Analyse (Inventar, Quer-Abhaengigkeiten, AD-012-Delta,
20 unzugeordnete Vault-Module mit Urteilen, Fundament-Check der 6 leeren
Service-Repos): [docs/audits/REPO_ZUORDNUNGS_ANALYSE.md](audits/REPO_ZUORDNUNGS_ANALYSE.md)
— Ergebnis: Zuordnung korrekt; 3 offene Owner-Entscheidungen (atc-security,
atc-monitoring, atc-standards-.atc).


---

## Technology Profiles (ATC-STD-100 §4, REQ-STD-104)

Soll-/Ist-Vergleich aller 25 Repos gegen die [Language Policy (ATC-STD-100)](https://github.com/A-TownChain-Okosystems/atc-standards/blob/main/standards/architecture/ATC-STD-100.md):
vollstaendiger Audit unter [audits/TECHNOLOGY_PROFILE_AUDIT_2026-09-07.md](audits/TECHNOLOGY_PROFILE_AUDIT_2026-09-07.md).

| Repo | Layer | Soll-Sprache | Ist (Code-LOC) | Status |
|---|---|---|---|---|
| a-townchain | L1 | Rust (Chain-Orchestrierung) | Rust 544 + Py 3,052 + TS 769 | ECHT |
| a-townchain-os | L4 | Rust (Integrations-Hub) | — (Skelett/Doku) | SKELETT |
| a-townchain-os-docs | L5 | Markdown | Py 43,259 + TS 652 | ECHT |
| atc-algorithm | L1 | Rust | — (Skelett/Doku) | SKELETT |
| atc-compute | L1 | Rust | — (Skelett/Doku) | SKELETT |
| atc-contracts | L1 | ATCLang/ABI: Rust | Py 3,300 + TS 548 | ECHT |
| atc-explorer | L2 | TypeScript | TS 248 | KLEIN |
| atc-indexer | L1 | Rust | — (Skelett/Doku) | KLEIN |
| atc-interop | L1 | Rust | Rust 173 | KLEIN |
| atc-launchpad | L2 | TypeScript | — (Skelett/Doku) | SKELETT |
| atc-marketplace | L2 | TypeScript | Rust 139 + TS 147 | KLEIN |
| atc-mining | L1 | Rust | — (Skelett/Doku) | SKELETT |
| atc-node | L1 | Rust | — (Skelett/Doku) | SKELETT |
| atc-oracle | L1 | Rust | — (Skelett/Doku) | SKELETT |
| atc-sdk | L1 | Rust SDK/CLI | Py 207 | KLEIN |
| atc-shivacore | L1 | Rust | Rust 49,481 | ECHT |
| atc-standards | L5 | Markdown/YAML | Py 808 | KLEIN |
| atc-storage | L1 | Rust | — (Skelett/Doku) | SKELETT |
| atc-vm | L1 | Rust | — (Skelett/Doku) | SKELETT |
| atc-wallet | L2/L1 | Rust + TS-UI | Py 446 | KLEIN |
| atclang | L1 | Rust (AD-021); Python=Referenz | Py 23,279 | ECHT |
| aurora-ai | L3 | Rust Core + Python AI | Rust 713 | ECHT |
| genesis-chronicles | L5 | Doku/Lore + ATCLang | Rust 132 + Py 1,668 | ECHT |
| genesis-engine | L4/L1 | Rust | Py 281 | KLEIN |
| globus-os | L4 | Rust | Rust 1,941 + TS 399 | ECHT |
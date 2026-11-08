# Tiefenanalyse: Repository-Zuordnung (07.09.2026)

**Frage:** Gehört jeder Inhalt in das richtige Repository — und ist die Zuordnung
korrekt? **Methode:** Voll-Inventar (Dateien/Zeilen je restauriertem Repo),
Import-Analyse aller restaurierten Module (Quer-Abhaengigkeiten), Kernel-Reinheits-
Check gegen AD-012, Restbestands-Analyse aller 20 unzugeordneten Vault-Module,
Fundament-Analyse fuer die 6 leeren Service-Repos.

## 1) Ergebnis: 13 Restaurierungen KORREKT

| Repo | Layer | Dateien | Zeilen | Module |
|---|---|---|---|---|
| atc-shivacore | L1 | 89 | 54.032 | Kernel-Rust-Crate K29 + Tools |
| aurora-ai | L2 | 377 | 80.170 | aurora-core/-agents/-memory/-runtime/-ai, aistudio |
| a-townchain | L3 | 205 | 13.923 | blockchain, atcnet, zkp, governance, dns, testnet |
| globus-os | L4 | 185 | 15.619 | globus-* (6), drivers, bootloader, editions (2) |
| 7x L5-Dienste | L5 | 242 | 11.003 | contracts, wallet, sdk(+cli+atcpkg), explorer, indexer, interop, marketplace |
| genesis-engine | L6 | 70 | 2.805 | engine, ecs, creatures, world |
| genesis-chronicles | L6 | 45 | 2.545 | Spiel + Framework (AD-025-korrigiert) |

**Import-Analyse:** 0 verkehrte Quer-Abhaengigkeiten (kein aufwaertiges Import
ueber Repo-/Layergrenzen). Module sind weiterhin Inseln (bestaetigt die
Verbund-Messung vom 03.09.: nur 7/132 Modul-Python-Dateien referenzieren andere
Module). EINZIGE externe Kante: atc-cli/repl.py -> atclang.vm.atcvm +
atclang.compiler.compiler — L5 nutzt L0 (Sprache aus SDK aufrufen): architektonisch
KORREKT und erwartet; die Pfade existieren im aufgebauten atclang-Repo.

## 2) Bekannte Abweichung: AD-012-Delta im Kernel-Crate

11 Service-Space-Dateien liegen NOCH im Rust-Kernel-Crate (blockchain.rs,
consensus.rs, genesis.rs, genesis_bridge.rs, gossip_bridge.rs, atcnet.rs, net.rs,
did.rs, remote_caps.rs, knowledge_graph.rs, security_audit.rs). Das war das
dokumentierte AD-012-Delta. **BEHOBEN am 07.09. (AD-028):** Migration in den
Crate service_space/ mit voller Test-Verifikation (Kernel 394/394 + Service
280/280 = 674/674, Boot L0-L10 gruen, Rust 1.98.1 stable).

## 3) Urteile fuer die 20 unzugeordneten Vault-Module

**NICHT restaurieren (Legacy, abgeloest):** atc-kernel (Python-Kernel, 85 Dateien),
atc-atclang, atclang, atc-vm, atc-stdlib (ATCLang-Legacy — in src/atclang
konsolidiert bzw. vom L0-Rebuild abgeloest).

**M8 Monorepo-Integration (Kernstack/Infra):** atc-backend, atc-frontend,
atc-gateway (API :4000), atc-ui (Neon-Dashboard), atc-mobile, atc-social,
atc-franchise, atc-ci, atc-deploy, atc-devtools, atc-ide.

**Hub-relevant (kanonisch im Hub vorhanden):** atc-whitepaper (Whitepaper-Kapitel
sind kanonisch in docs/whitepaper/; der TSX-Viewer ist M8-Frontend),
atc-standards (Standards sind kanonisch in docs/standards/; die .atc-Referenz-
implementierungen sind Kandidat fuer atc-contracts — Details unten).

**ENTSCHEIDUNGEN GETROFFEN (AD-028, 07.09. — delegiert):**
1. **atc-security** → atc-shivacore-Repo (Security-Tooling-Cluster, Commit bbbd9f1)
2. **atc-monitoring** → verbleibt im Vault bis M8
3. **atc-standards-.atc** → atc-contracts (modules/atc-standards-refs/, Commit dafd38d)

## 4) Die 6 leeren Service-Repos: Fundament vorhanden oder Neuland?

- **atc-storage:** Fundament im Kernel (ContentCap/ContentStore, Layer-5-Primitiven) —
  Service baut darauf auf. KEIN Vault-Code zu verschieben.
- **atc-compute:** Fundament im Kernel (DA-HEFT/TaskQueue/Tensor-Primitive) — dito.
- **atc-mining:** PoW-Logik in atc-blockchain vorhanden — Mining-Repo = Auslagerung
  bei M6-Entwicklung.
- **atc-oracle:** nur ein set_balance_oracle in governance_contract.py — echtes Neuland.
- **atc-node:** kein Fundament — aus atc-blockchain+atcnet+Kernel als Executable bauen.
- **atc-launchpad:** kein Fundand — echtes Neuland.
Urteil: Skelett-Zustand korrekt; KEIN Code falsch platziert.

## 5) Gesamt-Urteil

Die Zuordnung ist KORREKT: Layer-Reihenfolge, Modul-Verteilung und Repo-Grenzen
stimmen mit AD-012/013/026/027 ueberein. Zwei dokumentierte Abweichungen sind
bewusst und geplant (AD-012-Kernel-Delta, 3 offene Detail-Entscheidungen s.o.).
Keine fehlerhafte Platzierung gefunden — keine Nachverschiebung noetig.

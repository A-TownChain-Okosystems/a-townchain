# Lauffähigkeits-Roadmap M1-M8 (AD-027, 07.09.2026 — VERBINDLICH)

> **M1-Status (07.09.2026, verifiziert):** G1 ✅ Language Specification (specs/language/SPEC.md + registry.json, atclang abb3262) · G2 ✅ Semantics (src/atclang/semantics/ TypeChecker SEM-001…012, specs/semantics/SPEC.md + registry.json, semantisches Gate in compile_source, Parser-Fix parse_type — atclang e249a40, 20/20 Tests, Korpus CLEAN, 0 Regressionen). Nächstes Gate: **G3 (ATC-IR)**. 

**Ziel:** Das Ökosystem wird STÜCK FÜR STÜCK lauffähig. Nach jeder Meile läuft
ein echtes, verifizierbares Inkrement — kein Big-Bang (AD-023: Launch-Termin
offen). Jede Meile hat ein hartes Lauffähigkeits-Kriterium (Reality-Check:
Test-/Boot-/Run-Nachweis, nie nur Behauptung).

    M1 Sprache läuft        [L0 atclang]
    M2 Kernel läuft         [L1 atc-shivacore]        (baut auf M1)
    M3 KI läuft             [L2 aurora-ai]            (baut auf M2)
    M4 Blockchain läuft     [L3 a-townchain]         (baut auf M1+M2)
    M5 Betriebssystem läuft [L4 globus-os]            (baut auf M2)
    M6 Dienste laufen       [L5 13 Services]          (baut auf M4, teils M5)
    M7 Spiel läuft          [L6 genesis-*]           (baut auf M4+M5)
    M8 Ökosystem läuft      [L7 a-townchain-os]      (Integration ALLER)

## M1 — Sprache läuft (L0 atclang)

- **Status:** Python-Referenz läuft bereits (compile_source → CompiledModule,
  Pipeline grün). Kein Vault-Bedarf — das Repo steht als einziges aufgebaut.
- **Restschritte (Gates, AD-022):** ~~G1 Language Spec~~ (PASSED 07.09., SPEC.md + registry.json, Commit abb3262) → G2 AST/Semantics →
  G3 ATC-IR → G5 Bytecode Spec → G6 Independent Bytecode Verifier.
- **Kriterium:** Ein ATCLang-Programm wird zu einem ATCA-Artifact kompiliert,
  der Independent Verifier akzeptiert es, die ATVM führt es deterministisch
  aus (End-to-End-Demo hello.atc inkl. State-Transition).
- Rust Canonical Core folgt als Phase 2/3 (Differential Testing gegen Referenz).

## M2 — Kernel läuft (L1 atc-shivacore)

> **Update 04.10. (Freigabe-Zug 9):** SC-009 Kernel-Diagnostics & Event-Bridge v0.1.0-FROZEN (Zwei-Klassen-Verdrängung SECURITY/DIAGNOSTIC nachgezogen, Grant-Pflicht für SUBSCRIBE + automatische Katalog-Fortschreibung als nicht-bindende G7-relevante Notizen eingefroren). Kern-Spez-Stapel SC-001…SC-009 komplett FROZEN. Offen: SC-010…SC-013 (Themenfestlegung durch Owner), danach SC-ARCH-001…010-Freeze; G7-Audit-Trail-Merge durch Owner bei G7-Terminierung (Vorlage liegt vor).

> **Update 04.10. (Freigabe-Zug 8):** SC-008 Prozess- & Service-Erzeugung v0.1.0-FROZEN (nach Nachzug: REQ-SC008-07 Spawn als atomare OP-Sequenz mit Supervisor-Aufräum-Pflicht, REQ-SC008-10a empfängerunabhängiger Service-End, Loader-Frame-Grant-Wiring, Domain-Bindung=Erzeugungszeit). SC-009 Kernel-Diagnostics & Event-Bridge als DRAFT_REVIEW begonnen (Pflicht-Ereigniskatalog, IRQ-Deferral, Bridge im Service Space, Aurora NUR-LESEN). G7-Audit-Trail-VORLAGE committet (A: abgeleitete Aussagen / B: Nicht-Bindungs-Defaults / C: bereits wirksam) — Owner merged sie bei G7-Terminierung.

> **Update 04.10. (Freigabe-Zug 7):** SC-007 AddressSpace & Memory v0.1.0-FROZEN mit drei nachgezogenen Owner-Anmerkungen (REQ-SC007-13a Terminal-Suspend, W^X als Kernel-Politik-Default ohne Service-Space-Pfad, REQ-SC007-09a Rechte-Reduktion-Propagation via mint+revoke). SC-008 Prozess- & Service-Erzeugung als DRAFT_REVIEW begonnen: explizite Erzeugung ohne fork/exec, erzwungene Supervisor-Reserve-Thread-Cap (löst 13a strukturell), Exit-Notification ohne waitpid, vollständiger Untyped-Rückfluss. Nächste: SC-009.

> **Update 04.10. (Freigabe-Zug 6):** SC-006 Driver/Interrupt/Time v0.1.0-FROZEN (Owner-Überflug bestanden; Snapshot-Verdrahtung REQ-SC005-09a bestätigt; SC-005-Ableitung §3 ohne SC-DEC-O akzeptiert, Nachprotokollierung nur falls G7 separaten Audit-Trail erzwingt). SC-007 AddressSpace & Memory Objects als DRAFT_REVIEW begonnen (VA-Seite, W^X, Fault-Endpoint-Delivery, INV-09-Abgrenzung, HugePages-Pfad). Nächste: SC-008.

> **Update 04.10. (Freigabe-Zug 5):** SC-005 Capability-System v0.1.0-FROZEN nach 4-Punkte-Verifikation am Volltext (Badge-Immunität INV-05, Limit-Semantik REQ-SC005-07a/C-E03, Snapshot-Revocation REQ-SC005-09a/INV-09 aus SC-004 INV-06/07 abgeleitet, keine Initial-Allmacht REQ-SC005-11). SC-006 Driver/Interrupt/Time als DRAFT_REVIEW begonnen (IRQ/Timer/Device-Caps, Treiber im Service Space). Nächste: SC-007.

> **Update 04.10. (Freigabe-Zug 4):** SC-003 IPC & Endpoints + SC-004 Syscall-ABI gemeinsam v0.1.0-FROZEN (Default-only-Check SC-003 bestanden: keine versteckten ABI-Bindungen; SC-DEC-K/L/M freigegeben, N als Default ab SC-003 wirksam; K/L/M reversibel nur bis G7-Bindung, danach ABI-Freeze). SC-005 Capability-System-Vertiefung als DRAFT_REVIEW begonnen. Nächste: SC-006 Driver/Interrupt/Time.

> **Update 03.10. (Freigabe-Zug 3):** SC-003 IPC & Endpoints als DRAFT_REVIEW gestellt (nur Defaults, keine blockierenden SC-DEC). SC-004 Syscall-Interface/ABI v0.1.0-DRAFT_REVIEW committet — vier Owner-Kandidaten SC-DEC-K…N vorgelegt (ABI-Nummernraum, Register-Konvention, Cap-Uebergabe, Fehler-/Restart-Semantik). Reihenfolge danach: SC-005 Capability → SC-006 Driver/Interrupt/Time → … → SC-013 ATCLang-ABI-Bindung → SC-ARCH-001…010-Freeze.

> **Update 03.10. (Freigabe-Zug 2):** SC-DEC-A…J komplett freigegeben (Owner-Vorab-Regel; ab SC-003 keine blockierenden SC-DEC mehr, nur Defaults mit SC-ARCH-Review). SC-002 Scheduler & Scheduling-Domains FROZEN v0.1.0 (G=Bounded PI, H=tickless, I=250µs/1ms/10ms, J=HARDCUT für HARDRT). SC-003 IPC & Endpoints als DRAFT_REVIEW begonnen. Dependency-Review-Incident geheilt (PR #87/#88, Gate grün auf main 3faa92e).

> **Update 03.10. (AD-026-G1-Check):** G1 verifiziert PASS (atclang specs/VERSION.toml: G1-PASSED 2026-09-07, G2 accepted; ABI bleibt 0.1.0-DRAFT). SC-001 (Boot & Speicher) als v0.1.0 DRAFT_REVIEW im Kernel-Repo committet (docs/specs/SC-001-BOOT_MEMORY.md, 15e8d55): echte Bootchain B0-B5, Speichermodell, Initial-Caps, SC-DEC-A…F-Entscheidungen offen. L0–L10 bleibt In-Kernel-Testsequenz, kein echtes Booten.

- **Vault-Restauration:** docs/archive/monorepo-full/src/modules/atc-shivacore
  (60 .rs-Dateien, K29-Stand) → atc-shivacore-Repo. Byte-exakt im Vault
  gesichert (AD-020) — Restauration statt Neubau.
- **Kriterium:** 674/674 + Boot L0-L10. **STATUS 07.09. (AD-028):** Verifiziert —
  Kernel-Crate 394/394 + Service-Space-Crate 280/280 = 674/674, Boot gruen,
  Rust 1.98.1 stable. Kernel ist AD-012-konform (nur noch Primitive im Crate).
- Danach Ausbau per AD-013 (SC-001…SC-013: echte Microkernel-Objekte).

## M3 — KI läuft (L2 aurora-ai)

- **Vault-Restauration:** aurora-core/agents/memory/runtime/ai + aistudio
  (377 Dateien) → aurora-ai-Repo.
- **Kriterium:** Ein Agent-Task läuft Ende-zu-Ende durch die Kernel-Event-Bridge
  mit Capability-Token (AD-012/AD-021: Rust Core + Python AI-Layer).
- Ausbaustufe: DA-HEFT-Scheduler, Model Manager (AD-021-Phasenbild).

## M4 — Blockchain läuft (L3 a-townchain)

- **Vault-Restauration:** atc-blockchain/atcnet/wallet/contracts/bridge/
  dex/explorer/… (369 Dateien) → a-townchain-Repo.
- **Kriterium:** Zwei Nodes synchronisieren (Gossip), eine Transaktion wird
  validiert; Genesis Block mit Chain-ID 658467; ein ATCLang-Contract läuft
  als ATCA-Artifact auf der ATVM (nutzt M1). Kernel-Anbindung als System
  Service (AD-012).
- Vault-Wissen: K26-K29 (Genesis/Bridge/Gossip/Audit) sind bereits
  implementiert und getestet gewesen — schnell wieder lauffähig.

## M5 — Betriebssystem läuft (L4 globus-os)

- **Vault-Restauration:** globus-*/bootloader/drivers (12 OS-Module) →
  globus-os-Repo.
- **Kriterium:** globus-init bootet Userspace-Services auf ShivaCore
  (Bootchain AD-013: UEFI→Limine→ShivaCore→globus-init), Initial-Caps
  statt Unix-Root.

## M6 — Dienste laufen (L5, nach M4 parallelisierbar)

- **Reihenfolge:** atc-node zuerst (ATC-01 Core Node) → atc-wallet
  (ATC-Präfix, BIP44) → atc-sdk → atc-explorer → atc-indexer → atc-storage →
  atc-compute → atc-oracle → atc-interop → atc-mining → atc-marketplace →
  atc-launchpad.
- **Kriterium (Kern-Dreiklang):** Full Node verbindet sich zur Chain, Wallet
  erzeugt ATC-Adresse und transagiert, Explorer zeigt die Blöcke live.

## M7 — Spiel läuft (L6)

- **Reihenfolge:** genesis-engine zuerst (Engine-Loop, ECS) →
  genesis-chronicles (Spiel auf Engine).
- **Kriterium:** Spiel-Loop läuft; NFT-Mint (Genesis Chronicles) landet als
  Transaktion auf der Chain (nutzt M4-Contracts + ATC-9000).

## M8 — Ökosystem läuft (L7 a-townchain-os Monorepo)

- **LETZTER Schritt (AD-026):** Launch-Stack-Restauration aus dem Vault
  (docker-compose 10 Dienste, scripts/, config/, tests/) in den Monorepo und
  Verdrahtung aller Layer im Cargo-Workspace (AD-017).
- **Kriterium:** docker-compose up → Stack healthy, alle Dienste gegen die
  laufende Chain von M4; scripts/start.sh bootet das Gesamtsystem.

## Warum diese Reihenfolge logisch ist

1. Jede Stufe liefert ein lauffähiges Inkrement — nichts wartet auf
   "alles fertig".
2. Jede Stufe nutzt die vorherige: Verträge brauchen die Sprache (M1), die
   Chain braucht Verträge + Kernel (M1/M2/M4), Dienste brauchen die Chain
   (M4), das Spiel braucht Engine + Chain (M4/M7).
3. Der Vault (AD-020) macht M2/M4/M5/M8 zu Restaurationen mit Test-Nachweis
   statt teuren Neubauten — die ersten lauffähigen Stufen kommen schnell.
4. Konsistent mit AD-026 (Layer L0-L7) und AD-022 (ATCLang-Gates G0-G19).

**Status-Übersicht (07.09.2026):** G1 PASSED (M1) ·  M2-GATE VERIFIZIERT (394+280=674/674, AD-028) · M1 in Arbeit (G1 offen) · M2-M7: Basisstaende aus dem Vault restauriert (L1-L6, 13 Repos) — Gate-Verifikationen (674/674 Kernel-Tests, Boot, Node-Sync) im Rebuild-Lauf ausstehend · M8-Stack (Kernstack, docker, scripts, config, tests) verbleibt bis zur Integration im Vault.

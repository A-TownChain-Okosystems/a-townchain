# A-TownChain OS — Roadmap v2.0

> **Version:** 2.0 | **Stand:** 03.08.2026 | **Autor:** Aurora (MasterBrain · Base44)
> **Standards:** 99 ATC-Standards (ATC-01–99) — 94 spezifiziert mit Gas-Costs + Tests + Sprint
> **Wiki:** 69 Kapitel | **Audit-Score:** 91/100 | **Chain-ID:** 9000 (AD-004 offen) | **Non-EVM:** SHA-256

---

## Phasen-Übersicht

| Phase | Zeitraum | Status | Sprints |
|-------|----------|--------|---------|
| **Phase 1** — Foundation | Jan–Jun 2026 | ✅ DONE (100%) | 1.1–1.6 |
| **Phase 2** — Testnet + ATCLang | Jul–Dez 2026 | 🔵 AKTIV (65%) | 2.1–2.8 |
| **Phase 3** — Alpha Release | Jan–Apr 2027 | 🟡 PLANNED (20%) | 3.0–3.6 |
| **Phase 4** — Mainnet + Future | Jul–Dez 2027 | 🟡 PLANNED | 4.0–4.1, 4.2a–d |

---

## Sprint-Übersicht mit Standard-Zuweisung

### Phase 2 — Testnet + ATCLang (Sprints 2.1–2.8)

| Sprint | Titel | Status | % | Standards | Issues |
|--------|-------|--------|---|-----------|--------|
| | **2.1** | ATCLang Node Bootstrap | 🔵 AKTIV | 93% | | ATC-81–86, 92–94 | #72, #73, #74, #81 |
| **2.2** | P2P + Testnet | ✅ FERTIG | 100% | ATC-01, 06, 07 | #75, #82, #83, #84 |
| **2.3** | Consensus + Gas | 🔵 AKTIV | 95% | ATC-11, 13, 14, 19, 20, 23, 87–89 | #76 |
| **2.4** | Kernel + Syscalls | 🔵 AKTIV | 85% | ATC-08, 09, 10, 21, 22, 96 | #77 |
| **2.5** | NFT + Marketplace | ✅ FERTIG | 100% | ATC-12, 15, 16, 90, 95 | — |
| **2.6** | Governance + Security | 🔵 AKTIV | 85% | ATC-02–05, 17, 18, 91 | #78 |
| **2.7** | Testing + CI/CD | 🟡 PLANNED | 10% | ATC-98 | #79 |
| **2.8** | Multi-Node Testnet Live | 🟡 PLANNED | 15% | — (Infra) | — |

### Phase 3 — Alpha Release (Sprints 3.0–3.6)

| Sprint | Titel | Status | % | Standards | Issues |
|--------|-------|--------|---|-----------|--------|
| **3.0** | AI Orchestration | 🟡 PLANNED | 20% | ATC-24–31, 97 | #80 |
| **3.1** | UX + Apps + Privacy | 🟡 PLANNED | 0% | ATC-32–43 | — |
| **3.2** | Distributed Intelligence | 🟡 PLANNED | 0% | ATC-44–50 | — |
| **3.3** | Security Audit | 🟡 PLANNED | 0% | ATC-05, 46, 86 | #69 |
| **3.4–3.6** | Alpha Release | 🟡 PLANNED | 0% | — (Release) | — |

### Phase 4 — Mainnet + Future (Sprints 4.0–4.2)

| Sprint | Titel | Status | Standards | Issues |
|--------|-------|--------|-----------|--------|
| **4.0** | Mainnet Prep | 🟡 PLANNED | ATC-01, 81, 83–86 | #70, #71 |
| **4.1** | Mainnet Launch | 🟡 PLANNED | — (Launch) | — |
| **4.2** | Future Tiers | 🟡 PLANNED | ATC-51–80 | — |

---

## Kritische Pfade

### Sprint 2.1 (Fast fertig — 19 Parser-Fixes verbleiben)
1. ✅ #72 ATCLang Compiler (ATC-92) — Lexer + Parser + AST
2. ✅ #73 ATCLang VM (ATC-93) — 105 Op-Codes + Stack-VM
3. ✅ #81 ATCLang Stdlib (ATC-94) — 14 Module
4. ✅ #74 Konsens-Migration (ATC-81–86) — alle 6 Module als .atc
5. 🔴 Parser-Coverage 186/198 — 5 Fix-Kategorien verbleibend identifiziert

### Blocker (→ Michael)
- **AD-004** Chain-ID 9000 — REOPENED, Entscheidung nötig
- **AD-005** ATC-97 Protocol — Spezifikation finalisieren
- **#79** CI/CD Pipeline Fix — Branch-Protection blockiert API-Push

### Parser-Fix-Prioritäten (19 Fehler → 100% Coverage)
1. `::` Path-Operator (Type::method, Enum::Variant) — 7 Dateien
2. `if let Some(x) = expr` Pattern-Matching — 4 Dateien
3. `Ok(())` / Unit-Typ `()` als Expression — 3 Dateien
4. `map { k => v }` Map-Literal + `as` Cast — 2 Dateien
5. `&mut` Referenz-Operator — 1 Datei
6. Tuple-Typ in Generics `Option<(A, B)>` — 1 Datei
7. `return;` in Inline-Blocks — 1 Datei

---
*Aurora · 03.08.2026 15:30 (Europe/Berlin) · Commit de175b0*

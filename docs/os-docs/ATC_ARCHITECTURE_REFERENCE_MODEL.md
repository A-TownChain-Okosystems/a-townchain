# ATC_ARCHITECTURE_REFERENCE_MODEL — Referenzmodell des ATC-Stacks (v0.2.0, Proposed)

<!-- document_id: ATC-DOC-ARCHREF-001 -->
<!-- status: proposed -->
<!-- standard: ATC-STD-MD-001 -->
<!-- date: 2026-10-06 -->

> **Zweck:** Einheitliches Architektur-Referenzmodell über den gesamten ATC-Stack
> (33 Repositories — verifizierter Flottenstand via GitHub-API, Stand 2026-10-06). Referenz für Layer-Zuordnung, Roadmap-Debatte und
> Doku-Abgleich. `a-townchain-os` ist darin **L7 Integration/Orchestrierung**
> (keine Core-Implementierung) — Layer-Aussagen zielen immer auf den Gesamt-Stack,
> nie auf ein einzelnes Repo.
>
> **Quellen:** a-townchain/ARCHITECTURE.md (L3, Hybrid-Konsens, kanonisch in
> atc-algorithm), components/algorithm/README.md, a-townchain-os/ARCHITECTURE.md (L7),
> GitHub-Issue-API (verifiziert 06.10.2026, 20:3x UTC+2).

## 1. Klassifikationsmodell (verbindlich für alle Status-Aussagen)

| Klasse | Bedeutung |
|---|---|
| **NORMATIVE** | Standard/Spec ist freigegeben und in Kraft (atc-standards-Registry, §9) |
| **IMPLEMENTED** | Funktioniert nachweislich (Tests grün, Evidence/Run vorhanden) |
| **RESEARCH** | Forschungs-/Spekulationskandidat, keine Roadmap-Tatsache |
| **LEGACY/UNVERIFIED** | Altdoku-Claim ohne tragfähige Evidence — muss vor Verwendung re-verifiziert werden |

Grundregel: Existenz einer Spezifikation oder alter grüner Doku-Status ist nicht
gleich VERIFIED.

## 2. Schichtenmodell mit ATC-Zuordnung und Evidence-Status

| Layer | ATC-Zuordnung (Stack gesamt) | Klassifikation | Anmerkung |
|---|---|---|---|
| **Data Layer** | `a-townchain` (L3): Blocks, Transactions, State, Merkle/Storage | **UNVERIFIED** | Behauptet: SQLite-Persistenz seit v2.1.0 — Exact-SHA-Evidence fehlt |
| **Network Layer** | `a-townchain` / zuständiger Network-Stack | **UNVERIFIED** | Behauptet: P2P, ECDSA secp256k1/RFC 6979 — Exact-SHA-Evidence fehlt |
| **Consensus Layer** | `atc-algorithm` (kanonische Implementierung, vgl. a-townchain/components/algorithm) | ⚠️ **RESEARCH** (maturity: DEVELOPMENT — NICHT FINAL) | Hybrid PoH+PoS+PoW ist **Entwicklungs-/Spezifikationsziel**, kein eingefrorener Konsens. „Freeze normative consensus specification" steht ausdrücklich als nächster Schritt aus (components/algorithm/README.md:137). KEINE Formulierung „finaler Hybrid-Konsens definiert/frozen". |
| **Contract Layer** | ATC-001/ATC-8300/ATC-9900, SC-Gates SC-G0..G13 | IMPLEMENTED (Prototypen) + NORMATIVE (Gates seit 07.09.2026) | Registry: ATC-SC-TOKEN-001..003, SC-G0 offen |
| **Application Layer** | `atc-explorer`, `atc-wallet`, `atc-sdk` | **UNVERIFIED** | Behauptet: Explorer-API seit v2.1.0 — Exact-SHA-Evidence fehlt |
| **Security Layer** | `atc-standards` + ZKP-/Security-Familien (ATC-STD-100/300, ZKP-001..010) | NORMATIVE | Querschneidend über alle Layer |


### 2.1 Statusblock (Review-Fassung v0.2.0)

| Bereich | Status | Evidence / Bemerkung |
|---|---|---|
| Data | UNVERIFIED | Exact-SHA-Evidence fehlt |
| Network | UNVERIFIED | Exact-SHA-Evidence fehlt |
| Application | UNVERIFIED | Exact-SHA-Evidence fehlt |
| Consensus | RESEARCH | maturity: DEVELOPMENT |
| ATC-07 | LEGACY/UNVERIFIED | CLOSED ≠ IMPLEMENTED; Re-Verifikation offen |

Hebung auf IMPLEMENTED erst nach Exact-SHA-Evidence (Tests + Commit-SHA je Layer).

## 3. Konsens — Präzisierung

Der proprietäre Hybrid-Konsens (PoH+PoS+PoW, Chain-ID 658467) wird als
**Spezifikations- und Entwicklungspfad** geführt. Der normative Freeze steht aus
(components/algorithm/README.md, Schritt 1). Solange gilt:

- Architektur-Texte formulieren Zielzustand, nicht Ist-Zustand.
- Erst nach Freeze: Klassifikation NORMATIVE + IMPLEMENTED (mit Test-Evidence).

## 4. Sharding (ATC-07) — Statusdrift-Dokumentation

- **GitHub-Issue #84** („Network-Level Sharding & State Partitioning → ATCLang (ATC-07)"):
  **CLOSED seit 2026-07-05T12:45:37Z** (verifiziert via GitHub-API, 06.10.2026).
- **Konflikt:** Alte Roadmap-/Sprint-Doku behauptet teils ✅ implementiert; ein
  Datenbestand führte #84 als OPEN. Beides ist mit dem GitHub-Status unvereinbar.
- **Klassifikation: LEGACY/UNVERIFIED.**
- **Klarstellung: ATC-07 ist CLOSED, aber nicht IMPLEMENTED.**
  Der alte ✅-Claim bleibt bis zur Re-Verifikation LEGACY/UNVERIFIED.
  Aussagen wie „Sharding ist implementiert" sind bis zur Re-Verifikation gegen
  atc-algorithm/atc-node/ATCLang-Interfaces nicht zulässig.
  Offener Punkt → Task-DB (Statusdrift bereinigen).

## 5. Stateless Validation — Forschungskandidat (v3.x)

Terminologie-Regel: **Stateless Blockchain/Stateless Validation ≠ stateless dApp.**

Gemeint ist: Witnesses/State Proofs und Reduktion des lokal erforderlichen
vollständigen Zustands auf Validator-/Node-Seite. Relevanter für Data-/Storage-Layer
als die bisherige Doku-Kategorie „stateless dApps".

**Führung als:** RESEARCH — Stateless Validation / Stateless Blockchain /
Verifiable State Access. Spätere Prüfung gegen: Sharding (ATC-07), State Storage,
Merkle-/Witness-Format, Node-Synchronisation. Keine Roadmap-Tatsache.

## 6. Offene Punkte

| Punkt | Zuständigkeit | Status |
|---|---|---|
| Freeze normative consensus specification (components/algorithm/README.md:137) | atc-algorithm | OFFEN — Todo Task-DB |
| ATC-07 Statusdrift: Re-Verifikation + Altdoku-Bereinigung | a-townchain / Doku | OFFEN — Todo Task-DB |
| Research-Registratur v3.x-Kandidaten (Stateless Validation) | Doku | IN DIESER DATEI geführt |
| Exact-SHA-Evidence für Data/Network/Application sammeln (Hebung auf IMPLEMENTED) | Agents | OFFEN — Todo Task-DB |

## 7. Querverweise

- `a-townchain/ARCHITECTURE.md` (L3-Definition, Hybrid-Konsens-Bindung)
- `a-townchain/components/algorithm/README.md` (Freeze-Schritt, kanonische Implementierung)
- `a-townchain-os/ARCHITECTURE.md` (L7 Integration/Orchestrierung, keine Core-Implementierung)
- `atc-standards/standards/sc/` (SC-Gates SC-G0..G13, Contract Registry)
- Roadmap: ATC-07 (Sharding → Task-DB, LEGACY/UNVERIFIED), v3.x Research (Abschnitt 5)
- `ATC_OS_ARCHITECTURE_REFERENCE_MODEL.md` (OS-Stack-Referenzmodell, gleiche Klassifikation; Kernel-SSOT: globus-os)

## 8. Versionierung

| Version | Datum | Änderung |
|---|---|---|
| v0.1.0 | 2026-10-06 | Initiale Fassung (Draft): Schichtenmodell, Klassifikationsmodell, ATC-07-Statusdrift dokumentiert, Stateless Validation als RESEARCH |
| v0.2.0 | 2026-10-06 | Review-Korrekturen (CONDITIONAL PASS): Repo-Zahl 26→33 (GitHub-API-verifiziert), Data/Network/Application auf UNVERIFIED zurückgestuft (Exact-SHA-Evidence fehlt), Consensus vierte Klasse entfernt (RESEARCH, maturity: DEVELOPMENT), ATC-07-Klarstellung (CLOSED ≠ IMPLEMENTED); Status draft→proposed |

Status-Änderungen erfolgen per Owner-Review gemäß ATC-STD-MD-001. Aktueller Status: proposed (Review-Korrekturen v0.2.0 eingearbeitet).

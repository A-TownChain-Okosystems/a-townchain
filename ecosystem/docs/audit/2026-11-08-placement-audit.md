# Ablage-Audit 2026-11-08 — liegt jeder Code am richtigen Platz?

**Auftrag:** Prüfung, ob alle Dateien im Software-Stack korrekt abgelegt sind.
**Methode:** Dateilisten der aktiven Repos gegen den Fixstand vom 08.11.2026 verglichen (GitHub API, exakte SHA-Stände).

## Ergebnis in einem Satz

Die Fixes liegen im Monorepo `a-townchain/components/*` **korrekt** und vollständig, aber der aktive Standalone-Repo `atc-node` ist um vier Module und die gesamte P0-Härtung zurück — dieselbe Logik gilt für Wallet und Oracle, deren Standalone-Repos archiviert sind.

## Befunde

### P-01 (hoch) — Code-Drift: `atc-node` kennt die P0-Fixes nicht

| Prüfpunkt | `a-townchain/components/node` | `atc-node` (aktiv) |
|---|---|---|
| src-Dateien | 13 | 9 |
| `mempool.rs`, `did.rs`, `validator.rs`, `discovery.rs` | vorhanden | **fehlen** |
| `chain.rs` mit 256-Bit-Hash + `produce_tx_block` | ja | **nein** (208 Zeilen, alte Fassung) |
| `tests/` (8 Tx-Block-, 5 Hash-, 4 DID-Tests) | vorhanden | **fehlen** |
| letzter Push | 08.11.2026 | 02.11.2026 |

Damit ist SEC-P0-001/003/004/007 **nur im Monorepo** umgesetzt. Wer `atc-node` baut, erhält den ungehärteten Stand.

### P-02 (hoch) — Wallet/Oracle: SSOT-Repo archiviert, Code lebt doppelt

`atc-wallet` und `atc-oracle` sind archiviert (read-only). Der aktive Code liegt in `a-townchain/components/{wallet,oracle}` — mit `verification.rs` (SEC-P0-002), das im archivierten `atc-oracle` nie existierte. Die Archivierung ist korrekt, aber die kanonische Ablage ist nirgends verbindlich dokumentiert.

### P-03 (mittel) — ZKP doppelt im selben Repo

`components/zkp` und `modules/atc-zkp` enthalten **beide** ein `Cargo.toml`: zwei Crates gleicher Funktion im selben Repo. Zusätzlich existiert `atc-zkp` als eigenes aktives Repo. Das ist eine dreifache Ablage mit Drift-Risiko.

### P-04 (mittel) — ARCHITECTURE.md beschreibt den alten Ort

`ARCHITECTURE.md` nennt als Chain Protocol Core `modules/` und bezeichnet `components/*` als „importierte Subtree-Stände". Tatsächlich liegen die kanonischen, gehärteten Crates in `components/`. Doku und Realität widersprechen sich.

### P-05 (niedrig) — FILE_REGISTER veraltet

`a-townchain/FILE_REGISTER.md` (Stand 2026-09-17, 992 Dateien) und `a-townchain-os/FILE_REGISTER.md` führen keine der am 08.11. angelegten Dateien.

### P-06 (niedrig) — Ecosystem spiegelt Komponenten

`a-townchain-ecosystem/components/` enthält Mirror von `a-townchain`, `a-townchain-os`, `atc-algorithm` u. a. Solange die Mirror-Policy offen ist, wächst die Zahl möglicher Ablagen weiter.

## Korrekt abgelegt (gegengeprüft)

- `components/node/src/*.rs` — jede Datei ist in `lib.rs` als `pub mod` deklariert, keine Waisen.
- `components/node/tests/*.rs` — Integrations-Tests liegen korrekt unter `tests/`.
- `components/oracle/{src,tests}` — Verifier und Tests am richtigen Ort; Abhängigkeiten in `Cargo.toml` deklariert.
- `kai-os/integration/src/handoff.rs` — als `pub mod handoff` verdrahtet, `secp256k1`/`sha2` in `Cargo.toml`.
- Audit-Artefakte in `a-townchain-ecosystem/docs/audit/security/` — repo-übergreifender Befund gehört ins Meta-Repo; Register, Bericht und Repros liegen zusammen.
- Keine `target/`-Artefakte, keine Patch-Skripte, keine ungetrackten Dateien in den drei geprüften Repos.

## Empfehlung

1. **Entscheidung nötig:** Ist `a-townchain` das Monorepo-SSOT (dann `atc-node` als Mirror kennzeichnen oder archivieren) oder bleibt `atc-node` SSOT (dann müssen `mempool.rs`, `did.rs`, `validator.rs`, `discovery.rs`, die Härtung in `chain.rs`/`bootstrap.rs`/`gossip.rs` sowie die Tests dorthin portiert werden)?
2. `modules/atc-zkp` stilllegen oder `components/zkp` als kanonisch markieren.
3. Mirror-Policy für `ecosystem/components/` festlegen.
4. ARCHITECTURE.md auf den Ist-Zustand ziehen (in diesem Commit aktualisiert).

## Maßnahmen (08.11.2026, umgesetzt)

| Befund | Maßnahme | Beleg |
|---|---|---|
| P-01 Node-Drift | `atc-node` als **Mirror ohne kanonischen Code** gekennzeichnet: `SYNC-STATUS.md` mit Drift-Tabelle, Reaktivierungsplan und Begründung; README-Ablagehinweis | Branch `docs/placement-status` (Commits 4d6e2f00, 5f12108) |
| P-04 Doku-Drift | `ARCHITECTURE.md` in a-townchain: `components/*` als kanonisch, `modules/*` als Legacy | a-townchain `8a98876` |
| P-05 Register | Nachträge 2026-11-08 in beiden Datei-Registern (14 + 3 Dateien) | a-townchain `e1c6b40`, a-townchain-os `64d8d5d` |

**Offen (blockiert):** `atc-node/main` ist branch-protected (PR-Pflicht, Checks `audit`, `cargo-audit`, `codeql`).
Der Merge des Mirror-Hinweises braucht einen PR; PR-Erstellung ist durch die unverifizierte
E-Mail des Accounts gesperrt (HTTP 403 auf `POST /pulls`). Owner-Klick:
`https://github.com/A-TownChain-Okosystems/atc-node/compare/main...docs/placement-status?expand=1`

**Offen (Entscheidung):** Monorepo-SSOT endgültig festschreiben und `atc-node` archivieren
(7 von 8 Standalone-Komponenten-Repos sind bereits archiviert), oder `a-townchain` um ein
Root-Workspace-Manifest erweitern, damit `atc-node` das Wallet-Crate per Git-Pin beziehen kann.

## ENTSCHEIDUNG 08.11.2026 (Owner)

**Das Monorepo `a-townchain` ist die Source of Truth.** Neue Entwicklung, Tests und Releases
laufen ausschliesslich dort. `atc-node` bleibt Mirror ohne Release-Rolle (README-Hinweis und
`SYNC-STATUS.md` auf Branch `docs/placement-status`); Standalone-Repos erhalten keinen neuen Code.

Folgearbeit im Monorepo (laufend): DID-Registry persistierbar und ueber den Node-RPC abfragbar,
danach Stilllegung von `modules/atc-zkp` zugunsten von `components/zkp`.

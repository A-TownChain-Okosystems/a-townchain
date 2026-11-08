# A-TownChain Ecosystem — Systematischer Repository-Audit

- Audit-Datum: 2026-09-24
- Branch/Commit: `main @ 7431e656e46a787e83bfb27482885cc87faf4c6a`
- Scope: Repository-Struktur, Cargo/Crates, Code, Tests, Integration, Architektur, Security, CI/CD, Dependencies, Dokumentation
- Status: Befundbericht vor den Reparaturen

## 1. Struktur

Repository tree: 14,075 Einträge / 11,703 Dateien. Enthalten sind 27 Kern-Komponenten unter `components/`, 148 nicht archivierte Cargo-Manifeste, 13 GitHub-Workflows und umfangreiche historische/archivierte Dokumentation.

Root-`Cargo.toml` ist bewusst nur ein Integrations-Workspace mit 10 Mitgliedern. Die vollständige Rust-Abdeckung erfolgt zusätzlich über `ecosystem-release-readiness.yml`, das die eigenständigen Workspaces/Packages unter `components/` einzeln ermittelt.

## 2. Aktueller CI-Befund

Für Commit `7431e656`:
- Repository Governance: PASS
- Knowledge Indexer: PASS
- CodeQL: PASS
- Determinism Gate: PASS
- ATC Test Suite: PASS
- Ecosystem Integration: FAIL
- L1 Network E2E: FAIL
- Blockchain Core Integration: FAIL
- Ecosystem Release Readiness: FAIL

Die Fehler sind reale Build-/Test-/Integrationsfehler; Gates wurden nicht abgeschwächt.

## 3. Priorisierte Fehler

### E-001 — BLOCKER — atc-node RPC kompiliert nicht
- Datei: `components/a-townchain/components/node/src/rpc.rs`
- Zeilen: 53–54
- Ursache: `TcpStream::write_all` wird verwendet, aber `std::io::Write` nicht importiert.
- Wirkung: `cargo check`, Test und Clippy des `atc-node`-Packages schlagen fehl.
- Reproduktion: `cargo check --manifest-path components/a-townchain/components/node/Cargo.toml --all-targets`
- Fix: `Write` importieren; danach fmt/check/test/clippy.
- Risiko: gering.

### E-002 — BLOCKER — Wallet-Implementierung referenziert nicht vorhandenen Typ
- Datei: `components/a-townchain/components/wallet/src/tx.rs`
- Zeile: 7
- Ursache: `crate::keys::WalletKey` wird importiert, während `src/keys.rs` nur ein geplanter Stub ohne `WalletKey` ist.
- Wirkung: Wallet-Crate kompiliert nicht.
- Integrationsbefund: Das Repository enthält gleichzeitig eine funktionierende `components/atc-wallet/src/keys.rs`-Implementierung und einen zweiten, inkompletten Wallet-Pfad.
- Fix: Source-of-Truth/Integrationspfad eindeutig festlegen und die vorhandene funktionierende Key-Implementierung ohne kryptographische Abweichung anbinden; Legacy-Pfad erst nach Referenzprüfung entfernen.
- Risiko: mittel, da Wallet-Signing betroffen ist.

### E-003 — BLOCKER — ZKP-Prover inkompatibel mit arkworks 0.6 API-Nutzung
- Datei: `components/atc-zkp/crates/zkp-prover/src/lib.rs`
- Zeilen: 42, 55, Test um 115/119
- Ursache: `Groth16::circuit_specific_setup` und `Groth16::prove` sind Trait-Methoden; `SNARK` ist nicht importiert. Zusätzlich liefert `ark_std::test_rng()` hier nicht die von der eigenen Signatur geforderte `CryptoRng)-Garantie.
- Wirkung: ZKP-Prover kompiliert nicht.
- Reproduktion: `cargo test --workspace --all-targets` im ZKP-Workspace bzw. die von Release Readiness ausgeführte Workspace-Prüfung.
- Fix: `SNARK` in Scope bringen und für den Test einen expliziten CSPRNG-Typ wie `StdRng` mit `CryptoRng` verwenden; danach Prover/Verifier-Roundtrip testen.
- Risiko: mittel bis hoch, weil kryptographische Primitive betroffen sind.

### E-004 — BLOCKER/INTEGRATION — Multi-Process-Finality-Test hat keinen Validator-B-Vote für Block 2
- Datei: `components/a-townchain/modules/atc-blockchain/kernel/tests/multi_process_network_e2e.rs`
- Zeile des Fehlschlags: 129
- Ursache: Node B stimmt explizit über Block 1 ab, beendet seinen Initialprozess nach Empfang von Block 2 aber ohne den eigenen Validator-B-Vote für Block 2 zu senden. Node A wartet korrekt auf 2/3-gewichtete Finalität.
- Tatsächliches Verhalten: Block 2 wird importiert, erreicht aber die im Test geforderte Finalität nicht.
- Fix: Im Testablauf muss Node B Block 2 ebenfalls explizit abstimmen und broadcasten, bevor der Prozess endet. Das ist keine Gate-Abschwächung, sondern Vervollständigung des getesteten Consensus-Ablaufs.
- Risiko: gering bis mittel.

### E-005 — MEDIUM — Genesis-Physics-Test vergleicht f32 exakt
- Datei: `components/genesis-engine/modules/atc-genesis-physics/src/collision.rs`
- Test: `point_resolution_uses_nearest_face`
- Ursache: tatsächliches Ergebnis `0.100000024` wird per `assert_eq!` gegen `0.1` geprüft.
- Wirkung: Test schlägt wegen legitimer f32-Rundung fehl; die Richtung ist korrekt.
- Fix: numerische Toleranz/ULP-basierte Prüfung statt bit-exakter Dezimalgleichheit.
- Risiko: gering.

### E-006 — LOW/MEDIUM — Genesis-Assets Clippy-Gates schlagen wegen veralteter Idiome fehl
- Datei: `components/genesis-engine/modules/atc-genesis-assets/src/lib.rs`
- Zeilen: 187, 246
- Befunde: `manual_is_multiple_of`, manuelles OR-Pattern statt Range.
- Fix: Clippy-empfohlene semantisch identische Schreibweise verwenden.
- Risiko: gering.

### E-007 — LOW/MEDIUM — Genesis-ECS Clippy-Gates schlagen fehl
- Datei: `components/genesis-engine/modules/atc-genesis-ecs/src/lib.rs`
- Zeilen: 453, 886
- Befunde: öffentliches `len()` ohne `is_empty()`; manuelle `Default`-Implementierung trotz ableitbarem Default.
- Fix: `is_empty()` ergänzen und `Default` ableiten.
- Risiko: gering.

### E-008 — MEDIUM — Konsensustest enthält unbenutzte Variable
- Datei: `components/a-townchain/modules/atc-blockchain/kernel/src/consensus.rs`
- Test `weighted_finality_requires_two_thirds_stake`
- Ursache: Validator-Key `c` wird erzeugt/registriert, aber nicht verwendet; Release-Readiness erzwingt `-D warnings).
- Wirkung: Clippy/Test-Build des Blockchain-Workspace schlägt fehl.
- Fix: Variable entfernen, wenn sie für den Test nicht benötigt wird; Validator `c` bleibt als Stake-Szenario registriert.
- Risiko: gering.

### E-009 — HIGH — Netzwerk-Deserialisierung erlaubt potenziell übergroße Vec-Allokation
- Datei: `components/a-townchain/modules/atc-blockchain/kernel/src/network.rs`
- Bereich: `block_decode`
- Ursache: Der Netzwerkframe ist auf 8 MiB begrenzt, aber die dekodierte Transaktionsanzahl wird als u32 gelesen und direkt an `Vec::with_capacity(n)` übergeben, bevor geprüft wird, ob die Nutzlast diese Anzahl überhaupt enthalten kann.
- Sicherheitswirkung: Ein kleiner, authentifizierungsfreier/malformierter Blockframe kann eine sehr große Speicherallokation anfordern.
- Erwartetes Verhalten: Decoder muss vor der Allokation eine harte Transaktionszahl-/Payload-Grenze prüfen.
- Fix: Protokollgrenze (z.B. MAX_TX_PER_BLOCK bzw. aus Payload ableitbare Obergrenze) vor `with_capacity` erzwingen; negative Tests für absurd große Counts.
- Risiko des Fixes: gering; Protokollgrenze muss mit der kanonischen Blockgröße abgestimmt werden.

### E-010 — HIGH — Zwei ATC-VM-Implementierungen mit divergierendem Code
- Pfade:
  - `components/atc-vm`
  - `components/a-townchain/components/vm`
- Befund: `assembler.rs` ist aktuell identisch, `vm.rs` und `context.rs` sind nicht identisch. Der Root-Workspace und der produktive Blockchain-Adapter verwenden `components/atc-vm`; die zweite Implementierung ist damit ein divergierender Legacy-/Source-Pfad.
- Zusätzlich enthält die nested VM-Version weiterhin `TX_DOMAIN = "ATC-TX-DOMAIN"`, obwohl ATC-STD-600 den Legacy-Domain-String als retired definiert.
- Risiko: zukünftige Änderungen können zwei unterschiedliche VM-/Identity-Semantiken erzeugen.
- Fix: Referenzen vollständig erfassen, kanonischen VM-Pfad bestimmen, danach Legacy-Code entfernen oder eindeutig als nicht-produktiven historischen Import markieren.
- Risiko: mittel bis hoch, weil VM/Domain-Semantik betroffen ist.

### E-011 — HIGH — ATC-VM verwendet Wrapping-Arithmetik ohne expliziten Overflow-Gate
- Dateien: `components/atc-vm/src/vm.rs` und die nested VM-Kopie.
- Befund: Add/Sub/Mul verwenden `wrapping_add/sub/mul`.
- Testabdeckung: kein expliziter Overflow/Underflow-Test im VM-Testblock.
- Bewertung: Das ist als Sicherheits-/Semantikrisiko zu prüfen; aus dem aktuell sichtbaren Standardtext lässt sich nicht beweisen, dass Wrapping die normative gewünschte Semantik ist.
- Fix vor Änderung: Semantik in ATC-STD-/VM-Spezifikation klären und anschließend gezielte Overflow/Underflow-Tests hinzufügen. Nicht blind auf checked arithmetic umstellen.

### E-012 — HIGH — Wallet-Dokumentation widerspricht dem implementierten Signaturverfahren
- Datei: `components/a-townchain/components/wallet/README.md`
- Befund: Dokumentation beschreibt ECDSA/secp256k1, während der aktuelle Rust-L1-Pfad Ed25519 und `ATC-TX-DOMAIN-V2` verwendet.
- Zusätzlich existiert `components/atc-wallet` als funktionierender Rust-Wallet-Core mit Ed25519.
- Wirkung: falsche Security-/API-Dokumentation und unklare Source of Truth.
- Fix: Dokumentation auf den kanonischen Ed25519/V2-Pfad korrigieren oder den Legacy-Wallet-Pfad aus dem produktiven Integrationspfad entfernen, nachdem Referenzen geprüft wurden.

### E-013 — MEDIUM — CodeQL analysiert nur Python
- Datei: `.github/workflows/codeql.yml`
- Befund: `languages: python`; Repository enthält umfangreichen Rust- und TypeScript-Code.
- Wirkung: Der zentrale CodeQL-Gate deckt Rust/TypeScript nicht ab.
- Fix: CodeQL-Abdeckung an den tatsächlich produktiven Sprachen ausrichten, ohne andere Gates zu entfernen.
- Risiko: mittel.

### E-014 — MEDIUM — PostgreSQL-Client-Konfiguration deaktiviert TLS
- Dateien:
  - `components/aurora-ai/modules/atc-aistudio/src/db/drizzle.config.ts`
  - `components/a-townchain-os-docs/aistudio/src/db/drizzle.config.ts`
- Befund: `ssl: false`.
- Bewertung: Für lokale Entwicklungsumgebungen kann das beabsichtigt sein; als allgemeine Konfiguration ist es kein sicherer Default für produktive Datenbankverbindungen.
- Zusätzlich sind die beiden Konfigurationen doppelte Implementierungen.
- Fix: Umgebung explizit unterscheiden und Produktion mit TLS erzwingen; Doppelung über Source-of-Truth klären.

### E-015 — MEDIUM — Breite Rust-Testlücke
- Befund aus Repository-Strukturanalyse: 148 nicht archivierte Cargo-Manifeste; 108 Rust-Pakete haben keine externen `tests/`- oder `benches/`-Dateien. Das schließt Unit-Tests im `src/` ausdrücklich nicht aus.
- Besonders auffällig: mehrere ZKP-, Wallet-, Governance-, Bridge-, Aurora-, Genesis- und GlobusOS-Pakete.
- Fix: pro sicherheits-/zustandskritischer Komponente mindestens negative, Integrations- oder Restart/Recovery-Tests ergänzen; nicht pauschal jeden kleinen Utility-Crate mit künstlichen Tests aufblasen.

## 4. Root-Cause-Gruppierung

### Root Cause A — divergierende/teilweise synchronisierte Subtree-Komponenten
Betroffen: E-002, E-010, E-012 und Teile der Domain-Identity-Struktur.

### Root Cause B — vollständige Release-Readiness prüft mehr Code als der Root-Workspace
Betroffen: E-001, E-003, E-006, E-007, E-008.
Diese Fehler sind nicht dadurch verursacht, dass das Gate zu streng ist; sie werden durch die breitere tatsächliche Repository-Prüfung sichtbar.

### Root Cause C — Netzwerk-E2E-Testablauf
Betroffen: E-004.

### Root Cause D — fehlende Parser-Ressourcenbegrenzung
Betroffen: E-009.

### Root Cause E — Spezifikations-/Dokumentationsdrift
Betroffen: E-011, E-012, E-013, E-014.

## 5. Bereits vor diesem Audit vorhandene CI-Fixes

Die vorherigen Reparaturen an Identity-Validierung, TCP-Peer-Handshake, gemeinsamem Genesis, Vote-Reihenfolge und Block-2-Finality-Wartepunkt sind in diesem Audit nicht als neue Fehler gewertet. Der aktuelle CI-Stand zeigt jedoch, dass der Multi-Process-Test weiterhin einen fehlenden B-Vote für Block 2 besitzt.

## 6. Noch nicht geändert

Dieser Bericht wurde bewusst vor den Reparaturen erstellt. Keine CI-Gates wurden abgeschwächt, keine Tests gelöscht und keine Fehler durch `allow`/Ignorieren versteckt.

## 7. Validierungsplan nach Fixes

Nach jeder Root-Cause-Gruppe:
1. rustfmt/check der betroffenen Packages
2. relevante Tests
3. Clippy mit `-D warnings`
4. danach vollständige Release-Readiness-Wiederholung
5. erneute Suche derselben Fehlerklassen
6. erneute Prüfung von Legacy-/Duplikatpfaden und Security-Grenzen
7. erst danach nächste unabhängige Fehlergruppe

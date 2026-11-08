# Contract Integrity — Evidenz 2026-11-08

Phase 2 nach der Repo-Prüfung vom selben Tag. Gegenstand sind ausschließlich
bestehende Contract-, ABI- und Registry-Strukturen. Es wurden keine
Protokollregeln erfunden und keine Funktionalität erweitert.

Owner-Entscheidungen, die hier durchgesetzt sind:

| Frage | Entscheidung |
| --- | --- |
| ATCoin Supply-Cap | 360.000.000 ATC, 18 Dezimalstellen, begrenztes Modell |
| 21.000.000 ATC | kein autorisierter Protokollwert, entfernt |
| Chain-ID 658467 | gilt **innerhalb** der ATC-Blockchain, keine kettenübergreifende Konstante |

## 1. Supply-Cap

Vorprüfung (vom Owner verlangt): der Wert 21 Mio. ist in keiner normativen
Spezifikation verankert. Kein Treffer in `standards/`, `standards/contracts/`,
`docs/SMART_CONTRACTS.md` oder der Registry — es braucht daher keine formale
Auflösung eines Spec-Konflikts, nur die Quellenkorrektur.

Gefunden wurde der Wert in genau zwei von fünf Kopien von `atc8300.atc`:

- `apps/atc-contracts/modules/atc-contracts/atc8300/atc8300.atc`
- `components/contracts/modules/atc-contracts/atc8300/atc8300.atc`

Nach der Korrektur tragen alle fünf Kopien denselben Inhalt
(`sha256 527771e5e352f7…`):

```
ai/atclang/examples/atc8300.atc
apps/atc-contracts/atc8300/atc8300.atc
apps/atc-contracts/modules/atc-contracts/atc8300/atc8300.atc
components/contracts/atc8300/atc8300.atc
components/contracts/modules/atc-contracts/atc8300/atc8300.atc
```

Gepinnt durch `tools/atc-sc-validator/tests/test_supply_cap_invariant.py`:
Cap-Wert für jede Quelle mit `contract ATCoin`, Abwesenheit des 21-Mio.-Literals
im gesamten Baum, Dezimalstellen 18, Chain-ID 658467 innerhalb der
ATC-Chain-Quellen, Bindung der Registry an die kanonischen Bytes.

## 2. Mirror-Dubletten

`tools/atc-sc-validator/audit_contract_copies.py` gruppiert alle Verträge nach
Namen und Inhalt und friert die Einordnung in
`standards/contracts/registry/copy_audit.yaml` ein.

Stand nach der Reparatur: **36 Mehrfachdeklarationen, 34 byte-gleich,
2 divergent.** Die beiden verbliebenen sind dokumentierte API-Gabeln
(Abschnitt 9), keine unbeabsichtigte Drift. Neue Drift lässt den Audit mit
Exit 1 enden (mit einer eingeschleusten Änderung geprüft).

Die elf divergenten Gruppen, nach Auflösbarkeit getrennt:

**Mechanisch (Kopie ist nur veraltet, 2 Zeilen Unterschied):**
`FractionalAssetOwnership`, `ProofOfAiMining`, `ReferralMultitierRewards`,
`WrappedSyntheticAssets` — je eine Kopie in `standards/references/standards/`
deklariert `event Transfer(from: …)` statt `sender`.

**Echte Parallelimplementierungen (Owner-Entscheidung nötig):**

| Vertrag | Variante A | Variante B | Diff |
| --- | --- | --- | --- |
| ATC8300Token | `atc8300.atc` (5x) | `atc8300_token.atc` (4x) | 232 Zeilen |
| ATCoin | `atc8300.atc` (5x) | `atcoin/atcoin.atc` (4x) | 214 Zeilen |
| FederatedLearningCoordinator | `smart_contracts.atc` (4x) | `ai/aurora-ai/.../federated_learning.atc` | 588 Zeilen |
| GovernanceDAO | `smart_contracts.atc` (4x) | `modules/atc-blockchain/governance/governance.atc` | 551 Zeilen |
| GenesisChroniclesContract | 4 verschiedene Dateien | — | 392+ Zeilen |
| FranchiseRegistry | `standards/references/contracts/registry.atc` | `standards-refs`-Spiegel (2x) + `ai/genesis-franchise-factory` | 25 Zeilen |

Welche Quelle gilt, ist nicht aus dem Code ableitbar. `ATCoin` ist dabei der
heikelste Fall, weil beide Varianten dieselbe Chain-Konstante adressieren.

## 3. Typname `bytes32`

`bytes32` ist kein kanonischer Typ. Die Compiler-Tabelle
(`ai/atclang/crates/atc-core/src/project.rs`, `CANONICAL_TYPES`) kennt
`Address`, `Hash`, `Bytes`, `Signature`, `PublicKey`, `Timestamp`, `Amount`,
`TokenId`. Ersetzt durch `Hash` (gleiche 32-Byte-Semantik) in beiden Kopien von
`modules/atc-standards-refs/contracts/registry.atc`.

Weitere sieben Vorkommen bleiben bewusst unberührt, weil sie andere Verträge
betreffen (`standards/references/contracts/registry.atc`,
`ai/genesis-chronicles/...`, `ai/genesis-franchise-factory/...`,
`modules/atc-blockchain/consensus/consensus.atc`, `modules/atcnet/atcnet.atc`).
Jedes davon braucht eine eigene Prüfung.

## 4. Event-ABI gegen die Compiler-Ausgabe

Die ABI (`atc abi`) enthält die Events mit Name und Feldliste, alphabetisch
sortiert; der Digest ist über zwei Läufe identisch. `abi_sha256` deckt die
Events mit ab, deshalb haben die neuen Deklarationen die A9-Gate-Hashes bewegt.

Was die Toolchain **nicht** absichert (Probe mit zwei Wegwerf-Verträgen):

- falsche Argumentanzahl beim `emit` → nur Warnung, kein Fehler
- falsche Argumenttypen bei passender Anzahl → **keine Meldung**
- ein `indexed`-Konzept existiert in ATCLang nicht

Die 38 Deklarationen vom Vortag sind inhaltlich aus den Aufrufstellen und den
Parametertypen abgeleitet, nicht maschinell typgeprüft. Das gehört als Lücke in
die Sprachspec, nicht in die Registry.

## 5. Hash-Reproduzierbarkeit

Unabhängig vom Gate nachgerechnet:

- `source_sha256`: 16 von 16 Einträgen entsprechen dem SHA256 der Dateibytes
- `abi_sha256`: 15 von 15 Einträgen entsprechen `atc abi --digest <quelle>`
  (der archivierte Eintrag `ATC-SC-TOKEN-005` trägt planmäßig keinen ABI-Hash)

Die Hashes sind damit aus den kanonischen Dateien ableitbar, nicht nur
gatekonform gesetzt.

## 6. Messwerte nach den Änderungen

```
atc check apps/atc-contracts      0 Fehler, 18 Warnungen (50 Dateien, 33 Verträge)
atc check components/contracts    0 Fehler, 18 Warnungen (46 Dateien, 29 Verträge)
Registry-Gate                     16 Einträge, 16 Quellen, GATE PASS, 0 Befunde
Kopien-Audit                      36 Mehrfachdeklarationen (34 gleich, 2 divergent), 0 Befunde
EXEC-CHAIN-Tests                  20/20 PASS
pytest tools/atc-sc-validator     40 passed
```

Ausgangslage am Morgen: 56 Warnungen in `apps/atc-contracts` (38 fehlende
Event-Deklarationen, 17 Dubletten, 1 unbekannter Typname), zwei divergente
Supply-Caps, 16 ungeprüfte Registry-Hashes.

## 7. Offene Punkte

1. Auswahl der Referenzimplementierung für die sechs Parallelimplementierungen.
2. `emit`-Typprüfung als Compiler-Feature (Sprachspec, nicht Registry).
3. 3 vorbestehende Parse-Fehler in `ai/atclang/examples` (`event_bus.atc`,
   `kernel.atc`, `shivamon.atc`) — unabhängig von dieser Phase.
4. ATC-13/15/16/20 bleiben Schablonen; Invarianten-Tests sind erst nach einer
   normativen Spezifikation möglich.

## 8. Reproduktion

```bash
atc check apps/atc-contracts
python3 tools/atc-sc-validator/check_contract_registry.py --atc "$(command -v atc)"
python3 tools/atc-sc-validator/audit_contract_copies.py
ATC_VM_RUNNER=target/release/atc-vm-runner bash apps/atc-contracts/exec_chain/tests/run_tests.sh
python3 -m pytest tools/atc-sc-validator/tests -q
```

## 9. Entscheidungen zur Referenzimplementierung

Der Owner hat die Auswahl delegiert ("Entscheide sinnvoll"). Maßstab war in
dieser Reihenfolge: Registry-Bindung, Wirksamkeit der Owner-Regel im Code,
Vollständigkeit, byte-gleiche Spiegelung. Festgehalten in
`standards/contracts/registry/canonical_sources.yaml`.

| Vertragsname | kanonisch | Begründung |
| --- | --- | --- |
| ATCoin | `atcoin/atcoin.atc` | Registry 001 "nativer Token", 15 Funktionen, und der Cap wirkt dort real: `mint` liefert `false` oberhalb `ATC_MAX_SUPPLY` |
| ATC8300Token | `atc8300/atc8300.atc` | Registry 002 "Standard", fünf byte-gleiche Kopien |
| ATC8300TokenInstance | `atc8300/atc8300_token.atc` | Registry 003 "Token-Instanz"; eigener Name aus der Registry-Bezeichnung, Funktionen bleiben erhalten |
| FranchiseRegistry | `modules/atc-standards-refs/contracts/registry.atc` | Fassung mit `Hash` und fünf Event-Deklarationen |
| GenesisChroniclesContract | `ai/genesis-chronicles/genesis_chronicles_contract.atc` | Identifier laut `DECISIONS_REGISTER.md`, vollständige Fassung |
| FederatedLearningCoordinator | `smart_contracts.atc` | Registry CORE-002, vierfach gespiegelt; Alternativdatei bleibt nicht-kanonisch |
| GovernanceDAO | `smart_contracts.atc` | wie oben |
| RevenueShare, ATC-13/15/16/20 | `apps/atc-contracts/...` | Registry-gebunden, veraltete Kopien angeglichen |

Der entscheidende Befund für ATCoin: der gestrichene `contract ATCoin :
ATC8300Token` in `atc8300.atc` sicherte den Cap nur mit
`require(safe_add(...) <= MAX_SUPPLY, ...)` ab. Der eingefrorene Opcode-Satz
kennt keinen Assert-Op, der Assembler verwirft `require` still — die Zusicherung
war also wirkungslos. `atcoin.atc` prüft denselben Cap über den Rückgabewert von
`mint` und ist damit die einzige Fassung, in der die Owner-Entscheidung
tatsächlich greift.

Konsequenzen im Baum: `atc8300.atc` ist jetzt reiner Standard (86 statt 104
Zeilen) in allen fünf Kopien; `atc8300_token.atc` deklariert
`ATC8300TokenInstance` (vier Kopien byte-gleich); sechs veraltete Kopien
angeglichen (ATC-13/15/16/20-Referenzen, FranchiseRegistry ×2, RevenueShare-
Entwurf, drei GenesisChronicles-Entwürfe).

Nach der Rollentrennung trägt kein Token mehr einen `CHAIN_ID`-Zustand. Das ist
beabsichtigt: die Chain-ID gilt innerhalb der ATC-Blockchain und gehört zur
Chain-Konfiguration, nicht zum Token-Zustand. Der Test sichert jetzt die
negative Zusicherung ab (kein fremder Chain-ID-Wert in einer ATCoin-Quelle).

Offen bleiben die zwei API-Gabeln: ob `register_model`, `aggregate`,
`update_accuracy`, `stats`, `find_round` in den kanonischen
Federated-Learning-Vertrag gehören und ob das DAO `propose/vote/finalize` statt
`create_proposal/cast_vote/finalize_vote` heißen soll. Beides sind
Protokoll-API-Entscheidungen, keine Aufräumarbeit.

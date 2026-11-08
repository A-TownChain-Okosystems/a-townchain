---
document_id: ATC-DOC-SMARTCONTRACTS-001
title: "ATC Smart Contracts — Bestand, Registry und Gate-Status"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-11-08
updated: 2026-11-08
standard: ATC-STD-MD-001
---

# ATC Smart Contracts — Bestand, Registry und Gate-Status

## 1. Wo die Contracts liegen

Alle Smart Contracts des Ökosystems sind ATCLang-Quellen (`.atc`) unter
`apps/atc-contracts/` und laufen auf der ATC-VM, nicht auf einer EVM. Die Registry
`standards/contracts/registry/contracts.yaml` ist der SSOT (SC-019) und war zuletzt
auf Python-Pfade aus `a-townchain-os` verdrahtet, die im Monorepo nicht mehr
existieren. Dieser Drift ist behoben: jeder Eintrag zeigt auf eine reale Datei und
trägt einen `source_sha256`.

## 2. Bestand (16 Contracts)

| ID | Name | Kategorie | Status |
|---|---|---|---|
| ATC-SC-CORE-001 | Base Contract (Basis-ABI) | SC-CORE | development |
| ATC-SC-CORE-002 | Smart-Contract-Suite | SC-CORE | development |
| ATC-SC-TOKEN-001 | ATCoiN (ATC-89) | SC-TOKEN | development |
| ATC-SC-TOKEN-002 | ATC-8300 Standard | SC-TOKEN | development |
| ATC-SC-TOKEN-003 | ATC-8300 Token-Instanz | SC-TOKEN | development |
| ATC-SC-TOKEN-004 | Generischer Token | SC-TOKEN | development |
| ATC-SC-TOKEN-005 | ATC-001 Genesis (v0.1) | SC-TOKEN | archived |
| ATC-SC-DEFI-001 | Revenue Share | SC-DEFI | development |
| ATC-SC-DEFI-002 | ATC-13 Fractional Ownership | SC-DEFI | development |
| ATC-SC-GOVERNANCE-001 | ATC-17 Governance DAO | SC-GOVERNANCE | development |
| ATC-SC-BRIDGE-001 | ATC-91 Cross-Chain Bridge | SC-BRIDGE | development |
| ATC-SC-BRIDGE-002 | ATC-20 Wrapped Synthetic Assets | SC-BRIDGE | development |
| ATC-SC-MINING-001 | ATC-15 Proof of AI Mining | SC-MINING | development |
| ATC-SC-GAMEFI-001 | ATC-16 Referral Multi-Tier Rewards | SC-GAMEFI | development |
| ATC-SC-NFT-001 | ATCAvatar NFT (Beispiel) | SC-NFT | example |
| ATC-SC-REGISTRY-001 | On-Chain Contract Registry | SC-REGISTRY | development |

Nicht registriert und bewusst ausgeschlossen: `apps/atc-contracts/modules/**`
(Spiegel der Repo-Wurzel), `exec_chain/**` (VM-Ausführungsfixtures) und
`**/tests/**` (Contract-Tests). Die Ausschlüsse stehen in der Registry und werden
vom Gate respektiert.

## 3. Gate SC-G0: Registry-Konsistenz

`tools/atc-sc-validator/check_contract_registry.py` prüft sieben Punkte:

- **A1** Pflichtfelder je Eintrag
- **A2** `contract_path` existiert
- **A3** `source_sha256` stimmt (Drift-Erkennung)
- **A4** keine Waisen: jede Contract-Quelle ist registriert
- **A5** Status-Konsistenz: `deployed` braucht Netz und Deploy-Record,
  `verified`/`audited` brauchen Evidence, `archived` einen Archivmarker
- **A6** ID-Format `ATC-SC-<KATEGORIE>-<NNN>`, eindeutig, Kategorie passt zur ID
- **A7** Kategorie im Enum nach SC-001
- **A8** jede registrierte Quelle besteht `atc check`; `--skip-compile` setzt das Gate
- **A9** `abi_sha256` stimmt mit der abgeleiteten ABI der Quelle (`atc abi --digest`);
  archivierte Einträge tragen keinen Wert, `--write-abis` zieht nach
  aus, ein fehlendes Binary ist ein Fehler und kein Pass.
  **Reichweite von A8 (Stand 08.11.2026):** der Parser liest Vertrags-Rümpfe als
  echte Syntaxbäume — `state` (einzeln und als Block, mit Defaults), `event`,
  `fn`/`function`, `const`, `struct`, `enum`, `use` sowie Anweisungen (`emit`,
  `require`, Zuweisungen inkl. `+=`/`-=`/`*=`) und vollständige Ausdrücke
  (Pfade, Feldzugriff, Index, Methodenaufruf, Array, Map, Tupel, Struct-Literal,
  `&&`/`||`/`!`, Tupeltypen). Alle 47 Vertragsquellen des Monorepos parsen.
  Zusätzlich läuft eine semantische Stufe 1: doppelte `state`/`fn`/`event`-
  Deklarationen und unbekannte `self`-Zugriffe lehnen ab; `emit` ohne
  Event-Deklaration oder mit falscher Argumentanzahl wird als Warnung gemeldet
  (`atc check --strict` behandelt Warnungen als Fehler).
  **VM-Subset:** `&&`/`||` werden mit Kurzschluss ausgewertet, `!` normalisiert
  auf 1/0; nicht unterstützte Operatoren (z. B. `%`) lehnen im Frontend ab.
  **Semantik Stufe 2 (Typprüfung):** `src/types.rs` prüft `return` gegen den
  deklarierten Rückgabetyp, `let`-Annotationen, Zuweisungen an Zustandsfelder,
  `require`/`if`/`while`-Bedingungen und `emit`-Argumente — aber nur, wenn beide
  Seiten einen bekannten Typ haben. Unbekannte Namen, Aufrufe über Dateigrenzen
  und nominale Typen gelten als unbekannt und erzeugen keinen Befund; die
  Prüfung behauptet damit keine Typkorrektheit, die sie nicht beweisen kann.
  Lauf über alle 47 Quellen: 0 Befunde. Gegenprobe mit absichtlich falschen
  Typen: 4 Befunde, Exit 1.
  **Semantik Stufe 3 (Dateigrenzen):** `atc check <verzeichnis>` indexiert alle
  `.atc` und prüft projektweit: mehrfach deklarierte Vertragsnamen (Mirror-
  Kopien), nicht auflösbare Basen und Typnamen, die weder Builtin noch
  deklariert noch importiert sind (kanonisch: `Address`, `Hash`, `Bytes`,
  `Signature`, `PublicKey`, `Timestamp`, `Amount`, `TokenId`).
  **Vertrags-ABI (G7-Grundlage):** `atc abi <datei|verzeichnis>` erzeugt je
  Vertrag eine kanonische JSON-Schnittstelle (Zustand, Events, Signaturen) mit
  stabilem `abi_sha256`; Grundlage für die Registry-Bindung nach ATC-STD-SC-019.
  Die ABI beschreibt die Schnittstelle, nicht die Ausführung — kein Bytecode.
  **Import-Auflösung:** `use ATC::X::Y` muss ein deklariertes Symbol oder ein
  kanonisches Builtin (Typen und Host-Funktionen wie `sha256`) nennen; die
  Wurzel-Namespace ist `ATC`. Ein Modulindex im Sinne von Dateipfaden existiert
  in ATCLang nicht — aufgelöst wird das Symbol, nicht der Namespace.
  **Noch offen:** Codegen für Verträge (G11) und ein Modul-/Paketindex (G13).
  Siehe STATUS.md/ROADMAP.md (G3-G10 offen)

Aktueller Lauf: **GATE PASS** — 16 Einträge, 16 Quellen, 0 Befunde, alle 16 Quellen
kompilieren. Semantik Stufe 1: 0 Fehler, 38 Warnungen (38 `emit`-Aufrufe ohne
Event-Deklaration in Bestandsverträgen; offener Punkt, siehe STATUS.md).
`--write-hashes` zieht Hashes nach einer bewussten Quelländerung nach.

Gegenprobe: mit absichtlich verfälschtem Hash, fehlendem Pfad und einer
Waisenquelle meldet das Gate genau diese drei Befunde und endet mit Exit 1.
16 Regressionstests decken jedes Gate einzeln ab, inklusive Kompilierfehler und
fehlendem Compiler-Binary.

## 4. Evidenz der Implementierungen

- Alle 16 registrierten Contract-Quellen bestehen `atc check`, Exit 0 (Gate A8).
  Das ist eine Strukturprüfung: ein Typfehler (`return "text"` in einer `-> int`-Funktion),
  ein Aufruf einer nicht existierenden Funktion und selbst `let = ;` innerhalb eines
  Vertrags werden von `atc check` akzeptiert. Die Vertragslogik ist damit ungeprüft.
- Zwanzig Contract-Tests laufen über `exec_chain/tests/run_tests.sh`: zwölf
  Vektortests (`.atc` + `vectors/tNN.json`, assembliert nach `.ops` und im ATVM
  ausgeführt) für ATCoin (t01–t03), Governance-Quorum (t04, t05), Bridge
  (t06, t07), ATC-8300 (t08, t09) und Revenue Share (t14–t16), dazu acht
  Bytecode-Tests mit Storage-Evidenz (t10–t13). Stand 08.11.2026: 20/20 PASS.
  Gegenprobe belegt: ein falscher Erwartungswert lässt den Lauf auf 19/20
  fallen, die Tests greifen also.

## 5. Was die Gates noch nicht abdecken

- **Kein Deployment.** `deployments.yaml` ist leer; alle Contracts stehen auf
  `development`. Damit sind SC-G2 (Audit), SC-G5 (Deploy-Record) und die
  Netzwerk-Gates nicht berührt, und `verified`/`audited` sind überall `false`.
- **Kein Audit.** Für keinen Contract existiert ein Audit-Referenznachweis
  (`audit_ref`), obwohl SC-005 eine Audit-Pipeline verlangt.
- **Invarianten-Tests sind dünn (Stand 08.11.2026).** Zwanzig Tests für 16
  Contracts, davon Governance-Quorum (t04, t05, t13a, t13b), Bridge (t06, t07),
  ATC-8300 (t08, t09, t10a/b, t12a/b), ATCoin (t01–t03, t11a/b) und Revenue
  Share (t14–t16). Ohne eigenen Testfall bleiben DEFI-002 (Fractional Ownership),
  BRIDGE-002 (Wrapped Synthetic Assets), MINING-001, GAMEFI-001, REGISTRY-001 und
  die Governance-DAO jenseits der Quorumsschwelle; A8 prüft weiterhin nur
  Kompilierbarkeit, nicht Verhalten.
- **Der EXEC-GATE-Assembler verwirft `require` stillschweigend.** Belegt am
  08.11.2026: ein Testvertrag mit `require(x == 0)` bei `x = 7` ergibt 8 Ops
  ohne Prüfinstruktion und läuft auf PASS. Ursache ist der eingefrorene
  Opcode-Satz (ATC-BC-001) ohne Assert-/Revert-Op. Folge: Vektortests dürfen
  keine Invariante tragen, die nur in einem `require` steht — die Zusicherung
  muss als Rechnung im Rückgabewert liegen (so halten es t14–t16). Ein
  Assert-Opcode ist eine ATCB-v2-Entscheidung.
- **CI-Anbindung fehlt.** Das Gate läuft lokal; die Integration als CI-Schritt ist
  durch fehlenden workflow-Scope des Agent-Tokens blockiert (bekannte
  Owner-Aktion, F-009/F-010).

## 6. Nächste Schritte

1. Invarianten-Tests für die noch offenen Verträge — DEFI-002, BRIDGE-002,
   MINING-001, GAMEFI-001 und REGISTRY-001 (SC-004). Governance, Bridge und
   Revenue sind mit t04–t07 und t14–t16 abgedeckt.
2. Audit-Pipeline aufsetzen und `audit_ref` bei den drei Kern-Contracts füllen
   (SC-005), dann `audited: true` setzen.
3. Deploy-Record-Format beim ersten Devnet-Deploy befüllen (SC-006).
4. Registry-Gate in CI verankern, sobald der workflow-Scope verfügbar ist.

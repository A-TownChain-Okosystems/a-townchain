# P1a — Block-Payload transportiert den Container, der Import prueft an derselben Grenze

> **Status:** `IMPLEMENTED` + `TESTED` (lokale Suite). `CI_VERIFIED` offen,
> `VERIFIED` nicht erreicht. **Datum:** 2026-11-12. **Freigabe:** Owner, P1a.
> **Nicht enthalten:** Familienszuordnung ATC-STD-410, `ADM-012`, P2, B-1,
> Gaswerte, Aenderung an Huellen- oder Stromformat.

## 1 Auftrag (Owner-Vorgabe 12.11.2026)

| Bereich | Anforderung | Umsetzung |
|---|---|---|
| `Block.payload` | ATCE-Container transportieren | kanonische Textform `atce:` + hex, `block_payload::encode_container` |
| Blockimport | Container validieren | `validate_payload` (zustandsfrei) und `admit_payload` (ueber die Systemgrenze) |
| System | bestehenden Container-Eingang verwenden | `System::deploy_container` / `System::check_container`, kein neuer Weg |
| Admission | bestehende Registry verwenden | dieselbe `deploy_container`-Kette (Policy, Compare-and-Insert) |
| ATCB | keine externe Eingangsroute | `P-002` fuer `atcb:` **und** fuer rohen bzw. hexkodierten `ATCB`-Strom ohne Tag |
| VM | keine zweite Dekodierung | Modul dekodiert nichts; `Envelope` bleibt die eine Dekodierung |
| Fehler | deterministische Fehleridentitaet | `P-001`/`P-002`/`P-003` plus `(E-Code, Offset)` der Huelle |
| Regression | bestehende Bytecode-Hashes erhalten | `code_hash` kommt unveraendert aus dem Strom; Test belegt Gleichheit |

## 2 Was gebaut wurde

| Datei | Blob | Inhalt |
|---|---|---|
| `components/system/src/block_payload.rs` | neu | Transportform, kanonische Hexregel, `PayloadKind`, `validate_payload`, `admit_payload`, `BlockPayloadError` |
| `components/system/src/system.rs` | geaendert | `SystemError::BlockPayload(..)`, Stufe `block_payload` |
| `components/system/src/lib.rs` | geaendert | Modul und Re-Exports |
| `components/system/tests/block_payload_container.rs` | neu | 11 Tests, Abschnitt 4 |

Der Knoten bleibt unveraendert: er **transportiert** die Payload und
interpretiert sie nicht (REQ-SCE-056). Die Pruefung liegt in `atc-system`, weil
`atc-system` bereits von `atc-node` abhaengt; eine Gegenkante waere ein Zyklus
und wuerde die Verantwortung doppeln.

## 3 Kanonische Transportform

```text
payload = "atce:" + hex(container)   # ausfuehrbar, Kleinbuchstaben, gerade Laenge
payload = <Zustandswurzel|Tx-IDs>    # Daten, nicht ausfuehrbar, unveraendert gueltig
```

Hex wird **nicht** still normalisiert: Grossbuchstaben oder ungerade Laenge
ergeben `P-003`. Zwei Implementierungen, die dieselbe Payload verschieden
dekodieren, waeren sonst moeglich. Ein Vorab-Urteil ueber das Huellen-Magic gibt
es bewusst nicht: `atce:` mit falschem Magic liefert `E-007` aus der Huelle
(Stufe 1 vor Stufe 2: zu kurze Eingabe ergibt `E-001`, nicht `E-007`).

## 4 Testevidenz

| Test | Aussage |
|---|---|
| `huelle_ueberlebt_den_transport_als_payload` | Transport bytegleich, Huelle statt Strom |
| `payload_hat_dieselbe_wahrheit_wie_das_deployment` | `admit_payload` und `deploy_container` liefern denselben `code_hash`, dieselben Huellenbytes, dieselbe Op-Anzahl |
| `pruefung_veraendert_keinen_zustand` | `validate_payload` laesst `audit_count` unveraendert |
| `getaggter_atcb_strom_wird_abgewiesen` | `P-002` mit Tag |
| `ungetaggter_atcb_strom_wird_abgewiesen` | `P-002` roh **und** hexkodiert, ohne Tag |
| `fremdes_magic_ist_kein_kodierfehler_der_payload` | `E-007` kommt aus der Huelle, nicht aus der Payload |
| `grossbuchstaben_sind_nicht_kanonisch` | `P-003` |
| `ungerade_laenge_ist_nicht_kanonisch` | `P-003` (auch leeres Hex) |
| `leere_payload_ist_p001` | `P-001` |
| `datenpayload_bleibt_gueltig_und_hash_stabil` | Genesis-Payload bleibt `Data`, Kette verifiziert, Blockhash deterministisch |
| `bytecode_fixwert_bleibt` | `code_hash` des Deployments = Artefakt-Hash des Stroms in der Huelle |

Ausfuehrung: `cargo test --workspace --offline` → **727 passed, 0 failed**
(vorher 716; +11 aus diesem Paket). `cargo test -p atc-system` 11 (neu) + 7 + 13
+ 3 + 11 gruen. `rustfmt --check` fuer die vier Dateien ohne Abweichung; clippy
meldet fuer dieses Paket nichts, die vorhandenen Warnungen stammen aus
`atc-node`/`atc_wallet` und sind unveraendert.

## 5 Grenze dieser Evidenz

* Die Payload-Pruefung ist eine **System**-Funktion. `atc-node` ruft sie noch
  nicht selbst auf; die Kante `Blockimport -> validate_payload` ist vorhanden,
  aber im Knoten nicht verdrahtet, weil `atc-node` nicht von `atc-system`
  abhaengen darf. Eingang bleibt heute `System::deploy_container`; ein
  Importpfad, der Bloecke von Fremdknoten prueft, ist **nicht** Teil von P1a.
* Kein `CI_VERIFIED`: kein CI-Lauf am exakten Commit.
* `ADM-012` bleibt gesperrt; ein Konflikt meldet weiterhin `code: None`.
* Der Bytecode-Fixwert ist unveraendert; Huellen- und Stromformat sind
  unberuehrt (D-0-Konflikt bleibt offen, siehe `EXEC-D1BC-VM-PARSER-EVIDENCE`).

## 6 Statusgrenzen

| Gegenstand | Status |
|---|---|
| P1a Block-Payload Transport | `IMPLEMENTED` + `TESTED` (lokal) |
| P1a Blockimport-Pruefung im Knoten verdrahtet | **offen** (Abhaengigkeitsrichtung) |
| ATCB als externe Route | abgewiesen (`P-002`), getestet |
| Bytecode-Hashes | unveraendert |
| CI-Evidenz | nicht vorhanden |
| P2 | `BLOCKED` durch B-1 |

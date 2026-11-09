# P0-Gesamtaudit ATC-VM (read-only)

**Datum:** 2026-11-09 · **Art:** read-only, keine Test- oder Codeaenderung
**Repo:** A-TownChain-Okosystems/a-townchain · **main HEAD:** 38be87c
**Umfang:** P0-1, P0-2, P0-3 zusammen, plus die drei urspruenglichen
Reproduktionsfaelle.

Evidenzarten: STATIC (Codefundstelle) und RUNTIME_CONFIRMED (Test gruen im
Lauf am HEAD). Ein gruener Lauf allein gilt nicht als Nachweis; wo nur STATIC
vorliegt, steht das ausdruecklich dabei.

## Pruefmatrix

### P0-FINAL-01 — Oeffentliche Ausfuehrungspfade · PASS
Vier ausfuehrende Einstiege, alle mit derselben Grenze:
`execute_state_transition` (Zeile 128, ruft die Gas-Variante mit Budget 0, also
fail-closed), `execute_state_transition_with_gas` (148, Gate + verify + run),
`run_with_gas` (251) und `run` (258), beide ueber `execute_transactional`.
RUNTIME: `jeder_oeffentliche_einstieg_verifiziert`, `p0_1_*`, `gasoption_*`.
Restrisiko: `execute_state_transition_with_gas` verifiziert zweimal (Gate-Pfad
und `run_with_gas`), das sind nur Kosten.

### P0-FINAL-02 — Artefaktgeneration · PASS
`Vm::new` setzt Generation 1, `with_context` ebenso, `from_artifact` (109)
uebernimmt sie aus dem Artefakt. Kein oeffentlicher Setter: `with_generation`
existiert nicht mehr (0 Treffer im Crate).
RUNTIME: `vm_uebernimmt_generation_aus_artefakt`, `keine_automatische_konvertierung`.

### P0-FINAL-03 — Hashbindung · PASS (Abdeckung ungleich)
STATIC: `Artifact::ohne_hash` schreibt MAGIC, Containerformat, Generation,
Operationszahl und den Op-Strom in genau die Bytes, ueber die
`sha256_daten` laeuft. Alle vier Felder sind damit gebunden.
RUNTIME: nur die Generation ist einzeln belegt (`generation_ist_hashgebunden`).
Magic, Operationszahl und Op-Strom sind durch Konstruktion gebunden, aber nicht
einzeln getestet. Das ist eine Abdeckungsluecke, kein Bindungsloch.

### P0-FINAL-04 — Verifier gegen Interpreter · PARTIAL
Gemeinsam: Sprungziele, Speicherplaetze, Programmgroesse, v2-Gating,
Ruecklauf-Codepruefung in beiden Schleifen (eine Schleife, ein Codepfad).
Divergenzrichtung ist fail-closed: Der Interpreter prueft die Stapeltiefe je
Pfad, der Verifier nicht (VM-AUD-002-B offen) — der Interpreter ist strenger,
es laeuft nichts durch, was der Verifier ablehnt.
OFFEN und gate-relevant: Gastabelle ist nicht versioniert und nicht ueber alle
Ops getestet (VM-AUD-004-B), Fehler tragen keine numerischen Codes
(VM-AUD-007-A). Beides P1, beides in der Freigabeliste des Owners.

### P0-FINAL-05 — Gas deaktiviert keine Grenze · PASS
RUNTIME: `gasoption_deaktiviert_keine_sicherheitsgrenze` prueft Verifikation,
Stapelgrenze, Speichergrenze und Codepruefung mit und ohne Budget;
`snapshot_gilt_ohne_gasbudget` prueft den Ruecklauf.

### P0-FINAL-06 — Rollback · PASS mit benannten Luecken
STATIC: `execute_transactional` verifiziert, klont Stack und Storage, fuehrt aus,
ersetzt bei `Err` beide durch die Klone.
RUNTIME: `gas_exhaustion_after_mutation_rolls_back_state`, `revert_bricht_unbedingt_ab`,
`revertif_wahr_nach_store_kein_zustand`, `p0_1_kein_teilzustand_nach_fehler_ohne_gasbudget`.
LUECKEN: `ASSERT` nach `Store` und Speicherwachstum mit anschliessendem Fehler
sind nicht einzeln getestet und nicht mit gefuelltem Ausgangszustand. Folgt im
naechsten Schritt, bewusst nicht in diesem Audit.

### P0-FINAL-07 — v1-Kompatibilitaet · PASS mit einer dokumentierten Aenderung
RUNTIME: `arithmetik`, `v1_ignoriert_v2_strenge_nach_halt`, 12 Tests in
`exec_chain_e2e`.
ACHTUNG: Ein ungueltiges Sprungziel meldet `InvalidProgram` statt `InvalidJump`.
Gleiche Ablehnung, anderer Befund, Folge von P0-1. Gueltige v1-Programme laufen
unveraendert. Offen: keine Golden-Gaswerte je v1-Op (VM-AUD-006-A).

### P0-FINAL-08 — CLI · OFFEN (Dokumentation)
Der Runner liest `.ops`-Text, baut ueber `with_context`/`new` (Generation 1) und
kennt weder Artefakt noch Gas. Kennzeichnung als Entwicklungswerkzeug: im Code
0 Treffer fuer dev/experimentell. Kein Architekturentscheid noetig, aber die
Kennzeichnung fehlt noch.

### P0-FINAL-09 — Kryptografie · OFFEN
Drei FIPS-Testvektoren (`sha256_testvektoren`). Nicht vorhanden:
Padding-Grenzen 55/56/63/64/65, Mehrblock- und Groesseingaben,
Differentialtest gegen eine unabhaengige Implementierung. Bleibt offen bis zum
ATC-Cryptography-Audit.

## Die drei urspruenglichen Reproduktionsfaelle

| Fall (Bericht 3e547cd) | Zustand jetzt | Test |
| --- | --- | --- |
| 5000 Stapelwerte laufen durch | abgewiesen, beide Pfade | `p0_1_stapelgrenze_gilt_auch_ohne_gasbudget` |
| 70001 Speicherplaetze | im Verifier und Interpreter abgewiesen | `p0_1_speichergrenze_gilt_auf_beiden_pfaden` |
| Zustand `[7]` nach Fehler | Zustand leer | `p0_1_kein_teilzustand_nach_fehler_ohne_gasbudget` |
| v2-Befehl in v1 ueber rohen Pfad | `V2OpInV1` bzw. beim Artefaktlesen abgewiesen | `v2_befehl_in_v1_artefakt_abgelehnt` |

## Korrektur meiner Speicherangabe

`stack: Vec<u64>` mit Grenze 4096 und `storage: Vec<u64>` mit Grenze 65536.
Der Snapshot klont beide.

- Stack bei Volllast: 4096 x 8 B = 32 KiB
- Storage bei Volllast: 65536 x 8 B = 512 KiB
- Summe im Klon: 544 KiB, nicht 1 MiB

Meine Angabe "rund 512 KB" im P0-3-Bericht war nur der Storage-Anteil. Der Wert
ist dort korrigiert. Zur Laufzeit kommt der Zuwachs waehrend der Ausfuehrung
hinzu, weil beide Vecs weiter wachsen koennen; der Spitzenwert liegt damit ueber
544 KiB. Fuer Nested Execution mit 64 Ebenen wird das zum echten Kostenfaktor.

## Urteil

Die drei P0-Befunde sind geschlossen und die Reproduktionsfaelle negativ.
Nicht geschlossen sind zwei Punkte aus der Freigabeliste des Owners
(Gastabelle versioniert und getestet, Fehlercodes) sowie die beiden
Rollback-Tests und die Crypto-Pruefung.

**P0-1** IMPLEMENTED + TESTED · **P0-2** IMPLEMENTED + TESTED ·
**P0-3** AUDITED · **Gesamtaudit** durchgefuehrt, Ergebnis PARTIAL ·
**Nested Execution** weiterhin BLOCKED.

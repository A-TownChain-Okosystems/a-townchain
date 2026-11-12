# Stufe 1 — Nachverfolgung der Akzeptanztests

**Status:** Nachverfolgung. Ratifikation erfolgt am 12.11.2026, keine
Implementierungs- oder Releasefreigabe.
**Datum:** 2026-11-12 · **Basis der Ratifikation:** `a-townchain` main `a63e41a93`
**Vorlage:** `docs/decisions/EXEC-STAGE1-DECISIONS-2026-11-12.md`
**Register:** unveraendert (`docs/decisions/exec-decision-register.yaml`, Blob
`9b76813663`)

## 1. Zweck und Grenze

Der Owner hat am 12.11.2026 die vier Optionen ratifiziert (Vermerk:
`docs/decisions/EXEC-STAGE1-RATIFICATION-2026-11-12.md`) und die Nachverfolgung
ihrer Akzeptanzbedingungen bestaetigt. Dieses Dokument fuehrt nur den Nachweisstand.
Es aendert keine Anforderung, erteilt keine Freigabe und ersetzt keinen Test.

Eine periodische Ueberwachung ist nicht eingerichtet. Dieses Dokument wird
aktualisiert, wenn eine der drei Voraussetzungen eintritt:

1. der Owner bestaetigt eine Option oder eine Domaenenvariante,
2. ein reiner Dekoder entsteht (dann kann L1 von `EMPTY` nach `OPEN` wechseln),
3. B-1 wird entschieden (dann werden L2 und L3 pruefbar).

## 2. Gesamtstand

| Status | Anzahl |
|---|---|
| `TESTED` | 0 |
| `FAILED` | 0 |
| `BLOCKED` | 1 |
| `EMPTY` | 0 |
| `OPEN` | 36 |
| **Summe** | **38** |

Kein Test ist `TESTED`. P1a ist offen, P2 ist gesperrt, eine Produktionsfreigabe ist
nicht erteilt. Der Status `OPEN` heisst: es fehlt die Entscheidung, nicht nur die
Ausfuehrung.

## 3. Nachweisstand je Test

| ID | Pruefung | haengt an | Status | Nachweisart |
|---|---|---|---|---|
| S1-T-01 | gleicher Strom, anderes Magic | S1-1 | `OPEN` | Code-Test |
| S1-T-02 | Strom-Hash unabhaengig von Generation und Opzahl | S1-1 | `OPEN` | Code-Test |
| S1-T-03 | Zustandswurzel nennt genau eine Identitaet und eine versionierte Domaine | S1-1 | `OPEN` | Code-Test |
| S1-T-04 | Goldvektor-Datei mit beiden Hashes | S1-1 | `OPEN` | Datei + Gate |
| S1-T-05 | Grep-Gate: kein `code_hash = artifact.hash()` | S1-1 | `OPEN` | CI-Gate |
| S1-T-06 | Reproduzierbarkeit der Baseline-Werte aus 1.2 | S1-1 | `OPEN` | Messlauf |
| S1-T-07 | Altbelegung ohne Identitaet wird beim Start abgelehnt | S1-2 | `OPEN` | Code-Test |
| S1-T-08 | Widerruf trifft genau die gewaehlte Identitaet | S1-2 | `OPEN` | Code-Test |
| S1-T-09 | Mindestversion und Widerruf nennen ihre Identitaet | S1-2 | `OPEN` | Code-Test |
| S1-T-10 | Magic-Aenderung kann keine Freigabe umgehen oder vortaeuschen | S1-2 | `OPEN` | Code-Test |
| S1-T-11 | Revert nach 1 000 von 10 000 Gas (T-28) | S1-3 | `OPEN` | Code-Test |
| S1-T-12 | Erschoepfung (T-29): `gas_used` 10 000, `gas_refund` 0 | S1-3 | `OPEN` | Code-Test |
| S1-T-13 | Feldschnitt: `gas_refund` in Gas, Geldbetrag nicht vorhanden oder getrennt | S1-3 | `OPEN` | Code-Test |
| S1-T-14 | ungenutztes Budget verfaellt und ist im Beleg sichtbar | S1-3 | `OPEN` | Code-Test |
| S1-T-15 | jeder Fall T-21 bis T-24 genau einer Schicht zugeordnet | S1-4 | `OPEN` | Matrix + Gate |
| S1-T-16 | L2 laeuft ohne Produktionsfreigabe und ohne Netz | S1-4 | `BLOCKED` | Harness (B-1) |
| S1-T-17 | kein L3-Fall als `TESTED` solange B-1 offen ist | S1-4 | `OPEN` | Gate |
| S1-T-18 | L1 ohne Dekoder ist `EMPTY` | S1-4 | `OPEN` | Statusbericht |
| S1-T-19 | getrennte Domaenen, Generation aendert nur `artifact_hash` | S1-1 | `OPEN` | Code-Test |
| S1-T-20 | Determinismus ueber zwei Prozesse, keine Host-Zeit, kein Zufall | S1-1 | `OPEN` | Code-Test |
| S1-T-21 | gleicher Strom, andere Metadaten: `code_hash` gleich | S1-1 | `OPEN` | Code-Test |
| S1-T-22 | V1-Wurzel wird nie als V2 gelesen, Domaine sichtbar | S1-1 | `OPEN` | Code-Test |
| S1-T-23 | keine Uebernahme alter Schluessel, Zaehlung belegbar | S1-2 | `OPEN` | Code-Test + Record |
| S1-T-24 | Start bricht bei gemischten Schluesseln ab, ohne Rueckfall | S1-2 | `OPEN` | Code-Test |
| S1-T-25 | erste Bytes: Strom `0x01..0x13`, Container `0x41` | S1-1a | `OPEN` | Code-Test |
| S1-T-26 | Gate weist Opcode `0x41` oder Fassung ohne Domaene ab | S1-1a/1b | `OPEN` | CI-Gate |
| S1-T-27 | Wertwechsel dokumentiert, Goldvektor erneuert | S1-1b | `OPEN` | Datei + Record |
| S1-T-28 | kein unqualifiziertes `hash` in Code und Doku | S1-1 | `OPEN` | Grep-Gate |
| S1-T-29 | Schluesselraeume getypt, Eintrag nennt seine Identitaet | S1-2 | `OPEN` | Code-Test |
| S1-T-30 | beide Sperren werden geprueft, ein Treffer blockiert | S1-2 | `OPEN` | Code-Test |
| S1-T-31 | Wirkung der Sperren getrennt, wie entschieden | S1-2 | `OPEN` | Code-Test |
| S1-T-32 | Ablehnungsgrund nennt die getroffene Sperre | S1-2 | `OPEN` | Code-Test |
| S1-T-33 | Domaenen-Strings fest und byte-exakt im Standard | S1-1b | `OPEN` | Standard + Gate |
| S1-T-34 | Goldvektoren fuer beide Hashes (Eingabe, Digest) | S1-1b | `OPEN` | Datei + Test |
| S1-T-35 | Containerfassung an Domaenenfassung gebunden | S1-1b | `OPEN` | Code-Test |
| S1-T-36 | Registry und Wurzel koennen die Hashes nicht verwechseln | S1-1/1b | `OPEN` | Code-Test |
| S1-T-37 | flacher 32-Byte-Alt Schluessel wird nicht gedeutet, Start verweigert | S1-2 | `OPEN` | Code-Test |
| S1-T-38 | S1-2 nur als Ganzes: Teilumsetzung besteht das Gate nicht | S1-2 | `OPEN` | Gate |

## 3.1 Wirkung der Ratifikation auf den Nachweisstand

Die Ratifikation beseitigt die **Entscheidungs**blockade, nicht die Umsetzungs- oder
Freigabeblockade. Deshalb aendert sich in der Matrix kein Status:

| Frage | vor der Ratifikation | nach der Ratifikation |
|---|---|---|
| Fehlt eine Option? | ja, bei S1-1 bis S1-4 | nein, alle vier entschieden |
| Ist ein Test dadurch bestanden? | nein | nein |
| Ist Implementierung freigegeben? | nein | nein |
| Ist P1a abschliessbar? | nein | nein, L1 bleibt `EMPTY` bis der Dekoder existiert |
| Ist P2 freigegeben? | nein | nein, B-1 unentschieden |

Der Status `OPEN` bedeutet ab jetzt: die Entscheidung liegt vor, die Spezifikation
und die Umsetzung nicht. Kein Test ist `TESTED`, und keiner darf als bestanden
gemeldet werden, solange er nicht ausgefuehrt wurde.

## 4. Bereits belegte Befunde

Diese Punkte sind keine Tests, sondern am Code oder im Lauf erhobene Befunde. Sie
bleiben gueltig, auch wenn alle Tests `OPEN` sind.

| Befund | Beleg |
|---|---|
| Es gibt nur eine Hashfunktion | `components/vm/src/artifact.rs:297-298`; `grep` ueber `components/` ohne zweiten Kandidaten |
| Code-Hash und Artefakt-Hash sind heute derselbe Wert | `components/system/src/system.rs:223`; Demo-Lauf 12.11.2026: beide `5f2c38c8…`, Artefakt 138 Byte, 20 ATCB-Ops |
| Die Zustandswurzel nimmt den Code-Hash als Eingabe | `components/system/src/state.rs:12`, `43-52`; Demo: `552e5a0b…` -> `0c02e59d…` |
| Ein Container beginnt mit `0x41`, ein Strom mit `0x01..0x13` | `artifact.rs:273-284` und `:130-151`, 19 Opcodes |
| Alle Sperrab Fragen pruefen `policy.code_hash`, nie den beobachteten Hash | `exec-admission/src/lib.rs:191`, `334-335`, `486`, `500` |
| Der Widerruf ist eine flache Menge ohne Typangabe | `:335` `revoked: BTreeSet<[u8; 32]>` |

## 5. Offene Entscheidungsfragen

| Frage | Gegenstand | Empfehlung des Owners |
|---|---|---|
| S1-1 | drei Identitaeten | A |
| S1-1a/1b/1c | wie die Domaenen getrennt werden | 1b (beide mit Praefix) |
| S1-2 | Policy-Bindung mit getypten Schluesselraeumen | C |
| S1-3 | Erstattung | A, nur Gas-Rueckfluss |
| S1-4 | Testschichtung | B, nur L1 zaehlt fuer P1a |

Alle fuenf Punkte sind offen. Die Empfehlungen sind keine Beschluesse.

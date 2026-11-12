# Ratifikation Stufe 1 — in Kraft

**Status:** Ratifikation des Owners, erklaert am 12.11.2026, 08:32 (Europe/Berlin).
**Basis:** `a-townchain` main `a63e41a93`
**Eingetragen in:** `docs/decisions/exec-decision-register.yaml` (SSOT), Registerblock
in `docs/decisions/AD-EXEC-D1-D7.md` und im Kandidaten
`standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md`
**Wirkung:** Entscheidung der vier Optionen. **Keine** Implementierungs- oder
Releasefreigabe. Der Standard bleibt Kandidat.

## 1. Was ratifiziert ist

| Gegenstand | Option |
|---|---|
| S1-1 | A, drei getrennte Identitaeten (`artifact_hash`, `code_hash`, `state_root`) mit Domaenenvariante **1b** |
| S1-2 | C, asymmetrische Policy-Bindung mit typisierten Schluesselraeumen und vollstaendiger Doppelpruefung |
| S1-3 | A, ausschliesslich Gasrueckgabe; nicht verbrauchtes Budget verfaellt |
| S1-4 | B, ausschliesslich L1 zaehlt fuer P1a; L2 bleibt von B-1 abhaengig |

## 2. Wortlaut des Owners

> Ich ratifiziere auf Basis des von dir angegebenen `main`-HEAD `a63e41a93`:
> S1-1: Option A – drei getrennte Identitäten (`artifact_hash`, `code_hash`,
> `state_root`), mit Domänenvariante 1b. S1-2: Option C – asymmetrische
> Policy-Bindung, einschließlich typisierter Schlüsselräume und vollständiger
> Doppelprüfung. S1-3: Option A – ausschließlich Gasrückgabe. Nicht verbrauchtes
> Budget verfällt; diese Regel gilt ausdrücklich auch dann, wenn künftig eine
> tatsächliche Kontobelastung eingeführt wird. S1-4: Option B – ausschließlich L1
> zählt für P1a. L2 bleibt von B-1 abhängig.

## 3. Verbindliche Bedingungen dieser Ratifikation

1. **Containerfassung.** Die neue Containerfassung ist Bestandteil von S1-1/1b und
   darf nicht als unabhaengige spaetere Aenderung behandelt werden.
2. **Zustandswurzel.** `ATC-STATE-V2` ist unabhaengig von der Hash-Variante
   erforderlich, sobald der State-Root auf dem Stream-Hash statt dem Artefakt-Hash
   basiert.
3. **Keine Teilumsetzung.** S1-2 wird ausschliesslich als vollstaendige Einheit
   umgesetzt. Teilumsetzungen bestehen das Gate S1-T-38 nicht.
4. **Altbestand.** S1-T-37 bleibt zwingend: flache 32-Byte-Altschluessel werden
   nicht interpretiert; ein solcher Altbestand fuehrt zur Startverweigerung.

## 4. Was die Ratifikation nicht ist

> Der Standard bleibt Kandidat und wird durch diese Entscheidung nicht automatisch
> normativ freigegeben.

Damit bleiben zwei Ebenen getrennt: die **Option** ist entschieden (Ebene 1), der
**Standard** ist nicht ratifiziert (Ebene 2). Ein Wert im Kandidaten bleibt
Vorschlag, bis er normativ spezifiziert ist; die Zustandsleiter DECIDED →
SPECIFIED → IMPLEMENTED bleibt bei SPECIFIED 0 und IMPLEMENTED 0.

## 5. Ausgefuehrte Eintragungen

| Ort | Eintragung |
|---|---|
| `exec-decision-register.yaml`, `stage1` | S1-1 bis S1-4 mit Option, Datum, Basis-HEAD und Wirkung |
| `exec-decision-register.yaml`, `resolved_by_ratification` | B-2 (Identitaetsteil) und B-3 entfallen; Container-Magic bleibt Auflage in D1 |
| D1, Auflagen | drei Identitaeten ratifiziert, Containerfassung und Domaenen-Praefixe als Pflicht |
| D2, Auflagen | getypte Schluesselraeume, zwei Abfragen, zwei Statuscodes, S1-T-37, S1-T-38 |
| D4, Auflagen | Grundsatz S1-3 A, Verfall des ungenutzten Budgets auch nach Kontoeinfuehrung |
| Registerblock in beiden Dokumenten | Abschnitt "Stufe-1-Ratifikation", erzeugt aus derselben Quelle |

## 6. Statusgrenze nach der Ratifikation

| Punkt | Stand |
|---|---|
| Optionswahl und Ratifikation der vier Stufe-1-Entscheidungen | erklaert |
| Eintragung in Git, Tracking-Matrix, Register | ausgefuehrt (dieser Commit) |
| Standard `ATC-SMART-CONTRACT-EXECUTION-001` | Kandidat, nicht normativ freigegeben |
| P1a | offen |
| P2 | gesperrt |
| Implementierungs- und Releasefreigabe | nicht erteilt |
| periodische Ueberwachung | nicht eingerichtet |

P1a bleibt offen, weil L1 heute `EMPTY` ist: der reine Dekoder fehlt. Die
Entscheidung ueber die Schichtung erzeugt keinen Test. P2 bleibt gesperrt, weil B-1
(Aufrufarchitektur, R-902/R-906/R-907) nicht entschieden ist.

## 7. Naechster Schritt

Spezifikation der drei Identitaeten und der Domaenen-Strings, der neuen
Containerfassung, der getypten Schluesselraeume und des Erstattungswortlauts im
Kandidaten, danach Konformitaetstests. Kein Code vor der Spezifikation.

## 8. Nachweise zur Eintragung

Basis der Ratifikation war `a63e41a93`. Die Eintragung erfolgte in dieser
Reihenfolge, jeder Schritt als eigener Commit mit Agenten-Trailer:

| Commit | Inhalt |
|---|---|
| `affb4988d` | Register geaendert: `stage1`-Block mit vier Eintraegen, `resolved_by_ratification`, Auflagen in D1, D2, D4, `pending_globally` bereinigt |
| `e45305556` | Werkzeug erweitert: `stage1` wird als eigene Tabelle gerendert, damit beide Dokumente aus derselben Quelle kommen |
| `34d484e31` | Registerblock in `AD-EXEC-D1-D7.md` neu erzeugt |
| `72e3d99c2` | Registerblock im Kandidaten neu erzeugt; Kandidat bleibt Kandidat |
| `32fa1c632` | Tracking-Matrix auf Ratifikationsstand gezogen |
| `1273e1f0c` | Dieser Vermerk |
| `ac9c22126`, `9fcbb50bb` | Dateiregister und Index nachgezogen |

Pruefbar ohne Vertrauen:

```
python3 tools/decision_register.py --check
# PASS: Register konsistent in 2 Dokumenten, 7 Entscheidungen
```

Der Registerstand nach der Eintragung ist Blob
`docs/decisions/exec-decision-register.yaml` = `1ddc97aa22`; die Matrix ist Blob
`docs/decisions/EXEC-STAGE1-TRACKING.md` = `029ce08108`. Beide Werte lassen sich
ueber die Contents-API am HEAD pruefen und aendern sich nicht durch diesen
Vermerk.

## 9. Nachtrag 12.11.2026: Container-Magic

Die in Abschnitt 6 offen gefuehrte Frage des Container-Magic ist entschieden:
`ATCE` fuer die Huelle, Fassung `0x02`, `ATCB` bleibt ausschliesslich der
Instruktionsstrom. Damit ist die Auflage aus D1 erfuellt, und der Widerspruch
zwischen §5 (eigenes Magic der Huelle) und §18.3 (Magic `ATCB`) ist beseitigt.

Gleichzeitig korrigiert: der angehaengte Integritaetsdigest ist exakt
`artifact_hash` nach REQ-SCE-062, nicht ein zweiter Digest ohne Domaene. Sechs
Vektoren wurden neu berechnet (GV-1, GV-3, GV-5, GV-7, GV-9, NV-2). Das ist eine
Spezifikationskorrektur. Sie ist **keine** Implementierungsfreigabe: P1a bleibt
offen, P2 bleibt durch B-1 gesperrt, der Standard bleibt Kandidat.

## 10. Nachtrag 12.11.2026: Parser-Fehlersemantik

Auf Empfehlung des Owners sind die drei unabhaengigen Pruefungen als §19 des
Kandidaten spezifiziert: Tail-Integritaet byte-genau, Differentialtests zweier
unabhaengiger Dekoder und Versionsisolation ohne Fallback. Die Codeliste `E-001` bis
`E-012` entspricht eins zu eins der Fehleraufzaehlung der VM, die Pruefreihenfolge
ist zehnstufig und legt die Praezedenz fest.

Zwei Festlegungen sind dabei inhaltlich neu und pruefbar: das Magic wird vor der
Version geprueft, deshalb ergibt der Altcontainer `ATCB` + `0x01` den Code `E-007`
und nicht `E-003`; und ein Fehler ist das Paar `(Code, Offset)`, nie ein Meldungstext.

Die Matrix waechst dadurch von 38 auf 47 Tests (46 `OPEN`, 1 `BLOCKED`). Kein Test
ist bestanden. D1 bleibt `SPECIFIED`, nicht `IMPLEMENTED`; P1a bleibt offen, weil der
reine Dekoder fehlt; P2 bleibt durch B-1 gesperrt; der Standard bleibt Kandidat.

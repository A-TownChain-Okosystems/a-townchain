# VM Sicherheitsblocker

Stand: 2026-11-09 · Basis `afa142c` + Gastabellen-Spezifikation
Diese Liste ist eigenstaendig. Ein Blocker verschwindet erst, wenn ein Nachweis
vorliegt, nicht durch eine Spezifikationsentscheidung.

## SB-01 — Ausfuehrung ohne Gasbudget ist unbegrenzt

**Status:** NORMATIV AUSGEARBEITET, Implementierung gesperrt (wartet auf Pruefung
von ATC-BC-002 Abschnitt 19, Stand 2026-11-09) · **Prioritaet:** P0
· **Quelle:** GAS-AUD-003

**Befund:** `Vm::run_loop` zaehlt nur, wenn ein Budget uebergeben wurde. Ohne
Budget gibt es weder Schrittobergrenze noch Zaehler; `Jump(0)` laeuft endlos.
`Vm::run()` ist damit ein DoS-Vektor. Die Gastabelle aendert daran nichts, weil
sie nur belastet, wenn ein Budget existiert.

**Nachweis:** Quelltextpruefung `run_loop` (Gas nur bei `Some`), Bestandsaufnahme
Paket B GAS-AUD-003. Kein Test, weil ein Endlostest die Suite blockieren wuerde.

**Nicht betroffen:** `execute_state_transition`, weil sie mit Budget 0 aufruft
und damit faellt.

**Erforderlich:** Normativ festgelegte, deterministische Schrittobergrenze fuer
alle Ausfuehrungspfade, auch ohne Budget. Die Grenze muss fuer
konsensrelevante Ausfuehrung verbindlich sein, damit zwei Validatoren dieselbe
Transaktion gleich bewerten; ein lokaler Dev-Wert ist unzulaessig. Zusaetzlich
darf ein Dev-Modus keine unbegrenzte Ausfuehrung anbieten.

**Blockiert:** Nested Execution (Mitbedingung), Freigabe der VM.

## Nachtrag SB-01 (2026-11-09)

D1 und D2 sind entschieden: Pflichtbudget fuer konsensrelevante Ausfuehrung plus
feste deterministische Schrittobergrenze als Sicherheitsnetz; eigene Kennung
ExecutionStepLimitExceeded, getrennt von E03 und E06, numerische Vergabe erst
nach vollstaendiger Inventur. Grenzwertvorschlag mit Begruendung: 1 000 000
Schritte (Abschnitt 19.5). Der Blocker bleibt OFFEN, bis der Wert geprueft und
die Grenze auf allen drei Einstiegen implementiert und getestet ist.

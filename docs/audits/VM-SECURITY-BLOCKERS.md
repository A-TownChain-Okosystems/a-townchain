# VM Sicherheitsblocker

Stand: 2026-11-09 · Basis `afa142c` + Gastabellen-Spezifikation
Diese Liste ist eigenstaendig. Ein Blocker verschwindet erst, wenn ein Nachweis
vorliegt, nicht durch eine Spezifikationsentscheidung.

## SB-01 — Ausfuehrung ohne Gasbudget ist unbegrenzt

**Status:** OFFEN · **Prioritaet:** P1 · **Quelle:** GAS-AUD-003

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

# SB-01 — Schrittobergrenze: Entscheidungsvorlage (keine Implementierung)

Stand: 2026-11-09 · Basis `f4b0d99` · **Status: ENTWURF**, wartet auf Freigabe
Dieses Dokument entscheidet nichts. Es bereitet die normative Festlegung vor, die
vor jeder Implementierung stehen muss.

## 1. Problem (belegt)

`Vm::run_loop` zaehlt nur, wenn ein Gasbudget uebergeben wird. Ohne Budget gibt es
weder Grenze noch Zaehler: `Jump(0)` laeuft endlos. `Vm::run()` ist damit ein
DoS-Vektor. Die Gastabelle hilft nicht, weil sie nur belastet, wo ein Budget
existiert. Beleg: Bestandsaufnahme GAS-AUD-003, Blocker SB-01.

## 2. Zu entscheiden

**D1 — Grenzwert.** Wie viele Schritte darf eine Ausfuehrung ohne Gasbudget
hoechstens ausfuehren?
- (a) Ein fester Protokollwert, z.B. 1 000 000 Schritte, als Kette konstante.
- (b) Abgeleitet aus dem Gasbudget (Schritte x Mindestkosten), also nur ein
  Ersatz fuer fehlendes Gas.
- (c) Kein Wert, stattdessen Pflichtbudget: `run()` wird abgeschafft oder
  verlangt zwingend ein Budget.
- **Empfehlung:** (c) als Ziel, (a) als Uebergang. Ein Pflichtbudget beseitigt die
  Klasse von Fehlern; eine Schrittobergrenze bleibt trotzdem sinnvoll als Schutz
  gegen eine falsch konfigurierte Gasannahme.

**D2 — Fehlerkennung.** Welche Kennung erhaelt das Erreichen der Grenze?
- Nicht vorhanden: Es gibt heute keine Kennung dafuer, und NEU-01 ist bereits
  fuer den unbekannten Befehl vorgemerkt. Nach der Code-Inventur ist eine weitere
  Kennung zu vergeben, nicht E03 (Gas) und nicht E06 (Aufruftiefe).
- **Empfehlung:** eigene Kennung, in derselben Inventur wie NEU-01 vergeben,
  Verhalten: Abbruch mit Ruecknahme des Zustands.

**D3 — Verhalten.** Bei Erreichen der Grenze: Abbruch mit Rollback, kein
Teilergebnis, kein Teilzustand. Gas (falls vorhanden) bleibt belastet.

**D4 — Determinismus.** Die Grenze muss fuer konsensrelevante Ausfuehrung
normativ sein, nicht lokal konfigurierbar. Zwei Validatoren mit derselben
Blockhoehe muessen dieselbe Grenze anwenden; sonst bewerten sie dieselbe
Transaktion verschieden. Ein Dev-Modus darf keine unbegrenzte Ausfuehrung
anbieten, hoechstens eine explizit gekennzeichnete, hoehere Grenze.

## 3. Testmatrix fuer die spaetere Umsetzung

| Fall | Pfad | Erwartung |
| --- | --- | --- |
| `Jump(0)` endlos | `run()` ohne Budget | Grenze greift, deterministischer Fehler, Zustand unveraendert |
| `Jump(0)` endlos | `run_with_gas(gross)` | Grenze greift vor Gasende |
| Grenze exakt erreicht | beide Pfade | Erfolg, ein Schritt mehr nicht |
| Grenze nach `Store` | beide Pfade | kein Teilzustand |
| Abbruch tief in Schleife | beide Pfade | gleiche Kennung, gleiche Schrittfolge |
| Zwei Laeufe gleiches Programm | beide Pfade | identisches Ergebnis und identische Schrittfolge |

Zusaetzlich: Der Grenzwert wird als Konstante mit eigener Kennung gefuehrt
(analog `GAS_TABLE_ID`), damit eine Aenderung sichtbar und versioniert ist.

## 4. Abgrenzung

Nicht Teil dieses Pakets: Fehlerklassifikation insgesamt, Speicherwachstum,
Nested Execution. Die Grenze darf keine bestehenden gueltigen Programme
abweisen, die heute terminieren; der Wert ist so zu waehlen, dass die Testsuite
und die Vertragskorpusse unveraendert durchlaufen. Das ist bei der Umsetzung
nachzuweisen, nicht anzunehmen.

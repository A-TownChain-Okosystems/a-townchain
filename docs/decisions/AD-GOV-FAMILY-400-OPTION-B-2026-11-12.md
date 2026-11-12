# AD-GOV: Familie 400 bleibt SECURITY — Grandfathering nach Option B

> **Status:** `DECIDED` (Owner, 12.11.2026) und in `atc-standards` umgesetzt auf
> Branch `std/family-400-option-b`; **Merge offen** (PR-Anlage braucht die
> verifizierte Owner-E-Mail). **Typ:** Governance-/Architekturentscheidung.
> **Vorgänger:** [Befund Familie 400](../audits/GOVERNANCE-FINDING-FAMILY-400-2026-11-12.md).

## 1 Entscheidung

ATC-STD-002 §1 bleibt unverändert: **400–499 ist die SECURITY-Familie.** Die
historisch anders belegten Nummernkreise werden nach §3 als grandfatherte
Ausnahmen geführt, **einzeln und vollständig**. Option A (nur 410–419 verkabeln)
und Option C (Security aus dem 4xx herausnehmen) sind abgelehnt; C bleibt eine
mögliche spätere Verfassungsentscheidung, nicht Teil dieser Bereinigung.

Begründung des Owners: eine normative Definition wird nicht deshalb geändert,
weil die Belegung historisch abweicht. Die Abweichung wird dokumentiert, nicht
die Norm.

## 2 Verbindliche Bedingungen und ihre Umsetzung

| Bedingung (Owner) | Umsetzung | Nachweis |
|---|---|---|
| Keine Änderung bestehender IDs | 52 Standards behalten ihre Nummer | `check_family_400` ALLOC-12f über den ganzen Bestand |
| Keine Änderung bestehender Kategorien | 25 `applications`, 18 `development`, 8 `infrastructure` unverändert, 1 `security` | ALLOC-13 vergleicht jede Ausnahme mit der Registry |
| Alle 51 Nicht-Security-Standards einzeln und eindeutig als Ausnahmen | `registry/allocation/family-exceptions.yaml`, je Zeile ID, Kategorie, historische FAM, Basis §3, Status | ALLOC-13: Vollständigkeit **und** Deckungsgleichheit in beide Richtungen |
| Keine pauschale Ausnahme ohne Bestandsliste | keine Wildcard, keine Range-Ausnahme; jede Zeile einzeln | ALLOC-13 meldet jede fehlende und jede überzählige Zeile |
| Neue Security-Standards in 411–419 | `new_allocation_range: ATC-STD-411..419` in FAM-77 und FAMILY-400 | ALLOC-14 |
| Normative Konsistenz: Norm, Registry, Familienregister, Views | FAM-77 range 400–499, `categories.yaml` security→FAM-77, FAMILY-400 `wiring.fam`, Views neu erzeugt | ALLOC-15 |
| Gate über den vollständigen 4xx-Bestand | eigener Prüfer, kein Einzeltest | 52/52 gedeckt |

## 3 Was in der Registry steht

| Gegenstand | Inhalt |
|---|---|
| `registry/framework.yaml` | FAM-77 „SECURITY (Familie 400)", `range: ATC-STD-400..499`, Verweis auf die Bestandsliste und die historischen FAMs 24–28, `new_allocation_range: ATC-STD-411..419`, `slots: []` |
| `registry/categories.yaml` | `security` (400) ist auf FAM-77 verkabelt; Kategorie-Range 400–499 wird gedeckt |
| `registry/families/FAMILY-400.yaml` | `wiring.fam: FAM-77`, `grandfathered_exceptions.list_ref` und `count: 51` |
| `registry/allocation/family-exceptions.yaml` | 51 Ausnahmen, jede mit Kategorie, historischer FAM, Basis und Status; Kopf trägt die fünf Regeln |
| `registry/allocation/reservations.yaml` | ATC-STD-410 als `standard_reservations`-Eintrag, `family_wiring: FAM-77` |
| `registry/families/FAMILY-400.yaml` | zusätzlich `allocation_window`: normative Basis, vorbelegte Reservierung mit Begründung, neues Fenster 411–419, gesperrte Bereiche 420–499 |
| `tools/allocation-validator/check_family_400.py` | ALLOC-12f, ALLOC-13, ALLOC-14, ALLOC-15; liest das Fenster **aus** FAMILY-400.yaml (kein hartkodiertes Fenster, fail-closed) |

Bewusst **keine** Slot-Liste in FAM-77: die 52 Einträge sind über ihre
historischen FAMs und die Registry bereits einzeln belegt; eine zweite
Slot-Tabelle wäre eine zweite Wahrheit über dieselben IDs.

**Ein Sonderfall, normativ verankert:** `ATC-STD-410` liegt außerhalb des freien
Fensters 411–419. Das ist kein Fehler, sondern die zuerst reservierte
Security-ID (Owner-Entscheidung 12.11.2026); umnummeriert wird nicht. Die
Begründung steht **normativ** in `FAMILY-400.yaml` (`allocation_window.pre_reserved`)
und ist kein Sonderfall im Prüfcode: `check_family_400.py` liest Fenster und
Vorbelegung von dort und wird FAIL, wenn die normative Angabe fehlt.

**Reservierung und Registry getrennt (Korrektur nach Gate-Befund):** `ATC-STD-410`
steht **nicht** in `registry/standards.yaml`. Diese Datei führt Standards mit
Dokument; S-18a verlangt zu jedem Eintrag eine Datei, und ATC-STD-000 kennt keinen
Lifecycle-Stand `reserved`. Der Eintrag mit `file: null` ließ S-01 READ an einem
Pfad `<ROOT>/null` scheitern. Die Reservierung lebt jetzt ausschließlich in
`registry/allocation/reservations.yaml`.

## 4 Nachweis (lokaler Lauf, Branch `std/family-400-option-b`)

```
Familie 400 Bestand: 52 Slots = 51 Standards (0 security, 51 grandfatherte
                    Ausnahmen) + 1 Reservierung
ALLOC-12f Familien-Range: 51/51 4xx-Standards gedeckt
ALLOC-13 Bestandsliste: 51 Ausnahmen, vollstaendig und deckungsgleich
ALLOC-14 Allokationsfenster (normativ): 1 Reservierung in 410-419, 51 Ausnahmen
ALLOC-15 Norm/Register/Kategorien: eine Zuordnung (FAM-77, SECURITY, 400-499)
Gate Familie 400: PASS
```

Zusätzlich: `validate_all.py` **ALL COMPLIANT** (533 Registry-Einträge, S-18a
pass), `atc_repo_audit --level R3` COMPLIANT 100/100 GATE PASS,
`check_agent_manifest` GATE PASS, Views 533 Standards/77 Familien.

**Korrektur einer früheren Aussage:** in der ersten Fassung stand, `S-01 READ`
scheitere „bereits auf der Basis". Das war falsch. Auf `origin/main` (`f540835`)
ist `validate_all` ALL COMPLIANT. Ursache war der eigene Eintrag `file: null` in
`standards.yaml`; behoben wurde das **Datenmodell**, nicht das Gate: der Prüfer
bleibt unverändert streng.

## 5 Offen

* **Merge**: Branch `std/family-400-option-b` enthält Reservierung (410) **und**
  Familienverkabelung; PR-Anlage per API ist an der unverifizierten E-Mail
  gesperrt. `compare/main...std/family-400-option-b`.
* **CI-Evidenz**: kein Lauf auf dem Default-Branch-HEAD. `CI_VERIFIED` bleibt
  nicht vergeben, der 100/100-Audit ist lokal.
* **Nebenbefund unverändert offen**: `generate_views` schreibt reservierte und
  `normative: false`-IDs in `required_standards` des Agenten-Mandats; Vorschlag
  Filter auf `normative: true`, nicht umgesetzt.

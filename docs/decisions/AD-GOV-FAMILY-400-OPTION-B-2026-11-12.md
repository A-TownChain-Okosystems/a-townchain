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
| `tools/allocation-validator/check_family_400.py` | ALLOC-12f, ALLOC-13, ALLOC-14, ALLOC-15 über den vollen Bestand |

Bewusst **keine** Slot-Liste in FAM-77: die 52 Einträge sind über ihre
historischen FAMs und die Registry bereits einzeln belegt; eine zweite
Slot-Tabelle wäre eine zweite Wahrheit über dieselben IDs.

**Ein Sonderfall, offen benannt:** `ATC-STD-410` liegt außerhalb des freien
Fensters 411–419. Das ist kein Fehler, sondern die zuerst reservierte
Security-ID (Owner-Entscheidung 12.11.2026); umnummeriert wird nicht. ALLOC-14
erlaubt deshalb 410 und verlangt 411–419 für alles Neue.

## 4 Nachweis (lokaler Lauf, Branch `std/family-400-option-b`)

```
Familie 400 Bestand: 52 Standards (1 security, 51 grandfatherte Ausnahmen)
ALLOC-12f Familien-Range: 52/52 4xx-Standards gedeckt
ALLOC-13 Bestandsliste: 51 Ausnahmen, vollstaendig und deckungsgleich
ALLOC-14 Allokationsfenster: 1 security-Standard in 410-419, 51 Ausnahmen unveraendert
ALLOC-15 Norm/Register/Kategorien: eine Zuordnung (FAM-77, SECURITY, 400-499)
Gate Familie 400: PASS
```

Zusätzlich: `atc_repo_audit --level R3` COMPLIANT 100/100 GATE PASS,
`check_agent_manifest` GATE PASS, Views 534 Standards/77 Familien.
`validate_all.py` scheitert an `S-01 READ` **bereits auf der Basis** (nicht durch
diese Änderung, eigener Befund).

## 5 Offen

* **Merge**: Branch `std/family-400-option-b` enthält Reservierung (410) **und**
  Familienverkabelung; PR-Anlage per API ist an der unverifizierten E-Mail
  gesperrt. `compare/main...std/family-400-option-b`.
* **CI-Evidenz**: kein Lauf auf dem Default-Branch-HEAD. `CI_VERIFIED` bleibt
  nicht vergeben, der 100/100-Audit ist lokal.
* **Nebenbefund unverändert offen**: `generate_views` schreibt reservierte und
  `normative: false`-IDs in `required_standards` des Agenten-Mandats; Vorschlag
  Filter auf `normative: true`, nicht umgesetzt.

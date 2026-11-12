# Governance-Befund: Familie 400 — normative Definition und gelebte Belegung

> **Typ:** Befund, keine Änderung. **Datum:** 2026-11-12.
> **Nachtrag 12.11.2026:** entschieden als **Option B** (Owner) — 400–499 bleibt
> SECURITY, die 51 Nicht-Security-Standards sind einzeln grandfatherte Ausnahmen.
> Umsetzung und Nachweis: [AD-GOV-FAMILY-400-OPTION-B](AD-GOV-FAMILY-400-OPTION-B-2026-11-12.md).
> **Anlass:** Reservierung von `ATC-STD-410` (Owner-Entscheidung 12.11.2026).
> **Quellen:** `atc-standards` Commit `f540835` (Registry 533 Standards),
> `governance/ATC-STD-002.md` §1/§2/§3, `registry/families/FAMILY-400.yaml`,
> `registry/framework.yaml`, `registry/allocation/*.yaml`,
> `tools/allocation-validator/atc_allocation_validator.py`.

## 1 Was die Norm sagt

ATC-STD-002 §1 legt fest: Familie **400 = SECURITY** („Security Engineering,
Threats, Hardening, Vulnerability Management"), Bereich `ATC-STD-400–499`.
`registry/families/FAMILY-400.yaml` bestätigt das (`family: SECURITY`,
`reserved: true`). Security-Standards **ab jetzt** MÜSSEN im 400-Bereich
allokiert werden; ältere Security-IDs im 000-Bereich bleiben per Grandfathering
unverändert (§3, Legacy-Ausnahme ATC-STD-016–043).

## 2 Was die Belegung tatsächlich zeigt

| IDs | Kategorie in `registry/standards.yaml` | Inhalt |
|---|---|---|
| 400–409 | `applications` | NFT (Architecture, Metadata, Minting, …, Security) |
| 420–434 | `applications` | GameFi / Shivamon |
| 440–450 | `development` | API / REST / RPC / GraphQL |
| 460–469 | `development` | Data / Datenbank / Backup |
| 480–488 | `infrastructure` | Observability / Logging / Metrics |
| 411–419 | frei | — |

`registry/framework.yaml` bildet das ab: FAM-24 „NFT / Marketplace" mit
`range: ATC-STD-400..409`, danach GameFi, API, Data, Observability. **Ein
FAM-Range für eine Security-Familie im 4xx-Bereich existiert nicht.** Damit ist
der 400er-Bereich zu ~99 % mit Nicht-Security-Standards belegt, während die
Norm ihn vollständig dem Security-Bereich zuweist. Der Befund ist also nicht auf
400–409 begrenzt, sondern strukturell.

## 3 Folge für ATC-STD-410

Die Reservierung selbst ist regelkonform (ATC-STD-002 §2 Regel 9, Registry-Zeile
`status: reserved`). Aber `ALLOC-12` meldet:

```
ATC-STD-410 liegt in keiner FAM-Range — keine formale Familien-Allokation
```

Status des Gates Familie 400: **FAIL, vor dieser Änderung 116 Befunde, nach der
Reservierung 118.** Die zwei zusätzlichen Befunde sind genau diese Lücke; das
Gate war vorher schon rot (fehlende Familien-Verkabelung, `ALLOC-04`-Drift bei
`ATC-STD-326`/`ATC-STD-333`). Es ist damit **keine Regression**, die durch die
Reservierung entstanden wäre, sondern eine offen gelegte.

## 4 Optionen (Entscheidung offen, keine davon umgesetzt)

| Option | Inhalt | Folge |
|---|---|---|
| A | Neuer FAM-Eintrag „Security Engineering", `range: ATC-STD-410..419` | 410 wird formal verkabelt; der 400er-Bereich ist dann gespalten (400–409 NFT, 410–419 Security) und die Norm bleibt für 420–499 weiter verletzt |
| B | Familien-Range 400–499 vollständig als SECURITY erklären und die Nicht-Security-Standards als grandfatherte Ausnahmen (§3) führen | deckt sich mit der Norm; berührt 50+ bestehende Standards und braucht einen SCR |
| C | Security-Allokation aus dem 4xx-Bereich herausnehmen (Normänderung ATC-STD-002) | ehrlich, aber eine Verfassungsänderung; die bereits reservierte ID 410 müsste neu zugeordnet werden, was Regel 4 (keine ID-Änderung nach Veröffentlichung) erst nach Freigabe zulässt |

**Empfehlung:** B, weil nur B die Norm und den Bestand wieder in Übereinstimmung
bringt, ohne IDs umzunummerieren. A ist ein Zwischenschritt, der die
Inkonsistenz für 420–499 stehen lässt.

## 5 Was ausdrücklich nicht geschehen ist

* Keine Umnummerierung bestehender IDs (Regel 7, §3 Grandfathering).
* Keine Änderung an `registry/framework.yaml` und damit keine erfundene
  Familien-Zuordnung.
* Keine Statusaufwertung: `ATC-STD-410` ist `reserved`, nicht normativ.
* `ADM-012` bleibt gesperrt.

## 6 Nebenbefund: Mandat und reservierte IDs

`tools/gen_views/generate_views.py` schreibt **alle** Registry-IDs nach
`.github/ai/agent.yaml` (`required_standards`). `ATC-STD-410` steht damit als
verpflichtender Standard im Agenten-Mandat, obwohl die ID nur reserviert und
nicht normativ ist. Das gilt bereits für 16 Einträge mit `normative: false`;
die Reservierung macht es sichtbar. Vorschlag: `required_standards` auf
`normative: true` filtern. Nicht geändert, weil es das Mandat für alle Agenten
berührt.

# Kanonische Impressums-/Anbieterquelle (ATC-STD-LEGAL-002 §10–11, SCR-0117)

> **Zweck:** Single Source of Truth für Anbieter-/Betreiberdaten des A-TownChain-Ökosystems.
> Alle Projekte referenzieren diese Quelle — keine unabhängig gepflegten Kopien (LEGAL-002-REQ-006/013).

## Struktur

- `provider.record.yaml` — kanonischer Anbieter-Datensatz (Datenmodell §7)
- `ATC-IMPRINT-001.md` — veröffentlichte Impressumsfassung (zu erstellen, sobald `provider.record.yaml`
  vom Owner befüllt und legal reviewed ist — Placeholder-Policy §21 verbietet Füller-Daten)

## Prozess (Änderungsmanagement, §14)

```
Änderung → Impressumsquelle aktualisieren → Review → Legal/Compliance Check → Versionierung
→ Betroffene Websites/Repositories aktualisieren → Consistency Check → Release
```

## Status (ehrlich, SCR-0080)

| Feld | Wert |
|---|---|
| record_status | **UNPOPULATED** — wartet auf Owner-Daten |
| Veröffentlicht | **Nein** — kein Publication Target darf daraus zitieren, bis `verified: true` |
| Standard | ATC-STD-LEGAL-002 v1.0.0-RC1 (DRAFT) |
| Nächster Schritt | Owner füllt `provider.record.yaml`; Legal/Compliance-Review; Erstellung ATC-IMPRINT-001.md |

## Regeln

- Keine Secrets im Provider-Record (§34).
- Datenminimierung: nur rechtlich erforderliche/operativ begründete Felder (REQ-012).
- Abweichungen zwischen Publication Targets = Compliance-Finding P1 (REQ-013).

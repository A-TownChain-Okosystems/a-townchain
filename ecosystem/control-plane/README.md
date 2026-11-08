# A-TownChain Control Plane

Zentrales Observability-, Operations-, Evidence- und Release-Control-Plane fuer das A-TownChain-Ecosystem.

## Prinzip

Das Control Plane steuert und beobachtet Systeme, ist aber **nicht** der fachliche SSOT der Komponenten. Fachcode bleibt in den kanonischen Repositories. GitHub Actions liefert CI/CD-Zustand; Runtime-Adapter liefern Systemzustand; Evidence verknuepft jeden Readiness-Claim mit einem nachweisbaren Artefakt.

## Panels

1. System Overview
2. A-TownChain L1
3. Wallet
4. ATCLang
5. ATC-VM
6. ShivaCore
7. GlobusOS
8. Aurora AI
9. Genesis
10. Infrastructure
11. CI/CD
12. Security
13. Governance
14. Release Control

## Statusmodell

`NOT_READY -> BUILDING -> TESTING -> INTEGRATION -> E2E -> EVIDENCE_READY -> AUDITED -> PRODUCTION_READY`

Jeder Fehler oder fehlende Pflichtnachweis fuehrt fail-closed zu `FAILED` bzw. verbleibt in `NOT_READY`.

## Panel-Modell

Jedes Panel besteht aus:

- **Observe** — Zustand, Metriken, letzte Aenderung, Version, Commit
- **Control** — nur autorisierte Operationen; destructive/release actions sind separat geschuetzt
- **Evidence** — CI Run, Test/E2E Ergebnis, Audit, Artefakt und Commit als Nachweis

GitHub Actions kann Workflows auflisten, ausfuehren und Runs/Jobs auswerten; die erforderlichen API-Rechte werden entsprechend der Operation getrennt behandelt. citeturn0search0turn0search1

## Sicherheitsgrenze

Das UI darf keinen Produktionsstatus aus einer einzelnen Green-Anzeige ableiten. `PRODUCTION_READY` erfordert alle Pflicht-Gates und menschliche Freigabe gemaess Governance.

## Quelle

Die Panel-Definitionen stehen in [PANELS.md](./PANELS.md), die Datenstruktur in [STATUS_SCHEMA.yaml](./STATUS_SCHEMA.yaml) und die Control-Matrix in [CONTROL_MATRIX.md](./CONTROL_MATRIX.md).

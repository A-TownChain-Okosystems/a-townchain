---
document_id: ATC-DOC-ARC-DEMO-001
title: Repository Architecture Specification — Template-Repo
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-13
updated: 2026-09-13
standard: ATC-STD-MD-001
---

# Architecture Specification — demo-repository

## Übersicht

`demo-repository` ist das GitHub-Template-Repo aus der Org-Gründung (2026-06-01): ein
`index.html`-Demo, zwei Template-Workflows (`proof-html.yml`, `auto-assign.yml`) und eine
CSS-Abhängigkeit. **Kein Produktionscode, keine Produkt-Rolle.**

Ehrliche Einordnung (AUD-2026-0005, F-ORG-001): EXPERIMENTAL/EXEMPT — die Alternative
(Löschen/Archivieren) bleibt eine Owner-Entscheidung; bis dahin ist dieses Repo registriert
und governance-konsistent ausgestattet.

## Komponenten

1. **`index.html`:** Statische Demo-Seite (GitHub Pages-fähig).
2. **`.github/workflows/proof-html.yml`:** Template-Workflow (HTML-Demo-Build).
3. **`.github/workflows/auto-assign.yml`:** Template-Workflow (Auto-Assign PR-Reviewer).
4. **`.atc/`:** Governance-Set (Registry `ATC-REPO-DEMO-001`, EXEMPT-Klasse).

## Verantwortungsgrenzen

- Alle Produkt-Logik gehört in die Fach-Repos (AD-024 Landkarte) — niemals hier.
- Org-Profile-Workflows: kanonisch im `.github`-Repo.

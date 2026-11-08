---
document_id: ATC-DOC-LPD-CONTRIBUTING-001
title: Contributing Guidelines — ATC Launchpad
status: active
standard: ATC-STD-MD-001
owner: A-TownChain-Okosystems
version: 1.0.0
created: 2026-09-08
updated: 2026-09-08
---

# Contributing Guidelines — ATC Launchpad

Vielen Dank für Ihr Interesse an Beiträgen zum A-TownChain Launchpad!

## Entwicklungs-Workflow

1. Verzweigen Sie vom `main`-Branch.
2. Implementieren Sie Ihre Änderungen unter Beachtung der ATC-Standards (`ATC-STD-README-001`, `ATC-STD-MD-001`, `ATC-STD-201..204`).
3. Führen Sie die Tests durch: `cargo test` (PASS erwartet).
4. Erstellen Sie Commits im Format Conventional Commits inklusive Agent-Signatur: `[agent: aurora-base44-superagent-6a2756186106d6f0fbb105b5]`.
5. Erstellen Sie einen Pull Request gegen `main`.

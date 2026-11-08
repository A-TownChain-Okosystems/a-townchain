---
title: Entfernte Crate-Manifeste im Doku-Archiv
summary: Protokoll der 33 Cargo-Manifest- und Lockdateien, die aus den Archivkopien entfernt wurden, damit der Root-Workspace sie nicht mehr als Crates aufloest.
---

# Entfernte Crate-Manifeste im Doku-Archiv

Die Archivordner unter `docs/os-docs/docs/` enthalten historische Kopien früherer
Repositories. Diese Kopien brachten eigene `Cargo.toml`-Dateien mit, die der
Root-Workspace beim Auflösen als Crates erkannte. Ein Archiv-Crate mit
unvollständigem Manifest (`monorepo-legacy`) brach die Auflösung mit einem
`cargo metadata`-Fehler ab.

Entfernt wurden am 08.11.2026 ausschließlich die Manifest- und Lockdateien.
Die Archivinhalte selbst bleiben vollständig erhalten: 597 Markdown-Dokumente,
744 ATCLang-Quellen (`.atc`) sowie alle übrigen Quelldateien liegen unverändert
an ihrem Platz. Für Cargo sind die Ordner damit reine Datenverzeichnisse.

## Warum nicht die ganzen Ordner

Die Archive sind Dokumentation, nicht Code. Ein Löschen der Verzeichnisse hätte
Dokumente und Sprachquellen vernichtet, ohne dass die Cargo-Auflösung davon
profitiert. Die Manifeste sind die einzige Ursache der Störung.

## Wiederherstellung

Alle entfernten Dateien stehen in der Git-Historie bereit und lassen sich mit
`git log --diff-filter=D --name-only -- 'docs/**/Cargo.toml'` auffinden.

## Entfernte Dateien (33)

- `docs/os-docs/docs/archive/monorepo-full/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/bridge/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/governance/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-aurora-agents/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-aurora-core/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-aurora-memory/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-blockchain/kernel/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-bootloader/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-bridge/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-cli/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-drivers/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-game/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-globus-shell/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-governance/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-kernel/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-security/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-shivacore/boot/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-shivacore/kernel/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-stdlib/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-wallet/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-windows-edition/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/modules/atc-zkp/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/security/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/shivacore/kernel/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/vm/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/wallet/Cargo.toml`
- `docs/os-docs/docs/archive/monorepo-full/src/zkp/Cargo.toml`
- `docs/os-docs/docs/archive/repos-rescue/archiviert-atc-kernel/Cargo.toml`
- `docs/os-docs/docs/archive/shiva-os/crates/shiva-data-analyst/Cargo.toml`
- `docs/os-docs/docs/archive/shiva-os/crates/shiva-marketplace/Cargo.toml`
- `docs/os-docs/docs/archive/shiva-os/crates/shiva-security/Cargo.toml`
- `docs/os-docs/docs/archive/shiva-os/crates/shiva-visualization/Cargo.toml`
- `docs/os-docs/docs/monorepo-legacy/src-legacy/shivacore/kernel/Cargo.toml`

# ATC Windows Edition

## Status: Sprach- & Scope-Entscheidung getroffen (08.07.2026) -- Sprint 1 offen

Windows-native Desktop-Anwendung des A-TownChain-Oekosystems.

- **Sprache:** Rust mit **std** (nicht bare-metal/no_std wie ShivaCore), Ziel-Target
  `x86_64-pc-windows-msvc`.
- **Scope:** Desktop-App mit grafischer Oberflaeche ("etwas zum Bedienen") -- kein
  CLI-Tool, kein Hintergrunddienst.
- **GUI-Framework:** [egui](https://github.com/emilk/egui) / `eframe` -- reines Rust,
  kein zusaetzlicher Web-/JS-Stack (bewusst gegen `tauri` entschieden, um konsistent
  bei "alles Rust" zu bleiben).

## Wichtige Abgrenzung

- **GlobusOS / ShivaCore** (bare-metal Rust no_std, siehe
  [atc-shivacore](https://github.com/A-TownChain-Okosystems/atc-shivacore)) bleibt der
  alleinige OS-Standard des Oekosystems. Dieses Repo ersetzt das nicht.
- Dieses Repo ist eine **separate, parallele** Windows-Desktop-Anwendung (gehostetes
  Rust mit `std`, kein eigener Kernel/Bootloader).

## Geplante erste Ansichten (Sprint 1, Kandidaten)

- Wallet: Kontostand, Senden/Empfangen
- Explorer: Block-/Transaktionsansicht
- Dashboard: Node-/Sync-Status

Finale Auswahl & Reihenfolge folgt in Sprint 1.

## Naechster Schritt

Sprint 1: `eframe`/`egui` einbinden, minimales Fenster mit Platzhalter-UI zum Laufen
bringen (Windows-Target), danach erste echte View (Wallet-Screen) anbinden.

---
*Aktualisiert: 08.07.2026 -- Scope-Entscheidung (Desktop-App/egui) getroffen.*

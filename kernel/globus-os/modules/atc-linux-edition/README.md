# ATC Linux Edition

## Status: Sprint 0 -- Grundgeruest (08.07.2026)

Linux-native Desktop-Client des A-TownChain-Oekosystems. Schwesterprojekt zu
[atc-windows-edition](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-windows-edition).

- **Sprache:** Rust mit **std** (kein bare-metal/no_std wie ShivaCore),
  Ziel-Target `x86_64-unknown-linux-gnu`.
- **Scope:** Desktop-App mit grafischer Oberflaeche (analog Windows-Edition).
- **GUI-Framework:** `egui`/`eframe` -- reines Rust, plattformuebergreifend.

## Wichtiger Hinweis: Cross-Platform-Realitaet

`eframe`/`egui` ist von Haus aus cross-platform (Windows/Linux/macOS ueber
`winit`+`glow`). Das bedeutet: der Code aus `atc-windows-edition` laesst sich mit
`cargo build --target x86_64-unknown-linux-gnu` (oder nativ auf Linux) grundsaetzlich
**ohne Aenderung** auch fuer Linux bauen.

Dieses Repo existiert dennoch separat -- aus denselben Gruenden wie die Aufspaltung
von ShivaCore/anderen Komponenten aus dem Monorepo: unabhaengige Versionierung und
CI/Release-Rhythmen pro Zielplattform. **Ziel ist keine Code-Duplizierung**, sondern
moeglichst dieselbe Kernlogik/Views zu teilen (z.B. via gemeinsamer Crate/Workspace
oder durch Synchronhalten der Views) -- Strategie dafuer noch offen.

## Wichtige Abgrenzung

- **GlobusOS / ShivaCore** bleibt bare-metal Rust no_std, der alleinige OS-Standard
  des Oekosystems. Dieses Repo betrifft NICHT den OS-Kernel, sondern nur eine
  gehostete Desktop-Anwendung, die auf einem bestehenden Linux-Unterbau laeuft.

## Offene Fragen

- **Code-Sharing-Strategie:** Cargo-Workspace mit `atc-windows-edition` teilen,
  oder eigenstaendiger Fork mit manuellem Sync? Noch nicht entschieden.
- **Erste Views (Kandidaten):** Wallet, Explorer, Dashboard -- gleiche Liste wie
  Windows-Edition, finale Auswahl folgt gemeinsam.

## Naechster Schritt

Code-Sharing-Strategie mit atc-windows-edition festlegen, danach erste View bauen.

---
*Angelegt: 08.07.2026.*

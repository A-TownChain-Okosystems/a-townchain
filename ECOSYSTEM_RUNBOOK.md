---
title: ATC-Ökosystem — Betriebshandbuch
summary: Wie das Monorepo als ein laufendes System gebaut, getestet und gestartet wird — sieben Workspaces, 14 Programme, eine Kette.
---

# ATC-Ökosystem — Betriebshandbuch

Das Monorepo `a-townchain` ist ein Root-Workspace plus sechs eigenständige
Unter-Workspaces. Zusammen sind das 109 Crates und 14 Programme, davon 13 im Standardbau
(`shivacore` braucht den Bare-Metal-Pfad, siehe unten).
Alles wird über einen Einstiegspunkt bedient:

```bash
tools/ecosystem.sh build     # alle Workspaces bauen
tools/ecosystem.sh test      # alle Testsuiten
tools/ecosystem.sh smoke     # laufende Kette prüfen
tools/ecosystem.sh status    # Übersicht
```

## Aufbau

| Workspace | Crates | Programme | Rolle |
| --- | --- | --- | --- |
| Root (`Cargo.toml`) | 26 | `atc`, `atc-node`, `atc_wallet`, `atc-vm-runner`, `atc_cli` | Kette, Sprache, VM, Wallet, SDK |
| `kernel/kai-os` | 11 | `kai-os-cli`, `kai-os-node`, `kai-sync-node` | Kernel-Runtime, Node, Netzwerk, State |
| `kernel/globus-os` | 31 | `boot`, `shivacore`, `atc-linux-edition`, `atc-windows-edition` | OS-Schicht mit UEFI-Bootloader |
| `ai/genesis-engine` | 19 | `genesis` | Spiel-Engine (Renderer, Physik, ECS) |
| `tooling/atc-engineering` | 13 | `atc-audit-cli` | Gates, Findings, Verifikation |
| `components/zkp` | 7 | — | ZKP: Prover, Verifier, Circuits, VM |
| `apps/atc-marketplace` | 2 | — | Marktplatz |

### ShivaCore-Kernel (Bare Metal)

`shivacore` trägt `required-features = ["x86-boot"]` und wird von
`cargo build --workspace` bewusst übersprungen. Der Bau braucht ein eigenes
Ziel und `build-std`:

```bash
cd kernel/globus-os
cargo +nightly build -p shivacore --features x86-boot \
  --target x86_64-unknown-none -Z build-std=core,alloc
```

`./tools/ecosystem.sh build` führt diesen Schritt als achte Zeile aus und
schreibt `target/x86_64-unknown-none/release/shivacore` (ELF64, x86-64,
Einstieg `_start`, ~121 KB).

**Aufgelöst am 08.11.2026.** Der Bau brach zuvor in `sha2` und
`curve25519-dalek` ab (Debug: `rustc-LLVM ERROR: Do not know how to split the
result of this operator!`, Release: `SIGILL` im rustc). Ursache war keine
Toolchain-Grenze, sondern Ballast: beide Crates kamen ausschließlich über
`ed25519-dalek`, das der Kernel nirgends verwendete (`cargo check` meldete es
als ungenutzte Abhängigkeit, im Quelltext null Treffer). Nach dem Entfernen
baut der Kernel sauber. Signaturprüfung gehört ohnehin nicht in den Kernel,
sondern in Service-Space und Node.

Zwischenstand, falls der Codegen je wieder bricht: `cargo +nightly check`
mit demselben Ziel und `-Z build-std` prüft den Kernel in Sekunden ohne
LLVM-Codegen und trennt damit Typfehler von Toolchain-Fehlern.

### Kernel booten

```bash
./tools/ecosystem.sh boot          # baut ELF, erzeugt Images, bootet unter QEMU
ATC_BOOT_SECONDS=60 ./tools/ecosystem.sh boot   # längerer Lauf
```

Voraussetzungen sind `qemu-system-x86` und `ovmf`. Der Befehl baut den Kernel
falls nötig, erzeugt über `modules/atc-shivacore/boot` ein BIOS- und ein
UEFI-Image unter `/tmp/atc_boot/` und startet die UEFI-Variante auf `q35` mit
512 MB. Er gilt als bestanden, wenn der Kernel-Einstiegspunkt im seriellen
Protokoll (`/tmp/atc_boot_serial.log`) auftaucht.

Belegt am 08.11.2026: Einstiegspunkt erreicht, CPU- und ACPI-Erkennung
durchlaufen, Scheduler wechselt in 25 Sekunden 366-mal zwischen PID 1000 und
1001. Details in [`docs/boot/BOOT-EVIDENCE.md`](docs/boot/BOOT-EVIDENCE.md).

Der Kernel spricht über die serielle Schnittstelle, nicht in den Framebuffer —
ein Bildschirm-Screenshot zeigt nur den Hintergrund. Ein Lauf auf echter
Hardware ist nicht geprüft, nur QEMU mit TCG-Emulation.

Toolchains: alles baut auf stable. `kernel/globus-os` ist per
`rust-toolchain.toml` an Nightly gebunden, weil das Build-Skript des
UEFI-Bootloaders unstable Cargo-Flags (`-Z`) setzt. Die Datei gilt nur für
diesen Unterbaum; alle anderen Workspaces bleiben unberührt.

## Der laufende Verbund

`tools/ecosystem.sh smoke` startet einen Devnet-Node auf einem freien Port und
prüft jede Schicht gegen die nächste:

1. **Node** startet mit Genesis, Peer-Tabelle und Gossip-Dienst (RPC-Port plus 1).
2. **RPC** über den SDK-Client: `chain-id` → `atc`, `boot-hash`, `peers` → `2`.
3. **Sprache**: `atc check` validiert eine ATCLang-Quelle, `atc run` führt sie aus.
4. **VM**: `atc-vm-runner` führt eine Op-Kette aus und verlangt `ATVM PASS`.
5. **Wallet**: leitet den öffentlichen Schlüssel (secp256k1, komprimiert) ab.

Manuell, wenn der Node länger laufen soll:

```bash
./target/debug/atc-node 127.0.0.1:39471 &
./target/debug/atc_cli --addr 127.0.0.1:39471 chain-id
./target/debug/atc_cli --addr 127.0.0.1:39471 boot-hash
```

## Was zusammengeführt wurde

- **Ein Workspace als SSOT**: `atc_cli` bezog `atc-node` bisher über eine
  Git-Revision und erzeugte damit zwei Pakete gleichen Namens im Graph. Jetzt
  zeigt die Abhängigkeit auf `components/node`.
- **Eine Ketten-ID**: Der Node meldet die kanonische Zeichenkette `atc`. Der
  SDK-Client parste sie als `u64` und scheiterte an jeder echten Antwort; die
  numerische Domain-Separation `658467` bleibt den Signaturpfaden vorbehalten.
- **Ein Build**: `cargo check --workspace --all-targets` und
  `cargo test --workspace` laufen über alle 26 Root-Crates; die sechs
  Unter-Workspaces bauen über denselben Einstiegspunkt.

## Bekannte Grenzen

- **Devnet, kein Konsens**: Der Node hat Blockproduktion und Konsens nicht
  aktiviert (SCR-0117/0118, Devnet-MVP). Der Smoke prüft Erreichbarkeit und
  Datenpfade, nicht Finalität.
- **Boot-Hash noch 64 Bit**: `BOOT_HASH` liefert weiterhin einen `u64`, während
  Block-Hashes auf 256 Bit migriert sind (SEC-P0-007). Die Angleichung steht aus.
- **FV-002**: `modules/atc-blockchain/kernel` ist unvollständig (nur
  `blockchain.rs`, `alloc`/`mempool`/`consensus` fehlen) und bleibt mit
  `ecosystem/integration` aus dem Root-Workspace ausgenommen.
- **Kein TLS, keine Authentisierung** am Devnet-RPC — nur localhost.

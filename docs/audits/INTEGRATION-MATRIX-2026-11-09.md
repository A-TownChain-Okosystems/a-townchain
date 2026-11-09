# Integrationsmatrix — was ist wirklich miteinander verbunden?

**Status:** Bestandsaufnahme am Code, keine Änderung.
**Stand:** a-townchain `97a4a06`, globus-os `77e3c8c` (frische Klone, 09.11.2026).
**Methode:** Abhängigkeitsgraph aus allen 143 `Cargo.toml` (111 + 32 Crates),
aufgelöst über Paketnamen; danach Laufzeitkanten einzeln per Quelltextsuche geprüft.
Testcode zählt nicht als Produktkante.

---

## 1. Kurzantwort

**Nein.** Es gibt vier Universen, die einander nicht rufen: Compiler/VM, Kette,
Betriebssystem, KI. Innerhalb der Universen ist vieles auf Build-Ebene verbunden,
aber es existiert **keine einzige Kante, über die ein Produkt Bytecode ausführt,
`.atc` lädt oder die KI etwas startet**. Die Verbindungen sind Werkzeugketten und
Bibliotheksabhängigkeiten, keine Ausführungsketten.

---

## 2. Die vier Universen und ihre Inseln

| Universum | Crates | ohne eingehende Build-Kante |
|---|---|---|
| a-townchain (Compiler, VM, Kette, KI, Genesis, Werkzeuge) | 111 | **61** |
| globus-os (Kernel, Service Space, Systemdienste, Boot) | 32 | **11** |

Elf der elf globus-os-Inseln sind: `boot`, `shivacore-init`, `globus-runtime`,
`atc_drivers`, `globus-os-sdk`, `globus-settings`, `globus-virtualization`,
`atc-globus-shell`, `atc-linux-edition`, `atc-windows-edition`, `atc_bootloader`.
In a-townchain sind unter anderem alle `atc-aurora-*`, alle `atc-genesis-*`,
`atc-wallet`, `atc-exec-admission` und `atc-ecosystem-integration` ohne Zulieferer.

---

## 3. Komponentenmatrix

| Komponente | Build verbunden mit | Laufzeit-Kante | Befund |
|---|---|---|---|
| **ATCLang-Compiler** (`atc-core`) | 10 Werkzeug-Crates (`tooling/atc-engineering/*`) | keine | Compiler ist ein Werkzeug-Werkzeug, kein Produktbestandteil |
| **ATC-VM** (`atc-vm`) | **niemand** (0 von 143) | keine | vollständig, aber freistehend |
| **ATC-Node** | `atc-algorithm`, `atc-wallet` | keine Vertragsausführung | Kette prüft Blöcke/Transaktionen, führt keinen Bytecode aus |
| **ATC-Algorithm** | von `node`, `mining` genutzt | PoH im Konsens | verbunden |
| **ATC-Wallet** | **niemand** (Archivkopie auch in `node`-Deps) | Signieren im Node | zwei Wahrheiten |
| **Verträge** (`apps/atc-contracts`, `components/contracts`) | Registry-Gate + Werkzeuge | keine | Quellen und ABI-Hashes geprüft, nie ausgeführt |
| **Kernel** (`shivacore`) | 15 `system/*`-Crates + `service_space` über `libshivacore` | Boot | untereinander verbunden, nach außen dicht |
| **ABI** (`libshivacore`) | 9 Crates in beiden Repos | Syscall-Gate 0x80 | echte gemeinsame Schnittstelle |
| **Service Space** | `globus-runtime` (beide Repos) | — | wird gebaut, aber von keinem Startpfad geladen |
| **Systemdienste** (`vfs`, `net`, `devices`, `graphics`, `audio`, `package`, `update`, `identity`) | `globus-runtime` | — | nur über den ungenutzten Runtime erreicht |
| **Boot-Image** (`boot`) | Kernel + optionaler Ramdisk | CI baut `-p shivacore` und ruft `boot` **ohne Programm** | Image ohne Nutzlast |
| **Init** (`shivacore-init`) | `libshivacore` | — | Insel, wird von niemandem gestartet |
| **Treiber-Framework** (`atc-drivers`) | `globus-devices` | — | Insel, kein Nutzer |
| **KI in a-townchain** (`atc-aurora-core/-agents/-memory/-localization`, `sdk`) | nur untereinander | — | Inseln, keine Tool-Ausführung |
| **KI im OS** (`system/aurora-core`) | `globus-runtime` | — | 338 Zeilen Control-Plane-*Verträge*, keine Laufzeit |
| **Genesis** (`atc-genesis-*`, 6 Crates) | nur untereinander | — | Inseln, kein Exec-Pfad |
| **Admission** (`atc-exec-admission`) | **niemand** | — | erwartet: Kandidat ohne Durchsetzungspunkt |

---

## 4. Die fehlenden Kanten, jede einzeln belegt

1. **VM ← niemand.** Kein Crate beider Repos deklariert `atc-vm` als Abhängigkeit.
   Die VM ist fertig (EXEC-CHAIN 20/20) und schwebt frei.
2. **Compiler → nur Werkzeuge.** `atc-core` wird ausschließlich von zehn
   `tooling/atc-engineering`-Crates verwendet. Kein Loader, kein Node, kein Produkt.
3. **Kette → keine Ausführung.** `components/node` hängt an `atc-algorithm` und
   `atc-wallet`; Treffer für `atc_vm`, `Vm::` oder `execute_state_transition` in
   `node/src`: **null**.
4. **Kernel → keine ATCLang-Bindung.** Kernel-Deps sind `bootloader_api`,
   `libshivacore`, `x86_64`, `pic8259`, `uart_16550`, `spin`, `lazy_static`, `libm`.
   Das ist normativ so gewollt (SHIVA-KERNEL-REUSE: das OS darf ATCLang nicht im
   Kernelvertrag verlangen). Der Kernel ist korrekt dicht, nicht lückenhaft.
5. **OS-Userland → Boot.** Der Boot-Builder nimmt eine Ramdisk nur als Argument
   (`boot <kernel> <out> [ramdisk-elf]`); der CI-Workflow baut `-p shivacore` und
   ruft `boot` ohne Programm. Der Ramdisk-Pfad existiert und ist getestet, wird im
   Image aber nicht benutzt.
6. **KI → Ausführung.** `system/aurora-core` sind Verträge (contracts/errors/state/
   types), erreicht von `globus-runtime`, das selbst niemand startet. Die
   `atc-aurora-*`-Crates in a-townchain haben null Zulieferer. Es gibt keine
   Tool-Runtime und damit auch keinen Punkt, an dem eine Allowlist greifen könnte.
7. **Genesis → nichts.** Sechs Crates, keine eingehende Kante, kein Exec-Pfad.
8. **Admission → nichts.** `atc-exec-admission` hat null Nutzer, wie es der
   Kandidatenstatus verlangt.
9. **Kein `.atc` wird zur Laufzeit geladen.** Treffer für `include_str!`/`read` auf
   `.atc` außerhalb der Testmodule: null. Sprachquellen sind Eingaben für Werkzeuge,
   keine Betriebsmittel.

---

## 5. Zwei Kernel-Wahrheiten

`a-townchain/kernel/globus-os` ist ein Spiegel von `globus-os`. Er ist **keine
Kopie**: `kernel/src/main.rs` (`ec569ca5…` gegen `f86002ff…`) und
`kernel/src/capability.rs` (`66158d3a…` gegen `36400977…`) unterscheiden sich.
Zwei Kernel-Bäume, zwei Stände, ein Name.

---

## 6. Was tatsächlich verbunden ist

- **Kernel ↔ Systemdienste:** 15 Build-Kanten über `libshivacore`, dazu
  `service_space → kernel` (bestehende Richtung, aber zyklisch, weil der Kernel
  den Service Space nicht kennt und der Service Space den Kernel braucht).
- **Kette ↔ Krypto/Algorithmus:** `node → algorithm`, `node → wallet`.
- **SDK ↔ Kette:** `components/sdk → node`; `ecosystem/integration → node, sdk,
  atc-blockchain`.
- **Werkzeugkette:** Registry-Gate, ABI-Hashes, `atc`-CLI auf `atc-core`.

Das sind echte Kanten. Keine davon führt Code aus, den ATCLang erzeugt hat.

---

## 7. Reihenfolge, die daraus folgt

1. **VM an die Kette** — die kleinste echte Kante. Entweder die Kette führt
   Verträge aus oder sie sagt ausdrücklich, dass sie es nicht tut.
2. **Compiler an die VM** — Bytecode als Artefakt, das ein Loader prüft, statt als
   Datei, die niemand liest.
3. **Ramdisk ins CI-Image**, `shivacore-init` startet `globus-runtime` — erst damit
   ist das Betriebssystem mehr als ein Kernel mit Diensten daneben.
4. **Spiegel auflösen** — eine Kernel-Wahrheit statt zwei.
5. **KI an eine Tool-Grenze**, aber erst nach der Admission-Autorität
   (ATC-SEC-EXEC-001). Sonst baut man den Angriffsweg vor dem Tor.

---

## 8. Grenzen dieser Prüfung

Der Build-Graph sagt, was kompiliert und verlinkt wird. Laufzeitkanten sind per
Quelltextsuche belegt, nicht per Trace eines laufenden Systems; ein dynamisch
geladenes Modul oder ein Aufruf über einen String wäre damit nicht erfasst.
QEMU-Bootläufe zeigen: der Kernel bootet und startet Ring-3-Code — aber nur
eingebauten, nicht aus der Ramdisk des Images.

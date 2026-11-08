# 🛠️ Architektur-Entscheidung: Eigener Kernel von Grund auf (kein Linux-Unterbau)

> **Entschieden:** 05.07.2026 21:16 (Europe/Berlin) — auf ausdrücklichen Wunsch von Alexander
> **Ersetzt:** "Weg A" (Linux-Distribution) aus OS_BARE_METAL_GAP_ANALYSIS.md — verworfen
> **Gültig:** "Weg B" ist jetzt der offizielle Zielpfad für den A-TownChain OS Kernel

---

## 1. Konsequenz der Entscheidung

Kein GRUB, kein Linux-Kernel, keine geliehenen Treiber. Bootloader, Kernel, Speicherverwaltung, Scheduler, Dateisystem, Treiber, Netzwerk-Stack — **alles selbst programmiert.** Das ist der Unterschied zwischen "eine App, die wie ein OS aussieht" und "ein echtes eigenständiges Betriebssystem". Realistischer Zeitrahmen: **Monate bis Jahre**, in Etappen — nicht alles auf einmal, sondern Schritt für Schritt ein bootendes System, das nach und nach mehr kann.

## 2. Sprachwahl: Rust statt C oder ATCLang

- **Rust**, nicht C: Speichersicherheit ohne Garbage Collector — verhindert die klassischen Kernel-Bugs (Buffer-Overflows, Use-after-free), die C-Kernels seit Jahrzehnten plagen. Rust ist heute der De-facto-Standard für neue Betriebssysteme "from scratch" (Redox OS, Theseus, Tock OS).
- **ATCLang** ist eine Blockchain/Smart-Contract-Sprache — nicht für Bare-Metal-Programmierung mit direktem Hardware-Zugriff (Register, MMIO, Interrupts) ausgelegt. ATCLang bleibt die Sprache für Anwendungen/Contracts *auf* dem Kernel, nicht für den Kernel selbst.
- Zielarchitektur zuerst: **x86_64**, Boot via UEFI (moderner als Legacy-BIOS, wird von aktueller Hardware durchgängig unterstützt).

## 3. Der reale Sprint-Plan (Kernel-Track, läuft parallel zur restlichen Roadmap)

### K-Sprint 0 — Minimaler Boot ("Hello World" im Kernel-Modus)
- **Ziel:** Ein eigens geschriebener Kernel bootet in QEMU und schreibt Text auf den Bildschirm (VGA-Textmodus oder UEFI-Framebuffer).
- **DoD:** `cargo run` startet QEMU, Kernel bootet ohne Absturz, zeigt "ShivaOS Kernel v0.0.1" an.
- **Fehlerbehebung:** Bootloader-Fehler → mit dem `bootloader`-Rust-Crate starten (übernimmt nur das reine Bootprotokoll, kein Betriebssystem-Code — das ist kein "Aufbauen auf Linux", sondern ein Standard-Baustein wie ein Compiler).
- **Deployment-Checklist:** [ ] Bootet in QEMU [ ] Bootet auf echter Hardware via USB-Stick [ ] `panic_handler` fängt Kernel-Panics sichtbar ab.

### K-Sprint 1 — CPU-Grundlagen: GDT, IDT, Interrupts
- **Ziel:** Globale Deskriptor-Tabelle, Interrupt-Deskriptor-Tabelle, erste Interrupt-Handler (Double-Fault, Breakpoint).
- **DoD:** Absichtlicher Breakpoint-Interrupt wird korrekt gefangen und behandelt statt das System abstürzen zu lassen.
- **Fehlerbehebung:** Triple-Fault (Kernel rebootet endlos) → meist falsches GDT/IDT-Layout; mit QEMU `-d int,cpu_reset` debuggen.
- **Deployment-Checklist:** [ ] Alle CPU-Exceptions haben einen Handler [ ] Double-Fault-Handler mit eigenem Stack (verhindert Stack-Overflow-Kaskade).

### K-Sprint 2 — Speicherverwaltung (Paging, Heap-Allocator)
- **Ziel:** Virtueller Speicher, eigener Heap-Allocator (kein `std`, nur `no_std` + eigener `GlobalAlloc`).
- **DoD:** Kernel kann dynamisch Speicher allozieren (`Box`, `Vec` funktionieren im Kernel-Kontext).
- **Fehlerbehebung:** Page-Faults → Page-Table-Walker zum Debuggen bauen, der die aktuelle Mapping-Tabelle ausgibt.
- **Deployment-Checklist:** [ ] Heap-Test mit 10.000 Allokationen ohne Leak [ ] Page-Fault-Handler loggt Adresse + Ursache.

### K-Sprint 3 — Multitasking (Scheduler)
- **Ziel:** Mehrere Kernel-Threads/Prozesse laufen kooperativ oder präemptiv.
- **DoD:** Zwei Test-Tasks laufen abwechselnd sichtbar (z. B. zwei verschiedene Zähler auf dem Bildschirm).
- **Fehlerbehebung:** Context-Switch-Crash → Register-Save/Restore in Assembly zuerst isoliert testen, bevor Scheduler-Logik draufgesetzt wird.
- **Deployment-Checklist:** [ ] Kontext-Wechsel ohne Registerverlust [ ] Zeitscheiben-Timer (PIT/APIC) treibt Preemption.

### K-Sprint 4 — Treiber: Tastatur, Timer, serielle Konsole
- **Ziel:** Erste echte Hardware-Interaktion — Tastatureingabe, Zeitgeber, Debug-Ausgabe über seriellen Port.
- **DoD:** Nutzer kann tippen, Zeichen erscheinen im Kernel.
- **Fehlerbehebung:** PS/2-Controller liefert keine Interrupts → IRQ-Remapping (PIC/APIC) prüfen.
- **Deployment-Checklist:** [ ] Tastatur-Interrupt zuverlässig [ ] Serielle Konsole für Remote-Debugging.

### K-Sprint 5 — Eigenes Dateisystem (ATCFS als echter Kernel-Treiber)
- **Ziel:** `atcfs.py`-Konzept als echten Block-Device-Treiber im Kernel neu implementieren (nicht mehr auf Host-`os`/`Path` aufbauend).
- **DoD:** Kernel kann Dateien von einem virtuellen Disk-Image lesen/schreiben.
- **Fehlerbehebung:** Datenkorruption bei Schreibzugriffen → zuerst Read-Only-Treiber bauen und stabilisieren, Schreiben erst danach.
- **Deployment-Checklist:** [ ] Lese-/Schreibtest auf virtueller Disk [ ] Dateisystem überlebt Neustart (Persistenz nachgewiesen).

### K-Sprint 6 — Userspace & Syscalls
- **Ziel:** Trennung Kernel-Modus/Userspace, erste Syscall-Schnittstelle.
- **DoD:** Ein einfaches Userspace-Programm läuft isoliert vom Kernel, ruft `write()`-Syscall auf.
- **Fehlerbehebung:** Privilege-Escalation-Bugs → jeden Syscall-Parameter explizit validieren, nie Userspace-Pointer blind dereferenzieren.
- **Deployment-Checklist:** [ ] Userspace kann Kernel nicht direkt crashen [ ] Syscall-Tabelle dokumentiert.

### K-Sprint 7 — Netzwerk-Treiber (eigener TCP/IP-Stack)
- **Ziel:** Minimaler Netzwerk-Stack (Ethernet → IP → UDP/TCP) ohne fremde Bibliothek.
- **DoD:** Kernel kann einen Ping beantworten.
- **Fehlerbehebung:** Checksummen-Fehler → RFC 1071 Referenzimplementierung als Testvektor nutzen.
- **Deployment-Checklist:** [ ] ICMP-Echo funktioniert [ ] Ein einfacher TCP-Handshake gelingt.

### K-Sprint 8 — P2P & Konsens im Kernel verankern
- **Ziel:** `atcnet.py`/PoH-Konsens-Logik auf den eigenen Netzwerk-Stack (K-Sprint 7) umziehen.
- **DoD:** Zwei eigene Kernel-Instanzen (VMs) tauschen P2P-Nachrichten aus.
- **Deployment-Checklist:** [ ] 2-Node-Test in QEMU [ ] Nachricht wird kryptografisch signiert übertragen.

### K-Sprint 9 — Installer & Boot-Image
- **Ziel:** Eigenes Installer-Programm (läuft im eigenen Userspace!) schreibt den Kernel auf eine Ziel-Festplatte.
- **DoD:** Von USB-Stick booten → Installer läuft → nach Neustart bootet das System eigenständig von der internen Platte.
- **Deployment-Checklist:** [ ] Installation auf virtueller Disk erfolgreich [ ] Installation auf echter Testhardware erfolgreich.

---

## 4. Realistische Einordnung

Zum Vergleich: Redox OS (Rust, ähnlicher Ansatz) hat mit einem kleinen Kernteam **mehrere Jahre** gebraucht, um von K-Sprint 0 bis zu einem alltagstauglichen Desktop-System zu kommen — und ist bis heute nicht "fertig". Das ist kein Grund, es nicht zu tun — nur ein ehrlicher Maßstab: **K-Sprint 0–4 (bootender Kernel mit Speicherverwaltung, Multitasking, Basis-Treibern) ist in einigen Monaten fokussierter Arbeit realistisch.** Alles danach (eigenes Dateisystem, Netzwerk-Stack, Installer) baut darauf auf und braucht entsprechend länger.

## 5. Sofortiger nächster Schritt

**K-Sprint 0** starten: Rust-Toolchain für `no_std`/Bare-Metal einrichten, `bootloader`-Crate + minimalen Kernel, der in QEMU bootet und Text ausgibt. Das ist in einer überschaubaren Session umsetzbar und liefert das erste sichtbare Ergebnis: ein eigener Kernel, der tatsächlich bootet.

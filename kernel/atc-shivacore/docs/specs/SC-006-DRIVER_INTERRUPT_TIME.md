---
document_id: SC-006
title: "ShivaCore v0.1 Kernelspezifikation — Driver / Interrupt / Time"
version: 0.1.0-FROZEN
status: FROZEN v0.1.0 — Owner-Freigabe 04.10.2026 (Überflug bestanden, kein Blocker; keine SC-DEC-Kandidaten, nichts bindet binär vor G7)
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems / ShivaCore (Michael Wroblewski)
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-04
ad_refs: [AD-012, AD-013, AD-026, AD-027, AD-028]
depends: [SC-001-FROZEN, SC-002-FROZEN, SC-003-FROZEN, SC-004-FROZEN, SC-005-FROZEN, SHIVA-HAL-001]
series: SC-001…SC-013 (v0.1.0-Kernelspezifikation, AD-013)
---

# SC-006 — Driver / Interrupt / Time (v0.1.0, FROZEN 04.10.2026)

> **Status:** FROZEN v0.1.0 per AD-013 — Owner-Freigabe 04.10.2026:
> Überflug bestanden, kein Blocker. Bestätigt: IRQ {SUBSCRIBE, ACK} mit
> Trampolin-in-Notification und ACK vor Re-Trigger; Timer als tickloser
> One-Shot-Deadline-Geber (SC-DEC-H), monoton via HAL, keine Wanduhr im
> Kernel; MMIO nur via Frame-Grant, DMA nur mit Frame-geprüften Puffern;
> Treiber als Service-Space-Services, keine Treiber-Registry im Kernel.
> Snapshot-Verdrahtung zu REQ-SC005-09a bestätigt. Per Standing-Mandat:
> keine SC-DEC-Kandidaten, nichts bindet binär vor G7 — Defaults behalten
> SC-ARCH-001…010-Review.

## 1. Zweck

Verbindliche Spezifikation der Kernel-Seite von Interrupts (IRQ-Caps),
Zeitverwaltung (Timer-Caps) und Gerätevermittlung (Device-Caps inkl.
MMIO/DMA-Autorisierung). Der Kernel implementiert KEINE Treiber; er stellt
die Cap-vermittelten Primitive bereit, über die Userspace-Treiber (Service
Space) Hardware steuern (AD-012 Klarstellung, AD-028 Kernel-Reinheit).

## 2. Interrupts (IRQ-Caps, REQ-SC006-01…04)

- **REQ-SC006-01 (MUST):** IRQ-Zugriff ausschließlich über IRQ-Cap
  {SUBSCRIBE, ACK} (Rechte-Menge SC-005 §3); ohne Cap kein Interrupt-
  Empfang — INV-01.
- **REQ-SC006-02 (MUST):** Der Kernel trampolint jeden IRQ in eine
  kernel-verwaltete Notification am Endpoint des Subskribenten (SC-003
  §2); es existiert KEIN User-Handler im Kernelkontext, kein Signal-Pfad.
- **REQ-SC006-03 (MUST) ACK-Pflicht:** Die IRQ-Line bleibt gemaskiert bis
  der Subskribent invoke-ACK auf der IRQ-Cap ausführt; kein Re-Trigger vor
  ACK (kein Interrupt-Sturm durch Verpassen).
- **REQ-SC006-04 (MUST):** Der IRQ-Pfad im Kernel ist minimal: kein
  Alloc/Retype, kein Logging im Hot Path, keine Kernel-Sperren jenseits
  der IRQ-Sperre (Determinismus, SC-002 DOM-Prioritäten).

## 3. Zeitverwaltung (Timer-Caps, REQ-SC006-05…07)

- **REQ-SC006-05 (MUST):** Timer-Cap {SET_DEADLINE, ACK}; der Timer-Manager
  ist One-Shot-Deadline-Geber — KEIN Tick als ABI oder Zeitsemantik
  (SC-DEC-H, wirksam ab SC-002 §6 / SC-003 §6).
- **REQ-SC006-06 (MUST):** Deadline-Queue ist sortiert (früheste zuerst),
  Ablauf trampolint als Notification an den Endpoint des Deadline-Besitzers;
  periodische Timer sind Service-Space-Wiederholung, kein Kernel-Loop.
- **REQ-SC006-07 (MUST):** Absolute Deadlines in Kernel-Zeitbasis; die
  Basis ist monoton und wird vom HAL geliefert (SHIVA-HAL-001); keine
  Wanduhr im Kernel (RTC/UTC = Service Space).

## 4. Geräte (Device-Caps, REQ-SC006-08…11)

- **REQ-SC006-08 (MUST):** Device-Cap {ACCESS} referenziert genau ein
  Gerät; MMIO-Regionen werden nur über Frame-Mapping per Device-Cap
  sichtbar gemacht (SC-001 Frames; Framebuffer-Präzedenz SC-DEC-F).
- **REQ-SC006-09 (MUST):** DMA ausschließlich über Device-Cap autorisiert
  (Data Plane, AD-013); der Kernel prüft Puffer über Frame-Caps des
  Initiators — kein DMA auf fremden Speicher ohne GRANT (SC-005 §3).
- **REQ-SC006-10 (MUST):** Userspace-Treiber sind Services: sie halten
  Device/IRQ/Timer-Caps und arbeiten über invoke (SC-004); der Kernel kennt
  keine Treiber-Registry, nur Cap-Bindungen (kein /dev, kein Binden am Pfad).
- **REQ-SC006-11 (MUST):** Geräte-Enumeration und -Namen sind Service-
  Space-Aufgabe (globus-init/Geräte-Service); der Kernel meldet nur
  vom HAL erkannte Geräte-Objekte als capable-Objekte an globus-init.

## 5. HAL-Anbindung (Randbedingung)

SHIVA-HAL-001 ist der Mechanik-Vertrag (Entry/Exit, IRQ-Delivery,
Zeitquelle). Erste Referenzplattform: x86 (APIC/LAPIC-Timer, QEMU-M5).
Plattformspezifik (I/O-Ports, MSI(X)) ist HAL-Implementationssache —
das Kernel-ABI (SC-004) bleibt plattformneutral; kein Plattform-Detail
leckt in die invoke-Opcodes (INV-06/SC-004-N).

## 6. Invarianten (MUST)

INV-01 Kein IRQ ohne IRQ-Cap; Delivery nur als Notification.
INV-02 Kein Re-Trigger vor ACK (Line maskiert).
INV-03 Timer nur One-Shot; keine periodischen Kernel-Timer.
INV-04 DMA nur über Device-Cap mit Frame-Cap-geprüften Puffern.
INV-05 MMIO nur via Frame-Grant aus Device-Cap.
INV-06 IRQ-Pfad minimal (kein Alloc, kein Retype, kein Lock jenseits IRQ).
INV-07 Kernel-Zeit ist monoton, HAL-geliefert, keine Wanduhr.
INV-08 Keine Treiber-Registry im Kernel — Geräte binden nur über Caps.

## 7. Fehlerklassen (MUST-behandelbar)

D-E01 IRQ-Zugriff ohne Cap → AUTH_DENIED (SC-004 Y-E01).
D-E02 ACK ohne abonnierte Line → UNSUPPORTED_OP, keine Zustandsänderung.
D-E03 Deadline in der Vergangenheit/ungültig → ARG-Fehler, kein Wrap.
D-E04 Timer-Queue voll → RESOURCE-Fehler (Default §10), kein stilles Verwerfen.
D-E05 DMA ohne Device-Cap oder ohne Puffer-GRANT → AUTH_DENIED +
      Diagnostic-Event (Sicherheitsrelevant, kein stiller Fallback).
D-E06 Unbekanntes Gerät/Slot → INVALID_CAP (SC-004 Y-E02).
D-E07 HAL-Zeitquelle liefert nicht monoton → Diagnostic-Event +
      Deadline-Queue friert neu ein (kein Kernel-Panic im Userkontext,
      SC-004 Y-E07).

## 8. Testbarkeit (MUST)

- T3 Unit: IRQ-ACK-Zyklen, Maskenlogik, Timer-Queue-Ordnung,
  Deadline-Vergleich, DMA-Rechteprüfung.
- T4 Integration: IRQ→Notification→Service-Roundtrip; Timer-Wake mit
  SC-002-Domains (HARDRT-Admission); Frame-Grant/Revoke während DMA
  (SC-005 Snapshot-Semantik REQ-SC005-09a).
- T1 QEMU (M5): Ticklosigkeit (kein Idle-Burn), erster IRQ-Roundtrip
  (Tastatur/Timer), Monotonie über Boot.
- Determinismus: identische IRQ/Timer-Sequenzen ⇒ identische
  Notification-Ordnung je Endpoint.

## 9. Abgrenzung

- Kernel-Interim: interrupts.rs, timers.rs, devices.rs (K-Sprints) —
  Dispatch-Rahmen bleibt Referenz; Umstellung auf Cap-Semantik nach
  SC-001…SC-013 (AD-026).
- KEIN Treiber-Framework, kein Gerätetreiber-Bestand im Kernel (AD-028);
  konkrete Treiber (NIC, Storage, GPU) sind Service-Space-Sprints.
- Kein Power-Management-ABI (später, eigener Spez-Bedarf falls M5+).

## 10. Defaults (reversibel, Review bei SC-ARCH)

| Wert | Default | Ort |
|---|---|---|
| IRQ-Queue je Line | 32 Events | §2 |
| Timer-Deadlines je Cap | 1 (One-Shot) | §3 |
| Timer-Queue gesamt | 1024 | D-E04 |
| Notification-Bitmask | 64 Bit (≡ SC-003 §12) | §2, §3 |
| MMIO-Frame-Granularität | 4 KiB (≡ SC-001) | §4 |
| Kernel-Zeitbasis | HAL-monoton, 64 Bit | §3 |

## 11. Schnittstellen

- SC-001: Frames/Retype für MMIO-Mappings; Initial-Caps (globus-init
  erhält IRQ/Timer/Device-Caps, SC-005 §7).
- SC-002: Timer-Wake als Umschaltpunkt; DOM-Prioritäten im IRQ-Pfad.
- SC-003: Notifications als einziges IRQ/Timer-Delivery-Medium; I-E01…E07.
- SC-004: invoke/ACK-OPs; Fehlerklassen Y-E01…E07; Snapshot-Semantik.
- SC-005: Rechte-Mengen (IRQ/Timer/Device), Revocation, Badge-Provenanz
  für Geräte-Identität.
- SC-007 (folgend): AddressSpace/Mapping-Vertiefung oder VFS-Grenzfall
  nach AD-013-Reihenfolge.

## 12. Referenzen

AD-012 (Kernel nur Primitive), AD-013 (Device-Caps, DMA Data Plane,
globus-init), AD-026 (Reihenfolge), AD-027 (M2/M5), AD-028 (Kernel-
Reinheit, Service-Space-Treiber), SC-001…SC-005 (alle FROZEN),
SHIVA-HAL-001, SHIVA-BOOT-001 (Limine/Initial Task), SHIVA-GLOBUS-
INTEGRATION-001, kernel/src/interrupts.rs · timers.rs · devices.rs
(Interim), docs/specs/SC-001…SC-005.

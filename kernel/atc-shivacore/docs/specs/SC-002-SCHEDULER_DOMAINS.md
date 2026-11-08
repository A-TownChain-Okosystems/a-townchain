---
document_id: SC-002
title: "ShivaCore v0.1 Kernelspezifikation — Scheduler & Scheduling-Domains"
version: 0.1.0-FROZEN
status: FROZEN v0.1.0 — Owner-Freigabe 03.10.2026 (SC-DEC-G…J akzeptiert per Vorab-Regel mit Auflagen; Review bei SC-ARCH-001…010)
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems / ShivaCore (Michael Wroblewski)
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-03
ad_refs: [AD-012, AD-013, AD-026, AD-027, AD-028]
depends: [SC-001-FROZEN, SHIVA-ABI-001, SHIVA-HAL-001]
series: SC-001…SC-013 (v0.1.0-Kernelspezifikation, AD-013)
---

# SC-002 — Scheduler & Scheduling-Domains (v0.1.0, FROZEN 03.10.2026)

> **Status:** FROZEN v0.1.0 per AD-013 — Owner-Freigabe 03.10.2026: SC-DEC-G…J
> alle vier reversibel (Impact ≤ mittel) und mit Auflagen akzeptiert (§13).
> Review bei SC-ARCH-001…010.
> Implementierungsstand: scheduler.rs der K-Sprints ist ein Grundstock
> (Round-Robin, keine Domains) — Implementation erfolgt GEGEN diese Spec.

## 1. Zweck

Verbindliche Spezifikation der Thread-Verwaltung, CPU-Vergabe und der
Scheduling-Domains. Der Scheduler ist Kernel-Primitive (AD-012); DA-HEFT
(KI-Beschleuniger-Scheduling) bleibt separat im Service Space (AD-013)
und berührt den Kern ausschließlich über die Kernel-Event-Bridge.

## 2. Objektmodell (AD-013)

- **Thread**: Kernel-Objekt mit State RUNNING/READY/BLOCKED/SUSPENDED;
  Stack (aus Frames, SC-001), Priority NUR innerhalb seiner Domain,
  optionaler Timer (Watchdog).
- **Notification**: Signal-Objekt (SC-001 Initial-Caps); weckt Threads.
- **Timer**: Capability-vermittelt, absolute Alarme; keine freie
  Wanduhr-Nutzung in Contracts (Determinismus, ATC-STD-PoH-Referenz).
- **Endpoint-Interaktion**: Thread blockiert deterministisch auf
  send/recv (Details SC-003 IPC, folgt).

## 3. Scheduling-Domains (normativ, REQ-SC002-01…05)

| Domain | Zweck | Verdrängt | Zeitscheibe |
|---|---|---|---|
| DOM-HARDRT | harte Deadlines, Admission-Pflicht | alle anderen | WCET-Budget |
| DOM-SOFTRT | weiche Deadlines (A/V, Kernel-Services) | LATENCY abwärts | kurz, präemptiv |
| DOM-LATENCY | latenzsensibel (IPC-Forwarding, Input) | INTERACTIVE abwärts | kurz |
| DOM-INTERACTIVE | UI/Aurora-Interaktion | THROUGHPUT abwärts | mittel |
| DOM-THROUGHPUT | Batch/Validierung | BACKGROUND | lang |
| DOM-BACKGROUND | Wartung, Telemetrie | niemanden | lang, preemptierbar |

- **REQ-SC002-01 (MUST):** Domains verdrängen streng geordnet nach oben;
  innerhalb einer Domain entscheidet Priorität + FIFO-Alterung.
- **REQ-SC002-02 (MUST):** DOM-HARDRT-Threads erfordern bei Erzeugung eine
  WCET-Deklaration; die Admission-Prüfung verweigert Überlastung hart
  (kein stiller Degrad).
- **REQ-SC002-03 (MUST):** Kein Domain-Wechsel zur Laufzeit ohne
  Capability-gedeckte Neu-Erzeugung (kein renice).
- **REQ-SC002-04 (MUST):** Alle Umschaltungen erfolgen an definierten
  Punkten: Syscall-Rückkehr, IRQ-Trampolin, Timer-Alarm, Blockieren.
- **REQ-SC002-05 (MUST):** POSIX-frei (J-K10): keine nice-Werte, keine
  Unix-Signale, kein SIGHUP-Tasking; OS-Semantik nur über Notification.

## 4. SMP (per SC-DEC-E, gegated)

- Single-Kernel-Image, per-CPU-Runqueues (DOM-Ordnung je CPU).
- Initial Task weckt CPUs per CPU-Notification (SC-001 §5).
- Affinity = Capability an Thread-CPU-Bindung; Migration nur mit
  Migration-Cap.
- Keine globale Runqueue-Lock-Skala über 64 CPUs (v0.1: max 8 CPUs).

## 5. Zeit & Timer (REQ-SC002-06…07)

- **REQ-SC002-06 (MUST):** Timer ausschließlich über Timer-Caps; Kernel
  garantiert monotone Boot-Zeit (TSC-abgeleitet, HAL, SHIVA-HAL-001).
- **REQ-SC002-07 (MUST):** Keine Wanduhr im Kernel-Entscheidungspfad
  (Determinismus; Wanduhr = Service, AD-012/028).

## 6. DA-HEFT-Abgrenzung (normativ)

DA-HEFT (KI-Beschleuniger-HEFT) ist KEIN Kernel-Scheduler. Er läuft als
Service, erhält Beschleuniger-Zugriff ausschließlich über Device-Caps und
beantragt Kernel-Slots nur über die Domain-Admission. Verstöße = INV-10.

## 7. Schnittstellen

- SC-001: Kernel-Stacks (Frames), SMP-Aufwecken, Initial-Cap-Umfang.
- SC-003 (IPC, folgt): Blockieren auf Endpoints; Unblock-Priorität.
- SHIVA-HAL-001: Timer-Interrupt, IRQ-Trampoline, IPI-Mechanik.
- Aurora (L2, später): Kernel-Event-Bridge nur aus DOM-INTERACTIVE.

## 8. Invarianten (MUST)

INV-01 Jeder Thread gehört exakt einer Domain an (Erzeugungs-Cap).
INV-02 Kein Ready-Thread ohne Runqueue-Mitgliedschaft.
INV-03 HardRT-Auslastung je CPU ≤ 100 % nach Admission.
INV-04 Kein Kernel-Pfad blockiert länger als eine Zeitscheibe.
INV-05 Timer-Monotonie (kein Zurücksetzen der Boot-Zeit).
INV-06 Domain-Ordnung verletzt nie (HARDRT vor SOFTRT vor …).
INV-07 DA-HEFT-Code existiert nicht im Kernel-Crate (AD-028).
INV-08 Migration nur mit Migration-Cap.
INV-09 CPU-Start nur durch Initial-Task-Gate (SC-DEC-E).
INV-10 IRQ-Behandlung endet immer in definiertem Umschaltpunkt.

## 9. Fehlerfälle (MUST-behandelbar)

S-E01 HardRT-Admission verweigert → Fehlercode an Erzeuger, kein Boot-Einfluss.
S-E02 Deadline-Miss (HARDRT) → HARDCUT (SC-DEC-J, §13); andere
      Domains: Diagnostic-Event + kontrollierte Degradation, kein stilles
      Weiterlaufen.
S-E03 Starvation in DOM-BACKGROUND → Mindest-Budget je Slice-Fenster.
S-E04 Timer-Overflow (48-bit-TSC-Fenster) → HAL-Rollover-Handling.
S-E05 SMP-Race auf Runqueue-Migration → Cap-Gate + Test-T7.

## 10. Testbarkeit (MUST)

- T3 Unit: Domain-Auswahl, Verdrängungsordnung, Admission-Mathe.
- T4 Integration: Endpoint-Block/Unblock, Timer-Alarm, SMP-Aufwecken.
- T1 QEMU (M5): 2-CPU-Boot, Domains unter Last, Deadline-Haltebeweis
  für einen Synthetic-HardRT-Thread.
- T7 Stresstest: Migrationen, Timer-Sturm, IRQ-Last.

## 11. Meilenstein-Zuordnung (AD-027)

M2: T3/T4 grün (Scheduler-Teil von L0-L10 abgelöst). M5: T1 grün.
Kein M5-Gate ohne SMP-T1-Evidenz.

## 12. Verhältnis zum heutigen scheduler.rs

scheduler.rs (K-Sprint) = Round-Robin über Bereitschaftsliste ohne
Domains. Diese Spec ersetzt das Modell nicht destruktiv: Der bestehende
Bestand läuft als DOM-THROUGHPUT-Interim weiter, bis die Domain-Engine
nach SC-001…SC-013 gegen die Specs implementiert wird (AD-026-Reihenfolge:
erst Specs, dann Implementierung).

## 13. Entscheidungsprotokoll SC-DEC-G…J (Owner-Freigabe 03.10.2026, mit Auflagen)

| ID | Frage | Freigabe | Auflage / Default | Reversibel | Impact |
|---|---|---|---|---|---|
| G | Prioritäts-Inversion bei IPC | **ja** | Bounded PI, nur für blockierende IPC/Endpoint; keine unbegrenzte Vererbungskette (Implementierungs-Details in SC-003 §5: Tiefenlimit, Zyklus-Erkennung) | ja | mittel |
| H | Tick-Modell | **tickless** | Timer-Manager als One-Shot-Deadline-Geber; kein Tick als ABI oder Zeitsemantik | ja | mittel |
| I | Quanten-Werte | **Range 250 µs–10 ms** | Defaults: HARDRT 250 µs, Normal 1 ms, Background/Idle 10 ms; pro Domain konfigurierbar | ja | klein |
| J | Deadline-Miss-Politik | **HARDCUT für HARDRT** | Andere Domains: Log + kontrollierte Degradation, kein stilles Weiterlaufen | ja | mittel |

> Freigabe-Modus: Owner-Vorab-Regel — reversible Entscheidungen (Impact ≤ mittel)
> gelten mit Empfehlung als akzeptiert ("Owner accepted recommendation, review at
> SC-ARCH"). Neue SC-DEC-K… nur noch bei irreversiblen Entscheidungen oder
> Impact "groß" (Owner-Anweisung 03.10.2026).

## 14. Referenzen

AD-012 (Scheduler als Primitive), AD-013 (Domains HardRT…Background,
DA-HEFT separat), AD-026 (Reihenfolge), AD-027 (M2/M5), AD-028
(Kernel-Reinheit), SC-001-FROZEN (Frames, SMP-Gate, Initial-Caps),
SHIVA-HAL-001, SHIVA-ABI-001, J-K09/J-K10 (POSIX-Freiheit), kernel/src/
scheduler.rs (Interim-Bestand), docs/specs/SC-001-BOOT_MEMORY.md.

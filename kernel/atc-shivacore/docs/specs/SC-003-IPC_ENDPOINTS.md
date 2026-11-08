---
document_id: SC-003
title: "ShivaCore v0.1 Kernelspezifikation — IPC & Endpoints"
version: 0.1.0-FROZEN
status: FROZEN v0.1.0 — Owner-Freigabe 04.10.2026 (Default-only-Check bestätigt: keine versteckten ABI-Bindungen; Register-Transfer ≡ SC-004 §12)
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems / ShivaCore (Michael Wroblewski)
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-03
ad_refs: [AD-012, AD-013, AD-026, AD-027, AD-028]
depends: [SC-001-FROZEN, SC-002-FROZEN, SHIVA-ABI-001]
series: SC-001…SC-013 (v0.1.0-Kernelspezifikation, AD-013)
---

# SC-003 — IPC & Endpoints (v0.1.0, FROZEN 04.10.2026)

> **Status:** FROZEN v0.1.0 per AD-013 — Owner-Freigabe 04.10.2026. Default-
> only-Check (live): Register-Transfer-Wert identisch mit SC-004 §12 (FROZEN),
> Register-Layouts per Forward-Ref auf G7 verwiesen, keine Nummern-/Opcode-
> Festlegungen enthalten. SC-DEC-N (Fehler-/Restart-Semantik, SC-004 §5)
> ist ab dieser Spezifikation wirksam.

## 1. Zweck

Verbindliche Spezifikation des Endpoint-Objekts, der synchronen und
asynchronen Nachrichtenübertragung, der Blockierungssemantik und der
Unblock-Priorität. IPC ersetzt Signale, Pipes und Unix-Domains (J-K10):
alle Interprozeskommunikation läuft über Capability-gedeckte Endpoints.

## 2. Objektmodell (REQ-SC003-01…03)

- **REQ-SC003-01 (MUST) Endpoint:** Kernel-Objekt mit Rechte-Menge
  {SEND, RECV, DELEGATE, NOTIFY}; Rechteprüfung IMMER im Kernel
  ("Trust the capability, not the sender").
- **REQ-SC003-02 (MUST) Notification:** asynchroner, zustandszählender
  Signal-Trigger (kein Datenpayload; Wort-basiertes Bitmask-Feld).
- **REQ-SC003-03 (MUST) Badge:** kernel-vergebene, unveränderliche
  Absender-Kennung je Endpoint-Bindung; Fälschen ausgeschlossen.

## 3. Nachrichtenklassen (REQ-SC003-04…06)

- **REQ-SC003-04 (MUST) Control Plane:** synchrone Kurz-Nachrichten über
  Register-Transfer (Default: max 4 Maschinenwörter); blockierend,
  deterministisch, geordnet pro Endpoint.
- **REQ-SC003-05 (MUST) Data Plane:** große Nutzdaten ausschließlich über
  Frame-Cap-Transfer (SC-001) bzw. SharedMem mit Capability-Autorisierung;
  DMA nur über Device-Caps (AD-013: Control vs Data Plane getrennt).
- **REQ-SC003-06 (MUST):** Nie Nutzdaten über den Control-Endpoint; nie
  Control-Nachrichten über Data-Plane-Mappings.

## 4. Send/Recv-Semantik (REQ-SC003-07…10)

- **REQ-SC003-07 (MUST):** send/recv blockieren deterministisch (SC-002);
  kein Polling-Pfad, keine Busy-Wait-Syscalls.
- **REQ-SC003-08 (MUST):** Einreihung in die Warteschlange ist atomar mit
  dem State-Wechsel BLOCKED; aufweckende Seite wird per Unblock-Priorität
  (§5) eingeplant.
- **REQ-SC003-09 (MUST):** Pro Endpoint FIFO; passive Endpoints (kein
  Listener) sind ein definierter Fehlerzustand (I-E01), kein Klemmen.
- **REQ-SC003-10 (MUST):** Reply-Endpunkt wird pro Nachricht vom Kernel
  generiert (Badge-gebunden); kein freies Reply auf fremde Requests.

## 5. Blockierung & Bounded Priority Inheritance (SC-DEC-G, umgesetzt)

PI gilt ausschließlich für blockierende IPC/Endpoint, mit harten Grenzen:

- PI-Kette nur entlang send/recv-Blockierungen (SC-002-Domains bleiben
  erhalten; Vererbung hebt Priorität INNERHALB der Domain-Ordnung an).
- **Tiefenlimit PI = 8** (Default); Überschreitung = Fehlerzustand I-E03.
- Zykluserkennung über die Blockkette; Zyklus = Deadlock-Fehlerzustand
  I-E04 (Diagnostic-Event, beteiligte Threads suspendiert).
- Keine PI über Notification (asynchron, blockiert nicht).

## 6. Zeitverhalten (SC-DEC-H, tickless)

- Alle IPC-Wartesituationen nutzen den Timer-Manager als One-Shot-Deadline-
  Geber (Timeout-Cap); es existiert KEIN Tick als ABI oder Zeitsemantik.
- IPC-Timeout ist ein definierter Fehlerzustand (I-E05), kein Retry im Kernel.
- Fehler-/Restart-Semantik gem. SC-004 §5 (SC-DEC-N): Statuswort 8-Bit-Klasse
  + 24-Bit-Detail; nur blockierende OPs INTERRUPTED mit RESTARTABLE/ABORTED
  — ab SC-003 wirksam.

## 7. Invarianten (MUST)

INV-01 Keine IPC ohne Endpoint-Capability (Kernel prüft bei jedem OP).
INV-02 Jede Nachricht hat genau eine Badge-Herkunft.
INV-03 Notification-Zustand geht nie verloren (Zähler monoton bis Ack).
INV-04 Blockieren ist atomar mit Einreihung (REQ-SC003-08).
INV-05 Data Plane nie über Control-Endpoint (REQ-SC003-06).
INV-06 PI-Kette ist endlich (Tiefenlimit), zyklenfrei, IPC-only.
INV-07 Reply-Endpunkte sind kernel-generiert und Badge-gebunden.
INV-08 Queue-Tiefe je Endpoint ist begrenzt (Default 16) und hart geprüft.

## 8. Fehlerfälle (MUST-behandelbar)

I-E01 Passiver Endpoint → Fehlercode an Sender, keine Zustandsänderung.
I-E02 Queue voll → blockierter Sender (Control Plane) bzw. Fehlercode
      (Option "bounded-buffer" am Endpoint).
I-E03 PI-Tiefenlimit überschritten → Diagnostic-Event + Abbruch der OP.
I-E04 PI-Zyklus (Deadlock) → Diagnostic-Event + Suspension der Kette.
I-E05 IPC-Timeout → Fehlercode, Thread kehrt kontrolliert zurück.
I-E06 Capability-Widerruf während Block → kontrolliertes Aufwecken mit
      Fehlercode (kein stiller Datenzugriff nach Revocation).
I-E07 Endpoint-Close mit Wartenden → alle Wartenden mit Fehlercode
      aufgeweckt, Badge-losen Reply-Pfade entwertet.

## 9. Testbarkeit (MUST)

- T3 Unit: Rechteprüfung, Badge-Unveränderlichkeit, FIFO, Queue-Grenzen.
- T4 Integration: Block/Unblock-Kopplung mit SC-002-Domains, PI-Kette
  (Tiefe 8, Zyklus), Timeout, Revocation-während-Block.
- T1 QEMU (M5): Ping-Pong über Endpoints zwischen Initial Task und erstem
  Service; Determinismus-Messung (Send-Rate ohne Jitter > Slice).
- T7 Stress: PI-Sturm, Queue-Overflow-Last, 1000 gleichzeitige Blockierte.

## 10. Schnittstellen

- SC-001: Frame-Caps für Data Plane; CNode für Endpoint-Ableitung.
- SC-002: Block/Unblock, Domain-Vererbung, Timer (One-Shot-Deadline).
- SC-004 (Syscall-Interface, folgt): IPC-OPs als objektorientierte Syscalls
  an die Endpoint-Cap; konkrete Register-Layouts nach G7 (ATC-ABI-001,
  DRAFT) — nur Forward-Ref, keine ABI-Annahmen hier.
- Aurora (L2, später): Kernel-Event-Bridge konsumiert Notifications.

## 11. Abgrenzung

Kein Netzwerk-Stack (Service Space, AD-028), kein VFS, keine Unix-Signale.
ATCLang-Bindung (Syscall-Semantik aus Sicht der Sprache) erst nach G7.
Bestehender Rust-Interim-Bestand (ipc.rs, K-Sprint 5: Channel-FIFO) läuft
als Referenz weiter; Implementierung gegen diese Spec folgt nach
SC-001…SC-013 (AD-026: erst Specs, dann Implementierung).

## 12. Defaults (reversibel, Review bei SC-ARCH — keine SC-DEC-Punkte)

| Wert | Default | Ort |
|---|---|---|
| Register-Transfer | 4 Maschinenwörter (≡ SC-004 §12, FROZEN) | §3 |
| PI-Tiefenlimit | 8 | §5 |
| Queue-Tiefe je Endpoint | 16 | INV-08 |
| Notification-Bitmask | 64 Bit | §2 |
| IPC-Timeout-Granularität | Timer-Cap-abhängig | §6 |

## 13. Referenzen

AD-012 (IPC als Primitive), AD-013 (Endpoint-Objekt, OS-Bus, Control/Data
Plane), AD-026 (Reihenfolge), AD-027 (M2/M5), AD-028 (Kernel-Reinheit),
SC-001-FROZEN, SC-002-FROZEN (inkl. SC-DEC-G/H-Umsetzung), SHIVA-ABI-001,
kernel/src/ipc.rs (Interim-Bestand), kernel/src/scheduler.rs (Interim),
docs/specs/SC-001-BOOT_MEMORY.md, docs/specs/SC-002-SCHEDULER_DOMAINS.md.

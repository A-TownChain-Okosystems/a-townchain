---
document_id: SC-008
title: "ShivaCore v0.1 Kernelspezifikation — Prozess- & Service-Erzeugung"
version: 0.1.0-FROZEN
status: FROZEN v0.1.0 — Owner-Freigabe 04.10.2026 (nach Nachzug von Anmerkung 1 und 2; Anmerkung 3 + Domain-Klarstellung mit eingefroren)
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems / ShivaCore (Michael Wroblewski)
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-04
ad_refs: [AD-012, AD-013, AD-026, AD-027, AD-028]
depends: [SC-001-FROZEN, SC-002-FROZEN, SC-003-FROZEN, SC-004-FROZEN, SC-005-FROZEN, SC-006-FROZEN, SC-007-FROZEN]
series: SC-001…SC-013 (v0.1.0-Kernelspezifikation, AD-013)
---

# SC-008 — Prozess- & Service-Erzeugung (v0.1.0, FROZEN 04.10.2026)

> **Status:** FROZEN v0.1.0 per AD-013 — Owner-Freigabe 04.10.2026.
> Nachgezogen vor Freeze: (1) REQ-SC008-07 Spawn als Sequenz atomarer
> Einzel-OPs mit EXPLIZITER Supervisor-Aufräum-Pflicht per Revocation —
> kein Kernel-Rollback-Zustand (AD-028); (2) REQ-SC008-10a Empfänger-
> unabhängiger Service-End: Untyped-Rückfluss im Derivationsbaum des
> Creators läuft IMMER, Notification-Verwurf blockiert nie. Mit eingefroren:
> (3) Loader-Frame-Grant-Wiring (SC-005 §3/SC-006/SC-007), (4) Domain-
> Bindung = Erzeugungszeit (CONFIG wirkt auf Erzeugung, nie auf laufende
> Threads). Kein fork/exec/wait/signal; Defaults mit SC-ARCH-Review.

## 1. Zweck

Verbindliche Spezifikation der Erzeugung, Verwaltung und Beendigung von
Threads, Thread-Bündeln (Services) und deren Cap-Initialausstattung.
Löst strukturell die Terminal-Suspend-Lücke aus SC-007 REQ-SC007-13a:
Bei der Service-Erzeugung verbleibt eine Reserve-Thread-Cap beim
Supervisor, sodass Fault-Resume immer möglich ist.

## 2. Objektmodell (REQ-SC008-01…03)

- **REQ-SC008-01 (MUST) Service:** Kernel-agnostisches Aggregat
  {Thread-Set, AddressSpace, CSpace} — der Kernel kennt KEINE Service-
  Registry; ein Service ist die Cap-Sicht seines Supervisors.
- **REQ-SC008-02 (MUST) Supervisor:** Erzeugender Kontext (globus-init
  oder beauftragter Service); hält per Erzeugungsprotokoll mindestens:
  Reserve-Thread-Cap {SUSPEND, RESUME}, Fault-Endpoint-Cap (SC-007 §6),
  Exit-Endpoint (§5).
- **REQ-SC008-03 (MUST) Thread:** unverändert SC-002; Thread-Cap-Rechte
  {SUSPEND, RESUME, CONFIG}; CONFIG = Erzeugung/Parametrisierung.

## 3. Erzeugungsprotokoll (REQ-SC008-04…07)

- **REQ-SC008-04 (MUST) Explizit:** spawn = Sequenz von Kernel-OPs:
  CSpace (CNode) anlegen, AddressSpace anlegen (SC-007), Thread erzeugen
  (SC-002), Caps per mint in den Ziel-CSpace übertragen (SC-005 §4:
  Rechte-Reduktion Pflicht), Fault-Endpoint binden (SC-007 §6). Kein
  implizites Erben von Väter-Caps, kein Copy-on-Create.
- **REQ-SC008-05 (MUST) Supervisor-Reserve:** Das Protokoll ERZWINGT die
  Hinterlegung der Reserve-Thread-Cap im Supervisor-CSpace vor dem ersten
  Start des Threads; ohne Reserve-Thread-Cap kein Start (AUTH_DENIED an
  den Erzeuger — vor Ausführung, atomar).
- **REQ-SC008-06 (MUST) Image-Laden:** KEIN Kernel-Exec: Das Laden von
  Code/Initial-Daten ist Service-Space-Aufgabe (globus-init/Loader-Service)
  über Frame-GRANT + Mapping (SC-007, W^X geprüft); der Kernel verifiziert
  nur Cap-Ketten, nie Dateiformate. Wiring-Point (G7-relevant, nicht
  bindend): Die Loader-Schreibrechte entstehen als Frame-GRANT-Mint aus
  dem Supervisor-CSpace innerhalb der Spawn-Sequenz (SC-005 §3,
  SC-006-Muster); das Mapping ins Ziel-AS erfolgt per AS-Cap {MAP} des
  Loaders (SC-007 §3), W^X geprüft (REQ-SC007-05).
- **REQ-SC008-07 (MUST) Spawn-Sequenz:** spawn ist eine SEQUENZ atomarer
  Einzel-OPs (je OP SC-004 INV-07), KEINE einzelne Kernel-Transaktion. Bei
  Teilerfolg (z. B. DERIVATION_LIMIT aus REQ-SC005-07a nach angelegtem
  CSpace/AS) läuft keine OP zurück: Die erzeugten Objekte bleiben als Caps
  im Supervisor-CSpace, und der Supervisor trägt die EXPLIZITE Aufräum-
  Pflicht per Revocation (SC-005 §5). Der Kernel hält keinen Spawn-
  Transaktions- oder Rollback-Zustand (AD-028, keine Registry). Sichtbar
  wird ein Service erst mit dem Thread-Start — der LETZTEN OP der
  Sequenz; kein Thread läuft ohne vollständige Initialausstattung (INV-04).

## 4. Betrieb & Fault-Verdrahtung (REQ-SC008-08…09)

- **REQ-SC008-08 (MUST):** Fault-Delivery wie SC-007 §6: Suspend +
  Notification an den Fault-Endpoint des Supervisors; Resume über die
  Reserve-Thread-Cap — der SC-007-Terminal-Fall (13a) ist durch
  REQ-SC008-05 strukturell ausgeschlossen (Reserve kann nur der
  Supervisor selbst revoken; das ist supervisor-Entscheidung, kein
  Kernel-Fehlerzustand).
- **REQ-SC008-09 (MUST) Domain-Bindung (Erzeugungszeit, NICHT Boot-
  Zeit):** Domain-Zuweisung erfolgt bei JEDER Thread-Erzeugung, auch bei
  Laufzeit-Spawns: Default = Erbe der Erzeuger-Domain; Abweichung nur per
  CONFIG am erzeugten Thread (SC-002 Thread-Cap-Recht). Ein Domain-Wechsel
  zur Laufzeit existiert nicht — nur Neu-Erzeugung (REQ-SC002-03).
  Kein Widerspruch zu SC-002: CONFIG wirkt auf die ERZEUGUNG, nie auf
  laufende Threads.

## 5. Beendigung (REQ-SC008-10…12)

- **REQ-SC008-10 (MUST) Exit:** Thread-Exit = letzter OP des Threads
  (explizit) oder unbehobener Terminal-Fault (SC-007 §6); der Kernel
  entwertet die Thread-Cap und triggert die Exit-Notification an den
  Supervisor-Endpoint. KEIN Zombie-Zustand, kein waitpid.
- **REQ-SC008-10a (MUST) Empfängerunabhängiger Service-End:** Der
  Lebensende-Pfad (Entwertung, Revocation, Untyped-Rückfluss im
  Derivationsbaum des Creators, SC-005 §4/§5) läuft UNABHÄNGIG davon ab,
  ob Supervisor oder Exit-Endpoint noch existieren oder empfangen.
  Fehlt der Empfänger (oder Queue voll, P-E05), wird die Notification
  verworfen (Diagnostic-Event) — sie blockiert NIE den Lebensende-Pfad.
  Verwaiste Rückflüsse existieren nicht: Untyped fließt im Derivationsbaum
  zurück, nicht an Empfänger-Events.
- **REQ-SC008-11 (MUST) Service-End:** Ist der letzte Thread des Aggregats
  beendet, endet der Service: CSpace wird per Revocation geleert (Frames
  kehren über Untyped zurück, SC-001 §4), Mappings fallen mit Revocation
  (SC-007 REQ-SC007-09). Der Supervisor hält allein das Lebensende-Protokoll.
- **REQ-SC008-12 (MUST) Kein Auto-Restart:** Der Kernel startet nichts neu;
  Restart ist Supervisor-Politik (Analogie SC-DEC-N: definierte Zustände,
  keine stillen Weiterläufe).

## 6. Invarianten (MUST)

INV-01 Jeder laufende Thread hat einen Supervisor mit Reserve-Thread-Cap
       (REQ-SC008-05) und Fault-Endpoint-Bindung (SC-007 §6).
INV-02 Erzeugung ist explizit: kein Cap wandert ohne mint in einen
       neuen CSpace; keine implizite Vererbung.
INV-03 Supervisor-Reserve ist vor Thread-Start hinterlegt (atomar).
INV-04 Kein Thread läuft ohne vollständige Initialausstattung; ein
       halbfertiger Service ist unsichtbar (kein Thread gestartet);
       Teilobjekte bleiben Supervisor-Caps mit Aufräum-Pflicht.
INV-05 Exit-Notification wird vor der Entwertung VERSANDT; die
       Entwertung läuft empfangsunabhängig ab (REQ-SC008-10a). Kein
       blockierendes Warten auf Empfänger; Verwurf = Diagnostic-Event.
INV-06 Frames kehren bei Service-End vollständig in Untyped zurück
       (kein Leck, SC-001/SC-005-Verdrahtung).
INV-07 Keine PIDs ohne Cap: Identifikation nur über Caps/Badges.
INV-08 Der Kernel kennt keine Service-Registry — nur Objekte und Caps.

## 7. Fehlerklassen (MUST-behandelbar)

P-E01 spawn ohne Reserve-Thread-Cap-Hinterlegung → AUTH_DENIED VOR Start,
       atomarer Abbruch mit Revocation des Teilzustands.
P-E02 Ungültige Ziel-CSpace/AS-Caps → INVALID_CAP (Y-E02), Abbruch.
P-E03 Mint-Kette überschreitet Derivationslimit → DERIVATION_LIMIT (C-E03),
       Abbruch mit Teilaufräumung.
P-E04 Fault-Endpoint fehlt bei Thread-Start → AUTH_DENIED (INV-01).
P-E05 Exit-Notification-Queue des Supervisors voll → Diagnostic-Event +
       Entwertung läuft trotzdem ab (Service-End blockiert nie auf Queue).
P-E06 Image-Mapping verletzt W^X (SC-007) → ARG-Fehler an Loader-Service,
       kein Start.
P-E07 Supervisor revoken die eigene Reserve → künftig Terminal-Suspend
       möglich (dokumentierte Supervisor-Entscheidung, Diagnostic-Event).

## 8. Testbarkeit (MUST)

- T3 Unit: Erzeugungs-Zustandsmaschine (Reserve-Atomarität), Exit-
  Entwertung, Revocation-Aufräumpfade.
- T4 Integration: voller Service-Lebenszyklus (spawn → fault → resume →
  exit → Untyped-Rückfluss) gegen SC-005/SC-007; Restart-Sturm beim
  Supervisor; P-E05-Queue-Overflow.
- T1 QEMU (M5): globus-init erzeugt ersten Userspace-Service end-to-end
  (ohne Loader: vorgegebene Frames), Exit-Roundtrip.
- Determinismus: identische spawn-Sequenzen ⇒ identische Cap-Layouts
  (Slot-Belegung deterministisch).

## 9. Abgrenzung

- Kein fork/exec/wait/kill/signal — AD-013/J-K10; „Prozess" ist Cap-
  Aggregat-Sicht, kein Kernel-Subsystem.
- Loader (ELF-los, ATCLang-Image-Format) ist Service Space; Format-
  Fragen G7/ATCLang-ABI (SC-013).
- Restart-Politiken, Health-Checks, Supervision-Trees = Service Space.
- threads.rs/process-Interim-Bestände bleiben Referenz (AD-026).

## 10. Defaults (reversibel, Review bei SC-ARCH)

| Wert | Default | Ort |
|---|---|---|
| Threads je Service-Aggregat | 256 | §2 |
| Reserve-Cap-Slots im Supervisor | 2 je Service | §3 |
| Exit-Notification-Queue | 16 | P-E05 |
| Spawn-Kontextgröße | 4 Maschinenwörter (≡ SC-003/004) | §3 |
| Fault-Endpoint-Bindung | 1 je Thread (SC-007 §6) | §4 |
| Slot-Layout bei Erzeugung | deterministisch, documented order | §8 |

## 11. Schnittstellen

- SC-001: Untyped als Quelle aller Objekte; Rückfluss bei Service-End.
- SC-002: Thread/Domain-Bindung; CONFIG; HARDCUT-Verhalten (SC-DEC-J)
  interagiert mit Exit (unbehobener HARDRT-Deadline-Miss = Terminal).
- SC-003: Exit-/Fault-Notifications über Endpoints; I-E01…E07.
- SC-004: spawn/exit als invoke-OPs; Statuswort (SC-DEC-N).
- SC-005: mint-Ketten, Derivationslimit, Revocation als Aufräumpfad.
- SC-006: Device/IRQ/Timer-Caps als Teil der Initialausstattung möglich.
- SC-007: AS-Erzeugung, Fault-Modell, REQ-SC007-13a-Auflösung (§4),
  W^X beim Image-Mapping.
- SC-009 (folgend, nach AD-013-Reihenfolge): Kernel-Diagnostics /
  Event-Bridge-Schnittstelle für Aurora (L2).

## 12. Referenzen

AD-012 (Process Core als Primitive), AD-013 (kein fork/exec, globus-init,
Capability-Abdeckung, Kernel-Reinheit), AD-026 (Reihenfolge), AD-027
(M2/M5), AD-028 (Service-Space), SC-001…SC-007 (alle FROZEN, inkl.
SC-DEC-A…N + REQ-SC007-13a), SHIVA-GLOBUS-INTEGRATION-001 (globus-init),
SHIVA-BOOT-001, kernel/src/threads.rs · process.rs (Interim),
docs/specs/SC-001…SC-007.

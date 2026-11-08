---
document_id: SC-007
title: "ShivaCore v0.1 Kernelspezifikation — AddressSpace & Memory Objects"
version: 0.1.0-FROZEN
status: FROZEN v0.1.0 — Owner-Freigabe 04.10.2026 (kein Blocker; drei Anmerkungen im Freeze nachgezogen)
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems / ShivaCore (Michael Wroblewski)
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-04
ad_refs: [AD-012, AD-013, AD-026, AD-027, AD-028]
depends: [SC-001-FROZEN, SC-002-FROZEN, SC-003-FROZEN, SC-004-FROZEN, SC-005-FROZEN, SC-006-FROZEN, SHIVA-HAL-001]
series: SC-001…SC-013 (v0.1.0-Kernelspezifikation, AD-013)
---

# SC-007 — AddressSpace & Memory Objects (v0.1.0, FROZEN 04.10.2026)

> **Status:** FROZEN v0.1.0 per AD-013 — Owner-Freigabe 04.10.2026, kein
> Blocker. Drei Owner-Anmerkungen im Freeze nachgezogen: (1) REQ-SC007-13a
> Terminal-Suspend bei fehlender Resume-fähiger Thread-Cap (dokumentierter
> Endzustand + Diagnostic-Event; strukturelle Abhilfe via SC-008-Supervisor-
> Reserve); (2) REQ-SC007-05 W^X als Kernel-Politik-Default, änderbar nur
> per SC-DEC/SC-ARCH — kein Service-Space-Laufzeitpfad; (3) REQ-SC007-09a
> Rechte-Reduktion nur via mint+revoke, Propagation hält INV-03 global.
> Kein mmap/brk/malloc-Semantik-Äquivalent (AD-013, J-K10). Defaults mit
> SC-ARCH-001…010-Review.

## 1. Zweck

Verbindliche Spezifikation virtueller Adressräume, der Seiten-tabellen als
Kernel-Objekte, der Mapping-/Unmapping-Semantik über Cap-Ketten und der
Behandlung von Speicherzugriffsfehlern. HugePages (SC-DEC-A) werden als
Erweiterungspfad spezifiziert, ohne aktuelle Anforderung.

## 2. Objektmodell (REQ-SC007-01…03)

- **REQ-SC007-01 (MUST) AddressSpace:** Kernel-Objekt mit diskretem
  VA-Raum; Rechte {MAP, UNMAP, SWITCH} (SC-005 §3). SWITCH = aktiver
  Raumwechsel eines Threads (nur auf eigene Thread-Cap-Paar, §5).
- **REQ-SC007-02 (MUST) PageTable:** Kernel-Objekt, ausschließlich vom
  Kernel verwaltet; User-seitige Schreibzugriffe auf Tabelleneinträge
  existieren NICHT — Abbildungen entstehen nur über map/unmap-OPs.
- **REQ-SC007-03 (MUST) Frame-Bindung:** Mapping verknüpft Frame-Cap
  (SC-001, GRANT-Recht) mit VA in einem AddressSpace; Rechte des Mappings
  ⊆ Rechte der Frame-Cap (Monotonie wie SC-005 INV-03).

## 3. Mapping-Semantik (REQ-SC007-04…08)

- **REQ-SC007-04 (MUST):** map erfordert AddressSpace-Cap {MAP} + Frame-Cap
  {GRANT}; ohne beide: AUTH_DENIED (Y-E01), keine Zustandsänderung.
- **REQ-SC007-05 (MUST) W^X:** Eine VA-Region ist nie gleichzeitig WRITE-
  und EXECUTE-mapped (RW+X verboten). Verstöße werden beim map-Versuch
  abgelehnt (ARG), nie nachträglich abgeschwächt. Status: Kernel-Politik-
  Default, änderbar NUR per späterem SC-DEC bzw. SC-ARCH-001…010-Review —
  KEIN Service-Space-Cap-Anforderungspfad zur Laufzeit.
- **REQ-SC007-06 (MUST) Aliasing:** Ein Frame darf mehrfach gemappt sein
  (auch in mehreren AddressSpaces); jedes Mapping trägt seine eigene
  Rechte-Reduktion. Alias-Konsistz ist Frame-Physik — der Kernel
  garantiert keine Cache-Kohärenz über Aliase hinaus (HAL-Garantie).
- **REQ-SC007-07 (MUST) Overlap:** Überlappende map-Anforderungen werden
  abgelehnt (ARG-Fehler, Detail = kollidierende Region); kein stilles
  Überschreiben bestehender Mappings.
- **REQ-SC007-08 (MUST):** unmap ist synchron inkl. TLB-Shootdown im
  betroffenen AddressSpace; danach löst jeder Zugriff auf die Region den
  definierten Fault-Pfad (§6) aus.

## 4. Revocation-Kopplung (Verdrahtung zu SC-005, REQ-SC007-09…10)

- **REQ-SC007-09 (MUST):** Frame-Revocation (SC-005 §5) unmappt den Frame
  in ALLEN AddressSpaces synchron (TLB-Shootdown je Raum); INV-04
  (kein Zombie-Zugriff) gilt global für Mappings.
- **REQ-SC007-09a (MUST) Rechte-Reduktion-Propagation (Verdrahtung zu
  SC-005 INV-03):** Rechte-Reduktion erfolgt nie in-place: der einzige Weg
  ist mint (kleinere Maske) + revoke (Original). Revocation unmappt das
  Original in ALLEN AddressSpaces (REQ-SC007-09); Mappings unter der
  abgeleiteten Cap tragen die reduzierte Maske ab Erzeugung. Damit hält
  JEDES Mapping dauerhaft Mapping-Rechte ⊆ Frame-Cap-Rechte (INV-02) —
  auch über nachgelagerte Reduktionen hinweg; kein Mapping kann mehr
  Rechte tragen als seine Frame-Cap.
- **REQ-SC007-10 (MUST) Abgrenzung zu INV-09:** Die Snapshot-Semantik
  (SC-005 REQ-SC005-09a/INV-09) gilt für slot-resolvierte Caps in
  LAUFENDEN SYSCALLS. Rohe Speicherzugriffe sind keine Syscalls: nach
  unmap + TLB-Shootdown faulten sie sofort. Kein Snapshot für rohen
  Speicher — die beiden Semantiken kollidieren nicht.

## 5. SWITCH & Thread-Anbindung (REQ-SC007-11)

- **REQ-SC007-11 (MUST):** AddressSpace-SWITCH nur auf eigene Thread-Cap
  {CONFIG} + Ziel-AS-Cap {SWITCH}; beim Thread-Start fest gebunden,
  Wechsel nur definiert am Umschaltpunkt (SC-002 §5). Kein spontaner
  Adressraumwechsel im IRQ-Pfad (SC-006 INV-06).

## 6. Fault-Modell (REQ-SC007-12…13)

- **REQ-SC007-12 (MUST):** Ein Page-Fault (Mapping fehlt / Rechte fehlen /
  W^X-Verstoß zur Laufzeit nie möglich, da beim map geprüft) suspendiert
  den Thread und trampolint als Notification an den Fault-Endpoint des
  AddressSpace-Owners (SC-003 §2); Resume über Thread-Cap {RESUME} nach
  Behebung. Kein Signal-/SIGSEGV-Äquivalent im Kernel.
- **REQ-SC007-13 (MUST):** Ohne konfigurierten Fault-Endpoint: Thread
  suspendiert + Diagnostic-Event (kein Kill im Kernel, kein User-Panic,
  SC-004 Y-E07-Analog).
- **REQ-SC007-13a (MUST) Terminal-Suspend:** Ist ein Fault-Endpoint
  vorhanden, aber keine Resume-fähige Thread-Cap erreichbar (verloren oder
  revoked), ist der Thread TERMINAL SUSPENDIERT — dokumentierter
  Endzustand + Diagnostic-Event; kein stiller Kill, kein kernel-seitiges
  Auto-Resume. Strukturelle Abhilfe liefert SC-008: bei Service-Erzeugung
  verbleibt eine Reserve-Thread-Cap beim Supervisor (dort §4).

## 7. HugePages-Erweiterungspfad (SC-DEC-A-Nachtrag, nicht bindend)

4 KiB bleibt Basiseinheit (SC-001-FROZEN, SC-DEC-A). HugePages (2 MiB /
1 GiB) sind ein zukünftiger GRANULE-Parameter am Retype/Frame — transparent
für alle Cap- und Mapping-Semantiken, implizit, ohne User-sichtbares ABI.
Aktuell KEINE Anforderung; Aktivierung erst mit Hardware-Nachweis (M5+)
und eigenem SC-ARCH-Review. Nichts hier bindet.

## 8. Invarianten (MUST)

INV-01 map nur mit AS-Cap {MAP} + Frame-Cap {GRANT} (beide geprüft).
INV-02 Mapping-Rechte ⊆ Frame-Cap-Rechte (Monotonie).
INV-03 W^X: nie WRITE+EXECUTE auf derselben VA-Region.
INV-04 Frame-Revocation unmappt überall, synchron, ohne Zombies.
INV-05 PageTables sind kernel-exklusiv; User schreiben nie Einträge.
INV-06 unmap inkl. TLB-Shootdown ist synchron abgeschlossen.
INV-07 VA-Räume sind diskret; kein globales VA-Fenster über Spaces.
INV-08 Frames entstehen ausschließlich aus Untyped (SC-001); Mapping
       schafft keinen Speicher, nur Sichtbarkeit.

## 9. Fehlerklassen (MUST-behandelbar)

M-E01 map ohne Rechte → AUTH_DENIED (Y-E01), keine Zustandsänderung.
M-E02 Ungültiger VA/Alignment → ARG-Fehler (Detail = Region).
M-E03 Frame ohne GRANT → AUTH_DENIED + Diagnostic-Event.
M-E04 Überlappendes map → ARG-Fehler (kein stilles Überschreiben).
M-E05 AddressSpace-Tabelle voll → RESOURCE-Fehler (kein automatisches
      Nach-allozieren fremder Untyped).
M-E06 Fault ohne Fault-Endpoint → Suspend + Diagnostic-Event (§6).
M-E07 TLB-Inkonsistenz (interner Diagnosefall) → Diagnostic-Event +
      Raum-Neuladen; nie User-sichtbares Fehlverhalten.

## 10. Testbarkeit (MUST)

- T3 Unit: map/unmap-Algebra, Rechte-Schnitte, W^X-Prüfung, Overlap-
  Erkennung, Fault-Zustandsmaschine.
- T4 Integration: Frame-Revocation → Unmap-Welle über mehrere Spaces
  (SC-005 §5); Fault→Notification→Resume-Roundtrip (SC-003); SWITCH am
  Umschaltpunkt (SC-002); MMIO-Mapping über Device-Cap (SC-006 §4).
- T1 QEMU (M5): Usermode-Paging aktiv, Fault-Roundtrip im Live-System,
  TLB-Shootdown-Korrektheit.
- Determinismus: identische map/unmap-Sequenzen ⇒ identische
  Tabellenzustände und identische Fault-Ordnung.

## 11. Defaults (reversibel, Review bei SC-ARCH)

| Wert | Default | Ort |
|---|---|---|
| VA-Breite | 48 Bit (x86-Referenz via HAL) | §2 |
| Mapping-Granularität | 4 KiB (≡ SC-001) | §3 |
| Fault-Queue je AddressSpace | 16 suspendierte Threads | §6 |
| TLB-Shootdown | synchron, je AddressSpace | §3 |
| PageTable-Allokation | lazy on demand aus Untyped | §2 |
| HugePages | NICHT aktiv (Erweiterungspfad §7) | §7 |

## 12. Abgrenzung

- memory.rs / vm-Interim-Bestände (K-Sprints) bleiben Referenz; Umstellung
  auf Cap-Semantik nach SC-001…SC-013 (AD-026).
- KEIN Heap/malloc/brk im Kernel-ABI: Speicher kommt aus Retype (SC-001)
  plus Mapping (hier) — Allocation-Politik ist Service-Space.
- COW, ASLR, Overcommit sind Service-Space-Politiken (globus-init/Services),
  keine Kernel-Semantik. Keystone(en) wie Copy-on-Write könnten später als
  SC-ARCH-Review-Punkt zurückkommen — hier nicht spezifiziert.

## 13. Schnittstellen

- SC-001: Untyped/Retype als einzige Frame-Quelle; Kernel-Stacks; 4-KiB-
  Basis (SC-DEC-A); Initial-AS von globus-init per Cap.
- SC-002: SWITCH/Resume an Umschaltpunkten; Fault-Suspend als
  definierter Thread-Zustand (kein eigenständiger DOM).
- SC-003: Fault-Delivery als Notification an Fault-Endpoint; I-E01…E07.
- SC-004: map/unmap/switch als invoke-OPs; Fehlerstatuswort (SC-DEC-N).
- SC-005: Rechte-Mengen (§3), Revocation §5, INV-09-Abgrenzung (§4).
- SC-006: MMIO-Frames über Device-Cap; DMA-Puffer-Frames (§4 dort).
- SC-008 (folgend): Prozess/Service-Erzeugung (Thread-Sets, Service-Spawn
  durch globus-init) nach AD-013-Reihenfolge.

## 14. Referenzen

AD-012 (Kernel-Primitive inkl. Virtual Memory), AD-013 (kein mmap/fork,
globus-init, Capability-Abdeckung), AD-026 (Reihenfolge), AD-027 (M2/M5),
AD-028 (Kernel-Reinheit), SC-001…SC-006 (alle FROZEN, inkl. SC-DEC-A/G/H/
K/L/M/N), SHIVA-HAL-001 (Paging/TLB-Mechanik, x86-Referenz), SHIVA-BOOT-001
(Initial-AS), kernel/src/memory.rs (Interim), docs/specs/SC-001…SC-006.

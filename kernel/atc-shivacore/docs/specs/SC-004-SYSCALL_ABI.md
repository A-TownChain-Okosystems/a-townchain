---
document_id: SC-004
title: "ShivaCore v0.1 Kernelspezifikation — Syscall-Interface (ABI)"
version: 0.1.0-FROZEN
status: FROZEN v0.1.0 — Owner-Freigabe 04.10.2026 (SC-DEC-K/L/M freigegeben, N als Default bestätigt und ab SC-003 wirksam)
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems / ShivaCore (Michael Wroblewski)
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-03
ad_refs: [AD-012, AD-013, AD-026, AD-027, AD-028]
depends: [SC-001-FROZEN, SC-002-FROZEN, SC-003-DRAFT_REVIEW, SHIVA-HAL-001, SHIVA-ABI-001]
series: SC-001…SC-013 (v0.1.0-Kernelspezifikation, AD-013)
---

# SC-004 — Syscall-Interface / ABI (v0.1.0, FROZEN 04.10.2026)

> **Status:** FROZEN v0.1.0 per AD-013 — Owner-Freigabe 04.10.2026: K/L/M
> freigegeben wie empfohlen, N explizit als Default bestätigt (ab SC-003
> wirksam). AUFLAGE (Owner, 04.10.): K/L/M sind nur bis zur ersten binären
> Bindung (G7) reversibel — danach ABI-Freeze; bis dahin keine Reserved-
> Number-Festlegung, nur Dokumentation der Reservierungsbereiche.

## 1. Zweck

Verbindliche Spezifikation des objektorientierten, Capability-geprüften
Syscall-ABI. Ein Syscall ist ein Methodenaufruf an ein Kernel-Objekt über
eine Capability — keine globale Operation. Explizit verboten (AD-013,
J-K10): fd-Semantik, fork/exec, mmap/ioctl-Semantik, errno-artige globale
Fehlerzustände, freie PIDs ohne Cap-Bezug.

## 2. Design-Prinzipien (REQ-SC004-01…05)

- **REQ-SC004-01 (MUST):** Genau EINE Einstiegsoperation `invoke`
  (plus `yield` als freiwilliger Umschalte-OP); jeder Zugriff läuft
  über Cap-Slot + Opcode am Zielobjekt.
- **REQ-SC004-02 (MUST):** Kernel prüft Capability und Rechte VOR jeder
  Operation; Zeiger sind reine Daten, autorisieren nie.
- **REQ-SC004-03 (MUST):** Keine globalen Kernel-Namespaces (keine
  fd-Tabelle, kein pid-Wait ohne Process-Cap, kein /dev-Pfad-Lookup).
- **REQ-SC004-04 (MUST):** Fehler sind Teil des Return-Werts (Klasse +
  Detail); kein prozess-globaler Fehlerspeicher.
- **REQ-SC004-05 (MUST):** Nicht-blockierende OPs sind atomar: vollständig
  ausgeführt oder ohne Zustandsänderung abgelehnt.

## 3. Entry/Exit & Register-Konvention (Detail: SC-DEC-L, §13)

- **Entry (`invoke`):** Cap-Slot-Index (Zielobjekt), Opcode (Methode je
  Objekttyp-Familie), bis 4 Argument-Wörter, Cap-Argumente ausschließlich
  als Slot-Indizes der aufrufenden CNode (Details §12).
- **Exit (`sysret`):** einheitlich für ALLE Pfade (Erfolg, Fehler,
  Unterbrechung): Resultat-Wörter + Statuswort (Klasse + Restart-Info,
  §5). Kein zweiter Rückkehrmechanismus, keine Sprünge in User-Handler.
- **Stack:** User-Stack bleibt unberührt; Kernel nutzt per-CPU-Kernel-Stack
  (SC-001); Trampolin ohne Red-Zone-Ausnutzung; Alignment 16.
- **Preemption:** Syscall-Exit ist definierter Umschaltpunkt (REQ-SC002-04);
  Verdrahtung mit SC-002 §5 über Timer-Alarm/IRQ-Trampolin.

## 4. Capability-Übergabe bei syscall/sysret (Detail: SC-DEC-M, §13)

- Rein: Slot-Indizes; der Kernel resolved selbst (CNode des Threads,
  Rechte-Menge, Badge gem. SC-003 §2). Nie rohe Kernel-Adressen.
- Rausschreiben neuer Caps (create/derive/reply): Kernel schreibt in den
  Ziel-Slot der Ziel-CNode des Aufrufers und gibt den Slot-Index im
  Resultat zurück (REQ-SC003-10: Reply-Endpunkte kernel-generiert).
- Bei sysret verwirft der Kernel alle transienten Zwischen-Caps der OP;
  nichts überschreitet die Grenze unbemerkt (INV-06).
- Revocation während laufender OP → I-E06-Pfad aus SC-003.

## 5. Fehler- & Restart-Semantik (Detail: SC-DEC-N, §13)

- Statuswort: Fehlerklasse (AUTH, ARG, RESOURCE, STATE, INTERRUPTED,
  TIMEOUT, ABI) + 24-Bit-Detail-Code (Details §12).
- Nur blockierende OPs (send/recv, timer-wait) können INTERRUPTED liefern;
  Restart-Info: RESTARTABLE (idempotent wiederholbar) oder ABORTED
  (Zustand vor OP restauriert). Nicht-blockierende OPs: atomar (§2).
- Timeout folgt SC-003 §6 (tickless, One-Shot-Deadline; kein Kernel-Retry).

## 6. Nummernraum & ABI-Stabilität (Detail: SC-DEC-K, §13)

- Freigegeben (SC-DEC-K, 04.10.): Opcode = 16-Bit je Objekttyp-Familie +
  ABI-Version im Dispatch (Major/Minor); Major-Mismatch = definierter Fehler
  ABI_MISMATCH, kein Fallback (INV-08). Nummern-Freeze erst mit erster
  binärer Bindung (ATCLang-Codegen, G7) — danach ABI-Freeze. Bis dahin:
  KEINE Reserved-Number-Festlegung, nur Dokumentation der
  Reservierungsbereiche (Owner-Auflage).

## 7. Anbindung SC-002 / SC-003 (normativ)

- **SC-002:** invoke/yield sind Umschaltpunkte; Zeitscheiben-Verbrauch
  wird je OP berechnet; DOM-Wechsel nie per Syscall, nur per Neu-Erzeugung
  (REQ-SC002-03). yield = freiwillige Abgabe ohne Fehlerwort.
- **SC-003:** send/recv/poll-frei: invoke-OPs auf Endpoint-Caps;
  Register-Transfer identisch 4 Wörter; Badge/Provenanz nur kernel-seitig.
- **Timer:** invoke auf Timer-Cap (Deadline setzen/ack), tickless (SC-DEC-H).

## 8. Abgrenzung zu syscall.rs (Interim, K-Sprints)

Trägt heute: Dispatch-Rahmen im Hosted-Modus, Fehlercode-Konventionen als
Draft, Trait-Anbindung an threads/signals/system/userspace. Wird ersetzt:
freie/namespace-loser OPs, jede fd-artige oder pid-globale Semantik,
globale Fehlerzustände. syscall.rs läuft als Referenz-Host weiter; die
Implementierung gegen diese Spec erfolgt nach SC-001…SC-013 (AD-026).

## 9. Invarianten (MUST)

INV-01 Kein Syscall ohne Ziel-Cap (`invoke` referenziert immer einen Slot).
INV-02 Zeiger-Anteile autorisieren nichts; Autorität nur über Caps.
INV-03 Jeder Exit läuft durch genau einen definierten Umschaltpunkt.
INV-04 Fehler ist Teil des Return-Werts; kein globales Fehlersubstrat.
INV-05 Kernel verändert den User-Stack nie (Kernel-Stack per CPU).
INV-06 Transiente Caps überleben sysret nie.
INV-07 Nicht-blockierende OPs sind atomar (alle-oder-nichts).
INV-08 ABI-Major wird bei jedem invoke geprüft; Mismatch = definierter
     Fehler, kein teilweise-interpretierter Dispatch.

## 10. Fehlerklassen (MUST-behandelbar)

Y-E01 Unzureichende Rechte → AUTH_DENIED, keine Zustandsänderung.
Y-E02 Ungültiger/leerer Slot → INVALID_CAP.
Y-E03 Opcode für Objekttyp unbekannt → UNSUPPORTED_OP.
Y-E04 ABI-Major-Mismatch → ABI_MISMATCH (Y-E04 schlägt jeden OP-Typ).
Y-E05 Blockierende OP unterbrochen (Preemption/Signal) → INTERRUPTED +
      Restart-Info gem. §5.
Y-E06 Timeout → TIMEOUT-Statuswort, Verweis I-E05 (SC-003).
Y-E07 Kernel-interner Fehler → nie User-sichtbar; Diagnostic-Event +
      kontrollierter Thread-Stop (kein PANIC im Userkontext).

## 11. Testbarkeit (MUST)

- T3 Unit: Dispatch-Tabelle, Rechteprüfung je Opcode, Fehlerklassen-Wort,
  Atomicität nicht-blockierender OPs.
- T4 Integration: invoke-Ping-Pong über Endpoint-Cap (SC-003), Unterbrechung
  blockierender OP ↔ Preemption (SC-002), Cap-Revocation während invoke.
- T1 QEMU (M5): Usermode-Wechsel + syscall-Round-Trip; ABI-Mismatch-Suite.
- Determinismus: identischer Zustand + identische Argumente ⇒ identische
  Register-Rückgabe (keine versteckten Seiteneffekte).

## 12. Defaults (reversibel, Review bei SC-ARCH — §13 ausgenommen)

| Wert | Default | Ort |
|---|---|---|
| Argument-Register | 4 Wörter | §3 |
| Resultat-Register | 2 Wörter | §3 |
| Statuswort | 8-Bit-Klasse + 24-Bit-Detail | §5 |
| Stack-Alignment | 16, Red-Zone verboten | §3 |
| Opcode-Breite | 16 Bit je Objekttyp-Familie | §6 |
| CNode-Slot-Adressraum | 2^14 Slots | §4 |
| yield-OP | fester Kern-OP-Bereich | §2 |

## 13. Entscheidungsprotokoll SC-DEC-K…N (Owner-Freigabe 04.10.2026)

| ID | Frage | Freigabe | Reversibel? | Impact |
|---|---|---|---|---|
| K | Syscall-Nummernraum & ABI-Stabilität | **FREIGEGEBEN** wie empfohlen: versionierter Nummernraum, Freeze erst mit G7-Bindung | nur bis G7 (dann ABI-Freeze) | groß (ABI-Bruch später) |
| L | Register-Konvention | **FREIGEGEBEN**: Cap-Slot, Opcode, 4 Args, 2 Resultate, Status; sysret einheitlich | nur bis G7 (dann ABI-Freeze) | groß (ATCLang-Codegen G7) |
| M | Capability-Übergabe bei syscall/sysret | **FREIGEGEBEN**: Slot-Indizes rein, Kernel-resolviert; neue Caps in Ziel-CNode-Slot; transiente Caps bei Exit verworfen | nur bis globus-init/Services binden | groß (Sicherheitsmodell-Kern) |
| N | Fehler-/Restart-Semantik | **ALS DEFAULT BESTÄTIGT** (wirksam ab SC-003): 8-Bit-Klasse + 24-Bit-Detail; nur blockierende OPs INTERRUPTED mit RESTARTABLE/ABORTED | ja | mittel (prägt SC-003-Verträge) |

> AUFLAGE (Owner, 04.10.): K/L/M sind nur bis zur ersten binären Bindung
> (G7) respektive bis globus-init/Services binden (M) reversibel — danach
> ABI-Freeze. N ist ab SC-003 wirksam. Nächster Spez: SC-005
> Capability-System-Vertiefung → SC-006 Driver/Interrupt/Time.

## 14. Referenzen

AD-012 (Syscall-ABI als Primitive), AD-013 (objektorientiert, kein
fd/fork/exec/mmap), AD-026 (Reihenfolge), AD-027 (M2/M5), AD-028
(Kernel-Reinheit), SC-001-FROZEN (Kernel-Stacks, CNode, Frames),
SC-002-FROZEN (Umschaltpunkte, yield, Domains, Timer), SC-003
(Endpoint-invoke, Badge, I-E01…E07), SHIVA-HAL-001 (Entry/Exit-Mechanik),
SHIVA-ABI-001 (Vorstufe, nur Forward-Ref), ATC-ABI-001 (G7-DRAFT,
SCR-0071), kernel/src/syscall.rs (Interim-Bestand), kernel/src/main.rs.

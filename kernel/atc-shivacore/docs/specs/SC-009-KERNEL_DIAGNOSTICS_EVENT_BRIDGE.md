---
document_id: SC-009
title: "ShivaCore v0.1 Kernelspezifikation — Kernel-Diagnostics & Event-Bridge"
version: 0.1.0-FROZEN
status: FROZEN v0.1.0 — Owner-Freigabe 04.10.2026 (Verdrängungsregel nachgezogen; Punkte 2/3 als nicht-bindende Notizen mit eingefroren)
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems / ShivaCore (Michael Wroblewski)
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-04
ad_refs: [AD-012, AD-013, AD-026, AD-027, AD-028]
depends: [SC-001-FROZEN, SC-002-FROZEN, SC-003-FROZEN, SC-004-FROZEN, SC-005-FROZEN, SC-006-FROZEN, SC-007-FROZEN, SC-008-FROZEN]
series: SC-001…SC-013 (v0.1.0-Kernelspezifikation, AD-013)
---

# SC-009 — Kernel-Diagnostics & Event-Bridge (v0.1.0, FROZEN 04.10.2026)

> **Status:** FROZEN v0.1.0 per AD-013 — Owner-Freigabe 04.10.2026.
> Vor Freeze nachgezogen: (1) REQ-SC009-06 Zwei-Klassen-Verdrängung
> (DIAGNOSTIC zuerst FIFO; Security hart begrenzt mit OVERFLOW-Zähler;
> Schreiber blockiert nie — INV-07). Als nicht bindend, G7-relevant
> markiert und mit eingefroren: (2) REQ-SC009-02 Grant-Pflicht für
> SUBSCRIBE (kein ambientes Abo; Kernel-Politik-Default, SC-DEC-Kandidat),
> (3) REQ-SC009-04a automatische Katalog-Fortschreibung (Version =
> höchste gefrorene SC; M-E07 = SC-007-Bestand, kein Forward-Ref) und
> Deferral-Mechanik in REQ-SC009-05 (Marker → Drain → Ring-Write →
> Notification). Aurora NUR-LESEN (AD-013); Defaults mit SC-ARCH-Review.

## 1. Zweck

Verbindliche Spezifikation des strukturierten Diagnose-/Ereigniskanals des
Kernels: Diagnostic-Event-Objekte, deren Erzeugungspflicht, und die
Cap-vermittelte Auslieferung an Konsumenten (Kernel-Event-Bridge → Aurora,
L2). Der Kanal ersetzt dmesg/ printk als Semantik: Ereignisse sind
Kernel-Objekte mit definierten Klassen, keine Formatstrings, kein Pollen.

## 2. Objektmodell (REQ-SC009-01…03)

- **REQ-SC009-01 (MUST) Diagnostic-Event:** Kernel-Objekt {Klasse, Severity,
  Quell-Fehlerklasse (z. B. I-E04, C-E04, REQ-SC007-13a), Kernel-Zeit-
  stempel (monoton, SC-006 §3), Kontextwörter (Default 8)}.
- **REQ-SC009-02 (MUST) Diagnostic-Cap:** {SUBSCRIBE} — nur der Kernel
  erzeugt Events; Konsumenten abonnieren über gemintete Diagnostic-Caps.
  Grant-Pflicht: SUBSCRIBE entsteht ausschließlich per explizitem Grant
  (mint) aus dem Supervisor-CSpace — KEIN ambientes/Default-Abonnement
  (Verdrahtung SC-005 §4; sonst Timing-Seitenkanal über Ereignisraten).
  Status: Kernel-Politik-Default wie REQ-SC007-05 — NICHT bindend vor G7,
  SC-DEC-Kandidat, falls G7 davon abweichen will.
- **REQ-SC009-03 (MUST) Delivery:** Events werden als Notification-artige
  Auslieferung an den Endpoint des Konsumenten gepusht; das Abrufen der
  Detail-Records erfolgt per invoke (kein Polling-ABI, kein Formatstring).

## 3. Erzeugungspflicht (REQ-SC009-04…06)

- **REQ-SC009-04 (MUST) Pflicht-Events:** Alle in SC-001…SC-008 definierten
  Diagnostic-Events SIND Pflicht: Deadlock (I-E04), PI-Limit (I-E03),
  Revocation-während-Block (I-E06), Sicherheits-Verstöße (C-E04, D-E05,
  M-E03-Analog), Terminal-Zustände (REQ-SC007-13a, P-E07), HAL-Anomalien
  (D-E07), TLB-Inkonsistenz (SC-007 M-E07 — Bestand aus 001–008, KEIN
  Forward-Ref), Notification-Verwurf (REQ-SC008-10a).
- **REQ-SC009-04a (MUST) Katalog-Fortschreibung:** Der Pflicht-Katalog
  nimmt neue Diagnostic-Event-Klassen AUTOMATISCH auf, sobald sie in
  einer gefrorenen SC definiert sind (kein SC-009-Amendment-Zyklus);
  Katalogversion = höchste gefrorene SC-Nummer. NICHT bindend vor G7 —
  als Kategorie-B-Eintrag im G7-Audit-Trail geführt (TEMPLATE,
  G7_AUDIT_TRAIL_TEMPLATE.md).
- **REQ-SC009-05 (MUST) IRQ-Hot-Path-Verbot:** Im IRQ-Pfad wird KEIN Event
  synchron erzeugt (SC-006 INV-06); Events aus IRQ-Kontext werden
  deferred (minimaler Marker, Ausformung außerhalb des Hot Paths).
  Deferral-Mechanik: Der IRQ-Kontext schreibt nur einen Marker-Slot (Flag
  je Quelle); Ring-Write und Ausformung erfolgen im Drain — dem ersten
  Nicht-IRQ-Kontext auf dem zugehörigen CPU-Kern; die Konsumenten-
  Notification folgt nach dem Ring-Write, nie direkt aus dem IRQ-Pfad.
- **REQ-SC009-06 (MUST) Append-only & Zwei-Klassen-Verdrängung:** Events
  sind unveränderlich nach Erzeugung; Kernel-interner Ring je Konsument
  (Default 256, davon harte Security-Obergrenze 64). ZWEI Prioritäts-
  klassen: SECURITY und DIAGNOSTIC. Verdrängungsregel: (a) Ein neues
  SECURITY-Event verdrängt zuerst nach FIFO aus der DIAGNOSTIC-Klasse
  (Noise zuerst raus); (b) erreicht die Security-Klasse ihre harte
  Obergrenze, erzeugt der Kernel einen SECURITY_OVERFLOW-Zähler-Event und
  verwirft das älteste SECURITY-Event (dokumentierte Degradation, nie
  still); (c) der Schreiber blockiert NIE (INV-07), auch nicht bei vollem
  Security-Limit — der Overflow-Zähler ist der definierte Rückkanal.
  Kein Ring-Wachstum über die Obergrenzen hinaus.

## 4. Kernel-Event-Bridge (REQ-SC009-07…09)

- **REQ-SC009-07 (MUST) Bridge = Service Space:** Die Bridge ist ein
  Userspace-Service, KEIN Kernel-Bestandteil (AD-028): sie hält eine
  Diagnostic-Cap, abonniert, aggregiert, übersetzt für Aurora-Core (L2).
- **REQ-SC009-08 (MUST) NUR-LESEN für AI:** Aurora erhält ausschließlich
  Ereignis-Sicht über die Bridge (Capability-Tokens gem. AD-012) — KEIN
  Kernel-Steuerzugriff, kein Schreiben von Events, kein Eingriff in
  Scheduling/IPC. Beobachtung ja, Enforcement bleibt Kernel (AD-013).
- **REQ-SC009-09 (MUST) Keine Hidden Channels:** Es existiert kein
  Ereignis- oder Steuerpfad zu Aurora (oder irgendeinem Service) außer
  über Cap-geprüfte Endpoints dieser Spezifikation.

## 5. Invarianten (MUST)

INV-01 Events entstehen nur im Kernel, nur mit definierten Klassen.
INV-02 Kein Event synchron im IRQ-Hot-Path (deferred).
INV-03 Events sind append-only und unveränderlich.
INV-04 Verdrängung zuerst in der DIAGNOSTIC-Klasse (FIFO); Security-
       Events werden erst bei vollem Security-Limit verworfen — mit
       SECURITY_OVERFLOW-Zähler (REQ-SC009-06), nie still.
INV-05 Konsumenten brauchen Diagnostic-Caps; keine Broadcast-Events ohne
       Cap (Badge-Provenanz SC-005 §6).
INV-06 Die Bridge (und Aurora) hat NUR-LESEN; keine Kernel-Steuerung.
INV-07 Kein Event blockiert Kernel-Pfade: Erzeugung/Auslieferung sind
       nie Voraussetzung für eine OP (Analogie REQ-SC008-10a).

## 6. Fehlerklassen (MUST-behandelbar)

E-E01 Abonnement ohne Diagnostic-Cap → AUTH_DENIED (Y-E01).
E-E02 Ring leer beim Abruf → definierter Leerzustand (kein Fehler-Wort).
E-E03 Konsument-Endpoint weg → Event verworfen + Zähler (kein Blockieren).
E-E04 Unbekannte Event-Klasse beim Konsumenten → verworfen, Bridge-
      Verantwortung (Kernel liefert nur definierte Klassen).
E-E05 Deferred-Marker verloren (extremer Overflow) → Zähler-Event beim
      nächsten Drain (nie still).
E-E06 Zeitstempel nicht monoton (HAL-Anomalie) → D-E07-Pfad, Events
      behalten trotzdem Erzeugungsordnung (Ring-Sequenz).
E-E07 Bridge-Angriffsversuch (Scheiben/Fälschen) → unmöglich per
      INV-01/INV-03; Versuch selbst erzeugt ein Sicherheits-Event (C-E04).

## 7. Testbarkeit (MUST)

- T3 Unit: Ring-Algebra, Überlauf-Zähler, Append-only, Klassen-Mapping.
- T4 Integration: I-E04-Deadlock erzeugt Pflicht-Event; IRQ-Kontext-Deferral
  (kein synchrones Event); Bridge-Abo/Rückzug; E-E03-Verwurf.
- T1 QEMU (M5): Boot mit Bridge-Service, erster Pflicht-Event-Roundtrip
  (Terminal-Fall eines Test-Services), Monotonie über Boot.
- Determinismus: identische Fehlersequenzen ⇒ identische Event-Sequenzen.

## 8. Abgrenzung

- Kein dmesg/printk-ABI (Formatstrings, Pollen, globale Konsole) — Konsole/
  Log-Dateien sind Service Space.
- Aurora-Core-Verarbeitung (L2, Semantik der Ereignisse, AI-Entscheidungen)
  ist NICHT Teil dieser Spezifikation — nur die Kernel-Grenze.
- DefenderGPT: reine Beobachtungsinstanz; deren Service-Verhalten ist
  Service-Space-Spezifikation, nicht Kernel-Spec.
- panic.rs/logging-Interim-Bestände bleiben Referenz (AD-026).

## 9. Defaults (reversibel, Review bei SC-ARCH)

| Wert | Default | Ort |
|---|---|---|
| Ring je Konsument | 256 Events (Security-Klasse max 64) | §3 |
| Kontextwörter je Event | 8 | §2 |
| Severity-Stufen | 4 (INFO/WARN/ERROR/SECURITY) | §2 |
| Deferred-Marker je IRQ | 1 Slot, drain-basiert | §3 |
| Bridge-Abo-Endpunkte | 1 je Diagnostic-Cap | §4 |

## 10. Schnittstellen

- SC-002: Zeitstempelquelle (tickless, monoton via SC-006 §3).
- SC-003: Notification-Delivery-Mechanik; I-E01…E07-Quellen.
- SC-004: invoke-Abruf der Detail-Records; Statuswort (SC-DEC-N).
- SC-005: Diagnostic-Cap mint/revocation; Badge-Provenanz.
- SC-006: IRQ-Deferral; HAL-Anomalie-Quelle (D-E07).
- SC-007: Terminal-Suspend-Quelle (REQ-SC007-13a), M-E07.
- SC-008: Notification-Verwurf (REQ-SC008-10a), P-E05/E07-Quellen.
- SC-010ff (folgend, AD-013-Reihenfolge): verbleibende Kernel-Specs bis
  SC-013 (ATCLang-ABI-Bindung).

## 11. Referenzen

AD-012 (Kernel-Event-Bridge + Capability-Tokens, Minimal VFS nicht Kernel),
AD-013 (AI nur beobachten, Enforcement im Kernel, Kernel-Reinheit),
AD-026 (Reihenfolge), AD-027 (M2/M5), AD-028 (Service Space),
SC-001…SC-008 (alle FROZEN inkl. aller REQ-SC007-13a/SC008-10a-
Nachzüge), SHIVA-HAL-001, kernel/src/panic.rs (Interim), Aurora
(aurora-ai, L2 — nur Empfänger, keine Kernel-Rolle), docs/specs/
SC-001…SC-008.

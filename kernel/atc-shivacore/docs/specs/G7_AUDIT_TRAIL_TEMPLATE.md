---
document_id: G7-TRAIL
title: "G7-Audit-Trail — Vorlage (Nicht-Bindungen vor ABI-Freeze)"
version: 0.1.0-TEMPLATE
status: TEMPLATE — VORLAGE; wird erst mit G7-Terminierung verbindlich geführt. Owner merged diese Vorlage gegen eigene Notizen.
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems / ShivaCore (Michael Wroblewski)
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-04
owner_instruction: 04.10. — G7-Audit-Trail wird vom Owner übernommen, sobald G7 terminiert. §3-Nachprotokollierung (SC-005) nur, falls G7 einen separaten Trail erzwingt.
---

# G7-Audit-Trail (Vorlage)

> **Zweck:** Nachweis-Dokument für die ABI-Bindung G7 (ATC-ABI-001,
> ATCLang-Codegen): Alle Stellen in SC-001…SC-013, die NICHT binden,
> bevor G7 die erste binäre Bindung erzeugt. Zwei Kategorien:
>
> **A — "abgeleitet, keine neue Zusage":** Aussagen, die aus bereits
> gefrorenen Invarianten abgeleitet sind und daher keiner separaten
> Freigabe bedurften.
>
> **B — "Defaults binden nicht vor G7":** Reversible Detailwerte, die per
> Standing-Mandat (03.10.) als Defaults dokumentiert sind und erst bei
> SC-ARCH-001…010 bzw. G7 binär werden.

## A — Abgeleitete Aussagen (keine neue Zusage)

| Spec | Stelle | Aussage | Ableitung aus |
|---|---|---|---|
| SC-005 | REQ-SC005-09a / INV-09 | Snapshot-Semantik: laufender atomarer invoke schließt auf der Cap-Resolution zum Eintritt; Revocation wirkt am sysret; blockierte OPs per I-E06 | SC-004 INV-06/INV-07 (transiente Caps bei Exit; atomare OPs alle-oder-nichts) + SC-003 I-E06 (Wake mit Fehlercode) — markiert als "abgeleitet, KEINE neue Zusage" |
| SC-007 | REQ-SC007-09a | Rechte-Reduktion nie in-place; mint+revoke; Propagation hält INV-03 global | SC-005 §4/§5 (Derivation/Revocation) + SC-007 INV-02 (Monotonie) — Konsistenzschluss, keine neue Verpflichtung |
| SC-008 | REQ-SC008-10a | Lebensende-Pfad empfängerunabhängig; Untyped-Rückfluss im Derivationsbaum des Creators | SC-005 §4/§5 (Derivationsbaum als Rückfluss-Garant) + REQ-SC008-11 — schließt Auditor-Lücke, kein neues Versprechen |

## B — Defaults (binden nicht vor G7; Review bei SC-ARCH-001…010)

| Spec | Default | Wert |
|---|---|---|
| SC-003 | Register-Transfer | 4 Maschinenwörter (≡ SC-004 §12) |
| SC-004 | Argument-/Resultatregister, Statuswort, Slot-Adressraum, Alignment, yield-Bereich | 4/2 Wörter; 8+24 Bit; 2^14; 16; fester Kern-OP-Bereich |
| SC-005 | CNode-Slots, Rechte-Maske, Badge, Derivationstiefe, Revocation-Batch | 2^14; 16 Bit; 64 Bit; 16; 1024 |
| SC-006 | IRQ-Queue, Timer-Queue, Bitmask, MMIO-Granularität, Zeitbasis | 32; 1024; 64 Bit; 4 KiB; HAL-monoton 64 Bit |
| SC-007 | VA-Breite, Mapping-Granularität, Fault-Queue, TLB-Modus, PT-Allokation, HugePages | 48 Bit; 4 KiB; 16; synchron; lazy; NICHT aktiv (Pfad §7) |
| SC-007 | W^X-Politik | Kernel-Politik-Default — änderbar nur per SC-DEC/SC-ARCH-Review, KEIN Service-Space-Laufzeitpfad |
| SC-008 | Threads/Aggregat, Reserve-Slots, Exit-Queue, Spawn-Kontext | 256; 2; 16; 4 Wörter |
| SC-009 | Ring, Kontextwörter, Severity, Deferred-Marker, Abo-Endpunkte | 256; 8; 4 Stufen; 1 Slot; 1 |

## C — Wirksam bereits VOR G7 (keine Nicht-Bindung)

| Spec | Stelle | Grund |
|---|---|---|
| SC-004 | SC-DEC-K/L/M | Freigegeben 04.10.; reversibel nur BIS zur ersten binären Bindung (G7) bzw. globus-init-Bindung (M) — danach ABI-Freeze. Bis dahin: KEINE Reserved-Number-Festlegung, nur dokumentierte Reservierungsbereiche |
| SC-004 | SC-DEC-N (Fehler-/Restart-Semantik) | Als Default BESTÄTIGT und ab SC-003 WIRKSAM — bindet schon jetzt, keine Nicht-Bindung |
| SC-004 | INV-08 (ABI-Major-Prüfung) | Normativ im ABI-Dispatch, ab SC-004 wirksam |

## D — Pflege-Regeln (bei G7-Terminierung)

1. Owner merged diese Vorlage gegen eigene Notizen; ab dann ist der Trail
   verbindlich zu führen (jeder SC-Freeze ergänzt A/B).
2. Jede binäre Bindung (G7-Codegen, globus-init-Bindung) protokolliert:
   welche Kategorie-B-Defaults dadurch binär werden (Statuswechsel in
   Spalte "Wird binär am …").
3. Kategorie-A-Einträge bleiben dauerhaft als Ableitungs-Nachweis.
4. §3-Nachprotokollierung (SC-005 Snapshot) nur, falls G7 einen separaten
   Audit-Trail erzwingt — Owner-Regel 04.10.

## E — Referenzen

SC-001…SC-008 (FROZEN), SC-009 (DRAFT_REVIEW), SC-004 §13 (SC-DEC-K/L/M/N-
Protokoll), ATC-ABI-001 (G7-DRAFT, SCR-0071), SHIVA-ABI-001, AD-013,
Owner-Standing-Mandat 03.10. (Defaults statt blockierender SC-DEC),
Owner-Anweisung 04.10. (Audit-Trail nach G7-Terminierung).

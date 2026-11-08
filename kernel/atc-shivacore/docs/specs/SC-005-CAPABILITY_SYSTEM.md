---
document_id: SC-005
title: "ShivaCore v0.1 Kernelspezifikation — Capability-System-Vertiefung"
version: 0.1.0-FROZEN
status: FROZEN v0.1.0 — Owner-Freigabe 04.10.2026 nach 4-Punkte-Verifikation (Badge-Immunität, Limit-Semantik, Snapshot-Revocation, Initial-Allmacht)
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems / ShivaCore (Michael Wroblewski)
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-04
ad_refs: [AD-012, AD-013, AD-026, AD-027, AD-028]
depends: [SC-001-FROZEN, SC-002-FROZEN, SC-003-FROZEN, SC-004-FROZEN]
series: SC-001…SC-013 (v0.1.0-Kernelspezifikation, AD-013)
---

# SC-005 — Capability-System-Vertiefung (v0.1.0, FROZEN 04.10.2026)

> **Status:** FROZEN v0.1.0 per AD-013 — Owner-Freigabe 04.10.2026 nach
> 4-Punkte-Verifikation am Volltext: (1) Badge-Immunität als INV-05
> nachgezogen (mint nur NEUER Badge auf Ableitung, Original unberührt,
> Badge nie Rechtekanal); (2) Limit-Semantik als REQ-SC005-07a/C-E03
> (mint schlägt fehl, kein Truncate, kein Wrap); (3) Snapshot-Semantik
> REQ-SC005-09a/INV-09 — aus SC-004 INV-06/07 + SC-003 I-E06 ABGELEITET,
> keine neue Zusage; (4) REQ-SC005-11: kein All-Rights-Cap, kein ambient
> derive im Initial-Layout. Kernvorgabe AD-013: Handles, nicht Krypto.

## 1. Zweck

Verbindliche Vertiefung des Capability-Modells: CSpace/CNode-Aufbau,
Rechte je Objekttyp, Derivation (mint/derive/clone), Revocation und
Badge-Propagation. Das Capability-System ist die EINZIG Autoritätsquelle
des Kernels (SC-004 INV-01/02); hier wird seine Struktur normativ.

## 2. CSpace & CNode (REQ-SC005-01…03)

- **REQ-SC005-01 (MUST) CNode:** Knoten mit fester Slot-Anzahl (Default
  2^14, ≡ SC-004 §12); Slot ist entweder LEER, frei oder Capability.
- **REQ-SC005-02 (MUST) CSpace:** Baum aus CNodes; Wurzel = Initial-Cap
  des Owners; jeder Thread adressiert Caps über (CNode-Pfad, Slot).
- **REQ-SC005-03 (MUST) Cap-Eintrag:** {Objekttyp, Objektreferenz,
  Rechte-Maske (16 Bit, Default §12), Badge (64 Bit, unveränderlich)}.
  Der Kernel interpretiert NUR diese Struktur — keine Zeiger-Autorität
  (SC-004 INV-02).

## 3. Rechte je Objekttyp (REQ-SC005-04)

| Objekt | Rechte-Menge |
|---|---|
| Endpoint | SEND, RECV, DELEGATE, NOTIFY (SC-003 §2) |
| Notification | SIGNAL, WAIT |
| Frame | READ, WRITE, EXECUTE, GRANT (Mapping nur via AddressSpace-Cap) |
| AddressSpace | MAP, UNMAP, SWITCH |
| Thread | SUSPEND, RESUME, CONFIG (Domain-Neuerzeugung, SC-002) |
| CNode | DUPLICATE, MINT (Ableitung), MOVE |
| UntypedMemory | RETYPE (SC-001 §4) |
| Timer | SET_DEADLINE, ACK (SC-DEC-H) |
| Device/IRQ | ACCESS, SUBSCRIBE, ACK (Details SC-006, folgt) |

**REQ-SC005-04 (MUST):** Außerhalb dieser Menge existieren keine Rechte;
unbekannte Bits in der Maske = INVALID_CAP (Y-E02-Pfad, SC-004).

## 4. Derivation (REQ-SC005-05…07)

- **REQ-SC005-05 (MUST) mint:** Ableitung mit Rechte-Reduktion — Kind-Maske
  ⊆ Vater-Maske; Badge-Wahl nur bei mint auf Endpoints (SC-003 §2). Keine
  Rechte-Erweiterung, nirgends, nie.
- **REQ-SC005-06 (MUST) derive:** Kopie der Maske ohne Badge-Änderung;
  Derivationstiefe begrenzt (Default 16, §12).
- **REQ-SC005-07 (MUST):** Jede Ableitung wird im Derivationsbaum des
  Originals registriert — Voraussetzung für vollständige Revocation (§5).
- **REQ-SC005-07a (MUST) Verhalten am Derivationslimit:** Überschreitet eine
  Ableitung die Tiefe (Default 16, §12), schlägt der mint/derive FEHL mit
  DERIVATION_LIMIT (C-E03); das Statuswort-Detail enthält die erreichte
  Tiefe. Keine stille Truncation des Baums, kein Wrap-Around, keine
  Zustandsänderung am Original oder bestehenden Ableitungen.

## 5. Revocation (REQ-SC005-08…09)

- **REQ-SC005-08 (MUST):** revoke löscht rekursiv den gesamten
  Ableitungsbaum; bei Frames zusätzlich UNMAP in allen AddressSpaces.
  Revocation ist atomar je Baum; kein Zombie-Zugriff nach Rückkehr.
- **REQ-SC005-09 (MUST):** Revocation während blockierter OPs folgt
  SC-003 I-E06 (kontrolliertes Aufwecken mit Fehlercode) — inklusive
  Badge-lose Reply-Pfade (SC-003 I-E07).
- **REQ-SC005-09a (MUST) Snapshot-Semantik für laufende syscalls**
  (abgeleitet aus SC-004 INV-06/INV-07 + SC-003 I-E06, KEINE neue Zusage):
  Trifft eine Revocation eine Cap, die in einem bereits laufenden,
  nicht-blockierenden invoke slot-resolviert ist, schließt dieser
  syscall auf der Cap-Resolution zum Eintritt ab (atomar, alle-oder-
  nichts, SC-004 INV-07); die Revocation wirkt mit sysret (INV-09).
  NEUE Aufrufe auf dem Slot sehen danach C-E01/INVALID_CAP.
  Für blockierte OPs gilt SC-003 I-E06 (Aufwecken mit Fehlercode),
  nicht Snapshot. Es existiert KEINE Abbruch-Semantik für laufende
  atomare OPs — ein mitten drin abgebrochener invoke würde die
  Alle-oder-nichts-Garantie brechen.

## 6. Badge-Propagation (REQ-SC005-10)

Kernel-vergeben, unveränderlich (SC-003 REQ-SC003-03). Propagation
ausschließlich via mint auf Endpoint-Bindungen; derive ändert Badge nie.
mint setzt ausschließlich einen NEUEN Badge auf der abgeleiteten Cap; der
Badge des Originals bleibt unberührt und ist von abgeleiteten Caps aus
nie überschreibbar. Badge ist kein Rechtekanal: keine Badge-Operation hebt
die Rechte-Reduktion von REQ-SC005-05 auf (Badge-Immunität, INV-05).
Badge ist Provenanz-Grundlage für SC-003 §2 und identitätsstiftend für
globus-init-Delegationen (SHIVA-GLOBUS-INTEGRATION-001).

## 7. Initial-Layout (globus-init, SC-001 §5)

Initial Task erhält: Root-CNode-Cap, Initial-Untype-Caps (voll delegiert,
SC-DEC-D), Timer-, IRQ-, Device-Caps (Framebuffer per SC-DEC-F), Thread-Cap
für sich selbst. KEIN Unix-Root, keine impliziten Rechte (REQ-SC001-12).
globus-init delegiert an Services nur per mint mit Reduktion.

**REQ-SC005-11 (MUST) Keine Initial-Allmacht:** Das Initial-Layout enthält
KEINEN All-Rights-Cap über beliebige Objekttypen und kein ambientes derive:
jeder Initial-Cap ist typgebunden an genau eine Rechte-Menge aus §3, mint
erfolgt nur auf konkrete Ziel-Caps mit Reduktion. Es existiert keine
"weil initial, darf alles"-Autorität (Verstärkung von REQ-SC001-12).

## 8. Invarianten (MUST)

INV-01 Kein Kernel-Objektzugriff ohne Capability (alles: invoke, map, IPC).
INV-02 Cap-Struktur ist die einzige Autorität; Zeiger autorisieren nie.
INV-03 Derivation ist monoton: Rechte-Mengen wachsen nie aufwärts.
INV-04 Derivationsbaum ist vollständig registriert; Revocation hinterlässt
       keine erreichbaren Ableitungen.
INV-05 Badge-Immunität: Badges sind kernel-vergeben und nach Erzeugung
       unveränderlich; abgeleitete Caps ändern den Badge des Originals
       nie — nur ein neuer mint auf demselben Endpoint setzt einen NEUEN
       Badge auf der Ableitung. Der Badge ist kein Rechtekanal.
INV-06 Rechte-Masken enthalten nur Typ-Modi aus §3; fremde Bits = Fehler.
INV-07 Revocation während Block folgt SC-003 I-E06/E07 — kein stiller
       Weiterlauf.
INV-08 CSpace-Zugriff eines Threads nur über eigene CNode-Pfade (kein
       Cross-CSpace-Lesen ohne DUPLICATE-Maschine).
INV-09 Revocation wirkt an definierten Übergangspunkten (sysret bzw.
       I-E06-Wake): ein laufender atomarer OP sieht die Slot-Resolution
       bis zum Abschluss; danach ist der Slot unzugreifbar (C-E01).

## 9. Fehlerklassen (MUST-behandelbar)

C-E01 Ungültiger/leerer Slot → INVALID_CAP (SC-004 Y-E02).
C-E02 Rechte fehlen → AUTH_DENIED (SC-004 Y-E01), keine Zustandsänderung.
C-E03 Derivationstiefe überschritten → DERIVATION_LIMIT (Statuswort-
      Detail = erreichte Tiefe); mint/derive schlagen fehl — keine stille
      Truncation, kein Wrap, Original unverändert.
C-E04 mint mit Rechte-Erweiterung versucht → AUTH_DENIED + Diagnostic-Event.
C-E05 CNode voll → CSPACE_FULL (kein stills Überschreiben freier Slots).
C-E06 revoke auf Kernel-internes Original (nicht ableitbar) → verboten,
      AUTH_DENIED.
C-E07 fremde Bits in Rechte-Maske → INVALID_CAP.

## 10. Testbarkeit (MUST)

- T3 Unit: Maske-Algebra (mint/derive), Derivationsbaum-Registrierung,
  Revocation-Rekursion, Badge-Unveränderlichkeit.
- T4 Integration: revoke während blockierter send/recv (SC-003 I-E06),
  UNMAP-Wirkung auf laufende AddressSpaces, globus-init Initial-Set.
- T1 QEMU (M5): Boot mit Initial-Caps, erster Service-Start per
  Delegation, Frame-Mapping über GRANT.
- Determinismus: identische Cap-Operationen ⇒ identische Slot-Zustände.

## 11. Abgrenzung

- `shivaos/kernel/capabilities.py` (Jul-Sprint, Layer-2-Referenz) und
  `capabilities.rs` (Kernel-Interim) bleiben Referenz; Implementierung
  gegen diese Spec nach SC-001…SC-013 (AD-026).
- Kryptografische Caps (remote_caps, did, ContentCap/KeyCap Layer-5) sind
  Service-Space-Erweiterungen (AD-013 Klarstellung) — NICHT Teil dieser
  Spezifikation; Anbindung später über SC-006+/Globus-Integration.
- Kern-Caps sind Handles, keine Tokens: keine Signatur, keine Übertragung
  über Netz (AD-013).

## 12. Defaults (reversibel, Review bei SC-ARCH)

| Wert | Default | Ort |
|---|---|---|
| CNode-Slot-Anzahl | 2^14 (≡ SC-004 §12) | §2 |
| Rechte-Maske | 16 Bit | §2 |
| Badge | 64 Bit | §2, §6 |
| Derivationstiefe | 16 | §4 |
| Revocation-Batch | 1024 Slots je Durchgang | §5 |
| Derivationsbaum-Wachteln | pro Original, Kernel-intern | §4 |

## 13. Schnittstellen

- SC-001: Untype/Retype-Autorität; Initial-Cap-Umfang (§7).
- SC-002: Thread-Cap CONFIG = Domain-Neuerzeugung; kein Domain-Wechsel
  über Caps zur Laufzeit (REQ-SC002-03).
- SC-003: DELEGATE/Badge/I-E06-I-E07-Verdrahtung.
- SC-004: invoke = einzige OP-Maschine auf Caps; Slot-Adressierung
  (FROZEN, SC-DEC-L/M).
- SC-006 (folgend): Device/IRQ-Cap-Details; Timer-Cap-Feinspezifikation.

## 14. Referenzen

AD-012 (Kernel-Primitive), AD-013 (Kernel-enforced Handles, keine Krypto,
globus-init ohne Unix-Root), AD-026 (Reihenfolge), AD-027 (M2/M5), AD-028
(Kernel-Reinheit), SC-001-FROZEN (Untyped, Initial-Caps), SC-002-FROZEN
(Domains, SC-DEC-G/H), SC-003-FROZEN (Endpoint/Notification/Badge,
I-E01…E07), SC-004-FROZEN (invoke-ABI, SC-DEC-K/L/M/N),
SHIVA-GLOBUS-INTEGRATION-001, SHIVA-KERNEL-REUSE-001,
kernel/src/capabilities.rs (Interim), shivaos/kernel/capabilities.py
(Referenz-Host), docs/specs/SC-001…SC-004.

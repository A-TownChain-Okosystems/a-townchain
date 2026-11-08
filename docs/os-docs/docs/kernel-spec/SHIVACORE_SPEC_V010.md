# ShivaCore v0.1.0 — Technische Spezifikation (SC-001 Kernel Object Model)

> **Status:** ENTWURF · DRAFT_REVIEW v0.1.0-draft1 · **Datum:** 06.09.2026
> **Noch NICHT im DECISIONS_REGISTER** — Eintrag erfolgt beim Freeze.
> **Basis:** Architektur-Gate (AD-013, `SHIVACORE_V01_ARCHITECTURE_GATE.md`)
> **Anschlussfähigkeit:** Alignment-Tabelle zu K29 (`atc-shivacore/kernel/src/capability.rs`) am Ende.

---

## 0. Normative Konventionen (für ALLE SC-Spezifikationen)

| Schlüsselwort | Bedeutung |
|---|---|
| MUST / MUSS | Verpflichtend; Verletzung = Spezifikationsbruch |
| MUST NOT / DARF NICHT | Absolutes Verbot |
| SHOULD / SOLL | Empfohlen; Abweichung MUSS im DECISIONS_REGISTER begründet werden |
| MAY / KANN | Optional |

**Regeln für diesen Dokumentensatz:**
- **NK-001:** Jede MUST-Regel MUSS mindestens einen identifizierten Testvektor (TV-ID) haben.
- **NK-002:** Regeln sind stabil nummeriert (SC-001-Regeln: `KO-nnn`).IDs werden nie wiederverwendet.
- **NK-003:** Spezifikationsversion und Regel-Revisionsstand sind getrennt (Regeln können einzeln von SHOULD zu MUST geschärft werden).
- **NK-004:** Die Spezifikation ist implementierungssprachen-neutral; Codebeispiele sind informativ (Rust als Referenznotation).

---

## 1. Objekttypen (normativ)

**KO-101 (MUST):** Jede Kernel-Ressource ist ein `KernelObject` eines der folgenden Typen:

| Typ | Code (u8) | Zweck |
|---|---|---|
| `Thread` | 0x01 | Ausführungskontext (TCB-Halter) |
| `AddressSpace` | 0x02 | Virtueller Adressraum |
| `PageTable` | 0x03 | Seitenzuordnung |
| `Frame` | 0x04 | Physischer Speicherrahmen (4 KiB-Granularität) |
| `CNode` | 0x05 | Capability-Knoten (Slot-Array) |
| `Endpoint` | 0x06 | Synchron IPC-Objekt (send/receive/call) |
| `Notification` | 0x07 | Asynchrones Signal-Objekt (wort-basiert) |
| `Irq` | 0x08 | Interrupt-Handle (bind/wait/ack) |
| `Device` | 0x09 | Hardware-/Geräte-Handle (inkl. Secure RNG) |
| `UntypedMemory` | 0x0A | Nicht getypter Speicher (retype-Quelle) |
| `Timer` | 0x0B | Zeitgeber-Objekt (sleep/timer_create) |

- Codes 0x00 und 0x0C–0xFF sind reserviert (ABI-Stabilität).
- **KO-102 (MUST NOT):** Es DARF KEIN Kernel-Objekt geben, das außerhalb dieser Typen fällt. Neue Typen erfordern Register-Eintrag + ABI-Versionsprung.

---

## 2. Objekt-Identität

**KO-110 (MUST):** Jedes KernelObject hat eine `KernelObjectId` (64 Bit) mit folgender Aufteilung:

```
┌──────────────┬───────────────────────┬────────────────────────────┐
│ Typ (8 Bit)  │ Generation (24 Bit)   │ Index (32 Bit)             │
└──────────────┴───────────────────────┴────────────────────────────┘
```

- **KO-111 (MUST):** Der `Index` ist pro Typ monoton steigend; Wiederverwendung eines Index erfordert Generationserhöhung (ABA-/Handle-Wiederverwendungs-Schutz).
- **KO-112 (MUST):** Ein Handle auf ein nicht mehr existierendes Objekt MUSS bei jedem Zugriff `InvalidObject` liefern — niemals stillschweigend ein neues Objekt adressieren.
- **KO-113 (MUST):** Kernelobjekt-Erzeugung ist deterministisch: gleicher Eingabezustand ⇒ gleicher Ausgabezustand (NK/SC-ARCH: deterministischer Kernel). Zufall NUR über `Device(0x09, RNG)`-Capability.

---

## 3. Lebenszyklus

**KO-120 (MUST):** Objektzustände: `Active` → `Retired` (terminal). Kein Wiedereintritt.

```
   create              revoke/delete (letzte Capability weg)
───────► Active ───────────────────────────────► Retired ✝
           │
           └─► (keine anderen Transitionen)
```

- **KO-121 (MUST):** Ein Objekt wird automatisch `Retired`, sobald die letzte es referenzierende Capability gelöscht/widerrufen wurde (kein Dangling, kein Leak außerhalb von UntypedMemory-Retain).
- **KO-122 (MUST NOT):** Auf `Retired` DARF NICHT mehr zugegriffen werden; spätere Zugriffe liefern `InvalidObject`.
- **KO-123 (SHOULD):** `Retired`-Objekte SHOULD wiederverwendbar sein (Index+Generation) als Speicheroptimierung — aber nur mit neuem Generation-Zähler.

---

## 4. Capability-Struktur (normativ)

**KO-130 (MUST):** Capability = unforgeable Referenz mit:

```rust
struct Capability {
    target: KernelObjectId,   // referenziertes KernelObject
    rights:  Rights,          // 16-Bit-Bitfield, s.u.
    badge:   Badge,           // 64-Bit, Endpoint-/Notification-Semantik
    deriv:   DerivationInfo,  // Elterncap + Attenuationspfad
}
```

### 4.1 Rights-Bitfield (u16)

| Bit | Recht | Bedeutung |
|---|---|---|
| 0 | `READ` | Lesezugriff (Daten, State) |
| 1 | `WRITE` | Schreibzugriff |
| 2 | `EXEC` | Ausführung (Code/Thread-Ausführung) |
| 3 | `DELEGATE` | Weitergabe an Dritte erlaubt |
| 4 | `SEND` | Endpoint: senden |
| 5 | `RECEIVE` | Endpoint: empfangen |
| 6 | `CALL` | Endpoint: send+reply (RPC) |
| 7 | `GRANT` | Rights-Erweiterung bei Derivation (mint); Kernel-Only-Vergabe |
| 8–15 | — | reserviert, MUST 0 sein |

- **KO-131 (MUST):** Bits 0–3 sind identisch zu K29 (`Rights(u8)`, READ=1/WRITE=2/EXEC=4/DELEGATE=8) — das ist die Kompatibilitätsbrücke.
- **KO-132 (MUST NOT):** Derivation DARF NIE Rights erweitern; nur attenuieren (`subset_or_equal`). Erweiterung nur durch explizite `GRANT`-Operation des Kernel (nicht durch User-Delegation).
- **KO-133 (MUST):** Rechte sind objekttyp-bezogen; nicht anwendbare Rechte werden ignoriert (z. B. `EXEC` auf `Frame` ohne `EXEC`-Mapping).

### 4.2 Rechte-Matrix je Objekttyp (MUST)

| Objekt | Anwendbare Rechte |
|---|---|
| Thread | READ, WRITE, EXEC(delegierte Ausführung), DELEGATE |
| AddressSpace | READ, WRITE, EXEC, DELEGATE |
| PageTable / Frame | READ, WRITE, EXEC, DELEGATE |
| CNode | READ, DELEGATE (+ GRANT kernel-only) |
| Endpoint | SEND, RECEIVE, CALL, DELEGATE |
| Notification | SEND, RECEIVE, DELEGATE |
| Irq | READ(wait/ack), DELEGATE |
| Device | READ, WRITE, DELEGATE (typabhängig) |
| UntypedMemory | READ, WRITE (retype), DELEGATE |
| Timer | READ, DELEGATE |

### 4.3 Badge

- **KO-134 (MUST):** `Badge` ist ein 64-Bit-Wort, vom Kernel opak verwaltet; Semantik wird pro Endpoint/Notification vom Empfänger definiert (z. B. Absender-Identifikation bei gebadgten Proxies).
- **KO-135 (MUST NOT):** Badge DARF NICHT als Autorisierung allein dienen — sie annotiert, autorisiert tun es Rights.

---

## 5. CSpace / CNode-Modell

**KO-140 (MUST):** Ein CSpace ist ein Baum von CNodes; jeder CNode ist ein Slot-Array fester Größe (2^L Slots, L ∈ {5, 6} pro Konfiguration). Slot-Index = Capability-Handle innerhalb des CSpace.

**KO-141 (MUST):** Pro Prozess existiert genau EIN CSpace-Root. Der Kernel hält keine globale Capability-Tabelle als Autorisierungsquelle (K29-`CapabilityTable` ist Übergangsimplementierung — siehe Alignment).

**KO-142 (MUST):** `copy`/`move`/`revoke`/`delete` operieren auf Slots; `revoke` kaskadiert über den Derivationsbaum unter dem Ziel-Slot.

**KO-143 (MUST NOT):** Zwei Prozesse DARFEN NICHT in denselben CSpace schreiben (Isolation; geteilte Sub-CSpaces nur read-only für eine Seite).

---

## 6. UntypedMemory & Retype

**KO-150 (MUST):** `UntypedMemory` ist die einzige Quelle neuen getypten Speichers. `retype` erzeugt daraus: `Frame`, `PageTable`, `CNode`, `AddressSpace`.

**KO-151 (MUST):** Retype verbraucht Kapazität deterministisch (keine Überbuchung: `consumed ≤ total`).

**KO-152 (MUST NOT):** Physischer Speicher DARF NICHT ohne Umweg über `UntypedMemory`-Retype einem User zugänglich werden — außerhalb der Boot-Initial-Caps.

---

## 7. Fehlermodell (ABI-stabil)

**KO-160 (MUST):** Einheitlicher Fehlercode-Raum (u32), grob partitioniert:

| Range | Bereich |
|---|---|
| 0 | `Ok` |
| 0x01–0x0F | generisch (InvalidArgument, OutOfMemory, …) |
| 0x10–0x1F | Capability-Fehler (InvalidObject, InsufficientRights, NoGrant, SlotFull, …) |
| 0x20–0x2F | IPC (EndpointClosed, WouldBlock, QueueFull, …) |
| 0x30–0x3F | Memory (RetypeExhausted, FrameInUse, …) |
| 0x40–0x4F | Thread/Process |
| 0x50–0x5F | IRQ/Device/Timer |

- **KO-161 (MUST):** Fehlercodes sind ABI — nach Freeze nicht mehr umnummerierbar; neue Codes nur in reservierte Ränge.
- **KO-162 (MUST):** Rust-Referenznotation: `KernelError`-Enum mit `code(self) -> u32` und `from_code(u32)` (vollständig, keine Lücken interpretierter Codes).

---

## 8. Determinismus- & Sicherheits-Invarianten

- **KO-170 (MUST):** Alle Objekt-Operationen laufen ausschließlich kernelseitig; Userspace sieht nur Handles (SC-ARCH-009).
- **KO-171 (MUST):** Der Kernel hält zu jedem Zeitpunkt den vollständigen Derivationsbaum konsistent (kein zyklisches Delegieren — Delegation ist baumförmig).
- **KO-172 (MUST NOT):** Der Kernel DARF KEINE Objekte erzeugen, die von Userspace-Komponenten abhängen (SC-ARCH-010).

---

## 9. Testvektoren (Pflicht bei Freeze; erste Belegung)

| TV-ID | Regel | Kern |
|---|---|---|
| TV-KO-101 | KO-101 | Alle 11 Typen erzeugbar; Code-Reserve unverändert |
| TV-KO-111 | KO-111 | Index-Wiederverwendung nur mit Generation+1 |
| TV-KO-112 | KO-112 | Zugriff auf retired Handle ⇒ InvalidObject |
| TV-KO-121 | KO-121 | Letzte Cap gelöscht ⇒ Objekt Retired |
| TV-KO-131 | KO-131 | K29-Rights-Bits unverändert (u16-Kompat) |
| TV-KO-132 | KO-132 | Derivation attenuiert nie erweiternd |
| TV-KO-134 | KO-134 | Badge transportiert, autorisiert nicht |
| TV-KO-141 | KO-141 | Zwei Prozesse, getrennte CSpace-Roots, keine Kreuz-Sicht |
| TV-KO-142 | KO-142 | Revoke kaskadiert nur im Teilbaum |
| TV-KO-150/151 | KO-150/151 | Retype-Arten + Kapazität exhaustiv |

---

## 10. Offene Entscheidungs-Punkte (DRAFT — hier schärfen wir gemeinsam)

| ID | Frage | Empfehlung Aurora | Alternative |
|---|---|---|---|
| **SC-DEC-A** | ID-Schema: typisiert 64-Bit-Split (8/24/32) oder einfacher u64? | Split (O(1)-Dispatch, ABA-Schutz) | simpler Zähler (K29-nah) |
| **SC-DEC-B** | Rights u16 mit SEND/RECEIVE/CALL/GRANT (Bits 4–7) — so erweitern? | Ja; Bits 0–3 K29-kompatibel | 2 separate Rights-Wörter (objekt-/IPC-getrennt) |
| **SC-DEC-C** | CSpace-Modell: CNode-Baum ab v0.1 oder K29-Flat-Table als v0.1-Zustand? | CNode-Baum als Ziel, K29 als Übergang (Alignment-Tabelle) | Flat-Table v0.1, Baum v0.2 |
| **SC-DEC-D** | Badge opak 64 Bit? | Ja (seL4-Modell) | strukturiert (Flags+Subjekt-Feld) |
| **SC-DEC-E** | Fehlercodes: flacher u32-Raum mit Ranges? | Ja (ABI-stabil) | i32-errno-Modell à la POSIX (verboten? — Projekt ist non-POSIX) |
| **SC-DEC-F** | Generation: 24 Bit reicht? (16,7M Objekte/Typ/Index-Slot) | Ja für v0.1 | 16 Bit Generation + 32 Index (mehr Objekte) |

---

## 11. Alignment: K29-Implementierung ↔ v0.1-Spezifikation (Übergang)

| K29 (`capability.rs` heute) | v0.1-Spez | Migrationspfad |
|---|---|---|
| `Rights(u8)`, 4 Bits | `Rights(u16)`, 8 Bits | Bit 0–3 unverändert; neue Bits 4–7 additiv |
| `Capability{id, resource_type, resource_id, rights, owner, parent}` | `{target, rights, badge, deriv}` | `resource_id`→`target`; `owner`→CSpace-Zugehörigkeit; `parent`→`deriv`; `badge` NEU |
| `CapabilityTable` (global, by Pid) | CNode-Baum pro Prozess | v0.1: Table als Flach-Emulation eines CSpace-Root; CNode-Struktur danach |
| Kaskadierendes `revoke` (existent) | KO-142 | bereits konform ✅ |
| `delegate` mit Attenuation (existent) | KO-132 | bereits konform ✅ |
| `resource_type` als Enum | KO-101 Codes | Codes als numerische Repräsentation derselben Enum |

---

*Nächster Schritt nach SC-001-Freeze: SC-002 Capability Model (Operationen: copy/move/mint/revoke/delete-Semantik, Slot-Verwaltung), SC-003 CSpace/Derivation.*

# ShivaCore v0.1 — Architektur-Gate (VERBINDLICH)

> **Status:** VERBINDLICHER ARCHITEKTUR-GATE (AD-013) · **Datum:** 06.09.2026 · **Revision:** v1.1
> **Basierdokument:** `SHIVACORE_KERNEL_ARCHITECTURE.md` (AD-012)
> **Technische Spezifikation:** `../kernel-spec/SHIVACORE_SPEC_V010.md` (SC-001 Kernel Object Model, DRAFT_REVIEW)
> **Geltung:** Alle SC-ARCH-Regeln sind normativ (Freeze) und müssen vor jeder
> Kernel-Implementierung geprüft werden. Ausnahmen nur über DECISIONS_REGISTER.
> **v1.1-Changelog:** Freeze-Regeln mit Begründung + Durchsetzung ergänzt; Initial-Task-Boot
> detailliert; Objektbeziehungen, IPC-Semantik, Domain-Zuweisungstabelle, Sandbox-Matrix
> und Akzeptanzkriterien je SC-Schritt ergänzt.

## Architektur-Gate (Kernaussage)

> **ShivaCore ist ein echter Capability-Microkernel.
> Globus OS ist das darauf aufbauende Userspace-Betriebssystem.**

Zweck: Vermeidung einer schleichenden Vermischung von Kernel- und OS-Funktionen.

**Trennung (verbindlich):**
ShivaCore = Trust Base · Globus OS = OS-Layer · ATCLang = Native System/Application Language ·
Aurora = AI Platform · A-TownChain = Blockchain Platform · Genesis = Interactive/Game Platform.

---

## 1. Zielarchitektur (verbindlich)

```
┌─────────────────────────────────────────────────────────────────────┐
│                         APPLICATION SPACE                           │
│  Genesis │ Aurora │ Music │ Video │ 3D │ Office │ Browser │ Apps    │
├─────────────────────────────────────────────────────────────────────┤
│                         RUNTIME SPACE                               │
│  ATCLang Runtime │ WASM │ POSIX Layer │ Container │ Game Runtime    │
├─────────────────────────────────────────────────────────────────────┤
│                       GLOBUS OS SERVICES                            │
│  Init │ VFS │ Network │ GPU │ Audio │ USB │ Input │ Window │ AI    │
│  A-TownChain │ Wallet │ Crypto │ Package │ Update │ Logging        │
├─────────────────────────────────────────────────────────────────────┤
│                       SHIVACORE MICROKERNEL                         │
│  Scheduler │ IPC │ CSpace │ Memory │ Threads │ Interrupts │ Syscalls │
│                     Capability Security │ HAL                       │
├─────────────────────────────────────────────────────────────────────┤
│                         HARDWARE / HAL                              │
│          x86_64 │ AArch64 │ RISC-V │ CPU │ RAM │ MMIO │ DMA        │
└─────────────────────────────────────────────────────────────────────┘
```

**Schichtregel (MUST):** Abhängigkeiten zeigen NUR nach unten. Ein Layer DARF NICHT
Schichten über sich referenzieren. Die HAL ist Grenze zum Kernel; der Kernel ist
Grenze zu Services; Services zu Runtimes; Runtimes zu Applications.

---

## 2. Capability-Modell (Kern-Korrektur, normativ)

```
Capability Handle → CSpace Slot → Kernel Capability Object → Object + Rights
```

**Die Unfälschbarkeit entsteht durch Kernel-enforced Handles, CSpace-Isolation und
Objektverwaltung — NICHT durch Kryptografie.**

| Ebene | Mechanismus | Schutzziel |
|---|---|---|
| Handle | Kernel-vergebener Slot-Index (kein Pointer) | Userspace kann keine Capability fälschen oder raten |
| CSpace | Pro Prozess EIN CSpace-Root (CNode-Baum) | Isolation zwischen Prozessen |
| Kernel Capability Object | Referenz + Rights + Badge + Derivation | Autorisierung + Delegation |
| Kryptografie (später) | DID/Signaturen für Cross-Domain | Nur für persistente/überschreitende Delegation |

*Implementierungs-Delta:* `capability.rs` (K29) ist handle-basiert ✅;
`did.rs`/`remote_caps.rs` sind die getrennte Cross-Domain-Erweiterung ✅.

---

## 3. Root Server: globus-init (Kern-Korrektur, normativ)

`globus-init` ist **NICHT** „Process 0 mit Unix-Root-Rechten".

```
ShivaCore → Initial Task
            ├── CSpace Root          (kompletter Capability-Baum, boot-exklusiv)
            ├── Initial Address Space
            ├── BootInfo              (Speicherkarte, Geräte, Boot-Modus)
            ├── Untyped Memory Caps   (gesamter freier physischer Speicher)
            ├── IRQ Caps              (alle Interrupts, ungebunden)
            └── Device Caps           (MMIO/Ports, inkl. Secure RNG)
                 ↓
            globus-init
```

**globus-init startet (verbindliche Reihenfolge):**
1. Memory Manager Service (aus Untyped Caps: Frames, PageTables, CNodes)
2. Process Manager Service (Thread-Erzeugung für weitere Services)
3. Driver Manager (IRQ/Device-Caps an Treiber-Services delegieren)
4. VFS + Network
5. Security Manager
6. Service Manager (Start verbleibender Globus-OS-Services)

**MUST:** Nach jedem Schritt gibt globus-init nur die minimal benötigten Caps an den
jeweiligen Service weiter (Least Privilege). Root-Caps verbleiben im Initial Task;
sobald alle Services laufen, verwirft globus-init seine letzten exklusiven Caps
(Boot-Deprivilegierung).

---

## 4. Kernel Object Model (zentraler Baustein)

### 4.1 Objekttypen

```
Kernel Object
│
├── Thread          ├── Endpoint        ├── IRQ
├── AddressSpace    ├── Notification    ├── Device
├── PageTable       ├── CNode           ├── UntypedMemory
├── Frame           └── Timer
```

### 4.2 Objektbeziehungen (NEU, verbindlich)

```
AddressSpace ──besitzt──► PageTable ──mappt──► Frame
Thread ──gehört zu──► AddressSpace
Thread ──blockt auf──► Endpoint | Notification | IRQ | Timer
Endpoint / Notification ──verbindet──► Threads (n:m)
CNode ──enthält Slots──► Capabilities (auf alle Objekttypen)
UntypedMemory ──retype──► Frame | PageTable | CNode | AddressSpace
IRQ ──gebunden an──► Notification (IRQ-Event-Kanal)
Timer ──signalisiert──► Notification (Timeout-Kanal)
Device ──referenziert──► physische Ressource (MMIO/DMA via Frame-Caps)
```

**MUST:** Nur diese Beziehungen sind erlaubt. Insbesondere DARF ein Thread nicht
direkt einen Frame referenzieren — nur über ein Memory-Mapping (AddressSpace+PageTable)
mit zugehöriger Frame-Cap.

### 4.3 Capability-Struktur

```
Capability
├── Object Reference     ├── Badge
├── Rights                ├── Type
└── Derivation Metadata
```

*Detail-Spezifikation (IDs, Codes, Rights-Bits): siehe `SHIVACORE_SPEC_V010.md` —
dort stehen auch die offenen Entscheidungs-Punkte SC-DEC-A…F.*

---

## 5. Capability Derivation Tree (Delegation)

```
ROOT CAP
   ├── Network → NetServer → App A, App B
   └── Storage → VFS → App A, App C
```

**Regeln (MUST):**
- Delegation ist baumförmig (keine Zyklen).
- Derivation attenuiert NUR (Rights-Teilmengen); Erweiterung nur kernel-seitig (GRANT).
- `revoke` kaskadiert ausschliesslich im Teilbaum unter dem Ziel-Slot.
- Beispiel GPU: Root → GPU Service → Genesis → Render Worker erhält
  `GPU_FRAMEBUFFER`, `GPU_COMMAND_QUEUE`, `GPU_DMA_BUFFER` — **keinen** Zugriff auf
  beliebigen physischen Speicher.

---

## 6. IPC als Bus des Betriebssystems

```
Aurora AI →(IPC)→ AI Manager →(IPC)→ GPU Service →(IPC)→ GPU Driver → Hardware
```

Gleiches Modell für: App→VFS, App→Network, App→Audio, App→Camera,
App→Blockchain, App→Wallet, App→AI.

### 6.1 IPC-Semantik (NEU, verbindlich)

| Objekt | Semantik | Blocking | Nachrichten |
|---|---|---|---|
| **Endpoint** | Synchron (send/receive/call) | ja | gepointerte, typisierte Messages über Kernel-Copy |
| **Notification** | Asynchron (Signal-Wort, 64 Bit) | nein | Bit-Flags, kein Payload (Payload via geteilten Frame) |

**MUST:** `call` = send+block+reply-Endpoint (RPC-Primitive, Reply-Cap wird vom
Kernel automatisch erzeugt und nach Antwort invalidiert).

---

## 7. Shared Memory als Performance Plane

```
CONTROL PLANE:  IPC Message · Capability · Command · Event · Metadata
DATA PLANE:     Shared Memory · DMA · Ring Buffer · Zero-Copy · Mapping
```

Beispiel Aurora: IPC → Model Service (Kontrolle), Shared Memory → Tensor Buffer
→ GPU Service (Daten).

**MUST:** Data-Plane-Mappings sind reguläre Frame-Caps (Rights READ/WRITE,
DELEGATE nur bei dokumentierter Notwendigkeit). Der Kernel kennt keine
„implizit geteilten" Regionen.

---

## 8. Scheduler: Scheduling Domains

```
HARD REALTIME · SOFT REALTIME · INTERACTIVE · SYSTEM · NORMAL · BACKGROUND
```

### 8.1 Verbindliche Domain-Zuweisung (NEU)

| Komponente | Domain |
|---|---|
| Audio Engine | SOFT REALTIME |
| Input | INTERACTIVE |
| Desktop / Window Manager | INTERACTIVE |
| Kernel-Haushaltsaufgaben | SYSTEM |
| Blockchain Node (A-TownChain) | NORMAL |
| Aurora Training | BACKGROUND |
| Mining | BACKGROUND |

**MUST:** Realtime ist eine eigene Domain, keine „höchste normale Priorität".
Innerhalb einer Domain ist MLFQ für v0.1 zulässig; Domains haben feste
Budgetgrenzen (ein BACKGROUND-Task kann durch Prioritäts-Inversion nie
eine RT-Deadline gefährden).

*Implementierungs-Delta:* `scheduler.rs` (K29, DA-HEFT) ist der
Accelerator-/Lastverteilungs-Scheduler und bleibt davon getrennt.

---

## 9. Harte Architekturregel: Keine KI-Logik im Kernel

```
SHIVACORE MUST NOT depend on: Aurora AI · DefenderGPT · A-TownChain · Genesis
                              · ATCLang compiler · GUI
```

Richtung AUSSCHLIESSLICH: Applications → Services → ShivaCore → Hardware.
Niemals: ShivaCore → Aurora AI → Blockchain.

**Begründung:** Der Kernel MUSS bootstrapping-fähig und deterministisch bleiben.
**Durchsetzung (NEU):** Kernel-Crate-Abhängigkeitsliste ist Teil des CI-Gates:
`cargo tree -p shivacore` wird gegen eine Allowlist geprüft (core-Primitive
ed25519, sha3, rand — keine AI/Chain/GUI-Crates). Verstoß = Build-Fail.

---

## 10. Security Architecture

```
Capability Isolation · Memory Isolation · Process Isolation
            ↓
     Service Isolation
            ↓
   Application Sandbox
```

### 10.1 Sandbox-Matrix (NEU, verbindlich)

| Sandbox | Enforced durch | Kernel-Rechte des Sandbox-Inhalts |
|---|---|---|
| AI Sandbox (Aurora) | Capability-Mindestsatz + Telemetry | nur zugewiesene Caps; keine impliziten |
| Blockchain Sandbox (ATC-Node) | isolierter AddressSpace, Caps via ATC System Interface | keine impliziten |
| ATCLang Sandbox | Syscall ABI als EINZIGE Schnittstelle | nur über Runtime verliehene Caps |
| Application Sandbox | Service-vergebene Untermengen | least privilege |
| Driver Sandbox | eigene Prozesse, IRQ/Device-Caps einzeln delegiert | nur eigene Geräte |

### 10.2 DefenderGPT-Governance (verbindlich)

```
DefenderGPT → Security Telemetry → ShivaCore Events
```

DefenderGPT DARF: beobachten, Muster erkennen, Empfehlungen abgeben.
DefenderGET DARF NICHT: selbstständig Enforcement-Entscheidungen treffen.
**Die Enforcement-Entscheidung bleibt deterministisch im Kernel** (Regelwerk,
keine Wahrscheinlichkeiten im Pfad der Autorisierung).

---

## 11. Syscall-Modell: Objektorientierte Kernel-ABI

```
TASK:       thread_create · thread_exit · thread_control
MEMORY:     map · unmap · protect · share
IPC:        send · receive · call · reply
CAPABILITY: copy · move · revoke · delete
IRQ:        bind · wait · ack
TIME:       sleep · timer_create
```

**MUST:** Jeder Syscall prüft zuerst die Capability (SC-ARCH-009); Fehler vor
Nebeneffekten (kein partieller Zustand bei `InsufficientRights`).

---

## 12. Boot Chain (final)

```
UEFI → Limine → ShivaCore (CPU/Memory/Interrupt/Scheduler/CSpace/IPC Init)
→ globus-init (Memory/Process/Driver Manager, VFS, Network, Security, Service Manager)
→ Globus OS (Window Manager, Aurora, ATCLang, A-TownChain, Applications)
```

### 12.1 Boot-Fehlverhalten (NEU, verbindlich)

| Fehlerphase | Verhalten (MUST) |
|---|---|
| Kernel-Init (CPU/Mem/IRQ) | PANIC + Fehlercode auf Konsole; kein Weiterbooten |
| globus-init abstürzig | Kernel läuft weiter; globus-init NEUSTART (SC-ARCH-005), max. N Versuche, danach Recovery-Konsole |
| Service abstürzig | Service-Neustart durch Service Manager; Kernel unberührt |

---

## 13. Architecture Freeze Rules (SC-ARCH-001 … SC-ARCH-010) — mit Begründung & Durchsetzung

| Regel | Inhalt | Begründung | Durchsetzung |
|---|---|---|---|
| SC-ARCH-001 | ShivaCore ist ein Capability-basierter Microkernel | Trust Base minimal halten; jede Funktion außerhalb ist austauschbar/neustartbar | Architektur-Review + Kernel-Crate-Umfang im CI |
| SC-ARCH-002 | Hardware-Treiber laufen außerhalb des Kernel-Adressraums | Treiberfehler dürfen Kernel nicht zerstören | Kein `unsafe`-Treibercode im Kernel-Crate; Device-Caps nur an Userspace |
| SC-ARCH-003 | IPC ausschließlich über Kernel-vermittelte IPC-Objekte | Einheitlicher, auditerbarer Kommunikationspfad | Alle Inter-Service-Pfade über Endpoint/Notification; CI-Grep auf Shared-Global-Statics |
| SC-ARCH-004 | Kein direkter Speicherzugriff ohne Memory-Cap-Mapping | Isolation ist die Kern-Sicherheitsgarantie | Mapping-Operationen nur über MEMORY-Syscalls mit Frame-Cap |
| SC-ARCH-005 | Services dürfen neu starten, ohne den Kernel neu zu starten | Verfügbarkeit; Update-Fähigkeit | Supervision in globus-init/Service Manager; Restart-Tests im CI |
| SC-ARCH-006 | AI-Systeme: keine impliziten Kernel-Rechte | KI bleibt beobachtend/beratend; deterministische Autorisierung | Capability-Vergabe-Audit: jede Cap an AI hat dokumentierte Quelle |
| SC-ARCH-007 | Blockchain-Komponenten: keine impliziten Kernel-Rechte | ATC-Node ist ein Service wie jeder andere | wie 006, zusätzlich Abhängigkeits-Check (Abschnitt 9) |
| SC-ARCH-008 | ATCLang: keine impliziten Kernel-Rechte | ATCLang erreicht den Kernel nur via Syscall ABI + Cap Check | ABI-Tests: ohne Cap kein Syscall-Erfolg |
| SC-ARCH-009 | Alle privilegierten Operationen über Capabilities autorisiert | Keine Root-Ausnahmen; volle Auditierbarkeit | Syscall-Layer: Cap-Check ist PFLICHT vor jeder Operation |
| SC-ARCH-010 | Der Kernel hängt nicht von Userspace-Komponenten ab | Bootstrapping-Fähigkeit, Determinismus | Kernel baut+testet STANDALONE im CI (ohne Services) |

---

## 14. Implementierungs-Reihenfolge (SC-001 … SC-013) mit Akzeptanzkriterien (NEU)

| Schritt | Inhalt | Akzeptanzkriterium (Freeze-relevant) |
|---|---|---|
| SC-001 | Kernel Object Model | Spez gefroren (SC-DEC-A…F entschieden); alle 11 Objekttypen mit Testvektoren |
| SC-002 | Capability Model | copy/move/mint/revoke/delete-Semantik mit TV; Attenuations-Nie-Erweiterung bewiesen |
| SC-003 | CSpace / Derivation | Zwei-Prozess-Isolation bewiesen; Revoke-Kaskade nur im Teilbaum |
| SC-004 | IPC Semantics | Endpoint+Notification-Semantik, call/reply-RPC, Badge-Transport; Deadlock-Freiheit für Kernel-Pfad |
| SC-005 | Memory Model | Untyped→Retype, Frame-Mapping, protect/share; SC-ARCH-004 erzwungen |
| SC-006 | Thread / TCB Model | Thread-States, FPU/Lazy-State, Kopplung AddressSpace |
| SC-007 | Scheduler | 6 Domains mit Budgetgrenzen; MLFQ innerhalb; RT-kann-nicht-Blockiert-werden-Test |
| SC-008 | Syscall ABI | Objektorientierte ABI (11), Fehlercodes ABI-stabil, Cap-Check vor Nebeneffekt |
| SC-009 | Boot Protocol | UEFI→Limine→Kernel-Init-PANIC-Kodes; Initial-Task-Cap-Set |
| SC-010 | Userspace Root Server | globus-init-Reihenfolge + Boot-Deprivilegierung; Restart (SC-ARCH-005) |
| SC-011 | Driver Model | Treiber als Prozesse (SC-ARCH-002), IRQ/Device-Delegation |
| SC-012 | Globus OS Service ABI | Service-Standardschnittstelle (VFS, Network, GPU, …) auf IPC-Basis |
| SC-013 | ATCLang ABI | ATCLang Runtime→Syscall-Pfad (SC-ARCH-008) end-to-end |

Erst DANACH Anbindung von Aurora, A-TownChain und Genesis.

**Offen vor SC-001-Freeze:** Entscheidungs-Punkte SC-DEC-A…F in
`SHIVACORE_SPEC_V010.md` Abschnitt 10 (ID-Schema, Rights-u16, CSpace-Timing,
Badge, Fehlercodes, Generation-Breite).

---

*Verwandt: `SHIVACORE_KERNEL_ARCHITECTURE.md` (AD-012) ·
`../kernel-spec/SHIVACORE_SPEC_V010.md` (SC-001-Entwurf, DRAFT_REVIEW) ·
`SHIVACORE_KERNEL_STATUS.md` (Ist-Stand) · `../DECISIONS_REGISTER.md` (AD-013)*

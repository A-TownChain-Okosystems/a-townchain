---
document_id: SC-001
title: "ShivaCore v0.1 Kernelspezifikation — Boot & Speicher"
version: 0.1.0-FROZEN
status: FROZEN v0.1.0 — Owner-Freigabe 03.10.2026 (Vorab-Regel: SC-DEC-A…F alle reversibel, Empfehlungen akzeptiert; Review bleibt bei SC-ARCH-001…010)
repository: atc-shivacore
layer: L1-Kernel
owner: A-TownChain-Okosystems / ShivaCore (Michael Wroblewski)
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-03
ad_refs: [AD-012, AD-013, AD-026, AD-027, AD-028]
depends: [G1-PASSED atclang, SHIVA-BOOT-001, SHIVA-HAL-001, SHIVA-ABI-001]
series: SC-001…SC-013 (v0.1.0-Kernelspezifikation, AD-013)
---

# SC-001 — Boot & Speicher (v0.1.0, FROZEN 03.10.2026)

> **Status:** FROZEN v0.1.0 per AD-013 — Owner-Freigabe 03.10.2026 über die
> Vorab-Regel (alle sechs Entscheidungen reversibel, keine mit Impact "groß";
> §12-Empfehlungen als Default akzeptiert, Review bei SC-ARCH-001…010). Diese Spec erhebt keinen
> Implementierungs-Status ("No status without evidence") — die heutige
> Boot-Sequenz L0–L10 (kernel_init, J-K10-Ära) ist eine In-Kernel-Testsequenz
> und KEIN echtes Booten (§11).

## 0. Freigabekette (AD-026, verifiziert 03.10.2026)

- **G1 Language Specification: PASS** — Anker `specs/VERSION.toml` im atclang-Repo:
  "G1-PASSED (2026-09-07, specs/language/SPEC.md + registry.json)". G2 (Semantics)
  accepted 2026-09-07. Beide Artefakte live verifiziert.
- **ABI (G7): 0.1.0-DRAFT** (ATC-ABI-001, SCR-0071, "normativ erst nach
  Spec-Freeze"). **Konsequenz:** Diese Spec trifft KEINE ATCLang-ABI-Annahmen.
  Die einzige Sprachbindung ist die AD-012-Grenzregel (Syscall-ABI objektorientiert
  + Capability-Check, nie direkter Kernel-Speicher); alle konkreten ABI-Details
  werden per Forward-Referenz auf G7/ATC-ABI-001 differiert.
- **M1 gesamt:** noch offen (End-to-End ATCA→Verifier→ATVM ausstehend; Rust
  Canonical Core incomplete, 1/21 Crates). SC-001 ist Spezifikationsarbeit und
  gem. AD-026 nach G1-Erreichung zulässig; Implementierung erfolgt erst GEGEN
  diese Spec (und die nachfolgenden SC-002…SC-013).

## 1. Zweck

Verbindliche Spezifikation der echten Bootchain und des Speichermodells des
ShivaCore-Microkernels (AD-013: Thread/AddressSpace/PageTable/Frame/CNode/
Endpoint/Notification/IRQ/Device/UntypedMemory/Timer). Verfeinert die
Boundary-Contracts SHIVA-BOOT-001 (Boot-Handoff), SHIVA-HAL-001 (HAL) und
SHIVA-ABI-001 (Userspace-Grenze); bei Konflikt gewinnt diese Spec bis zur
Versionierung, danach der jüngere Stand per Änderungsverfahren.

## 2. Bootchain (Phasen B0–B5, normativ)

```
B0  UEFI-Firmware       EFI-Boot, lädt Limine-Dateien
B1  Limine-Bootloader   Limine Boot Protocol, sammelt Plattform-Info
B2  ShivaCore-Entry     x86_64 Higher-Half-Entry, Early-Boot
B3  Kernel-Init         HAL, Objektmodell, Capability-Wurzel, Memcheck
B4  Initial Task        globus-init mit BootInfo + initialer CSpace
B5  Globus OS Userspace erste Services
```

- **B0/B1:** Limine liefert per Protokoll-Requests: Memory Map (usable/
  reserved/ACPI/…), HHDM-Basis, CPU-Topologie, RSDP/ACPI, Framebuffer nur bei
  expliziter Freigabe (SHIVA-BOOT-001: "framebuffer information only when
  explicitly enabled"), boot-Module, immutable Command line.
- **B2:** Entry im Higher Half; keine Page-Abschaltung vor Early-Paging;
  Serial-Debug optional als Fallback-Ausgabeweg.
- **B3:** kernel_init in dieser Reihenfolge: HAL → Objektmodell-Registry →
  Capability-Wurzel (CSpace der Initial Task) → Speicher (Frames/Untyped aus
  Memory Map) → Memcheck (J-K09: Boot + Rolling) → Scheduler-Grundlast →
  Initial-Task-Ready.
- **B4:** Der Kernel startet GENAU EINEN privilegierten Startprozess:
  globus-init. KEIN Unix-Root, kein implizites Recht (AD-013).
- **B5:** Alle weiteren Prozesse entstehen ausschließlich durch Capability-
  Delegation aus der Initial-Task-CSpace.

## 3. BootInfo & Memory Map (REQ-SC001-01…04)

- **REQ-SC001-01 (MUST):** BootInfo ist eine versionierte, read-only Struktur
  an die Initial Task: BootInfo-Version, Memory-Map-Array, HHDM-Basis,
  CPU-Topologie, ACPI-Zeiger, Framebuffer-Deskriptor (optional), Modul-Tabelle,
  Boot-Log-Handle.
- **REQ-SC001-02 (MUST):** Jeder Memory-Map-Eintrag führt: Basis, Länge,
  Typ (USABLE/RESERVED/ACPI_RECLAIMABLE/ACPI_NVS/BAD/HW_RESERVED/PRESERVE),
  und Exklusivitäts-Anspruch des Kerns (Kernel-Image, initiale Strukturen).
- **REQ-SC001-03 (MUST):** Nach B3 gehört JEDER physische Frame entweder dem
  Kernel (Control) oder ist als UntypedMemory-Capability klassifiziert.
  Herrenloser Speicher ist ein Invariantenverstoß (INV-02).
- **REQ-SC001-04 (MUST):** BootInfo selbst liegt in Kernel-kontrollierten
  Frames; die Initial Task erhält read-only Capability, keine Schreibrechte.

## 4. Speichermodell

### 4.1 Physisch
- Granularität 4 KiB (Frame). Größere Allokationen = Mehrfach-Frames.
- Kernel-Block-Allokator (J-K09) für Kernel-eigene Strukturen, Budget max.
  48 MiB; Memcheck bei Boot, Rolling Updates, Slub für kleine Objekte.
- HW-reservierte Regionen werden nie in Untyped überführt.

### 4.2 UntypedMemory (REQ-SC001-05…07)
- **REQ-SC001-05 (MUST):** Alle USABLE-Regionen abzüglich Kernel-Reserve
  werden als UntypedMemory-Caps an die Initial Task delegiert (Umfang:
  siehe SC-DEC-D).
- **REQ-SC001-06 (MUST):** Retype-Kette Untyped → Frame / PageTable / CNode /
  andere Kernel-Objekte ist ausschließlich capability-vermittelt; jede
  Retype-Operation konsumiert exakt die typisierte Menge. Über-Retyp =
  INV-Verstoß.
- **REQ-SC001-07 (MUST):** Retyp ist irreversibel; Freigabe nur als Retype
  zurück (Reset des Objekts) mit Kernel-Memcheck der Zielregion.

### 4.3 Virtuell (REQ-SC001-08…10)
- **REQ-SC001-08 (MUST):** AddressSpace-Objekt = Wurzel-PageTable-Cap; nur
  mit AddressSpace-Cap mappable. Higher-Half-Layout: Kernel oben (Einheit
  siehe SC-DEC-B), Userspace unten.
- **REQ-SC001-09 (MUST):** Mapping NUR Frame→AddressSpace über Capability;
  kein anonymes mmap, kein fd (AD-013: POSIX-frei). Device-MMIO ausschließlich
  über Device-Caps an privilegierte Services.
- **REQ-SC001-10 (MUST):** PageTable-Einträge ohne Frame-Cap-Referenz sind
  verboten (INV-04); Unmap widerruft implizit alle abgeleiteten Mappings.

## 5. Initial Task & Capability-Ableitung (REQ-SC001-11…13)

- **REQ-SC001-11 (MUST):** globus-init erhält: BootInfo-Read-Cap, CSpace-Wurzel
  (Scope initial), Untyped-Caps je USABLE-Region, mindestens einen Endpoint zur
  Kernel-Debug/Control-Schnittstelle, Timer-Cap, CPU-Notification-Caps.
- **REQ-SC001-12 (MUST):** "Kein Unix-Root" = die Initial Task hat KEINE
  impliziten Rechte; jedes Recht ist eine Capability in ihrer CSpace. Kernel
  behält die Wurzel-CSpace-Kontrolle (INV-01).
- **REQ-SC001-13 (MUST):** Andere Boot-Wege (zweiter Initial-Prozess, Kernel-
  Shell, fest verdrahtete Services) existieren nicht.

## 6. Schnittstellen

- **HAL (SHIVA-HAL-001):** B2/B3 nutzen ausschließlich HAL-Primitive
  (cpu/memory/interrupt/timer/serial); kein direkter Hardwarezugriff im
  Kern-Code.
- **Syscall-ABI:** objektorientiertes Interface per AD-012/SHIVA-ABI-001;
  konkrete ABI-Layouts = ATC-ABI-001 (G7, DRAFT) — hier nur Forward-Ref.
- **Scheduler:** Domains HardRT…Background per AD-013; DA-HEFT getrennt.
  Details in SC-00x (Scheduling-Spez, folgt).

## 7. Invarianten (Kernel-checkbar, MUST)

INV-01 Kernel behält Kontrolle über die Capability-Wurzel-Struktur.
INV-02 Kein herrenloser physischer Speicher nach B3.
INV-03 Jedes Mapping hat eine Frame-Cap-Herkunft.
INV-04 Kein PageTable-Eintrag ohne Frame-Cap-Referenz.
INV-05 BootInfo ist nach B3 unveränderlich.
INV-06 Untyped-Verbrauch ist konservativ (nie überschritten).
INV-07 Kernel-Speicherbudget (48 MiB) wird zur Laufzeit nicht überschritten.
INV-08 Initial Task ist der einzige Boot-prozess.
INV-09 Capability-Revocation kaskadiert (J-K-Bestand aus capability.rs).

## 8. Fehlerfälle (Klassen B-E01…, MUST-behandelbar)

B-E01 Keine/garstige Memory Map → Boot-STOP mit Serial-Diagnose.
B-E02 Kein USABLE-Speicher über Mindestschwelle → Boot-STOP.
B-E03 Defektes/fehlendes Boot-Modul → Boot-STOP.
B-E04 Framebuffer fehlt trotz Freigabe → Serial-Fallback, kein Stopp.
B-E05 Capability-/Untyped-Erschöpfung in B3 → Boot-STOP mit Diagnose.
B-E06 Retype-Konflikt/zu kleine Region → Region-Skip + Diagnose.
B-E07 Memcheck-Fehler bei Boot → Boot-STOP (J-K09-Regel).
B-E08 Initial Task crasht → Kernel-Panic mit Boot-Log-Handle.

## 9. Testbarkeit (MUST)

- **T1 QEMU+Limine-Smoke:** B0–B5 Ende-zu-Ende (Ziel-M2/M5-Gate, AD-027),
  assertion auf INV-01…09.
- **T2 In-Kernel-L0–L10:** bleibt Testsequenz (kernel_init) — prüft
  Subsystem-Init, NICHT die Bootchain (siehe §11).
- **T3 Unit:** Frame/Untyped/Retype/PageTable/CSpace-Objekte.
- **T4 Integration:** Retype→Map→Unmap→Revocation-Kaskade;
  Initial-Task-BootInfo-Read.
- **T5 Fault-Injection:** B-E01…08 je mindestens ein Test.

## 10. Meilenstein-Zuordnung (AD-027)

M2 (Kernel läuft): T2/T3 grün. M5 (OS läuft): T1 B0–B5 + globus-init +
erste Userspace-Services. Kein M5-Gate ohne T1.

## 11. Explizite Abgrenzung — L0–L10 ist KEIN Booten

Die bestehende BootPhase-Sequenz L0–L10 (kernel_init.rs, K-/J-Sprints,
674/674 Tests) ist eine **In-Kernel-Test- und Initialisierungssequenz**
hinter dem x86-boot-Feature-Flag. Sie bootet KEIN Gerät: kein UEFI, kein
Limine, kein globus-init, kein Userspace. Die ECHTE Bootchain ist B0–B5
dieser Spezifikation und existiert heute nur als Dokumentation. Jede
Aussage "Kernel bootet" ohne B0–B5-Evidenz ist ein Reality-Check-Verstoß.

## 12. Entscheidungsprotokoll SC-DEC-A…F (Owner-Freigabe 03.10.2026, Vorab-Regel)

| ID | Frage | Entscheidung (Empfehlung akzeptiert) | Reversibel | Impact | Begründung |
|---|---|---|---|---|---|
| A | Seitengröße v0.1 | **4 KiB only** — HugePages als Erweiterung in SC-003+ (Memory-Objekt-Spez) | ja | klein | Kein echter Bedarf vor M5; HugePages nur für große Mappings/DMA; nachrüstbar ohne INV-Bruch, weil nur Addition |
| B | Higher-Half-Basis | **Limine-HHDM-Konvention** — Basis aus dem Limine-Protokoll abgeleitet, zur Implementierung als Build-Konstante fixiert | ja | mittel | Kein Boot-Code existiert, Umzug ist reine Konstanten-Änderung; eigene Adresse erfindet Kompatibilitätsrisiko gegen Limine |
| C | Retyp-Granularität | **4 KiB + Untyped-Splitting erlaubt** | ja | klein | 4 KiB deckt Frames/PageTables; Splitting erlaubt Teil-Retyp ohne Fragmentierungszwang |
| D | Initial-Umfang | **Volle USABLE-Untype-Delegation an globus-init** | ja | mittel | INV-06 schützt vor Über-Retyp; Kernel-Reserve bleibt Festbetrag; "Kernel behält Anteil" bricht REQ-SC001-12-Geist (keine impliziten Rechte) schwächer nicht |
| E | SMP-Start | **Gegated** — Initial Task weckt CPUs per Notification | ja | mittel | Bestimmt deterministisches Boot; gestaffelter Start schafft Rennen in B3; Domain-Trennung (AD-013) verlangt explizite Freigabe |
| F | Framebuffer | **Device-Cap an globus-init** | ja | klein | Folgt SHIVA-BOOT-001 ("only when explicitly enabled"); MMIO-Umweg über HAL verletzt Capability-Klarheit |

> Freigabe-Modus: Owner-Vorab-Regel vom 03.10. — reversible Entscheidungen mit
> Empfehlung gelten als akzeptiert ("Owner accepted recommendation, review at
> SC-ARCH"). Keine der sechs Entscheidungen ist irreversibel oder Impact "groß".

## 13. Referenzen

AD-012 (Kernel-Reinheit), AD-013 (Microkernel-Architektur, Objektmodell),
AD-026 (Layer-Reihenfolge, G1→SC-001), AD-027 (M2/M5-Kriterien),
AD-028 (Service-Space-Migration), atclang specs/VERSION.toml (G1-PASSED,
G2 accepted), specs/abi/SPEC.md ATC-ABI-001 (DRAFT, SCR-0071),
SHIVA-BOOT-001, SHIVA-HAL-001, SHIVA-ABI-001, J-K09 (Memory Manager,
Memcheck), J-K10 (Interface-Protokolle), kernel_init.rs (L0–L10),
docs/roadmap/LAUFFAEHIGKEITS_ROADMAP.md (M1/M2/M5).

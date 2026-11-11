# AD-EXEC-KERNEL-ENFORCEMENT — Code Admission: Durchsetzung im Kernel

> **Status:** ENTSCHIEDEN (Owner, 11.11.2026) · **Geltung:** verbindlich für alle Pfade, die ausführbaren Speicher erzeugen
> **Betroffene Repos:** `globus-os` (Enforcement), `a-townchain` (`components/exec-admission`)

## Entscheidung

**ARCHITECTURE DECISION: KERNEL ENFORCEMENT.**

- Autoritative Entscheidung: ShivaCore Kernel.
- Enforcement: unmittelbar vor der Freigabe ausführbaren Speichers, also bevor ELF-Code in einen
  ausführbaren Prozessadressraum gelangt und bevor die CPU die erste Instruktion abrufen kann.
- Nutzer-Loader: nicht vertrauenswürdiger Vorbereitungspfad. Er darf lesen und aufbereiten,
  nicht entscheiden.
- Entscheidungslogik: Wiederverwendung von `components/exec-admission`. Kein zweites
  Admission-System im Kernel.
- Protokollierung: sechs getrennte Prüfstufen mit eigenständigen Fehlergründen.
- Sicherheitsprinzip: fail-closed.
- Umgehung über alternative Pfade zu ausführbarem Speicher: unzulässig.

## Begründung in einem Satz

Ein Nutzer-Loader ist ersetzbar, umgehbar und nicht Teil der TCB; nur der Kernel kann
verhindern, dass ausführbarer Speicher ohne Autorisierung entsteht.

## Geltungsbereich: jede Stelle, nicht nur ELF

Die Entscheidung gilt für **alle** Wege zu ausführbarem Speicher:

1. ELF-Laden in einen Prozessadressraum.
2. `mmap` mit Ausführungsrecht (heute nicht vorhanden, Vorgriff verbindlich).
3. Nachträgliche Rechteänderung von Speicherseiten (heute: `update_flags`).
4. Dynamisches Laden ausführbarer Module (heute nur Simulation, siehe Inventur).
5. JIT und generierte ausführbare Bereiche.
6. Alternative Loader und direkte Kernel-Schnittstellen.

Für 4 und 5 sind später eigene Admission-Profile zu definieren. Sie sind keine pauschale
Ausnahme vom Grundprinzip.

## Was der Kernel nicht glauben darf

- **Kein vom Aufrufer gelieferter Hash.** Der Kernel berechnet den Hash über genau die Bytes,
  die er abbildet, und vergleicht ihn mit der Freigabe. Ein `observed_hash` aus dem
  Syscall-Argument ist eine Behauptung, kein Nachweis (TOCTOU).
- **Keine Signatur als Recht.** Eine gültige Signatur erteilt keine Capabilities. Rechte
  folgen aus Identität und Ausführungskontext, nicht aus der Signaturprüfung.
- **Keine Reihenfolge-Verletzung.** Erst identifizieren, dann entscheiden, dann abbilden.
  Ein fehlgeschlagener Schritt darf keinen teilweise ausführbaren Prozess hinterlassen.

## Folge für die TCB

Mit dieser Entscheidung ist der Admission-Pfad Teil der Trusted Computing Base. Fehler in
diesem Pfad oder in der Schlüsselverwaltung sind Fehler der gesamten Vertrauensgrenze. Daraus
folgt: der Pfad bekommt eigene Negativtests, eine eigene Freigabe und keine Ausnahmen.

## Abnahmekriterien des ersten Schnitts

`TESTED` erst, wenn die elf Gegenproben des Owners bestehen: autorisiertes ELF läuft;
unbekannter Herausgeber, manipuliertes ELF, widerrufener Schlüssel, abgelaufene oder
zurückgerollte Version, fehlende Capability werden je mit eigenem Grund abgelehnt; ein
direkter Loader-Bypass wird vom Kernel verhindert; eine direkte Ausführungsanforderung
verlangt Admission; eine Änderung zwischen Prüfung und Laden wird abgelehnt; ein Fehler in
einer Stufe führt zu keiner Freigabe; das Protokoll nennt je Stufe einen eigenen Grund.

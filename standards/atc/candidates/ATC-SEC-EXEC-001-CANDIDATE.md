# ATC-SEC-EXEC-001 — Zero Trust Code Admission & Execution Control

> **Standard-ID:** ATC-SEC-EXEC-001
> **Kategorie:** Security / Execution Control
> **Tier:** 0 (Kandidat für foundational)
> **Status:** KANDIDAT — **nicht normativ.** Keine Implementierungspflicht, keine Registry-Bindung, keine Ratifizierung.
> **Version:** 0.1.0-draft
> **Datum:** 09.11.2026
> **Autor:** Michael Wroblewski (Owner-Vorgabe), Entwurf durch Aurora
> **Entscheidung:** offen (Ratifizierung durch Owner, danach Übernahme nach `atc-standards`)
> **Grundlage:** `docs/audits/ZERO-TRUST-CODE-EXECUTION-BASELINE-2026-11-09.md` (Ist-Stand, read-only)
> **Abhängigkeiten:** ATC-03 (DID / Zero-Trust IAM), ATC-14 (Deterministic Execution), ATC-LIC (License Gate), ATC-BC-001 (Opcode-Satz), SHIVA-BOOT-001, SHIVA-SYSCALL-ABI, SHIVA-ABI-001

---

## 1. Zweck

Dieser Standard regelt verbindlich, unter welchen Bedingungen ausführbarer Code im
A-TownChain-/KAI-OS-Ökosystem **aufgenommen und ausgeführt** werden darf. Er regelt
ausdrücklich **nicht**, wie Code entsteht, wie er lizenziert wird und wie er
ökonomisch abgerechnet wird.

Drei Prinzipien sind unveränderlich und nicht abwägbar:

1. **No Trust by Presence** — Vorhandensein von Code ist keine Ausführungsberechtigung.
2. **No Execution without Authority** — keine Ausführung ohne explizite, prüfbare Autorisierung.
3. **No Privilege without Capability** — keine Ressource ohne nachgewiesene Berechtigung.

**Geltungsbereich:** jede Stelle, an der Bytes zu Anweisungen werden. Nativ (ELF),
virtuell (ATVM-Bytecode), nachgeladen (Treiber, Service Space), erzeugt (Genesis,
JIT) und delegiert (AI-Tool-Aufrufe).

**Nicht im Geltungsbereich:** Compiler-/Sprachregeln (ATCLang-SPEC), Lizenzdurchsetzung
(ATC-LIC), Marktplatz-Ökonomie.

---

## 2. Begriffe

| Begriff | Definition |
|---|---|
| **Code Unit** | die kleinste ausführbare Einheit, die eine Grenze überschreitet: ELF-Image, Bytecode-Artefakt, Treibermodul, Skript, JIT-Block |
| **Authority** | die einzige Instanz, die Aufnahme entscheidet (ShivaCore Code Authority) |
| **Admission** | Entscheidung „darf diese Code Unit ins System" |
| **Execution** | Entscheidung „darf diese Code Unit jetzt mit diesen Rechten laufen" |
| **Provenance** | nachweisbare Herkunft: Erzeuger, Build, Quelle, Kette |
| **Scope** | der Ausführungskontext, für den eine Freigabe gilt |
| **Revocation** | Widerruf einer zuvor erteilten Freigabe, wirksam ohne Neustart |

Admission und Execution sind **zwei Entscheidungen**, nicht eine. Eine Code Unit kann
aufgenommen und trotzdem für einen bestimmten Scope gesperrt sein.

---

## 3. Normatives Objekt: ExecutionPolicy

Jede Freigabe ist ein maschinenlesbares Objekt. Neun Felder, alle verpflichtend:

```text
ExecutionPolicy {
    code_hash             : Bytes[32]     // über die AUSFÜHRBARE Form, nicht die Quelle
    signer_identity       : DID           // wer freigibt (nicht: wer gebaut hat)
    source_provenance     : Provenance    // Erzeugerkette, Build-ID, Quell-Hash
    component_identity    : ComponentId   // wohin die Freigabe wirkt
    version               : SemVer        // monotone Version der Code Unit
    requested_capabilities: Rights[]      // angeforderte Rechte, nicht erteilte
    execution_scope       : Scope         // boot | service | contract | tool | one-shot
    expiration            : Int64         // Mikrosekunden seit Epoche, UTC
    revocation_status     : RevStatus     // active | revoked | suspended
}
```

### 3.1 Feldregeln (normativ)

- **code_hash**: Hash über die tatsächlich geladenen Bytes (ELF: Programmheader plus
  alle ladbaren Segmente in Ladereihenfolge; Bytecode: kanonisches Encoding). Ein
  Quell-Hash allein erfüllt die Anforderung **nicht**.
- **signer_identity**: DID nach ATC-03. Der Signer ist die freigebende Instanz, nicht
  zwingend der Autor. Autorisierung des Signers ist selbst eine Policy-Frage.
- **source_provenance**: muss mindestens Erzeuger-Identität, Build-Kennung und
  Quell-Hash binden; bei generiertem Code zusätzlich das erzeugende Modell plus
  Prompt-/Seed-Hash und die Freigabe-ID des erzeugenden Systems.
- **component_identity**: eine Freigabe gilt **nur** für die genannte Komponente.
  Komponentenübergreifende Gültigkeit ist ein eigener Policy-Eintrag.
- **requested_capabilities**: Rights nach SHIVA-SYSCALL-ABI. Angefordert ≠ erteilt;
  die Authority schneidet auf das Maximum der Komponente.
- **expiration**: ganzzahlig, byte-stabil, **kein** Float (Lehre aus dem
  RCT-Signaturvertrag: `-0.0`/`0.0`/`NaN` brechen die Signaturbasis).
- **revocation_status**: `revoked` wirkt sofort und überlebt Neustarts.

### 3.2 Signaturbindung (normativ)

- **Domain-Trennung:** Präfix `b"ATC-EXEC-ADM-v1\0"` vor dem Payload. Eine
  ExecutionPolicy-Signatur darf in keinem anderen Protokoll gültig sein.
- **Längenpräfixe:** jedes variable Feld mit `u32`-Längenpräfix in fester Reihenfolge.
  Verkettung ohne Präfix ist unzulässig (Kollisionsgefahr, siehe RCT-Befund).
- **Kanonisches Encoding:** genau eine Bytefolge pro Policy. Zwei semantisch gleiche
  Policies mit verschiedenen Codierungen sind ein Fehler, nicht zwei Freigaben.

---

## 4. Prüfreihenfolge (normativ, fail-closed)

Die Authority prüft **in dieser Reihenfolge** und bricht beim ersten Fehlschlag ab:

| # | Prüfung | Fehlercode |
|---|---|---|
| 1 | Authority erreichbar und initialisiert? | `ADM-010` |
| 2 | Policy strukturell gültig und kanonisch codiert? | `ADM-009` |
| 3 | Code Unit bekannt (Hash in der Registry)? | `ADM-001` |
| 4 | Integrität stimmt mit `code_hash` überein? | `ADM-002` |
| 5 | Signatur gültig und Signer autorisiert? | `ADM-003` |
| 6 | Komponente in der Freigabe enthalten? | `ADM-004` |
| 7 | Rechte im Rahmen der Komponentenfreigabe? | `ADM-005` |
| 8 | `expiration` nicht überschritten, Version nicht hinter der gesperrten Version? | `ADM-006` |
| 9 | `revocation_status` = active? | `ADM-007` |
| 10 | Provenance vollständig (inkl. Generierungsnachweis bei erzeugtem Code)? | `ADM-008` |
| 11 | **Isolation verfügbar** (Adressraum, NX/W^X, Capability-Grenze)? | `ADM-011` |

**Regel:** Jeder unbekannte Zustand ist eine Ablehnung. Nicht erreichbare Authority,
fehlende Registry, unlesbare Revocation-Liste: `BLOCKED`, niemals `AUTHORIZED`.
Es gibt keinen „Trust on first use"-Pfad und keinen stillen Rückfall auf
„unsigniert, aber lokal vorhanden".

---

## 5. Durchsetzungspunkte (Soll) und Ist-Stand

Die Authority ist nur so gut wie ihre Grenzen. Nach der Bestandsaufnahme existieren
heute **drei** reale Grenzen; alle weiteren sind Pflicht bei Einführung.

| Grenze | Ort | Ist-Stand (09.11.2026) |
|---|---|---|
| Nativ, Lader | `kernel/src/elf_loader.rs`, Aufruf `main.rs:339,372` | **keine** Prüfung: kein Hash, keine Signatur, keine Herkunft |
| Virtuell | `components/vm/src/vm.rs:101` vor `execute_state_transition_with_gas` | nur strukturell + Gas; keine Code-ID, keine Aufrufer-Autorität |
| Prozessstart | `process.rs:80`, `user_sched.rs:726` | keine Capability-Bindung beim Start |
| Service Space / Treiber | Service-Space-Startpfad | kein Startpfad im Image, damit auch kein Gate |
| AI-Tool-Aufruf | — | keine Tool-Runtime vorhanden |
| Genesis / generierter Code | — | kein Ausführungspfad vorhanden |
| Dynamisches Nachladen / JIT | — | nicht vorhanden |

**Normativ:** Wird eine dieser Grenzen gebaut, ist die Anbindung an die Authority
Teil der Definition of Done. Eine Grenze ohne Gate ist ein Standardverstoß.

---

## 6. Runtime-Isolation (normativ)

- **W^X:** keine Seite gleichzeitig schreibbar und ausführbar. Daten-NX ist belegt
  (`vmm.rs:881`), W^X für Codesegmente ist **nachzuweisen**, nicht anzunehmen.
- **Adressraumtrennung:** getrennte Adressräume pro Prozess (eigener CR3). Solange
  das fehlt, ist Isolation teilweise und muss als solche ausgewiesen werden.
- **Capability-Grenze:** Rechte werden beim Start vergeben und sind danach monoton
  nicht erweiterbar.
- **Kein Selbstaufstieg:** ein kompromittierter Prozess darf keine Kernel-Rechte
  erlangen; Rechteausweitung ist ein eigener Testfall.

---

## 7. Widerruf, Gültigkeit, Quarantäne

- Widerruf wirkt **ohne Neustart** für neue Ausführungen; laufende Ausführungen
  werden beendet oder in Quarantäne versetzt (Entscheidung pro Scope).
- **Rollback-Schutz:** eine Code Unit mit niedrigerer Version als die zuletzt
  freigegebene gilt als nicht autorisiert, außer eine explizite Rücknahme-Policy
  (eigener Eintrag, eigener Signer) liegt vor.
- **Quarantäne:** abgelehnter Code wird **nicht** ausgeführt, sondern zurückgehalten;
  Beweismittel (Bytes, Hash, Policy, Fehlercode, Zeitpunkt) bleiben erhalten.
- **Boot ohne Netz:** die Entscheidung muss offline reproduzierbar sein. Fehlt die
  Revocation-Quelle, gilt der zuletzt verankerte Stand, und das ist zu protokollieren.

---

## 8. Protokollierung (normativ)

Jeder Admission- und jeder Execution-Vorgang erzeugt genau einen Datensatz:

```text
{ timestamp, code_hash, signer_identity, component_identity, scope,
  decision (AUTHORIZED|BLOCKED), reason_code, policy_digest, authority_version }
```

- **Append-only**, nicht überschreibbar, auch für Ablehnungen.
- Ein Audit-Log, das nicht verdrahtet ist, erfüllt diese Anforderung nicht. Ist-Stand:
  `SecurityManager` wird in `kernel_init.rs:156` initialisiert und im Betrieb
  **nie aufgerufen**.
- Ohne Datensatz gilt der Vorgang als nicht stattgefunden (Nachweispflicht).

---

## 9. Testmatrix (verbindlich bei Implementierung)

| # | Fall | Erwartung |
|---|---|---|
| 1 | signierter, registrierter Code, passende Komponente | AUTHORIZED |
| 2 | unsigniertes natives ELF | BLOCKED `ADM-001`/`ADM-003` |
| 3 | Byte nach dem Signieren verändert | BLOCKED `ADM-002` |
| 4 | gültige Signatur, Signer nicht autorisiert | BLOCKED `ADM-003` |
| 5 | Freigabe für andere Komponente | BLOCKED `ADM-004` |
| 6 | mehr Rechte angefordert als freigegeben | BLOCKED `ADM-005` |
| 7 | abgelaufene Freigabe | BLOCKED `ADM-006` |
| 8 | widerrufene Freigabe, laufender Prozess | BLOCKED `ADM-007` + Beendigung |
| 9 | Replay einer alten Freigabe nach Versionssprung | BLOCKED (Rollback-Schutz) |
| 10 | von Genesis erzeugter Code ohne Freigabe | BLOCKED `ADM-008` |
| 11 | AI-Tool-Aufruf außerhalb der Allowlist | BLOCKED `ADM-004` |
| 12 | Authority nicht erreichbar | BLOCKED `ADM-010`, kein Rückfall |
| 13 | Policy in abweichender Codierung | BLOCKED `ADM-009` |
| 14 | Isolation nicht verfügbar | BLOCKED `ADM-011` |
| 15 | Protokolldatensatz für jeden Vorgang vorhanden | vollständig, append-only |

**Regression:** bestehende Vertrags-ABI-Hashes, `EXEC-CHAIN`-Vektoren und
Kernel-Boot-Evidenz bleiben unverändert.

---

## 10. Abnahmekriterien

1. Nicht autorisierter nativer Code wird vor Ausführung blockiert.
2. Manipulierter Code wird erkannt (`ADM-002`).
3. Unbekannte Treiber können nicht geladen werden.
4. Aurora AI kann nicht eigenmächtig Systemcode ausführen.
5. Genesis Engine kann generierten Code nicht ungeprüft starten.
6. ATC-VM akzeptiert ausschließlich validierte Programme.
7. Dynamische Codeausführung ist kontrolliert.
8. Widerrufene Berechtigungen werden durchgesetzt.
9. Ein kompromittierter Prozess erlangt keine Kernel-Rechte.
10. Jeder Blockierungs- und Freigabevorgang ist nachvollziehbar protokolliert.

**Grenze der Zusage:** eine absolute Garantie gegen jede Kompromittierung gibt es
nicht. Zugesagt ist die durchgängige, überprüfbare Durchsetzung der Grenzen
einschließlich Schutz gegen Umgehungsversuche.

---

## 11. Verhältnis zu bestehenden Standards

- **ATC-03 (DID / Zero-Trust IAM):** regelt Identität und Authentisierung. Dieser
  Standard regelt Aufnahme von *Code*, nicht von *Subjekten*. Ein gültiges DID ist
  Voraussetzung für `signer_identity`, aber keine Ausführungsberechtigung.
- **ATC-14 (Deterministic Execution):** berührt die Aufnahme den Konsens (etwa
  Vertragsbytecode), muss die Admission-Entscheidung deterministisch und
  netzwerkweit identisch sein. Für Boot-, Service- und Tool-Scope gilt diese
  Anforderung nicht.
- **ATC-LIC (License Gate):** Lizenz ist **keine** Sicherheitsfreigabe. Reihenfolge:
  **Admission vor License.** Unlizenziert und unsigniert sind zwei verschiedene
  Ablehnungsgründe und müssen getrennt protokolliert werden.
- **ATC-BC-001 (Opcode-Satz):** ein Isolation- oder Verifikationsopcode ist eine
  Bytecode-Änderung und damit eine eigene Entscheidung, nicht Teil dieses Standards.

---

## 12. Offene Entscheidungen (blockieren die Ratifizierung)

1. **Root of Trust:** TPM/Measured Boot oder softwareverankerte Kette? Ohne Wurzel
   ist `signer_identity` nicht rückführbar.
2. **Registry-Verankerung:** wo liegt die Code-Registry im Boot (Ramdisk, ATCFS) und
   wie wird sie selbst geprüft, ohne Zirkularität?
3. **Online vs. Offline-Widerruf:** welcher Stand gilt beim Boot ohne Netz, und wer
   darf ihn fortschreiben?
4. **Konsensbindung:** ist Admission für Vertragsausführung deterministisch
   (ATC-14) oder rein lokal?
5. **Signaturverfahren:** Ed25519 (wie DID/Multi-Sig) oder secp256k1 (wie
   Transaktionen)? Zwei Verfahren an derselben Grenze sind ein Fehler.
6. **Verhältnis zu ATC-LIC:** ein Gate oder zwei, und wer entscheidet zuerst?
7. **Eigentümerschaft der Authority:** Kernel-Funktion oder eigener Service Space?
   Die Bestandsaufnahme zeigt beides als möglich.

---

## 12a. Referenz-Durchsetzung (Stand 09.11.2026)

Die Prüfreihenfolge aus Abschnitt 4 ist als eigenständiges Crate ausführbar:
`components/exec-admission` (Paket `atc-exec-admission`, 21 Tests). Damit ist die
Entscheidungslogik prüfbar, **bevor** eine Grenze verdrahtet wird.

**Was das Crate ist:** kanonisches Encoding der `ExecutionPolicy`, Registry,
Prüfreihenfolge 1–11 mit `ADM-001`–`ADM-011`, fail-closed ohne Rückfall, und ein
append-only Protokoll, das jede Entscheidung mit Code festhält.

**Was es ausdrücklich nicht ist:** keine Kernel-Bindung, kein Loader, keine
VM-Anbindung, keine ABI-Änderung, keine Kryptografie. Die Signaturprüfung kommt
über den Trait `SignatureVerifier` von der Runtime; das Crate behauptet keine
kryptografische Sicherheit, es ordnet Entscheidungen zu. Die Enforcement-Schritte
in Abschnitt 5 bleiben unverändert offen und brauchen weiterhin einen
Owner-Beschluss zur Wurzel des Vertrauens.

Abgedeckte Fälle: gültige Freigabe, unbekannter Hash, abweichende Codierung,
Integritätsbruch, fehlende/ungültige Signatur, fremder Signer, fremde Komponente,
Rechte über Freigabe, Freigabe über Komponenten-Obergrenze, Ablauf, Rollback,
Widerruf, fehlende Provenance, generierter Code ohne Nachweis, fehlende Isolation,
nicht initialisierte Authority, Protokollpflicht, Reihenfolge der Fehlercodes sowie
die Kollisionsprobe ohne Längenpräfix.

---

## 13. Implementierungsreihenfolge (Vorschlag, unverbindlich)

1. Standard ratifizieren (dieses Dokument, mit Entscheidungen aus §12).
2. Registry und Policy-Format als SSOT, signierbar und kanonisch codiert.
3. Native Admission zuerst: `elf_loader.rs` vor der Segmentkopie, mit Testfällen 1–3.
4. VM-Admission: Code-Hash plus Signer vor `execute_state_transition_with_gas`.
5. Prozessstart an Capability binden (Testfall 6 und 9).
6. Audit-Log verdrahten, sonst ist keine Abnahme möglich (§8).
7. Isolation nachweisen (W^X, CR3), erst danach Attestation entwerfen.
8. AI-Tool-, Genesis- und JIT-Gates bei deren Einführung, nicht vorher.

**Reihenfolgeprinzip:** erst die Grenzen, die es gibt, dann die, die entstehen.
Ein Gate für einen Ausführungspfad, den niemand gebaut hat, ist Aufwand ohne Wirkung.

---

## 14. Was dieser Kandidat ausdrücklich nicht regelt

- TPM-Attestationsdetails und Measured-Boot-Protokoll.
- Monomorphisierung, Codegenerierung und Build-Reproduzierbarkeit im Detail.
- Marktplatz-Ökonomie, Preisbildung, Revenue-Share.
- Signaturverfahren im Feldformat (nur die Bindung ist normativ gefordert).

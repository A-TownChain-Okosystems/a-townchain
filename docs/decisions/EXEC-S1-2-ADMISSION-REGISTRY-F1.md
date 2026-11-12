# S1-2 / F-1 isoliert: Policy-Ueberschreibung in der Admission Registry

> **Status:** drei Entscheidungen getroffen (Owner, 12.11.2026), Compare-and-Insert
> **umgesetzt** und lokal geprueft. `ADM-012` ist **gesperrt**, bis der Standard
> normativ registriert ist; bis dahin traegt der Konflikt **keine**
> Fehleridentitaet (kein provisorischer Code). Kein `CI_VERIFIED`.
> Die Korrektur liegt ausschliesslich in `components/exec-admission`; die
> Containerintegration wurde dafuer **nicht** umgebaut. **Datum:** 2026-11-12.

## 1 Befund, am Code belegt

| Stelle | Verhalten |
|---|---|
| `components/exec-admission/src/lib.rs:190` | `Registry` ist `BTreeMap<[u8;32], ExecutionPolicy>` — Schluessel ist allein `code_hash` |
| `:199` | `pub fn insert(&mut self, policy)` ersetzt stillschweigend; kein Rueckgabewert, kein Vergleich |
| `:426` | `evaluate` vergleicht `policy.canonical_bytes()` mit `request.policy_bytes` und meldet bei Abweichung `ADM-009` |
| `:156` | der Text zu `ADM-009` lautet "Policy-Bytes weichen von der kanonischen Codierung ab" — er beschreibt eine **andere** Ursache als die tatsaechlich gepruefte |

Reproduktion: `System::deploy(quelle)` und `System::deploy_container(huelle)`
erzeugen fuer dasselbe Programm denselben `code_hash`, aber verschiedene Policy
Bytes, weil `source_provenance` verschieden ist (`atclang:nops` gegen
`atce:nops`). Der zweite Weg ueberschreibt die Freigabe des ersten; eine danach
ausgefuehrte erste Freigabe scheitert mit `ADM-009`. Belegt in
`components/system/tests/container_admission.rs` (Kommentar zu
`huelle_und_quelle_fuehren_zum_selben_ergebnis`) und im Nachweis
[EXEC-D0A-SYSTEM-INGRESS-EVIDENCE-2026-11-12.md](EXEC-D0A-SYSTEM-INGRESS-EVIDENCE-2026-11-12.md),
Abschnitt 5.

## 2 Was das Problem ist — und was nicht

**Nicht** betroffen: Bytecode-Integritaet, Signaturpruefung, Widerruf, die
Bindung der Freigabe an `code_hash` (S1-2 ist als Option C, asymmetrisch,
ratifiziert: Freigabe und Mindestversion am `code_hash`, Widerruf getrennt fuer
Code und Artefakt).

**Betroffen** ist die Trennung von **Code-Identitaet** und **Zulassungskontext**:
`code_hash` ist die Identitaet des Programms, die Policy traegt zusaetzlich
Herkunft und Kontext. Die Registry tut so, als sei "ein Programm = eine Policy",
und loest den Konflikt durch Ueberschreiben statt durch Entscheiden.

Wichtig fuer die Bewertung: die beiden kollidierenden Policies sind **beide
gueltig signiert** und beschreiben **dasselbe Programm**. Es geht also nicht um
Sicherheitsverlust, sondern um eine zweite Wahrheit ueber denselben
`code_hash` — genau das, was die Architekturregel ausschliesst.

## 2a Entscheidungen (Owner, 12.11.2026)

| # | Entscheidung | Inhalt |
|---|---|---|
| 1 | **Option A** | Die zuerst erfolgreich registrierte Policy bleibt unveraendert. Unbekannt = registrieren, bytegleich = idempotent, abweichend = `PolicyConflict`, niemals ueberschreiben, bei Konflikt keine Zustandsaenderung. Vergleich ueber die **vollstaendigen** kanonischen Policy-Bytes, atomar (Compare-and-Insert), keine implizite Aktualisierung, kein Last-Write-Wins, Konflikt reproduzierbar protokolliert. |
| 2 | **`ADM-012` genehmigt, aber an die Standardaenderung gebunden** | Der Konflikt erhaelt eine eigene Fehleridentitaet; `ADM-009` wird **nicht** umgedeutet. Vor der Implementierung: normative Codeliste, Admission-Fehlerdefinition, Rust, Referenzimplementierung, Testkorpus/Differentialtests, Dokumentation und Registry-Nachweis. Bis dahin bleibt `ADM-012` gesperrt. |
| 3 | **`source_provenance` bleibt signiert** | Herkunft ist sicherheitsrelevante Information und bleibt kryptografisch gebunden. Unterschiedliche Herkunft erzeugt unterschiedliche Policy-Identitaeten, also einen Konflikt. Keine Migration auf Option C, keine Entfernung des Herkunftsnachweises aus bestehenden Formaten. |

**Zusatzauflage (Owner).** Die Erstregistrierung muss deterministisch und
nachvollziehbar sein: unterschiedliche Registrierungsreihenfolgen auf
verschiedenen Knoten duerfen nicht zu unterschiedlichen Konsensentscheidungen
fuehren. Umgesetzt als Anforderung **R4** (Abschnitt 4a der Spezifikation) und als
Dokumentation an `Registry::register`: die Registry ist eine reine Funktion der
**bestaetigten** Deployment-Reihenfolge, nicht der lokalen Ankunftsreihenfolge.

## 3 Optionen

| Option | Inhalt | Folge |
|---|---|---|
| **A** (Empfehlung, Owner-Richtung) | `insert` wird zur Entscheidung: bytegleiche Policy = idempotent, abweichende Policy = getyoter Fehler, Registry bleibt unveraendert | eine Wahrheit, deterministische Ablehnung, kein stiller Ersatz; der zweite Zulassungsweg muss sich entscheiden lassen (siehe 5) |
| B | Registry auf `(code_hash, policy_digest)` schluesseln, mehrere Policies je Programm | mehrdeutig: welche Policy autorisiert eine Ausfuehrung? Widerruf und Mindestversion muessten je Policy gefuehrt werden; widerspricht S1-2 |
| C | `source_provenance` aus den signierten Policy-Bytes herausnehmen, als eigener Herkunftsnachweis fuehren | dann sind beide Wege bytegleich und der Konflikt verschwindet vollstaendig — beruehrt aber die kanonische Codierung und damit `ATC-SEC-EXEC-001` und alle bestehenden Signaturen |

**Empfehlung: A jetzt, C als eigene Entscheidung danach.** A beseitigt den
stillen Ersatz und macht den Konflikt sichtbar, ohne die Policy-Codierung
anzutasten. C ist langfristig die sauberere Antwort, weil Herkunft kein
Zulassungskriterium sein sollte — sie ist aber eine Formatentscheidung mit
eigener Migrationsfrage und nicht Teil dieses Schritts.

## 4 Regelvorschlag fuer A (umsetzbar, deterministisch)

1. `Registry::insert(&mut self, policy) -> Result<(), RegistryError>`.
2. Ist `code_hash` unbekannt: einfuegen, `Ok(())`.
3. Ist `code_hash` bekannt und `canonical_bytes()` **bytegleich**: `Ok(())`, kein
   Schreiben, `len()` unveraendert (Idempotenz, damit Wiederholungen nicht
   scheitern).
4. Ist `code_hash` bekannt und die Policy **abweichend**: `Err(PolicyConflict)`,
   die vorhandene Policy bleibt **unveraendert**; nichts wird geschrieben.
5. `System::deploy` und `System::deploy_container` bilden den Fehler auf
   `SystemError::Admission { code: Some("ADM-012"), detail }` ab. `ADM-012` ist
   die naechste freie Nummer (`ADM-001` bis `ADM-011` sind belegt).
6. Der Text von `ADM-009` wird berichtigt: geprueft wird "Policy weicht von der
   registrierten ab", nicht "nicht kanonisch codiert".
7. Keine zweite Registry, keine zweite Aufloesung, kein Ausweichpfad: `Registry`
   bleibt die einzige SSOT, `Authority` bleibt die einzige Pruefinstanz.

**Entschieden** (12.11.2026): die zuerst registrierte Policy gewinnt immer; eine
Ablehnung sperrt die vorhandene Freigabe **nicht**; `ADM-012` ist genehmigt, aber
erst nach normativer Registrierung zu implementieren; `source_provenance` bleibt
Teil der signierten Policy, Option C ist nicht freigegeben.

## 5 Abnahmekriterien (S1-2-T)

| ID | Fall | Erwartung |
|---|---|---|
| S1-2-T-01 | gleiche Policy zweimal eingetragen | `Ok`, `len()` unveraendert, keine Wirkung |
| S1-2-T-02 | gleicher `code_hash`, andere `source_provenance` | `Err(PolicyConflict)`, `len()` unveraendert |
| S1-2-T-03 | nach T-02: erste Freigabe ausfuehren | autorisiert, Block entsteht, **kein** `ADM-009` |
| S1-2-T-04 | Reihenfolge umgekehrt (Container zuerst) | derselbe Fehler, symmetrisch, kein Zustandsunterschied |
| S1-2-T-05 | Konflikt ueber `System::deploy` | `SystemError::Admission` mit `ADM-012`; **bis zur Registrierung** mit `code: None` (keine provisorische Kennung), kein Deployment, kein Registry-Eintrag |
| S1-2-T-06 | Konflikt ueber `System::deploy_container` | derselbe Fehler, keine Zustandsaenderung, kein Block |
| S1-2-T-07 | zwei verschiedene Programme | beide werden eingetragen, kein Fehler (Abgrenzung: Konflikt nur je `code_hash`) |
| S1-2-T-08 | Audit | der abgewiesene Eintrag erscheint im Audit-Log mit Code und `code_hash` |

## 6 Umsetzung und Evidenz

| Gegenstand | Ort | Status |
|---|---|---|
| Compare-and-Insert, First-Write-Wins, idempotente Registrierung, `PolicyConflict` | `components/exec-admission/src/lib.rs` (`Registry::register`, `RegistryError`) | umgesetzt |
| `ADM-009`-Text berichtigt (Anfrage weicht von der registrierten Policy ab) | `Authority::evaluate` | umgesetzt |
| Codeliste an einer Stelle maschinenlesbar | `standards/atc/conformance/admission-codes.json` (11 aktiv, `ADM-012` proposed) | umgesetzt |
| Bindung Spezifikation ↔ SSOT ↔ Rust, inkl. Sperre fuer `ADM-012` | `tools/check_admission_codes.py`, `tools/tests/test_admission_codes.py` | umgesetzt |
| Registrierungsregel R1–R5 | `ATC-SEC-EXEC-001` Kandidat 0.1.1-draft, Abschnitt 4a | spezifiziert |
| T-01, T-02, T-04, T-07, T-08 | `components/exec-admission/tests/registry_conflict.rs` | 5 Tests gruen |
| T-03, T-05, T-06 | `components/system/tests/registry_conflict.rs` | 3 Tests gruen |
| `ADM-012` in der Implementierung | `AdmCode` | **gesperrt** (Pruefer meldet einen Verstoss) |

**Registry-Nachweis.** Der Pruefer vergleicht drei Quellen und schlaegt fehl, wenn
ein aktiver Code fehlt, ein nicht registrierter Code implementiert ist, die
Reihenfolge abweicht oder ein vorgeschlagener Code vorzeitig auftaucht. Damit ist
die Sperre fuer `ADM-012` nicht Prosa, sondern ein Testfall.

**Offen (Governance).** Der Standard selbst ist weiterhin ein Kandidat, nicht
normativ registriert: `ATC-SEC-EXEC-001` steht nicht in
`atc-standards/registry/standards.yaml`, und eine ID-Vergabe fuer eine
Standardfassung ist eine Owner-Entscheidung, keine Agenten-Entscheidung. Deshalb
bleibt die Fehleridentitaet `ADM-012` gesperrt: der Systempfad meldet den Konflikt
mit `code: None`, ohne provisorische Kennung.

## 7 Was ausdruecklich nicht geaendert wird

* Keine zweite Implementierung der Registry oder der Authority.
* Keine Aenderung an `code_hash`-Bindung, Signaturpruefung, Widerruf, Ablauf.
* Keine Aenderung an Huelle, VM oder Systemeingang (`ATCE`/`ATCB` unveraendert).
* Keine Aenderung an `source_provenance` oder an bestehenden Signaturformaten.
* Kein `CI_VERIFIED`, keine Statusaufwertung aus lokalen Tests.

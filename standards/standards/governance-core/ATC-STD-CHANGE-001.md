---
standard:
  id: ATC-STD-CHANGE-001
  title: "ATC Change Control Dachnorm — Eine Änderung, ein Kanal, eine Gate-Landkarte: konsolidierte Zuordnung von ATC-STD-000 §19–33 (SCR), VERSION-001, UPDATE-001 und COMPAT-001 zur verbindlichen Entscheidungsmatrix „welches Instrument für welche Änderung“ — mit RACI, Notfallpfad und den 13 Change-Nachweis-Fragen als Prüfraster"
  version: "1.1.0"
  status: approved
  category: governance-core
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  classification: PUBLIC
  approved_by: "Michael Wroblewski (Owner, §9-Freigabe 08.09.2026, 00:36 UTC+2)"
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-VERSION-001
    - ATC-STD-UPDATE-001
    - ATC-STD-COMPAT-001
    - ATC-STD-AUDIT-001
  related_standards:
    - ATC-STD-STDDEV-001
    - ATC-STD-REGISTRY-001
    - ATC-STD-MILESTONE-001
    - ATC-STD-999
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-CHANGE-001 — ATC Change Control Dachnorm (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — §9-Freigabe Michael Wroblewski (Builder-Chat 08.09.2026, 00:36 UTC+2);
> normativ in Kraft ab 08.09.2026, §30-eingefroren (ATC-STD-000). SCR-0025 akzeptiert.
> **Familie:** Standards Governance Core (FAM-43). **Rolle:** Dachnorm — ordnet ZU,
> ersetzt NICHT: ATC-STD-000 §19–33, VERSION-001, UPDATE-001, COMPAT-001 bleiben
> unverändert normativ. **Kopplungen:** STDDEV-001, REGISTRY-001, MILESTONE-001,
> ATC-STD-999 (13 Nachweis-Fragen), AUDIT-001.

## Abstract

ATC-STD-CHANGE-001 ist die Dachnorm des Change Control: Sie konsolidiert die vier
Änderungs-Standards des Ökosystems zu EINER Entscheidungsmatrix — welche Änderung
läuft über welches Instrument, welche Gates greifen wann, wer entscheidet was
(RACI), wie der Notfallpfad funktioniert — und setzt die 13 Change-Nachweis-Fragen
des Master-Audits als verbindliches Prüfraster für JEDE Änderung. Sie erklärt die
Change-Control-Kette SCR→VERSION→UPDATE→COMPAT→AUDIT→REGISTRY zur einen, lückenlosen
Pipeline, die heute bereits in der Praxis gelebt wird.

Schlüsselwörter: MUSS/MUSS NICHT (absolut verbindlich), SOLLTE (empfohlen),
DARF/KANN (optional) — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Alle Änderungen am ATC-Ökosystem mit Governance-Relevanz: Code, Standards,
Registries, Protokolle, Dokumentation, Meilensteine — von der Dokument-Korrektur
bis zum Ecosystem-MAJOR.
**Gilt nicht:** Inhaltliche Regeln der Einzelstandards (die Fachnorm bleibt
maßgeblich); reine Laufzeit-Operationen ohne Zustandsänderung.

## §1 Zweck

Vier Fachnormen (SCR, VERSION, UPDATE, COMPAT) plus eine Dachnorm = keine Lücke,
keine Dopplung: Jede Änderung findet GENAU einen Pfad durch die Pipeline
(REQ-CH-001).

## §2 Änderungsarten-Matrix (verbindliche Zuordnung)

| Artefakt | PATCH | MINOR | MAJOR |
|---|---|---|---|
| Code/Software | Commit + CHANGELOG | Feature via UPD-Request | UPDATE-001 MAJOR + COMPAT-001 (UPD-G04) |
| Standard | SCR (redaktionell) | SCR + Validator (REQ-Erweiterung) | SCR + UPDATE-001 + COMPAT-001 |
| Registry | SCR + Gate | SCR + Gate | SCR + Generator + Konsistenz-Gates |
| Protokoll | SCR (Spec-Klarstellung) | SCR + versions.yaml | PROTOCOL-001 §19 (13 Schritte) |
| Dokumentation | Commit (LIVE-Doku) | SCR bei Strukturänderung | SCR |
| Meilenstein | — | Statuswechsel via S-20 | MILESTONE-001 (§7 Gates, §13 Human Gate) |

Die Zuordnung ist verbindlich; im Zweifel gilt die NÄCHST-strengere Spalte
(REQ-CH-002).

## §3 Eine Pipeline, ein Kanal

Jede Änderung MUSS durch den SCR-Kanal (ATC-STD-000 §19–33); es gibt KEINEN
track-freien Weg für Governance-Relevanz. Verbindliche Reihenfolge bei strukturellen
Änderungen: SCR → VERSION (Fenster) → UPDATE (Lifecycle) → COMPAT (bei MAJOR) →
AUDIT (Nachweis) → REGISTRY (SSOT-Sync) (REQ-CH-003). Commit-Trailer dokumentieren
SCR/AUD/Agent-Referenzen (REQ-CH-004).

## §4 Lifecycle-Zuordnung

SCR-Phasen (DECIDED → IMPLEMENTED → APPROVED) ↔ UPDATE-13-Stufen ↔ Standard-
Lifecycle (STDDEV-001 §3) sind konsistent zu führen: IMPLEMENTED ohne APPROVED nur
in der DRAFT-Phase; APPROVED nur mit §9-Freigabe-Dokumentation; EFFECTIVE erst nach
Registry-Sync (REQ-CH-005).

## §5 Gate-Landkarte

GREIFENDE Gates je Änderungsart (REQ-CH-006): UPD-G01..G09 (Update-Lifecycle;
G04 = MAJOR-COMPAT-Pflicht) · RR-G01..G08 (Release-Readiness) · S-Gates S-01..S-24 +
Manifest-Gate (Registry/Struktur) · TAX-CHECK-001..018 (Familien-/Kategorie-Bezug) ·
Acceptance Gates 1–8 (Meilensteine, MILESTONE-001 §7) · MAUD-PASS (Ecosystem-MAJOR,
ATC-STD-999 §7). Ein verletztet Gate stoppt die Pipeline bis zur Klärung — Gates
sind keine Empfehlungen (REQ-CH-007).

## §6 Rollen und Autorität (RACI)

- **Owner (§9):** Accountable für jede Freigabe (Standard, MAJOR, Meilenstein-
  ACCEPTED, Registry-Neuordnung). 
- **Autor-/Umsetzungs-Agent:** Responsible für Ausarbeitung, Validierung, Sync.
- **Validator-Suite (S-Gates):** Objective Checker (struktur), ersetzt NIEMALS das
  Human Gate.
- **Auditor:** Independent via REPO-AUDIT/MAUD.
KI-Agenten DÜRFEN Responsible sein; Accountable IMMER der Owner (REQ-CH-008;
AI-DECISION-001 übergeordnet).

## §7 Notfallpfad (Emergency)

Bei sicherheitskritischen Fixes: Emergency-Pfad nach UPDATE-001 (sofortiger Fix,
rückwirkende Dokumentation innerhalb 48h: SCR + UPD-Request + Validator + ggf.
AUD-Record). Der Notfallpfad suspendiert KEINE Gates — er verschiebt nur deren
Reihenfolge; die Nachholpflicht ist unverrückbar (REQ-CH-009).

## §8 Konsistenz-Regel (lex specialis)

Bei Widerspruch zwischen Dachnorm und Fachnorm gilt die Fachnorm (lex specialis);
CHANGE-001 regelt NUR die Zuordnung und Lückenlosigkeit. Bei Lücken (Änderungstyp
nicht in §2-Matrix): NÄCHST-strengere Zuordnung + SCR-Klärung (REQ-CH-010).

## §9 Traceability — die 13 Change-Nachweis-Fragen

JEDE Änderung MUSS die 13 Fragen des Master-Audits (ATC-STD-999 §3) beantworten
können: WHAT · WHY · WHO/WELCHER AGENT · WHERE · WHICH VERSION · WHICH STANDARD ·
WHICH REQUIREMENT · WHICH DEPENDENCIES · WHICH TESTS · WHICH DOCUMENTATION ·
WHICH SECURITY IMPACT · WHICH COMPATIBILITY IMPACT · WHICH AUDIT EVIDENCE.
Umsetzung im Bestand: Commit-Trailer, SCR-Dokumente, versions.yaml,
Session-Records (AOS-001), AUD-Records. Unbeantwortbare Fragen sind Findings
(BUG-005, P1–P2 je Impact; REQ-CH-011).

## Requirements (normativ)

- **REQ-CH-001** (§1): Eine Pipeline, keine Lücken, keine Dopplung.
- **REQ-CH-002** (§2): Änderungsarten-Matrix verbindlich; im Zweifel strenger.
- **REQ-CH-003** (§3): SCR-Kanal-Pflicht; Reihenfolge SCR→VERSION→UPDATE→COMPAT→
  AUDIT→REGISTRY.
- **REQ-CH-004** (§3): Commit-Trailer mit SCR/AUD/Agent-Referenzen.
- **REQ-CH-005** (§4): Lifecycle-Status konsistent; EFFECTIVE erst nach Sync.
- **REQ-CH-006** (§5): Gate-Landkarte je Änderungsart; MAUD-PASS bei Ecosystem-MAJOR.
- **REQ-CH-007** (§5): Verletzte Gates stoppen die Pipeline.
- **REQ-CH-008** (§6): RACI verbindlich; Owner immer Accountable.
- **REQ-CH-009** (§7): Emergency mit 48h-Nachholpflicht; keine Gate-Abschaffung.
- **REQ-CH-010** (§8): Lex specialis + Lückenregel (strengere Zuordnung).
- **REQ-CH-011** (§9): 13 Nachweis-Fragen je Änderung beantwortbar; Lücken sind
  Findings.
- **REQ-CH-012** (§10): Synchronisationspflicht aller nachgelagerten Registries MUSS vor Statusübergang zu APPROVED abgeschlossen sein.
- **REQ-CH-013** (§10): Gate-Bypass Verbot: Kein Gate DARF ohne explizites Owner-Bypass-Dokument übersprungen werden; unbefugte Bypasses MUESSEN als P1-Finding erfasst werden.
- **REQ-CH-014** (§10): Rollback-Planung MUSS für jede MINOR und MAJOR Änderung vor Freigabe vorliegen.
- **REQ-CH-015** (§10): Evidenz-Archivierung MUSS nach Abschluss der Änderung unveränderlich im Audit-Trail gesichert werden.


## §10 Metriken & Akzeptanzkriterien

1. **Pipeline-Durchlaufzeit:** MUSS für PATCH < 24h und für MINOR < 72h betragen.
2. **Gate-Erfolgsquote:** MUSS vor Merge 100% aller anwendbaren S-Gates und UPD-Gates betragen.
3. **Traceability-Index:** MUSS 100% der 13 Change-Nachweis-Fragen positiv beantworten.
4. **Emergency-Post-Mortem Rate:** MUSS innerhalb von 48h nach Notfall-Fixes 100% betragen.

## §11 Compliance & Verifikation

Die Einhaltung der Change-Control-Prozesse MUSS kontinuierlich in den Validator-Läufen S-01 bis S-25 und Repository-Audits nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 verifiziert werden. Jegliche Abweichung erzeugt automatisierte Findings F-001 bis F-999 und führt zur sofortigen Blockade der Change-Pipeline.

## Security Considerations

Change Control ist die zentrale Angriffsfläche der Governance: Eine ungetrackte
Änderung oder ein umgangenes Gate wäre eine unbemerkte Umbauung des Systems.
Schutz: SCR-Kanal-Pflicht ohne Ausnahme, Gate-Stopp-Regel, 48h-Nachholpflicht im
Notfall, 13-Fragen-Raster mit Finding-Folge, unabhängige Audit-Instanz (REPO-AUDIT,
MAUD), Human-Gate-Finalität beim Owner. Commit-Trailer-Manipulation wäre Fälschung
des Nachweises — Git-Historie + AUD-Records decken Widersprüche auf. Unbefugte Manipulationen der Change-Matrix oder Bypass-Versuche MUESSEN vom CI-System sofort als Severity-S1-Sicherheitsereignis klassifiziert und gemeldet werden.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.1.0** (2026-09-08): Additive Elaboration SCR-0041 — neue Regeln (§10, REQ-CH-012..015), Metriken, Compliance & Verifikation und Security-Klarstellungen, keine bestehende Regel geändert.
- **1.0.0** (2026-09-08): Initial Release — Konsolidierung der Change-Control-
  Kette als Dachnorm: Änderungsarten-Matrix (6 Artefakttypen × PATCH/MINOR/MAJOR),
  eine Pipeline (SCR→VERSION→UPDATE→COMPAT→AUDIT→REGISTRY), Lifecycle-Zuordnung,
  Gate-Landkarte (UPD-G01..09, RR-G01..08, S-01..24, TAX, Milestone, MAUD), RACI,
  Emergency-Pfad mit 48h-Nachholpflicht, lex-specialis-Regel, 13 Change-Nachweis-
  Fragen als Prüfraster. Ersetzt keine Fachnorm — ordnet zu (Abgrenzung zu
  ATC-STD-000 §19–33, VERSION-001, UPDATE-001, COMPAT-001 dokumentiert). 11 REQ-CH.
  SCR-0025; §9-Freigabe Michael Wroblewski 08.09.2026, 00:36 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- **NORMATIVE:** ATC-STD-000 (§19–33 SCR-Prozess), ATC-STD-VERSION-001, ATC-STD-UPDATE-001
- **INFORMATIVE:** SCR-0041, ATC-STD-BUG-001..005, ATC-STD-CHANGE-001
  (UPD-G01..G09), ATC-STD-COMPAT-001 (MAJOR-Gate UPD-G04)
- ATC-STD-999 §3 (13 Nachweis-Fragen), ATC-STD-AUDIT-001 (AUD-Records)
- ATC-STD-STDDEV-001, ATC-STD-REGISTRY-001, ATC-STD-MILESTONE-001
- ATC-STD-PROTOCOL-001 §19 (13-Schritte-Upgrade), ATC-STD-FRAMEWORK-001 (RR-Gates)
- Praxis-Nachweis: komplette Kette gelebt in SCR-0016..0024 (COMPAT→UPDATE→
  MILESTONE→REPO-AUDIT→PROTOCOL→TAXONOMY)

*ATC-STD-CHANGE-001 v1.0.1 · Owner-Entwurf Michael Wroblewski · Aurora (Superagent) · 08.09.2026 · SCR-0025*

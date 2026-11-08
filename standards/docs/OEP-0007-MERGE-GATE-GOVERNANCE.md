---
document:
  id: OEP-0007-MERGE-GATE-GOVERNANCE
  title: "OEP-7 — Merge/Gate Governance (Owner-Entscheidungs-Proposal)"
  version: "0.1.0-DRAFT"
  status: draft
  scr: SCR-0135
  chain_id: 658467
  authority: A-TownChain-Okosystems
  owner: "Michael Wroblewski (Owner) / Standards Governance"
  created: "2026-10-06"
  updated: "2026-10-06"
  classification: PUBLIC
  language: de-DE
  bezug:
    - "SPEC-ATC-GATE-METHODOLOGY-001 (PR #92, SCR-0132) — VERIFIED-Kernregel, wortgleich uebernommen"
    - "ATC-ORG-BASELINE-001 (SCR-0103) — MERGE_ALLOWED-Gate, Tier-Modell T0-T4"
    - "ATC-STD-MILESTONE-001 (SCR-0018) — Meilenstein-Governance"
    - "ATC-STD-UPDATE-001 (MAJOR-Revalidation UPD-G04) + COMPAT-001"
    - "F-160 (Admin-Bypass-Vorfall 96d5f12), F-122, atclang-Retro-Gruen-Vorfall (Anlass)"
  note: 'Owner-Entscheidungspunkt im OEP-Rahmen (EIP-artige Struktur: Motivation, Design, Konsequenzen). D1-D3 und Registry-Anschluss am 06.10.2026 vom Owner ENTSCHIEDEN; Dokument bleibt 0.1.0-DRAFT bis zum Methodik-Review (PR #92 nicht mergen davor).'
---

# OEP-7 — Merge/Gate Governance

## 1. Motivation

Ein grüner CI-Lauf ist kein Belastbarkeitsnachweis. Der atclang-Vorfall (Retro-Grün:
alter Check-Run gegen neuen Main-SHA als grün gelesen) und F-160 (Merge trotz
14/14 post-hoc grüner Checks, Gating umgangen, kein dokumentiertes Break-Glass) zeigen
 dieselbe Fehlerklasse aus zwei Richtungen: fehlende Bindung und fehlende Wirksamkeit.

Die Reifekette gilt ohne Ausnahme:

> IMPLEMENTED → TESTED → VERIFIED → RELEASED

Ein grüner CI-Lauf überspringt diese Kette nicht.

## 2. Problemstellung

- Verifikation ohne SHA-Bindung ist zufällig, nicht belastbar (Retro-Grün).
- Konfiguration ohne negativen Test ist eine Behauptung, kein Gate (F-160).
- Meilensteine ohne konsolidierte Registry erzeugen Parallelwahrheiten
  (M0–M14-Plan vs. ATC-M-001..008).
- Unentschiedene Scope-Modelle machen Gate-Ergebnisse unvergleichbar (W2-MECH-01).

## 3. Design

### 3.1 VERIFIED-Kernregel (wortgleich aus SPEC-ATC-GATE-METHODOLOGY-001)

VERIFIED = Result ∧ Freshness ∧ SHA-Binding ∧ Effectiveness ∧ Scope

Alle fünf Dimensionen sind notwendig. Fehlt eine, ist das Ergebnis NICHT VERIFIED —
unabhängig davon, wie grün der Run aussieht. Zustandsmaschine: VERIFIED /
PENDING-FRESH / PENDING-ENFORCE / PENDING-SCOPE / INVALID. Nur VERIFIED darf als
Evidence in ein Gate-geschütztes Merge eingehen.

### 3.2 D1 — Scope (ENTSCHIEDEN, Owner 06.10.2026)

Scope wird je Gate statisch deklariert (reviewbar, diff-bar, jeder Scope-Wechsel ist
ein eigener Change mit eigener Evidence). Ableitung nur als Ist-Messung gegen die
Deklaration (Unterdeckung wird maschinell sichtbar).

### 3.3 D2 — SHA-Quelle (ENTSCHIEDEN, Owner 06.10.2026)

Quelle bleibt CI-Metadaten (GITHUB_SHA) nach dem SCR-0086-Bindungsmuster: Evidence
schreibt den Wert ab, Validierung erfolgt gegen die Run-API. Kein handüberschreibbares
execution_sha-Feld (würde die von §3.3 der Methodik geschlossene Angriffsfläche
Retargeting wieder öffnen).

### 3.4 D3 — Enforcement (ENTSCHIEDEN, Owner 06.10.2026)

Neuer Enforcement-Test bei jeder Protection-Änderung PLUS periodische Stichprobe.
Die Stichprobe ist kein einmaliges Gate, sondern ein dauerhaftes Prozesskriterium.

### 3.5 F-160-Rollenregel (BINDEND)

Admin-Bypass nur als dokumentierte Ausnahme mit VORAB dokumentiertem Erwartungswert.
F-160 bleibt OPEN bis zum tatsächlichen Wirksamkeitsnachweis: Policy-Aktivierung,
dann dokumentierter Bypass-Versuch, dann nachgewiesene GitHub-Ablehnung. Eine
deklarierte Absicht oder Konfiguration reicht nicht. Agenten sind nie Approver;
Enforce-Admins folgt der Hub-Präzedenz.

### 3.6 Registry-Anschluss (ENTSCHIEDEN, Owner 06.10.2026)

Keine zweite M0–M14-Registry. Die bestehende ATC-M-001..008-Registry
(registry/milestones.yaml) bleibt SSOT; M0–M14 werden ausschließlich als Alt→Neu-
Abbildung innerhalb derselben Registry geführt (MAJOR-Revalidation nach
UPD-G04/COMPAT-001). ACCEPTED- und IN_PROGRESS-Stati bleiben erhalten und werden
durch die Revalidation nicht semantisch zurückgesetzt.

### 3.7 Tier-Bindung

OEP-7 verankert die VERIFIED-Pflicht im MERGE_ALLOWED-Gate des Tier-Modells
(ATC-ORG-BASELINE-001) — keine Parallelstruktur: ein Gate, eine Regel, eine Registry.

## 4. Konsequenzen

- Jede Protection-Änderung erzeugt eine neue Enforcement-Evidence-Pflicht.
- Der periodische Stichprobentest (bewusst negativer Run gegen ein geschütztes
  Repo) läuft im Fleet-Health-Rhythmus mit.
- PR #92 (Gate-Methodik) wird erst nach Methodik-Review gemergt.
- Golden Vectors werden vor den Freeze-Gates vorgezogen (ATCLang-Freeze M4.5/M4.7 im System-Meilensteinmodell, SCR-0136).
- ATCLang-Freeze (M4 im System-Meilensteinmodell, SCR-0136) bleibt ROT bis die
  Freeze-Gates erfüllt sind — 53/53 grüne Tests sind TESTED, nicht VERIFIED.

## 5. Entscheidungsstand

| Punkt | Stand |
|---|---|
| D1 Scope statisch | ENTSCHIEDEN (06.10.2026) |
| D2 CI-Metadaten als Quelle | ENTSCHIEDEN (06.10.2026) |
| D3 Enforcement je Änderung + Stichprobe | ENTSCHIEDEN (06.10.2026) |
| Registry-Anschluss (MAJOR-Revalidation) | ENTSCHIEDEN (06.10.2026) |
| F-160 | OPEN bis Wirksamkeitsnachweis |
| OEP-7 gesamt | 0.1.0-DRAFT bis Methodik-Review |

## 6. Nicht-Ziele

- Keine Neudefinition der Branch-Protection-Konfiguration (technische Umsetzung).
- Kein Parallelformat zur Evidence-Struktur (Einbettung in bestehende Bindung).
- Keine Aufweichung der Milestone-Lifecycle-Regeln (ATC-STD-MILESTONE-001 gilt).

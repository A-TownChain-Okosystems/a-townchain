---
document:
  id: SPEC-ATC-GATE-METHODOLOGY-001
  title: "Gate-Methodik — Draft 0.1.0 (VERIFIED-Bedingungen für alle Fleet-Gates)"
  version: "0.1.0-DRAFT"
  status: draft
  scr: SCR-0132
  chain_id: 658467
  authority: A-TownChain-Okosystems
  owner: "Alexander (Owner-Entwurf) / Standards Governance"
  created: "2026-10-06"
  updated: "2026-10-06"
  classification: PUBLIC
  language: de-DE
  umbrella: ATC-STD-PROTOCOL-001
  bezug:
    - "Evidence-Modell (bestehend, insbesondere .atc/evidence/evidence.yaml-Bindung SCR-0086)"
    - "ATC-PROTO-P2P-001 §9 (Envelope/Chain-ID-Bindung als Analogie für SHA-Bindung)"
    - "F-067, F-069"
    - "atclang-Sweep (Anlass, 06.10.2026)"
  note: 'Dokument-ID, kein Registry-Standard. Methodik-Spezifikation, normativ fuer alle Fleet-Gates sobald APPROVED. D1-D3 sind Draft-Blocker, Beratungsempfehlungen in Anhang A (nicht normativ).'
---

# Gate-Methodik — Draft 0.1.0

Klasse: Methodik-Spezifikation (normativ für alle Fleet-Gates)
Bezug: Evidence-Modell (bestehend), ATC-PROTO-P2P-001 §9 (Envelope/Chain-ID-Bindung als Analogie für SHA-Bindung), F-067, F-069, atclang-Sweep (Anlass)
Status: 0.1.0-DRAFT — nicht APPROVED

---

## 1. Zweck

Dieses Dokument definiert, wann ein Gate als VERIFIED gilt. Es definiert nicht, welche Gates existieren, welche Checks sie ausführen oder wie Branch Protection konkret konfiguriert ist. Es definiert die Bedingungen, unter denen ein Gate-Ergebnis als belastbare Aussage über den Zustand des Fleet gewertet werden darf.

Anlass: Der atclang-Sweep hat gezeigt, dass ein grüner Run und ein belastbares VERIFIED nicht dasselbe sind. Dieses Dokument schließt die Lücke formal.

---

## 2. Normative Kernregel

VERIFIED = Result ∧ Freshness ∧ SHA-Binding ∧ Effectiveness ∧ Scope

Alle fünf Dimensionen sind notwendig. Fehlt eine, ist das Ergebnis NICHT VERIFIED — unabhängig davon, wie grün der Run aussieht.

---

## 3. Die fünf Dimensionen

### 3.1 Result

Bedingung: Das Gate selbst meldet SUCCESS.

Nicht ausreichend:

- Ein vorgelagerter Schritt meldet SUCCESS, aber das Gate wurde nie ausgeführt.
- Ein Gate meldet SUCCESS, weil es keine Findings hatte — ohne dass nachgewiesen ist, dass es Findings hätte finden können (siehe §3.5 Scope).

Formal: result ∈ {SUCCESS, FAIL}. result = SUCCESS ist notwendig, nicht hinreichend.

### 3.2 Freshness

Bedingung: Der Run gehört zum aktuell geprüften main-SHA.

Ausschlussfall (atclang):

```
alter grüner Check-Run + neuer main-SHA = NICHT VERIFIED
```

Formal: run.commit_sha = current_main_sha. Kein Toleranzfenster. Kein „nah genug".

### 3.3 SHA-Binding

Bedingung: Der im Run deklarierte execution_sha ist identisch mit dem zu prüfenden SHA.

Freshness allein genügt nicht: Ein Run kann auf dem richtigen SHA ausgeführt worden sein, aber seine Ergebnisse können gegen einen anderen SHA deklariert sein (Retargeting, Rebase, Cherry-Pick, Force-Push-Nachwirkung).

Formal: run.execution_sha = target_sha.

Ausschlussfall:

```
grüner Run + kein SHA-Binding = NICHT VERIFIED
```

### 3.4 Effectiveness

Bedingung: Die Protection verweigert den Merge bei negativem Gate.

Ein grüner Run, der keinen Merge blockieren kann, ist ein Bericht, kein Gate.

Nachweis: Nicht durch Konfigurationslesen allein, sondern durch Enforcement-Test — ein bewusst negativer Run, gegen den ein Test-Merge geführt wird, der scheitern MUSS.

Formal: protection.enforced = true ∧ enforcement_test_passed = true.

Ausschlussfall:

```
grüner Run + SHA-Binding + keine nachgewiesene Enforcement-Wirkung = NICHT VERIFIED
```

### 3.5 Scope

Bedingung: Der Nachweis deckt exakt den angegebenen Prüfbereich ab.

Zwei Richtungen:

- Unterdeckung: Gate prüft weniger als deklariert (z. B. nur src/, nicht tests/; nur ein Crate im Workspace). → NICHT VERIFIED.
- Überdeckung mit falscher Signatur: Gate meldet Findings für Bereiche, die nicht Teil des Scopes sind (z. B. self-match im eigenen tools/-Baum, siehe W2-MECH-01). → Aussage ist kontaminiert. NICHT VERIFIED.

Formal: scope(reported) = scope(declared). Beide Richtungen.

---

## 4. Abgeleitete Regeln

- R1 — Kein Retro-Grün. Ein Run darf nicht rückwirkend als Nachweis für einen SHA verwendet werden, auf dem er nicht ausgeführt wurde.
- R2 — Kein Stale-Grün. Ein Run, der vor einer Änderung am geprüften Baum ausgeführt wurde, verliert seine Gültigkeit für alle nachfolgenden SHAs.
- R3 — Kein Silent-Pass. Ein Gate, das nach einer Änderung „nur noch weniger Findings" meldet, ist ohne True-Positive-Regression nicht als verbessert zu werten. Die Fähigkeit, echte Findings zu erzeugen, muss explizit nachgewiesen werden.
- R4 — Kein Config-only-Effectiveness. Enforcement wird durch Test bewiesen, nicht durch Lesen der Branch-Protection-Konfiguration.
- R5 — Kein Scope-Drift. Erweiterung oder Verengung des Prüfbereichs ist ein eigener Change mit eigener Evidence.

---

## 5. Statusübergänge

Ein Gate-Ergebnis hat genau einen der folgenden Zustände:

| Zustand | Bedeutung |
|---|---|
| VERIFIED | Alle fünf Dimensionen erfüllt |
| PENDING-FRESH | Result vorhanden, Freshness/SHA-Binding fehlt |
| PENDING-ENFORCE | Result + Fresh + SHA vorhanden, Effectiveness offen |
| PENDING-SCOPE | Scope-Nachweis fehlt oder widersprüchlich |
| INVALID | Result war FAIL oder Bindung inkonsistent |

Nur VERIFIED darf als Evidence in ein Gate-geschütztes Merge eingehen.

---

## 6. Anwendung auf die aktuelle Welle

| Gate / Paket | Status | Fehlende Dimension |
|---|---|---|
| W2-MECH-01 Kriterien 1–4 | VERIFIED (lokal) | — |
| W2-MECH-01 Kriterium 5 (Rollout) | PENDING-FRESH | Fresh Main Run nach PR #91 |
| W2-MECH-02 Cargo.lock | PENDING-FRESH | — |
| W2-MECH-03 fmt | PENDING-FRESH | — |
| W2-MECH-04 SSOT | IN ARBEIT | Scope-Nachweis vor Löschung |
| W2-DBG-01 | PENDING-FRESH | RCA + frischer Lauf |
| W2-DBG-02 F-159 | PENDING-FRESH | Slot-Overflow-Fix + frischer Lauf |
| atclang | INVALID (Retro-Grün) | Formal korrekt: Freshness verletzt |
| PR #89/#90/#91 | PENDING-ENFORCE | Enforcement-Test ausstehend |

---

## 7. Nicht-Ziele

- Dieses Dokument definiert nicht, wie Branch Protection technisch konfiguriert wird.
- Es definiert nicht, welche Checks erforderlich sind.
- Es definiert nicht, wie Evidence gespeichert wird — nur, wann Evidence gültig ist.

---

## 8. Übergang zur nächsten Stufe

Vor APPROVED dieses Drafts:

1. Referenz auf bestehende Evidence-Struktur prüfen (kein Parallelformat).
2. execution_sha-Feld gegen bestehende Run-Metadaten validieren.
3. Enforcement-Test-Vokabular an bestehende CI anschlussfähig machen.
4. Ein negativer Referenzfall (atclang) als historische Kalibrierung dokumentieren.

---

## Offene Punkte (nicht OEP, aber Draft-Blocker)

- D1: Wird Scope pro Gate deklariert oder aus dem Gate abgeleitet?
- D2: Wird execution_sha aus CI-Metadaten gelesen oder explizit in die Evidence geschrieben?
- D3: Enforcement-Test — pro Gate einmalig oder bei jeder Änderung an Protection?

---

## Anhang A — Beratungsempfehlungen der ausarbeitenden Instanz (NICHT normativ, keine Entscheidung)

### D1: Empfehlung — Scope statisch deklarieren, nicht ableiten

Begründung: Ein abgeleiteter Scope (aus Gate-Code) kann sich unbemerkt ändern (R5-Verletzung durch Implementierungsdrift — genau das W2-MECH-01-Muster). Ein statisch deklarierter Scope in der Gate-Definition ist reviewbar, diff-bar und macht jeden Scope-Wechsel zum sichtbaren Change mit eigener Evidence. Ableitung nur als Zusatz-Warnung (differenzierter Ist-Scope wird gemessen und gegen die Deklaration verglichen — Unterdeckung wird so maschinell sichtbar).

### D2: Empfehlung — Quelle bleibt CI-Metadaten, Evidence schreibt nur ab

Begründung: GITHUB_SHA ist die einzige nicht fälschbare Quelle im Run-Kontext (die Runtime-Umgebung setzt sie, der Job kann sie nicht vor dem Start manipulieren). Das bestehende SCR-0086-Muster (Bindung von GITHUB_SHA + test_run nach grünem Lauf in evidence.yaml) ist damit bereits D2-konform: Evidence schreibt den Wert ab, validiert wird gegen die Run-API (run.head_sha). Ein zusätzliches, von Hand überschreibbares execution_sha-Feld würde die Angreiffsfläche (Retargeting) wieder öffnen, die §3.3 gerade schließt.

### D3: Empfehlung — Enforcement-Test bei jeder Protection-Änderung, plus periodische Stichprobe

Begründung: Ein einmaliger Test bezeugt nur den Zustand zum Testzeitpunkt (R2-Analogie: Config-Stale-Grün). Jede Protection-Änderung (Checks, Reviews, enforce_admins) ist ein neuer Systemzustand mit eigener Evidence-Pflicht. Zusätzlich eine periodische Stichprobe (z. B. je Fleet-Health-Gate-Lauf ein bewusst negativer Test-Run gegen ein geschütztes Repo) deckt Config-Drift ab, der außerhalb des dokumentierten Change-Pfads entsteht — der F-160-Fall war exakt diese Klasse: Gating umgangen, weil der Zustand nie negativ getestet wurde.

---

Status: Draft 0.1.0 abgeschlossen. Bereit zur Einbettung in die bestehende Evidence-Struktur.

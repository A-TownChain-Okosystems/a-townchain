---
standard:
  id: ATC-AI-GOV-AUTH-001
  title: "ATC Automation Authority — Autorisierung der Engineering-Automatisierung, maschinenlesbare Ordnung, Lifecycle REGISTERED→VERIFIED, Evidence-Pflicht und Governance-Gates"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Spezifikation 07.10.)"
  created: "2026-10-07"
  updated: "2026-10-07"
  normative: false
  dependencies: [ATC-STD-000, ATC-AI-GOV-001, ATC-AI-GOV-CAPABILITY-001, ATC-AI-GOV-POLICY-001]
  related_standards: [ATC-AI-GOV-AUDIT-001, ATC-AI-GOV-FINDING-001, ATC-AI-GOV-INCIDENT-001, ATC-AI-GOV-CHANGE-001, ATC-STD-AI-DECISION-001, ATC-STD-AUDIT-001, ATC-STD-CHANGE-001]
  requirements: [REQ-AGOV-AUTH-001, REQ-AGOV-AUTH-002, REQ-AGOV-AUTH-003, REQ-AGOV-AUTH-004, REQ-AGOV-AUTH-005, REQ-AGOV-AUTH-006, REQ-AGOV-AUTH-007, REQ-AGOV-AUTH-008, REQ-AGOV-AUTH-009, REQ-AGOV-AUTH-010]
---

# ATC-AI-GOV-AUTH-001 — Automation Authority (v1.0.0, APPROVED)

> **Status:** APPROVED — Owner-Spezifikation 07.10.2026 (Volltext vom Owner: technische Autorisierungs- und Automatisierungsordnung statt klassischer persönlicher Vollmacht); maschinenlesbare Umsetzung: `.github`-Hub `ai/automation-authority.yaml`; SCR-0139. Keine klassische Vollmacht, sondern eine technische Ordnung mit Trennung von **Mensch, Automation, Repository, CI/CD, Evidence und Governance**.

## 1. Zweck (Purpose)

Die Autorisierung automatisierter Softwareentwicklung, Build, Verification und Release wird als deterministisch prüfbare Ordnung modelliert: Jeder Agent/Workflow beantwortet vor einer Aktion die Frage **„Darf ich diese Aktion ausführen?"** entlang einer festen Kette (Identity → Repository Scope → Action Scope → Policy → Approval → Execute → Evidence → Verify), ohne dass die Automation selbst zur unkontrollierten Governance-Instanz wird.

## 2. Geltungsbereich (Scope)

**Gilt:** Gesamtes A-TownChain Software-, Blockchain-, OS-, AI- und Engineering-Ökosystem: GitHub Actions und CI/CD-Runner, GitHub Apps und technische Bots, automatisierte Repository-Analyse, Build- und Testsysteme, Evidence- und Verification-Systeme, Release- und Artifact-Systeme, autorisierte AI-/Automation-Agents sowie weitere ausdrücklich freigegebene Engineering-Systeme.
**Nicht im Gelt:** Fachinhalte der autorisierten Tätigkeit (Familienstandards), Repositorystatus (state-machine.yaml), Berechtigungsvergabe im Einzelnen (CAPABILITY-001).

## 3. Normative Anforderungen

### REQ-AGOV-AUTH-001 — Autorisierte Identitäten und Systeme
id: REQ-AGOV-AUTH-001
Autorisiert sind ausschließlich die registrierten Identitätsklassen: **Human Maintainer**, **GitHub App / Bot**, **CI Runner**, **autorisierte AI-/Automation-Agents**. Jedes automatisierte System arbeitet unter einer registrierten Identität (vgl. `ai/agents.yaml`, `ai/agent.yaml`). Nicht registrierte Systeme haben keine Autorisierung (keine implizite Autorisierung).

### REQ-AGOV-AUTH-002 — Autorisierte Tätigkeiten
id: REQ-AGOV-AUTH-002
Innerhalb des jeweils freigegebenen Repository- und Berechtigungsumfangs: Repositories analysieren, Strukturen/Abhängigkeiten untersuchen, Implementierungen identifizieren, EXISTING-FIRST wiederverwenden, Fehler identifizieren und beheben, Quellcode ändern, Tests erstellen/aktualisieren, Builds und CI/CD-Pipelines ausführen, Dokumentation und Konfigurationen aktualisieren, technische Evidenz erzeugen (Run-IDs registrieren, Commit-SHAs erfassen, Logs/Artefakte sichern), Pull Requests erstellen/aktualisieren, Releases vorbereiten, autorisierte Tags erstellen, autorisierte Deployments ausführen.

### REQ-AGOV-AUTH-003 — Berechtigungsmodell (Least Privilege)
id: REQ-AGOV-AUTH-003
Berechtigungen werden nach Least Privilege vergeben und mindestens nach den Dimensionen **Organisation, Repository, Branch, Pfad, Aktion, Umgebung, Secret-/Key-Zugriff, Release-/Deployment** begrenzt. Eine Berechtigung für ein Repository ist keine Berechtigung für andere Repositories. Berechtigungs-Attestierung: `ai/authorization.yaml`; Capability-Modell: `ai/capabilities.yaml`.

### REQ-AGOV-AUTH-004 — EXISTING-FIRST-Prinzip
id: REQ-AGOV-AUTH-004
Vor jeder Implementierung prüft die Automation die sechs Fragen: (1) Existiert bereits eine Implementierung? (2) Existiert eine funktional äquivalente? (3) Existiert eine Single Source of Truth? (4) Existiert bereits eine Spezifikation? (5) Ist es möglicherweise eine Duplikation? (6) Muss bestehender Code erweitert, konsolidiert oder verschoben werden? Neue Implementierungen dürfen nicht allein deshalb erstellt werden, weil eine vorhandene Lösung nicht unmittelbar gefunden wurde.

### REQ-AGOV-AUTH-005 — Evidence-First-Prinzip
id: REQ-AGOV-AUTH-005
Eine technische Behauptung gilt nicht allein aufgrund vorhandener Dateien, Codezeilen oder grüner CI als verifiziert. Für relevante Engineering-Aktionen ist Evidenz zu erfassen: Run-ID, Repository, Branch, Commit SHA, Job, Step, Start-/Endstatus, Exit Code, relevante Logs, erzeugte Artefakte, Prüfresultat. Zustände mindestens: REGISTERED → RUNNING → ANALYZED → VERIFIED; Fehler: RUNNING → FAILED; Blockierung: RUNNING → BLOCKED; nachträglich erkannte Abweichung: VERIFIED → RESIDUAL.

### REQ-AGOV-AUTH-006 — Engineering-Automation-Lifecycle
id: REQ-AGOV-AUTH-006
Verbindlicher Ablauf je Automationsaktion: REGISTERED → DISCOVER → EXISTING-FIRST → ANALYZED → IMPLEMENT/FIX → BUILD → TEST → EVIDENCE → VERIFY. Bei Fehler: FIX → BUILD → TEST → EVIDENCE → VERIFY (Wiederholung bis alle Gates erfolgreich, Evidenz vollständig und keine offenen Blocker bestehen — oder ein dokumentierter BLOCKED-/RESIDUAL-Zustand vorliegt). Bei Blockierung: Human Approval / Dependency Resolution. Erst VERIFIED führt zu RELEASE/MERGE; danach RESIDUAL → Remediation oder CLOSED.

### REQ-AGOV-AUTH-007 — Merge- und Release-Governance
id: REQ-AGOV-AUTH-007
**GREEN CI ≠ VERIFIED** und **EXISTENCE ≠ IMPLEMENTED ≠ TESTED ≠ VERIFIED**. Ein Merge bzw. Release darf nur erfolgen, wenn die dafür definierten Gates und Evidence-Anforderungen erfüllt sind. Ein erfolgreicher CI-Lauf allein ist keine fachliche oder technische Verifikation.

### REQ-AGOV-AUTH-008 — Geschützte Aktionen
id: REQ-AGOV-AUTH-008
Ausdrückliche Freigabe bzw. Governance-Gate erforderlich für: Änderung von Branch Protection, Repository-Berechtigungen, Secrets, kryptografischen Schlüsseln, sicherheitskritischen Policies, Governance-Regeln, Consensus-/Blockchain-Regeln; Produktionseinsatz; irreversible Datenlöschung; Decommissioning von Repositories; Änderung normativer SoT-Komponenten.

### REQ-AGOV-AUTH-009 — Kryptografie, Sicherheit und Scope-Disziplin
id: REQ-AGOV-AUTH-009
Private Schlüssel und Zugangsdaten dürfen nicht in Quellcode, Logs, an Dritte oder in Artefakten veröffentlicht werden; Signierungs-, Schlüssel- und Identity-Funktionen ausschließlich über autorisierte Mechanismen. Die Automation etabliert keine neuen funktionalen Subsysteme außerhalb des genehmigten Architektur-Scopes; Architektur, SoT, Repository-Zuständigkeit, normative Spezifikationen, Governance und Security Boundaries gelten nicht allein durch technische Implementierung als geändert.

### REQ-AGOV-AUTH-010 — Auditierbarkeit und Widerruf
id: REQ-AGOV-AUTH-010
Minimale Audit-Kette: Authorization → Run-ID → Repository → Commit SHA → Job → Step → Evidence → Verification; Evidenz unveränderbar bzw. nachträglich nachvollziehbar speichern. Berechtigungen können jederzeit durch die autorisierte Governance-Stelle widerrufen/eingeschränkt werden; nach Widerruf keine neuen automatisierten Änderungen; erzeugte Evidenz und Auditdaten bleiben gemäß Aufbewahrungsregeln erhalten.

## 4. Grundprinzipien (verbindlich)

Standalone First, Ecosystem Second · EXISTING-FIRST · Least Privilege · Fail Closed · Evidence First · No Evidence, No Trust · No Gate Weakening · Exact-SHA Verification · Keine implizite Autorisierung · Keine funktionale Duplikation · Reproduzierbare Builds · Nachvollziehbare Änderungen.

## 5. SSOT und Maschinenlesbarkeit

Operative SSOT: `.github`-Hub **`ai/automation-authority.yaml`** (Identitäten, Scopes, Lifecycle-Transitions, Evidence-Schema, geschützte Aktionen, Entscheidungs-Kette, Widerruf). Bestehende Governance-Artefakte bleiben SSOT ihrer Domäne und werden referenziert, nicht dupliziert: `ai/capabilities.yaml` (Capability = WAS), `ai/authorization.yaml` (WO/Atttestierung), `ai/state-machine.yaml` (Repository-Compliance-Zustände, nicht identisch mit dem Engineering-Lifecycle REQ-AGOV-AUTH-006), `ai/policies.yaml` / `ai/policy-graph.yaml` (Policy), `ai/audit.yaml` / `ai/provenance.yaml` (Audit/Evidence).

## 6. Abgrenzungen

- Engineering-Lifecycle (dieser Standard, je **Aktion/Run**) ≠ Compliance-State-Machine (`ai/state-machine.yaml`, je **Repository**).
- Berechtigungsattestierung (`ai/authorization.yaml`) bleibt bestehen; AUTOMATION-AUTHORITY ist der übergeordnete Ordnungsrahmen, keine Ersatz-SSOT.
- Normative Autorität des Ökosystems bleibt bei atc-standards; Durchsetzung (HOW) obliegt der Engineering-Control-Plane (atc-engineering).

## 7. Referenzen

Owner-Spezifikation (Volltext) 07.10.2026 22:28 — Autorisierung für die A-TownChain-Ökosystem-Automatisierung, Abschnitte 1–15. SCR-0139. Änderungen ausschließlich via SCR + §9-Freigabe (STDDEV-001).

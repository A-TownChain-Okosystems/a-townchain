---
standard:
  id: ATC-AI-GOV-ACCESS-001
  title: "ATC Agent Governance — AI Access Role Model (Rollenprofile fuer GitHub-Zugangs-Identitaeten, Omni-Verbot, Token-Hygiene, Access Review)"
  version: "1.0.0"
  status: approved
  category: ai-gov
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf 14.09.)"
  created: "2026-09-14"
  updated: "2026-09-14"
  normative: false
  dependencies: [ATC-STD-000, ATC-AI-GOV-CAPABILITY-001]
  related_standards: [ATC-AI-GOV-001, ATC-AI-GOV-POLICY-001, ATC-AI-GOV-INCIDENT-001, ATC-AI-GOV-AUDIT-001]
  requirements: [REQ-AGOV-ACC-001, REQ-AGOV-ACC-002, REQ-AGOV-ACC-003, REQ-AGOV-ACC-004, REQ-AGOV-ACC-005, REQ-AGOV-ACC-006]
---

# ATC-AI-GOV-ACCESS-001 — AI Access Role Model (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 14.09. 09:36 (Rollenprofil-Matrix als Gegenentwurf zur
> Omni-Konfiguration aus der Owner-Beratung 14.09.); SCR-0121. Wartet auf §9-Freigabe (ATC-STD-000).

## 1. Zweck (Purpose)

Womit authentifiziert sich ein AI-System gegenueber GitHub und was darf diese Identitaet technisch
— ein explizites, pruefbares Zugriffs-Rollenmodell, das die Stufen-Architektur (Audit / Entwicklung /
Governance) der A-TownChain-Okosystems-Normung festzieht und Omni-Identitaeten verbietet.

## 2. Geltungsbereich (Scope)

**Gilt:** Alle AI-Zugangs-Identitaeten auf A-TownChain-Okosystems (klassische + fine-grained PATs,
GitHub Apps, Deploy Keys). **Nicht im Gelt:** fachliche Befugnisse im Oekosystem (CAPABILITY-001 —
ACCESS sagt womit authentifiziert wird, CAPABILITY was der Agent tun darf), menschliche Konten
(Stufe 3 bleibt Owner-reserviert, siehe §4/§6).

## 3. Kontrollprinzipien

1. **Least Privilege:** Jede Identitaet erhaelt nur die fuer ihr Profil noetigen Permissions.
2. **No Self-Expansion:** Keine Identitaet darf Berechtigungen oder Gates veraendern koennen, die
   sie selbst kontrollieren (Zirkularitaetsverbot, siehe §7).
3. **Stufe-3-Reserve:** Organisationsebene (Administration, Members, Settings) bleibt
   menschlich-Owner-reserviert.
4. **Separation of Duties:** Implementer-Identitaet ≠ Audit-Identitaet; Rollenprofile sind trennbar
   zu halten (konsistent mit MAINT-000 §5).
5. **Evidence & Audit:** Jede Zugriffsvergabe, Rotation und jede Ausnahme-Freigabe erzeugt einen
   maschinenlesbaren Evidence-Record (Schema MAINT-019).

## 4. Rollenprofile (stehende Identitaeten)

| Profil | Repository-Permissions | Zweck |
|---|---|---|
| **ATC-AUDITOR** (Stufe 1) | Contents R, Metadata R, Issues R, Pull requests R, Actions R, Checks R, Commit statuses R, Code scanning alerts R, Dependabot alerts R, Secret scanning alerts R, Security advisories R | Live-Org-Audits, Evidence-Collection, Conformance-Scans; schreibt NIE |
| **ATC-DEVELOPER** (Stufe 2) | Contents RW, Metadata R, Issues RW, Pull requests RW, Actions RW, Workflows RW, Commit statuses RW, Checks RW | Entwicklung, Branches/Commits, PRs, CI-Wartung; Merges nur nach Human Approval |
| **ATC-CI** (Stufe 2, eventgetrieben) | Contents R, Issues RW, Checks RW, Commit statuses RW, Workflows RW, Actions R + Webhook-Events (push, pull_request, issues, workflow_run, release, security) | GSEPF Event Processor, Evidence-Commits, Status-Updates |

## 5. Verbotene Permissions (global, fuer ALLE stehenden AI-Identitaeten)

Administration RW (Repo-Settings/Branch-Protection), Secrets/Dependabot-Secrets/Codespaces-Secrets
(alle Ebenen), Deployments/Environments RW, Pages RW, Webhooks RW, Custom Properties RW,
Organization: Administration, Members, Secrets, Variables, Webhooks, PAT-Requests/Approvals,
Projects RW, Self-hosted Runners RW, Team-Administration, Repository-Deletion, Billing.

## 6. Stufe 3 — Governance/Administration (bewusst KEINE stehende Identitaet)

Org-Administration (u.a. Branch-Protection, Dependency-Graph-Setting F-ORG-002, Actions-Org-Settings)
bleibt Owner-reserviert. **Ausnahmepfad:** zeitgebundene (< 24 h), zweckgebundene Einzel-Credentials
pro konkreter Aktion, nur mit expliziter Owner-Freigabe und Evidence-Record (M3-Analogon); danach
sofortiger Entzug. Eine stehende ATC-GOVERNANCE-Identitaet existiert nicht.

## 7. Omni-Verbot (normativ)

Eine einzelne Identitaet, die Administration + Secrets + Members + PAT-Verwaltung (jeweils RW)
buendelt, ist VERBOTEN — sie koennte die eigenen Kontroll-Gates umschreiben (Branch-Protection
abschalten, Workflows aendern, Berechtigungen selbst erweitern, Mitglieder einladen) und wuerde
jede SoD-Regil (ATC-AI-GOV-001 Zuständigkeits-Trennung; MAINT-000 §5) gegenstandslos machen.
Expliziter Gegenentwurf zur "ATC-Omni-Governance-Agent"-Maximalkonfiguration (Owner-Beratung 14.09.).

## 8. Token-Hygiene

Fine-grained PATs vor klassischen PATs (Scope-Enforcement statt Konvention); Befristung <= 90 Tage;
Rotation mit Audit-Log; Private Keys von GitHub Apps und Secrets NIE in Repositories oder
Agent-Kontexte; klassische PATs nur als dokumentierter Uebergangszustand mit Migrationsdatum.

## 9. GitHub-App-Leitlinie (Zielarchitektur)

Gesplittete Apps je Rollenprofil (ATC-AUDITOR-App, ATC-DEVELOPER-App, ATC-CI-App) statt einer
Omni-App; Webhook-Events → GSEPF Event Processor (DISCOVER → VALIDATE → AUDIT → POLICY CHECK → ACT);
Installation nur auf A-TownChain-Okosystems; Re-Consent des Owners bei jeder Permission-Aenderung
(nach §5/§7 geprueft).

## 10. REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-AGOV-ACC-001 | Jede AI-Zugangs-Identitaet ist genau einem Rollenprofil (§4) zugeordnet und als Manifest-Eintrag registriert | MUST |
| REQ-AGOV-ACC-002 | Stehende Identitaeten haben KEINE der §5-Verbotsrechte | MUST |
| REQ-AGOV-ACC-003 | Omni-Identitaeten (§7) werden nicht erstellt; bestehende werden gesplittet oder entzogen | MUST |
| REQ-AGOV-ACC-004 | Stufe-3-Aktionen laufen ueber Owner-Freigabe; Ausnahme-Credentials sind zeitgebunden, zweckgebunden und evidence-geprueft | MUST |
| REQ-AGOV-ACC-005 | Neue Zugangs-Vergaben: fine-grained, <= 90 Tage befristet, Rotation geloggt | MUST |
| REQ-AGOV-ACC-006 | AI-Zugriffsvergaben werden mindestens quartalsweise geprueft (Access Review, M1-Klasse analog MAINT-001) | MUST |

## 11. Implementierungsstatus (ehrlich, SCR-0080-Doktrin)

**SPECIFICATION_ONLY.** Live-Befund 14.09.: aktives Token ist ein klassisches PAT (repo, workflow,
read:org, read:user) auf ShivaCoreDev (Org-Admin-Rolle) — es uebersteigt das ATC-DEVELOPER-Profil
(repo-Scope umfasst faktisch Settings-APIs); Migrationsbedarf auf Fine-grained ist im Runbook
14.09. dokumentiert. Branch-Protection ist bei 27/30 Repos AUS (P0-Audit-Fund). CLAIMED != PASS —
dieser Standard beschreibt die Zielarchitektur, nicht den Ist-Zustand.

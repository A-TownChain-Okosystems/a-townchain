# Approval Decision — ATC-STD-000 v1.2.0 + Governance-Freeze-Abschluss

**Datum:** 07.09.2026, 20:00 UTC+2 · **Entscheider:** Owner (Michael Wroblewski) · **Status: APPROVED**

## Owner-Approval

**Entscheidung:** **APPROVED / ALLES FREIGEGEBEN** am 07.09.2026, 20:00 UTC+2.
Owner: Michael Wroblewski — Freigabe per Owner-Direktmandat im Builder-Chat
(„Alles freigeben", 07.09.2026 19:59 UTC+2).

## Freigegebene Punkte

| Punkt | Vorher | Nachher |
|-------|--------|---------|
| ATC-STD-000 v1.2.0 (§37 ID-Allokation/SCR-0001, §38 Security/F-004) | candidate | **approved** — gültige Verfassungsfassung |
| SCR-0003 Branch Protection (Option B) | PENDING-Textnachtrag | **accepted (Option B)** — physischer Stand verifiziert: Protected main aktiv (Status Checks, Required Reviews, kein Force-Push, kein Löschen), Agent-Direktpush als dokumentierte Owner-Ausnahme (enforce_admins: off, 1-Agent-per-Repo, Commit-Nachweis) |
| V-16 Conventional-Commits-Quote (75 % < 80 %, WARN) | WARN beobachten | **Disposition akzeptiert** — WARN ohne Blocker; ab sofort Conventional-Commits-Types normativ über AI-DEV-007 §1 (Commit-Format mit `<type>`), Quote steigt automatisch |
| SCR-0001 (ID-Allokation §37) | bereits ACCEPTED („Alles umsetzen") | in v1.2.0 in Kraft |
| SCR-0004 (Rollenmodell §14.1) | bereits CLOSED (v1.1.0) | in Kraft |

## Governance-Freeze: ABGESCHLOSSEN

Mit dieser Entscheidung sind normativ in Kraft und damit eingefroren (§30,
Änderungen nur via SCR):
- ATC-STD-000 v1.2.0 (Verfassung inkl. §7 Naming, §14.1 Rollen, §37 ID-Allokation, §38 Security)
- ATC-STD-204 v1.0.0 (Dependency & Interface)
- ATC-STD-AI-DEV-001/004/007/009 v1.0.0 (AI-Agenten-Governance)
- ATC-STD-BUG-001…004, ATC-STD-NET-001…008 (Owner-Mandate AD-040/041)
- ATC-STD-100/300, ATC-STD-ZKP-001…010 (Owner-Entwürfe, candidate — Status
  unverändert, unterliegen der Verfassung; eigene APPROVED-Freigaben folgen
  nach Fertigstellung der Review-Pakete)

Offene operativ arbeitbare Auflagen (keine Freigaben mehr):
1. Interface-Test-Suiten IFC-0001..0010 (seed → active) bis 07.10.2026.
2. Commit-Trailer-Rollout + AGENT_PROTOCOL-Migration bis 07.10.2026.
3. Agent-Manifeste (.github/ai/) + AGENTS.md in R2+-Repos bis 07.10.2026.
4. AI-DEV-002/003/005/006/008/010/011/012 (je eigene Freigabe nach Fertigstellung).

## Freigabebasis

- Owner-Direktmandat (Präzedenz: v1.0.0-Approval, AD-040/041-Mandate).
- Review-Chain §14 gegen v1.2.0: Technical PASS · Security PASS ·
  Architecture PASS (07.09.2026, dokumentiert in governance/).
- Physische §34-Integrität verifiziert: Branch Protection aktiv,
  CODEOWNERS, CI-Validator läuft je Push.

## Entscheidung (vom Owner)

- [x] APPROVE — Vermerk: Builder-Chat 07.09.2026, 19:59 UTC+2 („Alles freigeben")
- [ ] REQUEST CHANGES — Begründung: ____________
- [ ] REJECT — Begründung: ____________

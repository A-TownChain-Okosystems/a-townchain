# Approval Decision — ATC Smart Contract Standards Framework (ATC-STD-SC-001..020 v1.0.0)

**Datum:** 07.09.2026, 21:00 UTC+2 · **Entscheider:** Owner (Michael Wroblewski) · **Status: APPROVED**

## Owner-Approval

**Entscheidung:** **APPROVED / FREIGEGEBEN** am 07.09.2026, 21:00 UTC+2.
Owner: Michael Wroblewski — Freigabe per Owner-Direktmandat „Freigabe" im
Builder-Chat (Todo #118).

**Gegenstand:** Die komplette Familie ATC-STD-SC-001..020 v1.0.0
(CANDIDATE → APPROVED) — ATC Smart Contract Standards Framework
(Owner-Entwurf Michael, Formalfassung ATC-AI-ARCH-001).

**Umfang:** 58 Anforderungen (REQ-SC-001..058) über 20 Standards:
SC-001 General (Kategorien SC-CORE..SC-SYSTEM, SSOT-Kette, Compliance-
Gates SC-G0..G13 — kein Gate, kein Mainnet), SC-002 Identity, SC-003
Security, SC-004 Testing/Invarianten, SC-005 Audit, SC-006 Deployment,
SC-007 Upgrade (Default immutable), SC-008 Events, SC-009 Access Control,
SC-010 Treasury, SC-011 Token, SC-012 NFT, SC-013 DeFi, SC-014 Governance,
SC-015 Bridge, SC-016 Oracle, SC-017 GameFi, SC-018 Mining, SC-019 Contract
Registry (contracts/, Seed ATC-SC-TOKEN-001..003), SC-020 AI-Assisted
Development (Agent-Deploy-Verbot ohne Gates).

**Evidenz bei Freigabe:** validate_all 103/103 COMPLIANT · Contract-
Registry-Gate CONFORM (3 Contracts, 0 Deployments) · README-Gate 13/13 ·
MD-Gate CONFORM · Repo-Audit R3 100/100 GATE PASS · Agent-Manifest-Gate
PASS (103 Standards) · Mutationssuite 12/12 · Commit 0d45fc0.

## Konsequenzen

- Die Familie SC-001..020 ist **normativ in Kraft**; Immutabilität per
  ATC-STD-000 §30 — Änderungen ab sofort nur via SCR.
- Registry: **103 Standards, 102 APPROVED, 1 CANDIDATE offen** (ATC-STD-MD-001).
- Übergangsfristen: SC-Gate-Rollout auf die Contract-Repos
  (a-townchain-os, atc-vm) mit Registry-Befüllung (SC-G0 je Contract) —
  Agenten-lokale Prüfung bis zur CI-Integration (workflow-Scope-Blocker
  F-009/F-010). Deep-Standards ATC-STD-SC-BRIDGE-001 und ATC-STD-FEE-001
  folgen per SCR (ROADMAP).
- Task #118: Freigabe erledigt; Gate-Rollout-Komponente bleibt offen.

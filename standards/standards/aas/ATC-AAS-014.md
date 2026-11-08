---
standard:
  id: ATC-AAS-014
  title: "ATC-AAS-014 — Agent Security Standard"
  version: "1.1.0"
  status: approved
  category: aas
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-AAS-014 — Agent Security Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-014 (Agent Security Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Geschützte Klassen (niemals durch Agenten in Code/Logs/Issues/Commits/PRs)

API Keys · Tokens · SSH Keys · Wallet Keys · Private Keys ·
Environment-Variable-Werte (nur Namen referenzieren) · Credentials.

**Grundregel: Secrets dürfen niemals vom Agenten in Code, Logs, Issues,
Commits oder PRs geschrieben werden.** (Invariant `no_secret_access`,
AI-DEV-002 §3.)

## 2. Umgebungssicherheit

`environment` im Identity-Manifest (AAS-001) deklariert: Sandbox-Typ,
Netzzugriff (allowlist), erlaubte Kommandoausführung. Nicht deklarierte
Netzwerkziele/Kommandos sind verboten.

## 3. Umgang mit Secret-Fund

Findet ein Agent ein exponiertes Secret: KEIN Kopieren/Zitieren, sofort
Finding S0 (AI-DEV-005), Owner-Eskalation (AI-DEV-011 §2), Task BLOCKED
bis Bereinigung. Rotation ist Owner-Aktion.

## 4. Konsum

Benötigt eine Agentenarbeit ein Secret (z.B. Deploy), erfolgt der Zugriff
ausschließlich über vom Owner verwaltete Secret-Stores — der Agent
referenziert nur den Namen.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Agenten-spezifisch: Agent-Identitaet via AGENT_MANIFEST verifizierbar; Permissions nach Least-Privilege; Delegationen dokumentiert und widerrufbar; Zugangsdaten ausschliesslich als $ENV-Platzhalter (ATC-STD-203); Agent-Kommunikation authentifiziert, nie anonym.

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203, ATC-AAS-001 · INFORMATIVE: AGENT_MANIFEST.md v3.1.7, Registry-Kategorie aas
## 5. Ergänzende Verbindliche Betriebs- und Governance-Regeln

1. Passwörter, API-Keys und private Schlüssel MÜSSEN strikt aus Code, Commits, Logs und PRs ferngehalten werden.
2. Werden im Repository unbeabsichtigt Secrets entdeckt, MUSS der Agent sofort den Secret-Fund-Prozess einleiten.
3. Die Laufzeitumgebung des Agenten MUSS in einer isolierten Sandbox mit eingeschränkten Privilegien betrieben werden.
4. Der Zugriff auf Umgebungsvariablen MUSS über abgesicherte Secret-Management-Dienste erfolgen.
5. Ausgehende Netzwerkverbindungen MÜSSEN auf eine explizit genehmigte Whitelist beschränkt sein.
6. Secret-Scans MÜSSEN vor jedem Commit automatisiert im Workspace durchgeführt werden.

## 6. Metriken & Akzeptanzkriterien

- 0 im Repository freigelegte Secrets MÜSSEN im Codebasis-Bestand verbleiben (100% Secret-Free).
- 100% der Laufzeitumgebungen MÜSSEN in isolierten Sandboxes betrieben werden.
- Reaktionszeit bei Secret-Funden MUSS unter 10 Sekunden liegen.
- Netzwerkverbindungen außerhalb der Whitelist MÜSSEN zu 100% blockiert werden.

## 7. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) überwacht werden. Verstöße werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 eingeordnet. Secret-Funde MÜSSEN sofort behoben werden.

## 8. Security Considerations

Dieser Standard stellt die Kern-Sicherheitsanforderungen für den Betrieb von KI-Agenten dar. Schutz vor Secret-Leaks hat oberste Priorität. Umgebungsverletzungen MÜSSEN den Agenten isolieren.

## 9. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 10. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-002, AI-DEV-005, AI-DEV-011, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET

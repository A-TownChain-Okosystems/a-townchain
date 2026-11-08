# Approval Decision — ATC-STD-DESC-001 + ATC-STD-VERSION-001 (v1.0.0)

**Datum:** 07.09.2026, 21:57 UTC+2 · **Entscheider:** Owner (Michael Wroblewski) · **Status: APPROVED**

## Owner-Approval

**Entscheidung:** **APPROVED / FREIGEGEBEN** am 07.09.2026, 21:57 UTC+2.
Owner: Michael Wroblewski — Sammel-Freigabe per Owner-Direktmandat
„Alle freigegeben" im Builder-Chat (beide DRAFT-Standards).

**Gegenstand:**

1. **ATC-STD-DESC-001 v1.0.0** (DRAFT → APPROVED) — Standard Description
   Standard: Pflichtstruktur (14 Bereiche), erweitertes Metadatenmodell,
   7-Status-Dokumentationslifecycle (Mapping auf ATC-STD-000 §10),
   14 REQ-DESC-Anforderungen, 5 COM-DESC-Gates, Ausnahmeverfahren,
   Quality Gate (17 Kriterien), Maschinenlesbarkeits-Modell.
   SCR-0008, Owner-Entwurf 07.09. 21:42 UTC+2.

2. **ATC-STD-VERSION-001 v1.0.0** (DRAFT → APPROVED) — ATC Versioning
   Standard: SemVer 2.0.0, Pre-Release-Kette, getrennte Versionsebenen
   (Software/API/Protokoll/Konsens/State/Network/Contracts/Agents/Doku),
   Git-Tags, kanonische Versionsquelle, Release-/Build-IDs (Schema:
   releaseId-Dualformat + buildId), Release-Manifest, Monorepo-Regel,
   Verbotene Praktiken, Golden Rule. 22 REQ-VERSION-Anforderungen,
   5 COM-VERSION-Gates. SCR-0009, Owner-Entwurf 07.09. 21:50 UTC+2.

**Harmonisierungen (dokumentiert in SCR-0008/SCR-0009):**
DESC-001: Lifecycle-Mapping dokumentarisches Modell ↔ Registry-Modell;
VERSION-001: releaseId-Dualformat (ATC-REL-X.Y.Z + ATC-REL-YYYYMMDD-NNN),
Status-Katalog gilt für Software-Releases, Standards behalten §10-Lifecycle.

**Evidenz bei Freigabe:** validate_all 105/105 COMPLIANT · Repo-Audit R3
100/100 GATE PASS · DAG 105 Knoten / 226 Kanten, azyklisch · beide
Standards einzeln validiert S-01..S-16 COMPLIANT (0 Fails, 0 Warns).

**Wirkung:** Beide Standards sind ab sofort APPROVED und normativ
(`normative: true`). ACTIVE-äquivalent gemäß REQ-DESC-004/REQ-VERSION-006
erfolgt mit Registry-Status `stable` zu einem späteren Zeitpunkt.

**Immutabilität:** Ab APPROVED sind beide Standards gemäß ATC-STD-000 §30
eingefroren — Änderungen nur noch via SCR.

Verwandte Dokumente: SCR-0008, SCR-0009, AGENT_MASTERRULES.md

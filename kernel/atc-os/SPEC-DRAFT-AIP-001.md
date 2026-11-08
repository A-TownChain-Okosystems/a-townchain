# SPEC-DRAFT: ATC-97 / AIP-001 — Agent Interaction Protocol v1 (0.1.0-DRAFT)

**Status: 0.1.0-DRAFT (SCR-0088, 10.09.2026) — §9-Freigabe ausstehend, NICHT normativ.**
**Verortung:** Issue a-townchain-os#80 (Sprint 3.0, AD-005, Tasks T-801/T-802) - Kanonisch: atc-standards (ATC-97) - Referenz-Implementierungen: atc-shivacore (P2P-Layer K14) / aurora-ai (Agent Runtime) - Envelope-Basis: ATC-PROTO-P2P-001 (9+1-Feld-Envelope, deterministische Serialisierung)

## 1. Ziel
Verbindliches Interaktionsprotokoll fuer KI-Agenten im ATC-Oekosystem: Message-Passing (JSON-RPC 2.0 ueber das ShivaCore-P2P-Transport-Layer) und Task-Delegation zwischen Agenten.

## 2. Nachrichtenschicht (T-801) - REQ-AIP-001..005
- **REQ-AIP-001 (MUSS):** Jede Agentennachricht = P2P-Envelope (ATC-PROTO-P2P-001) mit Payload-Typ application/jsonrpc-2.0+atc.
- **REQ-AIP-002 (MUSS):** JSON-RPC-2.0-konforme Struktur (jsonrpc/method/params/id) mit ATC-Erweiterung atc: agent_did, task_id, session, ttl, capability_token.
- **REQ-AIP-003 (MUSS):** Deterministische Serialisierung (sort_keys, compact) — identisch zur ATC-Protokoll-Konvention; Signaturen per DID-Authentisierung (P2P-001).
- **REQ-AIP-004 (MUSS):** Methoden-Namespace atc.aip.v1.* (ping, describe, request_task, delegate, report, cancel, finalize).
- **REQ-AIP-005 (MUSS):** Capability-Token: Delegation nur innerhalb der delegierten Capabilities (ai/capabilities.yaml als Autorisierungs-SSOT; KEIN Elevations-Pfad).

## 3. Task-Delegation-Proto (T-802) - REQ-AIP-006..010
- **REQ-AIP-006 (MUSS):** Task-Lifecycle: OFFERED -> CLAIMED -> RUNNING -> REPORTED -> FINALIZED (kein Sprung, kein paralleler Claim; Atomicity via Consensus-Receipt bei kettenkritischen Tasks).
- **REQ-AIP-007 (MUSS):** Delegation-Request enthaelt: Ziel-Capability, Akzeptanzkriterien, max. Dauer, Beweis-Pflicht (Artefakt-Referenz), Rollback-Hinweis.
- **REQ-AIP-008 (MUSS):** Agenten melden Finalisierung NUR mit Evidence-Artefakt (Claim-to-Artifact-Pipeline, ATC-STD-REPO-AUDIT-001) — kein Selbst-Zertifikat.
- **REQ-AIP-009 (MUSS):** Timeout/Split-Brain: nicht finalisierte Tasks laufen nach TTL in EXPIRED und werden fuer Re-Delegation freigegeben; Idempotenz via task_id + nonce.
- **REQ-AIP-010 (MUSS):** Human-Gate: kritische Delegationen (security_class S4, capability p0) erfordern Owner-Approve (ai/policies.yaml AP-016).

## 4. Sicherheit
Untrusted-Prinzip: empfangene Agentenberichte sind RATSCHLAG, niemals Wahrheit — Verifikation ueber Evidence-Gate. Replay-Schutz via nonce+ttl; Rate-Limit pro agent_did (K15-Primitiven).

## 5. Testkategorien
Envelope-Conformance, Serialisierungs-Determinismus (Differential-Stil wie atclang), Delegations-Lifecycle-Suite, Capability-Ueberschreitungs-Versuche (muessen FAILen), TTL/Expiry-Rennen, Replay-Attacken.

## 6. Status-Gates
0.1.0-DRAFT (dieser Stand) -> 0.2.0 nach ShivaCore-Bindungs-Review -> §9-Owner-Freigabe -> v1.0.0 nach Conformance-Suite gruen (ATC-STD-PROTOCOL-002 CONF-BRONZE-Pfad).

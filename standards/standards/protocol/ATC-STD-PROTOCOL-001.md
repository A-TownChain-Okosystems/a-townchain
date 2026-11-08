---
standard:
  id: ATC-STD-PROTOCOL-001
  title: "ATC Protocol Standards — Dachstandard oberhalb der Einzelprotokolle: einheitliche Regeln für Identität, Versionierung, Nachrichten, Sicherheit, Fehler, Kompatibilität, Governance und Auditing aller ATC-Protokolle (ATC-PROTO-*)"
  version: "1.0.0"
  status: approved
  category: protocol
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  classification: PUBLIC
  approved_by: "Michael Wroblewski (Owner, §9-Freigabe 08.09.2026, 00:18 UTC+2)"
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-VERSION-001
    - ATC-STD-COMPAT-001
    - ATC-STD-UPDATE-001
    - ATC-STD-AUDIT-001
    - ATC-STD-FRAMEWORK-001
  related_standards:
    - ATC-STD-203
    - ATC-STD-204
    - ATC-STD-BUG-005
    - ATC-STD-MILESTONE-001
    - ATC-STD-REPO-AUDIT-001
    - ATC-STD-AOS-001
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
----

# ATC-STD-PROTOCOL-001 — ATC Protocol Standards (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — §9-Freigabe Michael Wroblewski (Builder-Chat 08.09.2026, 00:18 UTC+2);
> normativ in Kraft ab 08.09.2026, §30-eingefroren (ATC-STD-000). SCR-0023 akzeptiert.
> **Familie:** Protocol Standards (ATC-STD-PROTOCOL-001..999, FAM-42 — neuer
> Namensraum gem. FRAMEWORK-001 §6.3). **Maschinenlesbar:**
> registry/protocol-registry.yaml (SSOT, 26 ATC-PROTO-Familien, Validator S-23).

## Abstract

ATC-STD-PROTOCOL-001 ist der Dachstandard ÜBER allen Einzelprotokollen des
Ökosystems (Netzwerk, Blockchain, API, P2P, Cross-Chain, AI-Agent, Storage, System):
Jedes ATC-Protokoll folgt denselben Regeln für Governance, Architektur, Identität,
Nachrichten, Encoding, Versionierung, Kompatibilität, Handshake, Authentication,
Authorization, Security, Kryptografie, Fehler, Timeouts, Rate-Limiting, Replay-
Schutz, Observability, Audit, Upgrades und Registry. Neue Protokolle werden nicht
mehr individuell und inkonsistent definiert — sie instanziieren diesen Standard
als ATC-PROTO-[DOMAIN]-[NUMBER] in der zentralen Protocol-Registry.

Schlüsselwörter: MUSS/MUSS NICHT (absolut verbindlich), SOLLTE (empfohlen),
DARF/KANN (optional) — RFC-2119 gemäß ATC-STD-000 §10.

## Scope

**Gilt:** Alle ATC-Netzwerk-, Blockchain-, API-, P2P-, Cross-Chain-, AI-Agent-,
Storage- und Systemprotokolle (L0–L7), ab Neuanlage; Bestandprotokolle werden
bei ihrem nächsten MAJOR/UPDATE nachgezogen (UPDATE-001).

**Gilt nicht:** Applikations-APIs ohne Netz-/Konsensbezug (folgen ATC-STD-202/203);
externe Protokolle, die ATC nur konsumiert (dürfen diesen Standard freiwillig
anwenden, wo ATC sie erweitert).

## §1 Protocol Governance

Jedes Protokoll MUSS eindeutig definiert sein (REQ-PROTO-001). Pflichtfelder:
Protocol ID, Name, Version, Status, Owner, Scope, Specification, Test Suite,
Security Model, Compatibility Rules, Changelog, Deprecation Policy; Reference
Implementation empfohlen. Protokoll-Instanzbeispiel (P2P): ID ATC-PROTO-P2P-001,
Transport QUIC, Encoding CBOR, Security TLS 1.3, Authentication ATC Identity.

## §2 Protocol Architecture Standard

Jedes Protokoll MUSS nach Schichten organisiert sein (REQ-PROTO-002):
Application → Message/API → Identity/Authentication → Authorization →
Security/Encryption → Transport → Network/Discovery. Blockchain-Logik und
Transportlogik DÜRFEN NICHT unkontrolliert vermischt werden; Schichtverletzungen
sind Design-Findings (GAP-ARCH).

## §3 Protocol Identification Standard

Jedes Protokoll erhält eine eindeutige ID nach dem Schema
`ATC-PROTO-[DOMAIN]-[NUMBER]` (REQ-PROTO-003) — registriert in der Protocol-
Registry (§20). Beispiele: ATC-PROTO-P2P-001, ATC-PROTO-CONSENSUS-001,
ATC-PROTO-TX-001, ATC-PROTO-BRIDGE-001, ATC-PROTO-ZKP-001, ATC-PROTO-IDENTITY-001,
ATC-PROTO-STORAGE-001, ATC-PROTO-AI-001.

## §4 Message Standard

Alle Protokollnachrichten MÜSSEN einen standardisierten Envelope besitzen
(REQ-PROTO-004) mit 9 Pflichtfeldern: `protocol`, `version`, `message_type`,
`message_id`, `timestamp`, `sender`, `nonce`, `payload`, `signature`. Der Envelope
ist die Grundlage für Replay-Schutz (§16), Observability (§17) und Audit (§18).

## §5 Encoding Standard

ATC verwendet KEIN beliebiges Format je Protokoll (REQ-PROTO-005):
Human-readable (JSON/YAML/TOML) für Konfiguration, Dokumentation und APIs;
Machine (CBOR/Protobuf) für Netzwerkkommunikation; Blockchain: deterministische
Serialisierung ZWINGEND. **Grundregel:** Gleiche Eingabedaten erzeugen auf jedem
Node exakt dieselben Bytes.

## §6 Versioning Standard

Protokolle verwenden Semantic Versioning MAJOR.MINOR.PATCH (REQ-PROTO-006,
VERSION-001): MAJOR = Breaking Change, MINOR = neue kompatible Funktion, PATCH =
Bugfix/Sicherheitskorrektur. Zusätzlich MUSS je Protokoll definiert sein:
`supported_versions`, `minimum_version`, `maximum_version`, `deprecated_versions`.

## §7 Compatibility Standard

Jedes Protokoll MUSS definieren (REQ-PROTO-007): Backward-, Forward-, Node-, API-,
Data-, Network- und State-Compatibility. Bei MAJOR-Wechsel MUSS ein Compatibility
Layer v1→v2 geschaltet werden — alte Nodes dürfen NICHT unkontrolliert aus dem
Netzwerk fallen (COMPAT-001-Kopplung: keine UNKNOWN bei Release, Methoden A–F).

## §8 Handshake Standard

Kommunizierende ATC-Komponenten MÜSSEN vor Kommunikation Fähigkeiten austauschen
(REQ-PROTO-008): HELLO → PROTOCOL_NEGOTIATION → CAPABILITY_EXCHANGE →
AUTHENTICATION → KEY_EXCHANGE → SESSION_ESTABLISHED. Capabilities umfassen
mindestens: protocol_version, supported_consensus, supported_tx_versions,
supported_zkp, supported_compression, supported_encoding, supported_bridge_
protocols, supported_features.

## §9 Authentication Standard

Jede sicherheitsrelevante Kommunikation MUSS eine eindeutige Identität besitzen
(REQ-PROTO-009): Node/Validator/Wallet/Agent/Service/Organization/Contract ID.
**Grundsatz:** Eine technische Identität DARF NICHT ausschließlich über IP-Adresse
oder Hostname definiert werden.

## §10 Authorization Standard

Authentication („Wer bist du?") und Authorization („Was darfst du?") sind zu
trennen (REQ-PROTO-010): standardisiertes Capability-/Permission-Modell, z. B.
NODE_READ, NODE_WRITE, TX_SUBMIT, BLOCK_PROPOSE, BLOCK_VALIDATE, GOVERNANCE_VOTE,
BRIDGE_EXECUTE, ORACLE_PUBLISH, AI_EXECUTE, ADMIN_AUDIT.

## §11 Security Standard

Jedes Protokoll benÖtigt eine eigene Threat Model Specification (REQ-PROTO-011)
mit Pflichtprüfungen gegen: Replay, Man-in-the-Middle, Message Forgery, Signature
Forgery, Sybil, DoS/DDoS, Eclipse, Message Flooding, State Manipulation, Version
Downgrade, Credential Theft, Key Compromise.

## §12 Cryptography Standard

Kryptografie DARF NICHT direkt im Code fest verdrahtet werden (REQ-PROTO-012):
ATC Cryptographic Abstraction Layer mit austauschbaren Algorithmen-Klassen
(SignatureAlgorithm, HashAlgorithm, KeyExchangeAlgorithm, EncryptionAlgorithm,
ZKPAlgorithm) — Algorithmuswechsel ohne Neuimplementierung aller Protokolle.

## §13 Error Protocol Standard

Fehler MÜSSEN maschinenlesbar sein (REQ-PROTO-013): `error_code`
(ATC-PROTO-<KAT>-NNN), `category`, `message`, `retryable`, `severity`.
Fehlerklassen: PROTOCOL, NETWORK, AUTHENTICATION, AUTHORIZATION, VALIDATION,
CONSENSUS, STATE, CRYPTO, RESOURCE, TIMEOUT, RATE_LIMIT, SECURITY, INTERNAL.

## §14 Timeout-/Retry-Standard

Jedes Netzwerkprotokoll MUSS definieren (REQ-PROTO-014): Connection/Handshake/
Request/Response Timeout, Retry Limit, Backoff Strategy, Circuit Breaker.
**Keine unendlichen Retries.**

## §15 Rate-Limiting Standard

Protokolle MÜSSEN Flooding-Schutz besitzen (REQ-PROTO-015): Requests/s,
Messages/s, Bytes/s, Connections/IP, Connections/Identity, Transactions/Block —
konfigurierbar, mit Meldung via ERROR-Klasse RATE_LIMIT.

## §16 Replay Protection

Transaktionen und sicherheitsrelevante Nachrichten MÜSSEN Replay-geschützt sein
(REQ-PROTO-016): Nonce, Sequence Number, Timestamp, Expiration, **Chain ID**,
Domain Separation, Message ID. Blockchain-Transaktionen: eindeutige Chain ID
ZWINGEND (658467 im A-TownChain-Netz).

## §17 Observability Standard

Jedes Protokoll MUSS Ereignisse nachvollziehbar machen (REQ-PROTO-017) mit
Standardfeldern: protocol_id, protocol_version, message_id, request_id, trace_id,
sender_id, receiver_id, timestamp, result, error_code, latency. Integration: Logs,
Metrics, Tracing, AuditTrail, LogChain, Prometheus, Grafana.

## §18 Protocol Audit Standard

Jede kritische Protokolloperation MUSS auditierbar sein (REQ-PROTO-018): WHO,
WHAT, WHEN, WHERE, WHY, RESULT, SIGNATURE — besonders Validator-Changes, Consensus-
Changes, Bridge-Operations, Governance-Operations, Protocol Upgrades, Security
Events, AI-Agent-Actions, Admin-Actions. Nachweis über AUD-Records (AUDIT-001).

## §19 Protocol Upgrade Standard

Protokoll-Upgrades durchlaufen verbindlich (REQ-PROTO-019): Proposal →
Specification → Implementation → Unit Tests → Integration Tests → Compatibility
Tests → Security Audit → Testnet → Governance Approval → Deployment → Monitoring →
Activation. Für Blockchain-Protokolle zusätzlich: Activation Height, Activation
Time, Migration Strategy, Rollback Strategy. Ohne Governance Approval (Human Gate)
keine Aktivierung.

## §20 Protocol Registry

Die zentrale Registry lebt im atc-standards-Repository als SSOT (REQ-PROTO-020):
`registry/protocol-registry.yaml` (generiert von tools/protocol/gen_protocol_
registry.py), Validierung durch Validator S-23 je CI-Lauf. Eintrag = Autorisierung:
Ein Protokoll OHNE Registry-Eintrag ist kein ATC-Protokoll.

## §21 ATC Protocol-Matrix (26 Familien)

Verbindliche Protokollfamilien des Ökosystems (Registry-Initialstand, SCR-0023):
P2P, NODE, CONSENSUS, BLOCK, TX, MEMPOOL, VALIDATOR, STAKING, MINING, WALLET,
IDENTITY, REPUTATION, GOVERNANCE, ORACLE, ZKP, BRIDGE, IBC, DATA, STORAGE, AI,
AGENT, API, EVENT, AUDIT, UPGRADE — je als ATC-PROTO-<DOMAIN>-001 mit Priorität
und Status in der Registry. **Implementierungsreihenfolge:** P0 Fundament = dieser
Dachstandard (Governance, ID, Message, Versioning, Compatibility, Errors, Security,
Cryptography); P1 Blockchain = P2P, Node, TX, Block, Mempool, Consensus, Validator,
Staking/Mining; P1 Interoperabilität = Identity, ZKP, Oracle, Bridge, IBC;
P2 Ökosystem = AI, Agent-to-Agent, Storage, API, Event, Audit, Governance.
Ehrlichkeitsregel: `active`/`experimental` nur mit existierender Implementierung;
Baseline-Status ohne Implementierung ist `planned` (REQ-PROTO-021).

## Requirements (normativ)

- **REQ-PROTO-001** (§1): 12+ Pflichtfelder je Protokoll in der Registry.
- **REQ-PROTO-002** (§2): 7-Schichten-Architektur; keine Schichtvermischung.
- **REQ-PROTO-003** (§3): ID-Schema ATC-PROTO-[DOMAIN]-[NUMBER], Registry-Pflicht.
- **REQ-PROTO-004** (§4): 9-Felder-Nachrichten-Envelope für alle Nachrichten.
- **REQ-PROTO-005** (§5): Encoding-Standard; Blockchain deterministisch.
- **REQ-PROTO-006** (§6): SemVer + supported/minimum/maximum/deprecated_versions.
- **REQ-PROTO-007** (§7): 7 Kompatibilitätsdimensionen; Compatibility Layer bei MAJOR.
- **REQ-PROTO-008** (§8): 6-Phasen-Handshake mit Capability Exchange.
- **REQ-PROTO-009** (§9): eindeutige Identitäten; niemals nur IP/Hostname.
- **REQ-PROTO-010** (§10): Capability-/Permission-Modell für Authorization.
- **REQ-PROTO-011** (§11): Threat Model je Protokoll, 12 Pflichtprüfungen.
- **REQ-PROTO-012** (§12): Cryptographic Abstraction Layer, keine Festverdrahtung.
- **REQ-PROTO-013** (§13): maschinenlesbare Fehler mit ATC-PROTO-<KAT>-NNN.
- **REQ-PROTO-014** (§14): Timeouts, Retry-Limits, Circuit Breaker.
- **REQ-PROTO-015** (§15): konfigurierbare Rate-Limits.
- **REQ-PROTO-016** (§16): Replay-Schutz inkl. Chain ID für Transaktionen.
- **REQ-PROTO-017** (§17): 11 Observability-Standardfelder.
- **REQ-PROTO-018** (§18): Auditierbarkeit kritischer Operationen (AUD-Records).
- **REQ-PROTO-019** (§19): 13-Schritte-Upgrade-Prozess; Blockchain-Zusatzfelder.
- **REQ-PROTO-020** (§20): registry/protocol-registry.yaml ist SSOT; S-23 prüft.
- **REQ-PROTO-021** (§21): 26 Protokollfamilien verbindlich; Status-Ehrlichkeitsregel.

## Security Considerations

Der Standard selbst ist Sicherheitsarchitektur: Envelope-Signatur, Handshake-
Authentifizierung, Replay-Schutz (Chain ID), Rate-Limits und Threat Models sind
Pflichtbestandteile jedes Protokolls. Protocol-Registry-Manipulation wäre ein
Angriff auf die Protokoll-Autorisierung — Registry-Änderungen nur via SCR;
S-23 erzwingt Integrität je CI-Lauf. `active`-Status ohne verifizierte
Implementierung wäre ein Sicherheitsversprechen ohne Basis — daher Status-
Ehrlichkeitsregel (REQ-PROTO-021).

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog (Standard-intern)

- **1.0.0** (2026-09-08): Initial Release — Owner-Entwurf Michael Wroblewski
  (21 Abschnitte, 26-Familien-Matrix mit P0/P1/P2-Reihenfolge); maschinenlesbar:
  registry/protocol-registry.yaml (SSOT, 26 ATC-PROTO-Familien, generiert von
  tools/protocol/gen_protocol_registry.py) + Validator NEU S-23 (Negativtest
  verifiziert); Kopplungen VERSION/COMPAT/UPDATE/AUDIT/FRAMEWORK; Status-
  Ehrlichkeitsregel (planned als Baseline). SCR-0023; §9-Freigabe Michael Wroblewski 08.09.2026, 00:18 UTC+2 — APPROVED, normativ, §30-eingefroren.

## References

- registry/protocol-registry.yaml + tools/protocol/gen_protocol_registry.py (SSOT)
- ATC-STD-VERSION-001 (§6), ATC-STD-COMPAT-001 (§7), ATC-STD-UPDATE-001 (§19)
- ATC-STD-AUDIT-001 (§18 AUD-Records), ATC-STD-BUG-005 (Findings/RCA)
- ATC-STD-FRAMEWORK-001 (FAM-42, §6.3 Namensraum-Regel), ATC-STD-203/204
- ShivaCore K12/K14/K16 (P2P/Consensus-Impl.-Spuren), a-townchain Chain-ID 658467

*ATC-STD-PROTOCOL-001 v1.0.0 · Owner-Entwurf Michael Wroblewski · Aurora (Superagent) · 08.09.2026 · SCR-0023*

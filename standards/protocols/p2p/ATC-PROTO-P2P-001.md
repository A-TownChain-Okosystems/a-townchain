---
protocol:
  id: ATC-PROTO-P2P-001
  name: "ATC Peer-to-Peer Protocol — Verbindliche Spezifikation des P2P-Netzwerkschichts-Protokolls (Discovery, Peer-Lifecycle, Envelope, Gossip, Handshake, Sicherheit, Versionierung)"
  version: "1.0.0"
  status: approved
  domain: P2P
  layer: L1/L3
  chain_id: 658467
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  classification: PUBLIC
  language: de-DE
  umbrella: ATC-STD-PROTOCOL-001
  specification: "protocols/p2p/ATC-PROTO-P2P-001.md (diese Datei)"
  test_suite: "atc-shivacore modules/atc-shivacore/kernel/src/p2p.rs — 30 Unit-Tests (K14, alle grün)"
  reference_implementation: "ShivaCore K12 (Netzwerk-Stack) + K14 (P2P-Consensus Foundation, p2p.rs 861 LOC) + K13 (TCP/IP) + K6/K6b (DID, Ed25519) + K15 (Security Layer)"
  dependencies:
    - ATC-STD-PROTOCOL-001
    - ATC-STD-000
    - ATC-STD-VERSION-001
    - ATC-STD-COMPAT-001
    - ATC-STD-UPDATE-001
  related_protocols:
    - ATC-PROTO-NODE-001
    - ATC-PROTO-CONSENSUS-001
    - ATC-PROTO-BLOCK-001
    - ATC-PROTO-TX-001
    - ATC-PROTO-IDENTITY-001
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-PROTO-P2P-001 — ATC Peer-to-Peer Protocol (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — §9-Freigabe Michael Wroblewski (Builder-Chat
> 08.09.2026, 01:06 UTC+2, SCR-0027); Spezifikation verbindlich. Protokoll-Status
> bleibt bis zur verifizierten v1.0.0-Implementierung ehrlich `draft` (REQ-PROTO-021).
> **Registry:** registry/protocol-registry.yaml, Domain P2P (Impl.-Spuren ShivaCore K12/K14).
> **Referenzimplementierung:** atc-shivacore K14 `p2p.rs` (30 Tests) — das
> bestehende Wire-Format wird hier als **v0.9-Kompatibilitätsmodus** dokumentiert
> und nach normativem v1.0.0-Envelope weiterentwickelt.

## Abstract

ATC-PROTO-P2P-001 ist die verbindliche Spezifikation des Peer-to-Peer-Protokolls
der A-TownChain (Chain-ID 658467): Peer-Discovery, Peer-Lifecycle, Nachrichten-
Envelope, deterministisches Encoding, Gossip-Propagation, Handshake, DID-basierte
Authentisierung, Fehlerbehandlung, Timeouts, Rate-Limiting, Replay-Schutz, Observability
und Versionierung. Die Spezifikation instanziert ATC-STD-PROTOCOL-001 vollständig
(§1–§19) und dokumentiert zugleich ehrlich den Ist-Zustand der Referenzimplementierung
(ShivaCore K12/K14): Was implementiert ist (v0.9-Subset), was v1.0.0 normativ fordert
und über welche Kompatibilitätsregel beides koexistiert.

Schlüsselwörter: MUSS/MUSS NICHT (absolut verbindlich), SOLLTE (empfohlen),
DARF/KANN (optional) — RFC-2119 gemäß ATC-STD-000 §10.

## §1 Identität & Metadaten (REQ-P2P-001)

| Pflichtfeld (PROTOCOL-001 §1) | Wert |
|---|---|
| Protocol ID | ATC-PROTO-P2P-001 |
| Name | ATC Peer-to-Peer Protocol |
| Version | 1.0.0 (DRAFT) |
| Status | Spezifikation APPROVED (§9-Freigabe 08.09. 01:06); Protokoll-Registry-Status `draft` bis verifizierte v1.0.0-Implementierung |
| Owner | Michael (Owner) / Standards Governance |
| Scope | P2P-Netzwerkschicht: Discovery, Peer-Verwaltung, Nachrichtenaustausch, Gossip; NICHT Konsens-Logik (ATC-PROTO-CONSENSUS-001) und NICHT Block-/Tx-Semantik (ATC-PROTO-BLOCK/TX-001) |
| Specification | diese Datei (protocols/p2p/ATC-PROTO-P2P-001.md) |
| Test Suite | 30 Unit-Tests in p2p.rs (K14) + Conformance-Suite (ATC-PROTO-P2P-TEST, PROTOCOL-002-Kopplung, GEPLANT) |
| Security Model | §10 (Threat Model mit 12 Pflichtprüfungen) |
| Compatibility Rules | §18 (8 Kompatibilitätsarten, v0.9-Modus) |
| Changelog | §21 |
| Deprecation Policy | §22 |
| Reference Implementation | ShivaCore K12/K13/K14/K15 (atc-shivacore) |

## §2 Schichtenarchitektur (REQ-P2P-002)

Organisierung nach PROTOCOL-001 §2 — Blockchain-Logik und Transportlogik sind
getrennt (Schichtverletzung = GAP-ARCH-Finding):

| Schicht | P2P-Zuständigkeit | ShivaCore-Modul (Ist) |
|---|---|---|
| Application | Block-/Tx-Announcement-Weitergabe, Vote-Transport | K14 `announce_block`/`announce_tx`, consensus.rs |
| Message/API | Envelope, Message-Types, Serialisierung | K14 `P2pMessage::to_bytes/from_bytes` |
| Identity/Authentication | DID-basierte Peer-Identität | K6 (DID) + K6b (Ed25519), `Peer.did` |
| Authorization | Peer-Capabilities (§9) | GEPLANT v1.0.0 |
| Security/Encryption | Session-Keys, Signaturen | K15 Secure-Channel; GEPLANT v1.0.0 |
| Transport | TCP/UDP-Sockets, Ports | K13 (TCP/IP), K12 (Ethernet/ARP) |
| Network/Discovery | PeerTable, PeerList-Austausch, Bootstrap | K14 `PeerTable`, `make_peer_list` |

## §3 Nachrichten-Envelope v1.0.0 (REQ-P2P-003)

Jede P2P-Nachricht MUSS den ATC-Envelope mit den 9 Pflichtfeldern nach
PROTOCOL-001 §4 besitzen, erweitert um `chain_id` (Replay-Schutz, §15):

| # | Feld | Typ | Pflicht | Bedeutung |
|---|---|---|---|---|
| 1 | `protocol` | string | MUSS | konstant `ATC-PROTO-P2P` |
| 2 | `version` | semver | MUSS | Absender-Protokollversion |
| 3 | `message_type` | u8 | MUSS | Typ-Tabelle §4 |
| 4 | `message_id` | bytes32 | MUSS | eindeutig (Zufall oder Hash) |
| 5 | `timestamp` | u64 | MUSS | Millisekunden seit Epoch, UTC |
| 6 | `sender` | string | MUSS | DID des Absenders (K6) |
| 7 | `nonce` | bytes16 | MUSS | Replay-Schutz (§15) |
| 8 | `payload` | bytes | MUSS | typspezifisch (§4) |
| 9 | `signature` | bytes | MUSS | Ed25519-Signatur über kanonische Serialisierung (§11) |
| 10 | `chain_id` | u32 | MUSS | 658467 (A-TownChain) — Abweichung = Disconnect |

### 3.1 Deterministisches Encoding (REQ-P2P-004)

Gleiche Eingabedaten erzeugen auf jedem Node exakt dieselben Bytes
(PROTOCOL-001 §5): Signatur-Grundlage ist die kanonische Byte-Serialisierung —
`protocol[16] || version-major[1] || version-minor[1] || version-patch[1] ||
message_type[1] || message_id[32] || timestamp[8] || sender_len[2] || sender ||
nonce[16] || payload_len[4] || payload || chain_id[4]` (Big-Endian, keine
Felder variabler Reihenfolge). Userspace-/Service-APIs DÜRFEN zusätzlich
deterministisches CBOR (RFC 8949 §4.2.1) verwenden; die Kernel-Wire-Form ist
die Byte-Form.

### 3.2 v0.9-Kompatibilitätsmodus (Ist-Zustand K14)

Das bestehende K14-Wire-Format ist als v0.9 dokumentiert und bleibt solange
gültig, bis v1.0.0 aktiviert ist (Upgrade-Kette §19): `type[1] || chain_id[4]
|| did_len[2] || did || timestamp[8] || payload` — Big-Endian, Chain-ID-Abweisung
bei `!= 658467` implementiert (`P2pError::WrongChainId`). v0.9 enthält KEINE
Felder `version`, `message_id`, `nonce`, `signature` — genau diese Lücken sind
die Delta-Migration nach v1.0.0 (UPDATE-001 UPD-G-Kette; MINOR, da rückwärts
tolerant: v1.0.0-Nodes akzeptieren v0.9-Nachrichten mit eingeschränkter Sicherheit).

## §4 Message-Types (REQ-P2P-005)

Verbindliche Typ-Tabelle (Ist = K14 implementiert, NEU = v1.0.0):

| ID | Typ | Richtung | Payload | Status |
|---|---|---|---|---|
| 1 | Ping | → Peer | leer (v0.9) / Timestamp | Ist |
| 2 | Pong | ← Peer | `original_timestamp[8]` | Ist |
| 3 | Handshake (HELLO) | → Peer | v0.9: `listen_port[2]`; v1.0.0: + `supported_versions[]` | Ist/erweitert |
| 4 | HandshakeAck (SESSION_ESTABLISHED) | ← Peer | Capability-Bitmap (v1.0.0) | Ist/erweitert |
| 5 | BlockAnnounce | Broadcast | `block_hash[32] \|\| block_height[8]` | Ist |
| 6 | TxAnnounce | Broadcast | `tx_hash[32]` | Ist |
| 7 | Vote | Gossip | konsens-definiert (ATC-PROTO-CONSENSUS-001) | Ist |
| 8 | PeerList | ↔ Peer | `count[2] \|\| (ip[4]\|\|port[2]){count}` | Ist |
| 9 | Bye | → Peer | `reason_code[1]` | Ist |
| 10 | CapabilityExchange | ↔ Peer | Capability-Bitmap §8 | NEU v1.0.0 |
| 11 | AuthChallenge | ← Peer | `challenge[32]` | NEU v1.0.0 |
| 12 | AuthResponse | → Peer | `signature[64]` (Ed25519 über Challenge) | NEU v1.0.0 |
| 13 | KeyExchange | ↔ Peer | X25519-Public (32) + AKD | NEU v1.0.0 |

Unbekannte Typ-IDs MÜSSEN mit `ATC-PROTO-P2P-002` (UnknownMessageType) abgewiesen
und DÜRFEN NICHT zum Disconnect führen (Forward-Kompatibilität).

## §5 Peer-Lifecycle & PeerTable (REQ-P2P-006)

Peer-Statusmaschine (Ist-Zustand K14, `PeerStatus`): `Disconnected → Connecting →
Connected → (Bye/Fehler/Timeout) → Disconnected`. v1.0.0 ergänzt `Banned` (§10,
nach Verstoß) und `Verified` (nach erfolgreicher AuthResponse).

| Regel | Normativ |
|---|---|
| Max-Peer-Grenze | MUSS konfigurierbar sein (Ist: `max_peers` in PeerTable); Default 64 |
| Identität | Peer-Identität MUSS die DID sein, sob bekannt; IP/Port sind nur Transport-Adresse (PROTOCOL-001 §9) |
| Liveness | PING-Intervall 30 s; 2 verpasste PONGs = Status `Connecting`; 3 = Disconnect (§13) |
| Statistik | `bytes_sent`, `bytes_recv`, `last_seen` MUSS je Peer geführt werden (Ist) — Basis für §14 |
| Tabellen-Sicherheit | Mutex-geschützte ID- und (IP,Port)-Indizes (Ist); Doppel-Connect MUSS abgewiesen werden (`PeerAlreadyConnected`) |

## §6 Peer-Discovery (REQ-P2P-007)

1. **PeerList-Austausch (Ist):** Peers tauschen Typ-8-Listen auf Anfrage/become
   aktuell; Aufnahme in Tabelle nur nach Connect-Versuch.
2. **Bootstrap:** Mindestens eine konfigurierbare Seed-Liste MUSS existieren
   (DID + IP/Port); im Kernel: Start-Konfiguration, Userspace: DNS/Config-Datei.
3. **Kademlia/DHT (v1.1, GEPLANT):** strukturierte Discovery — bis dahin gilt
   PeerList-Gossip als ausreichend für Testnet-Größenordnung.
4. **Eclipse-Schutz:** Ein Peer DARF NICHT mehr als 25 % der aktiven Peer-Tabelle
   aus einer einzigen /24-Subnetz-Basis beisteuern (SOLLTE; v1.0.0-Verifikation
   in Conformance-Tests).

## §7 Gossip-Propagation (REQ-P2P-008)

Broadcast MUSS als fan-out an alle `Connected`-Peers mit dediziziertem
Sendepuffer implementiert sein (Ist: `GossipProtocol::broadcast`). Empfangene
Announcements (Typ 5/6/7) MÜSSEN an die eigene Peer-Menge weitergepropagt
werden, ABER: jede Weiterleitung MUSS `message_id`-Deduplizierung nutzen
(v0.9: implizit über Height-Tracking; v1.0.0: explizite Seen-Set-Bloomfilter,
max 64 kiB) — sonst COUNT-Finding (Flooding). Halte-/Seen-Limits: §13/§14.

## §8 Handshake (REQ-P2P-009)

Sechs-Phasen-Handshake normativ (PROTOCOL-001 §8), mit Ist-Abbildung:

| Phase | Nachricht | K14-Ist |
|---|---|---|
| 1 HELLO | Typ 3 Handshake (DID, Versionen, listen_port) | Ist (2-Phasen-Subset: 3→4) |
| 2 PROTOCOL_NEGOTIATION | im HELLO-Payload: `supported_versions` → höchste gemeinsame Version | v1.0.0-NEU |
| 3 CAPABILITY_EXCHANGE | Typ 10 + Typ 4 (Bitmap: supported_consensus, supported_tx_versions, supported_zkp, supported_encoding, supported_compression, supported_features) | v1.0.0-NEU |
| 4 AUTHENTICATION | Typ 11 + Typ 12 (Ed25519-Challenge-Response, §9) | v1.0.0-NEU |
| 5 KEY_EXCHANGE | Typ 13 (X25519, Session-Keys via HKDF, §11) | v1.0.0-NEU |
| 6 SESSION_ESTABLISHED | Typ 4 HandshakeAck finalisiert den Zustand `Verified` | Ist (erweitert) |

Versionsspannen ohne Schnittmenge MÜSSEN mit `ATC-PROTO-P2P-005`
(HandshakeFailed, nicht kompatibel) beantwortet werden; ein Connect ohne
abgeschlossene 6 Phasen DARF KEINE Daten-Nachrichten (Typ 5–8) annehmen.

## §9 Authentication & Authorization (REQ-P2P-010)

**Authentication:** Jeder Peer MUSS eine K6-DID besitzen; die DID wird im HELLO
übertragen (Ist: `Peer.set_did`, `our_did`), die Besitzprüfung erfolgt in Phase 4
über Ed25519-Challenge-Response (K6b). Identität DARF NICHT ausschließlich über
IP/Host definiert werden (PROTOCOL-001 §9). Ungültige DID = `ATC-PROTO-P2P-006
(InvalidDID)` + Disconnect.

**Authorization:** Daten-Nachrichten sind an Capability-Bindungen gebunden
(PROTOCOL-001 §10): `NODE_READ` (alle Peers nach Phase 6), `BLOCK_PROPOSE` /
`VOTE` (nur Peers mit Validatoren-Capability — Ausgestellt über Identity-
Protokoll ATC-PROTO-IDENTITY-001), `TX_SUBMIT` (alle `Verified`). Capability-
Prüfung bei Empfang; Verstoß = `ATC-PROTO-P2P-012`.

## §10 Security & Threat Model (REQ-P2P-011)

Pflichtprüfungen nach PROTOCOL-001 §11 mit ehrlichem Status je Bedrohung:

| Bedrohung | Schutz (Normativ) | Status |
|---|---|---|
| Replay | nonce + message_id + timestamp-window (§15) | v1.0.0-NEU |
| Man-in-the-Middle | Ed25519-Auth + X25519-KeyExchange + Session-Keys | v1.0.0-NEU (K15-Bausteine) |
| Message Forgery | Signaturpflicht je Nachricht (Envelope-Feld 9) | v1.0.0-NEU |
| Signature Forgery | Ed25519 (K6b); Abweisung mit 013 | v1.0.0-NEU |
| Sybil | DID-Kostenmodell + Banning + Subnetz-Limit (§6.4) | teilweise (Rate-Limit Ist) |
| DoS/DDoS | §14 Rate-Limits + Circuit-Breaker (§13.4) | teilweise (Zähler Ist) |
| Eclipse | §6.4 Subnetz-Regel + Seed-Diversität | GEPLANT |
| Message Flooding | Seen-Set-Dedup + Gossip-Fanout-Cap (64) | teilweise |
| State Manipulation | PeerTable-Mutex + Validierung je Empfang | Ist |
| Version Downgrade | min_version-Untergrenze in Phase 2 (`ATC-PROTO-P2P-005`) | v1.0.0-NEU |
| Credential Theft | Schlüssel nur im Kernel-Keystore (K15); kein Klartextspeicher | K15-Baustein |
| Key Compromise | Rotation: Session-Keys je Verbindung; DID-Key-Rotation über IDENTITY-001 | GEPLANT |

Verstöße gegen Absender-Identität oder Flooding-Limits MÜSSEN Status `Banned`
setzen (Dauer: konfigurierbar, Default 24 h).

## §11 Cryptography (REQ-P2P-012)

Kryptografie MUSS über die ATC Cryptographic Abstraction Layer laufen
(PROTOCOL-001 §12) — keine direkte Algorithmus-Verdrahtung: `SignatureAlgorithm`
(Ed25519, Pflicht), `KeyExchangeAlgorithm` (X25519, Pflicht), `HashAlgorithm`
(SHA-256), `EncryptionAlgorithm` (ChaCha20-Poly1305 für Session-Payloads,
SOLLTE), `KDF` (HKDF-SHA256, Pflicht für Session-Keys). Signatur-Grundlage ist
die kanonische Serialisierung §3.1 mit Domain-Separation-Präfix
`ATC-P2P-v1\0` (verhindert Cross-Protokoll-Signatur-Reuse).

## §12 Error-Protocol (REQ-P2P-013)

Maschinenlesbare Fehler (PROTOCOL-001 §13) — K14-Fehler auf ATC-Codes gemappt:

| ATC-Code | K14-Error (Ist) | Kategorie | retryable | severity |
|---|---|---|---|---|
| ATC-PROTO-P2P-001 | MessageTooShort | VALIDATION | nein | WARN |
| ATC-PROTO-P2P-002 | UnknownMessageType(u8) | PROTOCOL | nein | WARN |
| ATC-PROTO-P2P-003 | WrongChainId(u32) | SECURITY | nein | CRITICAL → Disconnect |
| ATC-PROTO-P2P-004 | PeerNotFound | NETWORK | ja | INFO |
| ATC-PROTO-P2P-005 | HandshakeFailed(String) | PROTOCOL | ja | WARN |
| ATC-PROTO-P2P-006 | InvalidDID | AUTHENTICATION | nein | WARN → Disconnect |
| ATC-PROTO-P2P-007 | PeerAlreadyConnected | NETWORK | nein | INFO |
| ATC-PROTO-P2P-008 | NotConnected | NETWORK | ja | INFO |
| ATC-PROTO-P2P-009 | BroadcastFailed | NETWORK | ja | WARN |
| ATC-PROTO-P2P-010..019 | (reserviert v1.0.0: EnvelopeInvalid, SignatureInvalid, NonceReuse, RateLimited, CapabilityDenied, …) | — | — | — |

Fehlerstruktur: `error_code || category || message || retryable || severity`
(§13-Kopplung). `WrongChainId` MUSS zusätzlich als Security-Event auditiert
werden (§17).

## §13 Timeouts & Retry (REQ-P2P-014)

| Parameter | Wert (normativ) |
|---|---|
| Connection Timeout | 10 s |
| Handshake Timeout (Phase 1→6) | 30 s |
| Request/Response (PING→PONG) | 15 s |
| PING-Intervall | 30 s (Liveness §5) |
| Retry-Limit Connect | 3, Backoff exponentiell 2^n s, Basis 2 s |
| Kein unendliches Retry | MUSS — nach 3 Fehlversuchen Status `Disconnected` für 5 min |
| Circuit Breaker | 5 Timeouts/60 s je Peer → Peer für 10 min gesperrt (`Banned` kurzzeitig) |

## §14 Rate-Limiting (REQ-P2P-015)

Konfigurierbare Grenzen (Defaults): Messages/s je Peer 100; Bytes/s je Peer
256 kiB; Bytes/s global 4 MiB; Connections je Identität 8; Connections je IP 4;
Announcements/s global (Typ 5/6/7) 500. Überschreitung = `ATC-PROTO-P2P-014
(RATE_LIMIT)` + Zähler-Reset nach 60 s; wiederholte Überschreitung → `Banned`.
Fundament: K14 `bytes_sent/bytes_recv`-Zähler je Peer (Ist) als Messbasis.

## §15 Replay-Schutz (REQ-P2P-016)

Jede Nachricht MUSS `nonce` (16 Byte, einmalig je Absender-Epoche) und
`message_id` (32 Byte) tragen; Empfangs-Regeln: (1) `chain_id == 658467`
sonst Disconnect (Ist), (2) `timestamp` innerhalb ±120 s-Fenster, (3) keine
Wiederholung von `message_id` im Seen-Set, (4) Domain Separation §11.
Verstoß = `ATC-PROTO-P2P-012 (NonceReuse)` + Security-Event.

## §16 Observability (REQ-P2P-017)

Standardfelder je Ereignis (PROTOCOL-001 §17): protocol_id, protocol_version,
message_id, trace_id (v1.0.0: aus message_id abgeleitet), sender_id, receiver_id,
timestamp, result, error_code, latency_ms. Quelle: PeerTable-Statistik (Ist) +
Gossip-Zähler; Ziel: Kernel-Log (K38 devfs/klog) → AuditTrail → LogChain
(ATC-PROTO-AUDIT-001-Kopplung, GEPLANT).

## §17 Audit (REQ-P2P-018)

Kritische Operationen MÜSSEN auditierbar sein (PROTOCOL-001 §18): Peer-Connect/
Disconnect/Ban, WrongChainId-Funde, Handshake-Fehler, Rate-Limit-Verstöße,
Key-Rotation. WHO (DID), WHAT (message_type/error_code), WHEN (timestamp),
RESULT, SIGNATURE (Envelope) — Nachweis über AUD-Records (AUDIT-001).

## §18 Versioning & Kompatibilität (REQ-P2P-019)

- `supported_versions`: ["0.9", "1.0.0"] · `minimum_version`: "0.9" ·
  `maximum_version`: "1.0.0" · `deprecated_versions`: [] ("0.9" wird mit
  v1.1.0 deprecated, Abschaltung mit v2.0.0 — §22).
- SemVer nach VERSION-001: MAJOR = Breaking (Wire-Format inkompatibel) —
  ATC-STD-COMPAT-001 ist zwingendes Gate (UPD-G04), mit Compatibility-Layer
  v0.9→v1.0.0 (§3.2) nach COMPAT-Methode C (Side-by-Side).
- 8 Kompatibilitätsarten je PROTOCOL-001 §7 definiert: Backward (v1.0.0 liest
  v0.9 — ja), Forward (v0.9 ignoriert unbekannte Felder — ja), Node (Gemischtes
  Netz erlaubt während Migration), API (Envelope-Felder nur erweiterbar),
  Data (Seen-Sets versioniert), Network (Typ-IDs 1–9 unverändert), State
  (PeerTable unabhängig von Version), (keine State-Breaking bei MINOR).
- MAJOR v2 (Abkündigung v0.9) MUSS über Upgrade-Kette §19 laufen.

## §19 Upgrade-Prozess (REQ-P2P-020)

Aktivierung von v1.0.0 (und jede Folgoversion) durchläuft verbindlich
PROTOCOL-001 §19: Proposal (SCR) → Specification (diese Datei) → Implementation
(ShivaCore) → Unit Tests → Integration Tests → Compatibility Tests (COMPAT-001)
→ Security Audit → Testnet → **Governance Approval (Human Gate)** → Deployment →
Monitoring → Activation. Für Blockchain-Kopplung: Activation Height MUSS im
Upgrade-Request festgelegt sein; Migration §3.2, Rollback = v0.9-Modus bleibt
code-seitig erhalten bis v2.0.0.

## §20 Test Suite (REQ-P2P-021)

- **Unit (Ist):** 30 Tests in p2p.rs (K14): Envelope-Serialisierung/Parsing,
  Chain-ID-Abweisung, PeerTable-CRUD, Gossip broadcast/send_to/handle_message,
  Handshake/PingPong, UnknownMessageType.
- **Conformance (GEPLANT):** ATC-PROTO-P2P-TEST gem. PROTOCOL-002: 6-Phasen-
  Handshake-Ablauf, Capability-Verhandlung, Auth-Challenge-Response, Replay-
  Abweisung, Rate-Limit-Trigger, Eclipse-Regel, Interop v0.9/v1.0.0.
- **Mutation (bestehend):** S-19-Prinzip — jede Wire-Format-Änderung MUSS
  einen Negativtest in der Suite haben.

## §21 Changelog

- **1.0.0** (2026-09-08): Initial Release (SCR-0027) — erste formale
  Spezifikation unter ATC-STD-PROTOCOL-001: 22 REQ-P2P, v1.0.0-Envelope
  (9+1 Felder), 13 Message-Types, 6-Phasen-Handshake, Threat-Model-Status,
  Fehlerkatalog ATC-PROTO-P2P-001..019, Timeout-/Rate-Limit-Defaults, v0.9-
  Kompatibilitätsmodus (K14-Ist dokumentiert). §9-Freigabe Michael Wroblewski
  08.09.2026, 01:06 UTC+2 — APPROVED, verbindlich.

## §22 Deprecation Policy

Protokollversionen werden mit zweifacher Release-Ankündigung deprecated:
Ankündigung in v1.0.x-Dokumentation + `deprecated_versions`-Eintrag; Abschaltung
frühestens mit der übernächsten MAJOR-Version (v0.9 → deprecated in v1.1.0,
Abschaltung v2.0.0). `Bye` (Typ 9, `reason_code`) MUSS für kontrollierte
Disconnects genutzt werden. Ein Protokoll-Status `deprecated` in der Registry
erfordert einen Nachfolge-Eintrag oder eine explizite EOL-Begründung (SCR).

*ATC-PROTO-P2P-001 v1.0.0 DRAFT · Domain P2P · Chain-ID 658467 · SCR-0027 ·
Registry registry/protocol-registry.yaml · Copyright (c) 2026 Michael Wroblewski*

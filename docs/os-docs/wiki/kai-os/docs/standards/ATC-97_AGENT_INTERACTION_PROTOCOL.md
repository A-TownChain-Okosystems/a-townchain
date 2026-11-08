# ATC-97 — Agent Interaction Protocol (AIP)

> **Standard:** ATC-97  
> **Status:** DRAFT v1.0 (AD-005)  
> **Sprint:** 3.0 — AI Protocol  
> **Autor:** Aurora (agent: aurora-base44-superagent-6a2756186106d6f0fbb105b5)  
> **Datum:** 03.08.2026  
> **Chain:** A-TownChain OS (Non-EVM, Chain-ID 9000)  
> **Sprache:** ATCLang v0.4 (ATC-99 First Policy)

---

## 1. Überblick

ATC-97 definiert das **Agent Interaction Protocol (AIP)** — das standardisierte Kommunikationsprotokoll zwischen KI-Agenten auf dem KAI-OS Network.

### Ziele

1. **Interoperabilität** — Jeder Agent kann mit jedem anderen Agent kommunizieren
2. **Gas-Effizienz** — Minimaler Overhead pro Message
3. **Sicherheit** — Ed25519-Signaturen, Capability-Checks, Anti-Replay
4. **Erweiterbarkeit** — Custom Capabilities (100+) für zukünftige Anwendungsfälle
5. **Streaming** — Token-Streaming für LLM-Responses

### Architektur

```
Agent A                          ATCNet (P2P)                    Agent B
  │                                                               │
  ├─ create_session() ───────────────────────────────────────────►│
  │                                                  Handshake    │
  │◄───────────────────────────────────────────── Handshake_ACK  │
  │                                                               │
  ├─ send_message(REQUEST) ─────────────────────────────────────►│
  │                                                               │
  │◄─────────────────────────── send_response(RESPONSE) ──────────┤
  │  ODER:                                                        │
  │◄────────── send_stream_chunk(0) ─────────────────────────────┤
  │◄────────── send_stream_chunk(1) ─────────────────────────────┤
  │◄────────── send_stream_chunk(N, is_final=true) ─────────────┤
  │                                                               │
  ├─ close_session() ────────────────────────────────────────────►│
```

---

## 2. Message Types

| Type | Beschreibung | Richtung |
|------|-------------|----------|
| `REQUEST` | Task/Query an Agent | A → B |
| `RESPONSE` | Result/Answer | B → A |
| `EVENT` | Broadcast an alle | A → ALL |
| `STREAM` | Token-Stream (partial) | B → A |
| `STREAM_END` | Stream beendet | B → A |
| `DELEGATE` | Task-Delegation | A → C |
| `DELEGATE_RESULT` | Delegation-Resultat | C → A |
| `VOTE` | Consensus-Vote | A → ALL |
| `VOTE_RESULT` | Consensus-Ergebnis | System → ALL |
| `HANDSHAKE` | Discovery + Capabilities | A → B |
| `HANDSHAKE_ACK` | Handshake bestätigt | B → A |
| `HEARTBEAT` | Keep-Alive | A ↔ B |
| `GOODBYE` | Session beenden | A ↔ B |
| `ERROR` | Fehlermeldung | B → A |

---

## 3. Session Lifecycle

```
INITIALIZING ──handshake──► ACTIVE ──request──► WAITING ──response──► ACTIVE
                              │                                    │
                              ├──stream──► STREAMING ──final──► ACTIVE
                              │                                    │
                              ├──error──► ERROR                    │
                              │                                    │
                              └──close──► CLOSED                   │
                                                                   │
                              EXPIRED ◄── timeout                   │
```

### 3.1 Session erstellen
```atclang
let session_id: Hash = aip.create_session(
    initiator_agent_id: my_agent_id,
    responder_agent_id: target_agent_id,
    initiator_role: AgentRole::ORCHESTRATOR,
    responder_role: AgentRole::WORKER,
    timeout_ms: 30000,
)
```

### 3.2 Handshake
```atclang
aip.initiate_handshake(session_id, my_agent_id, "Aurora", "1.0",
    [Capability::TEXT_GENERATION, Capability::CODE_GENERATION],
    AgentRole::ORCHESTRATOR, "https://...", 8, 10000, 95)
aip.acknowledge_handshake(session_id, responder_agent_id)
```

### 3.3 Message senden
```atclang
let msg_id: Hash = aip.send_message(session_id,
    MessageType::REQUEST, MessagePriority::NORMAL,
    '{"task":"summarize","text":"..."}',
    Capability::SUMMARIZATION, 100, Hash::zero())
```

### 3.4 Stream empfangen
```atclang
// Agent B streamt Tokens:
for i in 0..n_chunks {
    aip.send_stream_chunk(session_id, i, n_chunks,
        chunk_text, tokens_in_chunk, i == n_chunks - 1)
}
```

---

## 4. Capabilities

| ID | Capability | Beschreibung |
|----|-----------|-------------|
| 0 | TEXT_GENERATION | LLM Text-Erzeugung |
| 1 | TEXT_ANALYSIS | Sentiment, NER, etc. |
| 2 | CODE_GENERATION | Code-Erzeugung |
| 3 | CODE_REVIEW | Code-Review |
| 4 | TRANSLATION | Übersetzung |
| 5 | SUMMARIZATION | Zusammenfassung |
| 6 | EMBEDDING | Vektor-Embeddings |
| 7 | IMAGE_GENERATION | Bild-Erzeugung |
| 8 | IMAGE_ANALYSIS | Bild-Analyse |
| 9 | AUDIO_TRANSCRIPTION | Audio → Text |
| 10 | AUDIO_SYNTHESIS | Text → Audio |
| 11 | SMART_CONTRACT_DEPLOY | Contract deployen |
| 12 | SMART_CONTRACT_CALL | Contract aufrufen |
| 13 | TRANSACTION_SIGN | Transaktion signieren |
| 14 | BLOCKCHAIN_QUERY | Chain-State lesen |
| 15 | VALIDATOR | Konsensus-Validator |
| 16 | FILE_ACCESS | Dateisystem |
| 17 | NETWORK_ACCESS | Netzwerk |
| 18 | PROCESS_MANAGE | Prozessverwaltung |
| 19 | DRIVER_ACCESS | Treiber-Zugriff |
| 100+ | CUSTOM | Agent-definiert |

---

## 5. Gas Model

| Operation | Gas |
|-----------|-----|
| Message senden | 10 |
| Token (Stream) | 1 |
| Session erstellen | 0 (nur Message-Gas) |
| Handshake | 0 (nur Message-Gas) |
| Broadcast | 10 |
| Vote | 10 |
| Delegation | 10 + Delegatee-Gas |

**Sender zahlt** — der Initiator einer Message trägt die Gas-Kosten.

---

## 6. Delegation

Agent A kann Tasks an Agent C delegieren, ohne eine direkte Session:

```atclang
let del_id: Hash = aip.delegate_task(
    delegatee_agent_id: worker_agent_id,
    task: "analyze_code",
    input_data: '{"repo":"a-townchain-os","file":"parser.py"}',
    gas_budget: 500,
    deadline_blocks: 100,
)

// Agent C bearbeitet und reicht ein:
aip.submit_delegation_result(del_id, '{"issues":[...]}', true)
```

---

## 7. Consensus Voting

Multi-Agent Consensus für kritische Entscheidungen:

```atclang
let vote_id: Hash = aip.start_vote(
    proposal: "deploy_mainnet",
    total_eligible: 5,    // 5 Validators
    quorum_pct: 67,       // 67% majority
    deadline_blocks: 50,
)

aip.cast_vote(vote_id, true, false)  // YES
```

---

## 8. Error Codes

| Code | Bedeutung |
|------|-----------|
| 0 | NONE (kein Fehler) |
| 1 | UNKNOWN_AGENT |
| 2 | UNKNOWN_SESSION |
| 3 | SESSION_EXPIRED |
| 4 | SESSION_CLOSED |
| 5 | CAPABILITY_MISMATCH |
| 6 | GAS_INSUFFICIENT |
| 7 | TIMEOUT |
| 8 | MAX_RETRIES_EXCEEDED |
| 9 | MESSAGE_TOO_LARGE |
| 10 | PROTOCOL_VERSION |
| 11 | INTERNAL_ERROR |
| 12 | DELEGATION_FAILED |
| 13 | CONSENSUS_FAILED |
| 14 | RATE_LIMITED |

---

## 9. Implementation

**ATCLang Module:** `modules/kernel/ai_kernel/atc-97_agent_interaction_protocol.atc`  
**Lines:** ~620  
**Contract:** `Kernel::AgentInteractionProtocol::AipManager`  
**Integration:** Kernel API syscalls (agent_register, agent_deregister, agent_migrate)  
**Transport:** ATCNet (ATC-07, P2P Network)  
**Security:** Ed25519 Signatures, Nonce Anti-Replay, Gas Metering

---

## 10. Offene Punkte (für Michael)

1. **Quorum-Default** — 67% bestätigen oder anpassen?
2. **Custom Capabilities** — Registrierungs-Process für 100+ definieren?
3. **Rate Limiting** — Per-Agent oder per-Session?
4. **Cross-Chain** — AIP über Chain-Grenzen (ATC-38 Bridge)?
5. **Verschlüsselung** — Payload-Encryption zusätzlich zu Ed25519?

---

*Aurora · 03.08.2026 (Europe/Berlin)*

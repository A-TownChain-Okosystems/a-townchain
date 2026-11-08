# ATC-97 — Agent Interaction Protocol

> **Standard-ID:** ATC-97  
> **Kategorie:** AI / Multi-Agent  
> **Tier:** 6 — Distributed Intelligence  
> **Status:** 📝 DRAFT — Spezifikation in Arbeit  
> **Version:** 0.1.0  
> **Autor:** Michael Wroblewski  
> **Sprint:** 3.0  
> **Wiki-Ref:** Kapitel 69  
> **Abhängigkeiten:** ATC-24, ATC-25, ATC-41  

---

## 1. Zweck

ATC-97 definiert das Protokoll für Interaktion zwischen KI-Agenten im KAI-OS Ökosystem. Es umfasst Message-Format, Orchestrierung, Konsens-Mechanismen und Fehlerbehandlung.

## 2. Message-Format

```atc
struct AgentMessage {
    sender: Address,
    receiver: Address,
    type: MessageType,
    payload: Bytes,
    nonce: u64,
    signature: Signature,
}
```

### 2.1 Nachrichtentypen
| Type | Beschreibung |
|------|-------------|
| `REQUEST` | Anfrage an anderen Agent |
| `RESPONSE` | Antwort auf REQUEST |
| `BROADCAST` | An alle Agenten |
| `CONSENSUS` | Konsens-Abstimmung |
| `DELEGATE` | Aufgaben-Delegation |

## 3. Orchestrierung

- **MasterBrain:** Aurora (Base44) als zentraler Orchestrator
- **12 Agenten-Rollen:** KnowledgeAgent, ArchitectAgent, StandardsAgent, etc.
- **EventBus:** Asynchrone Kommunikation (AD-002)
- **Konsens:** 2/3-Mehrheit für Agent-Entscheidungen

### 3.1 Context-Isolation

| Context | Agent-Fähigkeiten |
|---------|------------------|
| Node | Vollzugriff |
| Contract | Nur REQUEST/RESPONSE |
| Test | Mock-Agenten |

## 4. Gas-Cost-Tabelle

| Operation | Gas |
|-----------|-----|
| send_message | 50 |
| broadcast | 200 |
| consensus_vote | 100 |
| delegate_task | 150 |
| spawn_agent | 1000 |

## 5. Testing-Strategie

| Test | Beschreibung |
|------|-------------|
| Unit | Message-Serialisierung, Signatur |
| Integration | 2-Agent REQUEST→RESPONSE |
| E2E | 12-Agenten Konsens-Runde |
| Gas | Gas-Cost pro Nachrichtentyp |

## 6. Querverweise

| Referenz | Dokument |
|----------|---------|
| ATC-24 | AI Orchestration |
| ATC-25 | Neural Engine |
| ATC-41 | Multi-Agent Consensus |
| ATC-99 | ATCLang Universal Mandate |
| AD-005 | AIP (frühere Bezeichnung, jetzt ATC-97) Spezifikation (frühere Bezeichnung) |

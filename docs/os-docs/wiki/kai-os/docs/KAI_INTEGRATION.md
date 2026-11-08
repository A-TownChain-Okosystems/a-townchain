# 🧠⛓️ KAI-OS Integration Guide

## Overview

KAI-OS (Künstliche Intelligenz-Blockchain-Betriebssystem) ist vollständig in A-TownChain OS integriert. Diese Dokumentation beschreibt die Integration und Verwendung.

## Module

### 1. AI Kernel (`core/ai_kernel.py`)

**Komponenten:**
- `AIKernel` — Zentrale Inference Engine
- `ReasoningEngine` — Neurosymbolischer Reasoning (neural nets + symbolic AI)
- `MemoryModule` — Kurzzeit (RAM) + Langzeit (IPFS) Speicher

**Verwendung:**

```python
from core.ai_kernel import AIKernel, InferenceRequest, DecisionType

# Initialisiere Kernel
kernel = AIKernel(model_name="llama3-8b-q4", inference_mode="hybrid")
await kernel.start()

# Mache Inferenz
request = InferenceRequest(
    request_id="req_123",
    prompt="Analysiere diese Daten",
    model="llama3-8b-q4",
    decision_type=DecisionType.OPTIMIZATION
)

result = await kernel.infer(request)
print(f"Output: {result.output}")
print(f"Confidence: {result.confidence}")
print(f"Reasoning: {result.reasoning_steps}")

# Treffe autonome Entscheidung
decision = await kernel.make_autonomous_decision(
    decision_type=DecisionType.RESOURCE_ALLOCATION,
    context={"available_cpu": 8, "available_ram": 32},
    options=["allocate_all", "allocate_half", "defer"]
)
```

### 2. Smart Contracts (`blockchain/smart_contracts.py`)

**Contracts:**
- `ResourceMarket` — Auktion von Rechenressourcen
- `AgentRegistry` — Agent-Registrierung und Verifizierung
- `FederatedLearning` — Koordination von Training Rounds
- `GovernanceDAO` — System Governance & Voting
- `PaymentChannel` — Mikrozahlungen

**Verwendung:**

```python
from blockchain.smart_contracts import SmartContractRegistry

# Registriere Contracts
registry = SmartContractRegistry()

# ResourceMarket
market = registry.resource_market
auction = market.create_auction(
    seller="node_001",
    resource_type="gpu",
    quantity=4.0,
    min_price=100,
    duration_hours=1
)

bid = market.place_bid(
    auction_id=auction.auction_id,
    bidder="user_123",
    price=150,
    quantity=4.0
)

winner, price = market.finalize_auction(auction.auction_id)

# AgentRegistry
agent_registry = registry.agent_registry
agent = agent_registry.register_agent(
    agent_id="agent_001",
    did="did:kai:z6Mkh...",
    name="DataAnalyzer",
    owner="user_123",
    model="llama3-8b-q4",
    capabilities=["read_storage", "call_contracts"]
)

# GovernanceDAO
governance = registry.governance_dao
proposal = governance.create_proposal(
    title="Upgrade AI Kernel to v2.1",
    description="Performance improvements and new capabilities",
    proposer="council_address",
    target="ai_kernel"
)

governance.cast_vote(
    proposal_id=proposal.proposal_id,
    voter="voter_123",
    vote="yes",
    conviction=3
)

approved = governance.finalize_vote(proposal.proposal_id)
```

### 3. API Routes (`backend/api/kai_routes.py`)

**Endpoints:**

#### Agent Management
- `GET /v1/agents` — Liste alle Agenten auf
- `POST /v1/agents` — Erstelle neuen Agenten
- `POST /v1/agents/{id}/invoke` — Rufe Agent auf
- `GET /v1/agents/{id}/tasks/{task_id}` — Task-Status
- `DELETE /v1/agents/{id}` — Lösche Agent

#### Storage
- `POST /v1/storage/upload` — Datei zu IPFS hochladen
- `GET /v1/storage/{cid}` — Datei abrufen
- `GET /v1/storage/{cid}/info` — Metadaten abrufen

#### Blockchain
- `GET /v1/chain/status` — Chain Status
- `GET /v1/chain/balance/{address}` — Balance
- `POST /v1/chain/transfer` — Token Transfer

#### Governance
- `GET /v1/governance/proposals` — Proposals auflisten
- `POST /v1/governance/proposals/{id}/vote` — Abstimmen

## Integration mit A-TownChain

### Backend Integration

```python
# backend/main.py
from api.kai_routes import register_kai_api
from core.ai_kernel import AIKernel

app = Flask(__name__)

# Registriere KAI-OS API
register_kai_api(app)

# Initialisiere AI Kernel
ai_kernel = AIKernel()

@app.before_first_request
async def startup():
    await ai_kernel.start()

@app.teardown_appcontext
async def shutdown(exception=None):
    await ai_kernel.shutdown()
```

### Configuration

Bearbeite `config/kai_config.toml`:

```toml
[ai_kernel]
model = "llama3-8b-q4"
inference_mode = "hybrid"
max_memory_gb = 8
gpu_enabled = false

[blockchain]
network = "testnet"
consensus = "npos"

[governance]
proposal_discussion_days = 7
approval_threshold_percent = 60
```

## Token Standards

| Standard | Type | Use Case |
|----------|------|----------|
| KAI | Governance | Staking, Voting, Premium Features |
| COMPUTE | Utility | Pay for computation, storage, bandwidth |
| REPUTATION | Non-transferable | Node/Agent quality metrics |

## Workflow Example: Autonomous Decision Making

```python
# 1. AI Kernel macht Entscheidung
kernel = AIKernel()
await kernel.start()

decision = await kernel.make_autonomous_decision(
    decision_type=DecisionType.RESOURCE_ALLOCATION,
    context={"load": 0.8, "available_cpu": 16},
    options=["scale_up", "optimize", "queue"]
)

# 2. Entscheidung wird on-chain geloggt
print(f"Decision: {decision['chosen_option']}")
print(f"On-Chain: {decision['on_chain_hash']}")

# 3. Bei kritischen Entscheidungen: Governance
if decision['confidence'] < 0.7:
    governance = registry.governance_dao
    proposal = governance.create_proposal(
        title=f"Ratify Decision: {decision['decision_id']}",
        description=f"Confidence: {decision['confidence']}",
        proposer="system",
        target="decisions"
    )
```

## Security

- **Zero-Trust:** Alle Aktionen erfordern Authentifizierung
- **Ed25519:** Asymmetrische Kryptografie für Signaturen
- **AES-256-GCM:** Daten-Verschlüsselung
- **On-Chain Audit:** Alle kritischen Entscheidungen werden geloggt
- **Federated Learning:** Rohdaten werden nie geteilt, nur Gradienten

## Performance Metrics

```python
stats = kernel.get_stats()
print(f"Total Inferences: {stats['total_inferences']}")
print(f"Avg Inference Time: {stats['avg_inference_time_ms']:.2f}ms")
print(f"Error Rate: {stats['error_count'] / stats['total_inferences'] * 100:.2f}%")
```

## Roadmap

- [ ] Phase 1: Core AI Kernel (lokale Inferenz)
- [ ] Phase 2: Federated Learning Integration
- [ ] Phase 3: Dezentrale Inferenz (verteilte Modelle)
- [ ] Phase 4: Mainnet Deployment
- [ ] Phase 5: Governance-gesteuerte Model Updates

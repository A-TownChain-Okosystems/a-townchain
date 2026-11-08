# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
⛓️ KAI-OS Smart Contracts (Python Implementation)
System-Smart-Contracts für Ressourcenverwaltung, Governance, Federated Learning

Based on KAI-OS Wiki Chapter 4
"""

from dataclasses import dataclass, asdict
from enum import Enum
from typing import Dict, List, Optional, Tuple
from datetime import datetime, timedelta
import hashlib
import json
import logging

logger = logging.getLogger(__name__)


# ============================================================================
# TOKEN STANDARDS
# ============================================================================

class TokenStandard(Enum):
    """KAI-OS Token Standards"""
    KAI_GOVERNANCE = "kai"          # Governance & Staking Token
    COMPUTE = "compute"              # Utility Token für Rechenzeit
    REPUTATION = "reputation"        # Non-transferable Reputation Points


@dataclass
class Token:
    """Basis-Token Struktur"""
    owner: str                        # Wallet Address (DID)
    amount: int                       # In Planck (10^-12)
    standard: TokenStandard
    locked_until: Optional[int] = None  # Unix timestamp


@dataclass
class Transfer:
    """Token Transfer"""
    from_address: str
    to_address: str
    amount: int
    token_type: TokenStandard
    tx_hash: str
    timestamp: int


# ============================================================================
# LEGACY ATC TOKEN (Compatibility)
# ============================================================================

class ATCToken:
    """Legacy A-TownChain ATC Token (für Kompatibilität)"""
    
    def __init__(self):
        self.balances = {}
        self.total_supply = 1_000_000
    
    def transfer(self, sender, receiver, amount):
        if self.balances.get(sender, 0) >= amount:
            self.balances[sender] -= amount
            self.balances[receiver] = self.balances.get(receiver, 0) + amount
            return True
        return False


# ============================================================================
# RESOURCE MARKET CONTRACT
# ============================================================================

class ResourceMarket:
    """
    💰 Auktion von Rechenressourcen
    
    Nodes bieten freie Ressourcen an:
    - CPU/GPU Stunden
    - RAM
    - Storage
    - Bandbreite
    """
    
    def __init__(self):
        self.active_auctions: Dict[str, 'ResourceAuction'] = {}
        self.completed_auctions: List['ResourceAuction'] = []
        self.bids: Dict[str, List['Bid']] = {}
        
    @dataclass
    class ResourceAuction:
        auction_id: str
        seller: str                     # Node-ID
        resource_type: str              # "cpu", "gpu", "ram", "storage", "bandwidth"
        quantity: float                 # in Units (hours, GB, etc.)
        min_price: int                  # in $COMPUTE
        duration_hours: int
        created_at: int
        expires_at: int
        status: str = "active"          # "active", "closed", "completed"
        winner: Optional[str] = None
        final_price: Optional[int] = None
    
    @dataclass
    class Bid:
        bid_id: str
        auction_id: str
        bidder: str
        price: int                      # in $COMPUTE
        quantity: float
        timestamp: int
    
    def create_auction(self,
                      seller: str,
                      resource_type: str,
                      quantity: float,
                      min_price: int,
                      duration_hours: int = 1) -> 'ResourceAuction':
        """Erstelle neue Ressourcen-Auktion"""
        now = int(datetime.utcnow().timestamp())
        auction_id = hashlib.sha256(
            f"{seller}{resource_type}{now}".encode()
        ).hexdigest()[:16]
        
        auction = self.ResourceAuction(
            auction_id=auction_id,
            seller=seller,
            resource_type=resource_type,
            quantity=quantity,
            min_price=min_price,
            duration_hours=duration_hours,
            created_at=now,
            expires_at=now + (duration_hours * 3600)
        )
        
        self.active_auctions[auction_id] = auction
        self.bids[auction_id] = []
        
        logger.info(f"✅ Auction created: {auction_id} ({resource_type})")
        return auction
    
    def place_bid(self,
                  auction_id: str,
                  bidder: str,
                  price: int,
                  quantity: float) -> 'Bid':
        """Platziere Gebot auf Auktion"""
        if auction_id not in self.active_auctions:
            raise ValueError(f"Auction {auction_id} not found")
        
        auction = self.active_auctions[auction_id]
        if datetime.fromtimestamp(auction.expires_at) < datetime.utcnow():
            raise ValueError(f"Auction {auction_id} has expired")
        
        if price < auction.min_price:
            raise ValueError(f"Price {price} below minimum {auction.min_price}")
        
        bid_id = hashlib.sha256(
            f"{auction_id}{bidder}{price}{int(datetime.utcnow().timestamp())}".encode()
        ).hexdigest()[:16]
        
        bid = self.Bid(
            bid_id=bid_id,
            auction_id=auction_id,
            bidder=bidder,
            price=price,
            quantity=quantity,
            timestamp=int(datetime.utcnow().timestamp())
        )
        
        self.bids[auction_id].append(bid)
        logger.info(f"✅ Bid placed: {bid_id} on {auction_id}")
        
        return bid
    
    def finalize_auction(self, auction_id: str) -> Tuple[Optional[str], int]:
        """
        Beende Auktion und wähle Gewinner
        
        Returns: (winner_address, winning_price)
        """
        if auction_id not in self.active_auctions:
            raise ValueError(f"Auction {auction_id} not found")
        
        auction = self.active_auctions[auction_id]
        auction_bids = self.bids.get(auction_id, [])
        
        if not auction_bids:
            logger.warning(f"⚠️ No bids for auction {auction_id}")
            auction.status = "closed"
            return None, 0
        
        # Wähle Gebot mit höchstem Preis
        winning_bid = max(auction_bids, key=lambda b: b.price)
        
        auction.status = "completed"
        auction.winner = winning_bid.bidder
        auction.final_price = winning_bid.price
        
        self.completed_auctions.append(auction)
        del self.active_auctions[auction_id]
        
        logger.info(f"✅ Auction {auction_id} finalized - Winner: {winning_bid.bidder}")
        
        return winning_bid.bidder, winning_bid.price


# ============================================================================
# AGENT REGISTRY CONTRACT
# ============================================================================

class AgentRegistry:
    """
    🤖 Registrierung & Verifizierung von Agenten
    
    Speichert:
    - Agent Metadaten
    - Capabilities & Permissions
    - Reputation Score
    - Audit Trail
    """
    
    @dataclass
    class Agent:
        agent_id: str
        did: str                        # Decentralized Identifier
        name: str
        owner: str
        model: str
        capabilities: List[str]        # ["read_storage", "call_contracts", ...]
        status: str = "active"          # "active", "suspended", "revoked"
        reputation_score: int = 0
        created_at: int = None
        last_activity: int = None
        audit_trail: List[str] = None
        
        def __post_init__(self):
            if self.created_at is None:
                self.created_at = int(datetime.utcnow().timestamp())
            if self.last_activity is None:
                self.last_activity = self.created_at
            if self.audit_trail is None:
                self.audit_trail = []
    
    def __init__(self):
        self.agents: Dict[str, self.Agent] = {}
        self.agent_index: Dict[str, List[str]] = {}  # owner -> [agent_ids]
    
    def register_agent(self,
                      agent_id: str,
                      did: str,
                      name: str,
                      owner: str,
                      model: str,
                      capabilities: List[str]) -> Agent:
        """Registriere neuen Agenten"""
        if agent_id in self.agents:
            raise ValueError(f"Agent {agent_id} already registered")
        
        agent = self.Agent(
            agent_id=agent_id,
            did=did,
            name=name,
            owner=owner,
            model=model,
            capabilities=capabilities
        )
        
        self.agents[agent_id] = agent
        
        if owner not in self.agent_index:
            self.agent_index[owner] = []
        self.agent_index[owner].append(agent_id)
        
        agent.audit_trail.append(f"registered by {owner}")
        
        logger.info(f"✅ Agent registered: {agent_id} ({name})")
        return agent
    
    def get_agent(self, agent_id: str) -> Optional[Agent]:
        """Hole Agent-Informationen"""
        return self.agents.get(agent_id)
    
    def verify_capability(self, agent_id: str, capability: str) -> bool:
        """Verifiziere, ob Agent Capability hat"""
        agent = self.get_agent(agent_id)
        if not agent:
            return False
        return capability in agent.capabilities
    
    def update_reputation(self, agent_id: str, delta: int):
        """Update Reputation Score"""
        agent = self.get_agent(agent_id)
        if not agent:
            raise ValueError(f"Agent {agent_id} not found")
        
        agent.reputation_score += delta
        agent.last_activity = int(datetime.utcnow().timestamp())
        agent.audit_trail.append(f"reputation updated: {delta:+d}")
        
        logger.info(f"✅ Reputation updated for {agent_id}: {agent.reputation_score}")


# ============================================================================
# FEDERATED LEARNING CONTRACT
# ============================================================================

class FederatedLearning:
    """
    🧠 Koordination von Federated Learning Trainingsrunden
    
    Process:
    1. Round initialisiert mit Base Model
    2. Nodes trainieren lokal
    3. Gradienten werden geteilt (nie Rohdaten)
    4. Smart Contract aggregiert
    5. Verbesserte Modelle werden distribuiert
    """
    
    @dataclass
    class TrainingRound:
        round_id: str
        model_id: str                   # IPFS Hash
        version: int
        participants: List[str]         # Node Addresses
        status: str = "open"            # "open", "training", "aggregating", "completed"
        start_time: int = None
        end_time: Optional[int] = None
        aggregated_model_id: Optional[str] = None
        accuracy_improvement: float = 0.0
        
        def __post_init__(self):
            if self.start_time is None:
                self.start_time = int(datetime.utcnow().timestamp())
    
    @dataclass
    class LocalUpdate:
        update_id: str
        round_id: str
        node_address: str
        gradient_cid: str               # IPFS CID
        local_accuracy: float
        data_samples: int
        timestamp: int
    
    def __init__(self):
        self.rounds: Dict[str, self.TrainingRound] = {}
        self.updates: Dict[str, List[self.LocalUpdate]] = {}
        self.model_history: List[Dict] = []
    
    def start_round(self,
                   model_id: str,
                   version: int,
                   participants: List[str]) -> TrainingRound:
        """Starte neue Trainingsrunde"""
        round_id = hashlib.sha256(
            f"{model_id}{version}{int(datetime.utcnow().timestamp())}".encode()
        ).hexdigest()[:16]
        
        round_obj = self.TrainingRound(
            round_id=round_id,
            model_id=model_id,
            version=version,
            participants=participants
        )
        
        self.rounds[round_id] = round_obj
        self.updates[round_id] = []
        
        logger.info(f"✅ Training round started: {round_id} (v{version}, {len(participants)} participants)")
        return round_obj
    
    def submit_update(self,
                     round_id: str,
                     node_address: str,
                     gradient_cid: str,
                     local_accuracy: float,
                     data_samples: int) -> LocalUpdate:
        """Submit lokales Training Update"""
        if round_id not in self.rounds:
            raise ValueError(f"Round {round_id} not found")
        
        update_id = hashlib.sha256(
            f"{round_id}{node_address}".encode()
        ).hexdigest()[:16]
        
        update = self.LocalUpdate(
            update_id=update_id,
            round_id=round_id,
            node_address=node_address,
            gradient_cid=gradient_cid,
            local_accuracy=local_accuracy,
            data_samples=data_samples,
            timestamp=int(datetime.utcnow().timestamp())
        )
        
        self.updates[round_id].append(update)
        logger.info(f"✅ Update submitted to {round_id} from {node_address}")
        
        return update
    
    def aggregate_updates(self, round_id: str) -> str:
        """
        Aggregiere alle Updates für eine Runde
        
        Returns: IPFS CID des aggregierten Modells
        """
        if round_id not in self.rounds:
            raise ValueError(f"Round {round_id} not found")
        
        round_obj = self.rounds[round_id]
        round_updates = self.updates.get(round_id, [])
        
        if not round_updates:
            raise ValueError(f"No updates for round {round_id}")
        
        # Berechne gewichteten Durchschnitt
        total_samples = sum(u.data_samples for u in round_updates)
        avg_accuracy = sum(
            (u.local_accuracy * u.data_samples / total_samples) for u in round_updates
        )
        
        # Generiere neue Modell-ID
        aggregated_model_id = hashlib.sha256(
            json.dumps({
                "updates": [u.update_id for u in round_updates],
                "timestamp": int(datetime.utcnow().timestamp())
            }, sort_keys=True).encode()
        ).hexdigest()
        
        round_obj.status = "completed"
        round_obj.aggregated_model_id = aggregated_model_id
        round_obj.accuracy_improvement = avg_accuracy
        round_obj.end_time = int(datetime.utcnow().timestamp())
        
        # Speichere in History
        self.model_history.append({
            "round_id": round_id,
            "model_id": aggregated_model_id,
            "version": round_obj.version + 1,
            "accuracy": avg_accuracy,
            "timestamp": round_obj.end_time
        })
        
        logger.info(f"✅ Aggregation completed for {round_id} - New model: {aggregated_model_id}")
        
        return aggregated_model_id


# ============================================================================
# GOVERNANCE DAO CONTRACT
# ============================================================================

class GovernanceDAO:
    """
    🗳️ Abstimmungen über System-Updates
    
    Process:
    1. Proposal einreichen
    2. 7 Tage Diskussion
    3. 10% Quorum, 60% Mehrheit für Annahme
    4. 48h Timelock
    5. Gestaffelte Deployment (10% → 50% → 100%)
    """
    
    @dataclass
    class Proposal:
        proposal_id: int
        title: str
        description: str
        proposer: str
        target: str                     # Component to update
        status: str = "discussion"      # "discussion", "voting", "timelock", "executing", "executed", "rejected"
        yes_votes: int = 0
        no_votes: int = 0
        abstain_votes: int = 0
        created_at: int = None
        voting_start: Optional[int] = None
        voting_end: Optional[int] = None
        execution_start: Optional[int] = None
        execution_end: Optional[int] = None
        
        def __post_init__(self):
            if self.created_at is None:
                self.created_at = int(datetime.utcnow().timestamp())
    
    def __init__(self):
        self.proposals: Dict[int, self.Proposal] = {}
        self.votes: Dict[int, Dict[str, str]] = {}  # proposal_id -> {voter -> vote}
        self.next_proposal_id = 1
    
    def create_proposal(self,
                       title: str,
                       description: str,
                       proposer: str,
                       target: str) -> Proposal:
        """Erstelle neues Governance Proposal"""
        proposal_id = self.next_proposal_id
        self.next_proposal_id += 1
        
        proposal = self.Proposal(
            proposal_id=proposal_id,
            title=title,
            description=description,
            proposer=proposer,
            target=target
        )
        
        self.proposals[proposal_id] = proposal
        self.votes[proposal_id] = {}
        
        logger.info(f"✅ Proposal created: #{proposal_id} - {title}")
        return proposal
    
    def cast_vote(self,
                  proposal_id: int,
                  voter: str,
                  vote: str,
                  conviction: int = 1) -> bool:
        """
        Stimme über Proposal ab
        
        Args:
            vote: "yes" | "no" | "abstain"
            conviction: 0-6 (höher = mehr Gewicht, längere Sperrzeit)
        """
        if proposal_id not in self.proposals:
            raise ValueError(f"Proposal {proposal_id} not found")
        
        proposal = self.proposals[proposal_id]
        
        if proposal.status not in ["voting", "discussion"]:
            raise ValueError(f"Proposal {proposal_id} not open for voting")
        
        if voter in self.votes[proposal_id]:
            raise ValueError(f"{voter} already voted on {proposal_id}")
        
        # Berechne Vote-Gewicht basierend auf Conviction
        weight = conviction + 1
        
        self.votes[proposal_id][voter] = vote
        
        if vote == "yes":
            proposal.yes_votes += weight
        elif vote == "no":
            proposal.no_votes += weight
        else:
            proposal.abstain_votes += weight
        
        logger.info(f"✅ Vote cast on #{proposal_id}: {voter} → {vote} (conviction: {conviction})")
        
        return True
    
    def finalize_vote(self, proposal_id: int) -> bool:
        """Beende Abstimmung und führe aus wenn genehmigt"""
        if proposal_id not in self.proposals:
            raise ValueError(f"Proposal {proposal_id} not found")
        
        proposal = self.proposals[proposal_id]
        total_votes = proposal.yes_votes + proposal.no_votes
        
        if total_votes == 0:
            proposal.status = "rejected"
            logger.warning(f"⚠️ Proposal #{proposal_id} rejected - no votes")
            return False
        
        approval_rate = proposal.yes_votes / total_votes
        
        if approval_rate >= 0.6:
            proposal.status = "timelock"
            proposal.execution_start = int(datetime.utcnow().timestamp()) + (48 * 3600)  # 48h timelock
            logger.info(f"✅ Proposal #{proposal_id} approved - timelock started")
            return True
        else:
            proposal.status = "rejected"
            logger.info(f"❌ Proposal #{proposal_id} rejected - {approval_rate:.1%} approval")
            return False


# ============================================================================
# PAYMENT CHANNEL CONTRACT
# ============================================================================

class PaymentChannel:
    """
    💳 Mikrozahlungen für Rechenzeit
    
    Ermöglicht bidirektionale Zahlungskanäle:
    - Eröffnen mit Anfangsguthaben
    - Mehrfache Zahlungen off-chain
    - Finalisierung on-chain
    """
    
    @dataclass
    class Channel:
        channel_id: str
        payer: str                      # Zahler
        payee: str                      # Empfänger
        balance: int                    # In $COMPUTE
        initial_balance: int
        status: str = "open"            # "open", "challenged", "closed"
        created_at: int = None
        closed_at: Optional[int] = None
        nonce: int = 0
        
        def __post_init__(self):
            if self.created_at is None:
                self.created_at = int(datetime.utcnow().timestamp())
    
    def __init__(self):
        self.channels: Dict[str, self.Channel] = {}
        self.payment_history: List[Dict] = []
    
    def open_channel(self,
                    payer: str,
                    payee: str,
                    initial_balance: int) -> Channel:
        """Öffne neuen Payment Channel"""
        channel_id = hashlib.sha256(
            f"{payer}{payee}{int(datetime.utcnow().timestamp())}".encode()
        ).hexdigest()[:16]
        
        channel = self.Channel(
            channel_id=channel_id,
            payer=payer,
            payee=payee,
            balance=initial_balance,
            initial_balance=initial_balance
        )
        
        self.channels[channel_id] = channel
        logger.info(f"✅ Payment channel opened: {channel_id}")
        
        return channel
    
    def send_payment(self,
                    channel_id: str,
                    amount: int) -> Dict:
        """Sende Zahlungs-Update über Channel"""
        if channel_id not in self.channels:
            raise ValueError(f"Channel {channel_id} not found")
        
        channel = self.channels[channel_id]
        
        if channel.status != "open":
            raise ValueError(f"Channel {channel_id} is not open")
        
        if amount > channel.balance:
            raise ValueError(f"Insufficient balance: {channel.balance} < {amount}")
        
        channel.balance -= amount
        channel.nonce += 1
        
        payment_record = {
            "channel_id": channel_id,
            "amount": amount,
            "new_balance": channel.balance,
            "nonce": channel.nonce,
            "timestamp": int(datetime.utcnow().timestamp())
        }
        
        self.payment_history.append(payment_record)
        logger.info(f"✅ Payment sent on {channel_id}: {amount} $COMPUTE")
        
        return payment_record
    
    def close_channel(self, channel_id: str):
        """Schließe Payment Channel"""
        if channel_id not in self.channels:
            raise ValueError(f"Channel {channel_id} not found")
        
        channel = self.channels[channel_id]
        channel.status = "closed"
        channel.closed_at = int(datetime.utcnow().timestamp())
        
        logger.info(f"✅ Payment channel closed: {channel_id}")


# ============================================================================
# CONTRACT REGISTRY
# ============================================================================

class SmartContractRegistry:
    """Zentrale Registry aller System-Contracts"""
    
    def __init__(self):
        self.resource_market = ResourceMarket()
        self.agent_registry = AgentRegistry()
        self.federated_learning = FederatedLearning()
        self.governance_dao = GovernanceDAO()
        self.payment_channel = PaymentChannel()
    
    def get_contract(self, contract_name: str):
        """Hole Smart Contract Instanz"""
        contracts = {
            "ResourceMarket": self.resource_market,
            "AgentRegistry": self.agent_registry,
            "FederatedLearning": self.federated_learning,
            "GovernanceDAO": self.governance_dao,
            "PaymentChannel": self.payment_channel,
        }
        
        if contract_name not in contracts:
            raise ValueError(f"Contract {contract_name} not found")
        
        return contracts[contract_name]
    
    def list_contracts(self) -> List[str]:
        """Liste alle verfügbaren Contracts"""
        return [
            "ResourceMarket",
            "AgentRegistry",
            "FederatedLearning",
            "GovernanceDAO",
            "PaymentChannel",
        ]

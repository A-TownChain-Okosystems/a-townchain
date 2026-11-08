# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# MIGRATION v3.2.1: Imports auf kanonischen Pfad aktualisiert (2026-06-12)
# Alt: from core.ai_kernel import ...
# Neu: from modules.kernel.ai_kernel.ai_kernel import ...

"""
Testing suite for KAI-OS AI Kernel, Smart Contracts, and API
"""

import pytest
import asyncio
from modules.kernel.ai_kernel.ai_kernel import AIKernel, AIRequest, DecisionType, ReasoningEngine
from blockchain.smart_contracts import (
    SmartContractRegistry, ResourceMarket, AgentRegistry,
    FederatedLearning, GovernanceDAO, PaymentChannel
)


class TestAIKernel:
    """Test AI Kernel functionality"""
    
    @pytest.fixture
    def kernel(self):
        return AIKernel(model_name="llama3-8b-q4")
    
    @pytest.mark.asyncio
    async def test_kernel_startup(self, kernel):
        """Test kernel startup"""
        await kernel.start()
        assert kernel.is_running
        await kernel.shutdown()
    
    @pytest.mark.asyncio
    async def test_inference_request(self, kernel):
        """Test inference request"""
        await kernel.start()
        
        request = AIRequest(
            request_id="test_req_001",
            prompt="What is 2+2?",
            model="llama3-8b-q4"
        )
        
        result = await kernel.infer(request)
        
        assert result.request_id == "test_req_001"
        assert len(result.reasoning_steps) > 0
        assert 0 <= result.confidence <= 1
        
        await kernel.shutdown()
    
    @pytest.mark.asyncio
    async def test_autonomous_decision(self, kernel):
        """Test autonomous decision making"""
        await kernel.start()
        
        decision = await kernel.make_autonomous_decision(
            decision_type=DecisionType.OPTIMIZATION,
            context={"load": 0.8},
            options=["optimize", "scale_up", "defer"]
        )
        
        assert "chosen_option" in decision
        assert "confidence" in decision
        assert "reasoning" in decision
        
        await kernel.shutdown()
    
    def test_reasoning_engine(self):
        """Test reasoning engine"""
        engine = ReasoningEngine()
        
        conclusion, steps, confidence = asyncio.run(
            engine.reason(
                query="What should we do?",
                context={"option1": "good", "option2": "better"}
            )
        )
        
        assert isinstance(conclusion, str)
        assert len(steps) > 0
        assert 0 <= confidence <= 1


class TestSmartContracts:
    """Test Smart Contracts"""
    
    @pytest.fixture
    def registry(self):
        return SmartContractRegistry()
    
    def test_resource_market(self, registry):
        """Test ResourceMarket contract"""
        market = registry.resource_market
        
        # Create auction
        auction = market.create_auction(
            seller="node_001",
            resource_type="gpu",
            quantity=4.0,
            min_price=100,
            duration_hours=1
        )
        
        assert auction.auction_id
        assert auction.status == "active"
        
        # Place bid
        bid = market.place_bid(
            auction_id=auction.auction_id,
            bidder="user_123",
            price=150,
            quantity=4.0
        )
        
        assert bid.bid_id
        assert bid.price == 150
        
        # Finalize
        winner, price = market.finalize_auction(auction.auction_id)
        
        assert winner == "user_123"
        assert price == 150
    
    def test_agent_registry(self, registry):
        """Test AgentRegistry contract"""
        agent_reg = registry.agent_registry
        
        # Register agent
        agent = agent_reg.register_agent(
            agent_id="agent_001",
            did="did:kai:z6Mkh123",
            name="DataAnalyzer",
            owner="user_123",
            model="llama3-8b-q4",
            capabilities=["read_storage", "call_contracts"]
        )
        
        assert agent.agent_id == "agent_001"
        assert agent.status == "active"
        
        # Verify capability
        has_capability = agent_reg.verify_capability(
            agent_id="agent_001",
            capability="read_storage"
        )
        assert has_capability
        
        # Update reputation
        agent_reg.update_reputation("agent_001", +10)
        updated_agent = agent_reg.get_agent("agent_001")
        assert updated_agent.reputation_score == 10
    
    def test_federated_learning(self, registry):
        """Test FederatedLearning contract"""
        fl = registry.federated_learning
        
        # Start round
        round_obj = fl.start_round(
            model_id="model_v1",
            version=1,
            participants=["node_001", "node_002", "node_003"]
        )
        
        assert round_obj.round_id
        assert round_obj.status == "open"
        
        # Submit updates
        update1 = fl.submit_update(
            round_id=round_obj.round_id,
            node_address="node_001",
            gradient_cid="QmGradient1",
            local_accuracy=0.92,
            data_samples=1000
        )
        
        update2 = fl.submit_update(
            round_id=round_obj.round_id,
            node_address="node_002",
            gradient_cid="QmGradient2",
            local_accuracy=0.89,
            data_samples=900
        )
        
        # Aggregate
        aggregated_cid = fl.aggregate_updates(round_obj.round_id)
        
        assert aggregated_cid
        assert round_obj.status == "completed"
        assert round_obj.accuracy_improvement > 0
    
    def test_governance_dao(self, registry):
        """Test GovernanceDAO contract"""
        gov = registry.governance_dao
        
        # Create proposal
        proposal = gov.create_proposal(
            title="Upgrade AI Kernel",
            description="Performance improvements",
            proposer="council",
            target="ai_kernel"
        )
        
        assert proposal.proposal_id
        assert proposal.status == "discussion"
        
        # Cast votes
        gov.cast_vote(
            proposal_id=proposal.proposal_id,
            voter="voter_1",
            vote="yes",
            conviction=3
        )
        
        gov.cast_vote(
            proposal_id=proposal.proposal_id,
            voter="voter_2",
            vote="yes",
            conviction=2
        )
        
        gov.cast_vote(
            proposal_id=proposal.proposal_id,
            voter="voter_3",
            vote="no",
            conviction=1
        )
        
        # Finalize
        approved = gov.finalize_vote(proposal.proposal_id)
        assert approved  # 6 yes votes vs 1 no vote = 85% approval
    
    def test_payment_channel(self, registry):
        """Test PaymentChannel contract"""
        pc = registry.payment_channel
        
        # Open channel
        channel = pc.open_channel(
            payer="payer_123",
            payee="payee_456",
            initial_balance=1000
        )
        
        assert channel.channel_id
        assert channel.status == "open"
        assert channel.balance == 1000
        
        # Send payments
        payment1 = pc.send_payment(
            channel_id=channel.channel_id,
            amount=100
        )
        
        assert payment1["amount"] == 100
        assert payment1["new_balance"] == 900
        
        # Close channel
        pc.close_channel(channel.channel_id)
        assert channel.status == "closed"


class TestIntegration:
    """Integration tests"""
    
    @pytest.mark.asyncio
    async def test_end_to_end_workflow(self):
        """Test complete workflow: AI decision -> Smart Contract execution"""
        # Initialize kernel and contracts
        kernel = AIKernel()
        await kernel.start()
        
        registry = SmartContractRegistry()
        
        # Make autonomous decision
        decision = await kernel.make_autonomous_decision(
            decision_type=DecisionType.RESOURCE_ALLOCATION,
            context={"available_cpu": 8},
            options=["allocate_all", "allocate_half", "defer"]
        )
        
        # Register decision in governance if low confidence
        if decision["confidence"] < 0.7:
            gov = registry.governance_dao
            proposal = gov.create_proposal(
                title=f"Ratify Decision {decision['decision_id']}",
                description=f"Confidence: {decision['confidence']}",
                proposer="system",
                target="decisions"
            )
            
            assert proposal.proposal_id
        
        await kernel.shutdown()


if __name__ == "__main__":
    pytest.main(["-v", __file__])

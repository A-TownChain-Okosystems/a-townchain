# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
# blockchain/contracts/governance/atc9900_governance.py -- ATC-9900 DAO (Issue #9)
from dataclasses import dataclass, field
from typing import Dict, Optional
from enum import Enum
import time

class ProposalStatus(Enum):
    ACTIVE   = "active"
    PASSED   = "passed"
    REJECTED = "rejected"
    EXPIRED  = "expired"

@dataclass
class Proposal:
    id: int; title: str; description: str; proposer: str
    created_at: float = field(default_factory=time.time)
    expires_at: float = 0.0
    votes_for: Dict[str,float] = field(default_factory=dict)
    votes_against: Dict[str,float] = field(default_factory=dict)
    status: ProposalStatus = ProposalStatus.ACTIVE

    @property
    def total_for(self): return sum(self.votes_for.values())
    @property
    def total_against(self): return sum(self.votes_against.values())
    @property
    def total_votes(self): return self.total_for + self.total_against

class ATC9900Governance:
    QUORUM_PERCENT   = 0.10
    MAJORITY_PERCENT = 0.50
    VOTING_PERIOD    = 7 * 24 * 3600

    def __init__(self, total_supply=21_000_000):
        self.total_supply = total_supply
        self.proposals: Dict[int, Proposal] = {}
        self._next_id = 1

    def create_proposal(self, title, description, proposer) -> Proposal:
        p = Proposal(id=self._next_id, title=title, description=description,
                     proposer=proposer, expires_at=time.time()+self.VOTING_PERIOD)
        self.proposals[self._next_id] = p; self._next_id += 1; return p

    def vote(self, proposal_id, voter, atc_balance, support) -> bool:
        p = self.proposals.get(proposal_id)
        if not p or p.status != ProposalStatus.ACTIVE: return False
        if time.time() > p.expires_at: p.status = ProposalStatus.EXPIRED; return False
        if voter in p.votes_for or voter in p.votes_against: return False
        (p.votes_for if support else p.votes_against)[voter] = atc_balance; return True

    def finalize(self, proposal_id) -> ProposalStatus:
        p = self.proposals[proposal_id]
        if p.total_votes < self.total_supply * self.QUORUM_PERCENT:
            p.status = ProposalStatus.REJECTED
        elif p.total_for / max(p.total_votes,1) > self.MAJORITY_PERCENT:
            p.status = ProposalStatus.PASSED
        else:
            p.status = ProposalStatus.REJECTED
        return p.status

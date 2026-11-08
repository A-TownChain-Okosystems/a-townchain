# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
blockchain/contracts/governance/governance_contract.py
ATC-9900 Governance DAO — Dezentrales Abstimmungssystem
Issue #9: Governance Contract

Features:
  - create_proposal: Proposal mit Optionen + Deadline
  - vote: Gewichtetes Voting (ATC-Balance)
  - execute_proposal: nach Ablauf + Quorum
  - Timelock: 48h nach Vote vor Ausführung
  - Quorum: min. 10% der ATC-Supply
"""
import time, hashlib
from dataclasses import dataclass, asdict, field
from enum import Enum
from typing import Optional
from blockchain.contracts.base.base_contract import BaseContract


class ProposalStatus(Enum):
    PENDING  = "PENDING"    # erstellt, Voting noch nicht gestartet
    ACTIVE   = "ACTIVE"     # Abstimmung läuft
    PASSED   = "PASSED"     # Quorum erreicht, Mehrheit dafür
    REJECTED = "REJECTED"   # Quorum verfehlt oder Mehrheit dagegen
    EXECUTED = "EXECUTED"   # Ausgeführt (nach Timelock)
    EXPIRED  = "EXPIRED"    # Abgelaufen ohne Ausführung


@dataclass
class Vote:
    voter:     str
    option:    int          # Index in proposal.options[]
    weight:    float        # ATC-Balance zum Zeitpunkt des Votes
    timestamp: int


@dataclass
class Proposal:
    proposal_id:  str
    creator:      str
    title:        str
    description:  str
    options:      list       # ["Ja", "Nein", "Enthalten"] etc.
    created_at:   int
    voting_end:   int        # Unix-Timestamp
    timelock_end: int        # voting_end + 48h
    status:       str        = "ACTIVE"
    votes:        dict       = field(default_factory=dict)   # voter → Vote
    vote_counts:  dict       = field(default_factory=dict)   # option_idx → total_weight
    executed_at:  int        = 0
    quorum_met:   bool       = False
    winning_option: int      = -1

    def to_dict(self):
        return {
            "proposal_id":    self.proposal_id,
            "creator":        self.creator,
            "title":          self.title,
            "description":    self.description,
            "options":        self.options,
            "created_at":     self.created_at,
            "voting_end":     self.voting_end,
            "timelock_end":   self.timelock_end,
            "status":         self.status,
            "vote_counts":    self.vote_counts,
            "voters":         len(self.votes),
            "executed_at":    self.executed_at,
            "quorum_met":     self.quorum_met,
            "winning_option": self.winning_option,
            "standard":       "ATC-9900"
        }


class GovernanceContract(BaseContract):
    """
    ATC-9900 Governance DAO.
    Stimmt über Protokoll-Parameter, Treasury, Upgrades ab.
    """

    QUORUM_PERCENT   = 10.0    # 10% der Supply muss abstimmen
    TIMELOCK_SECONDS = 48 * 3600  # 48h Timelock nach Voting-Ende
    MIN_OPTIONS      = 2
    MAX_OPTIONS      = 10
    DEFAULT_VOTING_PERIOD = 7 * 24 * 3600  # 7 Tage

    def __init__(self, owner: str, total_supply: float = 21_000_000.0):
        super().__init__(owner, contract_id="ATC_GOVERNANCE_9900")
        self.proposals:    dict[str, Proposal] = {}
        self.proposal_ids: list[str]           = []
        self.total_supply  = total_supply
        self._balances:    dict[str, float]    = {}

    # ── Balance-Integration ────────────────────────────
    def set_balance_oracle(self, balances: dict):
        """Setzt ATC-Balances (vom Token-Contract)."""
        self._balances = balances

    def _get_voting_power(self, voter: str) -> float:
        return self._balances.get(voter, 0.0)

    # ── Proposal erstellen ─────────────────────────────
    def create_proposal(
        self, creator: str, title: str,
        description: str, options: list,
        voting_period_secs: int = None
    ) -> dict:
        """Erstellt einen neuen Governance-Proposal."""
        self.when_not_paused()
        if not title.strip():
            raise ValueError("Title darf nicht leer sein")
        if len(options) < self.MIN_OPTIONS:
            raise ValueError(f"Mindestens {self.MIN_OPTIONS} Optionen erforderlich")
        if len(options) > self.MAX_OPTIONS:
            raise ValueError(f"Maximal {self.MAX_OPTIONS} Optionen erlaubt")

        now         = int(time.time())
        period      = voting_period_secs or self.DEFAULT_VOTING_PERIOD
        voting_end  = now + period
        timelock    = voting_end + self.TIMELOCK_SECONDS

        pid = "PROP-" + hashlib.sha256(
            f"{creator}{title}{now}".encode()
        ).hexdigest()[:12].upper()

        proposal = Proposal(
            proposal_id  = pid,
            creator      = creator,
            title        = title,
            description  = description,
            options      = options,
            created_at   = now,
            voting_end   = voting_end,
            timelock_end = timelock,
            status       = ProposalStatus.ACTIVE.value,
            vote_counts  = {str(i): 0.0 for i in range(len(options))}
        )

        self.proposals[pid]  = proposal
        self.proposal_ids.append(pid)
        self._emit("ProposalCreated", {
            "proposal_id": pid, "creator": creator,
            "title": title, "options": options,
            "voting_end": voting_end
        })
        return {"success": True, "proposal": proposal.to_dict()}

    # ── Abstimmen ──────────────────────────────────────
    def vote(self, voter: str, proposal_id: str, option: int) -> dict:
        """Stimmt auf einen Proposal ab. Gewicht = ATC-Balance."""
        self.when_not_paused()
        proposal = self._get_active_proposal(proposal_id)

        if voter in proposal.votes:
            raise ValueError(f"{voter} hat bereits abgestimmt")
        if option < 0 or option >= len(proposal.options):
            raise ValueError(f"Ungültige Option {option}")

        weight = self._get_voting_power(voter)
        if weight <= 0:
            raise PermissionError("Keine Stimmgewalt (0 ATC-Balance)")

        proposal.votes[voter] = asdict(Vote(voter, option, weight, int(time.time())))
        proposal.vote_counts[str(option)] = (
            proposal.vote_counts.get(str(option), 0.0) + weight
        )

        self._emit("VoteCast", {
            "voter": voter, "proposal_id": proposal_id,
            "option": option, "weight": weight
        })
        return {
            "success": True, "voter": voter,
            "option": proposal.options[option],
            "weight": weight
        }

    # ── Proposal abschließen ───────────────────────────
    def finalize_proposal(self, proposal_id: str) -> dict:
        """
        Schließt Voting ab und berechnet Ergebnis.
        Kann nach voting_end aufgerufen werden.
        """
        proposal = self.proposals.get(proposal_id)
        if not proposal:
            raise KeyError(f"Proposal {proposal_id} nicht gefunden")
        if proposal.status != ProposalStatus.ACTIVE.value:
            raise ValueError(f"Proposal ist nicht aktiv: {proposal.status}")

        now = int(time.time())
        if now < proposal.voting_end:
            raise ValueError(f"Abstimmung läuft noch bis {proposal.voting_end}")

        # Quorum prüfen
        total_voted = sum(proposal.vote_counts.values())
        quorum_required = self.total_supply * (self.QUORUM_PERCENT / 100)
        proposal.quorum_met = total_voted >= quorum_required

        if not proposal.quorum_met:
            proposal.status = ProposalStatus.REJECTED.value
            self._emit("ProposalRejected", {"proposal_id": proposal_id,
                       "reason": "Quorum nicht erreicht",
                       "total_voted": total_voted, "required": quorum_required})
            return {"success": False, "reason": "Quorum nicht erreicht",
                    "proposal": proposal.to_dict()}

        # Gewinner ermitteln
        max_votes = max(proposal.vote_counts.values())
        for idx, votes in proposal.vote_counts.items():
            if votes == max_votes:
                proposal.winning_option = int(idx)
                break

        proposal.status = ProposalStatus.PASSED.value
        self._emit("ProposalPassed", {
            "proposal_id": proposal_id,
            "winning_option": proposal.winning_option,
            "winning_text": proposal.options[proposal.winning_option],
            "total_voted": total_voted
        })
        return {"success": True, "proposal": proposal.to_dict()}

    # ── Ausführen (nach Timelock) ──────────────────────
    def execute_proposal(self, caller: str, proposal_id: str) -> dict:
        """
        Führt Proposal aus — nur nach Timelock.
        On-Chain: Ergebnis wird unveränderlich gespeichert.
        """
        proposal = self.proposals.get(proposal_id)
        if not proposal:
            raise KeyError(f"Proposal {proposal_id} nicht gefunden")
        if proposal.status != ProposalStatus.PASSED.value:
            raise ValueError(f"Proposal muss PASSED sein (ist: {proposal.status})")

        now = int(time.time())
        if now < proposal.timelock_end:
            remaining = proposal.timelock_end - now
            raise ValueError(f"Timelock aktiv: noch {remaining//3600}h {(remaining%3600)//60}min")

        proposal.status      = ProposalStatus.EXECUTED.value
        proposal.executed_at = now
        self._emit("ProposalExecuted", {
            "proposal_id":      proposal_id,
            "executed_by":      caller,
            "winning_option":   proposal.winning_option,
            "winning_text":     proposal.options[proposal.winning_option],
            "executed_at":      now
        })
        return {"success": True, "proposal": proposal.to_dict()}

    # ── Queries ────────────────────────────────────────
    def get_proposal(self, proposal_id: str) -> dict:
        p = self.proposals.get(proposal_id)
        return p.to_dict() if p else {"error": "Nicht gefunden"}

    def get_all_proposals(self, status_filter: str = None) -> list:
        proposals = list(self.proposals.values())
        if status_filter:
            proposals = [p for p in proposals if p.status == status_filter]
        return [p.to_dict() for p in proposals]

    def get_voter_status(self, voter: str, proposal_id: str) -> dict:
        p = self.proposals.get(proposal_id)
        if not p:
            return {"error": "Nicht gefunden"}
        voted = voter in p.votes
        return {
            "voter":  voter,
            "voted":  voted,
            "vote":   p.votes.get(voter) if voted else None,
            "power":  self._get_voting_power(voter)
        }

    def get_stats(self) -> dict:
        active   = sum(1 for p in self.proposals.values() if p.status == "ACTIVE")
        passed   = sum(1 for p in self.proposals.values() if p.status == "PASSED")
        executed = sum(1 for p in self.proposals.values() if p.status == "EXECUTED")
        return {
            "total_proposals": len(self.proposals),
            "active":   active,
            "passed":   passed,
            "executed": executed,
            "quorum_percent": self.QUORUM_PERCENT,
            "timelock_hours": self.TIMELOCK_SECONDS // 3600,
            "standard": "ATC-9900"
        }

    # ── Helpers ────────────────────────────────────────
    def _get_active_proposal(self, proposal_id: str) -> Proposal:
        p = self.proposals.get(proposal_id)
        if not p:
            raise KeyError(f"Proposal {proposal_id} nicht gefunden")
        now = int(time.time())
        if p.status != ProposalStatus.ACTIVE.value:
            raise ValueError(f"Proposal nicht aktiv: {p.status}")
        if now > p.voting_end:
            p.status = ProposalStatus.EXPIRED.value
            raise ValueError("Abstimmungs-Periode abgelaufen")
        return p

"""Lifecycle Manager (AD-43, kanonische Referenz-Implementierung).

Portiert lifecycle_manager_ad43.atc: 12-Phasen-Statemachine (Idea -> Archived).
Hinweis (ehrlich): Das GFF-v2-Wiki nennt '11 Phasen'; die kanonische Spec
lifecycle_manager_ad43.atc deklariert 12 Enum-Eintraege — die Spec gewinnt.
"""

from __future__ import annotations

import hashlib
import time
from dataclasses import dataclass, field
from enum import Enum


class LPhase(Enum):
    """LPhase aus lifecycle_manager_ad43.atc (12 Phasen, Reihenfolge verbindlich)."""

    IDEA = "Idea"
    CONCEPT = "Concept"
    PROTOTYPE = "Prototype"
    PRE_PROD = "PreProd"
    PRODUCTION = "Production"
    ALPHA = "Alpha"
    BETA = "Beta"
    RELEASE = "Release"
    LIVE_OPS = "LiveOps"
    EXPANSION = "Expansion"
    SUCCESSOR = "Successor"
    ARCHIVED = "Archived"


_PHASE_ORDER = [p for p in LPhase]


class MStatus(Enum):
    NOT_STARTED = "NotStarted"
    IN_PROGRESS = "InProgress"
    ACHIEVED = "Achieved"


@dataclass
class KPI:
    dau: int = 0
    mau: int = 0
    revenue: float = 0.0
    retention: float = 0.0
    crash_free: float = 100.0
    rating: float = 0.0
    sentiment: float = 0.5


@dataclass
class Milestone:
    id: str
    fid: str
    name: str
    phase: LPhase
    target: int
    achieved: int = 0
    status: MStatus = MStatus.NOT_STARTED
    criteria: list[str] = field(default_factory=list)
    done: list[str] = field(default_factory=list)


@dataclass
class LFranchise:
    id: str
    name: str
    phase: LPhase
    history: list[dict] = field(default_factory=list)
    start: float = 0.0
    target: float = 0.0
    budget: float = 0.0
    spent: float = 0.0
    team: list[str] = field(default_factory=list)
    risk: int = 5
    prob: float = 0.5
    milestones: list[str] = field(default_factory=list)
    kpi: KPI = field(default_factory=KPI)


@dataclass
class PhaseTemplate:
    phase: LPhase
    name: str
    desc: str
    duration_days: int
    deliverables: list[str]
    criteria: list[str]


def _init_templates() -> dict[LPhase, PhaseTemplate]:
    """init_templates aus AD-43 (dur/deliv/crit portiert)."""
    t = {}
    spec = [
        (LPhase.IDEA, "Idea", "Concept", 14, ["Vision"], ["Approved"]),
        (LPhase.CONCEPT, "Concept", "Design", 30, ["GDD"], ["GDD OK"]),
        (LPhase.PROTOTYPE, "Prototype", "Playable", 60, ["Slice"], ["Playable"]),
        (LPhase.PRE_PROD, "PreProd", "Planning", 90, ["Plan"], ["Plan OK"]),
        (LPhase.PRODUCTION, "Production", "Content", 365, ["Levels"], ["Complete"]),
        (LPhase.ALPHA, "Alpha", "Features", 60, ["Features"], ["Alpha"]),
        (LPhase.BETA, "Beta", "Polish", 60, ["Bugs"], ["Beta"]),
        (LPhase.RELEASE, "Release", "Launch", 30, ["Gold"], ["Launched"]),
        (LPhase.LIVE_OPS, "LiveOps", "Ops", 0, ["Seasons"], ["Active"]),
        (LPhase.EXPANSION, "Expansion", "DLC", 180, ["DLC"], ["DLC OK"]),
        (LPhase.SUCCESSOR, "Successor", "Next", 365, ["Plan"], ["New"]),
        (LPhase.ARCHIVED, "Archived", "End", 0, [], ["Closed"]),
    ]
    for phase, name, desc, dur, deliv, crit in spec:
        t[phase] = PhaseTemplate(phase, name, desc, dur, deliv, crit)
    return t


class LifecycleManager:
    """LifecycleMgr aus AD-43: Registry, Phasen-Uebergange, Milestones, Health."""

    def __init__(self) -> None:
        self.franchises: dict[str, LFranchise] = {}
        self.templates: dict[LPhase, PhaseTemplate] = _init_templates()
        self.transitions: list[dict] = []
        self.milestones: dict[str, Milestone] = {}
        self._now: float = time.time()

    def register(
        self, name: str, budget: float = 0.0, target: float = 0.0, *, now: float | None = None
    ) -> str:
        if not name:
            raise ValueError("register: name leer")
        ts = self._now if now is None else now
        fid = hashlib.sha256(f"{name}|{ts}".encode()).hexdigest()[:16]
        self.franchises[fid] = LFranchise(
            id=fid,
            name=name,
            phase=LPhase.IDEA,
            history=[
                {
                    "phase": LPhase.IDEA,
                    "entered": ts,
                    "exited": 0.0,
                    "notes": "Created",
                    "success": False,
                }
            ],
            start=ts,
            target=target,
            budget=budget,
        )
        return fid

    @staticmethod
    def _validate_transition(old: LPhase, new: LPhase) -> bool:
        """Adjacent-Schritt oder direkte Archivierung; Rueckspruenge verboten (AD-43-Semantik)."""
        i_old, i_new = _PHASE_ORDER.index(old), _PHASE_ORDER.index(new)
        return i_new == i_old + 1 or new == LPhase.ARCHIVED

    def transition(
        self, fid: str, new_phase: LPhase, by: str, notes: str = "", *, now: float | None = None
    ) -> bool:
        fr = self.franchises.get(fid)
        if fr is None:
            raise KeyError(f"Franchise {fid} nicht gefunden")
        if not self._validate_transition(fr.phase, new_phase):
            raise ValueError(f"Ungueltiger Uebergang {fr.phase.value} -> {new_phase.value}")
        ts = self._now if now is None else now
        old = fr.phase
        if fr.history:
            fr.history[-1]["exited"] = ts
            fr.history[-1]["success"] = True
        fr.phase = new_phase
        fr.history.append(
            {"phase": new_phase, "entered": ts, "exited": 0.0, "notes": notes, "success": False}
        )
        self.transitions.append(
            {
                "id": hashlib.sha256(f"{fid}|{ts}".encode()).hexdigest()[:16],
                "fid": fid,
                "from": old,
                "to": new_phase,
                "timestamp": ts,
                "by": by,
                "notes": notes,
            }
        )
        return True

    def add_milestone(
        self, fid: str, name: str, phase: LPhase, target: int, criteria: list[str]
    ) -> str:
        fr = self.franchises.get(fid)
        if fr is None:
            raise KeyError(f"Franchise {fid} nicht gefunden")
        mid = hashlib.sha256(f"{name}|{self._now}".encode()).hexdigest()[:16]
        self.milestones[mid] = Milestone(
            id=mid, fid=fid, name=name, phase=phase, target=target, criteria=list(criteria)
        )
        fr.milestones.append(mid)
        return mid

    def achieve_milestone(self, mid: str, achieved: int, done: list[str]) -> bool:
        m = self.milestones.get(mid)
        if m is None:
            raise KeyError(f"Milestone {mid} nicht gefunden")
        m.achieved = achieved
        m.done = list(done)
        m.status = MStatus.ACHIEVED if achieved >= m.target else MStatus.IN_PROGRESS
        return m.status == MStatus.ACHIEVED

    def health(self, fid: str) -> dict:
        fr = self.franchises.get(fid)
        if fr is None:
            raise KeyError(f"Franchise {fid} nicht gefunden")
        budget_used = (fr.spent / fr.budget) if fr.budget else 0.0
        return {
            "phase": fr.phase.value,
            "risk": fr.risk,
            "prob": fr.prob,
            "budget_used_ratio": round(budget_used, 4),
            "kpi_dau": fr.kpi.dau,
            "kpi_crash_free": fr.kpi.crash_free,
        }

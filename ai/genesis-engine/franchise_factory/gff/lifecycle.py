"""Lifecycle Manager (AD-43)."""
from __future__ import annotations

# ruff: isort: skip_file
import hashlib
from dataclasses import dataclass,field
from enum import Enum
class LPhase(Enum):
    IDEA="Idea"; CONCEPT="Concept"; PROTOTYPE="Prototype"; PRE_PROD="PreProd"; PRODUCTION="Production"; ALPHA="Alpha"; BETA="Beta"; RELEASE="Release"; LIVE_OPS="LiveOps"; EXPANSION="Expansion"; SUCCESSOR="Successor"; ARCHIVED="Archived"
_PHASE_ORDER=[p for p in LPhase]
class MStatus(Enum): NOT_STARTED="NotStarted"; IN_PROGRESS="InProgress"; ACHIEVED="Achieved"
@dataclass
class KPI:
    dau:int=0; mau:int=0; revenue:float=0.0; retention:float=0.0; crash_free:float=100.0; rating:float=0.0; sentiment:float=0.5
@dataclass
class Milestone:
    id:str; fid:str; name:str; phase:LPhase; target:int; achieved:int=0; status:MStatus=MStatus.NOT_STARTED; criteria:list[str]=field(default_factory=list); done:list[str]=field(default_factory=list)
@dataclass
class LFranchise:
    id:str; name:str; phase:LPhase; history:list[dict]=field(default_factory=list); start:float=0.0; target:float=0.0; budget:float=0.0; spent:float=0.0; team:list[str]=field(default_factory=list); risk:int=5; prob:float=0.5; milestones:list[str]=field(default_factory=list); kpi:KPI=field(default_factory=KPI)
@dataclass
class PhaseTemplate:
    phase:LPhase; name:str; desc:str; duration_days:int; deliverables:list[str]; criteria:list[str]
def _init_templates():
    spec=[(LPhase.IDEA,"Idea","Concept",14,["Vision"],["Approved"]),(LPhase.CONCEPT,"Concept","Design",30,["GDD"],["GDD OK"]),(LPhase.PROTOTYPE,"Prototype","Playable",60,["Slice"],["Playable"]),(LPhase.PRE_PROD,"PreProd","Planning",90,["Plan"],["Plan OK"]),(LPhase.PRODUCTION,"Production","Content",365,["Levels"],["Complete"]),(LPhase.ALPHA,"Alpha","Features",60,["Features"],["Alpha"]),(LPhase.BETA,"Beta","Polish",60,["Bugs"],["Beta"]),(LPhase.RELEASE,"Release","Launch",30,["Gold"],["Launched"]),(LPhase.LIVE_OPS,"LiveOps","Ops",0,["Seasons"],["Active"]),(LPhase.EXPANSION,"Expansion","DLC",180,["DLC"],["DLC OK"]),(LPhase.SUCCESSOR,"Successor","Next",365,["Plan"],["New"]),(LPhase.ARCHIVED,"Archived","End",0,[],["Closed"])]
    return {p:PhaseTemplate(p,n,d,dur,de,cr) for p,n,d,dur,de,cr in spec}
class LifecycleManager:
    def __init__(self): self.franchises={}; self.templates=_init_templates(); self.transitions=[]; self.milestones={}; self._now=0.0
    def register(self,name,budget=0.0,target=0.0,*,now=None):
        if not name: raise ValueError("register: name leer")
        ts=self._now if now is None else now; fid=hashlib.sha256(f"{name}|{ts}".encode()).hexdigest()[:16]
        self.franchises[fid]=LFranchise(fid,name,LPhase.IDEA,[{"phase":LPhase.IDEA,"entered":ts,"exited":0.0,"notes":"Created","success":False}],ts,target,budget); return fid
    @staticmethod
    def _validate_transition(old,new):
        a,b=_PHASE_ORDER.index(old),_PHASE_ORDER.index(new); return b==a+1 or new==LPhase.ARCHIVED
    def transition(self,fid,new_phase,by,notes="",*,now=None):
        fr=self.franchises.get(fid)
        if fr is None: raise KeyError(f"Franchise {fid} nicht gefunden")
        if not self._validate_transition(fr.phase,new_phase): raise ValueError(f"Ungueltiger Uebergang {fr.phase.value} -> {new_phase.value}")
        ts=self._now if now is None else now; old=fr.phase; fr.history[-1].update(exited=ts,success=True); fr.phase=new_phase; fr.history.append({"phase":new_phase,"entered":ts,"exited":0.0,"notes":notes,"success":False}); self.transitions.append({"id":hashlib.sha256(f"{fid}|{ts}".encode()).hexdigest()[:16],"fid":fid,"from":old,"to":new_phase,"timestamp":ts,"by":by,"notes":notes}); return True
    def add_milestone(self,fid,name,phase,target,criteria):
        fr=self.franchises.get(fid)
        if fr is None: raise KeyError(f"Franchise {fid} nicht gefunden")
        mid=hashlib.sha256(f"{name}|{self._now}".encode()).hexdigest()[:16]; self.milestones[mid]=Milestone(mid,fid,name,phase,target,criteria=list(criteria)); fr.milestones.append(mid); return mid
    def achieve_milestone(self,mid,achieved,done):
        m=self.milestones.get(mid)
        if m is None: raise KeyError(f"Milestone {mid} nicht gefunden")
        m.achieved=achieved; m.done=list(done); m.status=MStatus.ACHIEVED if achieved>=m.target else MStatus.IN_PROGRESS; return m.status==MStatus.ACHIEVED
    def health(self,fid):
        fr=self.franchises.get(fid)
        if fr is None: raise KeyError(f"Franchise {fid} nicht gefunden")
        return {"phase":fr.phase.value,"risk":fr.risk,"prob":fr.prob,"budget_used_ratio":round(fr.spent/fr.budget,4) if fr.budget else 0.0,"kpi_dau":fr.kpi.dau,"kpi_crash_free":fr.kpi.crash_free}

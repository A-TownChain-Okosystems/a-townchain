"""ATC-9900 DAO-Modell (factory.atc / factory.py-Prototyp, portiert & bereinigt)."""
from __future__ import annotations

# ruff: isort: skip_file
import hashlib
from dataclasses import dataclass, field
from enum import Enum
class FranchiseStatus(Enum):
    PROPOSAL="proposal"; ACTIVE="active"; SUSPENDED="suspended"; DISSOLVED="dissolved"
class RoyaltyTier(Enum):
    BRONZE=0.05; SILVER=0.04; GOLD=0.03; PLATINUM=0.02
@dataclass
class FranchiseVault:
    balance: float=0.0; total_in: float=0.0; total_out: float=0.0; transactions:list[dict]=field(default_factory=list)
    _clock: float=field(default=0.0, repr=False, compare=False)
    def deposit(self, amount:float, from_addr:str, note:str="")->None:
        if amount<0: raise ValueError("deposit: negativer Betrag")
        self.balance+=amount; self.total_in+=amount
        self.transactions.append({"type":"deposit","amount":amount,"from":from_addr,"note":note,"ts":self._clock})
    def withdraw(self, amount:float, to_addr:str, note:str="")->bool:
        if amount>self.balance: return False
        self.balance-=amount; self.total_out+=amount
        self.transactions.append({"type":"withdraw","amount":amount,"to":to_addr,"note":note,"ts":self._clock}); return True
@dataclass
class Franchise:
    id:str; name:str; owner:str; description:str; token_symbol:str; token_supply:float
    royalty_tier:RoyaltyTier=RoyaltyTier.BRONZE; status:FranchiseStatus=FranchiseStatus.PROPOSAL
    created:float=0.0; members:dict[str,float]=field(default_factory=dict)
    vault:FranchiseVault=field(default_factory=FranchiseVault); proposals:list[dict]=field(default_factory=list)
    def add_member(self,addr:str,stake:float)->None:
        if stake<0: raise ValueError("add_member: negativer Stake")
        self.members[addr]=self.members.get(addr,0)+stake
    def total_stake(self)->float: return sum(self.members.values())
    def voting_power(self,addr:str)->float:
        total=self.total_stake(); return 0.0 if total==0 else self.members.get(addr,0)/total
    def distribute_revenue(self,amount:float)->dict:
        if amount<0: raise ValueError("distribute_revenue: negativer Betrag")
        total=self.total_stake(); self.vault.deposit(amount,"revenue","Einnahmen")
        royalty=amount*self.royalty_tier.value; net=amount-royalty
        if total==0: return {"royalty":royalty,"net":net,"distribution":{}}
        return {"royalty":royalty,"net":net,"distribution":{a:(s/total)*net for a,s in self.members.items()}}
class FranchiseFactory:
    def __init__(self)->None: self._franchises={}; self._owner_index={}
    def create(self,name,owner,description,token_symbol,token_supply=1_000_000,royalty_tier=RoyaltyTier.BRONZE):
        fid=hashlib.sha256(f"{name}|{owner}|{len(self._franchises)}".encode()).hexdigest()[:16]
        f=Franchise(fid,name,owner,description,token_symbol,token_supply,royalty_tier,FranchiseStatus.ACTIVE)
        f.add_member(owner,token_supply*0.2); self._franchises[fid]=f; self._owner_index.setdefault(owner,[]).append(fid); return f
    def get(self,fid): return self._franchises.get(fid)
    def list_all(self,status=None): return [f for f in self._franchises.values() if status is None or f.status==status]
    def by_owner(self,owner): return [self._franchises[i] for i in self._owner_index.get(owner,[]) if i in self._franchises]
    def join(self,fid,member,stake):
        f=self.get(fid)
        if not f or f.status!=FranchiseStatus.ACTIVE or stake<0: return False
        f.add_member(member,stake); f.vault.deposit(stake,member,"Beitritt"); return True
    def suspend(self,fid):
        f=self.get(fid)
        if not f: return False
        f.status=FranchiseStatus.SUSPENDED; return True
    def stats(self):
        return {"total":len(self._franchises),"active":sum(1 for f in self._franchises.values() if f.status==FranchiseStatus.ACTIVE),"total_vault":sum(f.vault.balance for f in self._franchises.values())}

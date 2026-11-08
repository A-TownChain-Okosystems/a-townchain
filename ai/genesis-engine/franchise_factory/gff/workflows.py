"""Declarative AI production workflows for the Genesis Franchise Factory."""
from __future__ import annotations

# ruff: isort: skip_file

from collections.abc import Callable, Mapping
from dataclasses import dataclass, field
from enum import StrEnum
from typing import Any

# fmt: off
class WorkflowStage(StrEnum):
    INPUT="input"; ANALYZE="analyze"; PLAN="plan"; PRODUCE="produce"; QUALITY="quality"; INTEGRATE="integrate"; PUBLISH="publish"; MONITOR="monitor"; OPTIMIZE="optimize"; REPLICATE="replicate"
class FactoryId(StrEnum):
    TEXT="text-content"; SOFTWARE="software"; GAME="game"; MARKETING="marketing"; BUSINESS="business"; DOCUMENT="document"; RESEARCH="research"; ECOMMERCE="e-commerce"; CUSTOMER_SERVICE="customer-service"; AUTOMATION="automation"; KNOWLEDGE="knowledge"; AGENT="agent"; FRANCHISE="franchise"; STARTUP="startup"; EDUCATION="education"; BOOK="book"; VIRTUAL_WORLD="virtual-world"
@dataclass(frozen=True, slots=True)
class WorkflowDefinition:
    id:str; name:str; stages:tuple[WorkflowStage,...]; capabilities:tuple[str,...]; outputs:tuple[str,...]; description:str=""
@dataclass(slots=True)
class WorkflowContext:
    workflow_id:str; payload:dict[str,Any]; artifacts:dict[str,Any]=field(default_factory=dict); evidence:list[dict[str,Any]]=field(default_factory=list); stage:WorkflowStage|None=None
    def record_evidence(self,kind:str,value:Any)->None: self.evidence.append({"kind":kind,"value":value})
Executor=Callable[[WorkflowStage,WorkflowContext],WorkflowContext]
class WorkflowError(RuntimeError): pass
class WorkflowRegistry:
    def __init__(self,definitions:tuple[WorkflowDefinition,...]|None=None)->None: self._definitions={i.id:i for i in definitions or DEFAULT_WORKFLOWS}; self.validate()
    def get(self,workflow_id:str)->WorkflowDefinition:
        try:return self._definitions[workflow_id]
        except KeyError as exc: raise WorkflowError(f"Unknown workflow: {workflow_id}") from exc
    def all(self)->tuple[WorkflowDefinition,...]: return tuple(self._definitions.values())
    def validate(self)->None:
        if not self._definitions: raise WorkflowError("Workflow registry must not be empty")
        for d in self._definitions.values():
            if not d.stages or d.stages[0] is not WorkflowStage.INPUT: raise WorkflowError(f"Workflow {d.id} must start with INPUT")
            if WorkflowStage.QUALITY not in d.stages: raise WorkflowError(f"Workflow {d.id} requires a QUALITY gate")
            if len(set(d.stages))!=len(d.stages): raise WorkflowError(f"Workflow {d.id} contains duplicate stages")
class WorkflowEngine:
    def __init__(self,registry:WorkflowRegistry|None=None)->None:self.registry=registry or WorkflowRegistry()
    def run(self,workflow_id:str,payload:Mapping[str,Any],executor:Executor)->WorkflowContext:
        d=self.registry.get(workflow_id); context=WorkflowContext(d.id,dict(payload))
        for stage in d.stages:
            context.stage=stage; context=executor(stage,context)
            if not isinstance(context,WorkflowContext): raise WorkflowError(f"Executor returned invalid context at stage {stage}")
        context.record_evidence("workflow.completed",d.id); return context
def _definition(factory:FactoryId,name:str,capabilities:tuple[str,...],outputs:tuple[str,...],description:str)->WorkflowDefinition:
    return WorkflowDefinition(factory.value,name,(WorkflowStage.INPUT,WorkflowStage.ANALYZE,WorkflowStage.PLAN,WorkflowStage.PRODUCE,WorkflowStage.QUALITY,WorkflowStage.INTEGRATE,WorkflowStage.PUBLISH,WorkflowStage.MONITOR,WorkflowStage.OPTIMIZE),capabilities,outputs,description)
DEFAULT_WORKFLOWS=(
_definition(FactoryId.TEXT,"Text / Content Factory",("writing","translation","seo"),("content","localized-content"),"Text and content production."),
_definition(FactoryId.SOFTWARE,"Software Factory",("specification","architecture","code","tests","deployment"),("source","tests","documentation","release"),"Software delivery from idea to deployment."),
_definition(FactoryId.GAME,"Game Factory",("game-design","world","lore","characters","quests","gameplay","assets","ai-npc","testing","liveops"),("game-bible","game-content","build","liveops-plan"),"End-to-end game production."),
_definition(FactoryId.MARKETING,"Marketing Factory",("campaigns","ads","landing-pages","experiments","analytics"),("campaign","creative-plan","report"),"Campaign planning and optimization."),
_definition(FactoryId.BUSINESS,"Business Factory",("business-model","market-analysis","financial-model","pricing","kpis"),("business-plan","financial-model","kpi-plan"),"Business model production."),
_definition(FactoryId.DOCUMENT,"Document Factory",("contracts","offers","reports","sops","manuals"),("documents","document-set"),"Controlled business document production."),
_definition(FactoryId.RESEARCH,"Research Factory",("research","source-analysis","monitoring","synthesis"),("research-report","knowledge-update"),"Evidence-oriented research workflows."),
_definition(FactoryId.ECOMMERCE,"E-Commerce Factory",("catalog","product-analysis","pricing","sales-analysis"),("catalog","product-content","sales-report"),"E-commerce content and operations."),
_definition(FactoryId.CUSTOMER_SERVICE,"Customer-Service Factory",("support","faq","triage","crm","escalation"),("resolution","faq-update","support-report"),"Customer support automation."),
_definition(FactoryId.AUTOMATION,"Automation Factory",("triggers","agents","apis","data","notifications"),("automation","execution-log"),"Event-driven multi-step automation."),
_definition(FactoryId.KNOWLEDGE,"Knowledge Factory",("ingestion","rag","semantic-search","knowledge-graph","memory"),("knowledge-base","index","graph"),"Document-to-knowledge transformation."),
_definition(FactoryId.AGENT,"Agent Factory",("identity","tools","memory","permissions","workflows","agent-messaging"),("agent-definition","policy","workflow"),"Controlled AI-agent creation."),
_definition(FactoryId.FRANCHISE,"Franchise Factory",("business-model","brand","product","content","software","marketing","sales","automation","replication"),("franchise-package","operating-model","replication-plan"),"Replicate a validated business system as a franchise package."),
_definition(FactoryId.STARTUP,"Startup Factory",("validation","mvp","branding","product","launch","kpis"),("startup-package","mvp-plan","launch-plan"),"Startup formation from idea to launch."),
_definition(FactoryId.EDUCATION,"Education Factory",("curriculum","learning-material","exercises","assessment","tutoring"),("course","learning-platform-plan","assessment"),"Education product production."),
_definition(FactoryId.BOOK,"Book Factory",("research","outline","writing","editing","layout","translation","publishing"),("manuscript","book-package","publication-plan"),"Book production and publication."),
_definition(FactoryId.VIRTUAL_WORLD,"Virtual World Factory",("world","geography","cities","buildings","npcs","economy","factions","lore","simulation"),("world-bible","world-data","simulation"),"Persistent virtual-world production."),)
GAME_SUBSYSTEMS:Mapping[str,tuple[str,...]]={"concept":("genre","target","platforms","usp","core-loop","modes","monetization","technical-requirements"),"world":("continents","regions","biomes","cities","dungeons","buildings","climate","day-night","weather","portals"),"lore":("origin","peoples","factions","wars","timeline","secrets","canon","artifacts"),"character":("player","npc","classes","attributes","skills","progression","relationships"),"creature":("anatomy","abilities","weaknesses","attacks","movement","loot","variants","boss-mechanics"),"combat":("melee","ranged","magic","combos","dodge","parry","status","boss-phases","pvp-balance"),"quest":("main","side","faction","events","puzzles","boss","hidden","dynamic"),"level":("terrain","rooms","paths","encounters","loot","checkpoints","puzzles","secrets","scaling"),"item":("weapons","armor","accessories","consumables","resources","relics","artifacts","skins","crafting"),"weapon":("concept","design","3d","animation","vfx","sound","gameplay"),"animation":("idle","walk","run","jump","attack","combo","hit","death","emotes","interactions"),"ai-npc":("identity","memory","personality","goals","knowledge","behavior","routine","relationships"),"audio":("sfx","voice","ambient","music","dynamic-music","spatial-audio"),"vfx":("fire","water","explosions","magic","energy","portals","weather","abilities","boss-effects"),"economy":("loot","resources","crafting","pricing","demand","inflation","progression","rewards"),"multiplayer":("matchmaking","lobby","party","guilds","pvp","pve","raids","leaderboards","seasons","server"),"testing":("bugs","exploits","levels","combat","quests","economy","performance","network","ui","progression","ai-playtests"),"liveops":("telemetry","retention","funnel","balance","bugs","seasons","events","content")}

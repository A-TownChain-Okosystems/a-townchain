"""GFF Core — Pipeline-Orchestrator (AD-20, kanonische Referenz-Implementierung).

Portiert gff_core_ad20.atc: Franchise-Registry, 10-stufige Default-Pipeline,
Events. Der Executor ist injizierbar (Production-Integration spaeter via GCL/ATC-VM);
ohne Executor laeuft die Pipeline im ehrlichen DRY-RUN (kein Fake-Output).
"""

from __future__ import annotations

import hashlib
from collections.abc import Callable
from dataclasses import dataclass, field
from enum import Enum

from gff.spec_loader import FactorySpec, load_specs


class FranchiseStatus(Enum):
    CONCEPT = "Concept"
    IN_PRODUCTION = "InProduction"
    TESTING = "Testing"
    LIVE = "Live"
    EXPANDING = "Expanding"
    ARCHIVED = "Archived"


class PipelineStatus(Enum):
    PENDING = "Pending"
    IN_PROGRESS = "InProgress"
    COMPLETE = "Complete"
    SKIPPED = "Skipped"


@dataclass
class FranchiseBlueprint:
    name: str
    genre: str
    target_audience: str = ""
    world_count: int = 1
    character_count: int = 0
    quest_count: int = 0
    economy_model: str = ""
    monetization: list[str] = field(default_factory=list)
    platforms: list[str] = field(default_factory=list)


@dataclass
class Franchise:
    id: str
    name: str
    universe: str
    status: FranchiseStatus
    created_at: float
    blueprint: FranchiseBlueprint
    factories_used: list[str] = field(default_factory=list)
    progress: float = 0.0


@dataclass
class PipelineStage:
    name: str
    factory: str
    order: int
    enabled: bool = True
    status: PipelineStatus = PipelineStatus.PENDING


@dataclass
class StageResult:
    success: bool
    note: str = "dry-run"


Executor = Callable[[PipelineStage, Franchise], StageResult]


def _noop_executor(stage: PipelineStage, franchise: Franchise) -> StageResult:
    return StageResult(success=True, note="dry-run: kein Executor injiziert")


def default_pipeline() -> list[PipelineStage]:
    return [
        PipelineStage("Blueprint", "ip_factory", 0),
        PipelineStage("World", "world_factory", 1),
        PipelineStage("Character", "character_factory", 2),
        PipelineStage("Lore", "lore_factory", 3),
        PipelineStage("Quest", "quest_factory", 4),
        PipelineStage("Asset", "ai_content_factory", 5),
        PipelineStage("Game", "world_factory", 6),
        PipelineStage("Testing", "analytics_factory", 7),
        PipelineStage("LiveOps", "liveops_factory", 8),
        PipelineStage("Merchandise", "merchandise_factory", 9),
    ]


class GFFCore:
    def __init__(self, spec_dir=None, version: str = "1.0.0"):
        self.version = version
        self.initialized = True
        self.franchises: dict[str, Franchise] = {}
        self.active_franchise: str | None = None
        self.pipeline: list[PipelineStage] = default_pipeline()
        self.pipeline_running = False
        self.events: list[dict] = []
        self.factory_specs: dict[int, FactorySpec] = load_specs(spec_dir) if spec_dir is not None else {}
        self._event_clock = 0.0
        self._emit("GFFInitialized", version=version)

    def _emit(self, event: str, **data) -> None:
        self.events.append({"event": event, "ts": self._event_clock, **data})
        self._event_clock += 1.0

    def create_franchise(self, blueprint: FranchiseBlueprint, *, now: float | None = None) -> str:
        ts = self._event_clock if now is None else now
        fid = hashlib.sha256(f"{blueprint.name}|{ts}".encode()).hexdigest()[:16]
        if not blueprint.name:
            raise ValueError("blueprint.name darf nicht leer sein")
        self.franchises[fid] = Franchise(
            id=fid, name=blueprint.name, universe=f"{blueprint.name} Universe",
            status=FranchiseStatus.CONCEPT, created_at=ts, blueprint=blueprint,
            factories_used=[], progress=0.0,
        )
        self.active_franchise = fid
        self._emit("FranchiseCreated", fid=fid, name=blueprint.name)
        return fid

    def run_pipeline(self, franchise_id: str, *, executor: Executor = _noop_executor) -> bool:
        if franchise_id not in self.franchises:
            raise KeyError(f"Franchise {franchise_id} nicht gefunden")
        if self.pipeline_running:
            raise RuntimeError("Pipeline laeuft bereits (pipeline_running)")
        self.pipeline_running = True
        self._emit("PipelineStarted", fid=franchise_id)
        try:
            franchise = self.franchises[franchise_id]
            for stage in sorted(self.pipeline, key=lambda s: s.order):
                if not stage.enabled:
                    stage.status = PipelineStatus.SKIPPED
                    continue
                stage.status = PipelineStatus.IN_PROGRESS
                result = executor(stage, franchise)
                if not result.success:
                    self._emit("StageFailed", fid=franchise_id, stage=stage.name, note=result.note)
                    raise RuntimeError(f"Stage {stage.name} fehlgeschlagen: {result.note}")
                stage.status = PipelineStatus.COMPLETE
                if stage.factory not in franchise.factories_used:
                    franchise.factories_used.append(stage.factory)
                self._emit("StageComplete", fid=franchise_id, stage=stage.name)
            done = sum(1 for s in self.pipeline if s.status == PipelineStatus.COMPLETE)
            total = sum(1 for s in self.pipeline if s.enabled)
            franchise.progress = round(done / total, 4) if total else 0.0
            self._emit("PipelineComplete", fid=franchise_id, progress=franchise.progress)
            return True
        finally:
            self.pipeline_running = False

    def reset_pipeline(self) -> None:
        for s in self.pipeline:
            s.status = PipelineStatus.PENDING

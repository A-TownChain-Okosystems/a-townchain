# ruff: noqa
"""Genesis Franchise Factory (GFF) — Core-Paket.

Content-Pipeline-Orchestrator des Genesis-Oekosystems.
Kanonische Spec-Quelle: specs/*.atc (AD-20..AD-43, unveraendert aus dem
Org-Archiv a-townchain-os-docs/docs/archive/monorepo-full uebernommen).

Trennungsregel (verbindlich): GFF sitzt UEBER der Genesis Engine und ist
strikte Plattform — keinerlei Abhaengigkeit zu Genesis Chronicles.
"""

__version__ = "0.1.0"

from gff.artifacts import ArtifactContractError, ArtifactEnvelope, ArtifactKind, ArtifactRef
from gff.core import Franchise, FranchiseBlueprint, FranchiseStatus, GFFCore, PipelineStage
from gff.dao import FranchiseFactory as DAOFranchiseFactory
from gff.game_factory import GAME_FACTORY_GRAPH, GameFactoryGraphError, GameFactoryNode, topological_order
from gff.lifecycle import LifecycleManager, LPhase
from gff.workflows import (
    DEFAULT_WORKFLOWS,
    GAME_SUBSYSTEMS,
    FactoryId,
    WorkflowContext,
    WorkflowDefinition,
    WorkflowEngine,
    WorkflowError,
    WorkflowRegistry,
    WorkflowStage,
)

__all__ = [name for name in globals() if not name.startswith('_')]

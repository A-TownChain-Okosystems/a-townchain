"""Genesis Franchise Factory (GFF) — Core-Paket.

Content-Pipeline-Orchestrator des Genesis-Oekosystems.
Kanonische Spec-Quelle: specs/*.atc (AD-20..AD-43, unveraendert aus dem
Org-Archiv a-townchain-os-docs/docs/archive/monorepo-full uebernommen).

Trennungsregel (verbindlich): GFF sitzt UEBER der Genesis Engine und ist
strikte Plattform — keinerlei Abhaengigkeit zu Genesis Chronicles.
"""

__version__ = "0.1.0"

from gff.core import (  # noqa: E402,F401
    Franchise,
    FranchiseBlueprint,
    FranchiseStatus,
    GFFCore,
    PipelineStage,
)
from gff.dao import FranchiseFactory as DAOFranchiseFactory  # noqa: E402,F401
from gff.lifecycle import LifecycleManager, LPhase  # noqa: E402,F401

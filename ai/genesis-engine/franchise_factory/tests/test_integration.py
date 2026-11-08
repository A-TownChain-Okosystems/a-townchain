from gff.artifacts import ArtifactKind, ArtifactRef
from gff.core import FranchiseBlueprint, GFFCore
from gff.game_factory import GAME_FACTORY_GRAPH, dependency_refs, topological_order
from gff.lifecycle import LifecycleManager, LPhase
from gff.workflows import FactoryId, WorkflowEngine, WorkflowStage


def test_workflow_catalog_and_execution():
    seen = []
    def executor(stage, context):
        seen.append(stage)
        return context
    result = WorkflowEngine().run(FactoryId.GAME.value, {"name": "Example"}, executor)
    assert seen[0] is WorkflowStage.INPUT
    assert WorkflowStage.QUALITY in seen
    assert result.evidence[-1]["kind"] == "workflow.completed"


def test_game_factory_graph_is_acyclic_and_complete():
    order = topological_order()
    assert len(order) == len(GAME_FACTORY_GRAPH)
    assert order.index("item") < order.index("economy") < order.index("multiplayer") < order.index("build") < order.index("testing") < order.index("liveops")


def test_dependency_resolution_fails_closed():
    node = next(n for n in GAME_FACTORY_GRAPH if n.id == "economy")
    refs = (ArtifactRef("game", ArtifactKind.GAME_BIBLE, producer="concept"), ArtifactRef("items", ArtifactKind.ITEM, producer="item"))
    assert [r.kind for r in dependency_refs(node, refs)] == [ArtifactKind.GAME_BIBLE, ArtifactKind.ITEM]


def test_core_franchise_pipeline():
    core = GFFCore()
    fid = core.create_franchise(FranchiseBlueprint(name="Example", genre="RPG"), now=1.0)
    assert core.run_pipeline(fid)
    assert core.franchises[fid].progress == 1.0


def test_lifecycle_is_forward_only():
    manager = LifecycleManager()
    fid = manager.register("Example", now=1.0)
    assert manager.transition(fid, LPhase.CONCEPT, "test", now=2.0)
    try:
        manager.transition(fid, LPhase.IDEA, "test", now=3.0)
    except ValueError:
        pass
    else:
        raise AssertionError("Lifecycle rollback must fail closed")

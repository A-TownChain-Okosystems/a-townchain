import pytest

from gff.core import FranchiseBlueprint, FranchiseStatus, GFFCore, PipelineStatus, StageResult


@pytest.fixture()
def core():
    return GFFCore()


def test_init_emits_event(core):
    assert core.initialized
    assert core.version == "1.0.0"
    assert core.events[0]["event"] == "GFFInitialized"
    assert len(core.pipeline) == 10
    assert [s.order for s in core.pipeline] == list(range(10))


def test_create_franchise(core):
    fid = core.create_franchise(
        FranchiseBlueprint(name="Test IP", genre="RPG", world_count=2),
        now=1000.0,
    )
    f = core.franchises[fid]
    assert f.name == "Test IP"
    assert f.universe == "Test IP Universe"
    assert f.status == FranchiseStatus.CONCEPT
    assert f.progress == 0.0
    assert core.active_franchise == fid
    assert core.events[-1]["event"] == "FranchiseCreated"


def test_create_franchise_empty_name_rejected(core):
    with pytest.raises(ValueError):
        core.create_franchise(FranchiseBlueprint(name="", genre="RPG"))


def test_run_pipeline_dry_run(core):
    fid = core.create_franchise(FranchiseBlueprint(name="Dry", genre="RPG"), now=1.0)
    assert core.run_pipeline(fid) is True
    f = core.franchises[fid]
    assert f.progress == 1.0
    assert len(f.factories_used) > 0
    assert all(s.status == PipelineStatus.COMPLETE for s in core.pipeline)
    events = [e["event"] for e in core.events]
    assert "PipelineStarted" in events and "PipelineComplete" in events
    assert events.count("StageComplete") == 10
    assert core.pipeline_running is False


def test_run_pipeline_unknown_franchise(core):
    with pytest.raises(KeyError):
        core.run_pipeline("does-not-exist")


def test_pipeline_executor_failure_aborts(core):
    fid = core.create_franchise(FranchiseBlueprint(name="Fail", genre="RPG"), now=1.0)

    def failing(stage, franchise):
        return StageResult(success=False, note="asset-gen offline")

    with pytest.raises(RuntimeError, match="Blueprint"):
        core.run_pipeline(fid, executor=failing)
    assert core.pipeline_running is False  # finally-Block


def test_disabled_stage_skipped(core):
    core.pipeline[0].enabled = False
    fid = core.create_franchise(FranchiseBlueprint(name="Skip", genre="RPG"), now=1.0)
    core.run_pipeline(fid)
    assert core.pipeline[0].status == PipelineStatus.SKIPPED
    f = core.franchises[fid]
    assert f.progress == 1.0  # 9/9 enabled Stufen complete
    assert "ip_factory" not in f.factories_used


def test_pipeline_stage_order_respected(core):
    fid = core.create_franchise(FranchiseBlueprint(name="Order", genre="RPG"), now=1.0)
    seen = []

    def recorder(stage, franchise):
        seen.append(stage.name)
        return StageResult(success=True)

    core.run_pipeline(fid, executor=recorder)
    assert seen == [s.name for s in sorted(core.pipeline, key=lambda s: s.order)]


def test_reset_pipeline(core):
    fid = core.create_franchise(FranchiseBlueprint(name="R", genre="RPG"), now=1.0)
    core.run_pipeline(fid)
    core.reset_pipeline()
    assert all(s.status == PipelineStatus.PENDING for s in core.pipeline)

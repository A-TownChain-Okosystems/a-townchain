import pytest

from gff.lifecycle import LifecycleManager, LPhase, MStatus


def test_register_starts_in_idea():
    lm = LifecycleManager()
    fid = lm.register("New IP", budget=100, target=1000, now=1.0)
    fr = lm.franchises[fid]
    assert fr.phase == LPhase.IDEA
    assert fr.budget == 100 and fr.risk == 5 and fr.prob == 0.5
    assert fr.history[0]["phase"] == LPhase.IDEA


def test_full_chain_to_archive():
    lm = LifecycleManager()
    fid = lm.register("Chain", now=1.0)
    chain = [
        LPhase.CONCEPT,
        LPhase.PROTOTYPE,
        LPhase.PRE_PROD,
        LPhase.PRODUCTION,
        LPhase.ALPHA,
        LPhase.BETA,
        LPhase.RELEASE,
        LPhase.LIVE_OPS,
        LPhase.EXPANSION,
        LPhase.SUCCESSOR,
        LPhase.ARCHIVED,
    ]
    for p in chain:
        assert lm.transition(fid, p, by="owner") is True
    assert lm.franchises[fid].phase == LPhase.ARCHIVED
    assert len(lm.transitions) == len(chain)


def test_direct_archive_allowed():
    lm = LifecycleManager()
    fid = lm.register("Quick", now=1.0)
    assert lm.transition(fid, LPhase.ARCHIVED, by="owner") is True


def test_invalid_jump_rejected():
    lm = LifecycleManager()
    fid = lm.register("Bad", now=1.0)
    with pytest.raises(ValueError, match="Ungueltiger Uebergang"):
        lm.transition(fid, LPhase.RELEASE, by="owner")
    with pytest.raises(ValueError, match="Ungueltiger Uebergang"):
        lm.transition(fid, LPhase.BETA, by="owner")  # Beta nur nach vollstaendiger Kette erreichbar


def test_backward_transition_rejected():
    lm = LifecycleManager()
    fid = lm.register("Back", now=1.0)
    lm.transition(fid, LPhase.CONCEPT, by="owner")
    with pytest.raises(ValueError):
        lm.transition(fid, LPhase.IDEA, by="owner")


def test_unknown_franchise_rejected():
    lm = LifecycleManager()
    with pytest.raises(KeyError):
        lm.transition("nope", LPhase.CONCEPT, by="owner")


def test_milestones():
    lm = LifecycleManager()
    fid = lm.register("MS", now=1.0)
    mid = lm.add_milestone(fid, "Vertical Slice", LPhase.PROTOTYPE, 100, ["Playable"])
    m = lm.milestones[mid]
    assert m.status == MStatus.NOT_STARTED
    assert lm.achieve_milestone(mid, 100, ["Playable"]) is True
    assert m.status == MStatus.ACHIEVED
    assert lm.achieve_milestone(mid, 40, []) is False


def test_health_defaults():
    lm = LifecycleManager()
    fid = lm.register("H", budget=200, now=1.0)
    h = lm.health(fid)
    assert h["phase"] == "Idea"
    assert h["budget_used_ratio"] == 0.0
    assert h["kpi_crash_free"] == 100.0


def test_templates_complete():
    lm = LifecycleManager()
    assert set(lm.templates) == set(LPhase)
    assert lm.templates[LPhase.PRODUCTION].duration_days == 365

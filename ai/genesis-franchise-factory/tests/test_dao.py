import pytest

from gff.dao import FranchiseStatus, FranchiseVault, RoyaltyTier


def test_vault_deposit_withdraw():
    v = FranchiseVault()
    v.deposit(100.0, "alice", "test")
    assert v.balance == 100.0 and v.total_in == 100.0
    assert v.withdraw(40.0, "bob") is True
    assert v.balance == 60.0 and v.total_out == 40.0
    assert len(v.transactions) == 2


def test_vault_overdraft_rejected():
    v = FranchiseVault()
    v.deposit(10.0, "alice")
    assert v.withdraw(11.0, "bob") is False
    assert v.balance == 10.0


def test_vault_negative_deposit_rejected():
    with pytest.raises(ValueError):
        FranchiseVault().deposit(-1.0, "alice")


def test_create_franchise_dao():
    from gff.dao import FranchiseFactory

    ff = FranchiseFactory()
    f = ff.create("My IP", "owner1", "Test", "MIP", 1_000_000, RoyaltyTier.GOLD)
    assert f.status == FranchiseStatus.ACTIVE
    assert f.royalty_tier == RoyaltyTier.GOLD
    assert f.members["owner1"] == 200_000.0  # 20 % Owner-Stake
    assert ff.by_owner("owner1") == [f]
    assert ff.get(f.id) is f


def test_join_and_voting_power():
    from gff.dao import FranchiseFactory

    ff = FranchiseFactory()
    f = ff.create("IP2", "owner", "d", "IP2")
    assert ff.join(f.id, "member1", 100.0) is True
    assert ff.join("unknown", "x", 1.0) is False
    assert f.voting_power("member1") == 100.0 / (200_000.0 + 100.0)


def test_suspend_blocks_join():
    from gff.dao import FranchiseFactory

    ff = FranchiseFactory()
    f = ff.create("IP3", "owner", "d", "IP3")
    ff.suspend(f.id)
    assert f.status == FranchiseStatus.SUSPENDED
    assert ff.join(f.id, "m", 1.0) is False


def test_distribute_revenue_royalty():
    from gff.dao import FranchiseFactory

    ff = FranchiseFactory()
    f = ff.create("IP4", "owner", "d", "IP4", royalty_tier=RoyaltyTier.SILVER)
    ff.join(f.id, "m1", 100.0)
    result = f.distribute_revenue(1000.0)
    assert result["royalty"] == pytest.approx(40.0)  # 4 % SILVER
    assert result["net"] == pytest.approx(960.0)
    assert sum(result["distribution"].values()) == pytest.approx(960.0)
    assert f.vault.balance == pytest.approx(1100.0)  # 100 Join-Stake + 1000 Einnahme


def test_stats():
    from gff.dao import FranchiseFactory

    ff = FranchiseFactory()
    ff.create("A", "o", "d", "A")
    ff.create("B", "o", "d", "B")
    s = ff.stats()
    assert s["total"] == 2 and s["active"] == 2 and s["total_vault"] == 0.0

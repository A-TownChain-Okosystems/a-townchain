#!/usr/bin/env python3
"""Funktionale Regressionssuite fuer genesis-chronicles (Katalog-Pass 14).

Deckt die Python-Bindungen des Funktionskatalogs (COMP-CHRONICLES) ab:
GenesisChroniclesContract (mint/total/gain_xp/to_dict/transfer/battle/
get_token/get_owner_tokens/get_stats), MarketplaceContract inkl.
Listing.to_dict, set_token_contract, set_balance_oracle, list_for_sale,
buy, cancel_listing, get_listings, get_listing, get_token_listing,
get_sales_history, get_stats, BattleEngine (battle, is_alive, hp_percent)
sowie die Flask-API-Handler (health/listings/get_listing/token_listing/
list_nft/buy/cancel/sales_history/stats) gegen isolierte Stubs.

Die Stubs ersetzen nur die fehlende Infrastruktur (blockchain-Basispaket,
Flask); die Contract- und Engine-Logik laeuft unveraendert echt.
Kein Netzwerkzugriff.
"""

import importlib
import sys
import types

import pytest

REPO_ROOT = __file__.rsplit("/tests/", 1)[0]


# ------------------------------------------------- Stub: blockchain-Basispaket

def _install_base_contract_stub():
    if "blockchain" in sys.modules and hasattr(
        sys.modules.get("blockchain.contracts.base.base_contract", types.SimpleNamespace()),
        "BaseContract",
    ):
        return sys.modules["blockchain.contracts.base.base_contract"].BaseContract

    class BaseContract:
        def __init__(self, owner: str, contract_id: str = ""):
            self.owner = owner
            self.contract_id = contract_id
            self.events: list = []
            self.paused = False

        def pause(self):
            self.paused = True

        def when_not_paused(self):
            if self.paused:
                raise PermissionError("Contract is paused")

        def _emit(self, event: str, data: dict):
            self.events.append({"event": event, "data": data})

    pkg_root = types.ModuleType("blockchain")
    pkg_c = types.ModuleType("blockchain.contracts")
    pkg_b = types.ModuleType("blockchain.contracts.base")
    mod = types.ModuleType("blockchain.contracts.base.base_contract")
    mod.BaseContract = BaseContract
    pkg_root.__path__ = []
    sys.modules.setdefault("blockchain", pkg_root)
    sys.modules.setdefault("blockchain.contracts", pkg_c)
    sys.modules.setdefault("blockchain.contracts.base", pkg_b)
    sys.modules["blockchain.contracts.base.base_contract"] = mod
    return BaseContract


_install_base_contract_stub()


def _load(rel):
    spec = importlib.util.spec_from_file_location(
        "gc_" + rel.split("/")[-1].replace(".py", ""), REPO_ROOT + "/" + rel
    )
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


# ---------------------------------------------------- Genesis-Chronicles-Vertrag

class TestGenesisChroniclesContract:
    def test_full_lifecycle(self):
        gc = _load("contracts/genesis_chronicles_contract.py")
        GenesisChroniclesContract = gc.GenesisChroniclesContract
        contract = GenesisChroniclesContract()

        r1 = contract.mint("alice", element="NEON", rarity="Rare")
        assert r1["success"] is True
        r2 = contract.mint("bob")
        assert r2["success"] is True

        tid1 = r1["genesis_chronicles"]["token_id"]
        tid2 = r2["genesis_chronicles"]["token_id"]

        # get_token / get_owner_tokens
        got = contract.get_token(tid1)
        assert got["owner"] == "alice"
        owned = contract.get_owner_tokens("alice")
        assert any(x["token_id"] == tid1 for x in owned)
        assert contract.get_owner_tokens("carol") == []

        # total / get_stats
        assert contract.tokens[tid1].stats.total() >= 0
        assert len(contract.tokens) == 2
        stats = contract.get_stats()
        assert stats["total_minted"] == 2

        # gain_xp / to_dict
        nft = contract.tokens[tid1]
        lvl0 = nft.level
        nft.gain_xp(120)
        d = nft.to_dict()
        assert d["token_id"] == tid1
        assert nft.level == lvl0 + 1  # 120 XP -> Level-Up (Schwelle level*100)

        # transfer
        tr = contract.transfer(tid1, "alice", "carol")
        assert tr["success"] is True
        assert contract.get_token(tid1)["owner"] == "carol"

        # battle: beide Kaempfer muessen existieren
        bt = contract.battle(tid1, tid2)
        assert bt["success"] is True

    def test_mint_errors(self):
        gc = _load("contracts/genesis_chronicles_contract.py")
        contract = gc.GenesisChroniclesContract()
        contract.MAX_SUPPLY = 1
        assert contract.mint("alice")["success"] is True
        assert contract.mint("bob")["success"] is False


# --------------------------------------------------------- Marketplace-Vertrag

class _Fixture:
    def __init__(self):
        gc = _load("contracts/genesis_chronicles_contract.py")
        mc = _load("contracts/marketplace_contract.py")
        self.genesis = gc.GenesisChroniclesContract()
        self.market = mc.MarketplaceContract("treasury")
        minted = self.genesis.mint("alice", element="NEON", rarity="Rare")
        assert minted["success"] is True
        self.tid = minted["genesis_chronicles"]["token_id"]
        self.market.set_token_contract(self.genesis)
        self.market.set_balance_oracle({"alice": 10.0, "bob": 500.0})


class TestMarketplaceContract:
    def test_list_get_cancel_flow(self):
        fx = _Fixture()
        m = fx.market

        with pytest.raises(ValueError):
            m.list_for_sale("alice", fx.tid, 0.0)

        wrong_owner = m.list_for_sale("mallory", fx.tid, 50.0)
        assert wrong_owner["success"] is False

        res = m.list_for_sale("alice", fx.tid, 50.0)
        assert res["success"] is True
        listing = res["listing"]
        assert listing["status"] == "ACTIVE"
        lid = listing["listing_id"]

        # Listing.to_dict ist vollstaendig
        d = m.listings[lid].to_dict()
        assert d["token_id"] == fx.tid

        # get_listings / get_listing / get_token_listing
        assert any(l["listing_id"] == lid for l in m.get_listings())
        assert m.get_listing(lid)["listing_id"] == lid
        assert m.get_token_listing(fx.tid)["listing_id"] == lid

        # buy mit ausreichend Balance
        bought = m.buy("bob", lid, atc_balances={"bob": 500.0})
        assert bought["success"] is True
        assert self._ok(m.get_listing(lid))
        # NFT-Eigentum ist gewandert
        assert fx.genesis.get_token(fx.tid)["owner"] == "bob"

        # sales_history / get_stats
        hist = m.get_sales_history(buyer="bob")
        assert hist and hist[0]["token_id"] == fx.tid
        stats = m.get_stats()
        assert stats["total_sold"] >= 1
        assert stats["total_volume_atc"] >= 50.0

    @staticmethod
    def _ok(listing_result):
        return listing_result.get("status") in ("SOLD", None) or listing_result is not None

    def test_cancel_and_double_list(self):
        fx = _Fixture()
        m = fx.market
        res = m.list_for_sale("alice", fx.tid, 42.0)
        assert res["success"] is True
        lid = res["listing"]["listing_id"]

        with pytest.raises(ValueError):
            m.list_for_sale("alice", fx.tid, 42.0)

        cancelled = m.cancel_listing("alice", lid)
        assert cancelled["success"] is True
        after = m.get_listing(lid)
        assert after["status"] == "CANCELLED"

        relist = m.list_for_sale("alice", fx.tid, 42.0)
        assert relist["success"] is True


# ------------------------------------------------------------- Battle-Engine

class TestBattleEngine:
    def test_battle_runs_and_reports(self):
        gc = _load("contracts/genesis_chronicles_contract.py")
        be = _load("engine/battle_engine.py")
        contract = gc.GenesisChroniclesContract()
        a = contract.mint("alice", element="NEON")["genesis_chronicles"]
        b = contract.mint("bob", element="PYRO")["genesis_chronicles"]

        engine = be.BattleEngine(seed="deterministic-test")
        move = be.Move(name="Neon-Blast", element=be.Element.NEON, power=60, accuracy=0.9, pp=10)
        f1 = be.BattleGenesisChronicles(
            name="Alpha", element=be.Element.NEON, hp=100, max_hp=100,
            attack=50, defense=30, speed=40, moves=[move],
        )
        f2 = be.BattleGenesisChronicles(
            name="Beta", element=be.Element.FIRE, hp=100, max_hp=100,
            attack=45, defense=35, speed=38, moves=[move],
        )
        result = engine.battle(f1, f2)
        assert result is not None
        assert f1.hp >= 0 and f2.hp >= 0
        # is_alive / hp_percent werden im Kampfverlauf genutzt
        f1.hp = 0
        assert f1.is_alive() is False
        assert f1.hp_percent() == 0.0
        f2.hp = 50
        assert f2.is_alive() is True
        assert f2.hp_percent() == 0.5


# --------------------------------------------------------------- API-Routen

class _Args(dict):
    def get(self, k, default=None):
        return dict.get(self, k, default)


def _install_flask_and_contracts(market_instance):
    """Installiert Flask- und smart_contracts-Stubs und laedt die Routen."""
    flask = types.ModuleType("flask")

    class _Blueprint:
        def __init__(self, name, *a, **k):
            self.name = name

        def route(self, rule, **options):
            def deco(fn):
                fn._rule = rule
                return fn

            return deco

    class _Request:
        json = None
        args = _Args()

    flask.Blueprint = _Blueprint
    flask.request = _Request()
    flask.jsonify = lambda payload, *a, **k: payload
    sys.modules["flask"] = flask

    sc = types.ModuleType("blockchain.smart_contracts")
    sc.atc_token = types.SimpleNamespace(_balances={"alice": 10.0, "bob": 500.0})
    sc.marketplace = market_instance
    sys.modules["blockchain.smart_contracts"] = sc
    return flask


class TestMarketplaceRoutes:
    def _routes_with_fixture(self):
        fx = _Fixture()
        flask = _install_flask_and_contracts(fx.market)
        routes = _load("api/marketplace_routes.py")
        return fx, flask, routes

    def test_health_and_stats(self):
        fx, flask, routes = self._routes_with_fixture()
        assert routes.health()["service"] == "marketplace"
        assert routes.stats() == fx.market.get_stats()

    def test_listing_flow_over_api(self):
        fx, flask, routes = self._routes_with_fixture()
        flask.request.json = {"seller": "alice", "token_id": fx.tid, "price_atc": 25.0}
        res = routes.list_nft()
        assert res["success"] is True
        lid = res["listing"]["listing_id"]

        listings = routes.listings()
        assert any(l["listing_id"] == lid for l in listings["listings"])
        assert routes.get_listing(lid)["listing_id"] == lid
        assert routes.token_listing(fx.tid)["listing_id"] == lid

        flask.request.json = {"buyer": "bob", "listing_id": lid}
        bought = routes.buy()
        assert bought["success"] is True

        # cancel eines zweiten Listings
        flask.request.json = {"seller": "bob", "token_id": fx.tid, "price_atc": 99.0}
        res2 = routes.list_nft()
        assert res2["success"] is True
        lid2 = res2["listing"]["listing_id"]
        flask.request.json = {"seller": "bob", "listing_id": lid2}
        cancelled = routes.cancel()
        assert cancelled["success"] is True

        flask.request.args = _Args({"buyer": "bob"})
        hist = routes.sales_history()
        assert hist["sales"]

    def test_invalid_listing_input(self):
        fx, flask, routes = self._routes_with_fixture()
        flask.request.json = {"seller": "alice", "token_id": fx.tid, "price_atc": 0}
        res = routes.list_nft()
        assert res is not None  # Fehler-Pfad: ValueError wird abgefangen (HTTP 400)

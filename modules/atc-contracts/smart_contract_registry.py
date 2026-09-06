# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
SmartContractRegistry — Zentrales Contract-Management
Issue #1: Smart Contract Implementation
"""
import time
from blockchain.contracts.base.base_contract import BaseContract


class SmartContractRegistry:
    """
    Verwaltet alle deployed Contracts der A-TownChain.
    Einheitlicher Zugriffspunkt fuer alle Contract-Operationen.
    """

    def __init__(self):
        self._contracts: dict[str, BaseContract] = {}
        self._deploy_log: list[dict] = []

    def deploy(self, contract: BaseContract, deployer: str) -> dict:
        addr = contract.address
        self._contracts[addr] = contract
        log = {
            "address":  addr,
            "name":     contract.name(),
            "deployer": deployer,
            "ts":       int(time.time()),
        }
        self._deploy_log.append(log)
        print(f"[REGISTRY] Deployed: {contract.name()} @ {addr}")
        return log

    def get(self, address: str) -> BaseContract:
        if address not in self._contracts:
            raise KeyError(f"Contract not found: {address}")
        return self._contracts[address]

    def list_all(self) -> list:
        return [
            {"address": addr, "name": c.name(),
             "paused": c.paused, "owner": c.owner}
            for addr, c in self._contracts.items()
        ]

    def call(self, address: str, method: str, *args, **kwargs):
        contract = self.get(address)
        fn = getattr(contract, method, None)
        if not fn:
            raise AttributeError(f"Method {method} not found on {contract.name()}")
        return fn(*args, **kwargs)

    def get_deploy_log(self) -> list:
        return self._deploy_log

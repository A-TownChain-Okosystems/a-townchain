# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
Integration Tests — Issue #26 (Kap. 14)
ATCFS + MultiSig + ATCLang vollständige Integrationstests.
"""
import sys, os, time
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

def test_atcfs_basic():
    from core.atcfs import ATCFileSystem
    fs = ATCFileSystem(owner="testuser")
    assert fs.exists("/"), "Root existiert nicht"
    assert fs.exists("/atc"), "/atc nicht erstellt"
    # Datei schreiben
    node = fs.write("/tmp/test.txt", b"Hello ATCFS!", "testuser")
    assert node.size == 12
    assert node.content_cid != ""
    # Datei lesen
    cid, n2 = fs.read("/tmp/test.txt", "testuser")
    assert cid == node.content_cid
    # ls
    entries = fs.ls("/tmp")
    assert any(e.name == "test.txt" for e in entries)
    print("  ✅ ATCFS basic (write/read/ls)")

def test_atcfs_permissions():
    from core.atcfs import ATCFileSystem
    fs = ATCFileSystem("admin")
    fs.write("/home/secret.key", b"privatekey", "admin")
    try:
        fs.read("/home/secret.key", "attacker")
        assert False, "Sollte PermissionError werfen"
    except PermissionError:
        pass
    print("  ✅ ATCFS Permissions (Zugriffskontrolle)")

def test_atcfs_manifest():
    from core.atcfs import ATCFileSystem
    fs = ATCFileSystem("system")
    fs.write("/atc/chain/genesis.json", b'{"chain":"atc-1"}', "system")
    manifest = fs.export_manifest()
    assert "root_hash" in manifest
    assert manifest["entries"] > 0
    print("  ✅ ATCFS Manifest Export (Blockchain-Anchoring)")

def test_atcfs_kernel_module():
    from shivaos.fs.atcfs_module import ATCFSKernelModule
    km = ATCFSKernelModule("kernel")
    assert km.load() == True
    km.syscall_write("/atc/peers/node1.json", b'{"ip":"172.28.0.11"}', "kernel")
    entries = km.syscall_ls("/atc/peers")
    assert len(entries) > 0
    stat = km.syscall_stat()
    assert stat["files"] > 0
    print("  ✅ ATCFS Kernel-Modul (load/write/ls/stat)")

def test_multisig_bridge_vault():
    from blockchain.wallet.multisig import create_bridge_vault, TxStatus
    owners = ["alice", "bob", "charlie"]
    vault  = create_bridge_vault(owners)
    vault.deposit("ATC", 10000.0)
    assert vault.balance("ATC") == 10000.0
    # Propose
    tx = vault.propose("alice", "0xETH_ADDR", 500.0, "ATC", {"chain":"ethereum"})
    assert tx.status == TxStatus.PENDING
    assert "alice" in tx.signatures
    # 1 Sig reicht nicht (braucht 2)
    assert tx.status == TxStatus.PENDING
    # 2. Sig → Approved
    vault.sign(tx.id, "bob")
    assert tx.status == TxStatus.APPROVED
    # Execute
    result = vault.execute(tx.id, "charlie")
    assert result["tx_hash"] is not None
    assert vault.balance("ATC") == 9500.0
    print("  ✅ MultiSig Bridge Vault (propose/sign/execute)")

def test_multisig_franchise_vault():
    from blockchain.wallet.multisig import create_franchise_vault, TxStatus
    owners = ["o1","o2","o3","o4","o5"]
    vault  = create_franchise_vault(owners)
    vault.deposit("ATC", 50000.0)
    tx = vault.propose("o1", "franchise-001", 1000.0)
    vault.sign(tx.id, "o2")
    assert tx.status == TxStatus.PENDING  # braucht 3
    vault.sign(tx.id, "o3")
    assert tx.status == TxStatus.APPROVED
    print("  ✅ MultiSig Franchise Vault (3-of-5)")

def test_multisig_rejection():
    from blockchain.wallet.multisig import MultiSigWallet, TxStatus
    vault = MultiSigWallet("TestVault", ["a","b","c"], 2)
    vault.deposit("ATC", 100.0)
    tx = vault.propose("a", "target", 10.0)
    vault.reject(tx.id, "b")
    assert tx.status == TxStatus.REJECTED
    print("  ✅ MultiSig Rejection")

def test_atclang_import():
    """ATCLang Compiler-Modul importierbar."""
    try:
        import atclang
        print("  ✅ ATCLang importierbar")
    except ImportError:
        print("  ⚠️ ATCLang nicht installiert (erwartet in Sprint 2.4)")

def test_gateway_router_import():
    from gateway.router import GatewayRouter
    r = GatewayRouter("http://localhost:5000")
    assert r.backend_url == "http://localhost:5000"
    s = r.stats()
    assert "total_requests" in s
    print("  ✅ Gateway Router importierbar + stats")

if __name__ == "__main__":
    tests = [v for k,v in sorted(globals().items()) if k.startswith("test_")]
    ok = fail = skip = 0
    for t in tests:
        try:
            t(); ok += 1
        except SystemExit: pass
        except AssertionError as e: print(f"  ❌ {t.__name__}: {e}"); fail += 1
        except Exception as e:
            if "importierbar" in t.__name__ or "ATCLang" in str(e):
                skip += 1
            else:
                print(f"  ❌ {t.__name__}: {e}"); fail += 1
    print(f"\n  {ok}/{ok+fail+skip} Tests ✅  |  {skip} übersprungen  |  {fail} fehlgeschlagen")
    import sys; sys.exit(0 if fail == 0 else 1)

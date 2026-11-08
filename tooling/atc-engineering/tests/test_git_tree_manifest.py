import importlib.util
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "validate_git_tree_inventory.py"
SPEC = importlib.util.spec_from_file_location("validate_git_tree_inventory", SCRIPT)
module = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(module)


def fixture():
    commit = "a" * 40
    file_sha = "b" * 40
    submodule_sha = "c" * 40
    manifest = {
        "schema_version": "1.0.0",
        "commit_sha": commit,
        "entry_count": 4,
        "entries": [
            {"path": "src", "mode": "040000", "type": "tree", "sha": "d" * 40},
            {"path": "src/lib.py", "mode": "100644", "type": "blob", "sha": file_sha},
            {"path": "vendor", "mode": "040000", "type": "tree", "sha": "e" * 40},
            {"path": "vendor/dep", "mode": "160000", "type": "commit", "sha": submodule_sha},
        ],
    }
    inventory = {
        "repository": "owner/repo",
        "observed_commit_sha": commit,
        "files": [{"path": "src/lib.py", "blob_sha": file_sha, "status": "ANALYSED"}],
    }
    return manifest, inventory


class GitTreeInventoryTests(unittest.TestCase):
    def test_exact_tree_file_directory_and_submodule_agreement_passes(self):
        manifest, inventory = fixture()
        findings = module.validate_side("source", manifest, inventory)
        self.assertEqual(findings, [])
        report = module.reconcile(manifest, inventory, manifest, inventory)
        self.assertEqual(report["status"], "PASS")
        self.assertEqual(report["counts"]["source_submodules"], 1)

    def test_missing_tracked_file_blocks(self):
        manifest, inventory = fixture()
        inventory["files"] = []
        codes = {item["code"] for item in module.validate_side("source", manifest, inventory)}
        self.assertIn("GIT_TREE_FILE_MISSING_FROM_INVENTORY", codes)

    def test_blob_sha_mismatch_blocks(self):
        manifest, inventory = fixture()
        inventory["files"][0]["blob_sha"] = "f" * 40
        codes = {item["code"] for item in module.validate_side("source", manifest, inventory)}
        self.assertIn("GIT_BLOB_SHA_MISMATCH", codes)

    def test_untracked_inventory_file_blocks(self):
        manifest, inventory = fixture()
        inventory["files"].append({"path": "ghost.py", "blob_sha": "1" * 40})
        codes = {item["code"] for item in module.validate_side("source", manifest, inventory)}
        self.assertIn("INVENTORY_FILE_NOT_IN_GIT_TREE", codes)

    def test_missing_tree_directory_blocks(self):
        manifest, inventory = fixture()
        manifest["entries"] = [item for item in manifest["entries"] if item["path"] != "src"]
        manifest["entry_count"] = len(manifest["entries"])
        codes = {item["code"] for item in module.validate_side("source", manifest, inventory)}
        self.assertIn("GIT_DIRECTORY_MISSING_FROM_TREE_MANIFEST", codes)

    def test_commit_mismatch_blocks(self):
        manifest, inventory = fixture()
        inventory["observed_commit_sha"] = "9" * 40
        codes = {item["code"] for item in module.validate_side("source", manifest, inventory)}
        self.assertIn("TREE_INVENTORY_COMMIT_MISMATCH", codes)

    def test_invalid_mode_type_pair_blocks(self):
        manifest, inventory = fixture()
        manifest["entries"][1]["mode"] = "160000"
        codes = {item["code"] for item in module.validate_side("source", manifest, inventory)}
        self.assertIn("INVALID_TREE_MODE_TYPE", codes)


if __name__ == "__main__":
    unittest.main()

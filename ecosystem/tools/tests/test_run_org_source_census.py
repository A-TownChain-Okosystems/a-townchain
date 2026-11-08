import json
import subprocess
import tempfile
import unittest
from pathlib import Path

from tools.run_org_source_census import run_census


class OrganizationCensusTests(unittest.TestCase):
    def _git(self, root, *args):
        subprocess.run(["git", "-C", str(root), *args], check=True, capture_output=True)

    def _clean_checkout(self, root):
        root.mkdir(parents=True)
        self._git(root, "init")
        self._git(root, "config", "user.email", "test@example.invalid")
        self._git(root, "config", "user.name", "Census Test")
        (root / "lib.rs").write_text("pub fn execute() {}\n", encoding="utf-8")
        self._git(root, "add", "lib.rs")
        self._git(root, "commit", "-m", "test: initial source")
        return subprocess.run(
            ["git", "-C", str(root), "rev-parse", "HEAD"],
            check=True, capture_output=True, text=True,
        ).stdout.strip()

    def test_missing_active_checkout_blocks_organization(self):
        with tempfile.TemporaryDirectory() as temp:
            manifest = {"owner": "example", "repositories": [{"name": "missing", "archived": False}]}
            report = run_census(Path(temp), manifest)
            self.assertEqual(report["status"], "BLOCKED")
            self.assertEqual(report["summary"]["active_blocked"], 1)
            self.assertEqual(report["blocked_active_repositories"], ["example/missing"])

    def test_clean_checkout_is_scanned_but_remains_residual(self):
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            sha = self._clean_checkout(workspace / "sample")
            manifest = {"owner": "example", "repositories": [{"name": "sample", "archived": False}]}
            report = run_census(workspace, manifest)
            self.assertEqual(report["status"], "RESIDUAL")
            self.assertEqual(report["summary"]["active_scanned"], 1)
            self.assertEqual(report["repositories"][0]["source_sha"], sha)
            self.assertEqual(report["repositories"][0]["report"]["functional_verification"], "NOT_PERFORMED")

    def test_expected_sha_mismatch_blocks_checkout(self):
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            actual_sha = self._clean_checkout(workspace / "sample")
            manifest = {
                "owner": "example",
                "repositories": [{
                    "name": "sample",
                    "archived": False,
                    "expected_sha": "0" * 40 if actual_sha != "0" * 40 else "1" * 40,
                }],
            }
            report = run_census(workspace, manifest)
            self.assertEqual(report["status"], "BLOCKED")
            self.assertIn("EXPECTED_SHA_MISMATCH", report["repositories"][0]["residuals"])
            self.assertEqual(report["repositories"][0]["source_sha"], actual_sha)
            self.assertFalse(report["repositories"][0]["working_tree_clean"])

    def test_private_missing_checkout_is_excluded_not_blocked(self):
        with tempfile.TemporaryDirectory() as temp:
            manifest = {"owner": "example", "repositories": [{"name": "private-demo", "archived": False, "private": True}]}
            report = run_census(Path(temp), manifest)
            self.assertEqual(report["status"], "RESIDUAL")
            self.assertEqual(report["summary"]["active_scanned"], 0)
            self.assertEqual(report["summary"]["active_blocked"], 0)
            self.assertEqual(report["summary"]["active_excluded_private"], 1)
            entry = report["repositories"][0]
            self.assertEqual(entry["status"], "EXCLUDED_PRIVATE_NO_CI_ACCESS")
            self.assertIn("PRIVATE_REPOSITORY_EXCLUDED_NO_CI_ACCESS", entry["residuals"])
            self.assertEqual(report["blocked_active_repositories"], [])

    def test_private_checkout_present_is_scanned_normally(self):
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            sha = self._clean_checkout(workspace / "private-demo")
            manifest = {"owner": "example", "repositories": [{"name": "private-demo", "archived": False, "private": True}]}
            report = run_census(workspace, manifest)
            self.assertEqual(report["status"], "RESIDUAL")
            self.assertEqual(report["summary"]["active_scanned"], 1)
            self.assertEqual(report["summary"]["active_excluded_private"], 0)
            self.assertEqual(report["repositories"][0]["source_sha"], sha)

    def test_mixed_manifest_exclusion_keeps_other_failures_blocking(self):
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            self._clean_checkout(workspace / "public-ok")
            manifest = {"owner": "example", "repositories": [
                {"name": "public-ok", "archived": False},
                {"name": "private-demo", "archived": False, "private": True},
                {"name": "public-missing", "archived": False},
            ]}
            report = run_census(workspace, manifest)
            self.assertEqual(report["status"], "BLOCKED")
            self.assertEqual(report["summary"]["active_scanned"], 1)
            self.assertEqual(report["summary"]["active_excluded_private"], 1)
            self.assertEqual(report["summary"]["active_blocked"], 1)
            self.assertEqual(report["blocked_active_repositories"], ["example/public-missing"])

    def test_dirty_checkout_is_blocked_not_silently_scanned(self):
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            root = workspace / "sample"
            self._clean_checkout(root)
            (root / "lib.rs").write_text("pub fn changed() {}\n", encoding="utf-8")
            report = run_census(workspace, {"owner": "example", "repositories": [{"name": "sample"}]})
            self.assertEqual(report["status"], "BLOCKED")
            self.assertIn("WORKTREE_DIRTY", report["repositories"][0]["residuals"])


if __name__ == "__main__":
    unittest.main()

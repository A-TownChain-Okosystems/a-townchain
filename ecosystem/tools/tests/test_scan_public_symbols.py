import tempfile
import unittest
from contextlib import redirect_stderr
from io import StringIO
import subprocess
import sys
from unittest.mock import patch
from pathlib import Path

from tools.scan_public_symbols import main, scan


class PublicSymbolScanTests(unittest.TestCase):
    def test_rust_exports_are_discovered_without_claiming_verification(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "src").mkdir()
            (root / "src" / "lib.rs").write_text(
                "pub struct Api;\npub fn execute() {}\nfn private_helper() {}\n",
                encoding="utf-8",
            )
            report = scan(root, "example/repo", "a" * 40)
            symbols = {
                (entry["kind"], entry["name"])
                for file in report["files"]
                for entry in file["candidate_public_symbols"]
            }
            self.assertIn(("type", "Api"), symbols)
            self.assertIn(("function", "execute"), symbols)
            self.assertNotIn(("function", "private_helper"), symbols)
            self.assertEqual(report["completeness_status"], "RESIDUAL")
            self.assertEqual(report["functional_verification"], "NOT_PERFORMED")

    def test_python_private_names_are_excluded(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "api.py").write_text(
                "def run():\n    pass\ndef _helper():\n    pass\nclass Client:\n    pass\n",
                encoding="utf-8",
            )
            report = scan(root, "example/python-api", "b" * 40)
            symbols = {
                (entry["kind"], entry["name"])
                for file in report["files"]
                for entry in file["candidate_public_symbols"]
            }
            self.assertIn(("function", "run"), symbols)
            self.assertIn(("type", "Client"), symbols)
            self.assertNotIn(("function", "_helper"), symbols)

    def test_cli_rejects_source_sha_that_differs_from_git_head(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "lib.rs").write_text("pub fn execute() {}\\n", encoding="utf-8")
            subprocess.run(["git", "-C", str(root), "init"], check=True, capture_output=True)
            subprocess.run(["git", "-C", str(root), "config", "user.email", "test@example.invalid"], check=True)
            subprocess.run(["git", "-C", str(root), "config", "user.name", "Scanner Test"], check=True)
            subprocess.run(["git", "-C", str(root), "add", "lib.rs"], check=True)
            subprocess.run(["git", "-C", str(root), "commit", "-m", "test: source fixture"], check=True, capture_output=True)
            args = ["scan_public_symbols.py", "--root", str(root), "--repository", "example/repo",
                    "--source-sha", "0" * 40, "--output", str(root / "report.json")]
            error = StringIO()
            with patch.object(sys, "argv", args), redirect_stderr(error):
                with self.assertRaises(SystemExit) as raised:
                    main()
            self.assertEqual(raised.exception.code, 2)
            self.assertIn("source-sha mismatch", error.getvalue())
            self.assertFalse((root / "report.json").exists())

    def test_typescript_exported_entrypoints_are_discovered(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "index.ts").write_text(
                "export function execute() {}\nexport class Client {}\nexport const VERSION = '1';\n",
                encoding="utf-8",
            )
            report = scan(root, "example/typescript-api", "c" * 40)
            symbols = {
                (entry["kind"], entry["name"])
                for file in report["files"]
                for entry in file["candidate_public_symbols"]
            }
            self.assertIn(("function", "execute"), symbols)
            self.assertIn(("type", "Client"), symbols)
            self.assertIn(("value", "VERSION"), symbols)


if __name__ == "__main__":
    unittest.main()

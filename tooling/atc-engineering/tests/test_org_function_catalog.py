import importlib.util
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "org_function_catalog.py"
SPEC = importlib.util.spec_from_file_location("org_function_catalog", SCRIPT)
catalog = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(catalog)


class CatalogTests(unittest.TestCase):
    def test_git_blob_hash_matches_known_empty_blob(self):
        self.assertEqual(catalog.blob_sha(b""), "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")

    def test_symbol_id_does_not_depend_on_line_number(self):
        first = catalog.extract_python("owner/repo", "example.py", "def top(x):\n    return x\n", "a" * 40, "b" * 40)[0]
        shifted = catalog.extract_python("owner/repo", "example.py", "# inserted comment\ndef top(x):\n    return x\n", "c" * 40, "d" * 40)[0]
        self.assertEqual(first["symbol_id"], shifted["symbol_id"])
        self.assertNotEqual(first["source_start_line"], shifted["source_start_line"])

    def test_python_ast_finds_functions_and_methods_with_lines(self):
        source = "def top(x):\n    return x\n\nclass Thing:\n    def method(self):\n        pass\n"
        records = catalog.extract_python("owner/repo", "example.py", source, "a" * 40, "b" * 40)
        self.assertEqual([r["qualified_name"] for r in records], ["top", "Thing", "Thing.method"])
        self.assertEqual(records[0]["source_start_line"], 1)
        self.assertEqual(records[2]["source_start_line"], 5)
        self.assertTrue(all(r["observed_commit_sha"] == "b" * 40 for r in records))
        self.assertTrue(all("component_id" in r and "contract_refs" in r for r in records))

    def test_invalid_python_is_reported(self):
        with self.assertRaises(ValueError):
            catalog.extract_python("owner/repo", "broken.py", "def broken(:\n", "a" * 40, "b" * 40)

    def test_rust_extraction_is_explicitly_heuristic(self):
        records = catalog.extract_heuristic(
            "owner/repo", "src/lib.rs", "rust", "pub fn simple_hash(value: u32) -> u32 { value }\n",
            "a" * 40, "b" * 40
        )
        self.assertEqual(len(records), 1)
        self.assertEqual(records[0]["qualified_name"], "simple_hash")
        self.assertTrue(records[0]["limitations"])

    def test_heuristic_signature_uses_declaration_line_after_blank_lines(self):
        source = "\n\npub fn first(value: u32) -> u32 { value }\n\n\npub fn second() {}\n"
        records = catalog.extract_heuristic(
            "owner/repo", "src/lib.rs", "rust", source, "a" * 40, "b" * 40
        )
        self.assertEqual([record["qualified_name"] for record in records], ["first", "second"])
        self.assertEqual(records[0]["source_start_line"], 3)
        self.assertEqual(records[1]["source_start_line"], 6)
        self.assertEqual(records[0]["signature"], "pub fn first(value: u32) -> u32 { value }")
        self.assertEqual(records[1]["signature"], "pub fn second() {}")
        self.assertTrue(all(record["signature"].strip() for record in records))

    def test_document_parsers_accept_valid_and_reject_malformed_inputs(self):
        valid = {
            "json": '{"key": 1}',
            "markdown": "# Heading\n\nA paragraph.",
            "toml": '[package]\nname = "demo"\n',
            "yaml": "key: value\nitems:\n  - one\n",
        }
        invalid = {
            "json": '{"key": ',
            "markdown": "# Heading\n\nA paragraph.",  # CommonMark accepts ordinary text syntax.
            "toml": '[package\nname = "demo"',
            "yaml": "key: [unterminated",
        }
        for language, text in valid.items():
            with self.subTest(language=language):
                catalog.validate_document_format(language, text)
        for language in ("json", "toml", "yaml"):
            with self.subTest(language=language):
                with self.assertRaises(Exception):
                    catalog.validate_document_format(language, invalid[language])

    def test_rust_tree_sitter_extraction_uses_ast_ranges_and_nonempty_signatures(self):
        source = "\n\npub fn first(value: u32) -> u32 { value }\n\n\npub fn second() {}\n"
        records = catalog.extract_tree_sitter(
            "owner/repo", "src/lib.rs", "rust", source, "a" * 40, "b" * 40
        )
        self.assertEqual([record["qualified_name"] for record in records], ["first", "second"])
        self.assertEqual(records[0]["source_start_line"], 3)
        self.assertEqual(records[1]["source_start_line"], 6)
        self.assertTrue(all(record["signature"].strip() for record in records))
        self.assertTrue(all(not record["limitations"] for record in records))

    def test_symlink_is_inventoried_as_git_blob_without_following_target(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "target.txt").write_text("target contents", encoding="utf-8")
            (root / "link.txt").symlink_to("target.txt")
            result = catalog.scan(root, "owner/sample")
            link = next(item for item in result["files"] if item["path"] == "link.txt")
            self.assertEqual(link["status"], "SKIPPED")
            self.assertEqual(link["blob_sha"], catalog.blob_sha(b"target.txt"))
            exclusion = next(item for item in result["exclusions"] if item["path"] == "link.txt")
            self.assertEqual(exclusion["reason_code"], "SYMLINK_FILE_NOT_FOLLOWED")
            self.assertIn("link.txt#" + link["blob_sha"], exclusion["evidence"][0]["reference"])

    def test_git_worktree_metadata_is_not_inventoried(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / ".git").write_text("gitdir: /tmp/example/.git/worktrees/sample\n", encoding="utf-8")
            (root / "source.py").write_text("def visible(): pass\n", encoding="utf-8")
            result = catalog.scan(root, "owner/sample")
            paths = {entry["path"] for entry in result["files"]}
            self.assertIn("source.py", paths)
            self.assertNotIn(".git", paths)
            self.assertFalse(any(item["path"] == ".git" for item in result["exclusions"]))

    def test_scan_indexes_unsupported_files_with_reason_coded_exclusion(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "image.bin").write_bytes(b"binary")
            result = catalog.scan(root, "owner/sample")
            self.assertEqual(len(result["files"]), 1)
            self.assertEqual(result["files"][0]["status"], "UNSUPPORTED")
            exclusion = result["exclusions"][0]
            self.assertEqual(exclusion["reason_code"], "UNCLASSIFIED_UNKNOWN_FORMAT")
            self.assertEqual(exclusion["category"], "unsupported_format")
            self.assertEqual(exclusion["repository"], "owner/sample")
            self.assertEqual(exclusion["source_blob_sha"], catalog.blob_sha(b"binary"))
            self.assertRegex(exclusion["exclusion_id"], r"^[0-9a-f]{64}$")
            self.assertEqual(exclusion["rationale"], exclusion["reason"])
            self.assertTrue(exclusion["evidence"])
            self.assertIn(exclusion["source_blob_sha"], exclusion["evidence"][0]["reference"])
            self.assertTrue(result["limitations"])

    def test_excluded_directories_are_recorded_without_scanning_contents(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            excluded = root / "target"
            excluded.mkdir()
            (excluded / "generated.py").write_text("def hidden(): pass\n", encoding="utf-8")
            (root / "src.py").write_text("def visible(): pass\n", encoding="utf-8")
            result = catalog.scan(root, "owner/sample")
            paths = {entry["path"] for entry in result["files"]}
            self.assertIn("src.py", paths)
            self.assertNotIn("target/generated.py", paths)
            records = [item for item in result["exclusions"] if item["path"] == "target"]
            self.assertEqual(len(records), 1)
            self.assertEqual(records[0]["reason_code"], "EXCLUDED_DIRECTORY")
            self.assertRegex(records[0]["exclusion_id"], r"^[0-9a-f]{64}$")
            self.assertEqual(records[0]["category"], "scanner_policy")
            self.assertIsNone(records[0]["source_blob_sha"])
            self.assertEqual(records[0]["evidence"][0]["reference"], "SKIP_DIRS:target")

    def test_scan_reports_parser_coverage_per_language_and_does_not_claim_verification(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "lib.rs").write_text("pub fn visible() {}\n", encoding="utf-8")
            (root / "data.yaml").write_text("key: value\n", encoding="utf-8")
            result = catalog.scan(root, "owner/sample")
            self.assertEqual(result["parser_coverage"]["rust"]["parser_mode"], "tree_sitter_rust_cst")
            self.assertEqual(result["parser_coverage"]["rust"]["status"], "AUTHORITATIVE_COMPLETE")
            self.assertEqual(result["parser_coverage"]["rust"]["symbols_detected"], 1)
            self.assertEqual(result["parser_coverage"]["yaml"]["parser_mode"], "pyyaml_ast")
            self.assertEqual(result["parser_coverage"]["yaml"]["status"], "AUTHORITATIVE_COMPLETE")
            self.assertEqual(result["parser_coverage"]["yaml"]["unsupported_files"], 0)
            self.assertEqual(result["status"], "DETECTED")

    def test_reconciliation_reports_added_removed_and_changed_symbols(self):
        old = {"repository": "owner/repo", "observed_commit_sha": "a" * 40, "symbols": [
            {"symbol_id": "one", "source_blob_sha": "x", "signature": "fn old()"},
            {"symbol_id": "two", "source_blob_sha": "y", "signature": "fn changed()"},
        ]}
        current = {"repository": "owner/repo", "observed_commit_sha": "b" * 40, "symbols": [
            {"symbol_id": "two", "source_blob_sha": "z", "signature": "fn changed()"},
            {"symbol_id": "three", "source_blob_sha": "q", "signature": "fn new()"},
        ], "exclusions": []}
        report = catalog.reconcile(current, old)
        self.assertEqual(report["added_symbol_ids"], ["three"])
        self.assertEqual(report["removed_symbol_ids"], ["one"])
        self.assertEqual(report["changed_symbol_ids"], ["two"])
        self.assertEqual(report["status"], "RESIDUAL")

    def test_reconciliation_rejects_wrong_repository_baseline(self):
        with self.assertRaises(ValueError):
            catalog.reconcile({"repository": "owner/a", "symbols": []}, {"repository": "owner/b", "symbols": []})

    def test_scan_disambiguates_duplicate_heuristic_identities(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "lib.rs").write_text("pub fn duplicate() {}\npub fn duplicate() {}\n", encoding="utf-8")
            result = catalog.scan(root, "owner/sample")
            ids = [symbol["symbol_id"] for symbol in result["symbols"]]
            self.assertEqual(len(ids), 2)
            self.assertEqual(len(ids), len(set(ids)))


if __name__ == "__main__":
    unittest.main()

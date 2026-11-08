import importlib.util
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "build_pr29_finding_remediation_ledger.py"
SPEC = importlib.util.spec_from_file_location("build_pr29_finding_remediation_ledger", SCRIPT)
module = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(module)


class FindingRemediationLedgerTests(unittest.TestCase):
    def test_terminal_qualified_name_matches_parser_scope(self):
        self.assertEqual(module.base_name("RepositoryId::as_str"), "as_str")
        self.assertEqual(module.base_name("Owner.Type.method"), "method")

    def test_empty_signature_finding_maps_to_unique_current_symbol(self):
        old = {
            "symbols": [{
                "symbol_id": "old", "path": "src/lib.rs", "qualified_name": "as_str",
                "source_blob_sha": "a" * 40, "source_start_line": 41,
            }]
        }
        current = {
            "symbols": [{
                "symbol_id": "new", "path": "src/lib.rs", "qualified_name": "RepositoryId::as_str",
                "source_blob_sha": "a" * 40, "source_start_line": 42,
                "source_end_line": 42, "signature": "pub fn as_str(&self) -> &str",
            }]
        }
        result = module.resolve_symbol_finding(
            {"side": "source", "detail": "old:signature"},
            {"source": old, "catalog": old},
            {"source": current, "catalog": current},
        )
        self.assertEqual(result["status"], "RESOLVED")
        self.assertEqual(result["current_symbol_id"], "new")
        self.assertTrue(result["signature_sha256"])

    def test_ambiguous_symbol_match_stays_unresolved(self):
        old = {"symbols": [{"symbol_id": "old", "path": "src/lib.rs", "qualified_name": "as_str", "source_blob_sha": "a" * 40, "source_start_line": 41}]}
        symbol = {"symbol_id": "new", "path": "src/lib.rs", "qualified_name": "A::as_str", "source_blob_sha": "a" * 40, "source_start_line": 42, "signature": "pub fn as_str(&self) -> &str"}
        current = {"symbols": [symbol, dict(symbol, symbol_id="new2", qualified_name="B::as_str")]}
        result = module.resolve_symbol_finding({"side": "source", "detail": "old:signature"}, {"source": old, "catalog": old}, {"source": current, "catalog": current})
        self.assertEqual(result["status"], "UNRESOLVED")

    def test_file_status_requires_authoritative_parser_or_matching_exclusion(self):
        baseline_file = {"path": "Cargo.toml", "blob_sha": "b" * 40, "status": "UNSUPPORTED"}
        analysed = {
            "files": [{"path": "Cargo.toml", "blob_sha": "b" * 40, "status": "ANALYSED", "language": "toml"}],
            "parser_coverage": {"toml": {"status": "AUTHORITATIVE_COMPLETE", "parser_mode": "stdlib_tomllib", "unsupported_files": 0, "parse_errors": 0}},
            "exclusions": [],
        }
        result = module.resolve_file_status_finding({"side": "source", "detail": "Cargo.toml:UNSUPPORTED"}, {"source": {"files": [baseline_file]}, "catalog": {"files": [baseline_file]}}, {"source": analysed, "catalog": analysed})
        self.assertEqual(result["status"], "RESOLVED")
        analysed["files"][0]["status"] = "UNSUPPORTED"
        result = module.resolve_file_status_finding({"side": "source", "detail": "Cargo.toml:UNSUPPORTED"}, {"source": {"files": [baseline_file]}, "catalog": {"files": [baseline_file]}}, {"source": analysed, "catalog": analysed})
        self.assertEqual(result["status"], "UNRESOLVED")

    def test_parser_finding_requires_exact_mode_and_complete_status(self):
        inventory = {"parser_coverage": {"rust": {"parser_mode": "tree_sitter_rust_cst", "status": "AUTHORITATIVE_COMPLETE", "unsupported_files": 0, "parse_errors": 0}}}
        result = module.resolve_parser_finding({"side": "source", "detail": "rust:heuristic_regex:HEURISTIC_PARTIAL"}, {"source": inventory, "catalog": inventory})
        self.assertEqual(result["status"], "RESOLVED")
        inventory["parser_coverage"]["rust"]["status"] = "PARTIAL"
        result = module.resolve_parser_finding({"side": "source", "detail": "rust:heuristic_regex:HEURISTIC_PARTIAL"}, {"source": inventory, "catalog": inventory})
        self.assertEqual(result["status"], "UNRESOLVED")

    def test_tree_delta_requires_matching_sha_and_passing_independent_reports(self):
        entry = {"path": "docs/new.json", "change_type": "added", "source_blob_sha": None, "catalog_blob_sha": "c" * 40, "rationale": "reviewed", "evidence": ["compare"]}
        inventories = {"source": {"files": []}, "catalog": {"files": [{"path": "docs/new.json", "blob_sha": "c" * 40}]}}
        result = module.resolve_delta_finding({"code": "CATALOG_FILE_NOT_IN_SOURCE_CENSUS", "detail": "docs/new.json"}, inventories, {"expected_deltas": [entry]}, {"status": "PASS", "counts": {"blocking_findings": 0}}, {"status": "RECONCILED", "counts": {"blocking_findings": 0}})
        self.assertEqual(result["status"], "RESOLVED_AS_EXPECTED_DELTA")
        result = module.resolve_delta_finding({"code": "CATALOG_FILE_NOT_IN_SOURCE_CENSUS", "detail": "docs/new.json"}, inventories, {"expected_deltas": [entry]}, {"status": "BLOCKED", "counts": {"blocking_findings": 1}}, {"status": "RECONCILED", "counts": {"blocking_findings": 0}})
        self.assertEqual(result["status"], "UNRESOLVED")


if __name__ == "__main__":
    unittest.main()

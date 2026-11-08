import unittest

from tools.build_runtime_symbol_registry import build_registry, runtime_id
from tools.validate_runtime_symbol_registry import validate

SAMPLE_CENSUS = {
    "schema_id": "ATC-ORG-SOURCE-CENSUS-001",
    "status": "RESIDUAL",
    "repositories": [
        {
            "repository": "example/sample",
            "name": "sample",
            "source_sha": "a" * 40,
            "report": {"files": [
                {"path": "src/lib.rs", "language": "rust", "file_sha256": "f" * 64,
                 "candidate_public_symbols": [{"kind": "function", "name": "execute"}]},
                {"path": "src/types.rs", "language": "rust", "file_sha256": "e" * 64,
                 "candidate_public_symbols": [{"kind": "type", "name": "Config"}]},
                {"path": "tests/it.rs", "language": "rust", "file_sha256": "d" * 64,
                 "candidate_public_symbols": [{"kind": "function", "name": "helper"}]},
            ]},
        },
        {
            "repository": "A-TownChain-Okosystems/a-townchain-ecosystem",
            "name": "a-townchain-ecosystem",
            "source_sha": "b" * 40,
            "report": {"files": [
                {"path": "components/sample/src/lib.rs", "language": "rust", "file_sha256": "c" * 64,
                 "candidate_public_symbols": [{"kind": "function", "name": "execute"}]},
            ]},
        },
        {
            "repository": "example/quellrepo",
            "name": "quellrepo",
            "source_sha": "c" * 40,
            "report": {"files": [
                {"path": "src/main.rs", "language": "rust", "file_sha256": "b" * 64,
                 "candidate_public_symbols": [{"kind": "function", "name": "run"}]},
            ]},
        },
    ],
}
SAMPLE_CATALOG = {
    "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
    "components": [{
        "id": "COMP-SAMPLE",
        "name": "sample",
        "canonical_owner": "example/sample",
        "source_inventory": {"repository": "example/sample", "observed_head_sha": "a" * 40},
        "functions": [{
            "id": "FN-SAMPLE-001",
            "name": "execute",
            "source_binding": {"source_path": "src/lib.rs", "symbol_or_entrypoint": "execute",
                               "exact_source_sha": "a" * 40},
        }],
    }],
}
SAMPLE_MANIFEST = {
    "schema_id": "ATC-BUILD-MANIFEST-001",
    "repos": [
        {"repo": "sample", "status": "BUILD_ENTRY_EVIDENCED", "head_sha": "a" * 40},
        {"repo": "umbrella", "status": "BUILD_ENTRY_EVIDENCED", "head_sha": "b" * 40},
        {"repo": "quellrepo", "status": "CANONICAL_ELSEWHERE",
         "canonical_source_path": "sample/components/quellrepo", "head_sha": "c" * 40},
    ],
}


def build_sample_registry(catalog=SAMPLE_CATALOG, census=SAMPLE_CENSUS, manifest=SAMPLE_MANIFEST):
    return build_registry(census, catalog, manifest)


class RuntimeSymbolRegistryTests(unittest.TestCase):
    def test_every_candidate_is_accounted_exactly_once(self):
        registry = build_sample_registry()
        total = registry["summary"]["candidate_symbols_total"]
        self.assertEqual(total, 5)
        self.assertEqual(registry["summary"]["bound_to_catalog_functions"], 1)
        self.assertEqual(registry["summary"]["runtime_registry_entries"], 2)
        self.assertEqual(registry["summary"]["mirrored_reference_non_ssot"], 1)
        self.assertEqual(registry["summary"]["exclusions"].get("EXCLUDED_TEST"), 1)

    def test_runtime_ids_are_content_derived_and_stable(self):
        registry = build_sample_registry()
        ids = [entry["id"] for entry in registry["entries"]]
        self.assertEqual(len(ids), len(set(ids)))
        for entry in registry["entries"]:
            self.assertEqual(entry["id"], runtime_id(
                entry["repository"], entry["source_path"], entry["kind"], entry["symbol"]))
        rebuilt = build_sample_registry()
        self.assertEqual(ids, [entry["id"] for entry in rebuilt["entries"]])

    def test_quellrepo_entries_carry_ssot_finding(self):
        registry = build_sample_registry()
        quell = [e for e in registry["entries"] if e["repository"] == "example/quellrepo"]
        self.assertEqual(len(quell), 1)
        self.assertEqual(quell[0]["ssot"]["finding"], "QUERLREPO_REFERENCE_NON_SSOT")
        self.assertEqual(quell[0]["ssot"]["canonical_source_path"], "sample/components/quellrepo")

    def test_duplicate_catalog_binding_blocks_registry(self):
        catalog = {
            "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
            "components": [{
                "id": "COMP-SAMPLE",
                "name": "sample",
                "canonical_owner": "example/sample",
                "source_inventory": {"repository": "example/sample"},
                "functions": [
                    {"id": "FN-SAMPLE-001", "name": "execute",
                     "source_binding": {"source_path": "src/lib.rs", "symbol_or_entrypoint": "execute"}},
                    {"id": "FN-SAMPLE-002", "name": "execute again",
                     "source_binding": {"source_path": "src/lib.rs", "symbol_or_entrypoint": "execute"}},
                ],
            }],
        }
        registry = build_sample_registry(catalog=catalog)
        self.assertEqual(registry["status"], "BLOCKED")
        self.assertEqual(len(registry["catalog_duplicate_findings"]["duplicate_source_bindings"]), 1)

    def test_delegated_binding_is_documented_not_duplicate(self):
        catalog = {
            "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
            "components": [{
                "id": "COMP-SAMPLE",
                "name": "sample",
                "canonical_owner": "example/sample",
                "source_inventory": {"repository": "example/sample"},
                "functions": [
                    {"id": "FN-SAMPLE-001", "name": "execute",
                     "source_binding": {"source_path": "src/lib.rs", "symbol_or_entrypoint": "execute"}},
                    {"id": "FN-SAMPLE-002", "name": "delegated view",
                     "source_binding": {"delegates_to_function_id": "FN-SAMPLE-001"}},
                ],
            }],
        }
        registry = build_sample_registry(catalog=catalog)
        self.assertEqual(registry["status"], "RESIDUAL")
        self.assertEqual(registry["catalog_duplicate_findings"]["duplicate_source_bindings"], [])
        self.assertEqual(len(registry["catalog_duplicate_findings"]["delegated_bindings"]), 1)


class RuntimeSymbolRegistryValidatorTests(unittest.TestCase):
    def test_valid_registry_passes_all_gates(self):
        registry = build_sample_registry()
        errors, summary = validate(registry, SAMPLE_CENSUS, SAMPLE_MANIFEST)
        self.assertEqual(errors, [])
        self.assertEqual(summary["gates"]["R3_accounting"], "PASS")

    def test_tampered_id_fails(self):
        registry = build_sample_registry()
        registry["entries"][0]["id"] = "RTS-000000000000"
        errors, _ = validate(registry, SAMPLE_CENSUS, SAMPLE_MANIFEST)
        self.assertTrue(any("content-derived" in e for e in errors))

    def test_hidden_candidate_fails_accounting(self):
        registry = build_sample_registry()
        registry["summary"]["per_repository"]["example/sample"]["RUNTIME_SYMBOL"] -= 1
        errors, _ = validate(registry, SAMPLE_CENSUS, SAMPLE_MANIFEST)
        self.assertTrue(any("classification total" in e for e in errors))

    def test_missing_ssot_finding_fails(self):
        registry = build_sample_registry()
        for entry in registry["entries"]:
            if entry["repository"] == "example/quellrepo":
                entry["ssot"] = None
        errors, _ = validate(registry, SAMPLE_CENSUS, SAMPLE_MANIFEST)
        self.assertTrue(any("SSOT reference finding" in e for e in errors))


if __name__ == "__main__":
    unittest.main()

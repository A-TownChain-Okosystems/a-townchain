import unittest

from tools.reconcile_component_function_catalog import reconcile


class CatalogReconciliationTests(unittest.TestCase):
    def setUp(self):
        self.catalog = {
            "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
            "components": [{
                "id": "COMP-SAMPLE",
                "name": "sample",
                "canonical_owner": "example/sample",
                "source_inventory": {
                    "repository": "example/sample",
                    "observed_head_sha": "a" * 40,
                },
                "functions": [{
                    "id": "FN-SAMPLE-001",
                    "name": "execute",
                    "source_binding": {
                        "source_path": "src/lib.rs",
                        "symbol_or_entrypoint": "execute",
                        "exact_source_sha": "a" * 40,
                    },
                }],
            }],
        }
        self.census = {
            "schema_id": "ATC-ORG-SOURCE-CENSUS-001",
            "status": "RESIDUAL",
            "repositories": [{
                "repository": "example/sample",
                "status": "RESIDUAL",
                "source_sha": "a" * 40,
                "report": {
                    "files": [{
                        "path": "src/lib.rs",
                        "candidate_public_symbols": [{"kind": "function", "name": "execute"}],
                    }],
                },
            }],
        }

    def test_exact_sha_symbol_match_remains_functionally_residual(self):
        report = reconcile(self.catalog, self.census)
        self.assertEqual(report["status"], "RESIDUAL")
        self.assertEqual(report["summary"]["candidate_symbols_unbound"], 0)
        finding = report["function_findings"][0]
        self.assertEqual(finding["finding"], "LEXICAL_MATCH_ONLY_FUNCTIONAL_VERIFICATION_PENDING")
        self.assertEqual(report["summary"]["functional_verification"], "NOT_PERFORMED")

    def test_sha_mismatch_is_not_accepted_as_a_binding(self):
        self.catalog["components"][0]["functions"][0]["source_binding"]["exact_source_sha"] = "b" * 40
        report = reconcile(self.catalog, self.census)
        self.assertEqual(report["function_findings"][0]["finding"], "FUNCTION_BINDING_SHA_MISMATCH")
        self.assertEqual(report["summary"]["candidate_symbols_unbound"], 1)

    def test_missing_source_checkout_blocks_reconciliation(self):
        self.census["repositories"] = []
        report = reconcile(self.catalog, self.census)
        self.assertEqual(report["status"], "BLOCKED")
        self.assertEqual(report["function_findings"][0]["finding"], "SOURCE_CENSUS_UNAVAILABLE")


class DocumentedExclusionTests(unittest.TestCase):
    def _base(self):
        catalog = {
            "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
            "components": [{
                "id": "COMP-SAMPLE", "name": "sample", "canonical_owner": "example/sample",
                "source_inventory": {"repository": "example/sample", "observed_head_sha": "a" * 40},
                "functions": [],
            }],
        }
        census = {
            "schema_id": "ATC-ORG-SOURCE-CENSUS-001", "status": "RESIDUAL",
            "repositories": [{
                "repository": "example/sample", "status": "RESIDUAL", "source_sha": "a" * 40,
                "report": {"files": []},
            }],
        }
        return catalog, census

    def test_non_public_helper_absence_is_documented(self):
        catalog, census = self._base()
        catalog["components"][0]["functions"].append({
            "id": "FN-S-001", "name": "helper",
            "source_binding": {"source_path": "src/lib.rs", "symbol_or_entrypoint": "helper",
                                "binding_kind": "non_public_internal_helper", "exact_source_sha": "a" * 40},
        })
        report = reconcile(catalog, census)
        self.assertEqual(report["function_findings"][0]["finding"], "NON_PUBLIC_INTERNAL_HELPER_EXPECTED_ABSENT")
        self.assertEqual(report["summary"]["function_bindings_documented_exclusions"], 1)

    def test_delegated_binding_is_documented_not_duplicate(self):
        catalog, census = self._base()
        catalog["components"][0]["functions"].append({
            "id": "FN-S-002", "name": "delegated",
            "source_binding": {"delegates_to_function_id": "FN-S-001"},
        })
        report = reconcile(catalog, census)
        self.assertEqual(report["function_findings"][0]["finding"], "DELEGATED_BINDING_DOCUMENTED")
        self.assertEqual(report["summary"]["function_bindings_documented_exclusions"], 1)

    def test_self_referential_head_is_documented_not_mismatch(self):
        catalog, census = self._base()
        catalog["components"][0]["source_inventory"]["observed_head_sha"] = "b" * 40
        catalog["components"][0]["source_inventory"]["self_referential_head"] = True
        report = reconcile(catalog, census)
        self.assertEqual(report["component_findings"][0]["finding"], "SELF_REFERENTIAL_HEAD_BOUND_AT_CENSUS_RUN")
        self.assertEqual(report["summary"]["self_referential_component_heads"], 1)


if __name__ == "__main__":
    unittest.main()

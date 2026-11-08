"""Regression tests for tools.validate_catalog_taxonomy."""
import copy
import json
import unittest
from pathlib import Path

from tools.validate_catalog_taxonomy import validate_taxonomy

ROOT = Path(__file__).resolve().parents[2]
TAXONOMY_PATH = ROOT / "docs/architecture/ATC-CATALOG-TAXONOMY-001.json"
CATALOG_PATH = ROOT / "docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json"


class TaxonomyValidatorTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.taxonomy = json.loads(TAXONOMY_PATH.read_text(encoding="utf-8"))
        cls.catalog = json.loads(CATALOG_PATH.read_text(encoding="utf-8"))

    def test_repository_taxonomy_and_catalog_are_structurally_valid(self):
        self.assertEqual([], validate_taxonomy(self.taxonomy, self.catalog))

    def test_duplicate_category_id_fails_closed(self):
        taxonomy = copy.deepcopy(self.taxonomy)
        taxonomy["categories"].append(copy.deepcopy(taxonomy["categories"][0]))
        errors = validate_taxonomy(taxonomy)
        self.assertTrue(any("duplicate categories id" in error for error in errors))

    def test_dangling_family_category_fails_closed(self):
        taxonomy = copy.deepcopy(self.taxonomy)
        taxonomy["families"][0]["category_id"] = "CAT-DOES-NOT-EXIST"
        errors = validate_taxonomy(taxonomy)
        self.assertTrue(any("unknown category_id" in error for error in errors))

    def test_mapped_record_without_source_evidence_fails_closed(self):
        taxonomy = copy.deepcopy(self.taxonomy)
        catalog = {
            "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
            "components": [{
                "component_id": "COMP-TEST",
                "classification_status": "MAPPED",
                "category_id": taxonomy["categories"][0]["id"],
                "family_id": taxonomy["families"][0]["id"],
                "class_id": taxonomy["classes"][0]["id"],
            }],
        }
        errors = validate_taxonomy(taxonomy, catalog)
        self.assertTrue(any("MAPPED record requires source_evidence" in error for error in errors))

    def test_unknown_taxonomy_reference_fails_closed(self):
        taxonomy = copy.deepcopy(self.taxonomy)
        catalog = {
            "schema_id": "ATC-COMPONENT-FUNCTION-CATALOG-001",
            "components": [{
                "component_id": "COMP-TEST",
                "classification_status": "UNRESOLVED",
                "category_id": "CAT-UNKNOWN",
            }],
        }
        errors = validate_taxonomy(taxonomy, catalog)
        self.assertTrue(any("unknown taxonomy id" in error for error in errors))

    def test_catalog_primary_layers_match_master_architecture(self):
        errors = validate_taxonomy(self.taxonomy, copy.deepcopy(self.catalog))
        self.assertEqual([], errors)

    def test_primary_layer_drift_fails_closed(self):
        catalog = copy.deepcopy(self.catalog)
        next(c for c in catalog['components'] if c['id'] == 'COMP-VM')['layer'] = 'L2'
        errors = validate_taxonomy(self.taxonomy, catalog)
        self.assertTrue(any('COMP-VM: primary layer' in error for error in errors))

    def test_source_binding_counters_must_match_records(self):
        catalog = copy.deepcopy(self.catalog)
        catalog['scope']['function_source_bindings'] -= 1
        errors = validate_taxonomy(self.taxonomy, catalog)
        self.assertTrue(any('scope.function_source_bindings is stale' in error for error in errors))

    def test_demo_fixture_must_be_excluded_from_production_readiness(self):
        catalog = copy.deepcopy(self.catalog)
        next(c for c in catalog['components'] if c['id'] == 'COMP-DEMO').pop('catalog_scope', None)
        errors = validate_taxonomy(self.taxonomy, catalog)
        self.assertTrue(any('COMP-DEMO must be explicitly excluded' in error for error in errors))

if __name__ == "__main__":
    unittest.main()

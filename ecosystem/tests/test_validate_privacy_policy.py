import unittest

from validate_privacy_policy import validate


def requirement(**overrides):
    value = {
        "id": "ATC-PRIV-001",
        "title": "Example requirement",
        "statement": "The controller MUST document the processing purpose.",
        "category": "purpose_limitation",
        "normative_level": "MUST",
        "mapping_status": "UNRESOLVED",
        "implementation_status": "NOT_ASSESSED",
        "catalog_refs": [],
        "evidence_refs": [],
    }
    value.update(overrides)
    return value


class PrivacyPolicyValidatorTests(unittest.TestCase):
    def test_valid_draft_is_structurally_valid_only(self):
        doc = {
            "schema_id": "ATC-PRIVACY-POLICY-001",
            "version": "1.0.0",
            "status": "DRAFT",
            "declared_requirement_count": 1,
            "requirements": [requirement()],
        }
        self.assertEqual(validate(doc), [])

    def test_duplicate_ids_fail(self):
        doc = {
            "schema_id": "ATC-PRIVACY-POLICY-001",
            "version": "1.0.0",
            "status": "DRAFT",
            "requirements": [requirement(), requirement()],
        }
        self.assertTrue(any("duplicate requirement id" in error for error in validate(doc)))

    def test_verified_without_evidence_fails_closed(self):
        doc = {
            "schema_id": "ATC-PRIVACY-POLICY-001",
            "version": "1.0.0",
            "status": "DRAFT",
            "requirements": [requirement(
                mapping_status="MAPPED",
                catalog_refs=["ATC-X11-FUNC-EXAMPLE"],
                implementation_status="VERIFIED",
            )],
        }
        errors = validate(doc)
        self.assertTrue(any("requires evidence_refs" in error for error in errors))

    def test_mapped_without_target_fails(self):
        doc = {
            "schema_id": "ATC-PRIVACY-POLICY-001",
            "version": "1.0.0",
            "status": "DRAFT",
            "requirements": [requirement(mapping_status="MAPPED")],
        }
        self.assertTrue(any("MAPPED requires" in error for error in validate(doc)))


if __name__ == "__main__":
    unittest.main()

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def test_dependency_policy_schema():
    p = json.loads((ROOT / "security/dependency-policy.json").read_text())
    assert p["schema"] == "ATC-DEP-POLICY-1"
    assert "critical" in p["block_severities"]
    assert p["deny_unpinned"] is True


def test_dependency_policy_validation():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    valid = {
        "schema": "ATC-DEP-POLICY-1",
        "block_severities": ["critical", "high"],
        "deny_unpinned": True,
    }
    assert dg.validate_policy(valid) == []
    assert dg.validate_policy(
        {"schema": "wrong", "block_severities": ["critical"], "deny_unpinned": True}
    )
    assert dg.validate_policy(
        {"schema": "ATC-DEP-POLICY-1", "block_severities": [], "deny_unpinned": True}
    )
    assert dg.validate_policy(
        {"schema": "ATC-DEP-POLICY-1", "block_severities": ["urgent"], "deny_unpinned": True}
    )
    assert dg.validate_policy(
        {"schema": "ATC-DEP-POLICY-1", "block_severities": ["critical"], "deny_unpinned": "true"}
    )
    assert dg.validate_policy(
        {
            "schema": "ATC-DEP-POLICY-1",
            "block_severities": ["critical", "CRITICAL"],
            "deny_unpinned": True,
        }
    )


def test_dependency_advisory_schema():
    p = json.loads((ROOT / "security/advisories.json").read_text())
    assert p["schema"] == "ATC-DEP-ADVISORY-1"


def test_advisory_payload_requires_osv_provenance():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    errors = dg.validate_advisory_payload({"schema": "ATC-DEP-ADVISORY-1", "advisories": []})
    assert any("provenance" in x for x in errors)


def test_advisory_payload_accepts_valid_provenance():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    payload = {
        "schema": "ATC-DEP-ADVISORY-1",
        "source": {"format": "OSV", "input_sha256": "0" * 64},
        "advisories": [],
    }
    assert dg.validate_advisory_payload(payload) == []


def test_advisory_range_structure_is_strict():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    payload = {
        "schema": "ATC-DEP-ADVISORY-1",
        "source": {"format": "OSV", "input_sha256": "0" * 64},
        "advisories": [
            {
                "id": "GHSA-test",
                "severity": "high",
                "affected": [
                    {
                        "ecosystem": "npm",
                        "package": "foo",
                        "versions": [],
                        "ranges": [{"type": "SEMVER"}],
                    }
                ],
            }
        ],
    }
    assert any("invalid type/events" in x for x in dg.validate_advisory_payload(payload))


def test_osv_snapshot_validation_rejects_invalid_range_type_and_event():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import osv_snapshot

    base = {
        "schema": "ATC-DEP-ADVISORY-1",
        "source": {"format": "OSV", "input_sha256": "0" * 64},
        "advisories": [
            {
                "id": "GHSA-test",
                "severity": "high",
                "affected": [
                    {
                        "ecosystem": "npm",
                        "package": "foo",
                        "versions": [],
                        "ranges": [{"type": "", "events": [{"introduced": "0"}]}],
                    }
                ],
            }
        ],
    }
    errors = osv_snapshot.validate(base)
    assert any("invalid type/events" in x for x in errors)
    base["advisories"][0]["affected"][0]["ranges"][0] = {"type": "SEMVER", "events": [{}]}
    errors = osv_snapshot.validate(base)
    assert any("invalid event" in x for x in errors)


def test_osv_fetch_cvss_numeric_severity_mapping():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import osv_fetch

    assert osv_fetch.cvss_severity(9.8) == "critical"
    assert osv_fetch.cvss_severity(8.1) == "high"
    assert osv_fetch.cvss_severity(5.0) == "medium"
    assert osv_fetch.cvss_severity(2.0) == "low"
    assert osv_fetch.cvss_severity(11.0) is None


def test_osv_fetch_preserves_explicit_osv_severity():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import osv_fetch

    assert osv_fetch.normalize_severity({"database_specific": {"severity": "HIGH"}}) == "high"
    assert (
        osv_fetch.normalize_severity({"database_specific": {"severity": "critical"}}) == "critical"
    )


def test_base_head_diff_detects_add_remove_and_upgrade():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    base = [
        {"ecosystem": "cargo", "name": "foo", "version": "1.0.0"},
        {"ecosystem": "cargo", "name": "bar", "version": "1.0.0"},
    ]
    head = [
        {"ecosystem": "cargo", "name": "foo", "version": "1.1.0"},
        {"ecosystem": "cargo", "name": "baz", "version": "2.0.0"},
    ]
    added, removed, changed = dg.compare_nodes(base, head)
    assert [x["name"] for x in added] == ["baz"]
    assert [x["name"] for x in removed] == ["bar"]
    assert (
        len(changed) == 1
        and changed[0]["from"]["version"] == "1.0.0"
        and changed[0]["to"]["version"] == "1.1.0"
    )


def test_osv_semver_range_is_conservative():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    events = [{"introduced": "0"}, {"fixed": "1.2.0"}]
    assert dg.range_match("1.1.9", events, "cargo") == (True, "range")
    assert dg.range_match("1.2.0", events, "cargo") == (False, "range")
    assert dg.range_match("not-semver", events, "cargo")[1] == "unsupported"


def test_osv_multiple_ranges_are_not_flattened():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    ranges = [
        {"type": "SEMVER", "events": [{"introduced": "0"}, {"fixed": "1.0.0"}]},
        {"type": "SEMVER", "events": [{"introduced": "2.0.0"}, {"fixed": "3.0.0"}]},
    ]
    assert dg.range_match("1.5.0", ranges, "cargo") == (False, "range")
    assert dg.range_match("2.5.0", ranges, "cargo") == (True, "range")
    assert dg.range_match("3.0.0", ranges, "cargo") == (False, "range")


def test_osv_python_range():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    events = [{"introduced": "1.0.0"}, {"fixed": "2.0.0"}]
    assert dg.range_match("1.9.9", events, "python") == (True, "range")
    assert dg.range_match("2.0.0", events, "python") == (False, "range")


def test_osv_advisory_exact_match():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    ad = {
        "id": "GHSA-test",
        "affected": [{"ecosystem": "npm", "package": "foo", "versions": ["1.0.0"], "ranges": []}],
    }
    assert dg.advisory_matches({"ecosystem": "npm", "name": "foo", "version": "1.0.0"}, ad) == (
        True,
        "exact",
    )


def test_osv_snapshot_normalization():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import osv_snapshot

    raw = [
        {
            "id": "GHSA-test",
            "aliases": ["CVE-2099-0001"],
            "affected": [
                {
                    "package": {"ecosystem": "npm", "name": "foo"},
                    "versions": ["1.0.0"],
                    "ranges": [
                        {"type": "SEMVER", "events": [{"introduced": "0"}, {"fixed": "2.0.0"}]}
                    ],
                }
            ],
        }
    ]
    out = osv_snapshot.normalize(raw)
    assert out[0]["affected"][0]["ranges"][0]["events"][0]["introduced"] == "0"


def test_spdx_sbom_module_imports():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_sbom

    assert callable(dependency_sbom.main)


def test_osv_snapshot_rejects_invalid_raw_response_provenance():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import osv_snapshot

    payload = {
        "schema": "ATC-DEP-ADVISORY-1",
        "source": {"format": "OSV", "input_sha256": "a" * 64, "raw_response_sha256": "bad"},
        "advisories": [],
    }
    assert any("raw OSV response provenance" in e for e in osv_snapshot.validate(payload))


def test_osv_fetch_query_provenance_is_distinct():
    import hashlib
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import osv_fetch

    body = b'{"queries":[]}'
    query_sha = hashlib.sha256(body).hexdigest()
    raw = {"results": []}
    assert osv_fetch.validate_raw_response(raw, 0) == []
    assert (
        query_sha
        != hashlib.sha256(
            (json.dumps(raw, sort_keys=True, separators=(",", ":")) + "\\n").encode()
        ).hexdigest()
    )


def test_osv_fetch_validates_response_shape():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import osv_fetch

    assert osv_fetch.validate_raw_response({}, 1)
    assert osv_fetch.validate_raw_response({"results": []}, 1)
    assert osv_fetch.validate_raw_response({"results": [{"vulns": {}}]}, 1)
    assert osv_fetch.validate_raw_response({"results": [{"vulns": []}]}, 1) == []


def test_osv_fetch_rejects_non_versioned_empty_query_set():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import osv_fetch

    assert osv_fetch.validate_raw_response({"results": []}, 0) == []


def test_osv_query_provenance_uses_exact_persisted_bytes():
    import hashlib

    body = b'{"queries":[]}'
    persisted = body
    assert hashlib.sha256(persisted).hexdigest() == hashlib.sha256(body).hexdigest()


def test_base_graph_classification_pass():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    assert dg.classify_base_graph_result({"status": "PASS", "errors": []}) == "PASS"


def test_base_graph_classification_rejects_pass_with_errors():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    result = {
        "status": "PASS",
        "errors": ["unexpected stale error"],
    }
    assert dg.classify_base_graph_result(result) == "UNEXPECTED_FAILURE"


def test_base_graph_classification_expected_unpinned_blocked():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    result = {
        "status": "BLOCKED",
        "errors": [
            "requirements.txt:4: unpinned/unsupported requirement: pyyaml>=6.0,<7.0",
            "requirements.txt:5: unpinned/unsupported requirement: jsonschema>=4.0,<5.0",
        ],
    }
    assert dg.classify_base_graph_result(result) == "EXPECTED_BLOCKED"


def test_base_graph_classification_rejects_mixed_errors():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    result = {
        "status": "BLOCKED",
        "errors": [
            "requirements.txt:4: unpinned/unsupported requirement: pyyaml>=6.0,<7.0",
            "requirements-dev.txt:2: Parser error: unexpected token",
        ],
    }
    assert dg.classify_base_graph_result(result) == "UNEXPECTED_FAILURE"


def test_base_graph_classification_rejects_malformed_blocked_result():
    import sys

    sys.path.insert(0, str(ROOT / "tools"))
    import dependency_guard as dg

    assert (
        dg.classify_base_graph_result({"status": "BLOCKED", "errors": []}) == "UNEXPECTED_FAILURE"
    )

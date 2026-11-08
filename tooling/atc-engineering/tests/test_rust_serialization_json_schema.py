#!/usr/bin/env python3
"""Validate real Rust serializer output against the committed JSON Schemas."""
import json
import subprocess
import sys
from pathlib import Path

from jsonschema import Draft202012Validator, FormatChecker

ROOT = Path(__file__).resolve().parents[1]


def validate(crate: str, schema_path: str, example: str) -> None:
    completed = subprocess.run(
        ["cargo", "run", "--locked", "--quiet", "-p", crate, "--example", example],
        cwd=ROOT,
        check=True,
        text=True,
        capture_output=True,
    )
    instance = json.loads(completed.stdout)
    schema = json.loads((ROOT / schema_path).read_text(encoding="utf-8"))
    errors = sorted(
        Draft202012Validator(schema, format_checker=FormatChecker()).iter_errors(instance),
        key=lambda error: list(map(str, error.absolute_path)),
    )
    if errors:
        for error in errors:
            print(f"{crate} {list(error.absolute_path)}: {error.message}", file=sys.stderr)
        raise SystemExit(1)
    print(f"PASS {crate}: generated JSON conforms to {schema_path}")


if __name__ == "__main__":
    validate("atc-evidence", "schemas/evidence/ATC-EVD-001.json", "evidence_schema_fixture")
    validate("atc-findings", "schemas/findings/ATC-FND-001.json", "finding_schema_fixture")

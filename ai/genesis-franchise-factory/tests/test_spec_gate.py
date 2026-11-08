import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def test_spec_gate_passes():
    r = subprocess.run(
        [sys.executable, str(ROOT / "tools" / "validate_specs.py")],
        capture_output=True,
        text=True,
        check=False,
    )
    assert r.returncode == 0, f"Spec-Gate FAIL:\n{r.stdout}\n{r.stderr}"
    assert "RESULT: SPEC-GATE PASS" in r.stdout
    assert "24 AD-Specs" in r.stdout

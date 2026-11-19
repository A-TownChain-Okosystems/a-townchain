#!/usr/bin/env python3
"""Fail-closed gate for registry-generated state.

The registry is the SSOT. Generated views must carry the exact Git blob SHA of
registry/standards.yaml. A changed registry without regenerated views fails CI.
"""

import argparse
import re
import subprocess
from pathlib import Path

MARKER = re.compile(r"ATC-REGISTRY-BLOB-SHA256:\s*([0-9a-f]{40})")


def git_blob_sha(path: str) -> str:
    # Git's object ID is the immutable repository revision used by the gate.
    out = subprocess.check_output(["git", "rev-parse", f"HEAD:{path}"], text=True).strip()
    if not re.fullmatch(r"[0-9a-f]{40}", out):
        raise RuntimeError(f"invalid git object id for {path}: {out!r}")
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--registry", default="registry/standards.yaml")
    ap.add_argument("--views", nargs="+", default=["registry/GENERATED_STATE.md"])
    args = ap.parse_args()

    expected = git_blob_sha(args.registry)
    failures: list[str] = []

    for raw in args.views:
        path = Path(raw)
        if not path.is_file():
            failures.append(f"missing generated view: {path}")
            continue
        text = path.read_text(encoding="utf-8")
        match = MARKER.search(text)
        if not match:
            failures.append(f"{path}: missing ATC-REGISTRY-BLOB-SHA256 marker")
            continue
        if match.group(1) != expected:
            failures.append(
                f"{path}: registry revision {match.group(1)} != current {expected}; regenerate"
            )

    if failures:
        for item in failures:
            print(f"FAIL: {item}")
        return 1

    print(f"PASS: generated views are pinned to registry revision {expected}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

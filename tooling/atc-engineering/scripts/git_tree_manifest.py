#!/usr/bin/env python3
"""Emit a complete, NUL-safe manifest of a pinned Git tree."""
from __future__ import annotations

import argparse
import json
import re
import subprocess
from pathlib import Path

SHA40 = re.compile(r"^[0-9a-f]{40}$")


def build_manifest(root: Path, commit: str) -> dict:
    if not SHA40.fullmatch(commit):
        raise ValueError("commit must be a full 40-character lowercase SHA")
    subprocess.run(["git", "-C", str(root), "cat-file", "-e", f"{commit}^{{commit}}"], check=True)
    raw = subprocess.check_output(
        ["git", "-C", str(root), "ls-tree", "-r", "-t", "-z", "--full-tree", commit]
    )
    entries = []
    for record in raw.split(b"\0"):
        if not record:
            continue
        try:
            metadata, path_bytes = record.split(b"\t", 1)
            mode, object_type, sha = metadata.decode("ascii").split(" ", 2)
            path = path_bytes.decode("utf-8", errors="surrogateescape")
        except (ValueError, UnicodeDecodeError) as exc:
            raise ValueError(f"cannot parse git ls-tree record: {record!r}") from exc
        entries.append({"path": path, "mode": mode, "type": object_type, "sha": sha})
    entries.sort(key=lambda item: item["path"])
    return {
        "schema_version": "1.0.0",
        "repository_root": str(root.resolve()),
        "commit_sha": commit,
        "entry_count": len(entries),
        "entries": entries,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".", help="Git repository/worktree containing the pinned commit")
    parser.add_argument("--commit", required=True, help="exact full commit SHA")
    parser.add_argument("--output", required=True, help="output JSON path")
    args = parser.parse_args()
    result = build_manifest(Path(args.root), args.commit)
    target = Path(args.output)
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(result, ensure_ascii=True, sort_keys=True, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"commit_sha": result["commit_sha"], "entries": result["entry_count"], "output": str(target)}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

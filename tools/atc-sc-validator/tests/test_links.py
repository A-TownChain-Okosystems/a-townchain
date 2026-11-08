# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Regressionstests fuer die Markdown-Link-Pruefung."""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path

TOOL = Path(__file__).resolve().parents[2] / "linkcheck.py"


def load_tool():
    spec = importlib.util.spec_from_file_location("linkcheck", TOOL)
    mod = importlib.util.module_from_spec(spec)
    sys.modules["linkcheck"] = mod
    spec.loader.exec_module(mod)
    return mod


def write(tmp_path: Path, rel: str, text: str) -> Path:
    p = tmp_path / rel
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(text, encoding="utf-8")
    return p


def test_existing_target_is_not_reported(tmp_path):
    tool = load_tool()
    write(tmp_path, "a.md", "[b](b.md)")
    write(tmp_path, "b.md", "inhalt")
    assert tool.broken_links(tmp_path) == []


def test_missing_target_is_reported(tmp_path):
    tool = load_tool()
    write(tmp_path, "a.md", "[weg](nirgends.md)")
    findings = tool.broken_links(tmp_path)
    assert len(findings) == 1 and findings[0]["target"] == "nirgends.md"


def test_external_links_and_anchors_are_ignored(tmp_path):
    tool = load_tool()
    write(tmp_path, "a.md", "[gh](https://example.org/x) [hier](#abschnitt) [mail](mailto:a@b.de)")
    assert tool.broken_links(tmp_path) == []


def test_inline_code_is_not_a_link(tmp_path):
    tool = load_tool()
    write(tmp_path, "a.md", "Adresse: `H[pub_key_data](:32)` und `[x](fehlt.md)`")
    assert tool.broken_links(tmp_path) == []


def test_archive_trees_are_skipped(tmp_path):
    tool = load_tool()
    write(tmp_path, "docs/archive/alt.md", "[weg](nirgends.md)")
    write(tmp_path, "docs/aktiv.md", "[ok](archive/alt.md)")
    assert tool.broken_links(tmp_path) == []


def test_anchor_suffix_is_stripped_before_checking(tmp_path):
    tool = load_tool()
    write(tmp_path, "a.md", "[b](b.md#teil)")
    write(tmp_path, "b.md", "## Teil")
    assert tool.broken_links(tmp_path) == []


def test_repository_is_clean():
    tool = load_tool()
    root = TOOL.parents[1]
    findings = tool.broken_links(root)
    assert not findings, "tote relative Links: " + ", ".join(f["file"] for f in findings[:5])

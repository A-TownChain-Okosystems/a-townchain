import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))

from consensus_boundary_check import scan  # noqa: E402


class ConsensusBoundaryScanTests(unittest.TestCase):
    def test_detects_atclang_source_under_consensus_path(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / "modules" / "atc-blockchain" / "consensus"
            source.mkdir(parents=True)
            (source / "consensus.atc").write_text("contract ShivaConsensus {}\n")
            self.assertEqual(
                scan(tmp),
                ["modules/atc-blockchain/consensus/consensus.atc"],
            )

    def test_detects_rust_source_under_poh_path(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / "legacy" / "poh"
            source.mkdir(parents=True)
            (source / "poh.rs").write_text("pub fn tick() {}\n")
            self.assertEqual(scan(tmp), ["legacy/poh/poh.rs"])

    def test_ignores_non_code_files(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / "consensus"
            source.mkdir()
            (source / "design.md").write_text("# Consensus\n")
            self.assertEqual(scan(tmp), [])

    def test_ignores_archived_consensus_sources(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / "archive" / "consensus"
            source.mkdir(parents=True)
            (source / "consensus.atc").write_text("contract Legacy {}\n")
            self.assertEqual(scan(tmp), [])

    def test_results_are_sorted(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp) / "consensus"
            source.mkdir()
            (source / "z.rs").write_text("// source\n")
            (source / "a.atc").write_text("// source\n")
            self.assertEqual(
                scan(tmp),
                ["consensus/a.atc", "consensus/z.rs"],
            )


if __name__ == "__main__":
    unittest.main()

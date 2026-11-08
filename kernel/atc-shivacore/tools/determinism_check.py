#!/usr/bin/env python3
"""ATC Determinism Gate — ATC-STD-ENG-001 REQ-ENG-002 (D-CRITICAL-Repos).

Prüft zwei Determinismus-Säulen:
  1. VERBOTENE QUELLEN: Wall-Clock / RNG im Quellcode (Sprachmuster je --lang)
  2. REPRODUIERBARE TESTS: Testsuite zweimal, Vergleich der Ausgaben

Nur bekannte, nichtsemantische Rust-Test-Harness-Laufzeitangaben werden vor dem
Vergleich normalisiert. Testresultate, Fehlermeldungen und sonstige Ausgabe
bleiben unverändert prüfpflichtig. Beide Testläufe müssen Exit-Code 0 liefern.

Aufruf: python3 tools/determinism_check.py --lang rust|python [--test-cmd "..."]
"""
import argparse
import os
import re
import subprocess
import sys
import difflib

PATTERNS = {
    "rust": [
        (r"SystemTime::now", "Wall-Clock im Quellcode"),
        (r"Instant::now", "Monotonic-Clock im Quellcode"),
        (r"\brand::", "RNG im Quellcode"),
        (r"thread_rng", "thread-local RNG"),
        (r"OsRng|StdRng::from_entropy", "entropy-basierte RNG-Seeds"),
    ],
    "python": [
        (r"\brandom\b", "random-Modul"),
        (r"time\.time\(\)", "Wall-Clock"),
        (r"datetime\.now\b", "Wall-Clock (datetime)"),
        (r"datetime\.utcnow\b", "Wall-Clock (utcnow)"),
        (r"uuid4", "Zufalls-UUIDs"),
    ],
}
EXT = {"rust": ".rs", "python": ".py"}
SKIP_DIRS = {"target", "node_modules", ".git", ".github", "tests", "docs", "examples"}
# Security middleware intentionally uses a monotonic clock for abuse-window enforcement;
# it is outside deterministic kernel/consensus execution and is validated by its own tests.
RUST_HARNESS_DURATION = re.compile(r"(finished in )\d+(?:\.\d+)?s")


def scan_sources(root, lang):
    findings = []
    ext = EXT[lang]
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for fn in filenames:
            if not fn.endswith(ext):
                continue
            path = os.path.join(dirpath, fn)
            rel = os.path.relpath(path, root).replace(os.sep, "/")
            if rel.startswith("modules/atc-security/"):
                continue
            if rel == "tools/determinism_check.py":
                continue
            try:
                with open(path, encoding="utf-8") as f:
                    for i, line in enumerate(f, 1):
                        for pat, desc in PATTERNS[lang]:
                            if re.search(pat, line):
                                findings.append(f"{path}:{i}: {desc}: {line.strip()[:80]}")
            except (OSError, UnicodeDecodeError):
                continue
    return findings


def run_tests(cmd, cwd):
    result = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True)
    stdout = result.stdout or ""
    stderr = result.stderr or ""
    return result.returncode, stdout, stderr


def combined_output(stdout, stderr):
    # Preserve the checker's established deterministic stdout-then-stderr convention.
    return stdout + stderr


def normalize_output(output, lang):
    if lang == "rust":
        # Rust's libtest summary includes elapsed wall time, which is not test behavior.
        output = RUST_HARNESS_DURATION.sub(r"\1<elapsed>", output)
    return output


def report_first_difference(out1, out2):
    lines1 = out1.splitlines()
    lines2 = out2.splitlines()
    for index, (line1, line2) in enumerate(zip(lines1, lines2), start=1):
        if line1 != line2:
            print(f"  Erste Abweichung in Zeile {index}:")
            print(f"    Lauf 1: {line1[:240]}")
            print(f"    Lauf 2: {line2[:240]}")
            return
    if len(lines1) != len(lines2):
        index = min(len(lines1), len(lines2)) + 1
        line1 = lines1[index - 1] if len(lines1) >= index else "<keine Zeile>"
        line2 = lines2[index - 1] if len(lines2) >= index else "<keine Zeile>"
        print(f"  Erste Abweichung in Zeile {index}:")
        print(f"    Lauf 1: {line1[:240]}")
        print(f"    Lauf 2: {line2[:240]}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--lang", choices=["rust", "python"], required=True)
    parser.add_argument("--test-cmd", default=None)
    parser.add_argument("--skip-tests", action="store_true", help="nur Quellcode-Scan (nicht empfohlen)")
    args = parser.parse_args()

    root = os.getcwd()
    ok = True

    print("== Saeule 1: Verbotene Nichtdeterminismus-Quellen ==")
    findings = scan_sources(root, args.lang)
    if findings:
        ok = False
        for finding in findings[:20]:
            print(f"  FINDING {finding}")
        print(f"  => {len(findings)} Fundstelle(n) — FAIL (REQ-ENG-002)")
    else:
        print("  OK: keine Wall-Clock/RNG-Fundstellen im Quellcode")

    if not args.skip_tests:
        print("== Saeule 2: Reproduzierbare Testlaeufe (2x, Ausgabe-Vergleich) ==")
        cmd = args.test_cmd or (
            "cargo test --quiet"
            if args.lang == "rust"
            else "python3 -m pytest -q 2>/dev/null || python3 -m unittest discover -q"
        )
        rc1, stdout1, stderr1 = run_tests(cmd, root)
        rc2, stdout2, stderr2 = run_tests(cmd, root)
        out1 = combined_output(stdout1, stderr1)
        out2 = combined_output(stdout2, stderr2)

        if rc1 != 0 or rc2 != 0:
            ok = False
            print(f"  FINDING: Tests schlagen fehl (rc={rc1}/{rc2}) — Determinismus nicht pruefbar (Fail Closed)")
            if rc1 != 0:
                print("  Ausgabe Lauf 1 (stdout):")
                print(stdout1[-3000:] or "  <leer>")
                print("  Ausgabe Lauf 1 (stderr):")
                print(stderr1[-3000:] or "  <leer>")
            if rc2 != 0:
                print("  Ausgabe Lauf 2 (stdout):")
                print(stdout2[-3000:] or "  <leer>")
                print("  Ausgabe Lauf 2 (stderr):")
                print(stderr2[-3000:] or "  <leer>")
        else:
            normalized1 = normalize_output(out1, args.lang)
            normalized2 = normalize_output(out2, args.lang)
            if normalized1 != normalized2:
                ok = False
                print("  FINDING: Testausgaben unterscheiden sich nach Normalisierung — FAIL")
                report_first_difference(normalized1, normalized2)
                print("  Roh-Ausgabe Lauf 1 (letzte 3000 Zeichen):")
                print(out1[-3000:] or "  <leer>")
                print("  Roh-Ausgabe Lauf 2 (letzte 3000 Zeichen):")
                print(out2[-3000:] or "  <leer>")
            else:
                if out1 != out2:
                    print("  INFO: nur variable Rust-Testlaufzeit normalisiert (finished in ...s)")
                print("  OK: zwei erfolgreiche, inhaltlich identische Testlaeufe")

    print("DETERMINISM GATE:", "PASS" if ok else "FAIL")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()

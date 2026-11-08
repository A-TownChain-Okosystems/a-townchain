#!/usr/bin/env python3
"""ATC Determinism Gate v1.1.0 — ATC-STD-ENG-001 REQ-ENG-002 (D-CRITICAL-Repos).

SSOT: atc-standards/tools/determinism_check.py (W2-MECH-01, SCR-0131).
Verbraucher-Repos ziehen je CI-Lauf frisch vom atc-standards main
(Muster: atc_repo_audit.py, SCR-0115). Die bis v1.0.0 verteilten 12+ lokalen
Kopien werden nach Rollout entfernt — eine Quelle, keine Varianten.

Prueft zwei Determinismus-Saeulen (fail-closed, REQ-ENG-012):
  1. VERBOTENE QUELLEN: Wall-Clock / RNG im Quellcode (Sprachmuster je --lang)
  2. REPRODUIERBARE TESTS: Testsuite zweimal, Byte-Vergleich der Ausgaben

Aenderungen v1.0.0 -> v1.1.0 (Work-Order W2-MECH-01):
  1. SELF-EXCLUSION: Der eigene Datei-Pfad und das eigene tools/-Verzeichnis
     werden nie gescannt. v1.0.0 trug den Skip-Eintrag im dirnames-Filter,
     matchete aber einen Dateinamen — er griff nie; der Scanner fand
     seine eigenen Regex-Definitionen (Klasse: Checker-Selbsttreffer).
  2. STRING-/DOCSTRING-AUSSCHLUSS: Nur echte Aufrufe zaehlen. Python via
     tokenize (STRING/COMMENT-Tokens entfernt, Docstrings sind STRING),
     Rust via Zustandsmaschine (//, ///, //!, /* */ und String-Literale).
     Bei nicht parsbareer Datei wird konservativ die Rohzeile gescannt
     (False Positive vor False Negative — Fail-closed).
  3. Test C verpflichtet: Regressionstests A/B/C in tools/tests/ — ein Fix,
     der Treffer nur zum Verstummen bringt, ist kein Nachweis der Korrektheit.

Aufruf: python3 determinism_check.py --lang rust|python [--test-cmd "..."] [--skip-tests]
"""

import argparse
import os
import re
import subprocess
import sys

VERSION = "1.1.0"

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

# W2-MECH-01 Kriterium 2: eigener tools/-Baum wird nicht gescannt
# (CI-Hilfswerkzeuge; True-Positive-Nachweis bleibt ueber Test C verpflichtend).
SKIP_DIRS = {
    "target",
    "node_modules",
    ".git",
    ".github",
    "tests",
    "docs",
    "examples",
    "tools",
}
SELF_PATH = os.path.realpath(__file__)


def _blank_python_strings(lines):
    """Ersetzt STRING/COMMENT/Docstring-Bereiche durch Leerzeichen (tokenize).

    Zeilenstruktur bleibt erhalten -> Zeilennummern der Findings stimmen.
    Bei Parse-Fehler: Originalzeilen unveraendert zurueck (konservativ,
    False Positive vor False Negative).
    """
    import tokenize

    arr = [list(ln) for ln in lines]
    try:
        with open(_blank_python_strings.path, "rb") as f:
            for tok in tokenize.tokenize(f.readline):
                t = tok.type
                if t not in (tokenize.STRING, tokenize.COMMENT):
                    # Python >= 3.12: F-String-Teiltokens ebenfalls rausnehmen
                    for name in ("FSTRING_START", "FSTRING_MIDDLE", "FSTRING_END"):
                        if t == getattr(tokenize, name, -1):
                            break
                    else:
                        continue
                (r1, c1), (r2, c2) = tok.start, tok.end
                for r in range(r1, r2 + 1):
                    if not (1 <= r <= len(arr)):
                        continue
                    row = arr[r - 1]
                    cs = c1 if r == r1 else 0
                    ce = c2 if r == r2 else len(row)
                    for c in range(cs, min(ce, len(row))):
                        row[c] = " "
    except (SyntaxError, UnicodeDecodeError, tokenize.TokenError, IndentationError):
        return lines
    return ["".join(row) for row in arr]


def _strip_rust_line(line, state):
    """Zustandsmaschine: entfernt //, ///, //! und /* */ sowie String-Inhalte.

    'String-Inhalte' heisst: Anfuehrungszeichen bleiben als Leerzeichen
    erhalten, der Inhalt wird entfernt — ein Muster NUR in einem String-
    Literal erzeugt kein Finding, ein echter Aufruf bleibt lesbar.
    Lebenszeiten ('a) und Char-Literale werden nicht als Strings behandelt.
    """
    out = []
    i, n = 0, len(line)
    while i < n:
        if state.get("block"):
            j = line.find("*/", i)
            if j == -1:
                return "".join(out), state
            state["block"] = False
            i = j + 2
            continue
        if state.get("str"):
            while i < n:
                ch = line[i]
                if ch == "\\":
                    i += 2
                    continue
                if ch == '"':
                    state["str"] = False
                    i += 1
                    out.append(" ")
                    break
                i += 1
            continue
        if line.startswith("//", i):
            break  # Zeilen-/Doc-Kommentar: Rest faellt weg
        if line.startswith("/*", i):
            state["block"] = True
            i += 2
            continue
        if line[i] == '"':
            state["str"] = True
            i += 1
            continue
        out.append(line[i])
        i += 1
    return "".join(out), state


def scan_sources(root, lang):
    findings = []
    ext = EXT[lang]
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for fn in filenames:
            if not fn.endswith(ext):
                continue
            path = os.path.join(dirpath, fn)
            if os.path.realpath(path) == SELF_PATH:
                continue  # W2-MECH-01: nie den eigenen Code scannen
            try:
                with open(path, encoding="utf-8") as f:
                    text = f.read()
            except (OSError, UnicodeDecodeError):
                continue
            lines = text.splitlines()
            if lang == "python":
                _blank_python_strings.path = path
                scan_lines = _blank_python_strings(lines)
            else:
                state = {"block": False, "str": False}
                scan_lines = []
                for ln in lines:
                    stripped, state = _strip_rust_line(ln, state)
                    scan_lines.append(stripped)
            for i, scan_ln in enumerate(scan_lines, 1):
                for pat, desc in PATTERNS[lang]:
                    if re.search(pat, scan_ln):
                        findings.append(f"{path}:{i}: {desc}: {scan_ln.strip()[:80]}")
    return findings


def run_tests(cmd, cwd):
    r = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True)
    return (r.returncode, (r.stdout or "") + (r.stderr or ""))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--lang", choices=["rust", "python"], required=True)
    ap.add_argument("--test-cmd", default=None)
    ap.add_argument(
        "--skip-tests", action="store_true", help="nur Quellcode-Scan (nicht empfohlen)"
    )
    args = ap.parse_args()

    root = os.getcwd()
    ok = True

    print("== Saeule 1: Verbotene Nichtdeterminismus-Quellen ==")
    findings = scan_sources(root, args.lang)
    if findings:
        ok = False
        for f in findings[:20]:
            print(f"  FINDING {f}")
        print(f"  => {len(findings)} Fundstelle(n) — FAIL (REQ-ENG-002)")
    else:
        print("  OK: keine Wall-Clock/RNG-Fundstellen im Quellcode")

    if not args.skip_tests:
        print("== Saeule 2: Reproduzierbare Testlaeufe (2x, Byte-Vergleich) ==")
        cmd = args.test_cmd or (
            "cargo test --quiet"
            if args.lang == "rust"
            else "python3 -m pytest -q 2>/dev/null || python3 -m unittest discover -q"
        )
        rc1, out1 = run_tests(cmd, root)
        rc2, out2 = run_tests(cmd, root)
        if rc1 != 0 or rc2 != 0:
            ok = False
            print(
                f"  FINDING: Tests schlagen fehl (rc={rc1}/{rc2}) — Determinismus nicht pruefbar (Fail Closed)"
            )
        elif out1 != out2:
            ok = False
            print(
                "  FINDING: Testausgaben unterscheiden sich zwischen Lauf 1 und Lauf 2 — nichtdeterministisch!"
            )
        else:
            print("  OK: zwei identische Testlaeufe (Evidenz per Byte-Vergleich)")

    print("DETERMINISM GATE:", "PASS" if ok else "FAIL")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()

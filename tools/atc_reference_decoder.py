#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Unabhaengiger Referenzdekoder fuer SCE-ARTIFACT (Magic `ATCE`, Fassung `0x02`).

Reine Funktion nach Abschnitt 19.2a des Kandidaten
ATC-SMART-CONTRACT-EXECUTION-001. Kein Projektcode, keine Bindung an die Rust-VM,
keine gemeinsame Quelle: die Operandentabelle ist aus ATC-BC-002 Abschnitt 16
abgeschrieben, damit ein Differentialtest ueberhaupt zwei unabhaengige
Implementierungen vergleicht.

Dieser Dekoder ist **kein** Konformitaetsnachweis. Er ist eine von zwei
Implementierungen. Konform ist ein Dekoder erst, wenn zwei unabhaengige Dekoder auf
demselben Korpus dasselbe Paar `(Code, Offset)` liefern (REQ-SCE-084).

Aufruf:
    python3 tools/atc_reference_decoder.py <hex>
    python3 tools/atc_reference_decoder.py --corpus standards/atc/conformance/stage1-parser-corpus.json
    python3 tools/atc_reference_decoder.py --selftest
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from dataclasses import dataclass
from pathlib import Path

# --- Formatkonstanten (REQ-SCE-061, REQ-SCE-066, REQ-SCE-067) ------------------

MAGIC = b"ATCE"
CONTAINER_VERSION = 0x02
GENERATION_V1 = 0x01
GENERATION_V2 = 0x02
HEAD_LEN = 10
TAIL_LEN = 32
MIN_LEN = HEAD_LEN + TAIL_LEN
DOMAIN_ARTIFACT = b"ATC-ARTIFACT-V1\x00"

# Operandenlaengen je Opcode, ATC-BC-002 Abschnitt 16.
OPERANDS: dict[int, int] = {
    0x01: 8,  # Push(u64)
    0x02: 0,  # Add
    0x03: 0,  # Sub
    0x04: 0,  # Mul
    0x05: 0,  # Div
    0x06: 0,  # Dup
    0x07: 0,  # Swap
    0x08: 0,  # Eq
    0x09: 0,  # Lt
    0x0A: 4,  # Load(u32)
    0x0B: 4,  # Store(u32)
    0x0C: 0,  # Caller
    0x0D: 4,  # Jump(u32)
    0x0E: 4,  # JumpIfZero(u32)
    0x0F: 4,  # JumpIfNotZero(u32)
    0x10: 0,  # Halt
    0x11: 0,  # Assert (ab Generation 2)
    0x12: 4,  # Revert(u32) (ab Generation 2)
    0x13: 0,  # RevertIf (ab Generation 2)
}
V2_ONLY = frozenset({0x11, 0x12, 0x13})

CODE_NAMES = {
    "E-001": "TOO_SHORT",
    "E-002": "TRUNCATED_OPERAND",
    "E-003": "UNSUPPORTED_CONTAINER_VERSION",
    "E-004": "EMPTY_STREAM",
    "E-005": "COUNT_MISMATCH",
    "E-006": "INTEGRITY_MISMATCH",
    "E-007": "UNKNOWN_MAGIC",
    "E-008": "TRAILING_BYTES",
    "E-009": "UNKNOWN_GENERATION",
    "E-010": "UNKNOWN_OPCODE",
    "E-011": "GENERATION_MISMATCH",
    "E-012": "PROGRAM_TOO_LARGE",
}


@dataclass(frozen=True)
class Result:
    """Ergebnis eines Dekodierversuchs. Fehleridentitaet ist (Code, Offset)."""

    decision: str  # "accept" oder "reject"
    code: str | None
    offset: int | None
    instructions: int = 0
    stream_len: int = 0
    artifact_hash: str | None = None

    def as_dict(self) -> dict:
        return {
            "decision": self.decision,
            "code": self.code,
            "offset": self.offset,
            "instructions": self.instructions,
            "stream_len": self.stream_len,
        }


def _reject(code: str, offset: int) -> Result:
    if code not in CODE_NAMES:
        raise AssertionError(f"{code} ist nicht in der geschlossenen Codeliste")
    return Result("reject", code, offset)


def decode(data: bytes, max_stream_bytes: int | None = None) -> Result:
    """Reine Funktion der Bytes nach REQ-SCE-079 und REQ-SCE-086.

    `max_stream_bytes` ist ein optionaler Betreiberwert fuer Stufe 10 (E-012).
    Ohne Wert ist E-012 nicht erreichbar; die Grenzwerte sind offen (T-21 bis T-24).
    """
    total = len(data)

    # Stufe 1..4: Laenge, Magic, Fassung, Generation.
    if total < MIN_LEN:
        return _reject("E-001", total)
    if data[0:4] != MAGIC:
        return _reject("E-007", 0)
    if data[4] != CONTAINER_VERSION:
        return _reject("E-003", 4)
    generation = data[5]
    if generation not in (GENERATION_V1, GENERATION_V2):
        return _reject("E-009", 5)

    # REQ-SCE-085: die Stromgrenze folgt aus der Laenge, nie aus der Anzahl.
    tail_start = total - TAIL_LEN
    declared = int.from_bytes(data[6:10], "little")

    # Stufen 5..8: ein Durchlauf der Prozedur aus REQ-SCE-086.
    pos = HEAD_LEN
    seen = 0
    while seen < declared and pos < tail_start:
        opcode = data[pos]
        width = OPERANDS.get(opcode)
        if width is None:
            return _reject("E-010", pos)
        if opcode in V2_ONLY and generation != GENERATION_V2:
            return _reject("E-011", pos)
        if pos + 1 + width > tail_start:
            return _reject("E-002", tail_start)
        seen += 1
        pos += 1 + width

    if seen < declared:
        return _reject("E-005", tail_start)
    if pos < tail_start:
        return _reject("E-008", pos)
    if seen == 0:
        return _reject("E-004", HEAD_LEN)

    # Stufe 9: Tail ist exakt artifact_hash ueber Byte 0 bis tail_start-1.
    expected = hashlib.sha256(DOMAIN_ARTIFACT + data[:tail_start]).digest()
    if data[tail_start:] != expected:
        return _reject("E-006", tail_start)

    # Stufe 10: Groesstwert, sofern konfiguriert. Misst die Stromlaenge, nicht die
    # deklarierte Anzahl (REQ-SCE-089).
    if max_stream_bytes is not None and tail_start - HEAD_LEN > max_stream_bytes:
        return _reject("E-012", HEAD_LEN)

    return Result(
        "accept",
        None,
        None,
        instructions=seen,
        stream_len=tail_start - HEAD_LEN,
        artifact_hash=expected.hex(),
    )


def decode_hex(text: str, max_stream_bytes: int | None = None) -> Result:
    return decode(bytes.fromhex(text.strip()), max_stream_bytes=max_stream_bytes)


def selftest() -> int:
    """Drei Faelle aus Abschnitt 18.2, ohne Korpusdatei."""
    body = bytes.fromhex("4154434502010200000001010000000000000010")
    tail = hashlib.sha256(DOMAIN_ARTIFACT + body).digest()
    good = body + tail
    problems: list[str] = []

    result = decode(good)
    if result.decision != "accept" or result.instructions != 2:
        problems.append(f"gueltiger Container abgelehnt: {result}")
    if result.artifact_hash != tail.hex():
        problems.append("artifact_hash weicht vom Tail ab")

    if decode(good + b"\x00").code != "E-008":
        problems.append("angehaengtes Byte ergibt nicht E-008")
    if decode(good[:41]).code != "E-001":
        problems.append("41 Byte ergeben nicht E-001")
    if decode(good[:-1]).code != "E-005":
        problems.append("um ein Byte gekuerzter Container ergibt nicht E-005")

    for line in problems:
        print(f"FAIL: {line}")
    if problems:
        return 1
    print("PASS: Selftest (Annahme, E-008, E-001)")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("hex", nargs="?", help="Container als Hex")
    parser.add_argument("--corpus", help="Korpusdatei gegen den Dekoder laufen lassen")
    parser.add_argument("--selftest", action="store_true")
    args = parser.parse_args(argv)

    if args.selftest:
        return selftest()
    if args.corpus:
        corpus = json.loads(Path(args.corpus).read_text(encoding="utf-8"))
        deviations = 0
        for entry in corpus["eintraege"]:
            result = decode_hex(entry["eingabe_hex"])
            if (result.decision, result.code, result.offset) != (
                entry["entscheidung"],
                entry["code"],
                entry["offset"],
            ):
                deviations += 1
                print(
                    f"ABWEICHUNG {entry['id']}: erwartet "
                    f"{entry['entscheidung']}/{entry['code']}/{entry['offset']}, "
                    f"erhalten {result.decision}/{result.code}/{result.offset}"
                )
        print(f"Referenzdekoder: {len(corpus['eintraege'])} Eingaben, {deviations} Abweichungen")
        return 1 if deviations else 0
    if not args.hex:
        parser.error("Hex, --corpus oder --selftest erforderlich")

    result = decode_hex(args.hex)
    print(json.dumps(result.as_dict(), indent=2))
    return 0 if result.decision == "accept" else 1


if __name__ == "__main__":
    sys.exit(main())

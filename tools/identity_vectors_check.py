#!/usr/bin/env python3
"""Prueft die gemeinsamen Testvektoren fuer IdentityValue.

Der Pruefer ist kein Produktionsdekoder. Er prueft die Vektordatei gegen das
kanonische Layout aus ATC-SMART-CONTRACT-EXECUTION-001 (REQ-SCE-057 bis 059):
Kennzeichen in Byte 0..3, reservierte Nullbytes in 4..11, Adresse in 12..31.

Produktiver Code (Wallet, VM, Knoten) verwendet genau eine Implementierung,
nicht diesen Pruefer.

Aufruf:  python3 tools/identity_vectors_check.py [datei]
Exit:    0 = alle Vektoren stimmen, 1 = Abweichung
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

DEFAULT = Path("standards/atc/conformance/identity-value-vectors.json")

TOTAL_BYTES = 32
TAG_WALLET = 0x00000001
TAG_CONTRACT = 0x00000002
KNOWN_TAGS = {TAG_WALLET, TAG_CONTRACT}
INVALID_REASONS = {
    "TAG_ZERO",
    "TAG_UNKNOWN",
    "RESERVED_NOT_ZERO",
    "WORD_ZERO",
    "ADDRESS_ZERO",
}


def decode(word: bytes) -> tuple[str | None, str]:
    """Gibt (tag, fehlergrund) zurueck. tag ist None, wenn das Wort ungueltig ist."""
    if len(word) != TOTAL_BYTES:
        return None, "LENGTH"
    if word == bytes(TOTAL_BYTES):
        return None, "WORD_ZERO"
    tag = int.from_bytes(word[0:4], "big")
    reserved = word[4:12]
    address = word[12:32]
    if any(b != 0 for b in reserved):
        return None, "RESERVED_NOT_ZERO"
    if tag == 0:
        return None, "TAG_ZERO"
    if tag not in KNOWN_TAGS:
        return None, "TAG_UNKNOWN"
    if address == bytes(20):
        return None, "ADDRESS_ZERO"
    return ("wallet" if tag == TAG_WALLET else "contract"), ""


def main(argv: list[str]) -> int:
    path = Path(argv[1]) if len(argv) > 1 else DEFAULT
    if not path.exists():
        print(f"FAIL: Vektordatei fehlt: {path}")
        return 1

    data = json.loads(path.read_text())
    problems: list[str] = []

    def check(condition: bool, message: str) -> None:
        if not condition:
            problems.append(message)

    check(data.get("schema") == "atc.conformance.identity-value.v1", "falsches Schema")
    layout = data.get("layout", {})
    check(layout.get("total_bytes") == TOTAL_BYTES, "Layout nennt nicht 32 Byte")
    check(layout.get("tag", {}).get("bytes") == [0, 3], "Kennzeichenbereich falsch")
    check(layout.get("reserved", {}).get("bytes") == [4, 11], "reservierter Bereich falsch")
    check(layout.get("address", {}).get("bytes") == [12, 31], "Adressbereich falsch")

    vectors = data.get("vectors") or []
    check(len(vectors) >= 9, "zu wenige Vektoren")
    seen: dict[str, bytes] = {}
    tags: dict[str, str] = {}

    for v in vectors:
        vid = v.get("id", "?")
        raw = v.get("hex", "")
        if len(raw) != TOTAL_BYTES * 2 or any(c not in "0123456789abcdef" for c in raw):
            problems.append(f"{vid}: hex ist keine 32-Byte-Kette")
            continue
        word = bytes.fromhex(raw)
        seen[vid] = word
        tag, reason = decode(word)
        if v.get("valid"):
            if tag is None:
                problems.append(f"{vid}: sollte gueltig sein, ist aber {reason}")
            else:
                tags[vid] = tag
                if v.get("tag") and v["tag"] != tag:
                    problems.append(f"{vid}: Kennzeichen {tag} != erwartet {v['tag']}")
                if v.get("address") and v["address"] != "ATC:" + word[12:32].hex():
                    problems.append(f"{vid}: Adressfeld stimmt nicht mit dem Wort ueberein")
        else:
            if tag is not None:
                problems.append(f"{vid}: sollte ungueltig sein, gilt aber als {tag}")
            if not v.get("reason"):
                problems.append(f"{vid}: ungueltiger Vektor ohne Grund")
            elif v["reason"] not in INVALID_REASONS:
                problems.append(f"{vid}: unbekannter Grund {v['reason']}")
            elif v["reason"] != reason:
                problems.append(f"{vid}: Grund {v['reason']} != erkannt {reason}")

    if "T-38a" in seen and "T-38b" in seen:
        check(seen["T-38a"] != seen["T-38b"], "T-38a und T-38b sind byte-gleich")
        check(seen["T-38a"][12:32] == seen["T-38b"][12:32], "T-38a/T-38b haben verschiedene Adressen")
        check(tags.get("T-38a") == "wallet" and tags.get("T-38b") == "contract",
              "T-38a/T-38b tragen nicht Wallet und Vertrag")
    else:
        problems.append("T-38a/T-38b fehlen")

    if "T-39a" in seen and "T-39b" in seen:
        check(seen["T-39a"] != seen["T-39b"], "T-39a und T-39b sind byte-gleich")
        check(tags.get("T-39a") == "wallet", "T-39a (origin) traegt nicht das Wallet-Kennzeichen")
        check(tags.get("T-39b") == "contract", "T-39b (caller) traegt nicht das Vertragskennzeichen")
    else:
        problems.append("T-39a/T-39b fehlen")

    valid = sum(1 for v in vectors if v.get("valid"))
    invalid = len(vectors) - valid
    print(f"{path}: {len(vectors)} Vektoren ({valid} gueltig, {invalid} ungueltig), "
          f"Status {data.get('status')}, Standard {data.get('standard_version')}")

    if problems:
        for p in problems:
            print("FAIL:", p)
        return 1
    print("PASS: alle Vektoren entsprechen dem kanonischen Layout")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))

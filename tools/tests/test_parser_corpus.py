"""Tests fuer den Parser-Korpus und den Referenzdekoder (Abschnitt 19).

Die Tests pruefen drei Dinge: der Korpus ist reproduzierbar und von Hand unveraendert,
der Referenzdekoder bestaetigt jede Zeile, und die Abgrenzung der Stromcodes gilt
auch dort, wo zwei verschiedene Konstruktionen denselben Offset treffen.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

import pytest

TOOLS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(TOOLS))

from atc_reference_decoder import DOMAIN_ARTIFACT, decode, decode_hex  # noqa: E402
from build_parser_corpus import CORPUS, build, serialise, header, seal  # noqa: E402

DECODER = TOOLS / "atc_reference_decoder.py"


@pytest.fixture(scope="module")
def corpus() -> dict:
    return json.loads(CORPUS.read_text(encoding="utf-8"))


def entry(corpus: dict, cid: str) -> dict:
    for row in corpus["eintraege"]:
        if row["id"] == cid:
            return row
    raise AssertionError(f"{cid} fehlt im Korpus")


def test_korpus_ist_reproduzierbar() -> None:
    assert CORPUS.read_text(encoding="utf-8") == serialise(build())


def test_korpus_hat_mindestens_30_eingaben(corpus: dict) -> None:
    assert len(corpus["eintraege"]) >= 30


def test_alle_klassen_sind_belegt(corpus: dict) -> None:
    klassen = {row["klasse"] for row in corpus["eintraege"]}
    assert klassen == {f"C-{i}" for i in range(1, 9)}


def test_referenzdekoder_bestaetigt_jede_zeile(corpus: dict) -> None:
    abweichungen = []
    for row in corpus["eintraege"]:
        result = decode_hex(row["eingabe_hex"])
        if (result.decision, result.code, result.offset) != (
            row["entscheidung"],
            row["code"],
            row["offset"],
        ):
            abweichungen.append((row["id"], row))
    assert not abweichungen


def test_gleicher_offset_verschiedene_codes(corpus: dict) -> None:
    """Der Kern der Entscheidung: bei Offset 19 stehen E-005 und E-008 nebeneinander."""
    zu_klein = entry(corpus, "C-8-GV1-ZU-KLEIN")  # Anzahl 1, zwei Instruktionen im Strom
    gekuerzt = entry(corpus, "C-5-GV1-K1")  # zwei Instruktionen deklariert, Halt fehlt
    assert (zu_klein["offset"], gekuerzt["offset"]) == (19, 19)
    assert zu_klein["code"] == "E-008"
    assert gekuerzt["code"] == "E-005"
    assert decode_hex(zu_klein["eingabe_hex"]).code == "E-008"
    assert decode_hex(gekuerzt["eingabe_hex"]).code == "E-005"


def test_grenze_sauber_gegen_operand(corpus: dict) -> None:
    """Ein Byte Unterschied an derselben Stelle: E-005 an der Grenze, E-002 im Operanden."""
    grenze = decode_hex(entry(corpus, "C-5-GV1-K1")["eingabe_hex"])
    operandum = decode_hex(entry(corpus, "C-5-GV1-K2")["eingabe_hex"])
    assert (grenze.code, grenze.offset) == ("E-005", 19)
    assert (operandum.code, operandum.offset) == ("E-002", 18)


def test_struktur_vor_integritaet(corpus: dict) -> None:
    """Veralteter und neu berechneter Tail ergeben denselben Strukturcode."""
    for stale, sealed in (
        ("C-5-OPCODE", "C-5-OPCODE-NEU-VERSIEGELT"),
        ("C-6-K1", "C-6-K1-NEU-VERSIEGELT"),
        ("C-8-GV1-ZU-KLEIN", "C-8-GV1-ZU-KLEIN-NEU-VERSIEGELT"),
    ):
        a = decode_hex(entry(corpus, stale)["eingabe_hex"])
        b = decode_hex(entry(corpus, sealed)["eingabe_hex"])
        assert (a.code, a.offset) == (b.code, b.offset), f"{stale} gegen {sealed}"


def test_e012_nur_mit_konfiguriertem_grenzwert(corpus: dict) -> None:
    """REQ-SCE-089: ohne Betreiberwert ist E-012 nicht erreichbar."""
    data = bytes.fromhex(entry(corpus, "C-1-GV1")["eingabe_hex"])
    assert decode(data).decision == "accept"
    assert decode(data, max_stream_bytes=9).code == "E-012"
    assert decode(data, max_stream_bytes=10).decision == "accept"


def test_deklarierte_anzahl_ist_kein_groesstwert() -> None:
    """0xFFFFFFFF kostet einen Schritt und ergibt E-005, nicht E-012."""
    body = header(generation=1, count=0xFFFFFFFF) + bytes.fromhex("01010000000000000010")
    result = decode(seal(body))
    assert (result.code, result.offset) == ("E-005", 20)


def test_p2_korrektur_ist_wirklich_noetig() -> None:
    """Der frueher dokumentierte P2-Strom war nicht dekodierbar (Store ohne Operand)."""
    alter_strom = bytes.fromhex("010700000000000000012a00000000000000020b10")
    alt = header(generation=1, count=5) + alter_strom
    assert decode(alt + hashlib.sha256(DOMAIN_ARTIFACT + alt).digest()).code == "E-002"

    neuer_strom = bytes.fromhex("010700000000000000012a00000000000000020b0000000010")
    neu = header(generation=1, count=5) + neuer_strom
    result = decode(neu + hashlib.sha256(DOMAIN_ARTIFACT + neu).digest())
    assert result.decision == "accept"
    assert result.instructions == 5


def test_determinismus_ueber_zwei_prozesse(corpus: dict) -> None:
    """REQ-SCE-082: derselbe Aufruf in zwei Prozessen liefert dasselbe Ergebnis."""
    data_hex = entry(corpus, "C-4-V01")["eingabe_hex"]
    runs = [
        subprocess.run(
            [sys.executable, str(DECODER), data_hex],
            capture_output=True,
            text=True,
            check=False,
        )
        for _ in range(2)
    ]
    assert runs[0].stdout == runs[1].stdout
    assert json.loads(runs[0].stdout)["code"] == "E-003"

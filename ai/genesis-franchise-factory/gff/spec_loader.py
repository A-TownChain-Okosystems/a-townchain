"""Laedt und validiert die kanonischen .atc-Factory-Specs (AD-20..AD-43).

Dies ist ein Deskriptor-Parser (Metadaten-Extraktion), KEIN ATCLang-Compiler:
Der vollstaendige ATCLang-Compiler/Interpreter lebt eigenstaendig im atclang-Repo (L0).
"""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from pathlib import Path

AD_RE = re.compile(r"^//\s*AD-(\d+)\s*(?:[—\-]+\s*)?(.+)$", re.MULTILINE)
STRUCT_RE = re.compile(r"struct\s+(\w+)")
ENUM_RE = re.compile(r"enum\s+(\w+)")
FN_RE = re.compile(r"pub\s+fn\s+(\w+)")
COPYRIGHT_RE = re.compile(r"Copyright \(c\) 2026")
FORBIDDEN_RE = re.compile(r'import\s+"?[^"\n]*chronicles', re.IGNORECASE)


@dataclass(frozen=True)
class FactorySpec:
    """Metadaten-Deskriptor einer .atc-Factory-Spec."""

    ad_id: int
    title: str
    file: str
    structs: tuple[str, ...] = ()
    enums: tuple[str, ...] = ()
    functions: tuple[str, ...] = ()
    raw: str = field(default="", repr=False, compare=False)

    @property
    def has_implementation_surface(self) -> bool:
        """Ehrlich: Eine Spec gilt als substanzell, wenn struct + pub fn deklariert sind."""
        return bool(self.structs) and bool(self.functions)


class SpecValidationError(ValueError):
    pass


def load_spec(path: Path) -> FactorySpec:
    raw = path.read_text(encoding="utf-8")
    m = AD_RE.search(raw)
    if not m:
        raise SpecValidationError(f"{path.name}: kein '// AD-xx — Titel'-Header")
    ad_id = int(m.group(1))
    if not COPYRIGHT_RE.search(raw):
        raise SpecValidationError(f"{path.name}: Copyright-Header fehlt")
    if FORBIDDEN_RE.search(raw):
        raise SpecValidationError(
            f"{path.name}: verbotene Abhaengigkeit 'chronicles' (Plattform-Trennung)"
        )
    ad_in_name = re.search(r"_ad(\d+)\.atc$", path.name)
    if ad_in_name and int(ad_in_name.group(1)) != ad_id:
        raise SpecValidationError(
            f"{path.name}: AD-ID im Dateinamen ({ad_in_name.group(1)}) != Header ({ad_id})"
        )
    return FactorySpec(
        ad_id=ad_id,
        title=m.group(2).strip(),
        file=path.name,
        structs=tuple(STRUCT_RE.findall(raw)),
        enums=tuple(ENUM_RE.findall(raw)),
        functions=tuple(FN_RE.findall(raw)),
        raw=raw,
    )


def load_specs(spec_dir: Path) -> dict[int, FactorySpec]:
    """Laedt alle *_adNN.atc Specs eines Verzeichnisses, sortiert nach AD-ID."""
    specs: dict[int, FactorySpec] = {}
    for p in sorted(spec_dir.glob("*_ad*.atc")):
        s = load_spec(p)
        if s.ad_id in specs:
            raise SpecValidationError(
                f"AD-{s.ad_id:02d} doppelt: {specs[s.ad_id].file} vs {s.file}"
            )
        specs[s.ad_id] = s
    return specs

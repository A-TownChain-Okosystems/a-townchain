"""Laedt und validiert die kanonischen .atc-Factory-Specs (AD-20..AD-43)."""
from __future__ import annotations

import re
from dataclasses import dataclass, field
from pathlib import Path

AD_RE = re.compile(
    r"^//\s*AD-(\d+)\s*(?:[—\-]+\s*)?(.+)$",
    re.MULTILINE,
)
STRUCT_RE = re.compile(r"struct\s+(\w+)")
ENUM_RE = re.compile(r"enum\s+(\w+)")
FN_RE = re.compile(r"pub\s+fn\s+(\w+)")
COPYRIGHT_RE = re.compile(r"Copyright \(c\) 2026")
FORBIDDEN_RE = re.compile(
    r'import\s+"?[^"\n]*chronicles',
    re.IGNORECASE,
)


@dataclass(frozen=True)
class FactorySpec:
    ad_id: int
    title: str
    file: str
    structs: tuple[str, ...] = ()
    enums: tuple[str, ...] = ()
    functions: tuple[str, ...] = ()
    raw: str = field(default="", repr=False, compare=False)

    @property
    def has_implementation_surface(self) -> bool:
        return bool(self.structs) and bool(self.functions)


class SpecValidationError(ValueError):
    pass


def load_spec(path: Path) -> FactorySpec:
    raw = path.read_text(encoding="utf-8")
    match = AD_RE.search(raw)
    if not match:
        raise SpecValidationError(
            f"{path.name}: kein '// AD-xx — Titel'-Header"
        )

    ad_id = int(match.group(1))
    if not COPYRIGHT_RE.search(raw):
        raise SpecValidationError(f"{path.name}: Copyright-Header fehlt")
    if FORBIDDEN_RE.search(raw):
        raise SpecValidationError(
            f"{path.name}: verbotene Abhaengigkeit 'chronicles' "
            "(Plattform-Trennung)"
        )

    ad_in_name = re.search(r"_ad(\d+)\.atc$", path.name)
    if ad_in_name and int(ad_in_name.group(1)) != ad_id:
        raise SpecValidationError(
            f"{path.name}: AD-ID im Dateinamen ({ad_in_name.group(1)}) "
            f"!= Header ({ad_id})"
        )

    return FactorySpec(
        ad_id,
        match.group(2).strip(),
        path.name,
        tuple(STRUCT_RE.findall(raw)),
        tuple(ENUM_RE.findall(raw)),
        tuple(FN_RE.findall(raw)),
        raw,
    )


def load_specs(spec_dir: Path) -> dict[int, FactorySpec]:
    specs = {}
    for path in sorted(spec_dir.glob("*_ad*.atc")):
        spec = load_spec(path)
        if spec.ad_id in specs:
            raise SpecValidationError(
                f"AD-{spec.ad_id:02d} doppelt: "
                f"{specs[spec.ad_id].file} vs {spec.file}"
            )
        specs[spec.ad_id] = spec
    return specs

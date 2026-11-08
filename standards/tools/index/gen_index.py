#!/usr/bin/env python3
"""Generiert INDEX.md — Master-Index des ATC-Standards-Systems (SCR-0038).

INDEX.md ist NICHT-normativ und wird zu 100 % aus den SSOT-Registern
generiert (registry/*.yaml). Manuelle Aenderungen sind verboten —
Regeneration via tools/index/gen_index.py. Bei Konflikten gilt die
SSOT-Kaskade: Governance > Standard > Registry > INDEX > Implementierung.
"""

import os
from collections import Counter
from datetime import date

import yaml

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))


def R(n):
    return yaml.safe_load(open(os.path.join(ROOT, "registry", n), encoding="utf-8"))


std = R("standards.yaml")["standards"]
fw = R("framework.yaml")["framework"]
fams = fw["families"]
vs = R("versions.yaml")["versions"]
proto = R("protocol-registry.yaml")["protocol-registry"]
findings = R("findings.yaml")
F = findings.get("findings", findings if isinstance(findings, list) else [])
reg_files = sorted(
    f for f in os.listdir(os.path.join(ROOT, "registry")) if f.endswith((".yaml", ".json"))
)

today = date.today().isoformat()
sc = Counter(e["status"] for e in std)
latest = {e["id"]: vs.get(e["id"], [{}])[0].get("version", e["version"]) for e in std}


# Familien-Slot-Statistik
def slotstat(f):
    ss = f.get("slots", [])
    st = Counter(s["status"] for s in ss)
    return len(ss), st.get("BELEGT", 0), st.get("VERWEIST", 0)


fam_rows = "\n".join(
    f"| {f['id']} | {f['name']} | {n} | {b} | {v} |"
    for f, (n, b, v) in ((f, slotstat(f)) for f in fams)
)

std_rows = "\n".join(
    f"| {e['id']} | {e['title'].replace('|', '/')} | {e['category']} | {e['version']} | "
    f"{e['status']} | {e['file']} |"
    for e in sorted(std, key=lambda x: x["id"])
)

find_open = [x for x in F if isinstance(x, dict) and x.get("status") == "OPEN"]
find_res = len(F) - len(find_open)
proto_stat = Counter(p.get("status") for p in proto.get("protocols", []))

IDX = f'''---
document:
  id: ATC-STD-INDEX-001
  type: master-index
  version: "1.0.0"
  status: GENERATED
  normative: false
  owner: "A-TownChain Okosystems (Michael Wroblewski)"
  generated: "{today}"
  generator: "tools/index/gen_index.py (SCR-0038)"
  sources: "registry/standards.yaml + registry/framework.yaml + registry/categories.yaml + registry/taxonomy.yaml + registry/versions.yaml + registry/protocol-registry.yaml + registry/findings.yaml"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC Standards Index — Master Index (ATC-STD-INDEX-001, v1.0.0)

> **Nicht-normativ · generiert.** Diese Datei ist der zentrale Einstiegspunkt in
> das ATC-Standards-System — sie enthält KEINE eigenen Fachwahrheiten. Sie wird
> vollständig aus den SSOT-Registern generiert ({today}, SCR-0038); manuelle
> Änderungen sind verboten (Regeneration: `python3 tools/index/gen_index.py`).
> **SSOT-Kaskade bei Konflikten:** Governance (ATC-STD-000) → Standard →
> Registry → INDEX → Implementierung.

## 1. Zweck

Der Index beantwortet: welche Standards existieren, wo sie liegen, zu welcher
Familie sie gehören, welchen Status/welche Version sie haben — und verweist auf
Abhängigkeiten, offene Punkte und Ersatz-Beziehungen. Die normative Wahrheit
eines Standards liegt ausschließlich in seiner Standarddatei; die Registry
(registry/standards.yaml) ist das SSOT des Bestands.

## 2. Die Register (SSOT-Ebene) — alle maschinenlesbar

| Registry | Inhalt | Bindender Standard |
|---|---|---|
{chr(10).join(f"| registry/{f} | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |" for f in reg_files)}

Kernregister: **standards.yaml** (Bestand, {len(std)} Standards) · **versions.yaml**
(Versionierung je Standard) · **framework.yaml** (Katalog: {len(fams)} Familien,
{sum(len(f.get("slots", [])) for f in fams)} Slots) · **categories.yaml**
(Kategorien) · **taxonomy.yaml** (Domain/Familie/Kategorie) · **protocol-registry.yaml**
({sum(proto_stat.values())} Protokollfamilien, Status {dict(proto_stat)}) ·
**findings.yaml** (Findings: {len(find_open)} OPEN / {find_res} RESOLVED von {len(F)}).

## 3. Standardfamilien (Katalog, {len(fams)} Familien)

| FAM | Familie | Slots | BELEGT | VERWEIST |
|---|---|---|---|---|
{fam_rows}

Statusverteilung der {len(std)} Registry-Standards: {dict(sc)}.
Alle {len(std)} sind APPROVED und normativ (§30-eingefroren); Details je Standard
in registry/standards.yaml und registry/versions.yaml.

## 4. Master-Registry-Tabelle ({len(std)} Standards)

Sortiert nach ID; Version = aktuelle Registry-Version; Status = Registry-Status.

| Standard-ID | Titel | Kategorie | Version | Status | Datei |
|---|---|---|---|---|---|
{std_rows}

## 5. Statusmodell (Registry-Lifecycle)

Registry-Statusverteilung (Ist): {dict(sc)}. Lifecycle der Standards-Entwicklung
gemäß ATC-STD-STDDEV-001 / ATC-STD-TAXONOMY-001: Entwurf (Owner-Entwurf/SCR) →
§9-Freigabe (Owner, Human-Gate) → APPROVED (normativ, §30-eingefroren) → ggf.
DEPRECATED/RETIRED via Change-Control (ATC-STD-CHANGE-001). Protokolle folgen
zusätzlich REQ-PROTO-021 (draft bis verifizierte Implementierung).

## 6. Prioritätsmodell

P0 (kritisch, blockiert Release) · P1 (hoch, kurzfristig) · P2 (mittel, Roadmap) ·
P3 (Backlog/Optimierung). Verwendet in registry/findings.yaml, Issues und Audit-
Klassifikation (ATC-STD-REPO-AUDIT-001/002).

## 7. Offene Punkte (Auszug — Details: STATUS.md)

- Findings OPEN: {len(find_open)} (aktuelle Liste: registry/findings.yaml)
- Org-Audit-Ableitungen: Issues #94–98 (a-townchain-os) — CI 23/26, CodeQL,
  Versions-Baseline, verwaister Tag, ATC-STD-202-Klassifizierung
- ATC-LICENSE: 5 Lizenztypen PLANNED (SOURCE, COMMERCIAL, PROPRIETARY, DATA,
  EXPERIMENTAL); ATC-LICENSE.yaml-Manifeste + License Scanner S-26 ausstehend
- Protokollfamilien: {proto_stat.get("planned", 0)} planned / {proto_stat.get("draft", 0)} draft
  (registry/protocol-registry.yaml)

## 8. Integrität & automatische Prüfung

Die Index-Integritätsregeln werden nicht hier, sondern durch die Validator-Pipeline
erzwungen (tools/atc-std-validator/validate_all.py, je CI-Lauf): S-01 Metadaten,
S-14/S-19 Version/Status-Konsistenz, S-16 Naming, S-17 Duplikate, S-18 Registry-Parse,
S-21 Katalog, S-23 Protokolle, S-24 Taxonomie, S-25 Frontmatter — inklusive
Mutationstests M1–M12 (Fehler-erkennungs-pflichtig). Der Index selbst ist
regenerierbar und kann per Definition nicht driften.

## 9. Governance-Regel

INDEX.md ist Navigationsindex. Eine Änderung an einem Standard darf niemals
ausschließlich hier vorgenommen werden — sie läuft über: SCR → Standard-Datei →
registry/standards.yaml/versions.yaml → CHANGELOG → Validator → INDEX-Regeneration.
Dokument-ID ATC-STD-INDEX-001 ist KEIN Registry-Standard (kein REQ-Träger), sondern
die Kennung dieses generierten Dokuments.

## 10. Verzeichnisstruktur (Ist-Zustand)

atc-standards/ · INDEX.md (generiert) · README.md · CHANGELOG.md · STATUS.md ·
LICENSE (Apache-2.0) · AGENT_MANIFEST.md · AGENTS.md · governance/ (ATC-STD-000) ·
standards/<kategorie>/ (Fachstandards, {len(std)}-Bestand) · registry/ ({len(reg_files)}
SSOT-Dateien) · licenses/ (ATC-LICENSE-System) · schemas/ · tools/ (Generatoren +
atc-std-validator) · approval/ (§9-Freigabe-Archiv) · change-requests/ (SCR-0001…) ·
docs/ (Audits & Analysen) · templates/ · .github/workflows (Governance-CI, 2) +
ai/agent.yaml (Agenten-Bindung).

*ATC-STD-INDEX-001 v1.0.0 · generiert {today} · tools/index/gen_index.py · SCR-0038 · Aurora (Superagent)*
'''

open(os.path.join(ROOT, "INDEX.md"), "w", encoding="utf-8").write(IDX)
print(
    f"INDEX.md generiert: {len(IDX.splitlines())} Zeilen, {len(std)} Standards, {len(fams)} Familien"
)

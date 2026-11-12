# Engineering Audit

Repository: `atclang`
Status: BASELINE
Last verified: 2026-09-15

## Required audit loop

1. Implement code changes from authoritative requirements.
2. Run repository-native formatting, build and test checks.
3. Record functional errors and their evidence.
4. Record security findings and their evidence.
5. Record consistency findings across code, manifests and documentation.
6. Fix findings, then rerun the affected checks.
7. Verify module/function connectivity and fail closed on missing evidence.

## Current evidence boundary

This file establishes the organization-wide audit contract. It does **not** claim that the repository is production-ready. CI is the authoritative execution evidence.

## Security baseline

High-confidence credential patterns are rejected by `.github/workflows/repository-health.yml`. Cryptographic, parser, dependency and runtime security require repository-specific review.

## Consistency baseline

The repository name, documentation, manifests, public APIs and CI configuration must describe the same current implementation state. Target architecture must not be documented as implemented software.

## Evidence

- Automated repository-health workflow: `.github/workflows/repository-health.yml`
- Deep findings must be added here with reproducible evidence and a verification result.

## Nachtrag 2026-11-12 — Befunde F1 bis F5 (Owner-Audit) und Stand E1

Quelle: Owner-Audit vom 12.11.2026 gegen den GitHub-HEAD. Der Befund ist
monorepo-weit gueltig; die Umsetzung von E1 liegt in `crates/atc-core`.

**F1 (P0) — Contract-Body wird verworfen.** `lower_program` uebersprang
`Stmt::Contract`, erzeugte `ConstI64(0)`/`Return`, `atc run` gab `0` aus (Exit 0)
und `System::deploy` bestaetigte das Deployment. Datenverlust auf Sprachebene bei
gleichzeitigem Erfolgsstatus. **Status: behoben** (E1, `lower::executable_subset_gate`).

**F2 (P0) — Opcode-Inkompatibilitaet zwischen Compiler und VM.** Compiler und
ATC-VM interpretieren dieselben Opcode-Werte unterschiedlich. **Status: offen**
(Schritt E2: ATC-BC-001 als einzige normative Opcode-Tabelle, E3: VM-Decoder).

**F3 (P1/P2) — Fehlende IR, schwaches Gate.** Validierung und tatsaechliche
Kompilierung haben unterschiedliche Aussagekraft; `atc check` meldete "validiert"
fuer Quellen, die nicht ausfuehrbar sind. **Status: teilweise behoben** — `atc check
<datei>` nutzt jetzt dasselbe Gate wie `run`. Eine eigenstaendige IR wird nicht
eingefuehrt, bevor nachgewiesen ist, welche Compilerarchitektur vorhanden ist.

**F4 (P1) — i64/u128-Inkonsistenz** zwischen Bytecode und ABI-/Wirtschaftstypen.
**Status: offen** (Schritt E5).

**F5 (P1) — kein CI-Build fuer `atc-core`.** 219 lokale Tests sind kein
CI-Verifikationsnachweis. **Status: offen** (Schritt E5).

### Zweiter Teil von E1: Entry-Point und Pruefpfad

Ueber F1 hinaus zeigte die Nachpruefung zwei weitere stille Erfolge im Monorepo:

- `atc run` einer leeren oder rein deklarativen Quelle endete mit Exit 0 und der
  Ausgabe `0`. Jetzt: `lower::require_entry_point` fordert eine Funktion `main`.
- `atc check <datei>` akzeptierte Vertragsquellen mit "OK validiert". Jetzt nutzt
  `check` dasselbe Gate wie `run` und endet mit Exit 1.

Faktenkorrektur fuer Folgeaudits: das Gateway-Beispiel (`examples/main.atc`) hat
**10 Funktionen, 8 Zustandsfelder, 4 Events**; die **11** verlorenen Funktionen
gehoeren zu FranchiseToken (`apps/atc-contracts/contracts/token.atc`).

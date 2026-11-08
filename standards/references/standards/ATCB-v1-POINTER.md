# Registry-Pointer: ATCB v1 (ATC-BC-001)

> Verweis gemäß ATC-STD-000 §21/§19: ATCB v1 ist normativ in **atclang**
> beheimatet (Owner-Entscheidung 2026-10-06). Dieses Repository hält
> **keine Kopie** — ausschließlich den Pointer.

| Feld | Wert |
|---|---|
| Standard | ATC-BC-001 — Bytecode Specification (ATCB v1, Canonical Encoding) |
| Träger-Repository | atclang (normativer SSOT-Träger, Owner-Entscheidung 06.10.2026) |
| Normversion | 1.0.0-FROZEN (Format ATCB v1, `format_version` u16 BE = 1) |
| Freeze-Anker | ee26531338787667ae417333b4900b302eb36353 |
| Freeze-Datum | 2026-10-06 |
| Kanonischer Pfad | atclang/specs/bytecode/SPEC.md (§8.1 Trägerschaft) |
| Status | FROZEN / NOT VERIFIED (Gate 0 ausstehend) |
| Änderungen | SCR/MINOR (ATC-STD-UPDATE-001); Trägerschafts-Wechsel = SCR |
| Normative Kraft | SCR-0129 PENDING_OWNER_APPROVAL — ATC-STD-000 §9-Freigabe erfüllt (SCR-0098); Owner-Freigabe der Setzung offen; bei Approve: Registry → Standard → Validator/CI |

Begründung der Trägerschaft (atclang): evolutionäre Kopplung — ATCB ist der
G3-Output der ATCLang-Compiler-Pipeline (AD-022); Referenzimplementierung
(`encode_canonical()` / `decode_bytecode()`) und Determinism-/Differential-
Suiten leben in atclang; der Freeze-Anker wandert nicht.

Gebundene Komponenten (Bindungswirkung laut ATC-BC-001 §8): ATCLang (Codegen),
ATC-VM/ATVM (Verifikation/Ausführung), atc-toolchain (Verifikations-Adapter).

# 📋 File Register — a-townchain-os

> **Zweck:** Vollständige maschinell generierte Liste aller Dateien (SCR-0082, Matrix-Dim-12).
> **Auto-generiert:** 2026-09-10 10:48 UTC | via GitHub-Trees-API
> **Standards:** ATC-STD-201 · Evidence: `.atc/evidence/evidence.yaml`

---

## Zusammenfassung

| Metrik | Wert |
|--------|------|
| **Total Dateien** | 28 |
| **.md** | 16 |
| **.yaml** | 8 |
| **(keine)** | 3 |
| **.yml** | 1 |

---

## Dateiliste

- `.atc/compliance.yaml`
- `.atc/evidence/evidence.yaml`
- `.atc/lifecycle.yaml`
- `.atc/ownership.yaml`
- `.atc/repository.yaml`
- `.github/ai/agent.yaml`
- `.github/ai/capabilities.yaml`
- `.github/ai/policies.yaml`
- `.github/workflows/governance-ci.yml`
- `.gitignore`
- `AGENTS.md`
- `AGENT_MANIFEST.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `CODEOWNERS`
- `CODE_OF_CONDUCT.md`
- `CONTRIBUTING.md`
- `GOVERNANCE.md`
- `LICENSE`
- `README.md`
- `ROADMAP.md`
- `SECURITY.md`
- `STATUS.md`
- `docs/REPOSITORY_STANDARD.md`
- `docs/specs/ATC-AOS-001.md`
- `docs/specs/ATC-AOS-002.md`
- `docs/specs/ATC-AOS-003.md`
- `tests/TESTPLAN.md`

---

## Nachtrag 2026-11-08 (Handshake + Handoff)

| Datei | Zweck |
|---|---|
| `kai-os/network/src/auth.rs` | Challenge längenpräfigiert, Domain- und Chain-Bindung (SEC-P0-008) |
| `kai-os/integration/src/handoff.rs` | Handoff-Signatur auf secp256k1/RFC6979, Low-S, öffentliche Verifikation (SEC-P0-006) |
| `kai-os/integration/tests/e2e.rs` | E2E-Abgrenzung Ed25519 vs. secp256k1 + Domain-Separation |

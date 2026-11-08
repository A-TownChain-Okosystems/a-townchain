# ⚙️ Workflow Pipelines — atc-ci

---

## 1. Pipeline Übersicht

Das `atc-ci` Repo verwaltet automatisierte Workflows für Qualitätssicherung und Deployment:

- **`build-and-test.yml`**: Führt Tests über alle 70 Repositories aus.
- **`atclang-verify.yml`**: Validiert ATCLang Syntax, Typen und Bytecode-Kompilierung.
- **`security-audit.yml`**: Führt statische Analysen und BaFin Compliance Audits durch.
- **`testnet-deploy.yml`**: Rollout von Smart Contracts und Gateway Updates auf das Testnet.

---

## 2. Test Matrix

```yaml
strategy:
  matrix:
    python-version: ['3.11', '3.12']
    rust-target: ['x86_64-unknown-none', 'aarch64-unknown-none']
```

---
document_id: ATC-DOC-STORAGE-CONTRIB-001
title: Contributing Guidelines - atc-storage
version: 1.0.0
status: active
standard: ATC-STD-MD-001
created: 2026-09-08
updated: 2026-09-08
---

# Contributing to atc-storage

## Beitragsrichtlinien

Vielen Dank für Ihr Interesse an Beiträgen zu `atc-storage` im A-TownChain-Ökosystem.

### Entwicklungs-Workflow

1. **Standards einhalten:** Alle Beiträge müssen ATC-STD-000, ATC-STD-201, ATC-STD-202 und ATC-STD-203 entsprechen.
2. **Conventional Commits:** Verwenden Sie aussagekräftige Commit-Nachrichten (z. B. `feat: ...`, `fix: ...`, `docs: ...`).
3. **Tests durchführen:** Bevor Sie einen Pull Request erstellen, müssen alle Tests grün sein:
   ```bash
   cargo test
   ```
4. **Code Quality:** Stellen Sie sicher, dass keine Warnungen vorliegen und der Code formatiert ist (`cargo fmt`, `cargo clippy`).
5. **Dokumentation:** Aktualisieren Sie gegebenenfalls `CHANGELOG.md` und relevante Dokumentationsdateien.

### Governance & Security

- Architectural Changes benötigen ein SCR (System Change Request) gemäß ATC-STD-000.
- Sicherheitsrelevante Schwachstellen dürfen **nicht** über GitHub Issues gemeldet werden. Siehe [`SECURITY.md`](SECURITY.md).

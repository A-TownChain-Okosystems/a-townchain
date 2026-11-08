# Löschungs-Manifest 07.09.2026 (AD-047)

Owner-Mandat: „Nichtgebrauchte Legacy Dateien löschen".
Alle gelöschten Inhalte bleiben über die **Git-Historie dieses Repos**
(commit vor AD-047) vollständig rekonstruierbar: `git log --follow <pfad>`.

## 1. Test-/Demo-Müll (repos-rescue)
- archiviert-atc-test-delete-me-wiki, archiviert-demo-repository (4 Dateien)

## 2. Verwaiste Legacy-Module ohne Nachfolge-Repo (Legacy-Audit 07.09.)
Gelöscht aus monorepo-full/src/modules/ UND repos-rescue/archiviert-*
(je beide Snapshots, da kein aktuelles Repo die Inhalte referenziert):
atc-ui (28/4111 LOC), atc-ci (23/1135), atc-mobile (19/776), atc-monitoring,
atc-shivacore-tools, atc-deploy, atc-assets, atc-dns, atc-social, atc-ide,
atc-devtools, atc-atcpkg, atc-analytics, atc-linux-edition — 288 Dateien/8058 LOC.

NICHT gelöscht (weiterhin im Vault): atc-node-Stack (146 Dateien/7941 LOC,
wartet auf SC-Rebuild), atc-vm (30/614), atc-zkp Legacy (45/490, Referenz für
AD-045), atc-shivamon (AD-025: Vault-Archiv unangetastet), atc-whitepaper.

## 3. Supersede Plan-/Audit-Reports (70-Repo-Ära / K1-K8-Plan, durch AD-023/AD-024 superseded)
AUFRÄUM_BERICHT, COMPLETENESS_AUDIT, VOLLAUDIT, KONSOLIDIERUNGS_MATRIX,
KONSOLIDIERUNGS_ROADMAP, UMSETZUNGSPLAN, DATEI_PLATZIERUNG(+_FIX), FIXES,
PERFORMANCE_REPORT, AUDIT_REPORT, ARCHITECTURE_TREES, MIGRATION_MAP.
(NICHT gelöscht: REALITY_CHECK_2026-07-06 — referenziert von offener
Entscheidung AD-008.)

**Gesamt: 416 Dateien.** Bereinigte Verweise: ARCHITECTURE.md,
FILE_NAMING_CONVENTIONS.md, FILE_REGISTER.md (regeneriert).

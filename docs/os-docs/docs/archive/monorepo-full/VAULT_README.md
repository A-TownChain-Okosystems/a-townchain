# Monorepo-Vault — Gesamstand a-townchain-os (06.09.2026)

> Strukturgetreue, vollstaendige Kopie des Monorepos `a-townchain-os`
> zum Stand Commit 8606e50 (06.09.2026), unmittelbar vor dem Beschluss,
> die Repositories neu aufzubauen (AD-018).

**Inhalt:** kompletter Launch-Stack (src/-Kernstack, src/modules/ mit allen
60 Modulen inkl. ATCLang-Topologie-Fix, tests/ mit 731 Workspace-Tests,
docker/ mit 10-Dienste-Compose, scripts/ inkl. sync_modules.py AD-017,
config/, .github/-Workflows (hier als Archivkopie inaktiv), alle Build- und
Meta-Dateien).

**Zweck:** Kanonische Quelle fuer den Neuaufbau der Repositories. Die
Originale in a-townchain-os wurden entfernt; die Git-Historie dort enthaelt
jede Datei weiterhin (Reversibilitaet).

**Hinweis .github/:** liegt hier im Archivpfad und ist thereby inaktiv —
GitHub liest Workflows nur aus dem Repo-Root.

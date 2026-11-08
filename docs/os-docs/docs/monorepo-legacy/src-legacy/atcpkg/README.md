# ATC ATCPkg

## Status: Neu ausgegliedert (09.07.2026) -- reines Konzept-/Stub-Stadium

On-Chain Package-Manager-Konzept fuer das A-TownChain-Oekosystem
(Installieren, Publizieren, Verifizieren, Versionierung von Paketen/Modulen).

## Wichtig: zwei nicht-identische Implementierungen, Verhaeltnis ungeklaert

Bei der Migration wurden im Monorepo **zwei unterschiedliche** ATCPkg-Dateien
gefunden -- keine Duplikate, sondern inhaltlich verschiedene Entwuerfe:

- **`tools/manager.atc`** -- Referenz **ATC-24**, "Tools Layer", Sprint 3.0.
  145 Zeilen. Migriert aus `atcpkg/manager.atc`.
- **`kernel/manager.atc`** -- Referenz **ATC-96**, "Kernel Layer", v0.3
  ATCLang-Syntax. 208 Zeilen, umfangreicher (u.a. `PackageStatus`-Enum,
  Groessen-/Dependency-Limits). Migriert aus `modules/kernel/pkg/manager.atc`.

**Offene Frage:** Ist `kernel/manager.atc` eine neuere/vollstaendigere Version
von `tools/manager.atc`, oder sind das zwei bewusst getrennte Package-Manager
auf unterschiedlichen Layern (Tools vs. Kernel)? Aktuell nicht klar -- vor
Weiterentwicklung sollte das geklaert werden, um keine Doppelarbeit zu
produzieren.

## Scope-Hinweis (Standing Instruction)

ATC-24 faellt in den Bereich ATC-1-40 (verbindliche technische Spezifikation).
ATC-96 liegt ausserhalb des definierten Bereichs ATC-1-80 -- Einordnung
(technische Spec vs. Vision/Lore) ist fuer diese Nummer nicht abschliessend
geklaert und sollte bei Bedarf mit Alexander verifiziert werden.

## Kontext im Wiki (nur referenziert, nicht migriert)

- `docs/issues/ISSUE_27__ATCPKG__PLUGIN__MODUL-SYST.md`
- `docs/issues/ISSUE_30__WIKI_KAP._43__ATCPKG_REGIST.md`
- Wiki Kapitel 43 (ATCPkg-Registry) im Hauptrepo

---
*Migriert: 09.07.2026 -- kein produktiver Code, reines Konzeptstadium.*

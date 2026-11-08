# ADR-0001: System-of-Systems-Umbrella per git subtree

- Status: Accepted (SCR-0127, 2026-09-17)
- Entscheider: Owner (Direktive 17.09.2026)

## Kontext

Das A-TownChain-Ökosystem besteht aus unabhängigen Domänen-Repos (ATCLang,
A-TownChain-L1-Monorepo, ShivaCore-Kernel, Aurora AI, Genesis Engine). Es fehlte
eine Ebene, auf der das Gesamtsystem als ein System integriert, gebaut und
verifiziert wird.

## Entscheidung

Dach-Repository `a-townchain-ecosystem` mit Import der 5 Komponenten-Repos per
`git subtree` OHNE --squash (volle Historie, siehe MIGRATION_MANIFEST.yaml).

1. **SSOT-Disziplin**: Die Quell-Repos bleiben kanonisch; der Umbrella ist
   Integrationsschicht, kein Ersatz. Fachliche Entwicklung nur im Quell-Repo.
2. **Sync**: `git subtree pull --prefix=components/<name> <repo> main`.
3. **Honest Aggregation**: Komponenten-Gates laufen getreu den Original-Repos;
   vererbt rote Zustände (SCR-0124-Programm, genesis-engine PR #13) werden
   unverändert angezeigt, nicht weggefiltert.
4. **ATC-VM/ATC-Algorithm**: werden nicht separat importiert — kanonisch im
   L1-Monorepo enthalten (components/a-townchain/components/vm|algorithm);
   Separat-Import wäre Duplikatbildung.

## Konsequenzen

- Systemübergreifende Gates (Governance, Determinism, CodeQL, Test Suite)
  aggregieren den echten Ist-Zustand des Ökosystems.
- Rotmarken im Umbrella spiegeln Komponenten-Repos 1:1 und verschwinden erst
  mit deren Remediation (SCR-0124/0126, genesis-engine PR #13).
- ATCLang/ATC-VM/Genesis-Chronicles-Trennung (ATC-STD-000/201) bleibt gewahrt.

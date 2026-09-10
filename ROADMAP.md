# ROADMAP — a-townchain

> **A-TownChain Blockchain (Orchestration)** · Status ehrlich gebunden an `.atc/evidence/evidence.yaml` (SCR-0082)
> STATUSLEITER: SPECIFIED → IMPLEMENTED → TESTED → VERIFIED → AUDITED → RELEASED · CLAIMED ≠ PASS


## Jetzt (Q3 2026)
- [ ] M4-Integrationsstand halten (2 Nodes Gossip-Sync, Chain-ID 658467)
- [ ] Konsens-Refactor: Legacy-Konsensbestand → atc-algorithm-Dependency (F-105)
- [ ] Boundary-Check strict schalten nach Refactor

## Danach (Q4 2026)
- [ ] End-to-End Conformance: ATCLang→VM→Contract→Node→Chain
- [ ] Security-Audit (security: not_audited → audited)

## Release-Gate
- [ ] release: development → candidate erst nach conformance+audit
- [ ] KEIN Mainnet vor allen Gates (Evidence: .atc/evidence/evidence.yaml)

# Control Plane — Panel Registry

| ID | Panel | Primaere Systeme | Kernnachweise |
|---|---|---|---|
| SYS | System Overview | alle | Integration, Gesamt-Readiness |
| L1 | A-TownChain L1 | a-townchain, atc-node | Genesis→Tx→Block→Reward→State→Validator→Epoch→Evidence→Finality→Persist→Restart→Recovery |
| WALLET | Wallet | atc-wallet, atc-sdk | Wallet→Node→Block→Balance |
| LANG | ATCLang | atclang | parse→compile→test→determinism |
| VM | ATC-VM | atc-vm | execute→gas→determinism→errors |
| SHIVA | ShivaCore | atc-shivacore | boot→memory→IPC→capability→TPM/TEE |
| GLOBUS | GlobusOS | globus-os | boot→identity→storage→services→health |
| AURORA | Aurora AI | aurora-ai | agent identity→authorization→policy→execution→evidence |
| GENESIS | Genesis | genesis-engine, genesis-chronicles | editor→runtime→SDK→integration |
| INFRA | Infrastructure | CI/Runtime | CPU/RAM/disk/network/services |
| CICD | CI/CD | GitHub Actions | build/test/security/e2e/artifacts |
| SEC | Security | all | vulnerability/SBOM/provenance/attestation |
| GOV | Governance | .github, atc-standards | policy/standards/evidence/audit |
| RELEASE | Release Control | devnet/testnet/mainnet | gates→approval→release |

## L1 Control Actions

- Start/stop/restart node
- Drain/resync node
- Run multi-node E2E
- Inspect block/transaction/finality/evidence
- Emergency halt only behind explicit owner authorization

## Global Readiness Gates

1. Source present
2. Build green
3. Unit tests green
4. Integration tests green
5. Security gates green
6. Multi-process E2E green where applicable
7. Evidence bundle generated
8. Audit completed
9. Human approval recorded
10. Release policy satisfied

## UI rule

No panel may infer health from absence of errors alone. Every positive state must carry a timestamp, commit SHA/version, environment and evidence reference.

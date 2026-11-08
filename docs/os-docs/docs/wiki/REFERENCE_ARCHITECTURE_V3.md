# A-TownChain / GlobusOS Reference Architecture v3

This page is the documentation-hub view of the integrated architecture.

## Domains
D1 Experience, D2 Application, D3 Platform, D4 Execution, D5 System.

## Core boundary
Runtime -> Host ABI -> Syscall ABI -> ShivaCore -> HAL -> Hardware.

## Security
Identity -> Credential -> Capability -> Rights -> Policy -> Resource -> Audit.

## Blockchain
Wallet/SDK -> transaction -> mempool -> block -> validation -> consensus/finality -> state -> ATC-VM -> persistence -> restart recovery.

## AI
Aurora -> agent runtime -> planner -> tools -> policy -> capabilities -> Host ABI.

## Readiness
APPROVED, IMPLEMENTED, AUDITED and PRODUCTION_READY are separate evidence states.

The implementation source of truth for the active kernel is globus-os; this documentation hub does not replace source repositories.

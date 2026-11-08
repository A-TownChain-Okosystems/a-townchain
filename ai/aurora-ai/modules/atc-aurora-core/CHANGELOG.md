# 📝 Changelog — atc-aurora-core

## [Unreleased] — 2026-10-18

### Hinzugefügt
- UUIDv4 generation for agent IDs and execution run IDs.
- Agent ID syntax validation and retired identity tombstones prevent accidental identity reuse.
- JSON registry snapshots that preserve agent identity and revocation state across process restarts.
- Agent-bound task assignment with distinct swarm_id, task_id, agent_id, agent_role, capabilities and run_id.
- Fail-closed dispatch for assigned tasks when the agent is missing or revoked.
- Result attribution now requires task_id, agent_id and run_id.
- Structured agent/swarm/task/run attribution and agent-based lookup for audit records and execution/security events.
- Tests for duplicate identities, concurrent registration, persistence/reload, revocation and run-aware result aggregation.

### Verification status
- SDK Build and Test, Dependency Review and Repository Governance passed on the previously inspected head; final-head checks are still required after the last edits.
- The code-quality gate now format-checks changed Rust files, runs Cargo tests, and runs Clippy with warnings denied.
- Registry persistence is opt-in through save_json/load_json; deployment code must call these methods and protect the storage path.
- Metrics-backend integration is not included.

## [2026-08-08] — Initial
- Initial-Repository erstellt
- ARCHITECTURE.md, COMPONENT_PLAN.md, STATUS.md hinzugefügt

---
*Auto-generiert 2026-10-18 · Aurora (MasterBrain · Base44)*

# Aurora Swarm Agent Identity Contract

## Identity fields

| Field | Meaning | Lifecycle |
|---|---|---|
| `agent_id` | Stable logical identity of one registered agent | Generated once or supplied by a trusted provisioning system; persisted and reused after restart |
| `swarm_id` | Identity of the swarm that coordinates work | Set per swarm |
| `task_id` | Identity of one logical task | Stable for that task |
| `run_id` | Identity of one execution attempt | Fresh UUIDv4 each time a task is dispatched |
| `agent_role` | Role recorded from the active registry entry at assignment time | Assignment snapshot |
| `capabilities` | Capability names recorded from the active registry entry at assignment time | Assignment snapshot |
| `status` | Registry/task lifecycle status | Changes through explicit state transitions |

## Required call path

1. Provision an agent through `AgentRegistry::register_generated` or register a trusted, pre-existing ID.
2. Persist the registry with `save_json` and restore it with `load_json` at process startup. A caller that omits persistence has only process-local identity state.
3. Use `TaskScheduler::schedule_for_agent` for assigned work. It rejects unknown or revoked agents and snapshots role/capabilities into the task record.
4. Use `dispatch_ready_authorized` for assigned work. It rechecks current registry status before dispatch and fails closed if the agent is no longer active.
5. Read the task's `run_id` after dispatch and attach that value to the result passed to `ResultCollector::submit`.
6. Populate structured identity fields on `AuditRecord` and `AuroraEvent` for attributable audit/security/execution events.

## Invariants

- IDs must start with an ASCII alphanumeric and may contain only ASCII alphanumerics, colon, underscore, hyphen or dot.
- Duplicate `agent_id` registration is rejected without overwriting the existing record; unregister/revoke preserves an inactive tombstone so the ID cannot be reused.
- UUIDv4 is used for generated agent IDs and each execution run ID. Generated IDs are not regenerated automatically after restart.
- A new execution attempt gets a new `run_id`; it does not imply a new logical `agent_id`.
- Assigned tasks dispatched without the registry-aware dispatch API fail closed.
- Results without `task_id`, `agent_id`, `run_id` or a non-empty payload are rejected.
- Result aggregation is deterministic by `task_id`, `agent_id`, then `run_id`.
- Audit records and events can be queried by `agent_id` and retain `swarm_id`, `task_id`, and `run_id` as structured fields.

## Operational boundary

This change provides an in-process registry, explicit JSON snapshot persistence, structured audit/event attribution and deterministic orchestration primitives. It does not automatically wire a production daemon to load/save the registry, create OS-level agent processes, provide distributed locking across multiple service instances, guarantee tamper-proof audit storage, or connect these records to a metrics backend. Production deployment must configure access-controlled durable storage, populate identity fields in every emitted record, integrate metric labels, and ensure every scheduler integration uses the registry-aware methods. The Rust unit tests and exact-SHA CI remain the verification gate.

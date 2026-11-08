# Aurora AI Swarm Orchestration — Implementation and Acceptance

## Scope

This change implements orchestration primitives in the canonical Rust crate at modules/atc-aurora-core. It does not claim that legacy .atc stubs, external agent processes, durable queues, distributed scheduling, or production deployment are implemented.

## Components

### Agent registry
- Requires non-empty, caller-assigned stable IDs, names and roles.
- Rejects duplicate IDs rather than silently replacing an existing agent.
- Stores capability names and descriptive/risk metadata.
- Rejects empty/duplicate capability names and reports missing agents with typed errors.
- Returns active agents in stable ID order.
- Capability metadata is descriptive; authorization remains the responsibility of the existing fail-closed policy engine and downstream OS/kernel enforcement.

### Task scheduler
- Tracks queued, running and terminal states (completed, failed, cancelled, timed out).
- Requires dependencies to exist before scheduling and rejects failed/cancelled/timed-out dependencies.
- Dispatches ready tasks in lexical ID order within a configured concurrency limit.
- Starts timeout accounting when a task begins running; expire(now_ms) must be called by the runtime driver.
- Allows cancellation only for queued/running tasks; terminal states cannot be reopened.
- This is a deterministic in-memory scheduler model; it does not spawn workers, persist state, or preempt an external process.

### Result collector
- Requires a declared expected-agent set for each task.
- Rejects results from undeclared agents, empty payloads and duplicate task/agent results.
- Reports missing agents and returns results sorted by task ID then agent ID, independent of arrival order.
- Payload schema/domain validation beyond non-empty text remains the responsibility of the consuming task contract.

## Acceptance criteria

| ID | Criterion | Evidence |
|---|---|---|
| SW-REG-01 | Duplicate agent ID does not overwrite existing identity | Registry unit test |
| SW-REG-02 | Empty identity and duplicate capability names are rejected | Registry unit test |
| SW-REG-03 | Capability metadata is retained and active listing is deterministic | Registry unit test |
| SW-SCH-01 | Dependencies block dispatch until all prerequisites complete | Scheduler unit test |
| SW-SCH-02 | Parallel execution never exceeds configured limit | Scheduler unit test |
| SW-SCH-03 | Timeout and cancellation produce terminal states | Scheduler unit tests |
| SW-SCH-04 | Missing/failed dependencies and duplicate task IDs fail closed | Scheduler unit tests |
| SW-RES-01 | Unknown agent and duplicate results are rejected | Collector unit test |
| SW-RES-02 | Empty payload is rejected and missing agents are reported | Collector unit test |
| SW-RES-03 | Aggregation order is deterministic | Collector unit test |
| SW-CI-01 | Cargo tests pass on the exact PR head SHA | Link exact-SHA GitHub Actions run before marking PASS |

## Verification procedure

Run from the repository root:

```sh
cargo test --manifest-path modules/atc-aurora-core/Cargo.toml
cargo fmt --manifest-path modules/atc-aurora-core/Cargo.toml -- --check
```

Record the full commit SHA, command exit status and corresponding GitHub Actions run/job URLs in the pull request. Do not mark the change verified until those checks finish successfully.

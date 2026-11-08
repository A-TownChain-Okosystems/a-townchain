//! S04-S06 Integrationstests — Issue #103 Akzeptanzkriterien.
//! Deterministisch: kein RNG, keine Wall-Clock-Abhängigkeit (REQ-ENG-002).

use kai_os_runtime::capability::CapabilitySet;
use kai_os_runtime::resources::{ResourceError, ResourceManager};
use kai_os_runtime::sandbox::{Operation, Sandbox, SandboxError};
use kai_os_runtime::scheduler::Scheduler;

fn small_cap() -> CapabilitySet {
    CapabilitySet::builder()
        .cpu_ms(100)
        .memory(1_000)
        .storage(10_000)
        .allow_outbound(443)
        .allow_inbound(8000)
        .allow_syscall("read")
        .allow_syscall("write")
        .allow_crypto()
        .build()
        .unwrap()
}

fn closed_cap() -> CapabilitySet {
    CapabilitySet::builder().build().unwrap()
}

// ── AK 1: Capability-Validierung fail-closed ────────────────────────────

#[test]
fn capability_zero_quota_rejected() {
    assert!(CapabilitySet::builder().cpu_ms(0).build().is_err());
    assert!(CapabilitySet::builder().memory(0).build().is_err());
    assert!(CapabilitySet::builder().storage(0).build().is_err());
    // Baseline valid
    assert!(closed_cap().validate().is_ok());
}

// ── AK 2: Deny-by-default ───────────────────────────────────────────────

#[test]
fn deny_by_default_no_syscalls_no_ports_no_crypto() {
    let sb = Sandbox::new("agent-locked", closed_cap());
    let mut rm = ResourceManager::new();

    assert!(matches!(
        sb.execute(
            Operation::Syscall {
                name: "read".into()
            },
            &mut rm
        ),
        Err(SandboxError::Denied { .. })
    ));
    assert!(matches!(
        sb.execute(
            Operation::NetworkOut {
                bytes: 10,
                port: 443
            },
            &mut rm
        ),
        Err(SandboxError::Denied { .. })
    ));
    assert!(matches!(
        sb.execute(
            Operation::NetworkIn {
                bytes: 10,
                port: 80
            },
            &mut rm
        ),
        Err(SandboxError::Denied { .. })
    ));
    assert!(matches!(
        sb.execute(Operation::Sign { hash: "abc".into() }, &mut rm),
        Err(SandboxError::Denied { .. })
    ));
}

// ── AK 3: Quota-Enforcement (Exakt-am-Limit ok, Überschreitung Fehler) ──

#[test]
fn cpu_quota_enforced_exact_limit_ok() {
    let sb = Sandbox::new("cpu-agent", small_cap());
    let mut rm = ResourceManager::new();

    // 90 + 10 = 100 == Limit -> ok
    sb.execute(Operation::Compute { cpu_ms: 90 }, &mut rm)
        .unwrap();
    sb.execute(Operation::Compute { cpu_ms: 10 }, &mut rm)
        .unwrap();
    // 1 ms mehr -> Überschreitung
    let err = sb
        .execute(Operation::Compute { cpu_ms: 1 }, &mut rm)
        .unwrap_err();
    assert!(matches!(
        err,
        SandboxError::Resource(ResourceError::QuotaExceeded {
            resource: "cpu",
            ..
        })
    ));
    // Nach Tick-Reset wieder frei
    rm.reset_tick();
    sb.execute(Operation::Compute { cpu_ms: 100 }, &mut rm)
        .unwrap();
}

#[test]
fn memory_quota_and_free_model() {
    let sb = Sandbox::new("mem-agent", small_cap());
    let mut rm = ResourceManager::new();

    sb.execute(Operation::Alloc { bytes: 600 }, &mut rm)
        .unwrap();
    sb.execute(Operation::Alloc { bytes: 400 }, &mut rm)
        .unwrap(); // genau am Limit
    assert!(sb.execute(Operation::Alloc { bytes: 1 }, &mut rm).is_err());

    sb.execute(Operation::Free { bytes: 400 }, &mut rm).unwrap();
    assert_eq!(rm.usage("mem-agent").memory_bytes_current, 600);

    // Kein Negativsaldo (fail-closed)
    let err = sb
        .execute(Operation::Free { bytes: 1_000 }, &mut rm)
        .unwrap_err();
    assert!(matches!(
        err,
        SandboxError::Resource(ResourceError::InvalidFree { .. })
    ));
}

#[test]
fn storage_append_only_quota() {
    let sb = Sandbox::new("disk-agent", small_cap());
    let mut rm = ResourceManager::new();

    sb.execute(Operation::Store { bytes: 9_000 }, &mut rm)
        .unwrap();
    sb.execute(Operation::Store { bytes: 1_000 }, &mut rm)
        .unwrap(); // am Limit
    assert!(sb.execute(Operation::Store { bytes: 1 }, &mut rm).is_err());
    assert_eq!(rm.usage("disk-agent").storage_bytes_total, 10_000);
}

// ── AK 4: Network-Policy inbound/outbound getrennt ─────────────────────

#[test]
fn network_policy_directions_separate() {
    let sb = Sandbox::new("net-agent", small_cap());
    let mut rm = ResourceManager::new();

    // Outbound 443 erlaubt, 80 verboten
    sb.execute(
        Operation::NetworkOut {
            bytes: 100,
            port: 443,
        },
        &mut rm,
    )
    .unwrap();
    assert!(sb
        .execute(
            Operation::NetworkOut {
                bytes: 100,
                port: 80
            },
            &mut rm
        )
        .is_err());
    // Inbound 8000 erlaubt, 443 verboten
    sb.execute(
        Operation::NetworkIn {
            bytes: 100,
            port: 8000,
        },
        &mut rm,
    )
    .unwrap();
    assert!(sb
        .execute(
            Operation::NetworkIn {
                bytes: 100,
                port: 443
            },
            &mut rm
        )
        .is_err());

    let u = rm.usage("net-agent");
    assert_eq!(u.network_bytes_out, 100);
    assert_eq!(u.network_bytes_in, 100);
}

// ── AK 5: Policy -> Resource -> Execution Reihenfolge ──────────────────

#[test]
fn syscall_allowlist_and_crypto() {
    let sb = Sandbox::new("sys-agent", small_cap());
    let mut rm = ResourceManager::new();

    sb.execute(
        Operation::Syscall {
            name: "read".into(),
        },
        &mut rm,
    )
    .unwrap();
    assert!(sb
        .execute(
            Operation::Syscall {
                name: "exec".into()
            },
            &mut rm
        )
        .is_err());

    let sig1 = match sb
        .execute(
            Operation::Sign {
                hash: "deadbeef".into(),
            },
            &mut rm,
        )
        .unwrap()
    {
        kai_os_runtime::sandbox::ExecOutcome::Signed { signature } => signature,
        _ => panic!("Sign muss Signed liefern"),
    };
    assert_eq!(sig1.len(), 64); // SHA-256 hex
                                // Deterministisch: gleicher Input -> gleiche Signatur
    let sig2 = match sb
        .execute(
            Operation::Sign {
                hash: "deadbeef".into(),
            },
            &mut rm,
        )
        .unwrap()
    {
        kai_os_runtime::sandbox::ExecOutcome::Signed { signature } => signature,
        _ => panic!(),
    };
    assert_eq!(sig1, sig2);
}

// ── AK 6: Sandbox-Isolation ─────────────────────────────────────────────

#[test]
fn sandbox_isolation_neighbor_unaffected() {
    let a = Sandbox::new("greedy", small_cap());
    let b = Sandbox::new("well-behaved", small_cap());
    let mut rm = ResourceManager::new();

    // A erschöpft sein CPU-Budget
    a.execute(Operation::Compute { cpu_ms: 100 }, &mut rm)
        .unwrap();
    assert!(a
        .execute(Operation::Compute { cpu_ms: 1 }, &mut rm)
        .is_err());

    // B arbeitet unbeeinflusst weiter
    b.execute(Operation::Compute { cpu_ms: 100 }, &mut rm)
        .unwrap();
    b.execute(Operation::Alloc { bytes: 500 }, &mut rm).unwrap();
    assert_eq!(rm.usage("greedy").cpu_ms_used, 100);
    assert_eq!(rm.usage("well-behaved").cpu_ms_used, 100);
}

// ── AK 7+8: Scheduler-Determinismus & Round-Robin ───────────────────────

#[test]
fn scheduler_deterministic_turn_sequence() {
    let mut s1 = Scheduler::new();
    for id in ["a", "b", "c"] {
        s1.register(id);
    }

    let seq1: Vec<String> = (0..9).filter_map(|_| s1.next_turn()).collect();
    assert_eq!(seq1, vec!["a", "b", "c", "a", "b", "c", "a", "b", "c"]);

    // Identische Registration -> identische Sequenz
    let mut s2 = Scheduler::new();
    for id in ["a", "b", "c"] {
        s2.register(id);
    }
    let seq2: Vec<String> = (0..9).filter_map(|_| s2.next_turn()).collect();
    assert_eq!(seq1, seq2);
}

#[test]
fn scheduler_one_pass_fairness() {
    let mut s = Scheduler::new();
    for id in ["x", "y", "z"] {
        s.register(id);
    }

    let pass1 = s.one_pass();
    assert_eq!(pass1, vec!["x", "y", "z"]);
    let pass2 = s.one_pass();
    assert_eq!(pass2, vec!["x", "y", "z"]);
    // Fairness: jeder Sandbox genau ein Turn pro Pass
    assert_eq!(pass1.len(), 3);
}

//! S23-S24 Integrationstests — Issue #110. Deterministisch (REQ-ENG-002).

use kai_os_pkg::lockfile::Lockfile;
use kai_os_pkg::manifest::{Dependency, PackageManifest};
use kai_os_pkg::registry::{PkgError, PkgRegistry};
use kai_os_pkg::version::{Version, VersionReq};
use std::str::FromStr;

fn manifest(name: &str, version: &str, deps: Vec<Dependency>) -> PackageManifest {
    PackageManifest {
        name: name.into(),
        version: Version::from_str(version).unwrap(),
        deps,
    }
}

fn dep(name: &str, req: &str) -> Dependency {
    Dependency {
        name: name.into(),
        req: VersionReq::from_str(req).unwrap(),
    }
}

// ── Versions-Modell ─────────────────────────────────────────────────────

#[test]
fn version_parse_display_and_ordering() {
    let v = Version::from_str("1.2.3").unwrap();
    assert_eq!(v.to_string(), "1.2.3");
    assert!(Version::from_str("1.2").is_err()); // genau 3 Teile
    assert!(Version::from_str("a.b.c").is_err()); // Zahlen

    assert!(Version::from_str("1.2.3").unwrap() < Version::from_str("1.10.0").unwrap());
    assert!(Version::from_str("2.0.0").unwrap() > Version::from_str("1.99.99").unwrap());

    let caret = VersionReq::from_str("^1.2.0").unwrap();
    assert!(caret.matches(Version::from_str("1.2.0").unwrap()));
    assert!(caret.matches(Version::from_str("1.9.9").unwrap()));
    assert!(!caret.matches(Version::from_str("2.0.0").unwrap())); // Major-Grenze
    assert!(!caret.matches(Version::from_str("1.1.9").unwrap())); // unter Minimum

    let exact = VersionReq::from_str("1.2.3").unwrap();
    assert!(exact.matches(Version::from_str("1.2.3").unwrap()));
    assert!(!exact.matches(Version::from_str("1.2.4").unwrap()));
}

// ── AK: Deterministische Versionswahl ──────────────────────────────────

#[test]
fn resolution_picks_highest_matching_regardless_of_publish_order() {
    // Registry A: publishes aufsteigend
    let mut ra = PkgRegistry::new();
    for v in ["1.0.0", "1.2.0", "1.9.0", "2.0.0"] {
        ra.publish(manifest("atcfs", v, vec![]), b"content")
            .unwrap();
    }
    ra.publish(
        manifest("root", "1.0.0", vec![dep("atcfs", "^1.0.0")]),
        b"root-content",
    )
    .unwrap();

    // Registry B: gleiche Pakete, umgekehrte Publish-Reihenfolge
    let mut rb = PkgRegistry::new();
    for v in ["2.0.0", "1.9.0", "1.2.0", "1.0.0"] {
        rb.publish(manifest("atcfs", v, vec![]), b"content")
            .unwrap();
    }
    rb.publish(
        manifest("root", "1.0.0", vec![dep("atcfs", "^1.0.0")]),
        b"root-content",
    )
    .unwrap();

    let pa = ra.resolve("root").unwrap();
    let pb = rb.resolve("root").unwrap();
    // ^1.0.0 wählt 1.9.0 (höchste PASSende, Major-Grenze) — in BEIDEN Registries identisch
    assert_eq!(pa, pb);
    let atcfs = pa.packages.iter().find(|p| p.name == "atcfs").unwrap();
    assert_eq!(atcfs.version, Version::from_str("1.9.0").unwrap());
    assert_eq!(atcfs.integrity, kai_os_pkg::sha_hex(b"content"));
}

// ── AK: Zyklus-Erkennung fail-closed ───────────────────────────────────

#[test]
fn dependency_cycle_rejected() {
    let mut r = PkgRegistry::new();
    r.publish(manifest("a", "1.0.0", vec![dep("b", "^1.0.0")]), b"a")
        .unwrap();
    r.publish(manifest("b", "1.0.0", vec![dep("a", "^1.0.0")]), b"b")
        .unwrap();
    assert!(matches!(
        r.resolve("a"),
        Err(PkgError::CycleDetected { .. })
    ));
}

// ── AK: Fehlende Dependency fail-closed ────────────────────────────────

#[test]
fn missing_dependency_rejected() {
    let mut r = PkgRegistry::new();
    r.publish(
        manifest("root", "1.0.0", vec![dep("geist", "^1.0.0")]),
        b"root",
    )
    .unwrap();
    assert!(matches!(
        r.resolve("root"),
        Err(PkgError::MissingDependency { name, .. }) if name == "geist"
    ));
}

// ── AK: Publish-Immutabilität ──────────────────────────────────────────

#[test]
fn publish_immutability_enforced() {
    let mut r = PkgRegistry::new();
    r.publish(manifest("p", "1.0.0", vec![]), b"original")
        .unwrap();

    // Gleiche Version, anderer Content -> abgewiesen
    assert!(matches!(
        r.publish(manifest("p", "1.0.0", vec![]), b"geaendert"),
        Err(PkgError::DuplicateNameVersion { .. })
    ));
    // Identischer Content -> idempotent ok
    r.publish(manifest("p", "1.0.0", vec![]), b"original")
        .unwrap();
    // Neue Version -> ok
    r.publish(manifest("p", "1.1.0", vec![]), b"original")
        .unwrap();
}

// ── AK: Lockfile byte-deterministisch ──────────────────────────────────

#[test]
fn lockfile_byte_deterministic() {
    let build = || {
        let mut r = PkgRegistry::new();
        r.publish(manifest("base", "1.0.0", vec![]), b"base")
            .unwrap();
        r.publish(
            manifest("net", "2.1.0", vec![dep("base", "^1.0.0")]),
            b"net",
        )
        .unwrap();
        r.publish(
            manifest(
                "app",
                "1.0.0",
                vec![dep("net", "^2.0.0"), dep("base", "^1.0.0")],
            ),
            b"app",
        )
        .unwrap();
        let plan = r.resolve("app").unwrap();
        Lockfile::from_plan("app", &plan).to_text()
    };
    let t1 = build();
    let t2 = build();
    assert_eq!(t1, t2); // byte-identisch
                        // Deterministisch sortiert nach Name
    assert!(t1.contains("app 1.0.0"));
    assert!(t1.contains("base 1.0.0"));
    assert!(t1.contains("net 2.1.0"));
    assert!(t1.contains("# kai-os lockfile (root: app)"));
}

// ── AK: Integritätsprüfung ─────────────────────────────────────────────

#[test]
fn lockfile_integrity_verification() {
    let mut r = PkgRegistry::new();
    r.publish(manifest("base", "1.0.0", vec![]), b"echter-content")
        .unwrap();
    r.publish(
        manifest("app", "1.0.0", vec![dep("base", "^1.0.0")]),
        b"app",
    )
    .unwrap();

    let plan = r.resolve("app").unwrap();
    let lf = Lockfile::from_plan("app", &plan);
    assert!(lf.verify(&r)); // Original: ok

    // Manipulierte Integrity -> scheitert
    let mut tampered = lf.clone();
    tampered.entries[0].integrity = "0".repeat(64);
    assert!(!tampered.verify(&r));

    // Manipulierte Version (nicht in Registry) -> scheitert
    let mut wrong = lf.clone();
    wrong.entries[0].version = Version::from_str("9.9.9").unwrap();
    assert!(!wrong.verify(&r));
}

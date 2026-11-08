use atc_findings::{Finding, FindingRegistry, FindingSeverity};
use std::fs;

fn main() {
    let mut finding = Finding::new(
        "FND-SCHEMA-001",
        "A-TownChain-Okosystems/atc-engineering",
        "SCHEMA_CONFORMANCE_TEST",
        "test",
        FindingSeverity::Medium,
        "JSON Schema conformance fixture",
    )
    .expect("valid finding fixture");
    finding.path = Some("crates/atc-findings/src/lib.rs".into());
    finding.root_cause = r#"quote " and backslash \ "#.into();
    finding.implementation_plan = "validate emitted JSON against the committed schema".into();

    let mut registry = FindingRegistry::default();
    registry.register(finding).expect("unique fixture id");
    let root = std::env::temp_dir().join(format!(
        "atc-findings-schema-fixture-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    registry.write_json(&root).expect("write fixture");
    let output =
        fs::read_to_string(root.join(".atc/findings/registry.json")).expect("read fixture");
    print!("{output}");
    fs::remove_dir_all(root).expect("remove fixture directory");
}

use atc_core::EvidenceId;
use atc_evidence::{EvidenceRecord, EvidenceStatus};

fn main() {
    let mut record = EvidenceRecord::new(
        EvidenceId::new("EVD-2026-123").expect("valid fixture id"),
        "SCHEMA_CONFORMANCE_TEST",
        "A-TownChain-Okosystems/atc-engineering",
        "0123456789abcdef0123456789abcdef01234567",
        "schema-conformance-test",
        EvidenceStatus::Pass,
    );
    record.integrity_digest = "0123456789abcdef".into();
    record.provenance = r#"schema "conformance" fixture \ "#.into();
    println!("{}", record.to_json());
}

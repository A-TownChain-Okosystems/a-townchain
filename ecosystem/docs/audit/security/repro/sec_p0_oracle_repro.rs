//! SEC-P0-Repro: Mock-Verifier im Oracle-Konsenspfad (Audit 2026-11-05).
use atc_oracle::verification::{SignatureVerifierAdapter, StandardVerifierAdapter};

#[test]
fn repro_forged_signature_with_valid_prefix_is_accepted() {
    let verifier = StandardVerifierAdapter::new(false);
    let accepted = verifier
        .verify_signature(b"ORACLE:ATC-USD:price=999999", b"VALID_GARBAGE", b"FAKEKEY")
        .unwrap();
    assert!(accepted, "MOCK akzeptiert gefaelschte Signatur");
    println!("NACHWEIS SEC-P0-002: Signatur 'VALID_GARBAGE' gilt als gueltig (Mock im 'Standard'-Adapter)");
    let rejected = verifier.verify_signature(b"x", b"GARBAGE", b"KEY").unwrap();
    assert!(!rejected);
    println!("KONTROLLE: Verhalten haengt ausschliesslich am Praefix VALID");
}

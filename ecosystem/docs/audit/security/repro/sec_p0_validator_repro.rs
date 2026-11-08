//! SEC-P0-Repro: Validator-"Signatur" ist sha256(Prefix+Klartext-Secret) (Audit 2026-11-05).
use atc_algorithm::hash::atc_hash;
use atc_node::validator::{Validator, ValidatorProposal, ValidatorSet};

fn set() -> (ValidatorSet, String) {
    let secret = "geheimes-validator-passwort";
    let mut vs = ValidatorSet::new();
    let v = Validator { id: 1, pubkey: "pub".into(), voting_power: 100, is_active: true };
    vs.add(v).unwrap();
    (vs, secret.to_string())
}

#[test]
fn repro_signature_is_hash_with_plaintext_secret() {
    let (_vs, secret) = set();
    let sig = ValidatorProposal::compute_signature(7, 1, 42, &secret);
    // Nachweis 1: Signatur ist exakt sha256("PROPOSE:7:1:42:<secret>") als Hex
    let expected = hex_encode(&atc_hash(format!("PROPOSE:7:1:42:{}", secret).as_bytes()));
    assert_eq!(sig, expected, "Signatur == sha256(Prefix+Secret)");
    println!("NACHWEIS SEC-P0-003a: signature = sha256(\"PROPOSE:7:1:42:<secret>\") — Hash, keine Signatur");
    // Nachweis 2: verify() benoetigt das Secret des Signierers (keine Public-Key-Verifikation)
    let vs2 = { let (mut v, _) = set(); v };
    let prop = ValidatorProposal::create(7, vs2.get(1).unwrap(), 42, &secret).unwrap();
    assert!(prop.verify(&vs2, &secret).is_ok());
    assert!(prop.verify(&vs2, "falsches-secret").is_err());
    println!("NACHWEIS SEC-P0-003b: verify() verlangt das Signierer-Secret -> keine oeffentliche Verifizierbarkeit");
    // Nachweis 3: Jeder mit Secret-Kenntnis kann beliebige Proposals faelschen
    let forged = ValidatorProposal::compute_signature(999, 1, 12345, &secret);
    println!("NACHWEIS SEC-P0-003c: Secret-Inhaber erzeugt gueltige Signatur fuer beliebige Hoehe/Block (forged={})", &forged[..16]);
}

fn hex_encode(b: &[u8]) -> String { b.iter().map(|x| format!("{:02x}", x)).collect() }

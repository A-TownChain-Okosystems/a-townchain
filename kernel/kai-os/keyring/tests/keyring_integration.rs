//! S16-S18 Integrationstests — Issue #107 Akzeptanzkriterien.
//! Deterministisch: Seeds statt RNG, keine Wall-Clock (REQ-ENG-002).

use kai_os_keyring::keyring::{verify, KeyKind, Keyring, KeyringError};

const SEED_A: [u8; 32] = [11u8; 32];
const SEED_B: [u8; 32] = [77u8; 32];

fn ring() -> Keyring {
    let mut k = Keyring::new();
    k.import_seed("node-key", KeyKind::NodeIdentity, &SEED_A)
        .unwrap();
    k.import_seed("agent-key", KeyKind::AgentSigning, &SEED_B)
        .unwrap();
    k
}

// ── AK 2: Fail-closed bei unbekannten Keys ─────────────────────────────

#[test]
fn unknown_key_fail_closed() {
    let k = ring();
    assert!(matches!(
        k.sign("gibtsnicht", b"msg"),
        Err(KeyringError::UnknownKey(_))
    ));
    assert!(matches!(
        k.public_key("gibtsnicht"),
        Err(KeyringError::UnknownKey(_))
    ));
}

// ── AK 3: Deterministisches Signieren ──────────────────────────────────

#[test]
fn deterministic_signatures() {
    let k1 = ring();
    let k2 = ring(); // identische Seeds -> identische Keys
    let s1 = k1.sign("node-key", b"block-42").unwrap();
    let s2 = k2.sign("node-key", b"block-42").unwrap();
    assert_eq!(s1.to_vec(), s2.to_vec()); // gleicher Key+Msg -> identische Signatur

    // Andere Nachricht -> andere Signatur
    let s3 = k1.sign("node-key", b"block-43").unwrap();
    assert_ne!(s1.to_vec(), s3.to_vec());
}

// ── AK 4: Key-Isolation ────────────────────────────────────────────────

#[test]
fn key_isolation_verification() {
    let k = ring();
    let pub_node = k.public_key("node-key").unwrap();
    let pub_agent = k.public_key("agent-key").unwrap();
    assert_ne!(pub_node, pub_agent);

    let sig = k.sign("node-key", b"payload").unwrap();
    assert!(verify(&pub_node, b"payload", &sig)); // zugehöriger Key: ok
    assert!(!verify(&pub_agent, b"payload", &sig)); // fremder Key: scheitert
    assert!(!verify(&pub_node, b"anderes", &sig)); // manipulierte Msg: scheitert

    // Agent-Key signiert unabhängig
    let sig_a = k.sign("agent-key", b"payload").unwrap();
    assert!(verify(&pub_agent, b"payload", &sig_a));
    assert!(!verify(&pub_node, b"payload", &sig_a));
}

// ── AK 5: Kompatibilität mit kai-os-network ────────────────────────────

#[test]
fn derivation_matches_network_keypair() {
    let mut k = Keyring::new();
    let pub_kr = k
        .import_seed("test", KeyKind::NodeIdentity, &SEED_A)
        .unwrap();

    // Gleicher Seed im kai-os-network Keypair muss den identischen Public Key liefern
    let kp = kai_os_network::peer::Keypair::from_seed(&SEED_A);
    let pub_net: [u8; 32] = {
        let hex_id = kp.peer_id(); // Hex des Public Keys
        let bytes: Vec<u8> = (0..hex_id.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_id[i..i + 2], 16).unwrap())
            .collect();
        let arr: [u8; 32] = bytes.try_into().unwrap();
        arr
    };
    assert_eq!(pub_kr, pub_net); // eine Quelle der Wahrheit für Ed25519-Derivation

    // Signaturen sind kreuzkompatibel: Keyring-Sig verifiziert via Network-Keypair-Pub
    let sig = k.sign("test", b"cross-check").unwrap();
    assert!(verify(&pub_net, b"cross-check", &sig));
}

// ── AK 6: Revoke ───────────────────────────────────────────────────────

#[test]
fn revoke_makes_key_unusable() {
    let mut k = ring();
    let pub_agent = k.public_key("agent-key").unwrap();
    let sig = k.sign("agent-key", b"vorher").unwrap();
    assert!(verify(&pub_agent, b"vorher", &sig));

    k.revoke("agent-key").unwrap();

    // Danach: fail-closed
    assert!(matches!(
        k.sign("agent-key", b"nachher"),
        Err(KeyringError::UnknownKey(_))
    ));
    assert!(matches!(
        k.public_key("agent-key"),
        Err(KeyringError::UnknownKey(_))
    ));
    assert!(matches!(
        k.revoke("agent-key"),
        Err(KeyringError::UnknownKey(_))
    ));

    // Andere Keys unberührt
    assert!(k.sign("node-key", b"x").is_ok());
    assert_eq!(k.list().len(), 1);
}

// ── AK 7: Rotation ────────────────────────────────────────────────────

#[test]
fn rotation_replaces_secret_old_sigs_still_verifiable() {
    let mut k = ring();
    let old_pub = k.public_key("node-key").unwrap();
    let old_sig = k.sign("node-key", b"alter-block").unwrap();

    const SEED_NEW: [u8; 32] = [99u8; 32];
    let new_pub = k.rotate("node-key", &SEED_NEW).unwrap();

    assert_ne!(old_pub, new_pub); // neuer Public Key
    assert_eq!(k.public_key("node-key").unwrap(), new_pub);

    // Neue Signaturen nur mit neuem Public Key verifizierbar
    let new_sig = k.sign("node-key", b"neuer-block").unwrap();
    assert!(verify(&new_pub, b"neuer-block", &new_sig));
    assert!(!verify(&old_pub, b"neuer-block", &new_sig));

    // Alte Signaturen bleiben mit dem extern aufbewahrten alten Public Key prüfbar
    assert!(verify(&old_pub, b"alter-block", &old_sig));
    assert!(!verify(&new_pub, b"alter-block", &old_sig));
}

// ── AK 1: Boundary — die API veröffentlicht kein Secret ───────────────

#[test]
fn api_surface_publishes_no_secret_material() {
    let mut k = Keyring::new();
    let pub_a = k
        .import_seed("nur", KeyKind::NodeIdentity, &SEED_A)
        .unwrap();

    // list() liefert 32-Byte PUBLIC Keys (keine 32-Byte Seeds)
    for (id, public, kind) in k.list() {
        assert_eq!(public.len(), 32);
        assert_eq!(public, pub_a); // Public, nicht Seed
        assert_eq!(id, "nur");
        assert_eq!(kind, KeyKind::NodeIdentity);
    }
    // sign() liefert 64-Byte Signaturen, keine Seeds
    let sig = k.sign("nur", b"x").unwrap();
    assert_eq!(sig.len(), 64);
    // Duplikat-Import abgewiesen
    assert!(matches!(
        k.import_seed("nur", KeyKind::NodeIdentity, &SEED_A),
        Err(KeyringError::DuplicateKey(_))
    ));
}

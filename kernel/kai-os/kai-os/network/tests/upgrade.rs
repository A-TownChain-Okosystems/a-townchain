//! G2-D (Issue #112): Protokoll-Upgrade & Versionierung.
//! Deterministisch: feste Versionen, feste Roots — kein RNG, keine Uhr.

use kai_os_network::peer::Keypair;
use kai_os_network::tcp::TcpPeer;
use kai_os_network::upgrade::{
    negotiate, Hello, ProtocolSupport, UpgradeError, VersionedEnvelope, MIN_COMPATIBLE,
    PROTOCOL_VERSION,
};
use std::time::Duration;

fn hello(current: u32, min: u32, root: &str) -> Hello {
    Hello {
        support: ProtocolSupport { current, min },
        last_verified_root: root.into(),
    }
}

// ── AK: Deterministische Verhandlung ────────────────────────────────────

#[test]
fn negotiation_picks_highest_shared_version() {
    // Beide auf v2: gemeinsame Sprache v2
    let n = negotiate(&hello(2, 1, "root-a"), &hello(2, 1, "root-b")).unwrap();
    assert_eq!(n.version, 2);
    // Gemischt v2/v1 (min 1): gemeinsam v1 — hoechste SHARED, deterministisch
    let n = negotiate(&hello(2, 1, "a"), &hello(1, 1, "b")).unwrap();
    assert_eq!(n.version, 1);
    // Reihenfolge der Argumente aendert das Ergebnis nicht (Kommutativitaet)
    let x = negotiate(&hello(2, 1, "a"), &hello(1, 1, "b")).unwrap();
    let y = negotiate(&hello(1, 1, "b"), &hello(2, 1, "a")).unwrap();
    assert_eq!(x, y);
}

#[test]
fn incompatible_peers_rejected_with_resync_hint() {
    // wir v2 (min 2), gegenueber v1 (min 1): keine Ueberlappung
    let err = negotiate(&hello(2, 2, "unser-root"), &hello(1, 1, "ihr-root"));
    match err {
        Err(UpgradeError::Incompatible {
            ours,
            theirs,
            resync_root_hint,
        }) => {
            assert_eq!(ours, (2, 2));
            assert_eq!(theirs, (1, 1));
            // Snapshot-Return: der Abgewiesene erhaelt den letzten verifizierten
            // Root des Gegenuebers als RESYNC-Anhaltspunkt — kein Best-Effort
            assert_eq!(resync_root_hint, "ihr-root");
        }
        other => panic!("Incompatible erwartet, gefunden: {other:?}"),
    }
    // Und unlogische Supports werden abgewiesen (min > current)
    assert!(matches!(
        negotiate(&hello(1, 5, "a"), &hello(2, 1, "b")),
        Err(UpgradeError::MalformedSupport { .. })
    ));
}

// ── AK: Versionierte Envelopes — fail-closed, kein Blind-Parsing ───────

#[test]
fn versioned_envelope_rejects_unknown_versions_without_parsing_payload() {
    let support = ProtocolSupport::ours();
    // Aktuelle Version: OK
    let env = VersionedEnvelope::encode(PROTOCOL_VERSION, b"neue-nachricht");
    let payload = env.decode_checked(&support).unwrap();
    assert_eq!(payload, b"neue-nachricht".to_vec().as_slice());
    // Zukuenftige Version: abgewiesen, Payload wird NICHT interpretiert
    let future = VersionedEnvelope::encode(PROTOCOL_VERSION + 1, b"boeser-vorschlag");
    let err = future.decode_checked(&support);
    assert!(matches!(
        err,
        Err(UpgradeError::UnknownEnvelopeVersion { .. })
    ));
    // Veraltete Version unter min: abgewiesen
    let ancient = VersionedEnvelope::encode(MIN_COMPATIBLE.saturating_sub(1), b"ur-alt");
    assert!(matches!(
        ancient.decode_checked(&support),
        Err(UpgradeError::EnvelopeTooOld { .. })
    ));
}

#[test]
fn versioned_envelope_wire_roundtrip() {
    let env = VersionedEnvelope::encode(2, b"wire-test");
    let wire = env.to_wire();
    let back = VersionedEnvelope::from_wire(&wire).unwrap();
    assert_eq!(back, env);
    // Muell auf der Leitung: fail-closed
    assert!(VersionedEnvelope::from_wire(b"kein json").is_err());
}

// ── AK: Upgrade-Handshake ueber echte TCP-Verbindung ───────────────────

#[test]
fn versioned_handshake_over_tcp_with_rejection_and_resync() {
    let alice = Keypair::from_seed(&[7u8; 32]);
    let _alice_id = alice.peer_id();
    let bob_id = Keypair::from_seed(&[8u8; 32]).peer_id();

    // Server: aktueller Knoten (v2, min 1) mit letzter verifizierter Grenze
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    std::thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut peer = TcpPeer::from_stream(stream, Duration::from_secs(5)).unwrap();
        // 1. Empfangenes Hello pruefen (Verhandlung)
        let hello_bytes = peer.receive().unwrap();
        let remote: Hello = serde_json::from_slice(&hello_bytes).unwrap();
        let ours = Hello {
            support: ProtocolSupport::ours(),
            last_verified_root: "unser-genesis-root".into(),
        };
        match negotiate(&ours, &remote) {
            Ok(n) => {
                // 2. Ab hier versionierte Envelopes ueber die signierte Session
                peer.send(&serde_json::to_vec(&ours).unwrap()).unwrap();
                let env = VersionedEnvelope::encode(
                    n.version,
                    format!("negotiated-v{}", n.version).as_bytes(),
                );
                peer.send(&env.to_wire()).unwrap();
            }
            Err(e) => {
                // Abweisung MIT Resync-Hint — Peer kann auf unsere Grenze resyncen
                let rejection = serde_json::to_vec(&format!("REJECT:{e}:resync={}", e)).unwrap();
                peer.send(&rejection).unwrap();
            }
        }
    });

    let mut client = TcpPeer::connect(&addr, Duration::from_secs(5)).unwrap();
    // Kompatibler Client (v1, min 1) — Verhandlung muss v1 ergeben
    client
        .send(&serde_json::to_vec(&hello(1, 1, &bob_id)).unwrap())
        .unwrap();
    let server_hello: Hello = serde_json::from_slice(&client.receive().unwrap()).unwrap();
    let negotiated = negotiate(&hello(1, 1, &bob_id), &server_hello).unwrap();
    assert_eq!(negotiated.version, 1);
    // Danach: versionierte Envelope auf der ausgehandelten Version
    let wire = client.receive().unwrap();
    let env = VersionedEnvelope::from_wire(&wire).unwrap();
    assert_eq!(env.version, 1);
    assert_eq!(env.payload, b"negotiated-v1".to_vec().as_slice());
}

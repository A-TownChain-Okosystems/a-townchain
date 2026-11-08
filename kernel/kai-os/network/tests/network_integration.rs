//! S07-S09 Integrationstests — Issue #104 Akzeptanzkriterien.
//! Deterministisch: Keypairs aus Seeds, keine Wall-Clock, kein RNG (REQ-ENG-002).

use kai_os_network::auth::{self, AuthRegistry, ChallengeGenerator};
use kai_os_network::discovery::{Discovery, DiscoveryMessage};
use kai_os_network::peer::{verify_signature, Keypair, PeerInfo, PeerStore};
use kai_os_network::session::SecureSession;
use kai_os_network::transport::{frame, parse_frame, TransportError};

fn keypair_a() -> Keypair {
    Keypair::from_seed(&[1u8; 32])
}
fn keypair_b() -> Keypair {
    Keypair::from_seed(&[7u8; 32])
}

// ── AK 1: PeerId-Derivation + Signatur ──────────────────────────────────

#[test]
fn peer_id_deterministic_and_signatures_verify() {
    let a = keypair_a();
    let a2 = keypair_a();
    assert_eq!(a.peer_id(), a2.peer_id()); // gleicher Seed -> gleiche ID
    assert_ne!(a.peer_id(), keypair_b().peer_id());

    let sig = a.sign(b"hello");
    assert!(verify_signature(&a.public_bytes(), b"hello", &sig));
    assert!(!verify_signature(&a.public_bytes(), b"tampered", &sig));
    assert!(!verify_signature(
        &keypair_b().public_bytes(),
        b"hello",
        &sig
    ));

    // Ed25519 ist deterministisch: gleiche Nachricht -> gleiche Signatur
    assert_eq!(a.sign(b"hello").to_vec(), sig.to_vec());
}

// ── AK 2+3: Challenge-Response-Handshake + Replay-Schutz ────────────────

#[test]
fn handshake_success_and_tamper_fails() {
    let a = keypair_a();
    let b = keypair_b();
    let mut gen = ChallengeGenerator::new();
    let mut reg = AuthRegistry::new();

    let ch = gen.next(&a.peer_id(), &b.peer_id());
    let resp = auth::respond(&ch, &b);
    reg.verify(&ch, &resp, &b.public_bytes()).unwrap();

    // Manipulierte Signatur -> BadSignature
    let mut bad = resp.clone();
    bad.signature[0] ^= 0xFF;
    let ch2 = gen.next(&a.peer_id(), &b.peer_id());
    let bad_resp = auth::respond(&ch2, &b);
    let mut tampered = bad_resp.clone();
    tampered.signature = bad.signature;
    assert!(reg.verify(&ch2, &tampered, &b.public_bytes()).is_err());

    // Fremder Peer antwortet mit eigenem Key -> Signatur passt nicht
    let ch3 = gen.next(&a.peer_id(), &b.peer_id());
    let impostor = auth::respond(&ch3, &keypair_a());
    assert!(reg.verify(&ch3, &impostor, &b.public_bytes()).is_err());
}

#[test]
fn nonce_replay_rejected() {
    let a = keypair_a();
    let b = keypair_b();
    let mut gen = ChallengeGenerator::new();
    let mut reg = AuthRegistry::new();

    let ch1 = gen.next(&a.peer_id(), &b.peer_id());
    let resp1 = auth::respond(&ch1, &b);
    reg.verify(&ch1, &resp1, &b.public_bytes()).unwrap();

    // Dieselbe Nonce nochmal -> Replay
    assert!(reg.verify(&ch1, &resp1, &b.public_bytes()).is_err());

    // Zurückliegende Nonce -> Replay
    let ch2 = gen.next(&a.peer_id(), &b.peer_id());
    let resp2 = auth::respond(&ch2, &b);
    let mut older = resp2.clone();
    older.nonce = 1;
    assert!(reg.verify(&ch2, &older, &b.public_bytes()).is_err());
}

// ── AK 4+5: Secure Session + Signierte Envelopes ───────────────────────

#[test]
fn session_seq_replay_rejected() {
    let a = keypair_a();
    let b = keypair_b();
    let sid = auth::session_id(&a.peer_id(), &b.peer_id(), 1);
    let mut s_a = SecureSession::new(sid.clone(), &a.peer_id(), &b.peer_id(), &b.public_bytes());
    let mut s_b = SecureSession::new(sid, &b.peer_id(), &a.peer_id(), &a.public_bytes());

    let m1 = s_a.send(&a, b"tx-batch-1");
    let m2 = s_a.send(&a, b"tx-batch-2");
    assert_eq!(s_b.receive(&m1).unwrap(), b"tx-batch-1");
    assert_eq!(s_b.receive(&m2).unwrap(), b"tx-batch-2");

    // Replay von m1 (seq 1) -> abgewiesen
    assert!(s_b.receive(&m1).is_err());

    // Bidirektional
    let r1 = s_b.send(&b, b"block-announce");
    assert_eq!(s_a.receive(&r1).unwrap(), b"block-announce");
}

#[test]
fn envelope_payload_tamper_breaks_signature() {
    let a = keypair_a();
    let b = keypair_b();
    let sid = auth::session_id(&a.peer_id(), &b.peer_id(), 9);
    let mut s_a = SecureSession::new(sid.clone(), &a.peer_id(), &b.peer_id(), &b.public_bytes());
    let mut s_b = SecureSession::new(sid, &b.peer_id(), &a.peer_id(), &a.public_bytes());

    let mut signed = s_a.send(&a, b"original-payload");
    signed.envelope.payload = b"manipuliert".to_vec(); // MITM
    assert!(s_b.receive(&signed).is_err());

    // Andere Session-ID -> WrongSession
    let signed2 = s_a.send(&a, b"payload");
    let mut hijack = signed2.clone();
    hijack.envelope.session_id = "falsche-session".into();
    assert!(s_b.receive(&hijack).is_err());
}

// ── AK 6: Framing ──────────────────────────────────────────────────────

#[test]
fn framing_roundtrip_and_fail_closed() {
    let payload = b"peer-sync-daten";
    let framed = frame(payload);
    let (parsed, consumed) = parse_frame(&framed).unwrap();
    assert_eq!(parsed, payload);
    assert_eq!(consumed, framed.len());

    // Trunziert -> Incomplete
    assert!(matches!(
        parse_frame(&framed[..framed.len() - 2]),
        Err(TransportError::Incomplete { .. })
    ));
    // Nur Header-Fragment -> Incomplete
    assert!(matches!(
        parse_frame(&framed[..2]),
        Err(TransportError::Incomplete { .. })
    ));
    // Deklarierte Länge > verfügbar -> Incomplete (fail-closed, kein Vertrauen)
    let mut oversized = Vec::new();
    oversized.extend_from_slice(&1000u32.to_be_bytes());
    oversized.push(b'x');
    assert!(matches!(
        parse_frame(&oversized),
        Err(TransportError::Incomplete { .. })
    ));
}

// ── AK 7: Discovery ────────────────────────────────────────────────────

fn info(peer_id: &str, tick: u64) -> PeerInfo {
    PeerInfo {
        peer_id: peer_id.into(),
        addr: format!("{peer_id}.atc:5000"),
        caps: vec!["consensus".into()],
        last_seen_tick: tick,
    }
}

#[test]
fn discovery_announce_ping_peerlist_eviction() {
    let mut d = Discovery::new("self");

    // Announce -> Store wächst; Duplikat -> Dedup (upsert, kein Zweit-Eintrag)
    d.handle(
        DiscoveryMessage::Announce {
            info: info("peer-1", 0),
        },
        0,
    );
    d.handle(
        DiscoveryMessage::Announce {
            info: info("peer-1", 5),
        },
        5,
    );
    assert_eq!(d.store().len(), 1);
    assert_eq!(d.store().get("peer-1").unwrap().last_seen_tick, 5);

    d.handle(
        DiscoveryMessage::Announce {
            info: info("peer-2", 0),
        },
        0,
    );
    assert_eq!(d.store().len(), 2);

    // Ping -> Pong (Antwort erforderlich)
    let reply = d.handle(
        DiscoveryMessage::Ping {
            from_peer_id: "peer-1".into(),
            tick: 7,
        },
        7,
    );
    assert!(matches!(
        reply,
        Some(DiscoveryMessage::Pong { tick: 7, .. })
    ));

    // PeerList-Merge
    d.handle(
        DiscoveryMessage::PeerList {
            from_peer_id: "peer-1".into(),
            peers: vec![info("peer-3", 0)],
        },
        7,
    );
    assert_eq!(d.store().len(), 3);

    // Deterministische Sortierung
    let list = d.peer_list();
    assert_eq!(list[0].peer_id, "peer-1");
    assert_eq!(list[2].peer_id, "peer-3");

    // Stale-Eviction: nur peer-2 (Tick 0) ueberschreitet max_age bei now=15:
    // peer-1: 15-5=10 <= 14 bleibt; peer-2: 15-0=15 > 14 raus; peer-3: 15-7=8 bleibt
    let evicted = d.store_mut().evict_stale(15, 14);
    assert_eq!(evicted, vec!["peer-2"]);
    assert_eq!(d.store().len(), 2);
}

#[test]
fn peer_store_deterministic_ordering() {
    let mut store = PeerStore::new();
    for id in ["c", "a", "b"] {
        store.upsert(info(id, 0));
    }
    let ids: Vec<&str> = store.all().iter().map(|p| p.peer_id.as_str()).collect();
    assert_eq!(ids, vec!["a", "b", "c"]); // sortiert, nicht Einfüge-Order
}

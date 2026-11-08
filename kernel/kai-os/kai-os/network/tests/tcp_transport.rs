//! G2-C (Issue #112): Echter TCP-Transport über Loopback.
//! Feste Seeds, feste Ports (ephemeral via :0) — deterministisch (REQ-ENG-002).

use kai_os_network::auth::{AuthRegistry, ChallengeGenerator};
use kai_os_network::peer::Keypair;
use kai_os_network::session::{SecureSession, SignedEnvelope};
use kai_os_network::tcp::TcpPeer;
use kai_os_network::transport::MAX_FRAME;
use std::io::Write as _;
use std::net::TcpListener;
use std::sync::mpsc;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(5);

/// Startet einen Listener auf 127.0.0.1:0 und gibt (Adresse, Empfänger für Ergebnisse) zurück.
/// Der Worker-Akzept-Loop läuft in einem Thread und verarbeitet jede Verbindung
/// mit derselben Funktion (sequenziell — deterministisch).
fn start_server<F>(handler: F) -> (String, mpsc::Receiver<String>)
where
    F: Fn(TcpPeer) -> String + Send + Sync + 'static,
{
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let (tx, rx) = mpsc::channel();
    let handler = std::sync::Arc::new(handler);
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(stream) = stream else { break };
            let peer = TcpPeer::from_stream(stream, TIMEOUT).unwrap();
            let out = handler(peer);
            let _ = tx.send(out);
        }
    });
    (addr, rx)
}

fn payload(n: usize) -> Vec<u8> {
    (0..n).map(|i| (i % 251) as u8).collect()
}

// ── AK: Frame-Roundtrip über echte Sockets ─────────────────────────────

#[test]
fn tcp_frame_roundtrip_real_socket() {
    let (addr, rx) = start_server(|mut peer| {
        let got = peer.receive().unwrap();
        let echo = format!("echo:{}", got.len());
        peer.send(echo.as_bytes()).unwrap();
        "ok".into()
    });
    let mut client = TcpPeer::connect(&addr, TIMEOUT).unwrap();
    let data = payload(10_000);
    client.send(&data).unwrap();
    let echo = client.receive().unwrap();
    assert_eq!(echo, format!("echo:{}", 10_000).as_bytes());
    let r = rx.recv_timeout(TIMEOUT).unwrap();
    assert_eq!(r, "ok");
}

// ── AK: Mehrere Frames im Stream — Disziplin über Nachrichten hinweg ──

#[test]
fn tcp_multiple_frames_stay_separate() {
    let (addr, rx) = start_server(|mut peer| {
        let a = peer.receive().unwrap();
        let b = peer.receive().unwrap();
        let c = peer.receive().unwrap();
        let total = a.len() + b.len() + c.len();
        peer.send(&total.to_le_bytes()).unwrap();
        "ok".into()
    });
    let mut client = TcpPeer::connect(&addr, TIMEOUT).unwrap();
    client.send(b"eins").unwrap();
    client.send(&payload(300)).unwrap();
    client.send(b"drei".to_vec().as_slice()).unwrap();
    // Alle 3 gesendet, dann 1 Antwortframe mit der Summe — Frame-Grenzen bleiben erhalten
    let resp = client.receive().unwrap();
    let total = u64::from_le_bytes(resp[..8].try_into().unwrap());
    assert_eq!(total as usize, 4 + 300 + 4);
    assert_eq!(rx.recv_timeout(TIMEOUT).unwrap(), "ok");
}

// ── AK: Oversized-Frame — fail-closed ─────────────────────────────────

#[test]
fn tcp_oversized_frame_fail_closed() {
    let (addr, rx) = start_server(|mut peer| {
        let r = peer.receive();
        // Oversized muss als Fehler zurückkommen (nicht OOM, nicht still)
        match r {
            Err(kai_os_network::tcp::TcpError::Frame(
                kai_os_network::transport::TransportError::Oversized { .. },
            )) => "rejected".into(),
            _ => "WRONG".into(),
        }
    });
    // Böswilliger Client: Header deklariert > MAX_FRAME
    let mut raw = std::net::TcpStream::connect(&addr).unwrap();
    let mut evil = (MAX_FRAME as u32 + 1).to_be_bytes().to_vec();
    evil.extend_from_slice(&[0u8; 16]); // etwas Payload
    raw.write_all(&evil).unwrap();
    let _ = raw.flush();
    let r = rx.recv_timeout(TIMEOUT).unwrap();
    assert_eq!(r, "rejected");
}

// ── AK: Ed25519-Handshake über echte Leitung ───────────────────────────

#[test]
fn tcp_secure_session_handshake_over_the_wire() {
    // Alice und Bob: deterministische Keys aus Seeds (kein RNG)
    let alice = Keypair::from_seed(&[1u8; 32]);
    let bob = Keypair::from_seed(&[2u8; 32]);
    let alice_id = alice.peer_id();
    let bob_id = bob.peer_id();
    let alice_pub = alice.public_bytes();
    let bob_pub = bob.public_bytes();
    let sid = kai_os_network::auth::session_id(&alice_id, &bob_id, 1);
    // Klone für die Server-Seite (Closure movt ihre Umgebung)
    let (alice_id_c, bob_id_c, sid_c) = (alice_id.clone(), bob_id.clone(), sid.clone());

    let (addr, rx) = start_server(move |mut peer| {
        // Bob: Challenge empfangen, signieren, zurücksenden
        let challenge_bytes = peer.receive().unwrap();
        let challenge: kai_os_network::auth::Challenge =
            serde_json::from_slice(&challenge_bytes).unwrap();
        let response = kai_os_network::auth::respond(&challenge, &bob);
        peer.send(&serde_json::to_vec(&response).unwrap()).unwrap();
        // Danach signierte Envelope-Verbindung (Session aus Bobs Sicht)
        let mut session = SecureSession::new(sid_c.clone(), &bob_id_c, &alice_id_c, &alice_pub);
        let signed_bytes = peer.receive().unwrap();
        let signed: SignedEnvelope = serde_json::from_slice(&signed_bytes).unwrap();
        match session.receive(&signed) {
            Ok(payload) => {
                peer.send(&payload).unwrap();
                "verified".into()
            }
            Err(_) => "WRONG".into(),
        }
    });

    let mut client = TcpPeer::connect(&addr, TIMEOUT).unwrap();
    // Alice: deterministische Challenge erzeugen und über die Leitung schicken
    let mut gen = ChallengeGenerator::new();
    let challenge = gen.next(&alice_id, &bob_id);
    client
        .send(&serde_json::to_vec(&challenge).unwrap())
        .unwrap();
    let resp_bytes = client.receive().unwrap();
    let resp: kai_os_network::auth::ChallengeResponse =
        serde_json::from_slice(&resp_bytes).unwrap();

    // AuthRegistry verifiziert die Ed25519-Signatur (fail-closed)
    let mut registry = AuthRegistry::new();
    registry
        .verify(&challenge, &resp, &bob_pub)
        .expect("Ed25519-Antwort muss über echte TCP-Verbindung verifizieren");

    // Signierte Session-Nachricht über die Leitung
    let mut session = SecureSession::new(sid, &alice_id, &bob_id, &bob_pub);
    let signed = session.send(&alice, b"handshake-ueber-tcp");
    client.send(&serde_json::to_vec(&signed).unwrap()).unwrap();
    let echoed = client.receive().unwrap();
    assert_eq!(echoed, b"handshake-ueber-tcp".to_vec());
    assert_eq!(rx.recv_timeout(TIMEOUT).unwrap(), "verified");
}

// ── AK: Connection-Churn — Verbindungen kommen und gehen ───────────────

#[test]
fn tcp_connection_churn_reconnect() {
    let (addr, rx) = start_server(|mut peer| {
        let got = peer.receive().unwrap_or_default();
        peer.send(&got).unwrap();
        "ok".into()
    });
    // Drei separate Verbindungen hintereinander (Churn): jede ändert die Payload
    for i in 1..=3u8 {
        let mut client = TcpPeer::connect(&addr, TIMEOUT).unwrap();
        client.send(&[i]).unwrap();
        let echo = client.receive().unwrap();
        assert_eq!(echo, vec![i]);
        drop(client); // Verbindung weg — Server akzeptiert die nächste
    }
    assert_eq!(rx.recv_timeout(TIMEOUT).unwrap(), "ok");
    assert_eq!(rx.recv_timeout(TIMEOUT).unwrap(), "ok");
    assert_eq!(rx.recv_timeout(TIMEOUT).unwrap(), "ok");
}

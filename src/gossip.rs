// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Gossip-MVP (SCR-0118, F-139): pull-basierte Ketten-Synchronisation
//! zwischen Node-Instanzen ueber TCP. Wire-Protokoll: "STATUS" ->
//! "height best_hash"; "BLOCKS <from>" -> Blockliste (Bloecke mit ';',
//! Felder height/prev_hash/payload/hash mit '|'). Vor jeder Adoption wird
//! die Kandidaten-Kette VOLL verifiziert (Hashes, Hoehen, Verkettung) und
//! die Genesis-Bindung geprueft.
//! Ehrlichkeit: NUR Pull (kein Push an Peers), keine Periodik, keine
//! Signaturen, keine Auth/TLS, Devnet-Platzhalter-Hashes (FNV-1a).

use crate::chain::{Block, Chain};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct SyncReport {
    pub adopted: bool,
    pub neue_hoehe: u64,
    pub grund: String,
}

/// Gossip-Dienst: Dauerdienst auf TCP, bedient STATUS und BLOCKS-Abfragen.
pub fn serve_gossip(addr: &str, kette: Arc<Mutex<Chain>>) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let mut s = stream?;
        let mut reader = BufReader::new(s.try_clone()?);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let befehl = line.trim().to_string();
        let k = kette.lock().expect("Chain-Lock vergiftet");
        if befehl == "STATUS" {
            writeln!(s, "{} {}", k.height(), k.best_hash())?;
        } else if let Some(rest) = befehl.strip_prefix("BLOCKS ") {
            let from: usize = rest.trim().parse().unwrap_or(0);
            let teile: Vec<String> = k
                .blocks_from(from)
                .iter()
                .map(|b| format!("{}|{}|{}|{}", b.height, b.prev_hash, b.payload, b.hash))
                .collect();
            writeln!(s, teile.join(";"))?;
        } else {
            writeln!(s, "ERR")?;
        }
    }
    Ok(())
}

fn peer_antwort(peer_addr: &str, befehl: &str) -> Result<String, String> {
    let mut s = TcpStream::connect(peer_addr)
        .map_err(|e| format!("connect {}: {}", peer_addr, e))?;
    s.set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| format!("timeout: {}", e))?;
    s.write_all(format!("{}\n", befehl).as_bytes())
        .map_err(|e| format!("send: {}", e))?;
    let mut line = String::new();
    BufReader::new(s)
        .read_line(&mut line)
        .map_err(|e| format!("recv: {}", e))?;
    Ok(line.trim().to_string())
}

/// Pull-Sync: holt den Peer-Status, zieht bei groesserer Hoehe die komplette
/// Kette und adoptiert sie NUR nach voller Verifikation und Genesis-Bindung.
pub fn sync_pull(kette: &mut Chain, peer_addr: &str) -> Result<SyncReport, String> {
    let antwort = peer_antwort(peer_addr, "STATUS")?;
    let peer_hoehe: u64 = antwort
        .split_whitespace()
        .next()
        .ok_or("Peer-STATUS leer")?
        .parse()
        .map_err(|e| format!("Peer-Hoehe unlesbar: {}", e))?;
    if peer_hoehe <= kette.height() {
        return Ok(SyncReport {
            adopted: false,
            neue_hoehe: kette.height(),
            grund: format!("Peer-Hoehe {} <= eigene Hoehe {}", peer_hoehe, kette.height()),
        });
    }
    let daten = peer_antwort(peer_addr, "BLOCKS 0")?;
    let bloecke = parse_bloecke(&daten)?;
    let kandidat = Chain::from_blocks(bloecke)?;
    if kandidat.blocks().first() != kette.blocks().first() {
        return Err("Peer-Genesis abweichend — Adoption verweigert".to_string());
    }
    if kandidat.height() <= kette.height() {
        return Ok(SyncReport {
            adopted: false,
            neue_hoehe: kette.height(),
            grund: "Kandidat nicht laenger".to_string(),
        });
    }
    let neue_hoehe = kandidat.height();
    let grund = format!(
        "adoptiert von {} (Hoehe {} -> {})",
        peer_addr,
        kette.height(),
        neue_hoehe
    );
    *kette = kandidat;
    Ok(SyncReport { adopted: true, neue_hoehe, grund })
}

fn parse_bloecke(daten: &str) -> Result<Vec<Block>, String> {
    if daten.is_empty() || daten == "ERR" {
        return Err("Peer lieferte keine Bloecke".to_string());
    }
    let mut out = Vec::new();
    for teil in daten.split(';') {
        let f: Vec<&str> = teil.split('|').collect();
        if f.len() != 4 {
            return Err(format!("Block-Feldzahl {} != 4", f.len()));
        }
        out.push(Block {
            height: f[0].parse().map_err(|e| format!("height: {}", e))?,
            prev_hash: f[1].parse().map_err(|e| format!("prev_hash: {}", e))?,
            payload: f[2].to_string(),
            hash: f[3].parse().map_err(|e| format!("hash: {}", e))?,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::Genesis;

    fn warte_auf_dienst(port: u16) {
        for _ in 0..50 {
            if TcpStream::connect(("127.0.0.1", port)).is_ok() {
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        panic!("Gossip-Dienst auf Port {} nicht erreichbar", port);
    }

    fn start_gossip(kette: Chain) -> u16 {
        let shared = Arc::new(Mutex::new(kette));
        let handler = Arc::clone(&shared);
        let probe = TcpListener::bind("127.0.0.1:0").expect("probe fehlgeschlagen");
        let port = probe.local_addr().expect("keine Adresse").port();
        drop(probe);
        std::thread::spawn(move || {
            let _ = serve_gossip(&format!("127.0.0.1:{}", port), handler);
        });
        warte_auf_dienst(port);
        port
    }

    #[test]
    fn sync_uebernimmt_laengere_kette() {
        let g = Genesis::devnet();
        let mut a = Chain::from_genesis(&g);
        for p in ["tx-1", "tx-2", "tx-3"] {
            a.produce(p).expect("produce a");
        }
        let ziel = a.best_hash();
        let port = start_gossip(a);

        let mut b = Chain::from_genesis(&g);
        assert_eq!(b.height(), 0);
        let rep = sync_pull(&mut b, &format!("127.0.0.1:{}", port)).expect("sync fehlgeschlagen");
        assert!(rep.adopted, "Adoption erwartet: {}", rep.grund);
        assert_eq!(b.height(), 3);
        assert_eq!(b.best_hash(), ziel, "uebernommene Kette muss identisch sein");
        assert!(b.verify().is_ok());
    }

    #[test]
    fn kuerzerer_peer_wird_nicht_adoptiert() {
        let g = Genesis::devnet();
        let a = Chain::from_genesis(&g); // Hoehe 0
        let port = start_gossip(a);
        let mut b = Chain::from_genesis(&g);
        b.produce("eigener-block").expect("produce b");
        let alt = b.best_hash();
        let rep = sync_pull(&mut b, &format!("127.0.0.1:{}", port)).expect("sync fehlgeschlagen");
        assert!(!rep.adopted, "kuerzerer Peer darf nicht adoptieren: {}", rep.grund);
        assert_eq!(b.best_hash(), alt, "eigene Kette unangetastet");
    }

    #[test]
    fn gefaelschte_kette_wird_abgelehnt() {
        // Boeser Server: STATUS luegt Hoehe, BLOCKS liefert kaputte Hashes
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind fehlgeschlagen");
        let port = listener.local_addr().expect("keine Adresse").port();
        std::thread::spawn(move || {
            for s in listener.incoming() {
                let mut s = match s { Ok(s) => s, Err(_) => break };
                let mut reader = BufReader::new(s.try_clone().unwrap());
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line.trim() == "STATUS" {
                    writeln!(s, "5 999").unwrap();
                } else {
                    writeln!(s, "0|111|gefaelscht|222;1|333|tx|444").unwrap();
                }
            }
        });
        warte_auf_dienst(port);

        let g = Genesis::devnet();
        let mut b = Chain::from_genesis(&g);
        let ergebnis = sync_pull(&mut b, &format!("127.0.0.1:{}", port));
        assert!(ergebnis.is_err(), "gefaelschte Kette muss abgelehnt werden");
        assert_eq!(b.height(), 0, "keine Adoption");
        assert!(b.verify().is_ok(), "eigene Kette bleibt valide");
    }

    #[test]
    fn genesis_abweichend_wird_abgelehnt() {
        let mut g2 = Genesis::devnet();
        g2.chain_name = "Andere Chain".to_string();
        let mut a = Chain::from_genesis(&g2);
        a.produce("fremde-tx").expect("produce a");
        let port = start_gossip(a);

        let g = Genesis::devnet();
        let mut b = Chain::from_genesis(&g);
        let ergebnis = sync_pull(&mut b, &format!("127.0.0.1:{}", port));
        assert!(ergebnis.is_err(), "abweichende Genesis muss abgelehnt werden");
        assert_eq!(b.height(), 0, "keine Adoption");
    }

    #[test]
    fn verbindungsfehler_ehrlich() {
        let g = Genesis::devnet();
        let mut b = Chain::from_genesis(&g);
        assert!(sync_pull(&mut b, "127.0.0.1:1").is_err());
    }
}

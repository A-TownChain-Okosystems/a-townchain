// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Gossip-Transport (SCR-0118) — **ohne** Adoption (P1d-A / Befund F4).
//!
//! Wire-Protokoll: "STATUS" -> "height best_hash"; "BLOCKS <from>" ->
//! Blockliste (Bloecke mit ';', Felder height/prev_hash/payload/hash mit '|').
//!
//! Hier liegt **nur der Dienst**, der Peers bedient. Der frueher vorhandene
//! `sync_pull` hat die Kette nach reiner Strukturpruefung ersetzt
//! (`*kette = kandidat`) und dabei keine Payload geprueft — ein Peer konnte
//! damit ATCB-Stroeme oder beliebige Daten als kanonischen Bestand
//! einschleusen. Dieser Weg ist entfernt, nicht abgeschwaecht.
//!
//! Kanonisch ist `atc_node::gossip::sync_pull` in `components/node`, das ueber
//! `Chain::adopt_verified` jeden neuen Block durch den `ContainerValidator`
//! schickt. Diese Kopie des Crates wird nicht weiterentwickelt; sie ist als
//! Dublette von `components/node` zur Stilllegung vorgesehen.
//! Ehrlichkeit: NUR Pull (kein Push an Peers), keine Periodik, keine
//! Signaturen, keine Auth/TLS, Devnet-Feldbreite: 64-Bit-Traversal von
//! ATC-HASH-001 TownHash-256 (SCR-0120), nicht kryptoanalysiert.

use crate::chain::Chain;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

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
            writeln!(s, "{}", teile.join(";"))?;
        } else {
            writeln!(s, "ERR")?;
        }
    }
    Ok(())
}

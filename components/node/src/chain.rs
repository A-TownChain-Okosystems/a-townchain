// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Blockmodell + deterministische Devnet-Blockproduktion (SCR-0117, F-139).
//! Ehrlichkeit: KEIN Konsens (Konsens bleibt kanonisch ueber atc-algorithm,
//! F-067 offen — dieses Modul ist kein Konsens-Definer und keine Finalitaet),
//! Transaktionen werden geprueft (Signatur, DID-Bindung, Nonce-Ledger),
//! aber ohne Gebuehren-/State-Semantik, ATC-HASH-001 TownHash-256 (64-Bit-Traversal,
//! SCR-0120, nicht kryptoanalysiert, kein Merkle-Baum, Devnet-Cap 64.

use atc_algorithm::hash::atc_hash;

use crate::bootstrap::Genesis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub height: u64,
    pub prev_hash: [u8; 32],
    pub payload: String,
    pub hash: [u8; 32],
}

impl Block {
    /// 256-Bit-Blockhash (SEC-P0-007): SHA-256 ueber Domain, Hoehe, Hex des
    /// Vorgaenger-Hashes und Payload. Kein 64-Bit-Traversal mehr.
    fn compute_hash(height: u64, prev_hash: [u8; 32], payload: &str) -> [u8; 32] {
        atc_hash(format!("ATC-BLOCK-V1|{}|{}|{}", height, hash_hex(&prev_hash), payload).as_bytes())
    }

    /// Genesis-Block: Hoehe 0, prev_hash = Genesis-Boot-Hash (SCR-0106/0114)
    /// — die Blockkette ist damit kryptographisch-Platzhalter-gebunden an die
    /// Genesis-Wahrheit; Drift in der Genesis aendert die ganze Kette.
    pub fn genesis_block(g: &Genesis) -> Block {
        let payload = g.state_root.clone();
        let prev_hash = g.boot_hash256();
        Block {
            height: 0,
            prev_hash,
            hash: Block::compute_hash(0, prev_hash, &payload),
            payload,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chain {
    blocks: Vec<Block>,
}

impl Chain {
    pub fn from_genesis(g: &Genesis) -> Chain {
        Chain { blocks: vec![Block::genesis_block(g)] }
    }

    pub fn best(&self) -> &Block {
        self.blocks.last().expect("Kette enthaelt immer den Genesis-Block")
    }

    pub fn height(&self) -> u64 {
        self.best().height
    }

    pub fn best_hash(&self) -> [u8; 32] {
        self.best().hash
    }

    /// Deterministische Devnet-Blockproduktion (kein Konsens, dokumentiert):
    /// haengt an den Best-Block an. Gibt den neuen Best-Hash zurueck.
    pub fn produce(&mut self, payload: &str) -> Result<[u8; 32], String> {
        if self.blocks.len() >= 64 {
            return Err("Devnet-MVP: Kette auf 64 Bloecke begrenzt".to_string());
        }
        if payload.contains('|') || payload.contains(';') {
            return Err("Devnet-MVP: '|' und ';' in Payloads verboten (Gossip-Wire-Format, SCR-0118)".to_string());
        }
        let (height, prev_hash) = {
            let best = self.best();
            (best.height + 1, best.hash)
        };
        let hash = Block::compute_hash(height, prev_hash, payload);
        self.blocks.push(Block {
            height,
            prev_hash,
            payload: payload.to_string(),
            hash,
        });
        Ok(hash)
    }

    /// Vollstaendige Ketten-Verifikation: Hashes, Hoehen-Monotonie,
    /// Verkettung und Genesis-Bindung. Manipulation schlaegt fehl.
    pub fn verify(&self) -> Result<(), String> {
        if self.blocks.is_empty() {
            return Err("Kette leer".to_string());
        }
        for (i, b) in self.blocks.iter().enumerate() {
            if b.hash != Block::compute_hash(b.height, b.prev_hash, &b.payload) {
                return Err(format!("Hash inkonsistent bei Hoehe {}", b.height));
            }
            if i == 0 {
                if b.height != 0 {
                    return Err(format!("Genesis-Block muss Hoehe 0 haben, ist {}", b.height));
                }
            } else {
                let p = &self.blocks[i - 1];
                if b.height != p.height + 1 {
                    return Err(format!("Hoehen-Sprung bei Hoehe {}", b.height));
                }
                if b.prev_hash != p.hash {
                    return Err(format!("Verkettung gebrochen bei Hoehe {}", b.height));
                }
            }
        }
        Ok(())
    }

    /// Payload fuer eine Tx-Menge bilden, ohne die Kette oder den Ledger zu
    /// veraendern (Konsens-Phase vor der Finalitaet).
    pub fn tx_payload(
        &self,
        txs: &[crate::mempool::TxEnvelope],
        ledger: &mut crate::mempool::TxLedger,
    ) -> Result<String, String> {
        if txs.is_empty() {
            return Ok(String::new());
        }
        let ids = ledger
            .check_block(txs)
            .map_err(|e| format!("Tx-Pruefung fehlgeschlagen: {:?}", e))?;
        Ok(encode_tx_ids(&ids))
    }

    /// Hash, den der naechste Block mit dieser Payload traegt (ohne Anhaengen).
    pub fn preview_hash(&self, payload: &str) -> [u8; 32] {
        Block::compute_hash(self.height() + 1, self.best_hash(), payload)
    }

    /// Haengt eine bereits gepruefte Payload an und committet die Txs im Ledger.
    pub fn commit_payload(
        &mut self,
        payload: &str,
        txs: &[crate::mempool::TxEnvelope],
        ledger: &mut crate::mempool::TxLedger,
    ) -> Result<[u8; 32], String> {
        let hash = self.produce(payload)?;
        if !txs.is_empty() {
            ledger
                .commit_block(txs)
                .map_err(|e| format!("Ledger-Commit fehlgeschlagen: {:?}", e))?;
        }
        Ok(hash)
    }

    /// Produziert einen Block aus geprueften Transaktionen (SEC-P0-001).
    ///
    /// Reihenfolge: Ledger-Pruefung (Signatur, DID-Bindung, Nonce, Replay) ->
    /// Payload aus den Tx-IDs -> Blockanhang -> Ledger-Commit. Schlaegt die
    /// Pruefung fehl, bleibt die Kette unveraendert und der Ledger unberuehrt.
    pub fn produce_tx_block(
        &mut self,
        txs: &[crate::mempool::TxEnvelope],
        ledger: &mut crate::mempool::TxLedger,
    ) -> Result<[u8; 32], String> {
        let ids = ledger
            .check_block(txs)
            .map_err(|e| format!("Tx-Pruefung fehlgeschlagen: {:?}", e))?;
        let payload = encode_tx_ids(&ids);
        let hash = self.produce(&payload)?;
        ledger
            .commit_block(txs)
            .map_err(|e| format!("Ledger-Commit fehlgeschlagen: {:?}", e))?;
        Ok(hash)
    }

    /// Importiert einen empfangenen Block mit Transaktionen (SEC-P0-001).
    ///
    /// Prueft Hoehe gegen den Best-Block und alle Transaktionen, bevor der
    /// Block angehaengt wird. Kein Reorg, keine Finalitaet.
    pub fn import_tx_block(
        &mut self,
        height: u64,
        txs: &[crate::mempool::TxEnvelope],
        ledger: &mut crate::mempool::TxLedger,
    ) -> Result<[u8; 32], String> {
        if height != self.height() + 1 {
            return Err(format!(
                "Blockhoehe {} passt nicht zur Kette (erwartet {})",
                height,
                self.height() + 1
            ));
        }
        let ids = ledger
            .check_block(txs)
            .map_err(|e| format!("Tx-Pruefung fehlgeschlagen: {:?}", e))?;
        let payload = encode_tx_ids(&ids);
        let hash = self.produce(&payload)?;
        ledger
            .commit_block(txs)
            .map_err(|e| format!("Ledger-Commit fehlgeschlagen: {:?}", e))?;
        Ok(hash)
    }

    /// Kompletter Blockzugriff (read-only) fuer Gossip-Sync (SCR-0118).
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// Bloecke ab Index `from` (read-only) fuer Gossip-Wire (SCR-0118).
    pub fn blocks_from(&self, from: usize) -> &[Block] {
        &self.blocks[from.min(self.blocks.len())..]
    }

    /// Baut eine Kette aus einer empfangenen Blockfolge — NUR nach voller
    /// Verifikation (Hashes, Hoehen, Verkettung). Manipulation schlaegt fehl.
    pub fn from_blocks(blocks: Vec<Block>) -> Result<Chain, String> {
        if blocks.is_empty() {
            return Err("Kandidat-Kette leer".to_string());
        }
        let c = Chain { blocks };
        c.verify()?;
        Ok(c)
    }
}

/// Hex-Kodierung eines 32-Byte-Hashes (Gossip-Wire, 64 Zeichen).
pub fn hash_hex(hash: &[u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(64);
    for &b in hash {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

/// Parst 64 Hex-Zeichen zu einem 32-Byte-Hash.
pub fn parse_hash(text: &str) -> Result<[u8; 32], String> {
    let t = text.trim();
    if t.len() != 64 {
        return Err(format!("Hash muss 64 Hex-Zeichen haben, hat {}", t.len()));
    }
    let mut out = [0u8; 32];
    for (i, chunk) in t.as_bytes().chunks(2).enumerate() {
        let hi = (chunk[0] as char).to_digit(16).ok_or_else(|| "kein Hex".to_string())?;
        let lo = (chunk[1] as char).to_digit(16).ok_or_else(|| "kein Hex".to_string())?;
        out[i] = ((hi << 4) | lo) as u8;
    }
    Ok(out)
}

/// Kanonische Payload-Kodierung eines Tx-Blocks: Tx-IDs als hex, durch Komma
/// getrennt (Gossip-Wire-Format verbietet '|' und ';', SCR-0118).
fn encode_tx_ids(ids: &[[u8; 32]]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(ids.len() * 65);
    for (i, id) in ids.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        for &b in id {
            out.push(HEX[(b >> 4) as usize] as char);
            out.push(HEX[(b & 0x0f) as usize] as char);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_block_bindet_an_boot_hash() {
        let g = Genesis::devnet();
        let b = Block::genesis_block(&g);
        assert_eq!(b.height, 0);
        assert_eq!(b.prev_hash, g.boot_hash256());
        let c = Chain::from_genesis(&g);
        assert_eq!(c.height(), 0);
        assert!(c.verify().is_ok());
    }

    #[test]
    fn kette_waechst_und_verifiziert() {
        let g = Genesis::devnet();
        let mut c = Chain::from_genesis(&g);
        for p in ["tx-a", "tx-b", "tx-c"] {
            c.produce(p).expect("produce fehlgeschlagen");
        }
        assert_eq!(c.height(), 3);
        assert_eq!(c.blocks.len(), 4);
        assert!(c.verify().is_ok());
    }

    #[test]
    fn manipulation_wird_erkannt() {
        let g = Genesis::devnet();
        let mut c = Chain::from_genesis(&g);
        c.produce("ehrlich").unwrap();
        c.produce("ehrlich-2").unwrap();
        let mut gefaelscht = c.clone();
        gefaelscht.blocks[1].payload = "gefaelscht".to_string();
        assert!(gefaelscht.verify().is_err(), "Payload-Manipulation muss auffallen");
        let mut umgehaengt = c.clone();
        umgehaengt.blocks[2].prev_hash = [7u8; 32];
        assert!(umgehaengt.verify().is_err(), "Verkettungs-Bruch muss auffallen");
        assert!(c.verify().is_ok(), "Original bleibt valide");
    }

    #[test]
    fn determinismus_zwei_instanzen() {
        let g = Genesis::devnet();
        let mut k1 = Chain::from_genesis(&g);
        let mut k2 = Chain::from_genesis(&g);
        for p in ["a", "b", "c", "d"] {
            k1.produce(p).unwrap();
            k2.produce(p).unwrap();
        }
        assert_eq!(k1.best_hash(), k2.best_hash(), "gleiche Genesis + gleiche Payloads => identische Kette");
        assert_eq!(k1, k2);
    }

    #[test]
    fn andere_genesis_andere_kette() {
        let mut g = Genesis::devnet();
        let c1 = Chain::from_genesis(&g);
        g.chain_name = "A-TownChain Devnet ALT".to_string();
        let c2 = Chain::from_genesis(&g);
        assert_ne!(c1.best_hash(), c2.best_hash(), "Genesis-Drift muss die Kette aendern");
    }

    #[test]
    fn devnet_cap_ehrlich() {
        let g = Genesis::devnet();
        let mut c = Chain::from_genesis(&g);
        // Cap 64 gilt INKLUSIVE Genesis-Block: 63 produzierbar, dann ehrlich Err
        for _ in 0..63 {
            c.produce("fuell").unwrap();
        }
        assert_eq!(c.height(), 63);
        assert!(c.produce("eins zu viel").is_err(), "Cap muss greifen");
        assert_eq!(c.height(), 63, "Fehlgeschlagene Produktion aendert nichts");
    }
}

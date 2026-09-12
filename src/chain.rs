// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Blockmodell + deterministische Devnet-Blockproduktion (SCR-0117, F-139).
//! Ehrlichkeit: KEIN Konsens (Konsens bleibt kanonisch ueber atc-algorithm,
//! F-067 offen — dieses Modul ist kein Konsens-Definer und keine Finalitaet),
//! keine Transaktionssemantik, FNV-1a 64-bit als dokumentierter
//! nicht-kryptographischer MVP-Platzhalter, kein Merkle-Baum, Devnet-Cap 64.

use crate::bootstrap::{fnv1a, Genesis};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub height: u64,
    pub prev_hash: u64,
    pub payload: String,
    pub hash: u64,
}

impl Block {
    fn compute_hash(height: u64, prev_hash: u64, payload: &str) -> u64 {
        fnv1a(&format!("{}|{}|{}", height, prev_hash, payload))
    }

    /// Genesis-Block: Hoehe 0, prev_hash = Genesis-Boot-Hash (SCR-0106/0114)
    /// — die Blockkette ist damit kryptographisch-Platzhalter-gebunden an die
    /// Genesis-Wahrheit; Drift in der Genesis aendert die ganze Kette.
    pub fn genesis_block(g: &Genesis) -> Block {
        let payload = g.state_root.clone();
        let prev_hash = g.boot_hash();
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

    pub fn best_hash(&self) -> u64 {
        self.best().hash
    }

    /// Deterministische Devnet-Blockproduktion (kein Konsens, dokumentiert):
    /// haengt an den Best-Block an. Gibt den neuen Best-Hash zurueck.
    pub fn produce(&mut self, payload: &str) -> Result<u64, String> {
        if self.blocks.len() >= 64 {
            return Err("Devnet-MVP: Kette auf 64 Bloecke begrenzt".to_string());
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_block_bindet_an_boot_hash() {
        let g = Genesis::devnet();
        let b = Block::genesis_block(&g);
        assert_eq!(b.height, 0);
        assert_eq!(b.prev_hash, g.boot_hash());
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
        umgehaengt.blocks[2].prev_hash = 123;
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

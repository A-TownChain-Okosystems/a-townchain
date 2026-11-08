//! Merkle-Tree über deterministisch sortierte State-Entries (BTreeMap).
//! Odd-Level-Regel: letzter Knoten wird dupliziert (kanonisch, deterministisch).

use crate::sha_hex;
use std::collections::BTreeMap;

/// SHA-256 des leeren Inputs — Root des leeren States.
pub const EMPTY_ROOT: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub fn leaf_hash(key: &str, value: &str) -> String {
    sha_hex(format!("{key}|{value}").as_bytes())
}

fn levels(entries: &BTreeMap<String, String>) -> Vec<Vec<String>> {
    let mut levels = vec![entries
        .iter()
        .map(|(k, v)| leaf_hash(k, v))
        .collect::<Vec<String>>()];
    if levels[0].is_empty() {
        return levels;
    }
    // Kein unwrap in konsens-kritischem Code (Owner-Regel):
    // Borrow wird im Block gekapselt, leere Level beenden die Schleife explizit.
    loop {
        let next = {
            let Some(prev) = levels.last() else { break };
            if prev.len() <= 1 {
                break;
            }
            let mut next = Vec::with_capacity(prev.len() / 2 + 1);
            for pair in prev.chunks(2) {
                if pair.len() == 2 {
                    next.push(sha_hex(format!("{}|{}", pair[0], pair[1]).as_bytes()));
                } else {
                    // Odd-Regel: Duplikat des letzten Knotens
                    next.push(sha_hex(format!("{}|{}", pair[0], pair[0]).as_bytes()));
                }
            }
            next
        };
        levels.push(next);
    }
    levels
}

/// Merkle-Root — deterministisch, unabhängig von der Einfüge-Reihenfolge.
pub fn merkle_root(entries: &BTreeMap<String, String>) -> String {
    if entries.is_empty() {
        return EMPTY_ROOT.to_string();
    }
    match levels(entries).last() {
        Some(l) if !l.is_empty() => l[0].clone(),
        // Kann bei nicht-leerem Input nicht eintreten; fail-closed gegen EMPTY_ROOT.
        _ => EMPTY_ROOT.to_string(),
    }
}

/// Inclusion-Proof für einen Key (Liste der Geschwister-Hashes von unten nach oben).
#[derive(Debug, Clone, PartialEq)]
pub struct MerkleProof {
    /// (Geschwister-Hash, Geschwister-ist-rechts?)
    pub siblings: Vec<(String, bool)>,
}

pub fn prove(entries: &BTreeMap<String, String>, key: &str) -> Option<MerkleProof> {
    let idx = entries.keys().position(|k| k == key)?;
    let lvls = levels(entries);
    let mut siblings = Vec::new();
    let mut i = idx;
    for level in lvls.iter().take(lvls.len() - 1) {
        if i % 2 == 0 {
            if i + 1 < level.len() {
                siblings.push((level[i + 1].clone(), true));
            } else {
                siblings.push((level[i].clone(), true)); // Odd-Duplikat
            }
        } else {
            siblings.push((level[i - 1].clone(), false));
        }
        i /= 2;
    }
    Some(MerkleProof { siblings })
}

/// Proof-Verifikation: Leaf-Hash + Pfad muss exakt den Root reproduzieren.
pub fn verify_proof(leaf: &str, proof: &MerkleProof, root: &str) -> bool {
    let mut cur = leaf.to_string();
    for (sib, sib_is_right) in &proof.siblings {
        cur = if *sib_is_right {
            sha_hex(format!("{cur}|{sib}").as_bytes())
        } else {
            sha_hex(format!("{sib}|{cur}").as_bytes())
        };
    }
    cur == root
}

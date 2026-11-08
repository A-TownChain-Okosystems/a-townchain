// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! DID-Registry mit Schluesselrotation (SEC-P0-004).
//!
//! Problem vor dem Fix: Eine Signatur bewies nur Schluesselbesitz — jeder Key
//! durfte jede DID beanspruchen, weil der Verifier dem mitgelieferten Pubkey
//! glaubte. Hier loest die Registry die DID auf den autoritativen Schluessel
//! auf; der Verifier vertraut nur noch dieser Aufloesung.
//!
//! Regeln:
//! - Die DID wird aus dem Erst-Schluessel abgeleitet (self-certifying):
//!   `ATC:<hex(sha256(pubkey)[..20])>`.
//! - Rotation ist nur mit einem Beweis des **aktuellen** Schluessels moeglich
//!   (domain-separiert, mit DID, neuem Schluessel, Epoche und Zielversion).
//! - Alte Schluessel werden nicht geloescht, sondern mit `retired_at_epoch`
//!   stillgelegt — historische Signaturen bleiben damit pruefbar.

use std::collections::HashMap;
use std::path::Path;

use atc_wallet::keys::address_from_pubkey;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Domain-Tag der Rotationsnachricht.
pub const ROTATE_DOMAIN: &[u8] = b"ATC-DID-ROTATE-V1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DidError {
    UnknownDid,
    NoActiveKey,
    AlreadyRegistered,
    /// Mitgelieferter Pubkey ist nicht der autoritative Schluessel der DID.
    KeyMismatch,
    /// Rotationsbeweis fehlt, ist falsch oder stammt nicht vom aktuellen Key.
    BadRotationProof,
    /// Rotation auf einen bereits verwendeten Schluessel.
    KeyReuse,
    MalformedKey,
    /// Gespeichertes Dokument passt nicht zur DID oder ist inkonsistent.
    InconsistentDocument,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DidKey {
    #[serde(with = "hex33")]
    pub pubkey: [u8; 33],
    /// Ab dieser Registry-Epoche autoritativ.
    pub valid_from_epoch: u64,
    /// Ab dieser Epoche stillgelegt (None = aktuell).
    pub retired_at_epoch: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DidDocument {
    pub did: String,
    pub keys: Vec<DidKey>,
    pub version: u64,
}

impl DidDocument {
    pub fn current_key(&self) -> Option<[u8; 33]> {
        self.keys
            .iter()
            .find(|k| k.retired_at_epoch.is_none())
            .map(|k| k.pubkey)
    }

    /// Schluessel, der in einer bestimmten Epoche autoritativ war.
    pub fn key_at(&self, epoch: u64) -> Option<[u8; 33]> {
        self.keys
            .iter()
            .find(|k| {
                k.valid_from_epoch <= epoch
                    && k.retired_at_epoch.map(|r| epoch <= r).unwrap_or(true)
            })
            .map(|k| k.pubkey)
    }

    pub fn was_key(&self, pubkey: &[u8; 33]) -> bool {
        self.keys.iter().any(|k| &k.pubkey == pubkey)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DidRegistry {
    docs: HashMap<String, DidDocument>,
    epoch: u64,
}

impl DidRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn len(&self) -> usize {
        self.docs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.docs.is_empty()
    }

    pub fn contains(&self, did: &str) -> bool {
        self.docs.contains_key(did)
    }

    pub fn get(&self, did: &str) -> Option<&DidDocument> {
        self.docs.get(did)
    }

    pub fn resolve(&self, did: &str) -> Result<&DidDocument, DidError> {
        self.docs.get(did).ok_or(DidError::UnknownDid)
    }

    /// Registriert einen Erst-Schluessel und leitet die DID daraus ab.
    pub fn register(&mut self, pubkey: [u8; 33]) -> Result<String, DidError> {
        Self::validate_key(&pubkey)?;
        let did = address_from_pubkey(&pubkey);
        if self.docs.contains_key(&did) {
            return Err(DidError::AlreadyRegistered);
        }
        let doc = DidDocument {
            did: did.clone(),
            keys: vec![DidKey {
                pubkey,
                valid_from_epoch: self.epoch,
                retired_at_epoch: None,
            }],
            version: 1,
        };
        self.docs.insert(did.clone(), doc);
        Ok(did)
    }

    /// Aktueller autoritativer Schluessel einer DID.
    pub fn current_key(&self, did: &str) -> Result<[u8; 33], DidError> {
        self.resolve(did)?
            .current_key()
            .ok_or(DidError::NoActiveKey)
    }

    /// Prueft, ob `pubkey` der DID zugeordnet ist (aktuell oder historisch).
    pub fn is_known_key(&self, did: &str, pubkey: &[u8; 33]) -> bool {
        self.docs
            .get(did)
            .map(|d| d.was_key(pubkey))
            .unwrap_or(false)
    }

    /// Kanonische Rotationsnachricht: Domain | DID (laengenpraefigiert) |
    /// neuer Pubkey (33 Byte) | Zielversion | Epoche.
    pub fn rotation_message(did: &str, new_pubkey: &[u8; 33], new_version: u64, epoch: u64) -> Vec<u8> {
        let mut b = Vec::with_capacity(ROTATE_DOMAIN.len() + 4 + did.len() + 33 + 16);
        b.extend_from_slice(ROTATE_DOMAIN);
        b.extend_from_slice(&(did.len() as u32).to_be_bytes());
        b.extend_from_slice(did.as_bytes());
        b.extend_from_slice(new_pubkey);
        b.extend_from_slice(&new_version.to_be_bytes());
        b.extend_from_slice(&epoch.to_be_bytes());
        b
    }

    /// Rotiert den DID-Schluessel. Der Beweis muss vom aktuellen Schluessel
    /// stammen und die Zielversion sowie den neuen Schluessel decken.
    pub fn rotate(
        &mut self,
        did: &str,
        new_pubkey: [u8; 33],
        proof: &[u8; 64],
        target_version: u64,
    ) -> Result<(), DidError> {
        Self::validate_key(&new_pubkey)?;
        let doc = self.docs.get(did).ok_or(DidError::UnknownDid)?.clone();
        let current = doc.current_key().ok_or(DidError::NoActiveKey)?;
        if doc.was_key(&new_pubkey) {
            return Err(DidError::KeyReuse);
        }
        let expected_version = doc.version + 1;
        if target_version != expected_version {
            return Err(DidError::BadRotationProof);
        }
        let message = Self::rotation_message(did, &new_pubkey, target_version, self.epoch);
        if !verify_with(&current, &message, proof) {
            return Err(DidError::BadRotationProof);
        }
        let doc = self.docs.get_mut(did).ok_or(DidError::UnknownDid)?;
        for k in doc.keys.iter_mut() {
            if k.retired_at_epoch.is_none() {
                k.retired_at_epoch = Some(self.epoch);
            }
        }
        doc.keys.push(DidKey {
            pubkey: new_pubkey,
            valid_from_epoch: self.epoch,
            retired_at_epoch: None,
        });
        doc.version = target_version;
        self.epoch += 1;
        Ok(())
    }

    /// Prueft die geladene Registry gegen die DID-Ableitungsregel: die DID muss
    /// aus dem Erst-Schluessel folgen, genau ein Schluessel aktiv sein und die
    /// Version zur Schluesselzahl passen. Manipulierte Dateien werden abgelehnt.
    pub fn validate(&self) -> Result<(), DidError> {
        for (did, doc) in &self.docs {
            if &doc.did != did {
                return Err(DidError::InconsistentDocument);
            }
            let first = doc.keys.first().ok_or(DidError::InconsistentDocument)?;
            if address_from_pubkey(&first.pubkey) != *did {
                return Err(DidError::InconsistentDocument);
            }
            let active = doc.keys.iter().filter(|k| k.retired_at_epoch.is_none()).count();
            if active != 1 {
                return Err(DidError::InconsistentDocument);
            }
            if doc.version as usize != doc.keys.len() {
                return Err(DidError::InconsistentDocument);
            }
            for key in &doc.keys {
                Self::validate_key(&key.pubkey)?;
            }
        }
        Ok(())
    }

    fn validate_key(pubkey: &[u8; 33]) -> Result<(), DidError> {
        PublicKey::from_slice(pubkey)
            .map(|_| ())
            .map_err(|_| DidError::MalformedKey)
    }
}

/// ECDSA-Verifikation mit Low-S-Pflicht.
pub fn verify_with(pubkey: &[u8; 33], message: &[u8], signature: &[u8; 64]) -> bool {
    let pk = match PublicKey::from_slice(pubkey) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    let sig = match Signature::from_compact(signature) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    let mut normalized = sig;
    normalized.normalize_s();
    if normalized != sig {
        return false;
    }
    let digest: [u8; 32] = Sha256::digest(message).into();
    Secp256k1::verification_only()
        .verify_ecdsa(Message::from_digest(digest), &sig, &pk)
        .is_ok()
}

impl DidRegistry {
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| format!("Serialisierung: {e}"))
    }

    pub fn from_json(text: &str) -> Result<Self, String> {
        let reg: DidRegistry =
            serde_json::from_str(text).map_err(|e| format!("Parse-Fehler: {e}"))?;
        reg.validate().map_err(|e| format!("Registry inkonsistent: {e:?}"))?;
        Ok(reg)
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        std::fs::write(path, self.to_json()?).map_err(|e| format!("Schreiben: {e}"))
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("Lesen: {e}"))?;
        Self::from_json(&text)
    }
}

/// Serialisierung des 33-Byte-Pubkeys als Hex (serde kennt nur Arrays bis 32).
mod hex33 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &[u8; 33], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&super::hex_lower(v))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 33], D::Error> {
        let text = String::deserialize(d)?;
        super::parse_hex33(&text).map_err(serde::de::Error::custom)
    }
}

pub(crate) fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

pub(crate) fn parse_hex33(text: &str) -> Result<[u8; 33], String> {
    let t = text.trim();
    if t.len() != 66 {
        return Err(format!("Pubkey muss 66 Hex-Zeichen haben, hat {}", t.len()));
    }
    let mut out = [0u8; 33];
    for (i, chunk) in t.as_bytes().chunks(2).enumerate() {
        let hi = (chunk[0] as char).to_digit(16).ok_or_else(|| "kein Hex".to_string())?;
        let lo = (chunk[1] as char).to_digit(16).ok_or_else(|| "kein Hex".to_string())?;
        out[i] = ((hi << 4) | lo) as u8;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::SecretKey;

    fn keypair(seed: u8) -> (SecretKey, [u8; 33]) {
        let sk = SecretKey::from_byte_array([seed; 32]).unwrap();
        let pk = PublicKey::from_secret_key(&Secp256k1::new(), &sk).serialize();
        (sk, pk)
    }

    fn sign(sk: &SecretKey, message: &[u8]) -> [u8; 64] {
        let digest: [u8; 32] = Sha256::digest(message).into();
        Secp256k1::new()
            .sign_ecdsa(Message::from_digest(digest), sk)
            .serialize_compact()
    }

    #[test]
    fn did_wird_aus_dem_schluessel_abgeleitet() {
        let (_, pk) = keypair(1);
        let mut reg = DidRegistry::new();
        let did = reg.register(pk).unwrap();
        assert_eq!(did, address_from_pubkey(&pk));
        assert_eq!(reg.current_key(&did).unwrap(), pk);
        assert_eq!(reg.resolve(&did).unwrap().version, 1);
        assert_eq!(reg.register(pk), Err(DidError::AlreadyRegistered));
    }

    #[test]
    fn rotation_mit_beweis_des_aktuellen_schluessels() {
        let (sk1, pk1) = keypair(1);
        let (_, pk2) = keypair(2);
        let mut reg = DidRegistry::new();
        let did = reg.register(pk1).unwrap();

        let msg = DidRegistry::rotation_message(&did, &pk2, 2, reg.epoch());
        let proof = sign(&sk1, &msg);
        reg.rotate(&did, pk2, &proof, 2).unwrap();

        assert_eq!(reg.current_key(&did).unwrap(), pk2);
        assert!(reg.is_known_key(&did, &pk1), "alter Schluessel bleibt historisch bekannt");
        let doc = reg.resolve(&did).unwrap();
        assert_eq!(doc.version, 2);
        assert_eq!(doc.key_at(0), Some(pk1));
    }

    #[test]
    fn rotation_mit_fremdem_schluessel_wird_abgelehnt() {
        let (_, pk1) = keypair(1);
        let (sk_fremd, pk2) = keypair(2);
        let mut reg = DidRegistry::new();
        let did = reg.register(pk1).unwrap();

        let msg = DidRegistry::rotation_message(&did, &pk2, 2, reg.epoch());
        let proof = sign(&sk_fremd, &msg);
        assert_eq!(reg.rotate(&did, pk2, &proof, 2), Err(DidError::BadRotationProof));
        assert_eq!(reg.current_key(&did).unwrap(), pk1);
    }

    #[test]
    fn rotation_muss_neuen_schluessel_und_version_decken() {
        let (sk1, pk1) = keypair(1);
        let (_, pk2) = keypair(2);
        let (_, pk3) = keypair(3);
        let mut reg = DidRegistry::new();
        let did = reg.register(pk1).unwrap();

        // Beweis fuer pk2, eingereicht mit pk3 -> abgelehnt
        let msg = DidRegistry::rotation_message(&did, &pk2, 2, reg.epoch());
        let proof = sign(&sk1, &msg);
        assert_eq!(reg.rotate(&did, pk3, &proof, 2), Err(DidError::BadRotationProof));
        // Falsche Zielversion -> abgelehnt
        assert_eq!(reg.rotate(&did, pk2, &proof, 9), Err(DidError::BadRotationProof));
        // Korrekt -> akzeptiert
        assert!(reg.rotate(&did, pk2, &proof, 2).is_ok());
    }

    #[test]
    fn schluessel_wiederverwendung_wird_abgelehnt() {
        let (sk1, pk1) = keypair(1);
        let (_, pk2) = keypair(2);
        let mut reg = DidRegistry::new();
        let did = reg.register(pk1).unwrap();
        let msg = DidRegistry::rotation_message(&did, &pk2, 2, reg.epoch());
        let proof = sign(&sk1, &msg);
        reg.rotate(&did, pk2, &proof, 2).unwrap();
        // Zurueck auf den alten Schluessel ist gesperrt (Replay-Schutz der Rotation)
        let msg2 = DidRegistry::rotation_message(&did, &pk1, 3, reg.epoch());
        let (sk2, _) = keypair(2);
        let proof2 = sign(&sk2, &msg2);
        assert_eq!(reg.rotate(&did, pk1, &proof2, 3), Err(DidError::KeyReuse));
    }

    #[test]
    fn unbekannte_did_und_kaputte_schluessel() {
        let mut reg = DidRegistry::new();
        assert_eq!(reg.current_key("ATC:unbekannt"), Err(DidError::UnknownDid));
        assert_eq!(reg.register([0u8; 33]), Err(DidError::MalformedKey));
    }
    #[test]
    fn registry_ueberlebt_speichern_und_laden() {
        let (sk1, pk1) = keypair(1);
        let (_, pk2) = keypair(2);
        let mut reg = DidRegistry::new();
        let did = reg.register(pk1).unwrap();
        let msg = DidRegistry::rotation_message(&did, &pk2, 2, reg.epoch());
        let proof = sign(&sk1, &msg);
        reg.rotate(&did, pk2, &proof, 2).unwrap();

        let path = std::env::temp_dir().join("atc-did-registry-test.json");
        reg.save(&path).unwrap();
        let geladen = DidRegistry::load(&path).unwrap();
        assert_eq!(geladen, reg, "Roundtrip muss Zustand inkl. Epoche erhalten");
        assert_eq!(geladen.current_key(&did).unwrap(), pk2);
        assert!(geladen.is_known_key(&did, &pk1));
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn manipuliertes_dokument_wird_abgelehnt() {
        let (_, pk1) = keypair(1);
        let mut reg = DidRegistry::new();
        let did = reg.register(pk1).unwrap();
        let json = reg.to_json().unwrap();
        // DID auf eine fremde Zeichenkette umbiegen -> Ableitung passt nicht mehr
        let manipuliert = json.replace(&did, "ATC:0000000000000000000000000000000000000000");
        assert!(DidRegistry::from_json(&manipuliert).is_err());
    }

    #[test]
    fn kaputtes_json_wird_abgelehnt() {
        assert!(DidRegistry::from_json("{ kein json").is_err());
    }
}

// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! ATC-Artifact-Huelle `ATCE`, Fassung `0x02` (SCE-ARTIFACT, Kandidat §18/§19).
//!
//! Die Huelle ist eine **eigene Verarbeitungsschicht** und ersetzt das
//! VM-Bytecodeformat nicht: `ATCE` ist das aeussere Artefaktformat, `ATCB` mit
//! Fassung `0x01` bleibt das ausfuehrbare Bytecodeformat. Beide Formate sind
//! unabhaengig versioniert, und jeder Parser akzeptiert nur sein eigenes Magic
//! (D-0, Owner-Entscheidung 12.11.2026, Option 1).
//!
//! ```text
//! ATCE / 0x02                            <- diese Datei
//!   0..4   magic "ATCE"
//!   4      Containerfassung (0x02)
//!   5      Generation (1 = ATCB-v1, 2 = ATCB-v2)
//!   6..10  Anzahl Operationen (u32, Obergrenze, kein Endmarker)
//!   10..   Operationsstrom  = ATCB/0x01-Inhalt ohne eigenen Kopf
//!   -32..  artifact_hash = SHA-256(domain || bytes[0..tail_start])
//! ```
//!
//! Keine VM-Logik wird hier doppelt implementiert: Opcode-Tabelle, Operanden-
//! breiten und die Stromprozedur kommen aus [`crate::artifact`]
//! (`dekodiere_strom`, `schreibe_op`). Diese Datei ergaenzt nur Kopf, Fassung,
//! Generation, Digest und die Stufenreihenfolge der Huelle.
//!
//! Der Bytecode-Hash des inneren `ATCB`-Artefakts bleibt unberuehrt: die Huelle
//! hat einen **eigenen** Digest (`artifact_hash`) und veraendert weder Opcodes
//! noch [`crate::artifact::Artifact::hash`].

use crate::artifact::{
    dekodiere_strom, schreibe_op, sha256_daten, Artifact, ArtifactError, GENERATION_V1,
    GENERATION_V2, HASH_LEN, HEAD_LEN,
};
use crate::vm::Op;

/// Kennung der Artefakt-Huelle.
pub const MAGIC: [u8; 4] = *b"ATCE";
/// Unterstuetztes Format der Huelle.
pub const ENVELOPE_VERSION: u8 = 0x02;
/// Laenge des angehaengten `artifact_hash`.
pub const TAIL_LEN: usize = HASH_LEN;
/// Kleinste gueltige Huelle: Kopf plus Tail, ohne Strom.
pub const MIN_LEN: usize = HEAD_LEN + TAIL_LEN;
/// Domaenenpraefix des `artifact_hash` (REQ-SCE-062, byte-exakt 16 Byte).
pub const DOMAIN_ARTIFACT: &[u8] = b"ATC-ARTIFACT-V1\x00";

/// Fehler der Huelle. Fehleridentitaet ist das Paar `(Code, Offset)`; die
/// Meldung ist nicht normativ (§19.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvelopeError {
    code: &'static str,
    offset: usize,
}

impl EnvelopeError {
    fn new(code: &'static str, offset: usize) -> Self {
        EnvelopeError { code, offset }
    }

    /// Normativer Fehlercode (`E-001` bis `E-012`).
    pub fn code(&self) -> &'static str {
        self.code
    }

    /// Offset des ersten nicht akzeptierten Bytes.
    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl From<crate::artifact::StromFehler> for EnvelopeError {
    fn from(f: crate::artifact::StromFehler) -> Self {
        EnvelopeError::new(f.code(), f.offset())
    }
}

/// Geprueftes Artefakt der Huelle `ATCE`.
///
/// Entsteht ausschliesslich aus [`Envelope::parse`] oder
/// [`Envelope::from_artifact`]; es gibt keinen oeffentlichen Setter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    generation: u8,
    program: Vec<Op>,
    artifact_hash: [u8; 32],
}

impl Envelope {
    /// Liest die Huelle. Der Digest wird erst nach der Struktur geprueft
    /// (Stufen 1 bis 8 vor Stufe 9).
    pub fn parse(bytes: &[u8]) -> Result<Self, EnvelopeError> {
        Self::parse_with_limit(bytes, None)
    }

    /// Wie [`Envelope::parse`], mit Betreibergrenze fuer Stufe 10.
    ///
    /// `max_stream_bytes` ist die Obergrenze der **dekodierten Stromlaenge**
    /// (`tail_start - HEAD_LEN`, REQ-SCE-089), nicht der Anzahl und nicht der
    /// Gesamtlaenge. Ohne Wert ist `E-012` nicht erreichbar; der Betreiberwert
    /// ist offen (P1b).
    pub fn parse_with_limit(
        bytes: &[u8],
        max_stream_bytes: Option<usize>,
    ) -> Result<Self, EnvelopeError> {
        let total = bytes.len();

        // Stufen 1..4: Laenge, Magic, Fassung, Generation.
        if total < MIN_LEN {
            return Err(EnvelopeError::new("E-001", total));
        }
        if bytes[0..4] != MAGIC {
            return Err(EnvelopeError::new("E-007", 0));
        }
        if bytes[4] != ENVELOPE_VERSION {
            return Err(EnvelopeError::new("E-003", 4));
        }
        let generation = bytes[5];
        if generation != GENERATION_V1 && generation != GENERATION_V2 {
            return Err(EnvelopeError::new("E-009", 5));
        }

        // Die Stromgrenze folgt aus der Laenge, nie aus der Anzahl.
        let tail_start = total - TAIL_LEN;
        let declared = u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]) as usize;

        // Stufen 5..8: gemeinsame Stromprozedur mit dem ATCB-Parser.
        let program = dekodiere_strom(&bytes[..tail_start], declared, generation)?;

        // Stufe 9: Tail ist exakt artifact_hash ueber Byte 0 bis tail_start-1.
        let mut eingabe = Vec::with_capacity(DOMAIN_ARTIFACT.len() + tail_start);
        eingabe.extend_from_slice(DOMAIN_ARTIFACT);
        eingabe.extend_from_slice(&bytes[..tail_start]);
        let erwartet = sha256_daten(&eingabe);
        if bytes[tail_start..] != erwartet {
            return Err(EnvelopeError::new("E-006", tail_start));
        }

        // Stufe 10: Stromlaenge, nicht Anzahl (REQ-SCE-089).
        if let Some(grenze) = max_stream_bytes {
            if tail_start - HEAD_LEN > grenze {
                return Err(EnvelopeError::new("E-012", HEAD_LEN));
            }
        }

        Ok(Envelope {
            generation,
            program,
            artifact_hash: erwartet,
        })
    }

    /// Baut eine Huelle aus einem ATCB-Artefakt. Das Artefakt bleibt unveraendert;
    /// nur die Huelle bekommt einen eigenen Digest.
    pub fn from_artifact(artifact: &Artifact) -> Self {
        let mut env = Envelope {
            generation: artifact.generation(),
            program: artifact.program().to_vec(),
            artifact_hash: [0u8; 32],
        };
        env.artifact_hash = env.berechne_hash();
        env
    }

    pub fn generation(&self) -> u8 {
        self.generation
    }

    pub fn program(&self) -> &[Op] {
        &self.program
    }

    /// Anzahl dekodierter Instruktionen (`i`).
    pub fn instruction_count(&self) -> usize {
        self.program.len()
    }

    /// Dekodierte Stromlaenge in Bytes (`p - 10`).
    pub fn stream_len(&self) -> usize {
        self.program.iter().map(|op| 1 + operandenlaenge(op)).sum()
    }

    /// `artifact_hash` der Huelle: der Wert, der im Tail steht.
    pub fn artifact_hash(&self) -> [u8; 32] {
        self.artifact_hash
    }

    fn ohne_hash(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(HEAD_LEN + self.stream_len());
        buf.extend_from_slice(&MAGIC);
        buf.push(ENVELOPE_VERSION);
        buf.push(self.generation);
        buf.extend_from_slice(&(self.program.len() as u32).to_le_bytes());
        for op in &self.program {
            schreibe_op(&mut buf, op);
        }
        buf
    }

    fn berechne_hash(&self) -> [u8; 32] {
        let mut eingabe = Vec::with_capacity(DOMAIN_ARTIFACT.len() + HEAD_LEN + self.stream_len());
        eingabe.extend_from_slice(DOMAIN_ARTIFACT);
        eingabe.extend_from_slice(&self.ohne_hash());
        sha256_daten(&eingabe)
    }

    /// Kanonische Bytes der Huelle inklusive `artifact_hash`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = self.ohne_hash();
        let hash = self.berechne_hash();
        buf.extend_from_slice(&hash);
        buf
    }

    /// Uebergibt den Strom an das ausfuehrbare `ATCB`-Artefakt.
    ///
    /// Damit laeuft die Ausfuehrung ausschliesslich ueber den bestehenden
    /// VM-Pfad; die Huelle dekodiert nichts doppelt und fuehrt nichts selbst aus.
    pub fn into_artifact(self) -> Result<Artifact, EnvelopeError> {
        Artifact::new(self.generation, self.program).map_err(|e| match e {
            ArtifactError::ProgramTooLarge { .. } => EnvelopeError::new("E-012", HEAD_LEN),
            ArtifactError::UnknownGeneration { .. } => EnvelopeError::new("E-009", 5),
            ArtifactError::GenerationMismatch { .. } => EnvelopeError::new("E-011", 0),
            _ => EnvelopeError::new("E-012", HEAD_LEN),
        })
    }
}

/// Operandenlaenge eines bereits dekodierten Opcodes (Schreibrichtung).
fn operandenlaenge(op: &Op) -> usize {
    crate::artifact::operandenbreite(opcode_von(op)).unwrap_or(0)
}

/// Opcode eines dekodierten Befehls. Eine Quelle: `artifact::schreibe_op`.
fn opcode_von(op: &Op) -> u8 {
    let mut buf = Vec::with_capacity(9);
    schreibe_op(&mut buf, op);
    buf[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atcb(generation: u8, program: Vec<Op>) -> Artifact {
        Artifact::new(generation, program).expect("ATCB-Artefakt")
    }

    fn huelle(generation: u8, program: Vec<Op>) -> Envelope {
        Envelope::from_artifact(&atcb(generation, program))
    }

    fn fehler(bytes: &[u8]) -> EnvelopeError {
        Envelope::parse(bytes).expect_err("Huelle wurde akzeptiert")
    }

    #[test]
    fn atce_mit_atcb_wird_akzeptiert() {
        let a = atcb(
            GENERATION_V1,
            vec![Op::Push(1), Op::Push(2), Op::Add, Op::Halt],
        );
        let env = Envelope::from_artifact(&a);
        let bytes = env.to_bytes();
        assert_eq!(&bytes[0..4], b"ATCE");
        assert_eq!(bytes[4], 0x02);
        assert_eq!(bytes[5], GENERATION_V1);

        let gelesen = Envelope::parse(&bytes).expect("Huelle abgelehnt");
        assert_eq!(gelesen.instruction_count(), 4);
        assert_eq!(gelesen.artifact_hash(), env.artifact_hash());

        // Der innere ATCB-Container entsteht ohne Umweg und ist unveraendert.
        let inner = gelesen.into_artifact().expect("ATCB aus Huelle");
        assert_eq!(inner.program(), a.program());
        assert_eq!(inner.hash(), a.hash());
    }

    #[test]
    fn falsche_fassung_abgelehnt() {
        for fassung in [0x01u8, 0x03, 0xFF] {
            let mut bytes = huelle(GENERATION_V1, vec![Op::Halt]).to_bytes();
            bytes[4] = fassung;
            let err = fehler(&bytes);
            assert_eq!(
                (err.code(), err.offset()),
                ("E-003", 4),
                "Fassung {fassung}"
            );
        }
    }

    #[test]
    fn atcb_ohne_huelle_wird_abgelehnt() {
        // Die Huelle akzeptiert kein ATCB-Magic ...
        let a = atcb(GENERATION_V1, vec![Op::Halt]);
        let err = fehler(&a.to_bytes());
        assert_eq!((err.code(), err.offset()), ("E-007", 0));

        // ... und der ATCB-Parser akzeptiert keine ATCE-Huelle.
        let bytes = huelle(GENERATION_V1, vec![Op::Halt]).to_bytes();
        let err = Artifact::parse(&bytes).expect_err("ATCB akzeptierte ATCE");
        assert_eq!((err.code(), err.offset()), ("E-007", 0));
    }

    #[test]
    fn manipulierter_digest_abgelehnt() {
        let mut bytes = huelle(GENERATION_V1, vec![Op::Push(1), Op::Halt]).to_bytes();
        let tail_start = bytes.len() - TAIL_LEN;
        bytes[tail_start] ^= 0xFF;
        let err = fehler(&bytes);
        assert_eq!((err.code(), err.offset()), ("E-006", tail_start));

        // Auch ein manipulierter Strom faellt am Digest auf.
        let mut bytes = huelle(GENERATION_V1, vec![Op::Push(1), Op::Halt]).to_bytes();
        bytes[HEAD_LEN] = 0x01;
        bytes[HEAD_LEN + 1] = 0x02;
        let err = fehler(&bytes);
        assert_eq!(
            (err.code(), err.offset()),
            ("E-006", bytes.len() - TAIL_LEN)
        );
    }

    #[test]
    fn ungueltige_laenge_abgelehnt() {
        for laenge in [0usize, 1, 41] {
            let err = fehler(&vec![b'A'; laenge]);
            assert_eq!((err.code(), err.offset()), ("E-001", laenge));
        }
        // Genau 42 Byte mit Anzahl 0 und leerem Strom: E-004, nicht E-001.
        let bytes = huelle(GENERATION_V1, vec![Op::Halt]).to_bytes();
        let kopf = &bytes[..HEAD_LEN];
        let mut leer = kopf.to_vec();
        leer[6..10].copy_from_slice(&0u32.to_le_bytes());
        let err = fehler(&mit_hash(leer));
        assert_eq!((err.code(), err.offset()), ("E-004", HEAD_LEN));
    }

    fn mit_hash(mut rumpf: Vec<u8>) -> Vec<u8> {
        let mut eingabe = Vec::new();
        eingabe.extend_from_slice(DOMAIN_ARTIFACT);
        eingabe.extend_from_slice(&rumpf);
        let tail = sha256_daten(&eingabe);
        rumpf.extend_from_slice(&tail);
        rumpf
    }

    #[test]
    fn trunkierter_strom_abgelehnt() {
        // Anzahl 1, Push ohne vollstaendigen Operanden: E-002 an tail_start.
        let mut rumpf = Vec::new();
        rumpf.extend_from_slice(&MAGIC);
        rumpf.push(ENVELOPE_VERSION);
        rumpf.push(GENERATION_V1);
        rumpf.extend_from_slice(&1u32.to_le_bytes());
        rumpf.push(0x01);
        rumpf.extend_from_slice(&[0, 0, 0]);
        let tail_start = rumpf.len();
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-002", tail_start));

        // Anzahl 2 bei einer Instruktion: E-005 an tail_start.
        let mut rumpf = Vec::new();
        rumpf.extend_from_slice(&MAGIC);
        rumpf.push(ENVELOPE_VERSION);
        rumpf.push(GENERATION_V1);
        rumpf.extend_from_slice(&2u32.to_le_bytes());
        rumpf.push(0x10);
        let tail_start = rumpf.len();
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-005", tail_start));
    }

    #[test]
    fn unbekannter_opcode_ist_deterministisch() {
        let mut rumpf = Vec::new();
        rumpf.extend_from_slice(&MAGIC);
        rumpf.push(ENVELOPE_VERSION);
        rumpf.push(GENERATION_V1);
        rumpf.extend_from_slice(&1u32.to_le_bytes());
        rumpf.push(0x99);
        let bytes = mit_hash(rumpf);
        let erst = fehler(&bytes);
        let zweit = fehler(&bytes);
        assert_eq!((erst.code(), erst.offset()), ("E-010", HEAD_LEN));
        assert_eq!(
            erst, zweit,
            "zwei Laeufe derselben Eingabe muessen gleich sein"
        );
    }

    #[test]
    fn bytecode_hash_bleibt_unveraendert() {
        // Der fixierte ATCB-Hash gilt weiter; die Huelle hat einen eigenen Digest.
        let a = atcb(GENERATION_V2, vec![Op::Push(1), Op::Halt]);
        let hex: String = a.hash().iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(
            hex,
            "0dee103620709f6113425f657a4e48197ba5f69757dca7a20e4d4a116c13a2b4"
        );
        let env = Envelope::from_artifact(&a);
        assert_ne!(
            env.artifact_hash(),
            a.hash(),
            "Huelle und Bytecode sind getrennt"
        );
        // Auch nach dem Umlauf durch die Huelle unveraendert.
        let inner = Envelope::parse(&env.to_bytes())
            .expect("Huelle")
            .into_artifact()
            .expect("ATCB");
        assert_eq!(inner.hash(), a.hash());
        assert_eq!(
            inner.to_bytes(),
            a.to_bytes(),
            "die Huelle darf die ATCB-Bytes nicht veraendern"
        );
    }

    #[test]
    fn domaene_geht_in_den_digest_ein() {
        let env = huelle(GENERATION_V1, vec![Op::Halt]);
        let rumpf = env.ohne_hash();
        let ohne_domaene = sha256_daten(&rumpf);
        assert_ne!(
            env.artifact_hash(),
            ohne_domaene,
            "artifact_hash muss die Domaene einbeziehen (REQ-SCE-062)"
        );
        assert_eq!(DOMAIN_ARTIFACT.len(), 16, "Domaenenpraefix ist byte-exakt");
    }

    #[test]
    fn groesstwert_nur_mit_grenze() {
        let env = huelle(GENERATION_V1, vec![Op::Push(1), Op::Halt]);
        let bytes = env.to_bytes();
        let strom = env.stream_len();
        // Ohne Grenze: kein E-012.
        assert!(Envelope::parse(&bytes).is_ok());
        // Grenze exakt auf der Stromlaenge: akzeptiert (Grenzwerttest an der Kante).
        assert!(Envelope::parse_with_limit(&bytes, Some(strom)).is_ok());
        // Ein Byte darunter: E-012 bei 10.
        let err = Envelope::parse_with_limit(&bytes, Some(strom - 1)).expect_err("E-012 fehlt");
        assert_eq!((err.code(), err.offset()), ("E-012", HEAD_LEN));
    }

    #[test]
    fn generation_wird_aus_der_huelle_uebernommen() {
        let v1 = huelle(GENERATION_V1, vec![Op::Push(1), Op::Halt]);
        let v2 = huelle(GENERATION_V2, vec![Op::Push(1), Op::Halt]);
        assert_ne!(v1.artifact_hash(), v2.artifact_hash());
        assert_eq!(
            v1.into_artifact().expect("ATCB").generation(),
            GENERATION_V1
        );
        assert_eq!(
            v2.into_artifact().expect("ATCB").generation(),
            GENERATION_V2
        );
    }

    #[test]
    fn unbekannte_generation_abgelehnt() {
        let mut bytes = huelle(GENERATION_V1, vec![Op::Halt]).to_bytes();
        bytes[5] = 0x07;
        let err = fehler(&bytes);
        assert_eq!((err.code(), err.offset()), ("E-009", 5));
    }
}

// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Kanonisches ATCB-Artefakt (ATC-BC-002, Kandidat).
//!
//! Die Bytecode-Generation ist Teil des Artefakts und damit Teil des Hashes.
//! Ein Aufrufer kann sie nicht setzen: `Artifact` hat keine oeffentliche
//! Setter-Methode, `Vm::from_artifact` uebernimmt sie ausschliesslich von hier.
//!
//! Aufbau (alles Little Endian):
//!   0..4   magic  "ATCB"
//!   4      Containerformat (0x01)
//!   5      Generation (1 = ATCB-v1, 2 = ATCB-v2)
//!   6..10  Anzahl Operationen (u32)
//!   10..   Operationsstrom
//!   -32..  SHA-256 ueber alle vorhergehenden Bytes
//!
//! Es gibt keine Konvertierung zwischen Generationen. Ein v1-Artefakt bleibt
//! v1, ein v2-Artefakt bleibt v2.

use crate::vm::Op;

/// Kennung des Containers.
pub const MAGIC: [u8; 4] = *b"ATCB";
/// Unterstuetztes Containerformat.
pub const CONTAINER_VERSION: u8 = 1;
/// Laenge des angehaengten Hashes.
pub const HASH_LEN: usize = 32;
/// Generation ATCB-v1.
pub const GENERATION_V1: u8 = 1;
/// Generation ATCB-v2 (Kandidat).
pub const GENERATION_V2: u8 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactError {
    TooShort,
    BadMagic,
    UnsupportedContainer(u8),
    UnknownGeneration(u8),
    BadHash,
    UnknownOpcode(u8),
    GenerationMismatch { opcode: u8, generation: u8 },
    TruncatedOperand,
    TrailingBytes,
    ProgramTooLarge { size: usize },
}

// --- SHA-256 (FIPS 180-4), ohne Fremdbibliothek -------------------------------

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// SHA-256 ueber beliebige Bytes.
pub fn sha256_daten(data: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut nachricht = data.to_vec();
    let bitlen = (data.len() as u64).wrapping_mul(8);
    nachricht.push(0x80);
    while nachricht.len() % 64 != 56 {
        nachricht.push(0);
    }
    nachricht.extend_from_slice(&bitlen.to_be_bytes());
    for block in nachricht.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut out = [0u8; 32];
    for (i, wert) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&wert.to_be_bytes());
    }
    out
}

// --- Operationskodierung (stabil, Teil des Artefakts) -------------------------

fn opcode(op: &Op) -> u8 {
    match op {
        Op::Push(_) => 0x01,
        Op::Add => 0x02,
        Op::Sub => 0x03,
        Op::Mul => 0x04,
        Op::Div => 0x05,
        Op::Dup => 0x06,
        Op::Swap => 0x07,
        Op::Eq => 0x08,
        Op::Lt => 0x09,
        Op::Load(_) => 0x0A,
        Op::Store(_) => 0x0B,
        Op::Caller => 0x0C,
        Op::Jump(_) => 0x0D,
        Op::JumpIfZero(_) => 0x0E,
        Op::JumpIfNotZero(_) => 0x0F,
        Op::Halt => 0x10,
        Op::Assert => 0x11,
        Op::Revert(_) => 0x12,
        Op::RevertIf => 0x13,
    }
}

fn ist_v2_opcode(code: u8) -> bool {
    (0x11..=0x13).contains(&code)
}

fn schreibe_op(buf: &mut Vec<u8>, op: &Op) {
    buf.push(opcode(op));
    match op {
        Op::Push(v) => buf.extend_from_slice(&v.to_le_bytes()),
        Op::Load(s) | Op::Store(s) => buf.extend_from_slice(&(*s as u32).to_le_bytes()),
        Op::Jump(t) | Op::JumpIfZero(t) | Op::JumpIfNotZero(t) => {
            buf.extend_from_slice(&(*t as u32).to_le_bytes())
        }
        Op::Revert(c) => buf.extend_from_slice(&(*c as u32).to_le_bytes()),
        _ => {}
    }
}

fn lies_op(bytes: &[u8], pos: &mut usize, generation: u8) -> Result<Op, ArtifactError> {
    if *pos >= bytes.len() {
        return Err(ArtifactError::TruncatedOperand);
    }
    let code = bytes[*pos];
    *pos += 1;
    if ist_v2_opcode(code) && generation != GENERATION_V2 {
        return Err(ArtifactError::GenerationMismatch {
            opcode: code,
            generation,
        });
    }
    let operand32 = |pos: &mut usize| -> Result<u32, ArtifactError> {
        if *pos + 4 > bytes.len() {
            return Err(ArtifactError::TruncatedOperand);
        }
        let v = u32::from_le_bytes([
            bytes[*pos],
            bytes[*pos + 1],
            bytes[*pos + 2],
            bytes[*pos + 3],
        ]);
        *pos += 4;
        Ok(v)
    };
    let operand64 = |pos: &mut usize| -> Result<u64, ArtifactError> {
        if *pos + 8 > bytes.len() {
            return Err(ArtifactError::TruncatedOperand);
        }
        let mut b = [0u8; 8];
        b.copy_from_slice(&bytes[*pos..*pos + 8]);
        *pos += 8;
        Ok(u64::from_le_bytes(b))
    };
    Ok(match code {
        0x01 => Op::Push(operand64(pos)?),
        0x02 => Op::Add,
        0x03 => Op::Sub,
        0x04 => Op::Mul,
        0x05 => Op::Div,
        0x06 => Op::Dup,
        0x07 => Op::Swap,
        0x08 => Op::Eq,
        0x09 => Op::Lt,
        0x0A => Op::Load(operand32(pos)? as usize),
        0x0B => Op::Store(operand32(pos)? as usize),
        0x0C => Op::Caller,
        0x0D => Op::Jump(operand32(pos)? as usize),
        0x0E => Op::JumpIfZero(operand32(pos)? as usize),
        0x0F => Op::JumpIfNotZero(operand32(pos)? as usize),
        0x10 => Op::Halt,
        0x11 => Op::Assert,
        0x12 => Op::Revert(operand32(pos)? as u64),
        0x13 => Op::RevertIf,
        other => return Err(ArtifactError::UnknownOpcode(other)),
    })
}

// --- Artefakt ---------------------------------------------------------------

/// Kanonisches Artefakt. Generation ist privat und nicht setzbar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    generation: u8,
    program: Vec<Op>,
}

impl Artifact {
    /// Baut ein Artefakt. Nur Werkzeuge und Tests erzeugen Artefakte;
    /// die Ausfuehrung liest sie ausschliesslich aus Bytes.
    pub fn new(generation: u8, program: Vec<Op>) -> Result<Self, ArtifactError> {
        if generation != GENERATION_V1 && generation != GENERATION_V2 {
            return Err(ArtifactError::UnknownGeneration(generation));
        }
        if program.len() > crate::vm::MAX_PROGRAM_OPS {
            return Err(ArtifactError::ProgramTooLarge {
                size: program.len(),
            });
        }
        if generation == GENERATION_V1 {
            for op in &program {
                let code = opcode(op);
                if ist_v2_opcode(code) {
                    return Err(ArtifactError::GenerationMismatch {
                        opcode: code,
                        generation,
                    });
                }
            }
        }
        Ok(Artifact {
            generation,
            program,
        })
    }

    pub fn generation(&self) -> u8 {
        self.generation
    }

    pub fn program(&self) -> &[Op] {
        &self.program
    }

    fn ohne_hash(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(10 + self.program.len() * 9);
        buf.extend_from_slice(&MAGIC);
        buf.push(CONTAINER_VERSION);
        buf.push(self.generation);
        buf.extend_from_slice(&(self.program.len() as u32).to_le_bytes());
        for op in &self.program {
            schreibe_op(&mut buf, op);
        }
        buf
    }

    /// Kanonische Bytes inklusive angehaengtem SHA-256.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = self.ohne_hash();
        let hash = sha256_daten(&buf);
        buf.extend_from_slice(&hash);
        buf
    }

    /// SHA-256 ueber Kopf und Operationsstrom. Die Generation ist enthalten,
    /// eine Versionsaenderung aendert damit den Hash.
    pub fn hash(&self) -> [u8; 32] {
        sha256_daten(&self.ohne_hash())
    }

    /// Liest ein Artefakt aus Bytes. Jede Abweichung wird abgelehnt.
    pub fn parse(bytes: &[u8]) -> Result<Self, ArtifactError> {
        if bytes.len() < 10 + HASH_LEN {
            return Err(ArtifactError::TooShort);
        }
        if bytes[0..4] != MAGIC {
            return Err(ArtifactError::BadMagic);
        }
        if bytes[4] != CONTAINER_VERSION {
            return Err(ArtifactError::UnsupportedContainer(bytes[4]));
        }
        let generation = bytes[5];
        if generation != GENERATION_V1 && generation != GENERATION_V2 {
            return Err(ArtifactError::UnknownGeneration(generation));
        }
        let rumpf = &bytes[..bytes.len() - HASH_LEN];
        let erwarteter_hash = &bytes[bytes.len() - HASH_LEN..];
        if sha256_daten(rumpf) != erwarteter_hash {
            return Err(ArtifactError::BadHash);
        }
        let count = u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]) as usize;
        if count > crate::vm::MAX_PROGRAM_OPS {
            return Err(ArtifactError::ProgramTooLarge { size: count });
        }
        let mut pos = 10usize;
        let mut program = Vec::with_capacity(count);
        for _ in 0..count {
            program.push(lies_op(rumpf, &mut pos, generation)?);
        }
        if pos != rumpf.len() {
            return Err(ArtifactError::TrailingBytes);
        }
        Ok(Artifact {
            generation,
            program,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    #[test]
    fn sha256_testvektoren() {
        assert_eq!(
            hex(&sha256_daten(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hex(&sha256_daten(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        let lang = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        assert_eq!(
            hex(&sha256_daten(lang)),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    #[test]
    fn roundtrip_erhaelt_generation_und_programm() {
        let prog = vec![
            Op::Push(7),
            Op::Add,
            Op::Load(3),
            Op::JumpIfZero(0),
            Op::Halt,
        ];
        let a = Artifact::new(GENERATION_V1, prog.clone()).unwrap();
        let b = Artifact::parse(&a.to_bytes()).unwrap();
        assert_eq!(b.generation(), GENERATION_V1);
        assert_eq!(b.program(), prog.as_slice());
        assert_eq!(a.hash(), b.hash());
    }

    #[test]
    fn roundtrip_v2_mit_pruef_und_ruecklaufbefehlen() {
        let prog = vec![
            Op::Push(1),
            Op::Assert,
            Op::Push(0),
            Op::RevertIf,
            Op::Revert(0x0100),
        ];
        let a = Artifact::new(GENERATION_V2, prog.clone()).unwrap();
        let b = Artifact::parse(&a.to_bytes()).unwrap();
        assert_eq!(b.generation(), GENERATION_V2);
        assert_eq!(b.program(), prog.as_slice());
    }

    #[test]
    fn generation_ist_hashgebunden() {
        let prog = vec![Op::Push(1), Op::Halt];
        let v1 = Artifact::new(GENERATION_V1, prog.clone()).unwrap();
        let v2 = Artifact::new(GENERATION_V2, prog).unwrap();
        assert_ne!(
            v1.hash(),
            v2.hash(),
            "gleiche Ops, andere Generation: Hash muss abweichen"
        );
        // Manipulation des Generationsbytes faellt auf.
        let mut bytes = v1.to_bytes();
        bytes[5] = GENERATION_V2;
        assert_eq!(Artifact::parse(&bytes), Err(ArtifactError::BadHash));
    }

    #[test]
    fn unbekannte_generation_abgelehnt() {
        assert_eq!(
            Artifact::new(3, vec![Op::Halt]),
            Err(ArtifactError::UnknownGeneration(3))
        );
        assert_eq!(
            Artifact::new(0, vec![Op::Halt]),
            Err(ArtifactError::UnknownGeneration(0))
        );
    }

    #[test]
    fn v2_befehl_in_v1_artefakt_abgelehnt() {
        assert_eq!(
            Artifact::new(GENERATION_V1, vec![Op::Push(1), Op::Assert]),
            Err(ArtifactError::GenerationMismatch {
                opcode: 0x11,
                generation: 1
            })
        );
    }

    #[test]
    fn unbekannter_opcode_abgelehnt() {
        let a = Artifact::new(GENERATION_V1, vec![Op::Halt]).unwrap();
        let mut bytes = a.to_bytes();
        let pos = 10;
        bytes[pos] = 0x99;
        let laenge = bytes.len() - HASH_LEN;
        let hash = super::sha256_daten(&bytes[..laenge]);
        bytes.truncate(laenge);
        bytes.extend_from_slice(&hash);
        assert_eq!(
            Artifact::parse(&bytes),
            Err(ArtifactError::UnknownOpcode(0x99))
        );
    }

    #[test]
    fn zu_kurz_und_falsche_magic_abgelehnt() {
        assert_eq!(Artifact::parse(&[0u8; 5]), Err(ArtifactError::TooShort));
        let a = Artifact::new(GENERATION_V1, vec![Op::Halt]).unwrap();
        let mut bytes = a.to_bytes();
        bytes[0] = b'X';
        assert_eq!(Artifact::parse(&bytes), Err(ArtifactError::BadMagic));
    }

    #[test]
    fn ueberzaehlige_bytes_abgelehnt() {
        let a = Artifact::new(GENERATION_V1, vec![Op::Halt]).unwrap();
        let mut rumpf = a.ohne_hash();
        rumpf.push(0x10); // ein Op mehr als im Kopf steht
        let hash = super::sha256_daten(&rumpf);
        let mut bytes = rumpf;
        bytes.extend_from_slice(&hash);
        assert_eq!(Artifact::parse(&bytes), Err(ArtifactError::TrailingBytes));
    }

    #[test]
    fn keine_automatische_konvertierung() {
        // Es gibt keine Methode, die Generation zu wechseln; parse liefert
        // genau die Generation aus dem Artefakt.
        let a = Artifact::new(GENERATION_V1, vec![Op::Push(1), Op::Halt]).unwrap();
        assert_eq!(
            Artifact::parse(&a.to_bytes()).unwrap().generation(),
            GENERATION_V1
        );
    }

    #[test]
    fn vm_uebernimmt_generation_aus_artefakt() {
        let a = Artifact::new(GENERATION_V2, vec![Op::Push(1), Op::Halt]).unwrap();
        assert_eq!(crate::vm::Vm::from_artifact(&a).generation(), GENERATION_V2);
        let b = Artifact::new(GENERATION_V1, vec![Op::Push(1), Op::Halt]).unwrap();
        assert_eq!(crate::vm::Vm::from_artifact(&b).generation(), GENERATION_V1);
    }
}

#[cfg(test)]
mod hash_stabilitaet {
    use super::*;
    use crate::vm::Op;

    /// Die Gastabellen-Versionierung darf den Bytecode und seinen Hash nicht
    /// veraendern. Dieser Test fixiert den Hash fuer ein festes Programm: eine
    /// Aenderung an Container, Opcode-Kodierung oder Hashverfahren wird rot.
    #[test]
    fn gastabellen_version_aendert_den_bytecode_hash_nicht() {
        let a = Artifact::new(GENERATION_V2, vec![Op::Push(1), Op::Halt]).unwrap();
        let hex: String = a.hash().iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(
            hex, "0dee103620709f6113425f657a4e48197ba5f69757dca7a20e4d4a116c13a2b4",
            "Bytecode-Hash hat sich geaendert; das Artefaktformat ist eingefroren"
        );
        // Derselbe Hash entsteht aus den Bytes, nicht nur aus dem Feld.
        assert_eq!(Artifact::parse(&a.to_bytes()).unwrap().hash(), a.hash());
    }
}

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
//!   6..10  Anzahl Operationen (u32, Obergrenze, kein Endmarker)
//!   10..   Operationsstrom
//!   -32..  SHA-256 ueber alle vorhergehenden Bytes
//!
//! Pruefreihenfolge (D1-B/D1-C, §19.2a): Laenge, Magic, Format, Generation,
//! Strom, ueberzaehlige Bytes, leerer Strom, Integritaet, Groesstwert. Ein
//! Strukturfehler wird also **vor** dem Digest erkannt, und die Groessenpruefung
//! misst die **dekodierte** Laenge, nicht die deklarierte Anzahl. Fehleridentitaet
//! ist das Paar (Code, Offset); die Offsets zaehlen ab dem ersten Byte.
//!
//! Es gibt keine Konvertierung zwischen Generationen. Ein v1-Artefakt bleibt
//! v1, ein v2-Artefakt bleibt v2.

use crate::vm::Op;

/// Kennung des Containers.
pub const MAGIC: [u8; 4] = *b"ATCB";
/// Unterstuetztes Containerformat.
pub const CONTAINER_VERSION: u8 = 1;
/// Laenge des Kopfes bis zum Strom.
pub const HEAD_LEN: usize = 10;
/// Laenge des angehaengten Hashes.
pub const HASH_LEN: usize = 32;
/// Generation ATCB-v1.
pub const GENERATION_V1: u8 = 1;
/// Generation ATCB-v2 (Kandidat).
pub const GENERATION_V2: u8 = 2;

/// Fehler beim Bauen oder Lesen eines Artefakts.
///
/// Jede Variante traegt den Offset des ersten nicht akzeptierten Bytes. Bei
/// Fehlern aus `new` stehen die Kopfpositionen (5 fuer die Generation, 0 sonst),
/// weil dort noch keine Eingabebytes existieren.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactError {
    /// `E-001` Eingabe kuerzer als Kopf plus Hash. Offset: Eingabelaenge.
    TooShort { offset: usize },
    /// `E-007` Magic ist nicht `ATCB`. Offset: `0`.
    BadMagic { offset: usize },
    /// `E-003` Containerformat wird nicht unterstuetzt. Offset: `4`.
    UnsupportedContainer { version: u8, offset: usize },
    /// `E-009` Generation ist unbekannt. Offset: `5`.
    UnknownGeneration { generation: u8, offset: usize },
    /// `E-004` Die Anzahl ist `0` und der Strom ist leer. Offset: `10`.
    EmptyStream { offset: usize },
    /// `E-005` Der Strom endet an einer Instruktionsgrenze, bevor die
    /// deklarierte Anzahl erreicht ist. Offset: `tail_start`.
    CountMismatch { offset: usize },
    /// `E-006` Der Tail passt nicht zum Inhalt. Offset: `tail_start`.
    BadHash { offset: usize },
    /// `E-010` Opcode ist unbekannt. Offset: Position des Opcodes.
    UnknownOpcode { opcode: u8, offset: usize },
    /// `E-011` Opcode gehoert zu Generation 2, das Artefakt ist Generation 1.
    /// Offset: Position des Opcodes.
    GenerationMismatch {
        opcode: u8,
        generation: u8,
        offset: usize,
    },
    /// `E-002` Ein Operand reicht nicht bis `tail_start`. Offset: `tail_start`.
    TruncatedOperand { offset: usize },
    /// `E-008` Nach der deklarierten Anzahl bleiben Bytes vor dem Hash.
    /// Offset: Position des ersten ueberzaehligen Bytes.
    TrailingBytes { offset: usize },
    /// `E-012` Mehr Operationen als `MAX_PROGRAM_OPS`. Gemessen werden die
    /// **dekodierten** Operationen, nicht die deklarierte Anzahl (D1-C).
    ProgramTooLarge { ops: usize, offset: usize },
}

impl ArtifactError {
    /// Normativer Fehlercode aus `REQ-SCE-081`.
    pub fn code(&self) -> &'static str {
        match self {
            ArtifactError::TooShort { .. } => "E-001",
            ArtifactError::TruncatedOperand { .. } => "E-002",
            ArtifactError::UnsupportedContainer { .. } => "E-003",
            ArtifactError::EmptyStream { .. } => "E-004",
            ArtifactError::CountMismatch { .. } => "E-005",
            ArtifactError::BadHash { .. } => "E-006",
            ArtifactError::BadMagic { .. } => "E-007",
            ArtifactError::TrailingBytes { .. } => "E-008",
            ArtifactError::UnknownGeneration { .. } => "E-009",
            ArtifactError::UnknownOpcode { .. } => "E-010",
            ArtifactError::GenerationMismatch { .. } => "E-011",
            ArtifactError::ProgramTooLarge { .. } => "E-012",
        }
    }

    /// Offset des ersten nicht akzeptierten Bytes.
    pub fn offset(&self) -> usize {
        match self {
            ArtifactError::TooShort { offset }
            | ArtifactError::BadMagic { offset }
            | ArtifactError::UnsupportedContainer { offset, .. }
            | ArtifactError::UnknownGeneration { offset, .. }
            | ArtifactError::EmptyStream { offset }
            | ArtifactError::CountMismatch { offset }
            | ArtifactError::BadHash { offset }
            | ArtifactError::UnknownOpcode { offset, .. }
            | ArtifactError::GenerationMismatch { offset, .. }
            | ArtifactError::TruncatedOperand { offset }
            | ArtifactError::TrailingBytes { offset }
            | ArtifactError::ProgramTooLarge { offset, .. } => *offset,
        }
    }
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

/// Operandenbreite je Opcode (ATC-BC-002 §16). `None` = unbekannter Opcode.
fn operandenbreite(code: u8) -> Option<usize> {
    Some(match code {
        0x01 => 8, // Push(u64)
        0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x09 | 0x0C | 0x10 | 0x11 | 0x13 => 0,
        0x0A | 0x0B | 0x0D | 0x0E | 0x0F | 0x12 => 4,
        _ => return None,
    })
}

/// Dekodiert genau eine Instruktion. `tail_start` ist die einzige Stromgrenze.
fn dekodiere_op(rumpf: &[u8], pos: &mut usize, generation: u8) -> Result<Op, ArtifactError> {
    let tail_start = rumpf.len();
    let start = *pos;
    let code = rumpf[start];
    let Some(breite) = operandenbreite(code) else {
        return Err(ArtifactError::UnknownOpcode {
            opcode: code,
            offset: start,
        });
    };
    if ist_v2_opcode(code) && generation != GENERATION_V2 {
        return Err(ArtifactError::GenerationMismatch {
            opcode: code,
            generation,
            offset: start,
        });
    }
    if start + 1 + breite > tail_start {
        return Err(ArtifactError::TruncatedOperand { offset: tail_start });
    }
    let operand = &rumpf[start + 1..start + 1 + breite];
    *pos = start + 1 + breite;
    Ok(match code {
        0x01 => {
            let mut b = [0u8; 8];
            b.copy_from_slice(operand);
            Op::Push(u64::from_le_bytes(b))
        }
        0x02 => Op::Add,
        0x03 => Op::Sub,
        0x04 => Op::Mul,
        0x05 => Op::Div,
        0x06 => Op::Dup,
        0x07 => Op::Swap,
        0x08 => Op::Eq,
        0x09 => Op::Lt,
        0x0A => {
            Op::Load(u32::from_le_bytes([operand[0], operand[1], operand[2], operand[3]]) as usize)
        }
        0x0B => {
            Op::Store(u32::from_le_bytes([operand[0], operand[1], operand[2], operand[3]]) as usize)
        }
        0x0C => Op::Caller,
        0x0D => {
            Op::Jump(u32::from_le_bytes([operand[0], operand[1], operand[2], operand[3]]) as usize)
        }
        0x0E => Op::JumpIfZero(
            u32::from_le_bytes([operand[0], operand[1], operand[2], operand[3]]) as usize,
        ),
        0x0F => {
            Op::JumpIfNotZero(
                u32::from_le_bytes([operand[0], operand[1], operand[2], operand[3]]) as usize,
            )
        }
        0x10 => Op::Halt,
        0x11 => Op::Assert,
        0x12 => {
            Op::Revert(u32::from_le_bytes([operand[0], operand[1], operand[2], operand[3]]) as u64)
        }
        0x13 => Op::RevertIf,
        other => {
            return Err(ArtifactError::UnknownOpcode {
                opcode: other,
                offset: start,
            })
        }
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
            return Err(ArtifactError::UnknownGeneration {
                generation,
                offset: 5,
            });
        }
        if program.len() > crate::vm::MAX_PROGRAM_OPS {
            return Err(ArtifactError::ProgramTooLarge {
                ops: program.len(),
                offset: 0,
            });
        }
        if generation == GENERATION_V1 {
            for op in &program {
                let code = opcode(op);
                if ist_v2_opcode(code) {
                    return Err(ArtifactError::GenerationMismatch {
                        opcode: code,
                        generation,
                        offset: 0,
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
        let mut buf = Vec::with_capacity(HEAD_LEN + self.program.len() * 9);
        buf.extend_from_slice(&MAGIC);
        buf.push(CONTAINER_VERSION);
        buf.push(self.generation);
        buf.extend_from_slice(&(self.program.len() as u32).to_le_bytes());
        for op in &self.program {
            schreibe_op(&mut buf, op);
        }
        buf
    }

    /// Kanonische Bytes inklusive angehaengtem Hash.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = self.ohne_hash();
        let hash = sha256_daten(&buf);
        buf.extend_from_slice(&hash);
        buf
    }

    /// Hash ueber Kopf und Strom. Genau dieser Wert steht im Tail.
    pub fn hash(&self) -> [u8; 32] {
        sha256_daten(&self.ohne_hash())
    }

    /// Liest ein Artefakt nach der Pruefreihenfolge aus dem Modulkopf.
    ///
    /// Die Struktur wird **vor** der Integritaet geprueft, die Groessenpruefung
    /// misst die dekodierte Laenge, und `c` ist eine Obergrenze ohne
    /// Endmarkerfunktion.
    pub fn parse(bytes: &[u8]) -> Result<Self, ArtifactError> {
        let total = bytes.len();

        // Stufe 1..4: Laenge, Magic, Format, Generation.
        if total < HEAD_LEN + HASH_LEN {
            return Err(ArtifactError::TooShort { offset: total });
        }
        if bytes[0..4] != MAGIC {
            return Err(ArtifactError::BadMagic { offset: 0 });
        }
        if bytes[4] != CONTAINER_VERSION {
            return Err(ArtifactError::UnsupportedContainer {
                version: bytes[4],
                offset: 4,
            });
        }
        let generation = bytes[5];
        if generation != GENERATION_V1 && generation != GENERATION_V2 {
            return Err(ArtifactError::UnknownGeneration {
                generation,
                offset: 5,
            });
        }

        // Die Stromgrenze folgt aus der Laenge, nie aus der Anzahl.
        let rumpf = &bytes[..total - HASH_LEN];
        let tail_start = rumpf.len();
        let declared = u32::from_le_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]) as usize;

        // Stufen 5..8: ein Durchlauf. Keine Vorabreservierung ueber `c`.
        let mut program: Vec<Op> = Vec::with_capacity(declared.min(tail_start - HEAD_LEN));
        let mut pos = HEAD_LEN;
        while program.len() < declared && pos < tail_start {
            program.push(dekodiere_op(rumpf, &mut pos, generation)?);
        }
        if program.len() < declared {
            return Err(ArtifactError::CountMismatch { offset: tail_start });
        }
        if pos < tail_start {
            return Err(ArtifactError::TrailingBytes { offset: pos });
        }
        if program.is_empty() {
            return Err(ArtifactError::EmptyStream { offset: HEAD_LEN });
        }

        // Stufe 9: Integritaet nach der Struktur.
        if sha256_daten(rumpf) != bytes[tail_start..] {
            return Err(ArtifactError::BadHash { offset: tail_start });
        }

        // Stufe 10: dekodierte Laenge, nicht die deklarierte Anzahl (D1-C).
        if program.len() > crate::vm::MAX_PROGRAM_OPS {
            return Err(ArtifactError::ProgramTooLarge {
                ops: program.len(),
                offset: HEAD_LEN,
            });
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

    /// Baut einen Container mit neu berechnetem Hash (fuer Strukturpruefungen).
    fn mit_hash(mut rumpf: Vec<u8>) -> Vec<u8> {
        let hash = sha256_daten(&rumpf);
        rumpf.extend_from_slice(&hash);
        rumpf
    }

    fn fehler(bytes: &[u8]) -> ArtifactError {
        Artifact::parse(bytes).expect_err("Artefakt wurde akzeptiert")
    }

    fn kopf(generation: u8, count: u32) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&MAGIC);
        buf.push(CONTAINER_VERSION);
        buf.push(generation);
        buf.extend_from_slice(&count.to_le_bytes());
        buf
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
    fn huelle_bleibt_atcb_fassung_1() {
        // D1-B/D1-C aendern das Format nicht: Magic und Fassung bleiben.
        let a = Artifact::new(GENERATION_V1, vec![Op::Halt]).unwrap();
        let bytes = a.to_bytes();
        assert_eq!(&bytes[0..4], b"ATCB");
        assert_eq!(bytes[4], 1);
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
        // Manipulation des Generationsbytes faellt auf (Integritaet, E-006).
        let mut bytes = v1.to_bytes();
        bytes[5] = GENERATION_V2;
        let err = fehler(&bytes);
        assert_eq!(
            (err.code(), err.offset()),
            ("E-006", bytes.len() - HASH_LEN)
        );
    }

    #[test]
    fn unbekannte_generation_abgelehnt() {
        assert_eq!(
            Artifact::new(3, vec![Op::Halt]),
            Err(ArtifactError::UnknownGeneration {
                generation: 3,
                offset: 5
            })
        );
        assert_eq!(
            Artifact::new(0, vec![Op::Halt]),
            Err(ArtifactError::UnknownGeneration {
                generation: 0,
                offset: 5
            })
        );
        let mut bytes = Artifact::new(GENERATION_V1, vec![Op::Halt])
            .unwrap()
            .to_bytes();
        bytes[5] = 0x07;
        let err = fehler(&bytes);
        assert_eq!((err.code(), err.offset()), ("E-009", 5));
    }

    #[test]
    fn v2_befehl_in_v1_artefakt_abgelehnt() {
        assert_eq!(
            Artifact::new(GENERATION_V1, vec![Op::Push(1), Op::Assert]),
            Err(ArtifactError::GenerationMismatch {
                opcode: 0x11,
                generation: 1,
                offset: 0
            })
        );
        // Aus Bytes: E-011 an der Position des Opcodes, vor dem Digest.
        let bytes = Artifact::new(GENERATION_V2, vec![Op::Push(1), Op::Assert])
            .unwrap()
            .to_bytes();
        let mut rumpf = bytes[..bytes.len() - HASH_LEN].to_vec();
        rumpf[5] = GENERATION_V1;
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-011", 19));
    }

    #[test]
    fn unbekannter_opcode_abgelehnt() {
        let a = Artifact::new(GENERATION_V1, vec![Op::Halt]).unwrap();
        let bytes = a.to_bytes();
        let mut rumpf = bytes[..bytes.len() - HASH_LEN].to_vec();
        rumpf[HEAD_LEN] = 0x99;
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-010", HEAD_LEN));
    }

    #[test]
    fn zu_kurz_und_falsche_magic_abgelehnt() {
        let err = fehler(&[0u8; 5]);
        assert_eq!((err.code(), err.offset()), ("E-001", 5));
        let a = Artifact::new(GENERATION_V1, vec![Op::Halt]).unwrap();
        let mut bytes = a.to_bytes();
        bytes[0] = b'X';
        let err = fehler(&bytes);
        assert_eq!((err.code(), err.offset()), ("E-007", 0));
    }

    #[test]
    fn fremdes_containerformat_abgelehnt() {
        for fassung in [0x02u8, 0x03, 0xFF] {
            let mut bytes = Artifact::new(GENERATION_V1, vec![Op::Halt])
                .unwrap()
                .to_bytes();
            bytes[4] = fassung;
            let err = fehler(&bytes);
            assert_eq!((err.code(), err.offset()), ("E-003", 4), "Format {fassung}");
        }
    }

    #[test]
    fn ueberzaehlige_bytes_abgelehnt() {
        let a = Artifact::new(GENERATION_V1, vec![Op::Halt]).unwrap();
        let mut rumpf = a.ohne_hash();
        rumpf.push(0x10); // ein Op mehr als im Kopf steht
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-008", 11));
    }

    #[test]
    fn zu_kleine_anzahl_ergibt_trailing_bytes() {
        // Anzahl 2 -> 1, beide Instruktionen vorhanden: E-008 bei 19.
        let a = Artifact::new(GENERATION_V1, vec![Op::Push(0), Op::Halt]).unwrap();
        let mut rumpf = a.ohne_hash();
        rumpf[6..10].copy_from_slice(&1u32.to_le_bytes());
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-008", 19));
    }

    #[test]
    fn zu_grosse_anzahl_an_der_grenze_ergibt_count_mismatch() {
        // Anzahl 2 bei einer Instruktion: E-005 an tail_start, nicht E-002.
        let a = Artifact::new(GENERATION_V1, vec![Op::Push(0), Op::Halt]).unwrap();
        let mut rumpf = a.ohne_hash();
        rumpf.truncate(rumpf.len() - 1); // Halt entfernen, Anzahl bleibt 2
        let tail_start = rumpf.len();
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-005", tail_start));
    }

    #[test]
    fn abgeschnittener_operand_ergibt_e_002() {
        // Push braucht acht Operandenbytes; drei bleiben bis tail_start.
        let mut rumpf = kopf(GENERATION_V1, 1);
        rumpf.push(0x01);
        rumpf.extend_from_slice(&[0, 0, 0]);
        let tail_start = rumpf.len();
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-002", tail_start));
    }

    #[test]
    fn anzahl_null_mit_bytes_ergibt_e_008() {
        // c = 0, aber der Strom ist nicht leer: E-008, nicht E-004.
        let mut rumpf = kopf(GENERATION_V1, 0);
        rumpf.push(0x10);
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-008", HEAD_LEN));
    }

    #[test]
    fn leerer_strom_ergibt_e_004() {
        let rumpf = kopf(GENERATION_V1, 0);
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-004", HEAD_LEN));
    }

    #[test]
    fn deklarierte_anzahl_wird_nicht_als_groesse_benutzt() {
        // c = 0xFFFFFFFF bei zwei Instruktionen: E-005, nicht E-012 (D1-C).
        let a = Artifact::new(GENERATION_V1, vec![Op::Push(0), Op::Halt]).unwrap();
        let mut rumpf = a.ohne_hash();
        rumpf[6..10].copy_from_slice(&u32::MAX.to_le_bytes());
        let tail_start = rumpf.len();
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-005", tail_start));
    }

    #[test]
    fn groesstwert_misst_die_dekodierte_laenge() {
        // Mehr als MAX_PROGRAM_OPS dekodierte Operationen: E-012, Offset 10.
        let zu_viele = crate::vm::MAX_PROGRAM_OPS + 1;
        let mut rumpf = kopf(GENERATION_V1, zu_viele as u32);
        rumpf.extend(std::iter::repeat(0x10u8).take(zu_viele)); // Halt je Instruktion
        let err = fehler(&mit_hash(rumpf));
        assert_eq!((err.code(), err.offset()), ("E-012", HEAD_LEN));
        assert_eq!(
            err,
            ArtifactError::ProgramTooLarge {
                ops: zu_viele,
                offset: HEAD_LEN
            }
        );
    }

    #[test]
    fn struktur_vor_digest() {
        // Strukturmutation mit altem (jetzt falschem) Tail: Strukturcode gewinnt.
        let a = Artifact::new(GENERATION_V1, vec![Op::Push(0), Op::Halt]).unwrap();
        let mut bytes = a.to_bytes();
        bytes[HEAD_LEN] = 0x99;
        let err = fehler(&bytes);
        assert_eq!((err.code(), err.offset()), ("E-010", HEAD_LEN));

        // Reine Integritaetsverletzung ergibt E-006.
        let mut nur_tail = a.to_bytes();
        let letzte = nur_tail.len() - 1;
        nur_tail[letzte] ^= 0xFF;
        let err = fehler(&nur_tail);
        assert_eq!(
            (err.code(), err.offset()),
            ("E-006", bytes.len() - HASH_LEN)
        );
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

    #[test]
    fn fehlercodes_sind_die_geschlossene_liste() {
        let alle = [
            ArtifactError::TooShort { offset: 0 },
            ArtifactError::TruncatedOperand { offset: 0 },
            ArtifactError::UnsupportedContainer {
                version: 3,
                offset: 4,
            },
            ArtifactError::EmptyStream { offset: 0 },
            ArtifactError::CountMismatch { offset: 0 },
            ArtifactError::BadHash { offset: 0 },
            ArtifactError::BadMagic { offset: 0 },
            ArtifactError::TrailingBytes { offset: 0 },
            ArtifactError::UnknownGeneration {
                generation: 9,
                offset: 5,
            },
            ArtifactError::UnknownOpcode {
                opcode: 0x99,
                offset: 0,
            },
            ArtifactError::GenerationMismatch {
                opcode: 0x11,
                generation: 1,
                offset: 0,
            },
            ArtifactError::ProgramTooLarge { ops: 0, offset: 0 },
        ];
        let codes: Vec<&str> = alle.iter().map(|e| e.code()).collect();
        assert_eq!(
            codes,
            vec![
                "E-001", "E-002", "E-003", "E-004", "E-005", "E-006", "E-007", "E-008", "E-009",
                "E-010", "E-011", "E-012"
            ]
        );
    }
}

#[cfg(test)]
mod hash_stabilitaet {
    use super::*;
    use crate::vm::Op;

    /// Die Gastabellen-Versionierung darf den Bytecode und seinen Hash nicht
    /// veraendern. Dieser Test fixiert den Hash fuer ein festes Programm: eine
    /// Aenderung an Container, Opcode-Kodierung oder Hashverfahren wird rot.
    /// D1-B/D1-C aendern nur Pruefungen, nicht das Format; der Wert bleibt.
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

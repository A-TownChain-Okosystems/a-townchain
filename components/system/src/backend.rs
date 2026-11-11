// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Uebersetzer-Backend: ATCLang-Bytecode (atc-core) -> ATCB-Operationen (atc-vm).
//!
//! ## Was dieses Backend zusichert
//! Fuer das unterstuetzte Subset ist die Abbildung **byte-genau**: jede
//! ATCLang-Instruktion wird auf eine feste Folge von ATCB-Operationen gelegt,
//! Sprungziele werden absolut umgerechnet, und alles, was nicht exakt abbildbar
//! ist, wird **abgelehnt** statt genaehert.
//!
//! ## Was es ausdruecklich NICHT zusichert
//! Zwei Semantik-Unterschiede zwischen Referenz-Interpreter (atc-core) und
//! Ketten-VM (atc-vm) bleiben bestehen und werden nicht versteckt:
//!
//! 1. **Zahlenbereich.** atc-core rechnet mit `i64` und **gepruefter**
//!    Arithmetik (Ueberlauf ist ein Fehler). atc-vm rechnet mit `u64` und
//!    **umlaufender** Arithmetik (Wrapping). Ein Ueberlauf, den die Referenz
//!    ablehnt, laeuft in der VM weiter.
//! 2. **Vorzeichen.** `Lt` vergleicht in atc-core vorzeichenbehaftet, in atc-vm
//!    ohne Vorzeichen. Negative Werte koennen als Konstante gar nicht erst
//!    entstehen (siehe `NegativeConstant`) und nur durch Ueberlauf, also nur
//!    zusammen mit Punkt 1.
//!
//! Beides ist eine offene Entscheidung am Vertrag der VM (eigener Befehl mit
//! Ueberlauf-Abbruch bzw. Vorzeichen-Vergleich), nicht eine Eigenschaft dieses
//! Backends. `DIVERGENZEN` haelt sie maschinenlesbar fest.

use atc_core::bytecode::Instruction;
use atc_core::lower::CompiledProgram;
use atc_vm::vm::Op;

/// Maschinenlesbare Liste der Semantik-Unterschiede, die offen bleiben.
pub const DIVERGENZEN: &[&str] = &[
    "ueberlauf: atc-core geprueft (Fehler), atc-vm umlaufend",
    "vorzeichen: Lt in atc-core vorzeichenbehaftet, in atc-vm ohne Vorzeichen",
];

/// Warum ein Programm nicht in ein ATCB-Artefakt uebersetzt wird.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendError {
    /// `call` braucht eine Aufrufkonvention, die die VM nicht hat.
    UnsupportedCall { function: u16, argc: u16, pc: usize },
    /// Negative Konstante: der VM-Stapel ist `u64`.
    NegativeConstant { value: i64, pc: usize },
    /// Instruktion ohne exakte Entsprechung.
    UnsupportedInstruction { name: &'static str, pc: usize },
    /// Sprungziel liegt ausserhalb des Programms.
    JumpTargetOutOfRange { pc: usize, target: i64 },
    /// Programm enthaelt keine Instruktion.
    EmptyProgram,
    /// Programm ist groesser als die VM-Grenze.
    TooLarge { size: usize, limit: usize },
}

impl BackendError {
    pub fn message(&self) -> String {
        match self {
            BackendError::UnsupportedCall { function, argc, pc } => format!(
                "call auf Funktion {function} mit {argc} Argumenten an PC {pc}: kein Aufruf in der ATCB-VM"
            ),
            BackendError::NegativeConstant { value, pc } => format!(
                "Konstante {value} an PC {pc}: der VM-Stapel ist u64, negative Werte sind nicht darstellbar"
            ),
            BackendError::UnsupportedInstruction { name, pc } => format!(
                "Instruktion {name} an PC {pc} hat keine exakte Entsprechung in der ATCB-VM"
            ),
            BackendError::JumpTargetOutOfRange { pc, target } => format!(
                "Sprungziel {target} an PC {pc} liegt ausserhalb des Programms"
            ),
            BackendError::EmptyProgram => "Programm enthaelt keine Instruktion".to_string(),
            BackendError::TooLarge { size, limit } => format!(
                "Programm mit {size} Operationen ueberschreitet die VM-Grenze von {limit}"
            ),
        }
    }
}

/// Ergebnis der Uebersetzung: die Operationen plus die Zuordnung
/// ATCLang-PC -> ATCB-Index (fuer Nachweise und Diagnose).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Translation {
    pub ops: Vec<Op>,
    /// `pc_map[i]` ist der ATCB-Index, an dem ATCLang-Instruktion `i` beginnt.
    pub pc_map: Vec<usize>,
    /// Laenge des ATCLang-Programms in Instruktionen.
    pub source_len: usize,
}

/// Uebersetzt das Einstiegsprogramm in ATCB-Operationen.
pub fn translate(program: &CompiledProgram) -> Result<Translation, BackendError> {
    // Nur die Einstiegsfunktion wird uebersetzt. Weitere Funktionen sind
    // unerreichbar, weil `Call` unten abgelehnt wird: es gibt keinen Weg,
    // sie anzuspringen. Der Zwischencode des Lowerings legt zusaetzlich eine
    // implizite Funktion `__main__` an, die bei einer `main`-Funktion ohne
    // Top-Level-Statements leer bleibt.
    let function = &program.functions[program.entry as usize];
    let instructions = &function.bytecode.instructions;
    if instructions.is_empty() {
        return Err(BackendError::EmptyProgram);
    }

    let mut ops: Vec<Op> = Vec::with_capacity(instructions.len() * 2);
    let mut pc_map: Vec<usize> = Vec::with_capacity(instructions.len());
    // Sprungstellen: (ATCB-Index der Operation, ATCLang-Ziel-PC, PC der Quelle)
    let mut jumps: Vec<(usize, usize, usize)> = Vec::new();

    for (pc, instruction) in instructions.iter().enumerate() {
        pc_map.push(ops.len());
        match instruction {
            Instruction::ConstI64(value) => {
                if *value < 0 {
                    return Err(BackendError::NegativeConstant { value: *value, pc });
                }
                ops.push(Op::Push(*value as u64));
            }
            Instruction::LoadLocal(slot) => ops.push(Op::Load(*slot as usize)),
            Instruction::StoreLocal(slot) => ops.push(Op::Store(*slot as usize)),
            Instruction::Add => ops.push(Op::Add),
            Instruction::Sub => ops.push(Op::Sub),
            Instruction::Mul => ops.push(Op::Mul),
            Instruction::Div => ops.push(Op::Div),
            Instruction::Eq => ops.push(Op::Eq),
            Instruction::Lt => ops.push(Op::Lt),
            // a != b  ==  1 - (a == b)
            Instruction::Ne => {
                ops.push(Op::Eq);
                ops.push(Op::Push(1));
                ops.push(Op::Swap);
                ops.push(Op::Sub);
            }
            // a > b  ==  b < a
            Instruction::Gt => {
                ops.push(Op::Swap);
                ops.push(Op::Lt);
            }
            // a <= b ==  1 - (b < a)
            Instruction::Le => {
                ops.push(Op::Swap);
                ops.push(Op::Lt);
                ops.push(Op::Push(1));
                ops.push(Op::Swap);
                ops.push(Op::Sub);
            }
            // a >= b ==  1 - (a < b)
            Instruction::Ge => {
                ops.push(Op::Lt);
                ops.push(Op::Push(1));
                ops.push(Op::Swap);
                ops.push(Op::Sub);
            }
            Instruction::Neg => {
                return Err(BackendError::UnsupportedInstruction { name: "Neg", pc })
            }
            Instruction::Pop => {
                return Err(BackendError::UnsupportedInstruction { name: "Pop", pc })
            }
            Instruction::Call { function, argc } => {
                return Err(BackendError::UnsupportedCall {
                    function: *function,
                    argc: *argc,
                    pc,
                })
            }
            Instruction::Return => ops.push(Op::Halt),
            Instruction::Jump(distance) => {
                let target = jump_target(pc, *distance)?;
                jumps.push((ops.len(), target, pc));
                ops.push(Op::Jump(target));
            }
            Instruction::JumpIfFalse(distance) => {
                let target = jump_target(pc, *distance)?;
                jumps.push((ops.len(), target, pc));
                ops.push(Op::JumpIfZero(target));
            }
        }
    }

    // Abschluss-Halt: ein Sprung an das Programmende bedeutet "fertig" und
    // braucht in der VM ein Ziel, das tatsaechlich existiert.
    let end_halt = ops.len();
    ops.push(Op::Halt);

    for (index, target, pc) in jumps {
        let absolute = if target == instructions.len() {
            end_halt
        } else {
            *pc_map
                .get(target)
                .ok_or(BackendError::JumpTargetOutOfRange {
                    pc,
                    target: target as i64,
                })?
        };
        match &mut ops[index] {
            Op::Jump(slot) | Op::JumpIfZero(slot) | Op::JumpIfNotZero(slot) => *slot = absolute,
            _ => unreachable!("Sprungstelle zeigt nicht auf einen Sprung"),
        }
    }

    if ops.len() > atc_vm::vm::MAX_PROGRAM_OPS {
        return Err(BackendError::TooLarge {
            size: ops.len(),
            limit: atc_vm::vm::MAX_PROGRAM_OPS,
        });
    }

    Ok(Translation {
        ops,
        pc_map,
        source_len: instructions.len(),
    })
}

/// ATCLang-Spruenge sind PC-relativ, Bezugsbasis ist die Folgeinstruktion.
fn jump_target(pc: usize, distance: i16) -> Result<usize, BackendError> {
    // Nur Untergrenze hier: ein Ziel hinter dem Programmende wird unten beim
    // Aufloesen ueber die PC-Zuordnung abgelehnt, nicht geraten.
    let target = pc as i64 + 1 + distance as i64;
    if target < 0 {
        return Err(BackendError::JumpTargetOutOfRange { pc, target });
    }
    Ok(target as usize)
}

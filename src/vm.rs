// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Stack-Maschine mit Stack-Underflow-Schutz und Sprung-Validierung.

#[derive(Debug, Clone, PartialEq, Eq)]
// Jump/JumpIfNotZero: Teil der Bytecode-ISA; konstruiert vom kuenftigen
// Bytecode-Loader (vollstaendiger .atc-Compile-Pfad). Der .ops-Runner nutzt
// bewusst nur das EXEC-GATE-Subset (fail-closed, siehe ops.rs-Tests).
#[allow(dead_code)]
pub enum Op {
    Push(u64),
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Swap,
    Jump(usize),
    JumpIfNotZero(usize),
    Eq,
    Lt,
    Load(usize),
    Store(usize),
    Caller,
    JumpIfZero(usize),
    Halt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    StackUnderflow,
    InvalidJump(usize),
    DivisionByZero,
}

pub struct Vm {
    program: Vec<Op>,
    stack: Vec<u64>,
    // Contract-Execution-Kontext (Inkrement 1, atc-contracts#5):
    // caller wird vom Host (Node) gesetzt — das Programm kann ihn nur
    // lesen (Op::Caller), nie schreiben. Storage = persistente Slots.
    caller: u64,
    storage: Vec<u64>,
}

impl Vm {
    // Oeffentliche Lib-API (Vm::new/caller): im Runner-Bin-Target ungenutzt,
    // in der Lib (Contract-Hosts, Tests) teils nur aus Tests heraus — daher
    // explizit freigegeben statt totem Code zu verdächtigen.
    #[allow(dead_code)]
    pub fn new(program: Vec<Op>) -> Self {
        Vm { program, stack: Vec::new(), caller: 0, storage: Vec::new() }
    }

    /// Contract-Kontext: Caller-Identitaet und vorbelegte Storage-Slots.
    pub fn with_context(program: Vec<Op>, caller: u64, storage: Vec<u64>) -> Self {
        Vm { program, stack: Vec::new(), caller, storage }
    }

    #[allow(dead_code)]
    pub fn caller(&self) -> u64 {
        self.caller
    }

    /// Finaler Storage-Stand (Evidenz nach der Ausfuehrung).
    pub fn state(&self) -> &[u64] {
        &self.storage
    }

    fn binop(&mut self, f: impl Fn(u64, u64) -> u64) -> Result<(), VmError> {
        let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
        self.stack.push(f(a, b));
        Ok(())
    }

    fn valid_jump(&self, t: usize) -> Result<usize, VmError> {
        if t < self.program.len() { Ok(t) } else { Err(VmError::InvalidJump(t)) }
    }

    pub fn run(&mut self) -> Result<Vec<u64>, VmError> {
        let mut pc: usize = 0;
        while pc < self.program.len() {
            match self.program[pc].clone() {
                Op::Push(v) => self.stack.push(v),
                Op::Add => self.binop(|a, b| a.wrapping_add(b))?,
                Op::Sub => self.binop(|a, b| a.wrapping_sub(b))?,
                Op::Mul => self.binop(|a, b| a.wrapping_mul(b))?,
                Op::Div => {
                    // Ganzzahlige u64-Division, fail-closed gegen Div/0:
                    // checked_div statt stillschweigender Panic (Owner-Regel:
                    // explizite Fehlerbehandlung im konsens-kritischen Pfad).
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let q = a.checked_div(b).ok_or(VmError::DivisionByZero)?;
                    self.stack.push(q);
                }
                Op::Eq => self.binop(|a, b| (a == b) as u64)?,
                Op::Lt => self.binop(|a, b| (a < b) as u64)?,
                Op::Dup => {
                    let v = *self.stack.last().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(v);
                }
                Op::Swap => {
                    let n = self.stack.len();
                    if n < 2 {
                        return Err(VmError::StackUnderflow);
                    }
                    self.stack.swap(n - 1, n - 2);
                }
                Op::Load(slot) => {
                    // Nie geschriebener Slot = 0 (Standard-Initialisierung)
                    let v = self.storage.get(slot).copied().unwrap_or(0);
                    self.stack.push(v);
                }
                Op::Store(slot) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if slot >= self.storage.len() {
                        self.storage.resize(slot + 1, 0);
                    }
                    self.storage[slot] = v;
                }
                Op::Caller => self.stack.push(self.caller),
                Op::JumpIfZero(t) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if v == 0 {
                        pc = self.valid_jump(t)?;
                        continue;
                    }
                }
                Op::Jump(t) => {
                    pc = self.valid_jump(t)?;
                    continue;
                }
                Op::JumpIfNotZero(t) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if v != 0 {
                        pc = self.valid_jump(t)?;
                        continue;
                    }
                }
                Op::Halt => break,
            }
            pc += 1;
        }
        Ok(std::mem::take(&mut self.stack))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetik() {
        // (2 + 3) * 4 = 20
        let mut vm = Vm::new(vec![Op::Push(2), Op::Push(3), Op::Add, Op::Push(4), Op::Mul, Op::Halt]);
        assert_eq!(vm.run(), Ok(vec![20]));
    }

    #[test]
    fn loop_countdown() {
        // Terminierender Countdown 5->0:
        // 0 Push(5) | 1 Dup | 2 JumpIfNotZero(4) | 3 Halt | 4 Push(1) | 5 Sub | 6 Jump(1)
        // Zaehler bleibt auf dem Stack; Dup+JumpIfNotZero testen, Push(1)+Sub dekrementieren.
        // Vorher (SCR-0089-RCA): Loop ab Index 1 mit Dup/Push/Sub/JumpIfNotZero(1) war
        // ENDLOS — der Zaehler persistierte nie (jede Iteration: 5-1=4, Stack unten blieb 5).
        let mut vm = Vm::new(vec![
            Op::Push(5),
            Op::Dup,
            Op::JumpIfNotZero(4),
            Op::Halt,
            Op::Push(1),
            Op::Sub,
            Op::Jump(1),
        ]);
        assert_eq!(vm.run(), Ok(vec![0]));
    }

    #[test]
    fn underflow_und_invalid_jump() {
        let mut vm = Vm::new(vec![Op::Add]);
        assert_eq!(vm.run(), Err(VmError::StackUnderflow));
        let mut vm = Vm::new(vec![Op::Push(1), Op::Jump(99)]);
        assert_eq!(vm.run(), Err(VmError::InvalidJump(99)));
    }

    #[test]
    fn swap_und_eq() {
        let mut vm = Vm::new(vec![Op::Push(1), Op::Push(2), Op::Swap, Op::Eq, Op::Halt]);
        assert_eq!(vm.run(), Ok(vec![0]));
    }
}

// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Stack-Maschine with stack/jump safety. State-transition entrypoint is gated by ATC-STD-600.

use crate::context::{execution_gate, ChainContext, ContextError};

/// Maximum number of opcodes accepted by the verifier.
pub const MAX_PROGRAM_OPS: usize = 65_536;
/// Maximum number of stack values retained during execution.
pub const MAX_STACK_ITEMS: usize = 4_096;
/// Maximum number of storage slots retained by a VM state.
pub const MAX_STORAGE_SLOTS: usize = 65_536;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    // ATCB-v2 (Kandidat): Pruef- und Ruecklaufbefehle, nur Generation >= 2.
    Assert,
    Revert(u64),
    RevertIf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    StackUnderflow,
    StackLimitExceeded,
    StorageLimitExceeded { requested: usize, limit: usize },
    ProgramTooLarge { size: usize, limit: usize },
    InvalidJump(usize),
    DivisionByZero,
    OutOfGas { required: u64, remaining: u64 },
    InvalidProgram(&'static str),
    Context(ContextError),
    AssertFailed,
    Reverted(u64),
    InvalidRevertCode(u64),
    V2OpInV1,
    UnreachableCode(usize),
}

pub struct Vm {
    program: Vec<Op>,
    stack: Vec<u64>,
    caller: u64,
    storage: Vec<u64>,
    /// Bytecode-Generation: 1 = ATCB-v1, 2 = ATCB-v2 (Kandidat).
    generation: u8,
}

/// Erlaubter Ruecklauf-Code: 0 = allgemein, 0x0100..=0x01FF = reserviert.
pub fn gueltiger_ruecklauf_code(code: u64) -> bool {
    code == 0 || (0x0100..=0x01FF).contains(&code)
}

/// Kennung der normativen Gastabelle (ATC-BC-002, Abschnitt 18). Die Kennung
/// ist Teil des Protokollzustands, nicht des Artefakts (BC2-011).
pub const GAS_TABLE_ID: &str = "GAS_TABLE_V1";

/// Normative Kosten der Fassung `GAS_TABLE_V1`, alle 19 Befehlscodes.
///
/// `gas_cost` ist die ausfuehrende Quelle; diese Konstante ist der benannte,
/// unveraenderliche Spiegel fuer Nachweis und Dokumentation. Beide werden durch
/// Tests deckungsgleich gehalten, in beide Richtungen: fehlt ein Befehl hier
/// oder weicht ein Wert ab, wird die Suite rot. Eine Kostenänderung erfordert
/// eine neue Kennung (GAS_TABLE_V2), nie eine stille Aenderung dieser Liste.
pub const GAS_TABLE_V1: &[(&str, u64)] = &[
    ("Push", 1),
    ("Add", 2),
    ("Sub", 2),
    ("Mul", 2),
    ("Div", 2),
    ("Dup", 1),
    ("Swap", 1),
    ("Eq", 2),
    ("Lt", 2),
    ("Load", 3),
    ("Store", 3),
    ("Caller", 1),
    ("Jump", 2),
    ("JumpIfZero", 2),
    ("JumpIfNotZero", 2),
    ("Halt", 0),
    ("Assert", 2),
    ("Revert", 1),
    ("RevertIf", 3),
];

/// Kosten eines Befehls nach `GAS_TABLE_V1`. Erschoepfend: jeder Befehl ohne
/// Eintrag bricht den Build.
fn gas_cost(op: &Op) -> u64 {
    match op {
        Op::Push(_) | Op::Dup | Op::Swap | Op::Caller => 1,
        Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Eq | Op::Lt => 2,
        Op::Load(_) | Op::Store(_) => 3,
        Op::Jump(_) | Op::JumpIfZero(_) | Op::JumpIfNotZero(_) => 2,
        Op::Halt => 0,
        Op::Assert => 2,
        Op::Revert(_) => 1,
        Op::RevertIf => 3,
    }
}

impl Vm {
    pub fn new(program: Vec<Op>) -> Self {
        Vm {
            program,
            stack: Vec::new(),
            caller: 0,
            storage: Vec::new(),
            generation: 1,
        }
    }

    pub fn with_context(program: Vec<Op>, caller: u64, storage: Vec<u64>) -> Self {
        Vm {
            program,
            stack: Vec::new(),
            caller,
            storage,
            generation: 1,
        }
    }

    pub fn caller(&self) -> u64 {
        self.caller
    }

    /// Baut eine VM aus einem kanonischen Artefakt. Die Generation kommt
    /// ausschliesslich aus dem Artefakt, nie aus einem Aufruferparameter.
    pub fn from_artifact(artifact: &crate::artifact::Artifact) -> Self {
        Vm {
            program: artifact.program().to_vec(),
            stack: Vec::new(),
            caller: 0,
            storage: Vec::new(),
            generation: artifact.generation(),
        }
    }

    pub fn generation(&self) -> u8 {
        self.generation
    }
    pub fn state(&self) -> &[u64] {
        &self.storage
    }

    /// Normative state-transition entrypoint. Identity, Genesis, protocol and VM
    /// compatibility MUST pass before the interpreter is allowed to mutate state.
    pub fn execute_state_transition(
        &mut self,
        context: &ChainContext,
        computed_genesis_id: &str,
        expected_protocol: &str,
        expected_vm: &str,
    ) -> Result<Vec<u64>, VmError> {
        // Keep the legacy entrypoint fail-closed: callers must explicitly provide
        // the protocol gas budget through execute_state_transition_with_gas.
        self.execute_state_transition_with_gas(
            context,
            computed_genesis_id,
            expected_protocol,
            expected_vm,
            0,
        )
    }

    /// Canonical state-transition entrypoint with an explicit consensus gas budget.
    /// The chain supplies the normative gas limit; the VM never invents one.
    pub fn execute_state_transition_with_gas(
        &mut self,
        context: &ChainContext,
        computed_genesis_id: &str,
        expected_protocol: &str,
        expected_vm: &str,
        gas_limit: u64,
    ) -> Result<Vec<u64>, VmError> {
        execution_gate(context, computed_genesis_id, expected_protocol, expected_vm)
            .map_err(VmError::Context)?;
        self.verify()?;
        self.run_with_gas(gas_limit)
    }

    /// Structural bytecode verification. Execution is rejected before any state
    /// mutation when a control-flow target is outside the program.
    pub fn verify(&self) -> Result<(), VmError> {
        if self.program.is_empty() {
            return Err(VmError::InvalidProgram("empty program"));
        }
        if self.program.len() > MAX_PROGRAM_OPS {
            return Err(VmError::ProgramTooLarge {
                size: self.program.len(),
                limit: MAX_PROGRAM_OPS,
            });
        }
        if self.storage.len() > MAX_STORAGE_SLOTS {
            return Err(VmError::StorageLimitExceeded {
                requested: self.storage.len(),
                limit: MAX_STORAGE_SLOTS,
            });
        }
        if self.stack.len() > MAX_STACK_ITEMS {
            return Err(VmError::StackLimitExceeded);
        }
        // ATCB-v2: Pruef- und Ruecklaufbefehle nur in Generation 2.
        let v2_ops = self
            .program
            .iter()
            .any(|op| matches!(op, Op::Assert | Op::Revert(_) | Op::RevertIf));
        if v2_ops && self.generation < 2 {
            return Err(VmError::V2OpInV1);
        }
        // Fehlercodes ausserhalb des reservierten Raums sind unzulaessig.
        for op in &self.program {
            if let Op::Revert(code) = op {
                if !gueltiger_ruecklauf_code(*code) {
                    return Err(VmError::InvalidRevertCode(*code));
                }
            }
        }
        // Kein erreichbarer Code nach einem Abbruch (nur v2, v1 unveraendert).
        if self.generation >= 2 {
            let mut targets = std::collections::BTreeSet::new();
            for op in &self.program {
                match op {
                    Op::Jump(t) | Op::JumpIfZero(t) | Op::JumpIfNotZero(t) => {
                        targets.insert(*t);
                    }
                    _ => {}
                }
            }
            let mut terminated = false;
            for (idx, op) in self.program.iter().enumerate() {
                if terminated && !targets.contains(&idx) {
                    return Err(VmError::UnreachableCode(idx));
                }
                if matches!(op, Op::Halt | Op::Revert(_)) {
                    terminated = true;
                }
            }
        }
        // Speicherplaetze muessen im Programm selbst schon zulaessig sein, sonst
        // ist ein Artefakt gueltig und trotzdem nie ausfuehrbar.
        let mut max_slot: usize = 0;
        for op in &self.program {
            if let Op::Load(slot) | Op::Store(slot) = op {
                let required = slot.saturating_add(1);
                if required > max_slot {
                    max_slot = required;
                }
            }
        }
        if max_slot > MAX_STORAGE_SLOTS {
            return Err(VmError::StorageLimitExceeded {
                requested: max_slot,
                limit: MAX_STORAGE_SLOTS,
            });
        }
        for op in &self.program {
            match op {
                Op::Jump(t) | Op::JumpIfNotZero(t) | Op::JumpIfZero(t)
                    if *t >= self.program.len() =>
                {
                    return Err(VmError::InvalidProgram("jump target out of range"));
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Ausfuehrung mit explizitem Gasbudget. Verifiziert, begrenzt, transaktional.
    pub fn run_with_gas(&mut self, gas_remaining: u64) -> Result<Vec<u64>, VmError> {
        self.execute_transactional(Some(gas_remaining))
    }

    /// Ausfuehrung ohne Gasbudget. Verifiziert, begrenzt und transaktional wie
    /// `run_with_gas`; es fehlt ausschliesslich die Gaskostenrechnung.
    /// Es gibt keinen ungeprueften oeffentlichen Ausfuehrungspfad.
    pub fn run(&mut self) -> Result<Vec<u64>, VmError> {
        self.execute_transactional(None)
    }

    /// Einziger Ausfuehrungseinstieg. Reihenfolge ist bindend:
    /// verifizieren, Snapshot nehmen, ausfuehren, bei Fehler zuruecknehmen.
    fn execute_transactional(&mut self, gas_limit: Option<u64>) -> Result<Vec<u64>, VmError> {
        self.verify()?;
        let original_stack = self.stack.clone();
        let original_storage = self.storage.clone();
        match self.run_loop(gas_limit) {
            Ok(stack) => Ok(stack),
            Err(error) => {
                // Keine teilweise bestaetigte Zustandsaenderung nach aussen.
                self.stack = original_stack;
                self.storage = original_storage;
                Err(error)
            }
        }
    }

    /// Die eine Interpreter-Schleife. Grenzen gelten immer; Gas nur mit Budget.
    fn run_loop(&mut self, gas_limit: Option<u64>) -> Result<Vec<u64>, VmError> {
        let mut budget = gas_limit;
        let mut pc = 0usize;
        while pc < self.program.len() {
            let cost = gas_cost(&self.program[pc]);
            if let Some(remaining) = budget.as_mut() {
                if *remaining < cost {
                    return Err(VmError::OutOfGas {
                        required: cost,
                        remaining: *remaining,
                    });
                }
                *remaining -= cost;
            }
            match self.program[pc].clone() {
                Op::Push(v) => self.push_stack(v)?,
                Op::Add => self.binop(|a, b| a.wrapping_add(b))?,
                Op::Sub => self.binop(|a, b| a.wrapping_sub(b))?,
                Op::Mul => self.binop(|a, b| a.wrapping_mul(b))?,
                Op::Div => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack
                        .push(a.checked_div(b).ok_or(VmError::DivisionByZero)?);
                }
                Op::Eq => self.binop(|a, b| (a == b) as u64)?,
                Op::Lt => self.binop(|a, b| (a < b) as u64)?,
                Op::Dup => {
                    let v = *self.stack.last().ok_or(VmError::StackUnderflow)?;
                    self.push_stack(v)?;
                }
                Op::Swap => {
                    let n = self.stack.len();
                    if n < 2 {
                        return Err(VmError::StackUnderflow);
                    }
                    self.stack.swap(n - 1, n - 2);
                }
                Op::Load(slot) => {
                    self.push_stack(self.storage.get(slot).copied().unwrap_or(0))?;
                }
                Op::Store(slot) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let required = slot.checked_add(1).ok_or(VmError::StorageLimitExceeded {
                        requested: usize::MAX,
                        limit: MAX_STORAGE_SLOTS,
                    })?;
                    if required > MAX_STORAGE_SLOTS {
                        return Err(VmError::StorageLimitExceeded {
                            requested: required,
                            limit: MAX_STORAGE_SLOTS,
                        });
                    }
                    if required > self.storage.len() {
                        self.storage.resize(required, 0);
                    }
                    self.storage[slot] = v;
                }
                Op::Caller => self.push_stack(self.caller)?,
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
                Op::Assert => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if v == 0 {
                        return Err(VmError::AssertFailed);
                    }
                }
                Op::Revert(code) => {
                    if !gueltiger_ruecklauf_code(code) {
                        return Err(VmError::InvalidRevertCode(code));
                    }
                    return Err(VmError::Reverted(code));
                }
                Op::RevertIf => {
                    // Wahrheitstabelle (ATC-BC-002 15): Bedingung != 0 -> Ruecklauf,
                    // Code ist der Grund (0 = allgemeiner Ruecklauf).
                    let code = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let bedingung = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if !gueltiger_ruecklauf_code(code) {
                        return Err(VmError::InvalidRevertCode(code));
                    }
                    if bedingung != 0 {
                        return Err(VmError::Reverted(code));
                    }
                }
            }
            pc += 1;
        }
        Ok(std::mem::take(&mut self.stack))
    }

    fn push_stack(&mut self, value: u64) -> Result<(), VmError> {
        if self.stack.len() >= MAX_STACK_ITEMS {
            return Err(VmError::StackLimitExceeded);
        }
        self.stack.push(value);
        Ok(())
    }

    fn binop(&mut self, f: impl Fn(u64, u64) -> u64) -> Result<(), VmError> {
        let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
        let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
        self.stack.push(f(a, b));
        Ok(())
    }

    fn valid_jump(&self, t: usize) -> Result<usize, VmError> {
        if t < self.program.len() {
            Ok(t)
        } else {
            Err(VmError::InvalidJump(t))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> ChainContext {
        ChainContext {
            chain_id: "atc".into(),
            network_id: "devnet".into(),
            genesis_id: "a".repeat(64),
            protocol_version: "1.0.0".into(),
            vm_version: "1.0.0".into(),
        }
    }

    #[test]
    fn arithmetik() {
        let mut vm = Vm::new(vec![
            Op::Push(2),
            Op::Push(3),
            Op::Add,
            Op::Push(4),
            Op::Mul,
            Op::Halt,
        ]);
        assert_eq!(vm.run(), Ok(vec![20]));
    }

    #[test]
    fn verifier_rejects_invalid_jump_before_state_mutation() {
        let mut vm = Vm::with_context(vec![Op::Push(7), Op::Jump(99)], 1, vec![0]);
        let result = vm.execute_state_transition_with_gas(
            &context(),
            &"a".repeat(64),
            "1.0.0",
            "1.0.0",
            100,
        );
        assert_eq!(
            result,
            Err(VmError::InvalidProgram("jump target out of range"))
        );
        assert_eq!(vm.state(), &[0]);
    }

    #[test]
    fn gas_exhaustion_is_fail_closed() {
        let mut vm = Vm::with_context(vec![Op::Push(7), Op::Store(0), Op::Halt], 1, vec![0]);
        let result =
            vm.execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 2);
        assert!(matches!(result, Err(VmError::OutOfGas { .. })));
        assert_eq!(vm.state(), &[0]);
    }

    #[test]
    fn gas_exhaustion_after_mutation_rolls_back_state() {
        let mut vm = Vm::with_context(
            vec![Op::Push(7), Op::Store(0), Op::Push(8), Op::Halt],
            1,
            vec![0],
        );
        let result =
            vm.execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 4);
        assert!(matches!(result, Err(VmError::OutOfGas { .. })));
        assert_eq!(vm.state(), &[0]);
    }

    #[test]
    fn storage_growth_is_bounded() {
        let mut vm = Vm::with_context(
            vec![Op::Push(7), Op::Store(MAX_STORAGE_SLOTS), Op::Halt],
            1,
            vec![],
        );
        let result =
            vm.execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 10);
        assert!(matches!(result, Err(VmError::StorageLimitExceeded { .. })));
        assert!(vm.state().is_empty());
    }

    #[test]
    fn stack_growth_is_bounded() {
        let mut program = Vec::with_capacity(MAX_STACK_ITEMS + 1);
        program.extend(std::iter::repeat_n(Op::Push(1), MAX_STACK_ITEMS + 1));
        program.push(Op::Halt);
        let mut vm = Vm::new(program);
        let result = vm.execute_state_transition_with_gas(
            &context(),
            &"a".repeat(64),
            "1.0.0",
            "1.0.0",
            (MAX_STACK_ITEMS as u64) + 10,
        );
        assert_eq!(result, Err(VmError::StackLimitExceeded));
    }

    #[test]
    fn program_size_is_bounded() {
        let mut program = Vec::with_capacity(MAX_PROGRAM_OPS + 1);
        program.extend(std::iter::repeat_n(Op::Halt, MAX_PROGRAM_OPS + 1));
        let vm = Vm::new(program);
        assert_eq!(
            vm.verify(),
            Err(VmError::ProgramTooLarge {
                size: MAX_PROGRAM_OPS + 1,
                limit: MAX_PROGRAM_OPS,
            })
        );
    }

    #[test]
    fn explicit_gas_budget_executes_deterministically() {
        let mut vm = Vm::with_context(vec![Op::Push(7), Op::Store(0), Op::Halt], 1, vec![0]);
        let result =
            vm.execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 4);
        assert_eq!(result, Ok(vec![]));
        assert_eq!(vm.state(), &[7]);
    }

    #[test]
    fn state_transition_requires_identity_gate() {
        let mut vm = Vm::with_context(vec![Op::Push(7), Op::Store(0), Op::Halt], 1, vec![]);
        assert!(vm
            .execute_state_transition_with_gas(&context(), &"b".repeat(64), "1.0.0", "1.0.0", 10)
            .is_err());
        assert!(
            vm.state().is_empty(),
            "invalid context darf keinen State mutieren"
        );
        assert!(vm
            .execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 10)
            .is_ok());
        assert_eq!(vm.state(), &[7]);
    }

    #[test]
    fn underflow_und_invalid_jump() {
        let mut vm = Vm::new(vec![Op::Add]);
        assert_eq!(vm.run(), Err(VmError::StackUnderflow));
        // Verhaltensaenderung durch P0-1 (VM-AUD-003-A): Ein ungueltiges
        // Sprungziel wird jetzt VOR der Ausfuehrung vom Verifier abgewiesen.
        // Gleiche Ablehnung, anderer Befund; gueltige v1-Programme unberuehrt.
        let mut vm = Vm::new(vec![Op::Push(1), Op::Jump(99)]);
        assert_eq!(
            vm.run(),
            Err(VmError::InvalidProgram("jump target out of range"))
        );
    }
}

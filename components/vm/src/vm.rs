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
    Push(u64), Add, Sub, Mul, Div, Dup, Swap,
    Jump(usize), JumpIfNotZero(usize), Eq, Lt,
    Load(usize), Store(usize), Caller, JumpIfZero(usize), Halt,
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
}

pub struct Vm {
    program: Vec<Op>,
    stack: Vec<u64>,
    caller: u64,
    storage: Vec<u64>,
}


fn gas_cost(op: &Op) -> u64 {
    match op {
        Op::Push(_) | Op::Dup | Op::Swap | Op::Caller => 1,
        Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Eq | Op::Lt => 2,
        Op::Load(_) | Op::Store(_) => 3,
        Op::Jump(_) | Op::JumpIfZero(_) | Op::JumpIfNotZero(_) => 2,
        Op::Halt => 0,
    }
}

impl Vm {
    pub fn new(program: Vec<Op>) -> Self {
        Vm { program, stack: Vec::new(), caller: 0, storage: Vec::new() }
    }

    pub fn with_context(program: Vec<Op>, caller: u64, storage: Vec<u64>) -> Self {
        Vm { program, stack: Vec::new(), caller, storage }
    }

    pub fn caller(&self) -> u64 { self.caller }
    pub fn state(&self) -> &[u64] { &self.storage }

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

    pub fn run_with_gas(&mut self, mut gas_remaining: u64) -> Result<Vec<u64>, VmError> {
        self.verify()?;

        // State transitions are transactional: no partial stack/storage mutation
        // may escape when execution fails (e.g. OutOfGas after a successful Store).
        let original_stack = self.stack.clone();
        let original_storage = self.storage.clone();

        let result = self.run_with_gas_inner(&mut gas_remaining);
        match result {
            Ok(stack) => Ok(stack),
            Err(error) => {
                self.stack = original_stack;
                self.storage = original_storage;
                Err(error)
            }
        }
    }

    fn run_with_gas_inner(&mut self, gas_remaining: &mut u64) -> Result<Vec<u64>, VmError> {
        let mut pc = 0usize;
        while pc < self.program.len() {
            let cost = gas_cost(&self.program[pc]);
            if *gas_remaining < cost {
                return Err(VmError::OutOfGas { required: cost, remaining: *gas_remaining });
            }
            *gas_remaining -= cost;
            match self.program[pc].clone() {
                Op::Push(v) => self.push_stack(v)?,
                Op::Add => self.binop(|a, b| a.wrapping_add(b))?,
                Op::Sub => self.binop(|a, b| a.wrapping_sub(b))?,
                Op::Mul => self.binop(|a, b| a.wrapping_mul(b))?,
                Op::Div => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(a.checked_div(b).ok_or(VmError::DivisionByZero)?);
                }
                Op::Eq => self.binop(|a, b| (a == b) as u64)?,
                Op::Lt => self.binop(|a, b| (a < b) as u64)?,
                Op::Dup => {
                    let v = *self.stack.last().ok_or(VmError::StackUnderflow)?;
                    self.push_stack(v)?;
                }
                Op::Swap => {
                    let n = self.stack.len();
                    if n < 2 { return Err(VmError::StackUnderflow); }
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
                    if v == 0 { pc = t; continue; }
                }
                Op::Jump(t) => { pc = t; continue; }
                Op::JumpIfNotZero(t) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if v != 0 { pc = t; continue; }
                }
                Op::Halt => break,
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

    /// Raw interpreter retained for local/unit use. Consensus state transitions
    /// MUST use execute_state_transition_with_gas.
    pub fn run(&mut self) -> Result<Vec<u64>, VmError> {
        let mut pc = 0usize;
        while pc < self.program.len() {
            match self.program[pc].clone() {
                Op::Push(v) => self.stack.push(v),
                Op::Add => self.binop(|a, b| a.wrapping_add(b))?,
                Op::Sub => self.binop(|a, b| a.wrapping_sub(b))?,
                Op::Mul => self.binop(|a, b| a.wrapping_mul(b))?,
                Op::Div => {
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(a.checked_div(b).ok_or(VmError::DivisionByZero)?);
                }
                Op::Eq => self.binop(|a, b| (a == b) as u64)?,
                Op::Lt => self.binop(|a, b| (a < b) as u64)?,
                Op::Dup => {
                    let v = *self.stack.last().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(v);
                }
                Op::Swap => {
                    let n = self.stack.len();
                    if n < 2 { return Err(VmError::StackUnderflow); }
                    self.stack.swap(n - 1, n - 2);
                }
                Op::Load(slot) => self.stack.push(self.storage.get(slot).copied().unwrap_or(0)),
                Op::Store(slot) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if slot >= self.storage.len() { self.storage.resize(slot + 1, 0); }
                    self.storage[slot] = v;
                }
                Op::Caller => self.stack.push(self.caller),
                Op::JumpIfZero(t) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if v == 0 { pc = self.valid_jump(t)?; continue; }
                }
                Op::Jump(t) => { pc = self.valid_jump(t)?; continue; }
                Op::JumpIfNotZero(t) => {
                    let v = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if v != 0 { pc = self.valid_jump(t)?; continue; }
                }
                Op::Halt => break,
            }
            pc += 1;
        }
        Ok(std::mem::take(&mut self.stack))
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> ChainContext {
        ChainContext { chain_id: "atc".into(), network_id: "devnet".into(), genesis_id: "a".repeat(64), protocol_version: "1.0.0".into(), vm_version: "1.0.0".into() }
    }

    #[test]
    fn arithmetik() {
        let mut vm = Vm::new(vec![Op::Push(2), Op::Push(3), Op::Add, Op::Push(4), Op::Mul, Op::Halt]);
        assert_eq!(vm.run(), Ok(vec![20]));
    }

    #[test]
    fn verifier_rejects_invalid_jump_before_state_mutation() {
        let mut vm = Vm::with_context(vec![Op::Push(7), Op::Jump(99)], 1, vec![0]);
        let result = vm.execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 100);
        assert_eq!(result, Err(VmError::InvalidProgram("jump target out of range")));
        assert_eq!(vm.state(), &[0]);
    }

    #[test]
    fn gas_exhaustion_is_fail_closed() {
        let mut vm = Vm::with_context(vec![Op::Push(7), Op::Store(0), Op::Halt], 1, vec![0]);
        let result = vm.execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 2);
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
        let result = vm.execute_state_transition_with_gas(
            &context(),
            &"a".repeat(64),
            "1.0.0",
            "1.0.0",
            4,
        );
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
        let result = vm.execute_state_transition_with_gas(
            &context(),
            &"a".repeat(64),
            "1.0.0",
            "1.0.0",
            10,
        );
        assert!(matches!(
            result,
            Err(VmError::StorageLimitExceeded { .. })
        ));
        assert!(vm.state().is_empty());
    }

    #[test]
    fn stack_growth_is_bounded() {
        let mut program = Vec::with_capacity(MAX_STACK_ITEMS + 1);
        program.extend(std::iter::repeat(Op::Push(1)).take(MAX_STACK_ITEMS + 1));
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
        program.extend(std::iter::repeat(Op::Halt).take(MAX_PROGRAM_OPS + 1));
        let mut vm = Vm::new(program);
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
        let result = vm.execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 4);
        assert_eq!(result, Ok(vec![]));
        assert_eq!(vm.state(), &[7]);
    }

    #[test]
    fn state_transition_requires_identity_gate() {
        let mut vm = Vm::with_context(vec![Op::Push(7), Op::Store(0), Op::Halt], 1, vec![]);
        assert!(vm.execute_state_transition_with_gas(&context(), &"b".repeat(64), "1.0.0", "1.0.0", 10).is_err());
        assert!(vm.state().is_empty(), "invalid context darf keinen State mutieren");
        assert!(vm.execute_state_transition_with_gas(&context(), &"a".repeat(64), "1.0.0", "1.0.0", 10).is_ok());
        assert_eq!(vm.state(), &[7]);
    }

    #[test]
    fn underflow_und_invalid_jump() {
        let mut vm = Vm::new(vec![Op::Add]);
        assert_eq!(vm.run(), Err(VmError::StackUnderflow));
        let mut vm = Vm::new(vec![Op::Push(1), Op::Jump(99)]);
        assert_eq!(vm.run(), Err(VmError::InvalidJump(99)));
    }
}

// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Deterministic ATC-VM stack machine.
//!
//! Gas is charged before every opcode effect. This is fail-closed: an instruction
//! that cannot be paid for is never executed. The schedule below is the recovered
//! existing ATC/ShivaCore baseline and remains subject to the normative ATC-VM-001
//! gas-registry freeze; changing it is a consensus-visible change.

use crate::context::{execution_gate, ChainContext, ContextError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Push(u64), Add, Sub, Mul, Div, Dup, Swap,
    Jump(usize), JumpIfNotZero(usize), Eq, Lt,
    Load(usize), Store(usize), Caller, JumpIfZero(usize), Halt,
}

impl Op {
    /// Existing ATC/ShivaCore baseline gas schedule.
    pub const fn gas_cost(self) -> u64 {
        match self {
            Op::Push(_) => 3,
            Op::Add | Op::Sub | Op::Mul => 5,
            Op::Div => 10,
            Op::Dup | Op::Swap | Op::Caller => 2,
            Op::Eq | Op::Lt => 3,
            Op::Jump(_) | Op::JumpIfNotZero(_) | Op::JumpIfZero(_) => 8,
            Op::Load(_) => 200,
            Op::Store(_) => 5000,
            Op::Halt => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    StackUnderflow,
    InvalidJump(usize),
    DivisionByZero,
    Context(ContextError),
    OutOfGas { required: u64, remaining: u64 },
    GasOverflow,
}

pub struct Vm {
    program: Vec<Op>,
    stack: Vec<u64>,
    caller: u64,
    storage: Vec<u64>,
    gas_limit: u64,
    gas_used: u64,
}

impl Vm {
    /// Constructs a VM with an effectively unlimited local budget.
    /// Consensus/state-transition callers MUST provide an explicit budget.
    pub fn new(program: Vec<Op>) -> Self {
        Self::with_gas(program, u64::MAX)
    }

    pub fn with_gas(program: Vec<Op>, gas_limit: u64) -> Self {
        Vm {
            program,
            stack: Vec::new(),
            caller: 0,
            storage: Vec::new(),
            gas_limit,
            gas_used: 0,
        }
    }

    pub fn with_context(program: Vec<Op>, caller: u64, storage: Vec<u64>) -> Self {
        Self::with_context_and_gas(program, caller, storage, u64::MAX)
    }

    pub fn with_context_and_gas(
        program: Vec<Op>,
        caller: u64,
        storage: Vec<u64>,
        gas_limit: u64,
    ) -> Self {
        Vm {
            program,
            stack: Vec::new(),
            caller,
            storage,
            gas_limit,
            gas_used: 0,
        }
    }

    pub fn caller(&self) -> u64 { self.caller }
    pub fn state(&self) -> &[u64] { &self.storage }
    pub fn gas_limit(&self) -> u64 { self.gas_limit }
    pub fn gas_used(&self) -> u64 { self.gas_used }
    pub fn gas_remaining(&self) -> u64 { self.gas_limit.saturating_sub(self.gas_used) }

    /// Normative state-transition entrypoint. Identity, Genesis, protocol and VM
    /// compatibility MUST pass before the interpreter is allowed to mutate state.
    pub fn execute_state_transition(
        &mut self,
        context: &ChainContext,
        computed_genesis_id: &str,
        expected_protocol: &str,
        expected_vm: &str,
    ) -> Result<Vec<u64>, VmError> {
        execution_gate(context, computed_genesis_id, expected_protocol, expected_vm)
            .map_err(VmError::Context)?;
        self.run()
    }

    /// Raw interpreter. Every opcode is metered and charged before its effect.
    pub fn run(&mut self) -> Result<Vec<u64>, VmError> {
        let mut pc = 0usize;
        while pc < self.program.len() {
            let op = self.program[pc];
            self.charge(op.gas_cost())?;

            match op {
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

    fn charge(&mut self, cost: u64) -> Result<(), VmError> {
        let remaining = self.gas_remaining();
        if cost > remaining {
            return Err(VmError::OutOfGas { required: cost, remaining });
        }
        self.gas_used = self.gas_used.checked_add(cost).ok_or(VmError::GasOverflow)?;
        Ok(())
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
        let mut vm = Vm::new(vec![Op::Push(2), Op::Push(3), Op::Add, Op::Push(4), Op::Mul, Op::Halt]);
        assert_eq!(vm.run(), Ok(vec![20]));
        assert_eq!(vm.gas_used(), 3 + 3 + 5 + 3 + 5);
    }

    #[test]
    fn gas_is_charged_before_instruction_effect() {
        let mut vm = Vm::with_gas(vec![Op::Push(7), Op::Store(0)], 3);
        assert_eq!(vm.run(), Err(VmError::OutOfGas { required: 5000, remaining: 0 }));
        assert!(vm.state().is_empty(), "Store darf bei Out-of-Gas nicht ausgeführt werden");
        assert_eq!(vm.gas_used(), 3);
    }

    #[test]
    fn exact_budget_succeeds() {
        let mut vm = Vm::with_gas(vec![Op::Push(7), Op::Store(0), Op::Halt], 5003);
        assert_eq!(vm.run(), Ok(Vec::<u64>::new()));
        assert_eq!(vm.state(), &[7]);
        assert_eq!(vm.gas_used(), 5003);
        assert_eq!(vm.gas_remaining(), 0);
    }

    #[test]
    fn state_transition_requires_identity_gate() {
        let mut vm = Vm::with_context_and_gas(
            vec![Op::Push(7), Op::Store(0), Op::Halt],
            1,
            vec![],
            5003,
        );
        assert!(vm.execute_state_transition(&context(), &"b".repeat(64), "1.0.0", "1.0.0").is_err());
        assert!(vm.state().is_empty(), "invalid context darf keinen State mutieren");
        assert!(vm.execute_state_transition(&context(), &"a".repeat(64), "1.0.0", "1.0.0").is_ok());
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

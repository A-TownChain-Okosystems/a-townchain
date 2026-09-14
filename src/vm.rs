// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Stack-Maschine with stack/jump safety. State-transition entrypoint is gated by ATC-STD-600.

use crate::context::{execution_gate, ChainContext, ContextError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Push(u64), Add, Sub, Mul, Div, Dup, Swap,
    Jump(usize), JumpIfNotZero(usize), Eq, Lt,
    Load(usize), Store(usize), Caller, JumpIfZero(usize), Halt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    StackUnderflow,
    InvalidJump(usize),
    DivisionByZero,
    Context(ContextError),
}

pub struct Vm {
    program: Vec<Op>,
    stack: Vec<u64>,
    caller: u64,
    storage: Vec<u64>,
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
        execution_gate(context, computed_genesis_id, expected_protocol, expected_vm)
            .map_err(VmError::Context)?;
        self.run()
    }

    /// Raw bytecode interpreter. Callers performing chain state transitions MUST
    /// use execute_state_transition instead of invoking this directly.
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
    fn state_transition_requires_identity_gate() {
        let mut vm = Vm::with_context(vec![Op::Push(7), Op::Store(0), Op::Halt], 1, vec![]);
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

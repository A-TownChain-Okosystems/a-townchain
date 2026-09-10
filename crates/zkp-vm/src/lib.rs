// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ZK-Befehlszaehler (MVP).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Constraint { a: u64, b: u64 },
    Hint(u64),
}

pub struct ZkVm {
    program: Vec<Instruction>,
    constraint_count: u64,
    hint_count: u64,
}

impl ZkVm {
    pub fn new(program: Vec<Instruction>) -> Self {
        ZkVm { program, constraint_count: 0, hint_count: 0 }
    }

    pub fn trace(&mut self) -> (u64, u64) {
        for ins in &self.program {
            match ins {
                Instruction::Constraint { .. } => self.constraint_count += 1,
                Instruction::Hint(_) => self.hint_count += 1,
            }
        }
        (self.constraint_count, self.hint_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zaehlung() {
        let mut vm = ZkVm::new(vec![
            Instruction::Constraint { a: 1, b: 2 },
            Instruction::Hint(5),
            Instruction::Constraint { a: 3, b: 4 },
        ]);
        assert_eq!(vm.trace(), (2, 1));
    }
}

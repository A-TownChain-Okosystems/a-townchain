//! Symmetric multiprocessing topology and CPU lifecycle contract.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuState {
    Offline,
    Starting,
    Online,
    Halted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuInfo {
    pub id: CpuId,
    pub state: CpuState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmpError {
    InvalidCpu,
    DuplicateCpu,
    InvalidTransition,
}

#[derive(Debug, Default)]
pub struct CpuTopology {
    cpus: Vec<CpuInfo>,
}

impl CpuTopology {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, id: CpuId) -> Result<(), SmpError> {
        if id.0 == u32::MAX {
            return Err(SmpError::InvalidCpu);
        }
        if self.cpus.iter().any(|c| c.id == id) {
            return Err(SmpError::DuplicateCpu);
        }
        self.cpus.push(CpuInfo {
            id,
            state: CpuState::Offline,
        });
        self.cpus.sort_by_key(|c| c.id.0);
        Ok(())
    }
    pub fn start(&mut self, id: CpuId) -> Result<(), SmpError> {
        let cpu = self
            .cpus
            .iter_mut()
            .find(|c| c.id == id)
            .ok_or(SmpError::InvalidCpu)?;
        if cpu.state != CpuState::Offline {
            return Err(SmpError::InvalidTransition);
        }
        cpu.state = CpuState::Starting;
        cpu.state = CpuState::Online;
        Ok(())
    }
    pub fn halt(&mut self, id: CpuId) -> Result<(), SmpError> {
        let cpu = self
            .cpus
            .iter_mut()
            .find(|c| c.id == id)
            .ok_or(SmpError::InvalidCpu)?;
        if cpu.state != CpuState::Online {
            return Err(SmpError::InvalidTransition);
        }
        cpu.state = CpuState::Halted;
        Ok(())
    }
    pub fn get(&self, id: CpuId) -> Option<CpuInfo> {
        self.cpus.iter().find(|c| c.id == id).copied()
    }
    pub fn cpus(&self) -> &[CpuInfo] {
        &self.cpus
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cpu_lifecycle_is_explicit() {
        let mut t = CpuTopology::new();
        t.register(CpuId(1)).unwrap();
        t.start(CpuId(1)).unwrap();
        assert_eq!(t.get(CpuId(1)).unwrap().state, CpuState::Online);
        t.halt(CpuId(1)).unwrap();
        assert_eq!(t.get(CpuId(1)).unwrap().state, CpuState::Halted);
    }
}

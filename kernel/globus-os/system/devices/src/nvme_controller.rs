//! NVMe MMIO register programming and controller initialization runtime.

pub const CAP: u64 = 0x00;
pub const VS: u64 = 0x08;
pub const CC: u64 = 0x14;
pub const CSTS: u64 = 0x1c;
pub const AQA: u64 = 0x24;
pub const ASQ: u64 = 0x28;
pub const ACQ: u64 = 0x30;
pub const CC_EN: u32 = 1;
pub const CSTS_RDY: u32 = 1;
pub const CSTS_CFS: u32 = 1 << 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeControllerConfig {
    pub admin_queue_depth: u16,
    pub io_queue_depth: u16,
    pub page_size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvmeControllerError {
    InvalidConfig,
    Timeout,
    ControllerFatal,
    QueueSetupFailed,
    IdentifyFailed,
    AlreadyEnabled,
    NotEnabled,
}

pub trait NvmeMmio {
    fn read32(&self, offset: u64) -> u32;
    fn read64(&self, offset: u64) -> u64;
    fn write32(&mut self, offset: u64, value: u32);
    fn write64(&mut self, offset: u64, value: u64);
}

pub fn validate_config(c: NvmeControllerConfig) -> Result<(), NvmeControllerError> {
    if !c.admin_queue_depth.is_power_of_two()
        || !(2..=4096).contains(&c.admin_queue_depth)
        || !c.io_queue_depth.is_power_of_two()
        || !(2..=4096).contains(&c.io_queue_depth)
        || !c.page_size.is_power_of_two()
        || !(4096..=65536).contains(&c.page_size)
    {
        return Err(NvmeControllerError::InvalidConfig);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerState {
    Reset,
    Enabling,
    Ready,
    Fatal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdminQueueAddresses {
    pub submission: u64,
    pub completion: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeController {
    state: ControllerState,
    config: NvmeControllerConfig,
    admin: Option<AdminQueueAddresses>,
}

impl NvmeController {
    pub fn new(config: NvmeControllerConfig) -> Result<Self, NvmeControllerError> {
        validate_config(config)?;
        Ok(Self {
            state: ControllerState::Reset,
            config,
            admin: None,
        })
    }
    pub fn state(&self) -> ControllerState {
        self.state
    }
    pub fn config(&self) -> NvmeControllerConfig {
        self.config
    }
    pub fn configure_admin_queues(
        &mut self,
        mmio: &mut impl NvmeMmio,
        addresses: AdminQueueAddresses,
    ) -> Result<(), NvmeControllerError> {
        if self.state == ControllerState::Fatal {
            return Err(NvmeControllerError::ControllerFatal);
        }
        if !addresses
            .submission
            .is_multiple_of(self.config.page_size as u64)
            || !addresses
                .completion
                .is_multiple_of(self.config.page_size as u64)
        {
            return Err(NvmeControllerError::QueueSetupFailed);
        }
        let d = self.config.admin_queue_depth - 1;
        mmio.write32(AQA, ((d as u32) << 16) | d as u32);
        mmio.write64(ASQ, addresses.submission);
        mmio.write64(ACQ, addresses.completion);
        self.admin = Some(addresses);
        Ok(())
    }
    pub fn enable(&mut self, mmio: &mut impl NvmeMmio) -> Result<(), NvmeControllerError> {
        if self.state == ControllerState::Ready || self.state == ControllerState::Enabling {
            return Err(NvmeControllerError::AlreadyEnabled);
        }
        if self.admin.is_none() {
            return Err(NvmeControllerError::QueueSetupFailed);
        }
        if mmio.read32(CSTS) & CSTS_CFS != 0 {
            self.state = ControllerState::Fatal;
            return Err(NvmeControllerError::ControllerFatal);
        }
        let cc = mmio.read32(CC) | CC_EN;
        mmio.write32(CC, cc);
        self.state = ControllerState::Enabling;
        Ok(())
    }
    pub fn poll_ready(&mut self, mmio: &impl NvmeMmio) -> Result<(), NvmeControllerError> {
        if self.state != ControllerState::Enabling {
            return Err(NvmeControllerError::NotEnabled);
        }
        let status = mmio.read32(CSTS);
        if status & CSTS_CFS != 0 {
            self.state = ControllerState::Fatal;
            return Err(NvmeControllerError::ControllerFatal);
        }
        if status & CSTS_RDY != 0 {
            self.state = ControllerState::Ready;
            Ok(())
        } else {
            Err(NvmeControllerError::Timeout)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Mock {
        regs: [u64; 64],
    }
    impl Default for Mock {
        fn default() -> Self {
            Self { regs: [0; 64] }
        }
    }
    impl NvmeMmio for Mock {
        fn read32(&self, o: u64) -> u32 {
            self.regs[(o / 8) as usize] as u32
        }
        fn read64(&self, o: u64) -> u64 {
            self.regs[(o / 8) as usize]
        }
        fn write32(&mut self, o: u64, v: u32) {
            self.regs[(o / 8) as usize] = v as u64
        }
        fn write64(&mut self, o: u64, v: u64) {
            self.regs[(o / 8) as usize] = v
        }
    }
    fn config() -> NvmeControllerConfig {
        NvmeControllerConfig {
            admin_queue_depth: 32,
            io_queue_depth: 256,
            page_size: 4096,
        }
    }
    #[test]
    fn setup_programs_admin_queues() {
        let mut m = Mock::default();
        let mut c = NvmeController::new(config()).unwrap();
        c.configure_admin_queues(
            &mut m,
            AdminQueueAddresses {
                submission: 0x10000,
                completion: 0x20000,
            },
        )
        .unwrap();
        assert_eq!(m.read64(ASQ), 0x10000);
        assert_eq!(m.read64(ACQ), 0x20000);
    }
    #[test]
    fn ready_transition_is_explicit() {
        let mut m = Mock::default();
        let mut c = NvmeController::new(config()).unwrap();
        c.configure_admin_queues(
            &mut m,
            AdminQueueAddresses {
                submission: 0x10000,
                completion: 0x20000,
            },
        )
        .unwrap();
        c.enable(&mut m).unwrap();
        assert_eq!(c.state(), ControllerState::Enabling);
        m.write32(CSTS, CSTS_RDY);
        c.poll_ready(&m).unwrap();
        assert_eq!(c.state(), ControllerState::Ready);
    }
}

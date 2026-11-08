//! NVMe controller contracts and deterministic queue runtime.

pub const NVME_ADMIN_QUEUE_DEPTH: u16 = 32;
pub const NVME_DEFAULT_IO_QUEUE_DEPTH: u16 = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeQueueId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeDmaBuffer {
    pub address: u64,
    pub length: u32,
}

impl NvmeDmaBuffer {
    pub fn valid(self) -> bool {
        self.length > 0 && self.address.checked_add(self.length as u64).is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeCommand {
    pub opcode: u8,
    pub namespace: u32,
    pub data: Option<NvmeDmaBuffer>,
    pub command_id: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeCompletion {
    pub command_id: u16,
    pub status: u16,
    pub result: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvmeError {
    InvalidQueueDepth,
    InvalidBuffer,
    ControllerNotReady,
    SubmissionFailed,
    QueueFull,
    CompletionUnavailable,
    DuplicateCommand,
}

pub const fn valid_queue_depth(depth: u16) -> bool {
    depth >= 2 && depth <= 4096 && depth.is_power_of_two()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NvmeQueue {
    id: NvmeQueueId,
    depth: u16,
    submissions: Vec<NvmeCommand>,
    completions: Vec<NvmeCompletion>,
}

impl NvmeQueue {
    pub fn new(id: NvmeQueueId, depth: u16) -> Result<Self, NvmeError> {
        if !valid_queue_depth(depth) {
            return Err(NvmeError::InvalidQueueDepth);
        }
        Ok(Self {
            id,
            depth,
            submissions: Vec::with_capacity(depth as usize),
            completions: Vec::with_capacity(depth as usize),
        })
    }
    pub fn id(&self) -> NvmeQueueId {
        self.id
    }
    pub fn depth(&self) -> u16 {
        self.depth
    }
    pub fn pending(&self) -> usize {
        self.submissions.len()
    }
    pub fn submit(&mut self, command: NvmeCommand) -> Result<(), NvmeError> {
        if self.submissions.len() >= self.depth as usize {
            return Err(NvmeError::QueueFull);
        }
        if self
            .submissions
            .iter()
            .any(|c| c.command_id == command.command_id)
            || self
                .completions
                .iter()
                .any(|c| c.command_id == command.command_id)
        {
            return Err(NvmeError::DuplicateCommand);
        }
        if command.command_id == u16::MAX {
            return Err(NvmeError::SubmissionFailed);
        }
        if let Some(buffer) = command.data
            && !buffer.valid()
        {
            return Err(NvmeError::InvalidBuffer);
        }
        self.submissions.push(command);
        Ok(())
    }
    pub fn take_submission(&mut self) -> Option<NvmeCommand> {
        if self.submissions.is_empty() {
            None
        } else {
            Some(self.submissions.remove(0))
        }
    }
    pub fn complete(&mut self, completion: NvmeCompletion) -> Result<(), NvmeError> {
        if !self
            .submissions
            .iter()
            .any(|c| c.command_id == completion.command_id)
        {
            return Err(NvmeError::SubmissionFailed);
        }
        self.submissions
            .retain(|c| c.command_id != completion.command_id);
        self.completions.push(completion);
        Ok(())
    }
    pub fn poll_completion(&mut self) -> Option<NvmeCompletion> {
        if self.completions.is_empty() {
            None
        } else {
            Some(self.completions.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn command(id: u16) -> NvmeCommand {
        NvmeCommand {
            opcode: 1,
            namespace: 1,
            data: Some(NvmeDmaBuffer {
                address: 0x1000,
                length: 512,
            }),
            command_id: id,
        }
    }
    #[test]
    fn queue_round_trip() {
        let mut q = NvmeQueue::new(NvmeQueueId(1), 4).unwrap();
        q.submit(command(1)).unwrap();
        assert_eq!(q.pending(), 1);
        let c = q.take_submission().unwrap();
        assert_eq!(c.command_id, 1);
    }
    #[test]
    fn completion_requires_pending_command() {
        let mut q = NvmeQueue::new(NvmeQueueId(1), 4).unwrap();
        assert_eq!(
            q.complete(NvmeCompletion {
                command_id: 1,
                status: 0,
                result: 0
            }),
            Err(NvmeError::SubmissionFailed)
        );
    }
    #[test]
    fn rejects_invalid_dma() {
        let mut q = NvmeQueue::new(NvmeQueueId(1), 4).unwrap();
        assert_eq!(
            q.submit(NvmeCommand {
                opcode: 1,
                namespace: 1,
                data: Some(NvmeDmaBuffer {
                    address: u64::MAX,
                    length: 1
                }),
                command_id: 1
            }),
            Err(NvmeError::InvalidBuffer)
        );
    }
}

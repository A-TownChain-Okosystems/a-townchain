//! Deterministic userspace/service initialization graph.

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BootService {
    Security,
    Memory,
    Process,
    Ipc,
    Devices,
    Vfs,
    Identity,
    Wallet,
    Network,
    Graphics,
    Audio,
    Package,
    Update,
    Runtime,
    Settings,
}

impl fmt::Display for BootService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootStep {
    pub service: BootService,
    pub requires: &'static [BootService],
}

const EMPTY: &[BootService] = &[];
const SECURITY: &[BootService] = &[BootService::Security];
const MEMORY: &[BootService] = &[BootService::Security, BootService::Memory];
const PROCESS: &[BootService] = &[
    BootService::Security,
    BootService::Memory,
    BootService::Process,
];
const IPC: &[BootService] = &[
    BootService::Security,
    BootService::Memory,
    BootService::Process,
    BootService::Ipc,
];
const VFS: &[BootService] = &[
    BootService::Security,
    BootService::Memory,
    BootService::Process,
    BootService::Ipc,
    BootService::Devices,
];
const IDENTITY: &[BootService] = &[BootService::Security];
const WALLET: &[BootService] = &[BootService::Security, BootService::Identity];

/// Canonical initialization sequence. Every dependency is explicit.
pub const BOOT_PLAN: &[BootStep] = &[
    BootStep {
        service: BootService::Security,
        requires: EMPTY,
    },
    BootStep {
        service: BootService::Memory,
        requires: SECURITY,
    },
    BootStep {
        service: BootService::Process,
        requires: MEMORY,
    },
    BootStep {
        service: BootService::Ipc,
        requires: PROCESS,
    },
    BootStep {
        service: BootService::Devices,
        requires: IPC,
    },
    BootStep {
        service: BootService::Vfs,
        requires: VFS,
    },
    BootStep {
        service: BootService::Identity,
        requires: IDENTITY,
    },
    BootStep {
        service: BootService::Wallet,
        requires: WALLET,
    },
    BootStep {
        service: BootService::Network,
        requires: &[
            BootService::Ipc,
            BootService::Devices,
            BootService::Security,
        ],
    },
    BootStep {
        service: BootService::Graphics,
        requires: &[
            BootService::Ipc,
            BootService::Devices,
            BootService::Security,
        ],
    },
    BootStep {
        service: BootService::Audio,
        requires: &[
            BootService::Ipc,
            BootService::Devices,
            BootService::Security,
        ],
    },
    BootStep {
        service: BootService::Package,
        requires: &[BootService::Ipc, BootService::Vfs, BootService::Security],
    },
    BootStep {
        service: BootService::Update,
        requires: &[
            BootService::Ipc,
            BootService::Vfs,
            BootService::Package,
            BootService::Identity,
        ],
    },
    BootStep {
        service: BootService::Runtime,
        requires: &[
            BootService::Ipc,
            BootService::Vfs,
            BootService::Network,
            BootService::Identity,
        ],
    },
    BootStep {
        service: BootService::Settings,
        requires: &[BootService::Ipc, BootService::Vfs, BootService::Identity],
    },
];

pub fn validate_boot_plan() -> bool {
    for (index, step) in BOOT_PLAN.iter().enumerate() {
        for requirement in step.requires {
            if !BOOT_PLAN[..index]
                .iter()
                .any(|candidate| candidate.service == *requirement)
            {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_plan_is_ordered() {
        assert!(validate_boot_plan());
    }
    #[test]
    fn security_is_first() {
        assert_eq!(
            BOOT_PLAN.first().map(|s| s.service),
            Some(BootService::Security)
        );
    }
}

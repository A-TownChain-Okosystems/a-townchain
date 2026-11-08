//! Page-fault classification and fail-closed handling boundary.

use crate::paging::PhysicalAddress;
use crate::{PageFlags, VirtualAddress};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultAccess {
    Read,
    Write,
    Execute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFault {
    pub address: VirtualAddress,
    pub access: FaultAccess,
    pub present: bool,
    pub user: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultAction {
    Map {
        physical: PhysicalAddress,
        flags: PageFlags,
    },
    Deny,
}

pub fn classify(
    access: FaultAccess,
    present: bool,
    user: bool,
    address: VirtualAddress,
) -> PageFault {
    PageFault {
        address,
        access,
        present,
        user,
    }
}

pub fn authorize_fault(
    fault: PageFault,
    mapping: Option<(PhysicalAddress, PageFlags)>,
) -> FaultAction {
    if fault.present {
        return FaultAction::Deny;
    }
    let Some((physical, flags)) = mapping else {
        return FaultAction::Deny;
    };
    if fault.user && !flags.user {
        return FaultAction::Deny;
    }
    let allowed = match fault.access {
        FaultAccess::Read => flags.read,
        FaultAccess::Write => flags.write,
        FaultAccess::Execute => flags.execute,
    };
    if allowed {
        FaultAction::Map { physical, flags }
    } else {
        FaultAction::Deny
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allows_only_declared_access() {
        let flags = PageFlags::user_read_only();
        let read = classify(FaultAccess::Read, false, true, VirtualAddress(0x4000));
        let write = classify(FaultAccess::Write, false, true, VirtualAddress(0x4000));
        assert!(matches!(
            authorize_fault(read, Some((PhysicalAddress(0x8000), flags))),
            FaultAction::Map { .. }
        ));
        assert_eq!(
            authorize_fault(write, Some((PhysicalAddress(0x8000), flags))),
            FaultAction::Deny
        );
    }

    #[test]
    fn present_faults_fail_closed() {
        let fault = classify(FaultAccess::Read, true, true, VirtualAddress(0x4000));
        assert_eq!(
            authorize_fault(
                fault,
                Some((PhysicalAddress(0x8000), PageFlags::user_read_only()))
            ),
            FaultAction::Deny
        );
    }
}

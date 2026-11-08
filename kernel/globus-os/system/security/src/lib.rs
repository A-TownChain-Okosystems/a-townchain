//! GlobusOS security primitives. Authority is explicit and deny-by-default.

pub mod capability_registry;
pub mod identity;
pub use capability_registry::{CapabilityObject, CapabilityRegistry};

/// A capability identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Capability(pub u128);

/// A permission right for accessing a resource or capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Right {
    /// Permission to read.
    Read,
    /// Permission to write.
    Write,
    /// Permission to execute.
    Execute,
    /// Permission to map memory or resources.
    Map,
    /// Permission to perform cryptographic signing.
    Sign,
    /// Administrative permission with full access.
    Admin,
}

/// A grant pairing a capability with a granted right.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grant {
    /// The capability associated with this grant.
    pub capability: Capability,
    /// The right granted for the capability.
    pub right: Right,
}

/// The outcome of an authorization check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authorization {
    /// The requested action is permitted.
    Allowed,
    /// The requested action is denied.
    Denied,
}

/// Authorizes a requested right against an optional grant.
pub fn authorize(grant: Option<Grant>, requested: Right) -> Authorization {
    match grant {
        Some(g) if g.right == requested || g.right == Right::Admin => Authorization::Allowed,
        _ => Authorization::Denied,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signing_is_distinct_from_execute() {
        let grant = Grant {
            capability: Capability(1),
            right: Right::Execute,
        };
        assert_eq!(authorize(Some(grant), Right::Sign), Authorization::Denied);
    }
    #[test]
    fn admin_can_sign() {
        let grant = Grant {
            capability: Capability(1),
            right: Right::Admin,
        };
        assert_eq!(authorize(Some(grant), Right::Sign), Authorization::Allowed);
    }
}

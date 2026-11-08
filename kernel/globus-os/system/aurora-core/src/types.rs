use core::fmt;

macro_rules! typed_id {
    ($name:ident) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, crate::AuroraError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(crate::AuroraError::InvalidRequest);
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

typed_id!(AgentId);
typed_id!(SkillId);
typed_id!(ToolId);
typed_id!(ModelId);
typed_id!(RequestId);
typed_id!(SessionId);
typed_id!(CapabilityId);
typed_id!(PolicyId);
typed_id!(ApprovalId);
typed_id!(EventId);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuroraRequest {
    pub request_id: RequestId,
    pub session_id: SessionId,
    pub principal: String,
    pub intent: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuroraResponse {
    pub request_id: RequestId,
    pub status: crate::RequestStatus,
    pub result: Option<String>,
    pub audit_reference: Option<String>,
}

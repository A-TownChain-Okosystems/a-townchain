//! Typed IP/socket endpoint addresses. Parsing is intentionally strict and allocation-free for IPv4.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ipv4Address(pub [u8; 4]);

impl Ipv4Address {
    pub const UNSPECIFIED: Self = Self([0, 0, 0, 0]);
    pub const LOOPBACK: Self = Self([127, 0, 0, 1]);
    pub const fn new(octets: [u8; 4]) -> Self {
        Self(octets)
    }
    pub fn is_unspecified(self) -> bool {
        self == Self::UNSPECIFIED
    }
    pub const fn is_loopback(self) -> bool {
        self.0[0] == 127
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IpAddress {
    V4(Ipv4Address),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EndpointAddress {
    pub ip: IpAddress,
    pub port: u16,
}

impl EndpointAddress {
    pub const fn ipv4(ip: Ipv4Address, port: u16) -> Self {
        Self {
            ip: IpAddress::V4(ip),
            port,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressError {
    ZeroPort,
}

pub fn validate_endpoint(endpoint: EndpointAddress) -> Result<(), AddressError> {
    if endpoint.port == 0 {
        Err(AddressError::ZeroPort)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_port() {
        assert!(validate_endpoint(EndpointAddress::ipv4(Ipv4Address::LOOPBACK, 443)).is_ok());
        assert_eq!(
            validate_endpoint(EndpointAddress::ipv4(Ipv4Address::LOOPBACK, 0)),
            Err(AddressError::ZeroPort)
        );
    }
}

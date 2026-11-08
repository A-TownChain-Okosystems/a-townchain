//! Network policy boundary. Protocol implementations remain behind explicit services.

pub mod address;
pub mod socket;

pub use address::{AddressError, EndpointAddress, IpAddress, Ipv4Address, validate_endpoint};
pub use socket::{Socket, SocketState, SocketTable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Ipv4,
    Ipv6,
    Tcp,
    Udp,
    Dns,
    Dhcp,
    Tls,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkPolicy {
    Disabled,
    Restricted,
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketId(pub u64);

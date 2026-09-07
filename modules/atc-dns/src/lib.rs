// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// atc-dns — Decentralized naming service
pub mod resolver;
pub mod registry;
pub mod zones;
pub mod cache;

pub use resolver::DnsResolver;
pub use registry::DnsRegistry;
pub use zones::ZoneManager;
pub use cache::DnsCache;
